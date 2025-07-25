/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::marker::PhantomData;
use std::mem;
use std::sync::Arc;

use app_units::Au;
use canvas_traits::canvas::*;
use compositing_traits::CrossProcessCompositorApi;
use euclid::default::{Box2D, Point2D, Rect, Size2D, Transform2D, Vector2D};
use euclid::point2;
use fonts::{
    ByteIndex, FontBaseline, FontContext, FontGroup, FontMetrics, FontRef, GlyphInfo, GlyphStore,
    LAST_RESORT_GLYPH_ADVANCE, ShapingFlags, ShapingOptions,
};
use ipc_channel::ipc::IpcSender;
use log::warn;
use pixels::Snapshot;
use range::Range;
use servo_arc::Arc as ServoArc;
use style::color::AbsoluteColor;
use style::properties::style_structs::Font as FontStyleStruct;
use unicode_script::Script;
use webrender_api::ImageKey;

use crate::backend::{
    Backend, DrawOptionsHelpers as _, GenericDrawTarget as _, PatternHelpers,
    StrokeOptionsHelpers as _,
};

// Asserts on WR texture cache update for zero sized image with raw data.
// https://github.com/servo/webrender/blob/main/webrender/src/texture_cache.rs#L1475
const MIN_WR_IMAGE_SIZE: Size2D<u64> = Size2D::new(1, 1);

/// A wrapper around a stored PathBuilder and an optional transformation that should be
/// applied to any points to ensure they are in the matching device space.
pub(crate) struct PathBuilderRef<'a> {
    pub(crate) builder: &'a mut Path,
    pub(crate) transform: Transform2D<f32>,
}

