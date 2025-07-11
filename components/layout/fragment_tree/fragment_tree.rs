/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::cell::Cell;

use app_units::Au;
use base::print_tree::PrintTree;
use compositing_traits::display_list::AxesScrollSensitivity;
use fxhash::FxHashSet;
use malloc_size_of_derive::MallocSizeOf;
use style::animation::AnimationSetKey;

use super::{BoxFragment, ContainingBlockManager, Fragment};
use crate::ArcRefCell;
use crate::context::LayoutContext;
use crate::geom::PhysicalRect;

#[derive(MallocSizeOf)]
pub struct FragmentTree {
    /// Fragments at the top-level of the tree.
    ///
    /// If the root element has `display: none`, there are zero fragments.
    /// Otherwise, there is at least one:
    ///
    /// * The first fragment is generated by the root element.
    /// * There may be additional fragments generated by positioned boxes
    ///   that have the initial containing block.
    pub(crate) root_fragments: Vec<Fragment>,

    /// The scrollable overflow rectangle for the entire tree
    /// <https://drafts.csswg.org/css-overflow/#scrollable>
    scrollable_overflow: Cell<Option<PhysicalRect<Au>>>,

    /// The containing block used in the layout of this fragment tree.
    pub(crate) initial_containing_block: PhysicalRect<Au>,

    /// Whether or not the viewport is sensitive to scroll input events.
    pub viewport_scroll_sensitivity: AxesScrollSensitivity,
}

impl FragmentTree {
    pub(crate) fn new(
        layout_context: &LayoutContext,
        root_fragments: Vec<Fragment>,
        initial_containing_block: PhysicalRect<Au>,
        viewport_scroll_sensitivity: AxesScrollSensitivity,
    ) -> Self {
        let fragment_tree = Self {
            root_fragments,
            scrollable_overflow: Cell::default(),
            initial_containing_block,
            viewport_scroll_sensitivity,
        };

        // As part of building the fragment tree, we want to stop animating elements and
        // pseudo-elements that used to be animating or had animating images attached to
        // them. Create a set of all elements that used to be animating.
        let mut animations = layout_context.style_context.animations.sets.write();
        let mut invalid_animating_nodes: FxHashSet<_> = animations.keys().cloned().collect();
        let mut image_animations = layout_context
            .image_resolver
            .node_to_animating_image_map
            .write()
            .to_owned();
        let mut invalid_image_animating_nodes: FxHashSet<_> = image_animations
            .keys()
            .cloned()
            .map(|node| AnimationSetKey::new(node, None))
            .collect();

        fragment_tree.find(|fragment, _level, containing_block| {
            if let Some(tag) = fragment.tag() {
                invalid_animating_nodes.remove(&AnimationSetKey::new(tag.node, tag.pseudo));
                invalid_image_animating_nodes.remove(&AnimationSetKey::new(tag.node, tag.pseudo));
            }

            fragment.set_containing_block(containing_block);
            None::<()>
        });

        // Cancel animations for any elements and pseudo-elements that are no longer found
        // in the fragment tree.
        for node in &invalid_animating_nodes {
            if let Some(state) = animations.get_mut(node) {
                state.cancel_all_animations();
            }
        }
        for node in &invalid_image_animating_nodes {
            image_animations.remove(&node.node);
        }

        fragment_tree
    }

    pub fn print(&self) {
        let mut print_tree = PrintTree::new("Fragment Tree".to_string());
        for fragment in &self.root_fragments {
            fragment.print(&mut print_tree);
        }
    }

    pub(crate) fn scrollable_overflow(&self) -> PhysicalRect<Au> {
        self.scrollable_overflow
            .get()
            .expect("Should only call `scrollable_overflow()` after calculating overflow")
    }

    /// Calculate the scrollable overflow / scrolling area for this [`FragmentTree`] according
    /// to <https://drafts.csswg.org/cssom-view/#scrolling-area>.
    pub(crate) fn calculate_scrollable_overflow(&self) {
        let scrollable_overflow = || {
            let Some(first_root_fragment) = self.root_fragments.first() else {
                return self.initial_containing_block;
            };

            let scrollable_overflow = self.root_fragments.iter().fold(
                self.initial_containing_block,
                |overflow, fragment| {
                    fragment
                        .calculate_scrollable_overflow_for_parent()
                        .union(&overflow)
                },
            );

            // Assuming that the first fragment is the root element, ensure that
            // scrollable overflow that is unreachable is not included in the final
            // rectangle. See
            // <https://drafts.csswg.org/css-overflow/#scrolling-direction>.
            let first_root_fragment = match first_root_fragment {
                Fragment::Box(fragment) | Fragment::Float(fragment) => fragment.borrow(),
                _ => return scrollable_overflow,
            };
            if !first_root_fragment.is_root_element() {
                return scrollable_overflow;
            }
            first_root_fragment.clip_wholly_unreachable_scrollable_overflow(
                scrollable_overflow,
                self.initial_containing_block,
            )
        };

        self.scrollable_overflow.set(Some(scrollable_overflow()))
    }

    pub(crate) fn find<T>(
        &self,
        mut process_func: impl FnMut(&Fragment, usize, &PhysicalRect<Au>) -> Option<T>,
    ) -> Option<T> {
        let info = ContainingBlockManager {
            for_non_absolute_descendants: &self.initial_containing_block,
            for_absolute_descendants: None,
            for_absolute_and_fixed_descendants: &self.initial_containing_block,
        };
        self.root_fragments
            .iter()
            .find_map(|child| child.find(&info, 0, &mut process_func))
    }

    /// Find the `<body>` element's [`Fragment`], if it exists in this [`FragmentTree`].
    pub(crate) fn body_fragment(&self) -> Option<ArcRefCell<BoxFragment>> {
        fn find_body(children: &[Fragment]) -> Option<ArcRefCell<BoxFragment>> {
            children.iter().find_map(|fragment| {
                match fragment {
                    Fragment::Box(box_fragment) | Fragment::Float(box_fragment) => {
                        let borrowed_box_fragment = box_fragment.borrow();
                        if borrowed_box_fragment.is_body_element_of_html_element_root() {
                            return Some(box_fragment.clone());
                        }

                        // The fragment for the `<body>` element is typically a child of the root (though,
                        // not if it's absolutely positioned), so we need to recurse into the children of
                        // the root to find it.
                        //
                        // Additionally, recurse into any anonymous fragments, as the `<body>` fragment may
                        // have created anonymous parents (for instance by creating an inline formatting context).
                        if borrowed_box_fragment.is_root_element() ||
                            borrowed_box_fragment.base.is_anonymous()
                        {
                            find_body(&borrowed_box_fragment.children)
                        } else {
                            None
                        }
                    },
                    Fragment::Positioning(positioning_context)
                        if positioning_context.borrow().base.is_anonymous() =>
                    {
                        // If the `<body>` element is a `display: inline` then it might be nested inside of a
                        // `PositioningFragment` for the purposes of putting it on the first line of the implied
                        // inline formatting context.
                        find_body(&positioning_context.borrow().children)
                    },
                    _ => None,
                }
            })
        }

        find_body(&self.root_fragments)
    }
}
