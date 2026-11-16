//! The `<sub>` handler; see the "Subscript" section of
//! `translation_approach.md`.
//!
//! Whether a single `~` opens or closes a subscript depends on the characters
//! emitted on both sides of it, which a handler cannot see. So, as for
//! emphasis, the handler writes Markdown only where its own content guarantees
//! the delimiters: when the content has whitespace at both ends, which moves
//! outside the tildes.

use std::{cell::Cell, rc::Rc};

use markup5ever_rcdom::{Node, NodeData};

use crate::{
    Context, Element,
    element_handler::{
        HandlerResult, Handlers,
        element_util::{
            escape_html_block_blank_lines, serialize_element_result, serialize_subtree_html,
        },
    },
    options::TranslationMode,
    util::{
        node::{get_parent_node, is_block_element, is_type_1_element},
        text::{concat_strings, frame_as_block, trim_newlines},
    },
};

thread_local! {
    /// How many subscripts' content the walk is inside; see [`in_subscript`].
    static SUBSCRIPT_DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// Whether text being walked is inside a subscript's content, where every `~`
/// of a run of three or more is escaped.
pub(crate) fn in_subscript() -> bool {
    SUBSCRIPT_DEPTH.with(|depth| depth.get() > 0)
}

struct SubscriptContentGuard;

impl SubscriptContentGuard {
    fn enter() -> Self {
        SUBSCRIPT_DEPTH.with(|depth| depth.set(depth.get() + 1));
        SubscriptContentGuard
    }
}

impl Drop for SubscriptContentGuard {
    fn drop(&mut self) {
        SUBSCRIPT_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

/// Whether `node` has a descendant which ends the paragraph a subscript is read
/// in: a `<pre>`, or an element with a CommonMark type 6 HTML block tag name.
pub(crate) fn has_block_descendant(node: &Rc<Node>) -> bool {
    node.children.borrow().iter().any(|child| {
        let is_block = match &child.data {
            NodeData::Element { name, .. } => {
                let tag = name.local.as_ref();
                tag == "pre" || (is_block_element(tag) && !is_type_1_element(tag))
            }
            _ => false,
        };
        is_block || has_block_descendant(child)
    })
}

fn is_sub_element(node: &Node) -> bool {
    matches!(&node.data, NodeData::Element { name, .. } if *name.local == *"sub")
}

fn is_inside_sub(node: &Rc<Node>) -> bool {
    let mut current = get_parent_node(node);
    while let Some(parent) = current {
        if is_sub_element(&parent) {
            return true;
        }
        current = get_parent_node(&parent);
    }
    false
}

pub(super) fn sub_handler(handlers: &dyn Handlers, element: &Element) -> Option<HandlerResult> {
    let is_faithful = handlers.options().translation_mode == TranslationMode::Faithful;

    // Text inside a `<pre>` or `<code>` stays literal; faithful mode's
    // serialization makes the enclosing code go out as HTML.
    if element.context.literal {
        return Some(if is_faithful {
            serialize_element_result(handlers, element)
        } else {
            handlers.walk_children(element.node, element.context)
        });
    }

    // Step 1 of either mode.
    if has_block_descendant(element.node) {
        if !is_faithful {
            let mut result = handlers.walk_children(element.node, element.context);
            // An inline context holds a single line; trimmed as for a `<span>`.
            if element.context.is_inline() {
                trim_newlines(&mut result.content);
            }
            return Some(result);
        }
        if element.context.is_block() {
            return Some(HandlerResult::html(serialize_as_html_block(element)));
        }
    }
    let is_nested = is_inside_sub(element.node);
    if is_faithful && (!element.attrs.is_empty() || is_nested) {
        return Some(serialize_element_result(handlers, element));
    }

    let content = {
        let _guard = SubscriptContentGuard::enter();
        handlers.walk_children_content(element.node, Context::INLINE)
    };
    if is_nested {
        return Some(content.into());
    }

    let (lead, trimmed, trail) = split_edge_whitespace(&content, is_faithful);
    let is_markdown = !trimmed.is_empty()
        && !lead.is_empty()
        && !trail.is_empty()
        && !has_unescaped_tilde(trimmed);
    Some(if is_markdown {
        concat_strings!(lead, "~", trimmed, "~", trail).into()
    } else if is_faithful {
        // The content already walked: walking the children again would, for
        // one, collect a referenced link twice. The element has no attributes
        // here, so its open tag is bare.
        HandlerResult::html(concat_strings!("<sub>", content, "</sub>"))
    } else {
        content.into()
    })
}

/// Splits `content` into its leading whitespace, the remaining content, and its
/// trailing whitespace, with whitespace being [`char::is_whitespace`]. In Pure
/// mode a hard line break at either end moves out with the whitespace: a
/// `\⏎` break leaves its backslash behind once the line ending is trimmed.
fn split_edge_whitespace(content: &str, is_faithful: bool) -> (&str, &str, &str) {
    let mut start = content.len() - content.trim_start().len();
    if !is_faithful {
        while content[start..].starts_with("\\\n") {
            start += 2;
            start = content.len() - content[start..].trim_start().len();
        }
    }
    let mut end = start + content[start..].trim_end().len();
    if !is_faithful && content[end..].starts_with('\n') {
        let body = &content[start..end];
        let backslashes = body.len() - body.trim_end_matches('\\').len();
        if !backslashes.is_multiple_of(2) {
            // The backslash of a `\⏎` break, and any whitespace before it.
            end = start + content[start..end - 1].trim_end().len();
        }
    }
    (&content[..start], &content[start..end], &content[end..])
}

/// Whether `text` holds a `~` which an odd run of backslashes does not escape.
fn has_unescaped_tilde(text: &str) -> bool {
    let mut backslashes: usize = 0;
    for ch in text.chars() {
        if ch == '\\' {
            backslashes += 1;
            continue;
        }
        if ch == '~' && backslashes.is_multiple_of(2) {
            return true;
        }
        backslashes = 0;
    }
    false
}

/// Faithful step 1: an HTML block holding the element's subtree as written, with a line
/// ending after `<sub>` so that the line opens a type 7 block, since `sub` is
/// not a type 6 name, and blank lines encoded as for a type 6 block.
fn serialize_as_html_block(element: &Element) -> String {
    let html = serialize_subtree_html(element);
    let open_tag_end = html.find('>').map_or(0, |index| index + 1);
    let mut block = String::with_capacity(html.len() + 1);
    block.push_str(&html[..open_tag_end]);
    block.push('\n');
    block.push_str(&html[open_tag_end..]);
    frame_as_block(&escape_html_block_blank_lines(block))
}

#[cfg(test)]
mod tests {
    use super::{has_unescaped_tilde, split_edge_whitespace};

    #[test]
    fn splits_edge_whitespace_and_pure_hard_breaks() {
        assert_eq!((" ", "a", " "), split_edge_whitespace(" a ", true));
        assert_eq!(("", "a\\", "\n"), split_edge_whitespace("a\\\n", true));
        assert_eq!(("", "a", "\\\n"), split_edge_whitespace("a\\\n", false));
        assert_eq!(("", "a\\\\", "\n"), split_edge_whitespace("a\\\\\n", false));
        assert_eq!(("\\\n", "a", ""), split_edge_whitespace("\\\na", false));
        assert_eq!(("", "a", "  \n"), split_edge_whitespace("a  \n", false));
    }

    #[test]
    fn finds_unescaped_tildes() {
        assert!(has_unescaped_tilde("a~b"));
        assert!(!has_unescaped_tilde(r"a\~b"));
        assert!(has_unescaped_tilde(r"a\\~b"));
        assert!(!has_unescaped_tilde("ab"));
    }
}