impl PathBuilderRef<'_> {
    /// <https://html.spec.whatwg.org/multipage#dom-context-2d-lineto>
    pub(crate) fn line_to(&mut self, pt: &Point2D<f32>) {
        let pt = self.transform.transform_point(*pt).cast();
        self.builder.line_to(pt.x, pt.y);
    }

    pub(crate) fn move_to(&mut self, pt: &Point2D<f32>) {
        let pt = self.transform.transform_point(*pt).cast();
        self.builder.move_to(pt.x, pt.y);
    }

    pub(crate) fn rect(&mut self, rect: &Rect<f32>) {
        let (first, second, third, fourth) = (
            Point2D::new(rect.origin.x, rect.origin.y),
            Point2D::new(rect.origin.x + rect.size.width, rect.origin.y),
            Point2D::new(
                rect.origin.x + rect.size.width,
                rect.origin.y + rect.size.height,
            ),
            Point2D::new(rect.origin.x, rect.origin.y + rect.size.height),
        );
        self.move_to(&first);
        self.line_to(&second);
        self.line_to(&third);
        self.line_to(&fourth);
        self.close();
        self.move_to(&first);
    }

    /// <https://html.spec.whatwg.org/multipage#dom-context-2d-quadraticcurveto>
    pub(crate) fn quadratic_curve_to(&mut self, cp: &Point2D<f32>, endpoint: &Point2D<f32>) {
        let cp = self.transform.transform_point(*cp).cast();
        let endpoint = self.transform.transform_point(*endpoint).cast();
        self.builder
            .quadratic_curve_to(cp.x, cp.y, endpoint.x, endpoint.y)
    }

    /// <https://html.spec.whatwg.org/multipage#dom-context-2d-beziercurveto>
    pub(crate) fn bezier_curve_to(
        &mut self,
        cp1: &Point2D<f32>,
        cp2: &Point2D<f32>,
        endpoint: &Point2D<f32>,
    ) {
        let cp1 = self.transform.transform_point(*cp1).cast();
        let cp2 = self.transform.transform_point(*cp2).cast();
        let endpoint = self.transform.transform_point(*endpoint).cast();
        self.builder
            .bezier_curve_to(cp1.x, cp1.y, cp2.x, cp2.y, endpoint.x, endpoint.y)
    }

    pub(crate) fn arc(
        &mut self,
        center: &Point2D<f32>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        ccw: bool,
    ) {
        let center = self.transform.transform_point(*center).cast();
        let _ = self.builder.arc(
            center.x,
            center.y,
            radius as f64,
            start_angle as f64,
            end_angle as f64,
            ccw,
        );
    }

    /// <https://html.spec.whatwg.org/multipage#dom-context-2d-arcto>
    pub(crate) fn arc_to(&mut self, cp1: &Point2D<f32>, cp2: &Point2D<f32>, radius: f32) {
        let cp0 = if let (Some(inverse), Some(point)) =
            (self.transform.inverse(), self.builder.last_point())
        {
            inverse.transform_point(Point2D::new(point.x, point.y).cast())
        } else {
            *cp1
        };

        // 2. Ensure there is a subpath for (x1, y1) is done by one of self.line_to calls

        if (cp0.x == cp1.x && cp0.y == cp1.y) || cp1 == cp2 || radius == 0.0 {
            self.line_to(cp1);
            return;
        }

        // if all three control points lie on a single straight line,
        // connect the first two by a straight line
        let direction = (cp2.x - cp1.x) * (cp0.y - cp1.y) + (cp2.y - cp1.y) * (cp1.x - cp0.x);
        if direction == 0.0 {
            self.line_to(cp1);
            return;
        }

        // otherwise, draw the Arc
        let a2 = (cp0.x - cp1.x).powi(2) + (cp0.y - cp1.y).powi(2);
        let b2 = (cp1.x - cp2.x).powi(2) + (cp1.y - cp2.y).powi(2);
        let d = {
            let c2 = (cp0.x - cp2.x).powi(2) + (cp0.y - cp2.y).powi(2);
            let cosx = (a2 + b2 - c2) / (2.0 * (a2 * b2).sqrt());
            let sinx = (1.0 - cosx.powi(2)).sqrt();
            radius / ((1.0 - cosx) / sinx)
        };

        // first tangent point
        let anx = (cp1.x - cp0.x) / a2.sqrt();
        let any = (cp1.y - cp0.y) / a2.sqrt();
        let tp1 = Point2D::new(cp1.x - anx * d, cp1.y - any * d);

        // second tangent point
        let bnx = (cp1.x - cp2.x) / b2.sqrt();
        let bny = (cp1.y - cp2.y) / b2.sqrt();
        let tp2 = Point2D::new(cp1.x - bnx * d, cp1.y - bny * d);

        // arc center and angles
        let anticlockwise = direction < 0.0;
        let cx = tp1.x + any * radius * if anticlockwise { 1.0 } else { -1.0 };
        let cy = tp1.y - anx * radius * if anticlockwise { 1.0 } else { -1.0 };
        let angle_start = (tp1.y - cy).atan2(tp1.x - cx);
        let angle_end = (tp2.y - cy).atan2(tp2.x - cx);

        self.line_to(&self.transform.transform_point(tp1));
        if [cx, cy, angle_start, angle_end]
            .iter()
            .all(|x| x.is_finite())
        {
            self.arc(
                &self.transform.transform_point(Point2D::new(cx, cy)),
                radius,
                angle_start,
                angle_end,
                anticlockwise,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn ellipse(
        &mut self,
        center: &Point2D<f32>,
        radius_x: f32,
        radius_y: f32,
        rotation_angle: f32,
        start_angle: f32,
        end_angle: f32,
        ccw: bool,
    ) {
        let center = self.transform.transform_point(*center).cast();
        let _ = self.builder.ellipse(
            center.x,
            center.y,
            radius_x as f64,
            radius_y as f64,
            rotation_angle as f64,
            start_angle as f64,
            end_angle as f64,
            ccw,
        );
    }

    pub(crate) fn close(&mut self) {
        self.builder.close_path();
    }
}

#[derive(Default)]
struct UnshapedTextRun<'a> {
    font: Option<FontRef>,
    script: Script,
    string: &'a str,
}

impl UnshapedTextRun<'_> {
    fn script_and_font_compatible(&self, script: Script, other_font: &Option<FontRef>) -> bool {
        if self.script != script {
            return false;
        }

        match (&self.font, other_font) {
            (Some(font_a), Some(font_b)) => font_a.identifier() == font_b.identifier(),
            (None, None) => true,
            _ => false,
        }
    }

    fn into_shaped_text_run(self) -> Option<TextRun> {
        let font = self.font?;
        if self.string.is_empty() {
            return None;
        }

        let word_spacing = Au::from_f64_px(
            font.glyph_index(' ')
                .map(|glyph_id| font.glyph_h_advance(glyph_id))
                .unwrap_or(LAST_RESORT_GLYPH_ADVANCE),
        );
        let options = ShapingOptions {
            letter_spacing: None,
            word_spacing,
            script: self.script,
            flags: ShapingFlags::empty(),
        };
        let glyphs = font.shape_text(self.string, &options);
        Some(TextRun { font, glyphs })
    }
}

pub(crate) struct TextRun {
    pub(crate) font: FontRef,
    pub(crate) glyphs: Arc<GlyphStore>,
}

impl TextRun {
    fn bounding_box(&self) -> Rect<f32> {
        let mut bounding_box = None;
        let mut bounds_offset: f32 = 0.;
        let glyph_ids = self
            .glyphs
            .iter_glyphs_for_byte_range(&Range::new(ByteIndex(0), self.glyphs.len()))
            .map(GlyphInfo::id);
        for glyph_id in glyph_ids {
            let bounds = self.font.typographic_bounds(glyph_id);
            let amount = Vector2D::new(bounds_offset, 0.);
            let bounds = bounds.translate(amount);
            let initiated_bbox = bounding_box.get_or_insert_with(|| {
                let origin = Point2D::new(bounds.min_x(), 0.);
                Box2D::new(origin, origin).to_rect()
            });
            bounding_box = Some(initiated_bbox.union(&bounds));
            bounds_offset = bounds.max_x();
        }
        bounding_box.unwrap_or_default()
    }
}

#[derive(Clone, Copy)]
pub(crate) enum Filter {
    Bilinear,
    Nearest,
}

