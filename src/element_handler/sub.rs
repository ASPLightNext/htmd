//! The `<sub>` handler and the resolution of its delimiters; see the
//! "Subscript" section of `translation_approach.md`.
//!
//! Whether a single `~` opens or closes a subscript depends on the characters
//! emitted on both sides of it, which a handler cannot see: a parent writes its
//! own delimiters (`*`, `[`, `| `) only after its children are walked, and a
//! following sibling is walked later still. So the handler writes its Markdown
//! candidate, `lead~content~trail`, with a private-use marker just inside each
//! tilde, and [`resolve`] decides every candidate once the document is
//! assembled. The markers sit inside the tildes so that the candidate still
//! begins and ends with the characters its final text will: a parent checking
//! its content's first character, as the emphasis handler does for
//! punctuation, sees a `~` (or the edge whitespace) either way.

use std::{cell::RefCell, rc::Rc};

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
        unicode::is_unicode_punctuation,
    },
};

/// Supplementary Private Use Area-A holds the opening markers and Area-B the
/// closing ones, one code point per candidate, so a marker is one `char` like
/// the `~` it travels with.
const OPEN_MARKER_BASE: u32 = 0xF0000;
const CLOSE_MARKER_BASE: u32 = 0x100000;
/// U+FFFFE/U+FFFFF and U+10FFFE/U+10FFFF are noncharacters; stop short of them.
const MAX_CANDIDATES: usize = 0xFFFE;

/// One Markdown candidate awaiting [`resolve`].
struct Candidate {
    /// The whitespace step 4 placed before the opening `~`.
    lead: String,
    /// The whitespace step 4 placed after the closing `~`.
    trail: String,
    /// Whether a text sibling's own whitespace meets `lead` or `trail`. The walk
    /// compresses a text node's ASCII whitespace into single spaces and drops
    /// one of two spaces meeting at a boundary, so the space left outside the
    /// tildes is then the sibling's, and stays when the element falls back to
    /// HTML. Other whitespace, such as a non-breaking space, is never merged.
    lead_is_shared: bool,
    trail_is_shared: bool,
    /// Faithful mode's fallback: the element serialized as HTML with its edge
    /// whitespace inside. Pure mode falls back to the content alone.
    html: Option<String>,
}

/// The candidates of one conversion. Conversions nest (a handler may convert a
/// fragment), so each enters a frame of its own.
#[derive(Default)]
struct Frame {
    candidates: Vec<Candidate>,
    /// Whether the document already holds a marker character, which would be
    /// mistaken for one of ours; computed on the first candidate.
    document_has_marker_chars: Option<bool>,
}

thread_local! {
    static FRAMES: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) };
    /// How many subscripts' content the walk is inside; see [`in_subscript`].
    static SUBSCRIPT_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

/// The marker frame of one conversion; dropping it discards the frame.
pub(crate) struct ConversionScope(());

impl ConversionScope {
    pub(crate) fn enter() -> Self {
        FRAMES.with(|frames| frames.borrow_mut().push(Frame::default()));
        ConversionScope(())
    }
}

impl Drop for ConversionScope {
    fn drop(&mut self) {
        FRAMES.with(|frames| frames.borrow_mut().pop());
    }
}

/// Whether text being walked is inside a subscript's content, where every `~`
/// of a run of three or more is escaped, and every `^`.
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

fn open_marker(id: usize) -> char {
    char::from_u32(OPEN_MARKER_BASE + id as u32).expect("id is below MAX_CANDIDATES")
}

fn close_marker(id: usize) -> char {
    char::from_u32(CLOSE_MARKER_BASE + id as u32).expect("id is below MAX_CANDIDATES")
}

fn open_marker_id(ch: char) -> Option<usize> {
    let offset = (ch as u32).checked_sub(OPEN_MARKER_BASE)? as usize;
    (offset < MAX_CANDIDATES).then_some(offset)
}

fn is_marker_char(ch: char) -> bool {
    (OPEN_MARKER_BASE..=0x10FFFF).contains(&(ch as u32))
}

/// Whether `ch` is a marker this conversion issued, given `issued` markers of
/// each kind. A document holding a character of the marker range gets no
/// markers (see [`register`]), so once any is issued, every character of the
/// issued ids is one; before that, none is.
fn is_issued_marker(ch: char, issued: usize) -> bool {
    let code = ch as u32;
    let issued = issued as u32;
    (OPEN_MARKER_BASE..OPEN_MARKER_BASE + issued).contains(&code)
        || (CLOSE_MARKER_BASE..CLOSE_MARKER_BASE + issued).contains(&code)
}

