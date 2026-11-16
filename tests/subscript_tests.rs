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

/// The section's table, row by row: HTML in, faithful expected, pure expected.
const TABLE: [(&str, &str, &str); 16] = [
    ("x<sub> a </sub>y", "x ~a~ y", "x ~a~ y"),
    ("<sub>a</sub>", "<sub>a</sub>", "a"),
    ("x <sub>i</sub> y", "x <sub>i</sub> y", "x i y"),
    ("H<sub>2</sub>O", "H<sub>2</sub>O", "H2O"),
    ("H<sub> 2</sub>O", "H<sub> 2</sub>O", "H 2O"),
    ("H<sub>2 </sub>O", "H<sub>2 </sub>O", "H2 O"),
    ("a~b", r"a\~b", r"a\~b"),
    ("x<sub> a~b </sub>y", r"x ~a\~b~ y", r"x ~a\~b~ y"),
    ("a<sub> </sub>b", "a<sub> </sub>b", "a b"),
    ("a<sub></sub>b", "a<sub></sub>b", "ab"),
    (
        r#"x<sub class="c"> a </sub>y"#,
        r#"x<sub class="c"> a </sub>y"#,
        "x ~a~ y",
    ),
    ("x<sub> <em>a</em> </sub>y", "x ~*a*~ y", "x ~*a*~ y"),
    (
        "x<sub> <sub>a</sub> </sub>y",
        "x ~<sub>a</sub>~ y",
        "x ~a~ y",
    ),
    (
        "x<sub> <sub> a </sub> </sub>y",
        "x<sub> ~a~ </sub>y",
        "x ~a~ y",
    ),
    ("<sub>a</sub><sub>b</sub>", "<sub>a</sub><sub>b</sub>", "ab"),
    (
        "<sub><p>a</p><p>b</p></sub>",
        "<sub>\n<p>a</p><p>b</p></sub>",
        "a\n\nb",
    ),
];

#[test]
fn table_rows_faithful() {
    for (html, expected, _) in TABLE {
        assert_eq!(expected, faithful(html), "row {html:?}");
    }
}

#[test]
fn table_rows_pure() {
    for (html, _, expected) in TABLE {
        assert_eq!(expected, pure(html), "row {html:?}");
    }
}

/// Each faithful row reads back as the `<sub>` it came from: a Markdown
/// subscript, or the serialized element.
#[test]
fn table_rows_render() {
    for (html, rendered) in [
        ("x<sub> a </sub>y", "<p>x <sub>a</sub> y</p>\n"),
        ("<sub>a</sub>", "<p><sub>a</sub></p>\n"),
        ("x <sub>i</sub> y", "<p>x <sub>i</sub> y</p>\n"),
        ("H<sub>2</sub>O", "<p>H<sub>2</sub>O</p>\n"),
        ("H<sub> 2</sub>O", "<p>H<sub> 2</sub>O</p>\n"),
        ("a~b", "<p>a~b</p>\n"),
        ("x<sub> a~b </sub>y", "<p>x <sub>a~b</sub> y</p>\n"),
        (
            "x<sub> <em>a</em> </sub>y",
            "<p>x <sub><em>a</em></sub> y</p>\n",
        ),
        (
            "x<sub> <sub>a</sub> </sub>y",
            "<p>x <sub><sub>a</sub></sub> y</p>\n",
        ),
    ] {
        assert_eq!(rendered, render(&faithful(html)), "row {html:?}");
    }
}

/// Moving whitespace outside both tildes is safe whatever the element's
/// neighbors: a letter, a parent's delimiter, or another subscript.
#[test]
fn edge_whitespace_needs_no_neighbors() {
    for html in [
        "<p>a<sub> i </sub>b</p>",
        "<p>(<sub> i </sub>)</p>",
        "<p><em>x<sub> i </sub>y</em></p>",
        "<p>x<sub> i </sub><span>O</span></p>",
    ] {
        let markdown = faithful(html);
        assert!(!markdown.contains("<sub>"), "{html:?}: {markdown:?}");
        assert_eq!(
            2 * html.matches("<sub>").count(),
            render(&markdown).matches("sub>").count(),
            "{html:?}: {markdown:?}"
        );
    }
    // Adjacent equal subscripts are a run, which faithful mode serializes.
    assert_eq!(
        "a<sub> i </sub><sub> j </sub>b",
        faithful("<p>a<sub> i </sub><sub> j </sub>b</p>")
    );
    // A non-breaking space is whitespace too.
    assert_eq!(
        "a\u{a0}~i~\u{a0}b",
        faithful("<p>a<sub>&nbsp;i&nbsp;</sub>b</p>")
    );
    assert_eq!(
        "<p>a\u{a0}<sub>i</sub>\u{a0}b</p>\n",
        render(&faithful("<p>a<sub>&nbsp;i&nbsp;</sub>b</p>"))
    );
    // At a paragraph's start the moved whitespace stays, as emphasis's does.
    assert_eq!(" *a* b", faithful("<p><em> a</em> b</p>"));
    assert_eq!(" ~a~ b", faithful("<p><sub> a </sub>b</p>"));
}