pub(crate) struct CanvasData<'a, B: Backend> {
    backend: B,
    drawtarget: B::DrawTarget,
    /// <https://html.spec.whatwg.org/multipage/#current-default-path>
    path_state: Path,
    state: CanvasPaintState<'a, B>,
    saved_states: Vec<CanvasPaintState<'a, B>>,
    compositor_api: CrossProcessCompositorApi,
    image_key: ImageKey,
    font_context: Arc<FontContext>,
}

impl<'a, B: Backend> CanvasData<'a, B> {
    pub(crate) fn new(
        size: Size2D<u64>,
        compositor_api: CrossProcessCompositorApi,
        font_context: Arc<FontContext>,
        backend: B,
    ) -> CanvasData<'a, B> {
        let size = size.max(MIN_WR_IMAGE_SIZE);
        let draw_target = backend.create_drawtarget(size);
        let image_key = compositor_api.generate_image_key_blocking().unwrap();
        let (descriptor, data) = draw_target.image_descriptor_and_serializable_data();
        compositor_api.add_image(image_key, descriptor, data);
        CanvasData {
            state: backend.new_paint_state(),
            backend,
            drawtarget: draw_target,
            path_state: Path::new(),
            saved_states: vec![],
            compositor_api,
            image_key,
            font_context,
        }
    }

    pub(crate) fn image_key(&self) -> ImageKey {
        self.image_key
    }

    pub(crate) fn draw_image(
        &mut self,
        snapshot: Snapshot,
        dest_rect: Rect<f64>,
        source_rect: Rect<f64>,
        smoothing_enabled: bool,
    ) {
        // We round up the floating pixel values to draw the pixels
        let source_rect = source_rect.ceil();
        // It discards the extra pixels (if any) that won't be painted
        let snapshot = if Rect::from_size(snapshot.size().to_f64()).contains_rect(&source_rect) {
            snapshot.get_rect(source_rect.to_u32())
        } else {
            snapshot
        };

        let draw_options = self.state.draw_options.clone();
        let writer = |draw_target: &mut B::DrawTarget| {
            write_image::<B>(
                draw_target,
                snapshot,
                dest_rect,
                smoothing_enabled,
                &draw_options,
            );
        };

        if self.need_to_draw_shadow() {
            let rect = Rect::new(
                Point2D::new(dest_rect.origin.x as f32, dest_rect.origin.y as f32),
                Size2D::new(dest_rect.size.width as f32, dest_rect.size.height as f32),
            );

            // TODO(pylbrecht) pass another closure for raqote
            self.draw_with_shadow(&rect, writer);
        } else {
            writer(&mut self.drawtarget);
        }
    }

    pub(crate) fn save_context_state(&mut self) {
        self.saved_states.push(self.state.clone());
    }

    pub(crate) fn restore_context_state(&mut self) {
        if let Some(state) = self.saved_states.pop() {
            let _ = mem::replace(&mut self.state, state);
            self.drawtarget.set_transform(&self.state.transform);
            self.drawtarget.pop_clip();
        }
    }

    pub(crate) fn fill_text_with_size(
        &mut self,
        text: String,
        x: f64,
        y: f64,
        max_width: Option<f64>,
        is_rtl: bool,
        size: f64,
    ) {
        // > Step 2: Replace all ASCII whitespace in text with U+0020 SPACE characters.
        let text = replace_ascii_whitespace(text);

        // > Step 3: Let font be the current font of target, as given by that object's font
        // > attribute.
        let Some(ref font_style) = self.state.font_style else {
            return;
        };

        let font_group = self
            .font_context
            .font_group_with_size(font_style.clone(), Au::from_f64_px(size));
        let mut font_group = font_group.write();
        let Some(first_font) = font_group.first(&self.font_context) else {
            warn!("Could not render canvas text, because there was no first font.");
            return;
        };

        let runs = self.build_unshaped_text_runs(&text, &mut font_group);
        // TODO: This doesn't do any kind of line layout at all. In particular, there needs
        // to be some alignment along a baseline and also support for bidi text.
        let shaped_runs: Vec<_> = runs
            .into_iter()
            .filter_map(UnshapedTextRun::into_shaped_text_run)
            .collect();
        let total_advance = shaped_runs
            .iter()
            .map(|run| run.glyphs.total_advance())
            .sum::<Au>()
            .to_f64_px();

        // > Step 6: If maxWidth was provided and the hypothetical width of the inline box in the
        // > hypothetical line box is greater than maxWidth CSS pixels, then change font to have a
        // > more condensed font (if one is available or if a reasonably readable one can be
        // > synthesized by applying a horizontal scale factor to the font) or a smaller font, and
        // > return to the previous step.
        //
        // TODO: We only try decreasing the font size here. Eventually it would make sense to use
        // other methods to try to decrease the size, such as finding a narrower font or decreasing
        // spacing.
        if let Some(max_width) = max_width {
            let new_size = (max_width / total_advance * size).floor().max(5.);
            if total_advance > max_width && new_size != size {
                self.fill_text_with_size(text, x, y, Some(max_width), is_rtl, new_size);
                return;
            }
        }

        // > Step 7: Find the anchor point for the line of text.
        let start = self.find_anchor_point_for_line_of_text(
            x as f32,
            y as f32,
            &first_font.metrics,
            total_advance as f32,
            is_rtl,
        );

        // > Step 8: Let result be an array constructed by iterating over each glyph in the inline box
        // > from left to right (if any), adding to the array, for each glyph, the shape of the glyph
        // > as it is in the inline box, positioned on a coordinate space using CSS pixels with its
        // > origin is at the anchor point.
        self.maybe_bound_shape_with_pattern(
            self.state.fill_style.clone(),
            &Rect::from_size(Size2D::new(total_advance, size)),
            |self_| {
                self_.drawtarget.fill_text(
                    shaped_runs,
                    start,
                    &self_.state.fill_style,
                    &self_.state.draw_options,
                );
            },
        );
    }

    /// <https://html.spec.whatwg.org/multipage/#text-preparation-algorithm>
    pub(crate) fn fill_text(
        &mut self,
        text: String,
        x: f64,
        y: f64,
        max_width: Option<f64>,
        is_rtl: bool,
    ) {
        let Some(ref font_style) = self.state.font_style else {
            return;
        };

        let size = font_style.font_size.computed_size();
        self.fill_text_with_size(text, x, y, max_width, is_rtl, size.px() as f64);
    }

    /// <https://html.spec.whatwg.org/multipage/#text-preparation-algorithm>
    /// <https://html.spec.whatwg.org/multipage/#dom-context-2d-measuretext>
    pub(crate) fn measure_text(&mut self, text: String) -> TextMetrics {
        // > Step 2: Replace all ASCII whitespace in text with U+0020 SPACE characters.
        let text = replace_ascii_whitespace(text);
        let Some(ref font_style) = self.state.font_style else {
            return TextMetrics::default();
        };

        let font_group = self.font_context.font_group(font_style.clone());
        let mut font_group = font_group.write();
        let font = font_group
            .first(&self.font_context)
            .expect("couldn't find font");
        let ascent = font.metrics.ascent.to_f32_px();
        let descent = font.metrics.descent.to_f32_px();
        let runs = self.build_unshaped_text_runs(&text, &mut font_group);

        let shaped_runs: Vec<_> = runs
            .into_iter()
            .filter_map(UnshapedTextRun::into_shaped_text_run)
            .collect();
        let total_advance = shaped_runs
            .iter()
            .map(|run| run.glyphs.total_advance())
            .sum::<Au>()
            .to_f32_px();
        let bounding_box = shaped_runs
            .iter()
            .map(TextRun::bounding_box)
            .reduce(|a, b| {
                let amount = Vector2D::new(a.max_x(), 0.);
                let bounding_box = b.translate(amount);
                a.union(&bounding_box)
            })
            .unwrap_or_default();

        let FontBaseline {
            ideographic_baseline,
            alphabetic_baseline,
            hanging_baseline,
        } = match font.baseline() {
            Some(baseline) => baseline,
            None => FontBaseline {
                hanging_baseline: ascent * HANGING_BASELINE_DEFAULT,
                ideographic_baseline: -descent * IDEOGRAPHIC_BASELINE_DEFAULT,
                alphabetic_baseline: 0.,
            },
        };

        let anchor_x = match self.state.text_align {
            TextAlign::End => total_advance,
            TextAlign::Center => total_advance / 2.,
            TextAlign::Right => total_advance,
            _ => 0.,
        };
        let anchor_y = match self.state.text_baseline {
            TextBaseline::Top => ascent,
            TextBaseline::Hanging => hanging_baseline,
            TextBaseline::Ideographic => ideographic_baseline,
            TextBaseline::Middle => (ascent - descent) / 2.,
            TextBaseline::Alphabetic => alphabetic_baseline,
            TextBaseline::Bottom => -descent,
        };

        TextMetrics {
            width: total_advance,
            actual_boundingbox_left: anchor_x - bounding_box.min_x(),
            actual_boundingbox_right: bounding_box.max_x() - anchor_x,
            actual_boundingbox_ascent: bounding_box.max_y() - anchor_y,
            actual_boundingbox_descent: anchor_y - bounding_box.min_y(),
            font_boundingbox_ascent: ascent - anchor_y,
            font_boundingbox_descent: descent + anchor_y,
            em_height_ascent: ascent - anchor_y,
            em_height_descent: descent + anchor_y,
            hanging_baseline: hanging_baseline - anchor_y,
            alphabetic_baseline: alphabetic_baseline - anchor_y,
            ideographic_baseline: ideographic_baseline - anchor_y,
        }
    }

    fn build_unshaped_text_runs<'b>(
        &self,
        text: &'b str,
        font_group: &mut FontGroup,
    ) -> Vec<UnshapedTextRun<'b>> {
        let mut runs = Vec::new();
        let mut current_text_run = UnshapedTextRun::default();
        let mut current_text_run_start_index = 0;

        for (index, character) in text.char_indices() {
            // TODO: This should ultimately handle emoji variation selectors, but raqote does not yet
            // have support for color glyphs.
            let script = Script::from(character);
            let font = font_group.find_by_codepoint(&self.font_context, character, None, None);

            if !current_text_run.script_and_font_compatible(script, &font) {
                let previous_text_run = mem::replace(
                    &mut current_text_run,
                    UnshapedTextRun {
                        font: font.clone(),
                        script,
                        ..Default::default()
                    },
                );
                current_text_run_start_index = index;
                runs.push(previous_text_run)
            }

            current_text_run.string =
                &text[current_text_run_start_index..index + character.len_utf8()];
        }

        runs.push(current_text_run);
        runs
    }

    /// Find the *anchor_point* for the given parameters of a line of text.
    /// See <https://html.spec.whatwg.org/multipage/#text-preparation-algorithm>.
    fn find_anchor_point_for_line_of_text(
        &self,
        x: f32,
        y: f32,
        metrics: &FontMetrics,
        width: f32,
        is_rtl: bool,
    ) -> Point2D<f32> {
        let text_align = match self.state.text_align {
            TextAlign::Start if is_rtl => TextAlign::Right,
            TextAlign::Start => TextAlign::Left,
            TextAlign::End if is_rtl => TextAlign::Left,
            TextAlign::End => TextAlign::Right,
            text_align => text_align,
        };
        let anchor_x = match text_align {
            TextAlign::Center => -width / 2.,
            TextAlign::Right => -width,
            _ => 0.,
        };

        let ascent = metrics.ascent.to_f32_px();
        let descent = metrics.descent.to_f32_px();
        let anchor_y = match self.state.text_baseline {
            TextBaseline::Top => ascent,
            TextBaseline::Hanging => ascent * HANGING_BASELINE_DEFAULT,
            TextBaseline::Ideographic => -descent * IDEOGRAPHIC_BASELINE_DEFAULT,
            TextBaseline::Middle => (ascent - descent) / 2.,
            TextBaseline::Alphabetic => 0.,
            TextBaseline::Bottom => -descent,
        };

        point2(x + anchor_x, y + anchor_y)
    }

    pub(crate) fn fill_rect(&mut self, rect: &Rect<f32>) {
        if self.state.fill_style.is_zero_size_gradient() {
            return; // Paint nothing if gradient size is zero.
        }

        if self.need_to_draw_shadow() {
            self.draw_with_shadow(rect, |new_draw_target: &mut B::DrawTarget| {
                new_draw_target.fill_rect(rect, &self.state.fill_style, &self.state.draw_options);
            });
        } else {
            self.maybe_bound_shape_with_pattern(
                self.state.fill_style.clone(),
                &rect.cast(),
                |self_| {
                    self_.drawtarget.fill_rect(
                        rect,
                        &self_.state.fill_style,
                        &self_.state.draw_options,
                    );
                },
            );
        }
    }

    pub(crate) fn clear_rect(&mut self, rect: &Rect<f32>) {
        self.drawtarget.clear_rect(rect);
    }

    pub(crate) fn stroke_rect(&mut self, rect: &Rect<f32>) {
        if self.state.stroke_style.is_zero_size_gradient() {
            return; // Paint nothing if gradient size is zero.
        }

        if self.need_to_draw_shadow() {
            self.draw_with_shadow(rect, |new_draw_target: &mut B::DrawTarget| {
                new_draw_target.stroke_rect(
                    rect,
                    &self.state.stroke_style,
                    &self.state.stroke_opts,
                    &self.state.draw_options,
                );
            });
        } else {
            self.maybe_bound_shape_with_pattern(
                self.state.stroke_style.clone(),
                &rect.cast(),
                |self_| {
                    self_.drawtarget.stroke_rect(
                        rect,
                        &self_.state.stroke_style,
                        &self_.state.stroke_opts,
                        &self_.state.draw_options,
                    );
                },
            )
        }
    }

    pub(crate) fn begin_path(&mut self) {
        // Erase any traces of previous paths that existed before this.
        self.path_state = Path::new();
    }

    pub(crate) fn close_path(&mut self) {
        self.path_builder().close();
    }

    pub(crate) fn fill(&mut self) {
        if self.state.fill_style.is_zero_size_gradient() {
            return; // Paint nothing if gradient size is zero.
        }

        let path = self.path_state.clone();

        self.maybe_bound_shape_with_pattern(
            self.state.fill_style.clone(),
            &path.bounding_box(),
            |self_| {
                self_.drawtarget.fill(
                    &path,
                    &self_.state.fill_style,
                    &self_.state.draw_options.clone(),
                );
            },
        )
    }

    pub(crate) fn fill_path(&mut self, path: &Path) {
        if self.state.fill_style.is_zero_size_gradient() {
            return; // Paint nothing if gradient size is zero.
        }

        self.drawtarget
            .fill(path, &self.state.fill_style, &self.state.draw_options);
    }

    pub(crate) fn stroke(&mut self) {
        if self.state.stroke_style.is_zero_size_gradient() {
            return; // Paint nothing if gradient size is zero.
        }

        let path = self.path_state.clone();

        self.maybe_bound_shape_with_pattern(
            self.state.stroke_style.clone(),
            &path.bounding_box(),
            |self_| {
                self_.drawtarget.stroke(
                    &path,
                    &self_.state.stroke_style,
                    &self_.state.stroke_opts,
                    &self_.state.draw_options,
                );
            },
        )
    }

    pub(crate) fn stroke_path(&mut self, path: &Path) {
        if self.state.stroke_style.is_zero_size_gradient() {
            return; // Paint nothing if gradient size is zero.
        }

        self.maybe_bound_shape_with_pattern(
            self.state.stroke_style.clone(),
            &path.bounding_box(),
            |self_| {
                self_.drawtarget.stroke(
                    path,
                    &self_.state.stroke_style,
                    &self_.state.stroke_opts,
                    &self_.state.draw_options,
                );
            },
        )
    }

    pub(crate) fn clip(&mut self) {
        let path = &self.path_state;

        self.drawtarget.push_clip(path);
    }

    pub(crate) fn clip_path(&mut self, path: &Path) {
        self.drawtarget.push_clip(path);
    }

    pub(crate) fn is_point_in_path(
        &mut self,
        x: f64,
        y: f64,
        fill_rule: FillRule,
        chan: IpcSender<bool>,
    ) {
        let mut path = self.path_state.clone();
        path.transform(self.get_transform().cast());
        chan.send(path.is_point_in_path(x, y, fill_rule)).unwrap();
    }

    pub(crate) fn move_to(&mut self, point: &Point2D<f32>) {
        self.path_builder().move_to(point);
    }

    pub(crate) fn line_to(&mut self, point: &Point2D<f32>) {
        self.path_builder().line_to(point);
    }

    fn path_builder(&mut self) -> PathBuilderRef {
        PathBuilderRef {
            builder: &mut self.path_state,
            transform: Transform2D::identity(),
        }
    }

    pub(crate) fn rect(&mut self, rect: &Rect<f32>) {
        self.path_builder().rect(rect);
    }

    pub(crate) fn quadratic_curve_to(&mut self, cp: &Point2D<f32>, endpoint: &Point2D<f32>) {
        self.path_builder().quadratic_curve_to(cp, endpoint);
    }

    pub(crate) fn bezier_curve_to(
        &mut self,
        cp1: &Point2D<f32>,
        cp2: &Point2D<f32>,
        endpoint: &Point2D<f32>,
    ) {
        self.path_builder().bezier_curve_to(cp1, cp2, endpoint);
    }

    pub(crate) fn arc(
        &mut self,
        center: &Point2D<f32>,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        ccw: bool,
    ) {
        self.path_builder()
            .arc(center, radius, start_angle, end_angle, ccw);
    }

    pub(crate) fn arc_to(&mut self, cp1: &Point2D<f32>, cp2: &Point2D<f32>, radius: f32) {
        self.path_builder().arc_to(cp1, cp2, radius);
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn ellipse(
        &mut self,
        center: &Point2D<f32>,
        radius_x: f32,
        radius_y: f32,
        rotation_angle: f32,
        start_angle: f32,
        end_angle: f32,
        ccw: bool,
    ) {
        self.path_builder().ellipse(
            center,
            radius_x,
            radius_y,
            rotation_angle,
            start_angle,
            end_angle,
            ccw,
        );
    }

    pub(crate) fn set_fill_style(&mut self, style: FillOrStrokeStyle) {
        self.backend
            .set_fill_style(style, &mut self.state, &self.drawtarget);
    }

    pub(crate) fn set_stroke_style(&mut self, style: FillOrStrokeStyle) {
        self.backend
            .set_stroke_style(style, &mut self.state, &self.drawtarget);
    }

    pub(crate) fn set_line_width(&mut self, width: f32) {
        self.state.stroke_opts.set_line_width(width);
    }

    pub(crate) fn set_line_cap(&mut self, cap: LineCapStyle) {
        self.state.stroke_opts.set_line_cap(cap);
    }

    pub(crate) fn set_line_join(&mut self, join: LineJoinStyle) {
        self.state.stroke_opts.set_line_join(join);
    }

    pub(crate) fn set_miter_limit(&mut self, limit: f32) {
        self.state.stroke_opts.set_miter_limit(limit);
    }

    pub(crate) fn set_line_dash(&mut self, items: Vec<f32>) {
        self.state.stroke_opts.set_line_dash(items);
    }

    pub(crate) fn set_line_dash_offset(&mut self, offset: f32) {
        self.state.stroke_opts.set_line_dash_offset(offset);
    }

    pub(crate) fn get_transform(&self) -> Transform2D<f32> {
        self.drawtarget.get_transform()
    }

    pub(crate) fn set_transform(&mut self, transform: &Transform2D<f32>) {
        self.path_state.transform(self.get_transform().cast());
        self.state.transform = *transform;
        self.drawtarget.set_transform(transform);
        if let Some(inverse) = transform.inverse() {
            self.path_state.transform(inverse.cast());
        }
    }

    pub(crate) fn set_global_alpha(&mut self, alpha: f32) {
        self.state.draw_options.set_alpha(alpha);
    }

    pub(crate) fn set_global_composition(&mut self, op: CompositionOrBlending) {
        self.backend.set_global_composition(op, &mut self.state);
    }

    /// <https://html.spec.whatwg.org/multipage/#reset-the-rendering-context-to-its-default-state>
    pub(crate) fn recreate(&mut self, size: Option<Size2D<u64>>) {
        let size = size
            .unwrap_or_else(|| self.drawtarget.get_size().to_u64())
            .max(MIN_WR_IMAGE_SIZE);

        // Step 1. Clear canvas's bitmap to transparent black.
        self.drawtarget = self
            .backend
            .create_drawtarget(Size2D::new(size.width, size.height));

        // Step 2. Empty the list of subpaths in context's current default path.
        self.path_state = Path::new();

        // Step 3. Clear the context's drawing state stack.
        self.saved_states.clear();

        // Step 4. Reset everything that drawing state consists of to their
        // initial values.
        self.state = self.backend.new_paint_state();

        self.update_image_rendering();
    }

    /// Update image in WebRender
    pub(crate) fn update_image_rendering(&mut self) {
        let (descriptor, data) = self.drawtarget.image_descriptor_and_serializable_data();

        self.compositor_api
            .update_image(self.image_key, descriptor, data);
    }

    // https://html.spec.whatwg.org/multipage/#dom-context-2d-putimagedata
    pub(crate) fn put_image_data(&mut self, snapshot: Snapshot, rect: Rect<u32>) {
        assert_eq!(rect.size, snapshot.size());
        let source_surface = self
            .drawtarget
            .create_source_surface_from_data(snapshot)
            .unwrap();
        self.drawtarget.copy_surface(
            source_surface,
            Rect::from_size(rect.size.to_i32()),
            rect.origin.to_i32(),
        );
    }

    pub(crate) fn set_shadow_offset_x(&mut self, value: f64) {
        self.state.shadow_offset_x = value;
    }

    pub(crate) fn set_shadow_offset_y(&mut self, value: f64) {
        self.state.shadow_offset_y = value;
    }

    pub(crate) fn set_shadow_blur(&mut self, value: f64) {
        self.state.shadow_blur = value;
    }

    pub(crate) fn set_shadow_color(&mut self, value: AbsoluteColor) {
        self.backend.set_shadow_color(value, &mut self.state);
    }

    pub(crate) fn set_font(&mut self, font_style: FontStyleStruct) {
        self.state.font_style = Some(ServoArc::new(font_style))
    }

    pub(crate) fn set_text_align(&mut self, text_align: TextAlign) {
        self.state.text_align = text_align;
    }

    pub(crate) fn set_text_baseline(&mut self, text_baseline: TextBaseline) {
        self.state.text_baseline = text_baseline;
    }

    // https://html.spec.whatwg.org/multipage/#when-shadows-are-drawn
    fn need_to_draw_shadow(&self) -> bool {
        self.backend.need_to_draw_shadow(&self.state.shadow_color) &&
            (self.state.shadow_offset_x != 0.0f64 ||
                self.state.shadow_offset_y != 0.0f64 ||
                self.state.shadow_blur != 0.0f64)
    }

    fn create_draw_target_for_shadow(&self, source_rect: &Rect<f32>) -> B::DrawTarget {
        let mut draw_target = self.drawtarget.create_similar_draw_target(&Size2D::new(
            source_rect.size.width as i32,
            source_rect.size.height as i32,
        ));
        let matrix = self.state.transform.then(
            &Transform2D::identity().pre_translate(-source_rect.origin.to_vector().cast::<f32>()),
        );
        draw_target.set_transform(&matrix);
        draw_target
    }

    fn draw_with_shadow<F>(&self, rect: &Rect<f32>, draw_shadow_source: F)
    where
        F: FnOnce(&mut B::DrawTarget),
    {
        let shadow_src_rect = self.state.transform.outer_transformed_rect(rect);
        let mut new_draw_target = self.create_draw_target_for_shadow(&shadow_src_rect);
        draw_shadow_source(&mut new_draw_target);
        self.drawtarget.draw_surface_with_shadow(
            new_draw_target.surface(),
            &Point2D::new(shadow_src_rect.origin.x, shadow_src_rect.origin.y),
            &self.state.shadow_color,
            &Vector2D::new(
                self.state.shadow_offset_x as f32,
                self.state.shadow_offset_y as f32,
            ),
            (self.state.shadow_blur / 2.0f64) as f32,
            self.backend.get_composition_op(&self.state.draw_options),
        );
    }

    /// Push a clip to the draw target to respect the non-repeating bound (either x, y, or both)
    /// of the given pattern.
    fn maybe_bound_shape_with_pattern<F>(
        &mut self,
        pattern: B::Pattern<'_>,
        path_bound_box: &Rect<f64>,
        draw_shape: F,
    ) where
        F: FnOnce(&mut Self),
    {
        let x_bound = pattern.x_bound();
        let y_bound = pattern.y_bound();
        // Clear operations are also unbounded.
        if self.state.draw_options.is_clear() || (x_bound.is_none() && y_bound.is_none()) {
            draw_shape(self);
            return;
        }
        let rect = Rect::from_size(Size2D::new(
            x_bound.unwrap_or(path_bound_box.size.width.ceil() as u32),
            y_bound.unwrap_or(path_bound_box.size.height.ceil() as u32),
        ))
        .cast();
        let rect = self.get_transform().outer_transformed_rect(&rect);
        self.drawtarget.push_clip_rect(&rect.cast());
        draw_shape(self);
        self.drawtarget.pop_clip();
    }

    /// It reads image data from the canvas
    /// canvas_size: The size of the canvas we're reading from
    /// read_rect: The area of the canvas we want to read from
    #[allow(unsafe_code)]
    pub(crate) fn read_pixels(
        &self,
        read_rect: Option<Rect<u32>>,
        canvas_size: Option<Size2D<u32>>,
    ) -> Snapshot {
        let canvas_size = canvas_size.unwrap_or(self.drawtarget.get_size().cast());

        if let Some(read_rect) = read_rect {
            let canvas_rect = Rect::from_size(canvas_size);
            if canvas_rect
                .intersection(&read_rect)
                .is_none_or(|rect| rect.is_empty())
            {
                Snapshot::empty()
            } else {
                self.drawtarget.snapshot().get_rect(read_rect)
            }
        } else {
            self.drawtarget.snapshot()
        }
    }
}

