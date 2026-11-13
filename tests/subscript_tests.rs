//! Tests of the "Subscript" section of `translation_approach.md`. Every
//! conversion here is htmd's own output. Where a test also renders that output,
//! it reads it back with pulldown-cmark 0.13.4 and
//! `ENABLE_SUBSCRIPT | ENABLE_STRIKETHROUGH`, the reader the section assumes;
//! a block fallback is checked once more by parsing the rendered HTML into a
//! DOM, since a `<p>` the renderer writes around a block can rearrange it.

use htmd::{
    HtmlToMarkdown,
    options::{BrStyle, HeadingStyle, LinkStyle, Options, TranslationMode},
};
use pretty_assertions::assert_eq;
use pulldown_cmark::Options as CommonMarkOptions;
use scraper::Html;

mod common;
use common::{convert_faithful, convert_faithful_options, convert_pure, render_markdown};

fn convert_pure_options(html: &str, options: Options) -> String {
    HtmlToMarkdown::builder()
        .options(Options {
            translation_mode: TranslationMode::Pure,
            ..options
        })
        .build()
        .convert(html)
        .unwrap()
}

/// `markdown` read back by the reader the section assumes.
fn render(markdown: &str) -> String {
    render_with(markdown, CommonMarkOptions::empty())
}

/// [`render`], with the reader also enabling `extra`.
fn render_with(markdown: &str, extra: CommonMarkOptions) -> String {
    render_markdown(
        markdown,
        CommonMarkOptions::ENABLE_SUBSCRIPT | CommonMarkOptions::ENABLE_STRIKETHROUGH | extra,
    )
}

fn convert_setext(html: &str) -> String {
    convert_faithful_options(
        html,
        Options {
            heading_style: HeadingStyle::Setex,
            ..Default::default()
        },
    )
    .unwrap()
}

/// The `<body>` of the DOM an HTML5 parser builds from `html`.
fn dom(html: &str) -> String {
    let document = Html::parse_document(html);
    let body = scraper::Selector::parse("body").unwrap();
    document.select(&body).next().unwrap().inner_html()
}

fn faithful(html: &str) -> String {
    convert_faithful(html).unwrap()
}

fn pure(html: &str) -> String {
    convert_pure(html).unwrap()
}