/// Without whitespace at both ends, the element is serialized whatever its
/// neighbors, with its edge whitespace inside.
#[test]
fn no_neighbor_lookup() {
    for html in [
        "<p>x <sub>i</sub> y</p>",
        "<p>x, <sub>i</sub>, y</p>",
        "<h1><sub>i</sub></h1>",
        r#"<p><a href="u"><sub>i</sub></a></p>"#,
        "<p>a <sub> b</sub>c</p>",
        "<p>a<sub>b </sub> c</p>",
    ] {
        let markdown = faithful(html);
        assert!(markdown.contains("<sub>"), "{html:?}: {markdown:?}");
        assert!(render(&markdown).contains("<sub>"), "{html:?}");
    }
    assert_eq!("a <sub> b</sub>c", faithful("<p>a <sub> b</sub>c</p>"));
    assert_eq!("a bc", pure("<p>a <sub> b</sub>c</p>"));
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
        convert_pure_options("<p>x<sub> a<br></sub>y</p>", two_spaces.clone())
    );
    assert_eq!(
        "x ~a~\\\ny",
        convert_pure_options("<p>x<sub> a<br></sub>y</p>", backslash.clone())
    );
    assert_eq!(
        "x  \n~a~ y",
        convert_pure_options("<p>x<sub><br>a </sub>y</p>", two_spaces)
    );
    assert_eq!(
        "x \\\n~a~ y",
        convert_pure_options("<p>x <sub><br>a </sub>y</p>", backslash.clone())
    );
    // Every break at the end moves out, not just the last one.
    let breaks = convert_pure_options("<p>x<sub> a<br><br> </sub>y</p>", backslash.clone());
    assert_eq!("x ~a~\\\n\\\n y", breaks);
    assert_eq!("<p>x <sub>a</sub><br />\n<br />\ny</p>\n", render(&breaks));
    assert_eq!(
        "x ~a~ \\\n\\\ny",
        convert_pure_options("<p>x<sub> a <br><br></sub>y</p>", backslash)
    );
    // Faithful mode writes the tag, which is content.
    assert_eq!("x ~a<br>~ y", faithful("<p>x<sub> a<br> </sub>y</p>"));
    assert_eq!("<p>x <sub>a<br></sub> y</p>\n", render("x ~a<br>~ y"));
}

