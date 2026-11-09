Subscript: submission
=====================

Spec section: `### Subscript`, appended to `translation_approach.md` (branch
`subscript` of the [ASPLightNext/htmd](https://github.com/ASPLightNext/htmd)
fork, based on `2e7bf3a`).

Team contributions
------------------

The fork has no commits or pull requests beyond upstream yet, so each line below
must be filled in by its owner.

* sbe80 (Footnotes): TODO
* drj228 (Strikethrough): TODO
* raf322 (Task lists): TODO
* ams2083 (Smart punctuation): TODO
* jhg246 (Heading attributes): TODO
* jit45 (Definition list): TODO
* ewj55 (Superscript): TODO
* TODO-your-id (Subscript): Wrote the Subscript section of
  `translation_approach.md`, which specifies how `<sub>` is translated to
  pulldown-cmark's `~a~` syntax and when it falls back to HTML; every example
  was checked against pulldown-cmark 0.13.4.

Output of "Verify that you can compile and run the project in the current git repo."
-------------------------------------------------------------------------------------

Run in `/Users/kaspi/kbtu/htmd` (HEAD `f5cc012`, with uncommitted changes). The
agent was told where the Homebrew rustup binaries are, since they are not on
PATH. An earlier attempt failed because no Rust toolchain was installed; this is
the output after installing one.

> Yes, the project compiles and runs. All three CI checks pass, and the two
> examples I ran succeeded. I used toolchain cargo 1.99.1 / rustc 1.99.1, with
> `PATH=/opt/homebrew/opt/rustup/bin:$PATH` set for every command.
>
> The working tree isn't clean, so these results include uncommitted changes
> that were already there: `src/element_handler/p.rs` and `unsupported_html.md`
> are modified, and `.idea/` and `toc.md` are untracked. HEAD is `f5cc012`. I
> didn't change any tracked files.
>
> `htmd` is a library crate, so "run" here means the test suite and the
> examples. The three checks `AGENTS.md:52-59` says CI requires:
>
> | Command                               | Result                                                                                                                    |
> | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
> | `cargo build`                         | Succeeded: compiled all dependencies and `htmd v0.5.5` in 1m 12s, exit 0                                                  |
> | `cargo clippy -q -- --deny warnings`  | No warnings, exit 0                                                                                                       |
> | `cargo test`                          | Exit 0, no failures: 29 unit tests passed (1 ignored), 189 integration tests passed across 10 files in `tests/`, and 4 doc tests passed |
>
> Examples:
>
> * `cargo run -q --example simple` exited 0. It prints nothing because the
>   example is a series of `assert_eq!` checks, so a silent exit means they all
>   passed.
> * `cargo run -q --example table` exited 0 and printed the converted Markdown
>   table (Rust / Python / JavaScript rows) correctly.
> * I did not run `examples/page-to-markdown`, because it writes
>   `output_hacker_news.md` and `output_wikipedia.md` into the repo.
>
> I also didn't run the benchmarks (`cargo bench`).

Review rounds
-------------

Each round used a fresh reviewer with the same prompt (below). I applied only
small, targeted edits, no rewrites. When reviewers disagreed, I tested the claim
against pulldown-cmark 0.13.4.

| Round | Findings | Main changes made |
| ----- | -------- | ----------------- |
| 1 | 7 | Step order contradicted the interior-whitespace row; neighbors now come from the emitted Markdown, not sibling nodes; nested `<sub>` through a `<span>`; code, math, and image `alt` text handled by the `~` escaping rule; whitespace defined as `char::is_whitespace` |
| 2 | 5 | An unescaped `~` neighbor merges into `~~`; Pure-mode nested subscripts and trailing `<br>`; autolinks exempted from `\~` escaping (HTML-block escaping finding rejected as out of scope) |
| 3 | 6 | Pure `<sub><br></sub>` left a bare `~~`; only single `~` escaped, so `~~~` tests keep passing; punctuation defined (includes symbols, checked against pulldown-cmark); corrected the explanation for `~a~~b~` (line-start escape finding rejected as an implementation detail) |
| 4 | 6 | `~a~~b~` actually reads as text (the round 3 reviewer was wrong; verified); runs of two escaped too (`<sub>a~~b</sub>`, strikethrough); code fence at line start; `^` escaped when superscript is also enabled |
| 5 | 5 | `\~~~` leaves a live `~~` that blocks the closing `~`; Pure combines adjacent subscripts (`~ab~`); "Unicode whitespace" renamed to the `char::is_whitespace` set; hard-break reason depends on `BrStyle`; a backslash neighbor escapes the opener |
| 6 | 4 | Findings below, all applied |

Last review
-----------

Prompting claude-opus-5-5 (effort: TODO) through the Kasper CLI agent, with a
read-only reviewer sub-agent, produced the following output. No strikethrough is
needed: I reproduced findings 1, 2, and 4 with pulldown-cmark 0.13.4, and finding
3 is a gap in the spec text. The reviewer's "not verified" note about `©` is
also correct (`is_punctuation('©')` is true).

System prompt:

> You are a strict technical reviewer of a design spec for htmd, a Rust
> HTML-to-Markdown converter. Report only concrete errors: factual mistakes,
> contradictions, wrong examples, and missing edge cases that would produce
> incorrect output. Do not propose rewrites, style changes, or new prose. Each
> finding: one or two sentences, with the evidence.

Prompt:

> Review the section named "Subscript" in subscript-wt/translation_approach.md
> (the last section of that file).
>
> Context: the rest of that file is the established spec; read the "Translating
> HTML nodes", "Emphasis and strong emphasis", and "Math" sections for
> conventions (Faithful vs Pure mode, serialization as HTML, adjacent-element
> runs). The target reader is pulldown-cmark 0.13.4; its source is copied to
> .review-ref/ (firstpass.rs: delim_run_can_open/delim_run_can_close near line
> 2254; parse.rs: tilde handling near line 988 and InlineStack::find_match near
> line 1627; specs super_sub.txt and strikethrough.txt). Verify claims against
> that source.
>
> Return a numbered list of findings, at most 12, most severe first. Then a
> one-line count: "Findings: N".

Output:

> 1. **Nested subscripts can be misread (most severe).** In Faithful mode, an
>    inner subscript whose opening `~` has punctuation on both sides closes the
>    outer subscript instead of opening the inner one. Take
>    `<sub>a.<sub>.b</sub> c</sub>`. No step serializes either element: the
>    inner one's neighbours are `.` and a space, and the outer content
>    `a.~.b~ c` neither begins nor ends with `~`. So the spec emits
>    `~a.~.b~ c~`. In that output, the inner opener (`.` on each side) can both
>    open and close (`.review-ref/firstpass.rs:2293`, `:2336`). `find_match`
>    then pairs it with the outer opener, because `(1+1)%3 != 0`
>    (`.review-ref/parse.rs:1640-1646`). The result is `<sub>a.</sub>.b~ c~`.
>    Steps 3 and 5 only check the outer content's ends and the inner element's
>    outside neighbours, so nothing catches this.
>
> 2. **Escaping only the first `~` of a run of three or more leaves a working
>    strikethrough delimiter.** I read lines 799-802 as escaping only the first
>    `~` outside a subscript, which matches `\~~~` in `src/dom_walker.rs:445`.
>    The `~~` left over is a run of two, and it can close because the character
>    before it is `~` (`firstpass.rs:2285`, `:2332`). With
>    `ENABLE_STRIKETHROUGH` set, two such lines in one paragraph pair up.
>    Example: in Pure mode with a Markdown hard break, `<p>~~~a<br>~~~b</p>`
>    becomes `\~~~a\⏎\~~~b`, which reads as `~<del>a<br>~</del>b`. This also
>    contradicts line 797, which says a literal tilde can never open or close
>    strikethrough.
>
> 3. **Pure mode doesn't say what happens when a combined run fails step 3 or
>    5.** The Pure paragraph exempts runs of adjacent subscripts from the
>    "written as its content alone" fallback, and it combines them. If the
>    combined subscript fails step 5 or step 3, no output is given, and Pure
>    mode cannot fall back to HTML. Examples: a letter before it, as in
>    `x<sub>a</sub><sub>b</sub>`, or a combined content that begins or ends with
>    `~`.
>
> 4. **"Pairs only with a run of the same length" leaves out a shared blocking
>    rule.** All tilde runs share one lower bound, `lower_bounds[TILDES]`
>    (`parse.rs:1597`, `:1614`). A run that can close but finds no match raises
>    that bound past every earlier `~` opener (`parse.rs:1661`). So an unescaped
>    `~~` inside the content blocks the closing `~` even though it is a
>    different length. The spec expects `~~` content (step 3 cites
>    strikethrough), and custom handlers fall under these rules. A handler that
>    writes an intraword strikethrough in the middle, as in `~x~~b~~ y~`, reads
>    as `~x<del>b</del> y~`. Step 5 only checks the characters next to the
>    delimiters, so it misses this.
>
> Checked and correct:
>
> * The opening and closing rules (`firstpass.rs:2254-2337`, which matches the
>   linked lines).
> * Runs of three or more never become delimiters (`firstpass.rs:935`).
> * Strikethrough-or-text behaviour when subscript is off
>   (`parse.rs:988-1010`).
> * `\~` works inside a subscript (`super_sub.txt:20-22`).
> * `~a~~b~` and `~a^ b~` read as text: both are blocked by the shared bound.
> * Every table row.
>
> Not verified: that pulldown-cmark's `is_punctuation` counts `©` as
> punctuation, because `scanners.rs` and `puncttable.rs` were not in
> `.review-ref/`.
>
> Findings: 4