impl<B: Backend> Drop for CanvasData<'_, B> {
    fn drop(&mut self) {
        self.compositor_api.delete_image(self.image_key);
    }
}

const HANGING_BASELINE_DEFAULT: f32 = 0.8;
const IDEOGRAPHIC_BASELINE_DEFAULT: f32 = 0.5;

#[derive(Clone)]
pub(crate) struct CanvasPaintState<'a, B: Backend> {
    pub(crate) draw_options: B::DrawOptions,
    pub(crate) fill_style: B::Pattern<'a>,
    pub(crate) stroke_style: B::Pattern<'a>,
    pub(crate) stroke_opts: B::StrokeOptions,
    /// The current 2D transform matrix.
    pub(crate) transform: Transform2D<f32>,
    pub(crate) shadow_offset_x: f64,
    pub(crate) shadow_offset_y: f64,
    pub(crate) shadow_blur: f64,
    pub(crate) shadow_color: B::Color,
    pub(crate) font_style: Option<ServoArc<FontStyleStruct>>,
    pub(crate) text_align: TextAlign,
    pub(crate) text_baseline: TextBaseline,
    pub(crate) _backend: PhantomData<B>,
}

/// It writes an image to the destination target
/// draw_target: the destination target where the image_data will be copied
/// image_data: Pixel information of the image to be written. It takes RGBA8
/// image_size: The size of the image to be written
/// dest_rect: Area of the destination target where the pixels will be copied
/// smoothing_enabled: It determines if smoothing is applied to the image result
/// premultiply: Determines whenever the image data should be premultiplied or not
fn write_image<B: Backend>(
    draw_target: &mut B::DrawTarget,
    snapshot: Snapshot,
    dest_rect: Rect<f64>,
    smoothing_enabled: bool,
    draw_options: &B::DrawOptions,
) {
    if snapshot.size().is_empty() {
        return;
    }

    let image_rect = Rect::new(Point2D::zero(), snapshot.size().cast());

    // From spec https://html.spec.whatwg.org/multipage/#dom-context-2d-drawimage
    // When scaling up, if the imageSmoothingEnabled attribute is set to true, the user agent should attempt
    // to apply a smoothing algorithm to the image data when it is scaled.
    // Otherwise, the image must be rendered using nearest-neighbor interpolation.
    let filter = if smoothing_enabled {
        Filter::Bilinear
    } else {
        Filter::Nearest
    };

    let source_surface = draw_target
        .create_source_surface_from_data(snapshot)
        .unwrap();

    draw_target.draw_surface(source_surface, dest_rect, image_rect, filter, draw_options);
}

pub(crate) trait RectToi32 {
    fn ceil(&self) -> Rect<f64>;
}

impl RectToi32 for Rect<f64> {
    fn ceil(&self) -> Rect<f64> {
        Rect::new(
            Point2D::new(self.origin.x.ceil(), self.origin.y.ceil()),
            Size2D::new(self.size.width.ceil(), self.size.height.ceil()),
        )
    }
}

fn replace_ascii_whitespace(text: String) -> String {
    text.chars()
        .map(|c| match c {
            ' ' | '\t' | '\n' | '\r' | '\x0C' => '\x20',
            _ => c,
        })
        .collect()
}