/// The number of characters `text` will hold once its markers are resolved,
/// assuming each candidate stays Markdown. Column padding and setext underlines
/// count with this, so a subscript's markers do not widen them, while the
/// document's own characters all count.
pub(crate) fn resolved_char_count(text: &str) -> usize {
    let issued = FRAMES.with(|frames| {
        frames
            .borrow()
            .last()
            .map_or(0, |frame| frame.candidates.len())
    });
    if issued == 0 {
        return text.chars().count();
    }
    text.chars()
        .filter(|&ch| !is_issued_marker(ch, issued))
        .count()
}

/// Registers a candidate, returning its id, or `None` when no marker may be
/// issued: the frame is full, or the document already holds a marker character.
fn register(node: &Rc<Node>, candidate: Candidate) -> Option<usize> {
    FRAMES.with(|frames| {
        let mut frames = frames.borrow_mut();
        let frame = frames.last_mut()?;
        let has_marker_chars = *frame
            .document_has_marker_chars
            .get_or_insert_with(|| tree_has_marker_chars(&root_of(node)));
        if has_marker_chars || frame.candidates.len() >= MAX_CANDIDATES {
            return None;
        }
        frame.candidates.push(candidate);
        Some(frame.candidates.len() - 1)
    })
}

fn root_of(node: &Rc<Node>) -> Rc<Node> {
    let mut current = node.clone();
    while let Some(parent) = get_parent_node(&current) {
        current = parent;
    }
    current
}

