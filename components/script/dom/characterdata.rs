/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

//! DOM bindings for `CharacterData`.
use std::cell::LazyCell;

use dom_struct::dom_struct;
use script_bindings::codegen::InheritTypes::{CharacterDataTypeId, NodeTypeId, TextTypeId};

use crate::dom::bindings::cell::{DomRefCell, Ref};
use crate::dom::bindings::codegen::Bindings::CharacterDataBinding::CharacterDataMethods;
use crate::dom::bindings::codegen::Bindings::NodeBinding::Node_Binding::NodeMethods;
use crate::dom::bindings::codegen::Bindings::ProcessingInstructionBinding::ProcessingInstructionMethods;
use crate::dom::bindings::codegen::UnionTypes::NodeOrString;
use crate::dom::bindings::error::{Error, ErrorResult, Fallible};
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::{DomRoot, LayoutDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::cdatasection::CDATASection;
use crate::dom::comment::Comment;
use crate::dom::document::Document;
use crate::dom::element::Element;
use crate::dom::mutationobserver::{Mutation, MutationObserver};
use crate::dom::node::{ChildrenMutation, Node, NodeDamage};
use crate::dom::processinginstruction::ProcessingInstruction;
use crate::dom::text::Text;
use crate::dom::virtualmethods::vtable_for;
use crate::script_runtime::CanGc;

// https://dom.spec.whatwg.org/#characterdata
#[dom_struct]
pub(crate) struct CharacterData {
    node: Node,
    data: DomRefCell<DOMString>,
}

impl CharacterData {
    pub(crate) fn new_inherited(data: DOMString, document: &Document) -> CharacterData {
        CharacterData {
            node: Node::new_inherited(document),
            data: DomRefCell::new(data),
        }
    }

    pub(crate) fn clone_with_data(
        &self,
        data: DOMString,
        document: &Document,
        can_gc: CanGc,
    ) -> DomRoot<Node> {
        match self.upcast::<Node>().type_id() {
            NodeTypeId::CharacterData(CharacterDataTypeId::Comment) => {
                DomRoot::upcast(Comment::new(data, document, None, can_gc))
            },
            NodeTypeId::CharacterData(CharacterDataTypeId::ProcessingInstruction) => {
                let pi = self.downcast::<ProcessingInstruction>().unwrap();
                DomRoot::upcast(ProcessingInstruction::new(
                    pi.Target(),
                    data,
                    document,
                    can_gc,
                ))
            },
            NodeTypeId::CharacterData(CharacterDataTypeId::Text(TextTypeId::CDATASection)) => {
                DomRoot::upcast(CDATASection::new(data, document, can_gc))
            },
            NodeTypeId::CharacterData(CharacterDataTypeId::Text(TextTypeId::Text)) => {
                DomRoot::upcast(Text::new(data, document, can_gc))
            },
            _ => unreachable!(),
        }
    }

    #[inline]
    pub(crate) fn data(&self) -> Ref<DOMString> {
        self.data.borrow()
    }

    #[inline]
    pub(crate) fn append_data(&self, data: &str) {
        self.queue_mutation_record();
        self.data.borrow_mut().push_str(data);
        self.content_changed();
    }

    fn content_changed(&self) {
        let node = self.upcast::<Node>();
        node.dirty(NodeDamage::Other);

        // If this is a Text node, we might need to re-parse (say, if our parent
        // is a <style> element.) We don't need to if this is a Comment or
        // ProcessingInstruction.
        if self.is::<Text>() {
            if let Some(parent_node) = node.GetParentNode() {
                let mutation = ChildrenMutation::ChangeText;
                vtable_for(&parent_node).children_changed(&mutation);
            }
        }
    }

    // Queue a MutationObserver record before changing the content.
    fn queue_mutation_record(&self) {
        let mutation = LazyCell::new(|| Mutation::CharacterData {
            old_value: self.data.borrow().clone(),
        });
        MutationObserver::queue_a_mutation_record(self.upcast::<Node>(), mutation);
    }
}

impl CharacterDataMethods<crate::DomTypeHolder> for CharacterData {
    // https://dom.spec.whatwg.org/#dom-characterdata-data
    fn Data(&self) -> DOMString {
        self.data.borrow().clone()
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-data
    fn SetData(&self, data: DOMString) {
        self.queue_mutation_record();
        let old_length = self.Length();
        let new_length = data.encode_utf16().count() as u32;
        *self.data.borrow_mut() = data;
        self.content_changed();
        let node = self.upcast::<Node>();
        node.ranges()
            .replace_code_units(node, 0, old_length, new_length);
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-length
    fn Length(&self) -> u32 {
        self.data.borrow().encode_utf16().count() as u32
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-substringdata
    fn SubstringData(&self, offset: u32, count: u32) -> Fallible<DOMString> {
        let data = self.data.borrow();
        // Step 1.
        let mut substring = String::new();
        let remaining = match split_at_utf16_code_unit_offset(&data, offset) {
            Ok((_, astral, s)) => {
                // As if we had split the UTF-16 surrogate pair in half
                // and then transcoded that to UTF-8 lossily,
                // since our DOMString is currently strict UTF-8.
                if astral.is_some() {
                    substring += "\u{FFFD}";
                }
                s
            },
            // Step 2.
            Err(()) => return Err(Error::IndexSize),
        };
        match split_at_utf16_code_unit_offset(remaining, count) {
            // Steps 3.
            Err(()) => substring += remaining,
            // Steps 4.
            Ok((s, astral, _)) => {
                substring += s;
                // As if we had split the UTF-16 surrogate pair in half
                // and then transcoded that to UTF-8 lossily,
                // since our DOMString is currently strict UTF-8.
                if astral.is_some() {
                    substring += "\u{FFFD}";
                }
            },
        };
        Ok(DOMString::from(substring))
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-appenddatadata
    fn AppendData(&self, data: DOMString) {
        // FIXME(ajeffrey): Efficient append on DOMStrings?
        self.append_data(&data);
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-insertdataoffset-data
    fn InsertData(&self, offset: u32, arg: DOMString) -> ErrorResult {
        self.ReplaceData(offset, 0, arg)
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-deletedataoffset-count
    fn DeleteData(&self, offset: u32, count: u32) -> ErrorResult {
        self.ReplaceData(offset, count, DOMString::new())
    }

    // https://dom.spec.whatwg.org/#dom-characterdata-replacedata
    fn ReplaceData(&self, offset: u32, count: u32, arg: DOMString) -> ErrorResult {
        let mut new_data;
        {
            let data = self.data.borrow();
            let prefix;
            let replacement_before;
            let remaining;
            match split_at_utf16_code_unit_offset(&data, offset) {
                Ok((p, astral, r)) => {
                    prefix = p;
                    // As if we had split the UTF-16 surrogate pair in half
                    // and then transcoded that to UTF-8 lossily,
                    // since our DOMString is currently strict UTF-8.
                    replacement_before = if astral.is_some() { "\u{FFFD}" } else { "" };
                    remaining = r;
                },
                // Step 2.
                Err(()) => return Err(Error::IndexSize),
            };
            let replacement_after;
            let suffix;
            match split_at_utf16_code_unit_offset(remaining, count) {
                // Steps 3.
                Err(()) => {
                    replacement_after = "";
                    suffix = "";
                },
                Ok((_, astral, s)) => {
                    // As if we had split the UTF-16 surrogate pair in half
                    // and then transcoded that to UTF-8 lossily,
                    // since our DOMString is currently strict UTF-8.
                    replacement_after = if astral.is_some() { "\u{FFFD}" } else { "" };
                    suffix = s;
                },
            };
            // Step 4: Mutation observers.
            self.queue_mutation_record();

            // Step 5 to 7.
            new_data = String::with_capacity(
                prefix.len() +
                    replacement_before.len() +
                    arg.len() +
                    replacement_after.len() +
                    suffix.len(),
            );
            new_data.push_str(prefix);
            new_data.push_str(replacement_before);
            new_data.push_str(&arg);
            new_data.push_str(replacement_after);
            new_data.push_str(suffix);
        }
        *self.data.borrow_mut() = DOMString::from(new_data);
        self.content_changed();
        // Steps 8-11.
        let node = self.upcast::<Node>();
        node.ranges()
            .replace_code_units(node, offset, count, arg.encode_utf16().count() as u32);
        Ok(())
    }

    // https://dom.spec.whatwg.org/#dom-childnode-before
    fn Before(&self, nodes: Vec<NodeOrString>, can_gc: CanGc) -> ErrorResult {
        self.upcast::<Node>().before(nodes, can_gc)
    }

    // https://dom.spec.whatwg.org/#dom-childnode-after
    fn After(&self, nodes: Vec<NodeOrString>, can_gc: CanGc) -> ErrorResult {
        self.upcast::<Node>().after(nodes, can_gc)
    }

    // https://dom.spec.whatwg.org/#dom-childnode-replacewith
    fn ReplaceWith(&self, nodes: Vec<NodeOrString>, can_gc: CanGc) -> ErrorResult {
        self.upcast::<Node>().replace_with(nodes, can_gc)
    }

    // https://dom.spec.whatwg.org/#dom-childnode-remove
    fn Remove(&self, can_gc: CanGc) {
        let node = self.upcast::<Node>();
        node.remove_self(can_gc);
    }

    // https://dom.spec.whatwg.org/#dom-nondocumenttypechildnode-previouselementsibling
    fn GetPreviousElementSibling(&self) -> Option<DomRoot<Element>> {
        self.upcast::<Node>()
            .preceding_siblings()
            .filter_map(DomRoot::downcast)
            .next()
    }

    // https://dom.spec.whatwg.org/#dom-nondocumenttypechildnode-nextelementsibling
    fn GetNextElementSibling(&self) -> Option<DomRoot<Element>> {
        self.upcast::<Node>()
            .following_siblings()
            .filter_map(DomRoot::downcast)
            .next()
    }
}

pub(crate) trait LayoutCharacterDataHelpers<'dom> {
    fn data_for_layout(self) -> &'dom str;
}

impl<'dom> LayoutCharacterDataHelpers<'dom> for LayoutDom<'dom, CharacterData> {
    #[allow(unsafe_code)]
    #[inline]
    fn data_for_layout(self) -> &'dom str {
        unsafe { self.unsafe_get().data.borrow_for_layout() }
    }
}

/// Split the given string at the given position measured in UTF-16 code units from the start.
///
/// * `Err(())` indicates that `offset` if after the end of the string
/// * `Ok((before, None, after))` indicates that `offset` is between Unicode code points.
///   The two string slices are such that:
///   `before == s.to_utf16()[..offset].to_utf8()` and
///   `after == s.to_utf16()[offset..].to_utf8()`
/// * `Ok((before, Some(ch), after))` indicates that `offset` is "in the middle"
///   of a single Unicode code point that would be represented in UTF-16 by a surrogate pair
///   of two 16-bit code units.
///   `ch` is that code point.
///   The two string slices are such that:
///   `before == s.to_utf16()[..offset - 1].to_utf8()` and
///   `after == s.to_utf16()[offset + 1..].to_utf8()`
fn split_at_utf16_code_unit_offset(s: &str, offset: u32) -> Result<(&str, Option<char>, &str), ()> {
    let mut code_units = 0;
    for (i, c) in s.char_indices() {
        if code_units == offset {
            let (a, b) = s.split_at(i);
            return Ok((a, None, b));
        }
        code_units += 1;
        if c > '\u{FFFF}' {
            if code_units == offset {
                debug_assert_eq!(c.len_utf8(), 4);
                warn!("Splitting a surrogate pair in CharacterData API.");
                return Ok((&s[..i], Some(c), &s[i + c.len_utf8()..]));
            }
            code_units += 1;
        }
    }
    if code_units == offset {
        Ok((s, None, ""))
    } else {
        Err(())
    }
}