/// Faithful step 2: attributes. A subscript inside a subscript follows emphasis
/// inside emphasis: the inner one is translated, and the outer one is
/// serialized when the inner one's tildes are in its content.
#[test]
fn attributes_and_nesting() {
    assert_eq!("<em>*a*</em>", faithful("<em><em>a</em></em>"));
    assert_eq!(
        "x<sub> ~a~ </sub>y",
        faithful("x<sub> <sub> a </sub> </sub>y")
    );
    assert_eq!(
        "<p>x<sub> <sub>a</sub> </sub>y</p>\n",
        render(&faithful("x<sub> <sub> a </sub> </sub>y"))
    );
    assert_eq!(
        r#"x <sub title="t">a</sub> y"#,
        faithful(r#"x <sub title="t">a</sub> y"#)
    );
    assert_eq!(" ~a<sub>b</sub>~", faithful("<sub> a<sub>b</sub> </sub>"));
    assert_eq!(" ~ab~", pure("<sub> a<sub>b</sub> </sub>"));
    assert_eq!(
        " ~a.<sub>.b</sub> c~",
        faithful("<sub> a.<sub>.b</sub> c </sub>")
    );
    assert_eq!(
        "<p><sub>a.<sub>.b</sub> c</sub></p>\n",
        render(&faithful("<sub> a.<sub>.b</sub> c </sub>"))
    );
}

/// Nothing to subscript, or a `~` the content cannot escape.
#[test]
fn empty_and_unescapable_content() {
    // The walk collapses the whitespace of a text node, as for any element.
    assert_eq!("a<sub> </sub>b", faithful("a<sub>  </sub>b"));
    assert_eq!("a b", pure("a<sub>  </sub>b"));
    assert_eq!("ab", pure("a<sub></sub>b"));
    // A code span's tilde is literal, so the element falls back.
    assert_eq!(
        "x<sub> `a~` </sub>y",
        faithful("x<sub> <code>a~</code> </sub>y")
    );
    assert_eq!("x `a~` y", pure("x<sub> <code>a~</code> </sub>y"));
}

/// Adjacent subscripts are an adjacent-element run, as for emphasis.
#[test]
fn adjacent_subscripts() {
    for (html, faithful_md, pure_md) in [
        ("<sub>a</sub><sub>b</sub>", "<sub>a</sub><sub>b</sub>", "ab"),
        (
            "<sub>a</sub><sub>b</sub><sub>c</sub>",
            "<sub>a</sub><sub>b</sub><sub>c</sub>",
            "abc",
        ),
        (
            "x<sub> a</sub><sub>b </sub>y",
            "x<sub> a</sub><sub>b </sub>y",
            "x ~ab~ y",
        ),
        (
            "<sub><em>a</em></sub><sub>b</sub>",
            "<sub>*a*</sub><sub>b</sub>",
            "*a*b",
        ),
    ] {
        assert_eq!(faithful_md, faithful(html), "faithful {html:?}");
        assert_eq!(pure_md, pure(html), "pure {html:?}");
    }
    assert_eq!(
        "<p>x <sub>ab</sub> y</p>\n",
        render(&pure("x<sub> a</sub><sub>b </sub>y"))
    );
}

/// Faithful step 1's block fallback: in a block context, faithful mode writes a
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
}

/// Why faithful step 1 needs a block, read by the parser alone (handwritten
/// Markdown, not an htmd conversion): written as a subscript, or as a raw HTML
/// inline, the element sits in the paragraph the renderer writes, and the HTML
/// parser closes that paragraph at the inner `<p>`, leaving the `<sub>` empty.
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

/// Pure step 1 drops the delimiters of a subscript holding a block and
/// translates its children where the subscript is.
#[test]
fn block_descendants_pure() {
    assert_eq!("a\n\nb", pure("<sub><p>a</p><p>b</p></sub>"));
    assert_eq!("a\n\nb", pure("<sub><p>a</p></sub><sub>b</sub>"));
    assert_eq!("> a", pure("<blockquote><sub><p>a</p></sub></blockquote>"));
    // In an inline context the children are inline too, written like a
    // `<span>`'s.
    assert_eq!("# xa", pure("<h1>x<sub><p>a</p></sub></h1>"));
    assert_eq!(
        pure("<h1>x<span><p>a</p></span></h1>"),
        pure("<h1>x<sub><p>a</p></sub></h1>")
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
    assert_eq!(r"x ~a\~\~b~ y", faithful("<p>x<sub> a~~b </sub>y</p>"));
    assert_eq!(r"x ~a \~\~\~ b~ y", faithful("x<sub> a ~~~ b </sub>y"));
    assert_eq!(
        "<p>x <sub>a ~~~ b</sub> y</p>\n",
        render(&faithful("x<sub> a ~~~ b </sub>y"))
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
        "x <code><sub>i</sub></code>",
        faithful("<p>x <code><sub>i</sub></code></p>")
    );
    assert_eq!("x `i`", pure("<p>x <code><sub>i</sub></code></p>"));
}

/// Column widths and setext underlines count the subscript's tildes.
#[test]
fn widths_count_tildes() {
    assert_eq!(
        "| h      |\n| ------ |\n| ab ~i~ |",
        faithful(
            "<table><thead><tr><th>h</th></tr></thead><tbody><tr><td>ab<sub> i </sub></td></tr></tbody></table>"
        )
    );
    assert_eq!(
        "x ~i~\n=====",
        convert_faithful_options(
            "<h1>x<sub> i </sub></h1>",
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

/// A `^` is not special to this section: it is left as written.
#[test]
fn carets_are_left_alone() {
    assert_eq!("x ~a^ b~ y", faithful("x<sub> a^ b </sub>y"));
    assert_eq!("a^ b", faithful("<p>a^ b</p>"));
}