/// Whether any text, comment, or attribute value under `node` holds a character
/// of the marker range; such a document gets no markers at all.
fn tree_has_marker_chars(node: &Rc<Node>) -> bool {
    let holds = |text: &str| text.chars().any(is_marker_char);
    let found = match &node.data {
        NodeData::Text { contents } => holds(&contents.borrow()),
        NodeData::Comment { contents } => holds(contents),
        NodeData::Element { attrs, .. } => attrs.borrow().iter().any(|attr| holds(&attr.value)),
        _ => false,
    };
    found || node.children.borrow().iter().any(tree_has_marker_chars)
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

/// Whether a `<sub>` sibling sits immediately before or after `node`, with no
/// node of any kind between them.
fn has_adjacent_sub_sibling(node: &Rc<Node>) -> bool {
    let Some(parent) = get_parent_node(node) else {
        return false;
    };
    let siblings = parent.children.borrow();
    let Some(index) = siblings
        .iter()
        .position(|sibling| Rc::ptr_eq(sibling, node))
    else {
        // A combined run (see `dom_walker::walk_children`) is not among its
        // parent's children.
        return false;
    };
    (index > 0 && is_sub_element(&siblings[index - 1]))
        || siblings
            .get(index + 1)
            .is_some_and(|next| is_sub_element(next))
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

    // Step 1, and Pure mode's rule for a block descendant, which comes before
    // combining (`dom_walker::walk_children` never combines such an element).
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
    if is_faithful
        && (!element.attrs.is_empty()
            || is_inside_sub(element.node)
            || has_adjacent_sub_sibling(element.node))
    {
        return Some(serialize_element_result(handlers, element));
    }

    let content = {
        let _guard = SubscriptContentGuard::enter();
        handlers.walk_children_content(element.node, Context::INLINE)
    };

    // Pure mode writes a subscript inside a subscript as its content alone.
    if !is_faithful && is_inside_sub(element.node) {
        return Some(content.into());
    }

    let (lead, trimmed, trail) = split_edge_whitespace(&content, is_faithful);

    // The element as HTML, holding the content already walked: walking the
    // children again would, for one, collect a referenced link twice. The
    // element has no attributes here (step 1), so its open tag is bare.
    let html = || concat_strings!("<sub>", content, "</sub>");

    // Step 2.
    if trimmed.is_empty() {
        return Some(if is_faithful {
            HandlerResult::html(html())
        } else {
            concat_strings!(lead, trail).into()
        });
    }

    // Step 3.
    if has_unescaped_tilde(trimmed) {
        return Some(if is_faithful {
            HandlerResult::html(html())
        } else {
            content.into()
        });
    }

    // Steps 4-6: the candidate, decided by `resolve`.
    let html = is_faithful.then(html);
    let (prev_text, next_text) = sibling_texts(element.node);
    let candidate = Candidate {
        lead: lead.to_string(),
        trail: trail.to_string(),
        lead_is_shared: lead.starts_with(' ')
            && prev_text.is_some_and(|text| text.ends_with(|ch: char| ch.is_ascii_whitespace())),
        trail_is_shared: trail.ends_with(' ')
            && next_text.is_some_and(|text| text.starts_with(|ch: char| ch.is_ascii_whitespace())),
        html,
    };
    let fallback = candidate.html.clone();
    let Some(id) = register(element.node, candidate) else {
        // Without a marker the neighbors cannot be checked, so fall back as
        // step 5 would for a rejected candidate.
        return Some(match fallback {
            Some(html) => HandlerResult::html(html),
            None => content.into(),
        });
    };
    let mut markdown = String::with_capacity(content.len() + 2 * '~'.len_utf8() + 8);
    markdown.push_str(lead);
    markdown.push('~');
    markdown.push(open_marker(id));
    markdown.push_str(trimmed);
    markdown.push(close_marker(id));
    markdown.push('~');
    markdown.push_str(trail);
    Some(markdown.into())
}

/// The text of the text nodes immediately before and after `node`, if any.
fn sibling_texts(node: &Rc<Node>) -> (Option<String>, Option<String>) {
    let Some(parent) = get_parent_node(node) else {
        return (None, None);
    };
    let siblings = parent.children.borrow();
    let Some(index) = siblings
        .iter()
        .position(|sibling| Rc::ptr_eq(sibling, node))
    else {
        return (None, None);
    };
    let text_of = |sibling: Option<&Rc<Node>>| match sibling.map(|sibling| &sibling.data) {
        Some(NodeData::Text { contents }) => Some(contents.borrow().to_string()),
        _ => None,
    };
    (
        text_of(index.checked_sub(1).and_then(|before| siblings.get(before))),
        text_of(siblings.get(index + 1)),
    )
}

/// Splits `content` into its leading whitespace, the trimmed content, and its
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

/// Step 1: an HTML block holding the element's subtree as written, with a line
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

/// Decides every candidate in `output`, in document order, against the
/// characters emitted around its tildes (step 5), writing either the Markdown
/// or the fallback. A candidate's neighbor is final when it comes before it;
/// one after it is read as written, so a following candidate counts as its
/// opening `~`.
pub(crate) fn resolve(output: &mut String) {
    let candidates = FRAMES.with(|frames| {
        frames
            .borrow_mut()
            .last_mut()
            .map(|frame| std::mem::take(&mut frame.candidates))
            .unwrap_or_default()
    });
    if candidates.is_empty() {
        return;
    }
    let source = std::mem::take(output);
    let mut result = String::with_capacity(source.len());
    let mut rest = source.as_str();
    while let Some((open_index, id)) = rest
        .char_indices()
        .find_map(|(index, ch)| open_marker_id(ch).map(|id| (index, id)))
    {
        let close = close_marker(id);
        let content_start = open_index + open_marker(id).len_utf8();
        let Some(close_offset) = rest[content_start..].find(close) else {
            // A marker whose partner was not written; keep the text around it.
            result.push_str(&rest[..open_index]);
            rest = &rest[content_start..];
            continue;
        };
        let content_end = content_start + close_offset;
        let after_close = content_end + close.len_utf8();
        result.push_str(&rest[..open_index]);
        let content = &rest[content_start..content_end];
        let after = &rest[after_close..];
        let (Some(before), Some(after)) = (result.strip_suffix('~'), after.strip_prefix('~'))
        else {
            // Something rewrote the tildes; leave the text as it is.
            result.push_str(content);
            rest = after;
            continue;
        };
        let before_len = before.len();
        let candidate = candidates.get(id);
        if is_valid_subscript(&result[..before_len], content, after) {
            result.push_str(content);
            result.push('~');
        } else {
            result.truncate(before_len);
            match candidate.and_then(|candidate| candidate.html.as_deref()) {
                Some(html) => {
                    let candidate = candidate.expect("html implies a candidate");
                    // The edge whitespace goes back inside the element. Of a
                    // run whose space a text sibling shares, the walk merged
                    // only that one space, its first (leading) or last
                    // (trailing) character; it stays outside, and the rest of
                    // the run is the element's.
                    let lead = if candidate.lead_is_shared {
                        &candidate.lead[1..]
                    } else {
                        candidate.lead.as_str()
                    };
                    if result.ends_with(lead) {
                        result.truncate(result.len() - lead.len());
                    }
                    result.push_str(html);
                    let trail = if candidate.trail_is_shared {
                        &candidate.trail[..candidate.trail.len() - 1]
                    } else {
                        candidate.trail.as_str()
                    };
                    rest = after.strip_prefix(trail).unwrap_or(after);
                    continue;
                }
                None => result.push_str(content),
            }
        }
        rest = after;
    }
    result.push_str(rest);
    *output = result;
}

/// Step 5: whether `~content~` between `before` and `after` reads as a
/// subscript, by the single-tilde rules of the "Subscript" section.
fn is_valid_subscript(before: &str, content: &str, after: &str) -> bool {
    let is_space = |ch: char| ch.is_whitespace();
    let is_space_or_punct = |ch: char| ch.is_whitespace() || is_unicode_punctuation(ch);
    let prev = before.chars().next_back();
    let next = after.chars().next();
    let (Some(first), Some(last)) = (content.chars().next(), content.chars().next_back()) else {
        return false;
    };

    let opens = !is_space(first) && prev.is_none_or(is_space_or_punct);
    let closes = !is_space(last) && (next.is_none_or(is_space_or_punct) || last == '~');
    // A `~` or `\` outside either tilde would merge with it or escape it.
    let unescaped_before = matches!(prev, Some('~' | '\\')) && {
        let without_prev = &before[..before.len() - 1];
        let backslashes = without_prev.len() - without_prev.trim_end_matches('\\').len();
        backslashes.is_multiple_of(2)
    };
    let unescaped_after = matches!(next, Some('~' | '\\'));
    opens && closes && !unescaped_before && !unescaped_after
}

/// A copy of the `<sub>` elements `run` as one element holding all of their
/// children, which Pure mode writes in place of the run.
pub(crate) fn combine_run(parent: &Rc<Node>, run: &[Rc<Node>]) -> Rc<Node> {
    let NodeData::Element {
        name,
        template_contents,
        mathml_annotation_xml_integration_point,
        ..
    } = &run[0].data
    else {
        unreachable!("a run holds only elements")
    };
    let combined = Node::new(NodeData::Element {
        name: name.clone(),
        attrs: RefCell::new(Vec::new()),
        template_contents: RefCell::new(template_contents.borrow().clone()),
        mathml_annotation_xml_integration_point: *mathml_annotation_xml_integration_point,
    });
    combined.parent.set(Some(Rc::downgrade(parent)));
    // The children keep their own parents, so the ancestors they look up are
    // the elements they were written in.
    combined
        .children
        .borrow_mut()
        .extend(run.iter().flat_map(|sub| sub.children.borrow().clone()));
    combined
}

/// Whether `node` is a `<sub>` which Pure mode may combine with an adjacent one.
pub(crate) fn is_combinable_sub(node: &Rc<Node>) -> bool {
    is_sub_element(node) && !has_block_descendant(node)
}

#[cfg(test)]
mod tests {
    use super::{has_unescaped_tilde, is_valid_subscript, split_edge_whitespace};

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

    #[test]
    fn checks_delimiter_neighbors() {
        assert!(is_valid_subscript("", "a", ""));
        assert!(is_valid_subscript("x ", "i", " y"));
        assert!(!is_valid_subscript("H", "2", "O"));
        assert!(is_valid_subscript("(", "i", "<span>"));
        assert!(!is_valid_subscript("(", "i", "O)"));
        assert!(is_valid_subscript("x ", r"a\~", "b"));
        assert!(!is_valid_subscript("x~", "a", ""));
        assert!(is_valid_subscript(r"x\~", "a", ""));
        assert!(!is_valid_subscript("x\\", "a", ""));
        assert!(is_valid_subscript("x\\\\", "a", ""));
        assert!(!is_valid_subscript("", "a", "~b"));
        assert!(!is_valid_subscript("", "a", "\\*"));
        assert!(is_valid_subscript("x\u{a0}", "a", "\u{a0}y"));
    }
}