/// The section's table, row by row, in faithful mode.
#[test]
fn table_rows_faithful() {
    let rows = [
        ("<sub>a</sub>", "~a~"),
        ("x <sub>i</sub> y", "x ~i~ y"),
        ("H<sub>2</sub>O", "H<sub>2</sub>O"),
        ("a~b", r"a\~b"),
        ("<sub>a~b</sub>", r"~a\~b~"),
        ("x<sub> a </sub>y", "x ~a~ y"),
        ("a<sub> </sub>b", "a<sub> </sub>b"),
        ("a<sub></sub>b", "a<sub></sub>b"),
        (r#"<sub class="x">a</sub>"#, r#"<sub class="x">a</sub>"#),
        ("<sub><em>a</em></sub>", "~*a*~"),
        ("<sub><sub>a</sub></sub>", "~<sub>a</sub>~"),
        ("<sub>a</sub><sub>b</sub>", "<sub>a</sub><sub>b</sub>"),
        ("H<sub> 2</sub>O", "H<sub> 2</sub>O"),
        ("H<sub>2 </sub>O", "H<sub>2 </sub>O"),
        ("(<sub>i</sub><span>O</span>)", "(~i~<span>O</span>)"),
        (
            "<sub><em>a</em></sub><sub>b</sub>",
            "<sub>*a*</sub><sub>b</sub>",
        ),
        (
            "<sub><p>a</p><p>b</p></sub>",
            "<sub>\n<p>a</p><p>b</p></sub>",
        ),
    ];
    for (html, expected) in rows {
        assert_eq!(expected, faithful(html), "row {html:?}");
    }
}

/// The same rows in pure mode, which writes no HTML: what faithful mode
/// serializes is written as its content, and adjacent subscripts combine.
#[test]
fn table_rows_pure() {
    let rows = [
        ("<sub>a</sub>", "~a~"),
        ("x <sub>i</sub> y", "x ~i~ y"),
        ("H<sub>2</sub>O", "H2O"),
        ("a~b", r"a\~b"),
        ("<sub>a~b</sub>", r"~a\~b~"),
        ("x<sub> a </sub>y", "x ~a~ y"),
        ("a<sub> </sub>b", "a b"),
        ("a<sub></sub>b", "ab"),
        (r#"<sub class="x">a</sub>"#, "~a~"),
        ("<sub><em>a</em></sub>", "~*a*~"),
        ("<sub><sub>a</sub></sub>", "~a~"),
        ("<sub>a</sub><sub>b</sub>", "~ab~"),
        ("H<sub> 2</sub>O", "H 2O"),
        ("H<sub>2 </sub>O", "H2 O"),
        ("(<sub>i</sub><span>O</span>)", "(iO)"),
        ("<sub><em>a</em></sub><sub>b</sub>", "~*a*b~"),
        ("<sub><p>a</p><p>b</p></sub>", "a\n\nb"),
    ];
    for (html, expected) in rows {
        assert_eq!(expected, pure(html), "row {html:?}");
    }
}

/// Each Markdown subscript the table writes reads back as the `<sub>` it came
/// from, and each fallback as its HTML.
#[test]
fn table_rows_render() {
    for (html, rendered) in [
        ("<sub>a</sub>", "<p><sub>a</sub></p>\n"),
        ("x <sub>i</sub> y", "<p>x <sub>i</sub> y</p>\n"),
        ("H<sub>2</sub>O", "<p>H<sub>2</sub>O</p>\n"),
        ("a~b", "<p>a~b</p>\n"),
        ("<sub>a~b</sub>", "<p><sub>a~b</sub></p>\n"),
        ("x<sub> a </sub>y", "<p>x <sub>a</sub> y</p>\n"),
        ("<sub><em>a</em></sub>", "<p><sub><em>a</em></sub></p>\n"),
        (
            "<sub><sub>a</sub></sub>",
            "<p><sub><sub>a</sub></sub></p>\n",
        ),
        (
            "(<sub>i</sub><span>O</span>)",
            "<p>(<sub>i</sub><span>O</span>)</p>\n",
        ),
        (
            "<sub><em>a</em></sub><sub>b</sub>",
            "<p><sub><em>a</em></sub><sub>b</sub></p>\n",
        ),
    ] {
        assert_eq!(rendered, render(&faithful(html)), "row {html:?}");
    }
}

/// The candidate's neighbors are the characters emitted around it, including
/// those a parent writes and those of a sibling element in the current mode.
#[test]
fn delimiters_check_emitted_neighbors() {
    // A `<span>` is HTML in faithful mode, so `<` follows the closing tilde; in
    // pure mode its text `O` does, which blocks it.
    assert_eq!(
        "(~i~<span>O</span>)",
        faithful("(<sub>i</sub><span>O</span>)")
    );
    assert_eq!("(iO)", pure("(<sub>i</sub><span>O</span>)"));
    // Parents' delimiters: a table cell's padding, a heading marker, link
    // brackets, emphasis, and a list item marker.
    assert_eq!(
        "| h   |\n| --- |\n| ~i~ |",
        faithful(
            "<table><thead><tr><th>h</th></tr></thead><tbody><tr><td><sub>i</sub></td></tr></tbody></table>"
        )
    );
    assert_eq!("# ~i~", faithful("<h1><sub>i</sub></h1>"));
    assert_eq!("[~i~](u)", faithful(r#"<a href="u"><sub>i</sub></a>"#));
    assert_eq!("*~2~*", pure("<em><sub>2</sub></em>"));
    assert_eq!(
        "*   ~i~ item",
        faithful("<ul><li><sub>i</sub> item</li></ul>")
    );
    // Punctuation, symbols included, and a non-breaking space are boundaries.
    assert_eq!("x, ~i~, y", faithful("x, <sub>i</sub>, y"));
    assert_eq!("©~i~©", faithful("©<sub>i</sub>©"));
    assert_eq!("x\u{a0}~i~\u{a0}y", faithful("x&nbsp;<sub>i</sub>&nbsp;y"));
    // A letter on either side blocks the tilde next to it.
    assert_eq!("a<sub>i</sub> y", faithful("a<sub>i</sub> y"));
    assert_eq!("x <sub>i</sub>b", faithful("x <sub>i</sub>b"));
    for html in [
        "<p>x, <sub>i</sub>, y</p>",
        "<p>x&nbsp;<sub>i</sub>&nbsp;y</p>",
        "<h1><sub>i</sub></h1>",
    ] {
        assert!(render(&faithful(html)).contains("<sub>i</sub>"), "{html:?}");
    }
}

/// A `~` or `\` outside either tilde would merge with it or escape it; an
/// escaped one is ordinary punctuation.
#[test]
fn delimiters_reject_tilde_and_backslash_neighbors() {
    // An escaped tilde before the opener, and an escaped backslash.
    assert_eq!(r"\~~a~ x", faithful("~<sub>a</sub> x"));
    assert_eq!(r"x\\~a~ y", faithful(r"x\<sub>a</sub> y"));
    assert_eq!("<p>~<sub>a</sub> x</p>\n", render(r"\~~a~ x"));
    assert_eq!("<p>x\\<sub>a</sub> y</p>\n", render(r"x\\~a~ y"));
    // A backslash escape after the closer.
    assert_eq!(r"x <sub>a</sub>\[1\]", faithful("x <sub>a</sub>[1]"));
    assert_eq!(r"x a\[1\]", pure("x <sub>a</sub>[1]"));
    // A following subscript's opener is a tilde: the first falls back.
    assert_eq!("ab", pure("<sub>a</sub><span></span><sub>b</sub>"));
    assert_eq!(
        "~a~<span></span>~b~",
        faithful("<sub>a</sub><span></span><sub>b</sub>")
    );
}

/// The closing tilde also closes after a `~`, even before a letter.
#[test]
fn closer_after_escaped_tilde() {
    assert_eq!(r"x ~a\~~b", faithful("x <sub>a~</sub>b"));
    assert_eq!("<p>x <sub>a~</sub>b</p>\n", render(r"x ~a\~~b"));
}

/// Edge whitespace moves outside the candidate, and goes back inside the
/// element when it falls back; a space a text sibling shares stays outside.
#[test]
fn edge_whitespace() {
    assert_eq!("x ~a~ y", faithful("x<sub> a </sub>y"));
    assert_eq!("H<sub> 2</sub>O", faithful("H<sub> 2</sub>O"));
    assert_eq!("H<sub>2 </sub>O", faithful("H<sub>2 </sub>O"));
    assert_eq!("a <sub> b</sub>c", faithful("<p>a <sub> b</sub>c</p>"));
    assert_eq!("a<sub>b </sub> c", faithful("<p>a<sub>b </sub> c</p>"));
    // Any ASCII whitespace is compressed to a space first, so it is shared too.
    assert_eq!("a <sub> b</sub>c", faithful("<p>a\t<sub> b</sub>c</p>"));
    assert_eq!("a<sub>b </sub> c", faithful("<p>a<sub>b </sub>\nc</p>"));
    assert_eq!("H 2O", pure("H<sub> 2</sub>O"));
    assert_eq!("H2 O", pure("H<sub>2 </sub>O"));
    // At a paragraph's start the moved whitespace stays, as emphasis's does.
    assert_eq!(" *a* b", faithful("<p><em> a</em> b</p>"));
    assert_eq!(" ~a~ b", faithful("<p><sub> a</sub> b</p>"));
    // Faithful fallback keeps the whitespace inside the rendered element.
    assert_eq!(
        "<p>H<sub> 2</sub>O</p>\n",
        render(&faithful("H<sub> 2</sub>O"))
    );
}

/// In pure mode a hard break at either end moves out with the whitespace, so
/// neither tilde meets a line ending or a space.
#[test]
fn pure_hard_breaks_move_outside() {
    let two_spaces = Options::default();
    let backslash = Options {
        br_style: BrStyle::Backslash,
        ..Default::default()
    };
    assert_eq!(
        "x ~a~  \ny",
        convert_pure_options("<p>x <sub>a<br></sub>y</p>", two_spaces.clone())
    );
    // A moved `\⏎` break puts a backslash after the closing tilde, which step 5
    // rejects as an outside neighbor, so the content is written alone...
    assert_eq!(
        "x a\\\ny",
        convert_pure_options("<p>x <sub>a<br></sub>y</p>", backslash.clone())
    );
    // ...though the parser alone (handwritten Markdown, not a conversion) reads
    // that candidate as a subscript before a hard break.
    assert_eq!("<p>x <sub>a</sub><br />\ny</p>\n", render("x ~a~\\\ny"));
    assert_eq!(
        "x  \n~a~ y",
        convert_pure_options("<p>x <sub><br>a</sub> y</p>", two_spaces)
    );
    assert_eq!(
        "x \\\n~a~ y",
        convert_pure_options("<p>x <sub><br>a</sub> y</p>", backslash)
    );
    // Faithful mode writes the tag, which is content.
    assert_eq!("x ~a<br>~ y", faithful("<p>x <sub>a<br></sub> y</p>"));
    assert_eq!("<p>x <sub>a<br></sub> y</p>\n", render("x ~a<br>~ y"));
}

/// Step 1: attributes, and a subscript inside a subscript.
#[test]
fn attributes_and_nesting() {
    assert_eq!(
        r#"x <sub title="t">a</sub> y"#,
        faithful(r#"x <sub title="t">a</sub> y"#)
    );
    assert_eq!("x ~a~ y", pure(r#"x <sub title="t">a</sub> y"#));
    assert_eq!("~a<sub>b</sub>~", faithful("<sub>a<sub>b</sub></sub>"));
    assert_eq!("~ab~", pure("<sub>a<sub>b</sub></sub>"));
    // The nested opener here would close the outer subscript.
    assert_eq!(
        "~a.<sub>.b</sub> c~",
        faithful("<sub>a.<sub>.b</sub> c</sub>")
    );
    assert_eq!(
        "<p><sub>a.<sub>.b</sub> c</sub></p>\n",
        render(&faithful("<sub>a.<sub>.b</sub> c</sub>"))
    );
}

/// Steps 2 and 3: nothing to subscript, or a `~` the content cannot escape.
#[test]
fn empty_and_unescapable_content() {
    // The walk collapses the whitespace of a text node, as for any element.
    assert_eq!("a<sub> </sub>b", faithful("a<sub>  </sub>b"));
    assert_eq!("a b", pure("a<sub>  </sub>b"));
    assert_eq!("ab", pure("a<sub></sub>b"));
    // A code span's tilde is literal, so the element falls back.
    assert_eq!(
        "x <sub>`a~`</sub> y",
        faithful("x <sub><code>a~</code></sub> y")
    );
    assert_eq!("x `a~` y", pure("x <sub><code>a~</code></sub> y"));
}

/// Adjacent subscripts: siblings with no node between them, whatever their
/// attributes or children.
#[test]
fn adjacent_subscripts() {
    for (html, faithful_md, pure_md) in [
        (
            "<sub>a</sub><sub>b</sub>",
            "<sub>a</sub><sub>b</sub>",
            "~ab~",
        ),
        (
            r#"<sub class="x">a</sub><sub>b</sub>"#,
            r#"<sub class="x">a</sub><sub>b</sub>"#,
            "~ab~",
        ),
        (
            r#"<sub>a</sub><sub class="y">b</sub>"#,
            r#"<sub>a</sub><sub class="y">b</sub>"#,
            "~ab~",
        ),
        (
            "<sub><em>a</em></sub><sub>b</sub>",
            "<sub>*a*</sub><sub>b</sub>",
            "~*a*b~",
        ),
        (
            "<sub>a</sub><sub><em>b</em></sub>",
            "<sub>a</sub><sub>*b*</sub>",
            "~a*b*~",
        ),
        (
            "<sub>a</sub><sub>b</sub><sub>c</sub>",
            "<sub>a</sub><sub>b</sub><sub>c</sub>",
            "~abc~",
        ),
    ] {
        assert_eq!(faithful_md, faithful(html), "faithful {html:?}");
        assert_eq!(pure_md, pure(html), "pure {html:?}");
    }
    // A whitespace text node between them makes them neighbors, not a run.
    assert_eq!("~a~ ~b~", faithful("<sub>a</sub> <sub>b</sub>"));
    assert_eq!("~a~ ~b~", pure("<sub>a</sub> <sub>b</sub>"));
    assert_eq!("<p><sub>a</sub> <sub>b</sub></p>\n", render("~a~ ~b~"));
    assert_eq!(
        "<p><sub><em>a</em>b</sub></p>\n",
        render(&pure("<sub><em>a</em></sub><sub>b</sub>"))
    );
}

/// Step 1's block fallback: in a block context, faithful mode writes a
/// subscript holding a block as an HTML block. The rendered HTML parses back to
/// the source's structure, with a line ending added at the start of `<sub>`.
#[test]
fn block_descendants_faithful() {
    let html = "<sub><p>a</p><p>b</p></sub>";
    let markdown = faithful(html);
    assert_eq!("<sub>\n<p>a</p><p>b</p></sub>", markdown);
    let rendered = render(&markdown);
    assert_eq!("<sub>\n<p>a</p><p>b</p></sub>", rendered);
    assert_eq!(dom(html), dom(&rendered).replacen("<sub>\n", "<sub>", 1));

    // With attributes, inside a blockquote, beside text, and holding a `<pre>`.
    assert_eq!(
        "<sub class=\"x\">\n<p>a</p></sub>",
        faithful(r#"<sub class="x"><p>a</p></sub>"#)
    );
    let quoted = "<blockquote><sub><p>a</p></sub></blockquote>";
    assert_eq!("> <sub>\n> <p>a</p></sub>", faithful(quoted));
    assert_eq!(
        dom(quoted),
        dom(&render(&faithful(quoted)))
            .replace("<sub>\n", "<sub>")
            .replace("\n", "")
    );
    let beside = "x<sub><p>a</p></sub>y";
    assert_eq!("x\n\n<sub>\n<p>a</p></sub>\n\ny", faithful(beside));
    assert_eq!(
        "<p>x</p>\n<sub>\n<p>a</p></sub>\n<p>y</p>\n",
        render(&faithful(beside))
    );
    assert_eq!(
        "<sub>\n<pre>a</pre></sub>",
        faithful("<sub><pre>a</pre></sub>")
    );

    // An inline context needs no block: a heading's raw HTML holds the `<p>`.
    assert_eq!(
        "# x<sub><p>a</p></sub>",
        faithful("<h1>x<sub><p>a</p></sub></h1>")
    );
    assert_eq!(
        dom("<h1>x<sub><p>a</p></sub></h1>"),
        dom(&render(&faithful("<h1>x<sub><p>a</p></sub></h1>"))).trim_end()
    );
    assert_eq!("# ~<p>a</p>~", faithful("<h1><sub><p>a</p></sub></h1>"));
    assert_eq!(
        dom("<h1><sub><p>a</p></sub></h1>"),
        dom(&render(&faithful("<h1><sub><p>a</p></sub></h1>"))).trim_end()
    );
}

/// Why step 1 needs a block, read by the parser alone (handwritten Markdown,
/// not an htmd conversion): written as a subscript, or as a raw HTML inline,
/// the element sits in the paragraph the renderer writes, and the HTML parser
/// closes that paragraph at the inner `<p>`, leaving the `<sub>` empty.
#[test]
fn block_descendant_breaks_a_paragraph_parser_only() {
    for markdown in ["~<p>a</p><p>b</p>~", "<sub><p>a</p><p>b</p></sub>"] {
        assert_eq!(
            "<p><sub></sub></p><p>a</p><p>b</p><p></p>",
            dom(&render(markdown)).trim_end(),
            "{markdown:?}"
        );
    }
}

/// Pure mode drops the delimiters of a subscript holding a block and translates
/// its children where the subscript is; that comes before combining.
#[test]
fn block_descendants_pure() {
    assert_eq!("a\n\nb", pure("<sub><p>a</p><p>b</p></sub>"));
    assert_eq!("a\n\n~b~", pure("<sub><p>a</p></sub><sub>b</sub>"));
    assert_eq!(
        "<p>a</p>\n<p><sub>b</sub></p>\n",
        render(&pure("<sub><p>a</p></sub><sub>b</sub>"))
    );
    assert_eq!("~a~\n\nb", pure("<sub>a</sub><sub><p>b</p></sub>"));
    assert_eq!(
        "~a~\n\nb\n\n~cd~",
        pure("<sub>a</sub><sub><p>b</p></sub><sub>c</sub><sub>d</sub>")
    );
    assert_eq!("> a", pure("<blockquote><sub><p>a</p></sub></blockquote>"));
    // In an inline context the children are inline too, written like a
    // `<span>`'s.
    assert_eq!("# xa", pure("<h1>x<sub><p>a</p></sub></h1>"));
    assert_eq!(
        pure("<h1>x<span><p>a</p></span></h1>"),
        pure("<h1>x<sub><p>a</p></sub></h1>")
    );
    // Faithful mode writes the same adjacent pair as two serialized elements.
    assert_eq!(
        "<sub>\n<p>a</p></sub>\n\n<sub>b</sub>",
        faithful("<sub><p>a</p></sub><sub>b</sub>")
    );
}

/// Literal tildes: a run of one or two is escaped anywhere in a text node; a
/// longer run at a text node's start and inside subscript content, every tilde
/// of it.
#[test]
fn literal_tilde_runs() {
    assert_eq!(r"a\~b \~\~c", faithful("<p>a~b ~~c</p>"));
    assert_eq!("<p>a~b ~~c</p>\n", render(r"a\~b \~\~c"));
    assert_eq!(r"\~\~\~ foo", faithful("<p>~~~ foo</p>"));
    assert_eq!(
        "A sentence containing ~~~",
        faithful("<p>A sentence containing ~~~</p>")
    );
    // At a text node's start mid-line, every tilde: escaping only the first
    // would leave a `~~` that pairs into a strikethrough.
    let html = "<p><em>a</em>~~~b c~~~<em>d</em>~~~e</p>";
    assert_eq!(r"*a*\~\~\~b c~~~*d*\~\~\~e", faithful(html));
    assert_eq!(
        "<p><em>a</em>~~~b c~~~<em>d</em>~~~e</p>\n",
        render(&faithful(html))
    );
    assert!(render(r"*a*\~~~b c~~~*d*\~~~e").contains("<del>"));
    // Inside subscript content.
    assert_eq!(r"~a\~\~b~", faithful("<sub>a~~b</sub>"));
    assert_eq!(r"~a \~\~\~ b~", faithful("<sub>a ~~~ b</sub>"));
    assert_eq!(
        "<p><sub>a ~~~ b</sub></p>\n",
        render(&faithful("<sub>a ~~~ b</sub>"))
    );
    // An image description escapes a run of one or two.
    assert_eq!(
        r"![a\~b~~~c](i)",
        faithful(r#"<img src="i" alt="a~b~~~c">"#)
    );
}

/// Code spans, code blocks, math, autolinks, and link destinations keep their
/// tildes as written.
#[test]
fn literal_contexts_keep_tildes() {
    assert_eq!("`a~b`", faithful("<p><code>a~b</code></p>"));
    assert_eq!(
        "```\n~~~\na~b\n```",
        convert_faithful_options(
            "<pre><code>~~~\na~b</code></pre>",
            Options {
                code_block_style: htmd::options::CodeBlockStyle::Fenced,
                ..Default::default()
            }
        )
        .unwrap()
    );
    assert_eq!(
        "$a~b$",
        faithful(r#"<p><span class="math math-inline">a~b</span></p>"#)
    );
    assert_eq!(
        "<https://x.org/~u>",
        convert_faithful_options(
            r#"<p><a href="https://x.org/~u">https://x.org/~u</a></p>"#,
            Options {
                link_style: LinkStyle::InlinedPreferAutolinks,
                ..Default::default()
            }
        )
        .unwrap()
    );
    assert_eq!(
        "[t](https://x.org/~u)",
        faithful(r#"<p><a href="https://x.org/~u">t</a></p>"#)
    );
    // A subscript inside code stays code: faithful mode serializes the code.
    assert_eq!(
        "x <code>~i~</code>",
        faithful("<p>x <code><sub>i</sub></code></p>")
    );
    assert_eq!("x `i`", pure("<p>x <code><sub>i</sub></code></p>"));
}

/// Markers do not count toward column widths or setext underlines.
#[test]
fn widths_count_resolved_text() {
    assert_eq!(
        "| h      |\n| ------ |\n| ab ~i~ |",
        faithful(
            "<table><thead><tr><th>h</th></tr></thead><tbody><tr><td>ab <sub>i</sub></td></tr></tbody></table>"
        )
    );
    assert_eq!(
        "x ~i~\n=====",
        convert_faithful_options(
            "<h1>x <sub>i</sub></h1>",
            Options {
                heading_style: HeadingStyle::Setex,
                ..Default::default()
            }
        )
        .unwrap()
    );
}

/// A fallback reuses the content already walked, so a referenced link inside
/// is collected once.
#[test]
fn fallback_collects_a_referenced_link_once() {
    let options = Options {
        link_style: LinkStyle::Referenced,
        ..Default::default()
    };
    assert_eq!(
        "H<sub>[a][1]</sub>O\n\n[1]: u",
        convert_faithful_options(r#"<p>H<sub><a href="u">a</a></sub>O</p>"#, options).unwrap()
    );
}

/// A document already holding one of the marker characters gets no markers;
/// each subscript then falls back as a rejected candidate would.
#[test]
fn marker_characters_in_the_document() {
    assert_eq!(
        "\u{F0000} <sub>a</sub>",
        faithful("<p>&#xF0000; <sub>a</sub></p>")
    );
    assert_eq!("\u{F0000} a", pure("<p>&#xF0000; <sub>a</sub></p>"));
    assert_eq!("~a~", faithful("<p><sub>a</sub></p>"));
}

/// Widths count the document's own characters of the marker range: only a
/// marker this conversion issued is left out.
#[test]
fn widths_count_genuine_marker_range_characters() {
    let heading = convert_setext("<h1>&#xF0000;</h1>");
    assert_eq!("\u{F0000}\n=", heading);
    assert_eq!("<h1>\u{F0000}</h1>\n", render(&heading));
    assert_eq!("a\u{10FFFD}\n==", convert_setext("<h1>a&#x10FFFD;</h1>"));
    assert_eq!(
        "| h   |\n| --- |\n| \u{F0000}\u{F0001}\u{F0002} |",
        faithful(
            "<table><thead><tr><th>h</th></tr></thead><tbody><tr><td>&#xF0000;&#xF0001;&#xF0002;</td></tr></tbody></table>"
        )
    );
    // With a subscript too, the collision guard issues no marker: the
    // subscript falls back, and every character still counts.
    let mixed = convert_setext("<h1>&#xF0000; <sub>i</sub></h1>");
    assert_eq!(concat!("\u{F0000} <sub>i</sub>\n", "=============="), mixed);
    assert_eq!(14, "\u{F0000} <sub>i</sub>".chars().count());
    assert!(render(&mixed).starts_with("<h1>"));
    // Without one, the issued markers alone are left out.
    assert_eq!("x ~i~\n=====", convert_setext("<h1>x <sub>i</sub></h1>"));
}

/// A `^` in a subscript's text is escaped, which reads as `^` whether or not
/// the reader enables superscript; literal contexts and destinations keep
/// theirs.
#[test]
fn carets_in_subscript_content() {
    let superscript = CommonMarkOptions::ENABLE_SUPERSCRIPT;
    assert_eq!(r"x ~a\^ b~ y", faithful("x <sub>a^ b</sub> y"));
    assert_eq!(r"x ~a\^ b~ y", pure("x <sub>a^ b</sub> y"));
    for extra in [CommonMarkOptions::empty(), superscript] {
        assert_eq!(
            "<p>x <sub>a^ b</sub> y</p>\n",
            render_with(&faithful("x <sub>a^ b</sub> y"), extra)
        );
    }
    // Unescaped, the caret breaks the subscript for a superscript reader
    // (handwritten Markdown, read by the parser alone).
    assert_eq!(
        "<p>x ~a^ b~ y</p>\n",
        render_with("x ~a^ b~ y", superscript)
    );
    // Outside a subscript a caret is left alone.
    assert_eq!("a^ b", faithful("<p>a^ b</p>"));
    // A fallback keeps the escape, which reads the same.
    assert_eq!(r"H<sub>a\^b</sub>O", faithful("H<sub>a^b</sub>O"));
    assert_eq!(
        "<p>H<sub>a^b</sub>O</p>\n",
        render_with(&faithful("H<sub>a^b</sub>O"), superscript)
    );
    // Literal contexts and destinations.
    for (html, markdown, rendered) in [
        (
            "x <sub><code>a^ b</code></sub> y",
            "x ~`a^ b`~ y",
            "<p>x <sub><code>a^ b</code></sub> y</p>\n",
        ),
        (
            r#"x <sub><a href="u^v">t</a></sub> y"#,
            "x ~[t](u^v)~ y",
            "<p>x <sub><a href=\"u^v\">t</a></sub> y</p>\n",
        ),
        (
            r#"x <sub><img src="i" alt="a^ b"></sub> y"#,
            "x ~![a^ b](i)~ y",
            "<p>x <sub><img src=\"i\" alt=\"a^ b\" /></sub> y</p>\n",
        ),
    ] {
        assert_eq!(markdown, faithful(html), "{html:?}");
        for extra in [CommonMarkOptions::empty(), superscript] {
            assert_eq!(rendered, render_with(markdown, extra), "{html:?}");
        }
    }
    let autolink = convert_faithful_options(
        r#"x <sub><a href="https://x.org/^u">https://x.org/^u</a></sub> y"#,
        Options {
            link_style: LinkStyle::InlinedPreferAutolinks,
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!("x ~<https://x.org/^u>~ y", autolink);
    assert!(render_with(&autolink, superscript).contains("<sub><a href=\"https://x.org/^u\">"));
    // Math keeps its LaTeX; the section assumes a reader enabling math.
    let math = faithful(r#"x <sub><span class="math math-inline">a^ b</span></sub> y"#);
    assert_eq!("x ~$a^ b$~ y", math);
    assert_eq!(
        "<p>x <sub><span class=\"math math-inline\">a^ b</span></sub> y</p>\n",
        render_with(&math, superscript | CommonMarkOptions::ENABLE_MATH)
    );
}

/// A non-breaking space is never merged with a space, so a fallback shares
/// none: the element's edge whitespace goes back inside it and nothing is left
/// outside.
#[test]
fn fallback_shares_no_non_breaking_space() {
    assert_eq!(
        "a\u{a0}<sub> b</sub>c",
        faithful("<p>a&nbsp;<sub> b</sub>c</p>")
    );
    assert_eq!(
        "H<sub>b </sub>\u{a0}c",
        faithful("<p>H<sub>b </sub>&nbsp;c</p>")
    );
    // Nor is a non-breaking space at the element's own edge.
    assert_eq!(
        "a <sub>\u{a0}b</sub>c",
        faithful("<p>a <sub>&nbsp;b</sub>c</p>")
    );
    assert_eq!(
        "H<sub>b\u{a0}</sub> c",
        faithful("<p>H<sub>b&nbsp;</sub> c</p>")
    );
    // A run mixing a shared space with non-breaking ones: only the shared
    // space stays outside, and the rest of the run goes inside.
    assert_eq!(
        "a <sub> \u{a0}b</sub>c",
        faithful("<p>a <sub> &nbsp;b</sub>c</p>")
    );
    assert_eq!(
        "H<sub>b\u{a0} </sub> c",
        faithful("<p>H<sub>b&nbsp; </sub> c</p>")
    );
    assert_eq!(
        "a <sub> \u{a0}\u{a0}b</sub>c",
        faithful("<p>a <sub> &nbsp;&nbsp;b</sub>c</p>")
    );
    assert_eq!(
        "H<sub>b\u{a0}\u{a0} </sub> c",
        faithful("<p>H<sub>b&nbsp;&nbsp; </sub> c</p>")
    );
    for html in [
        "<p>a&nbsp;<sub> b</sub>c</p>",
        "<p>H<sub>b </sub>&nbsp;c</p>",
        "<p>a <sub>&nbsp;b</sub>c</p>",
        "<p>a <sub> &nbsp;b</sub>c</p>",
        "<p>H<sub>b&nbsp; </sub> c</p>",
        "<p>a <sub> &nbsp;&nbsp;b</sub>c</p>",
        "<p>H<sub>b&nbsp;&nbsp; </sub> c</p>",
    ] {
        assert_eq!(
            dom(html),
            dom(&render(&faithful(html))).trim_end(),
            "{html:?}"
        );
    }
}
