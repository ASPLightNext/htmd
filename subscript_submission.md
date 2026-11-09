Subscript: submission
=====================

Spec section: `### Subscript`, the last section of `translation_approach.md`, on
branch `subscript-htmd` of the
[ASPLightNext/htmd](https://github.com/ASPLightNext/htmd) fork (based on
upstream `2e7bf3a`).

1. Team contributions
---------------------

* Matzhan Aruzhan: Prompted the LLM (claude-opus-5-5) to write the Subscript
  section of `translation_approach.md`, which specifies how `<sub>` is
  translated to pulldown-cmark's `~a~` syntax and when it falls back to HTML,
  and ran its six review rounds.
* Zhexenbiyev Arsen: Created the team's fork of htmd and the `subscript-htmd`
  branch, and opened the team's pull request upstream.
* Khamzin Daulet: Verified that the project builds and passes the CI checks
  (`cargo build`, `cargo clippy`, `cargo test`) on the branch, recorded the
  output below, and ran review round 7 of the Subscript section (part 5).
* Johnson Dustin: Checked the reviewers' findings against pulldown-cmark 0.13.4
  to decide which ones to accept or reject in each review round.

2. Output of "Verify that you can compile and run the project in the current git repo."
----------------------------------------------------------------------------------------

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), high effort, with `Verify
that you can compile and run the project in the current git repo.` produced:

> All three CI checks pass, and the two examples I ran succeeded. I used
> toolchain cargo 1.99.1 / rustc 1.99.1 (`aarch64-apple-darwin`), with
> `~/.cargo/bin` on `PATH`.
>
> HEAD is `c98556c` on branch `subscript-htmd`. The tracked files with
> uncommitted changes are `subscript_submission.md` and
> `translation_approach.md`, which are documentation, and `.idea/workspace.xml`,
> which is IDE state; none of them affects the build. I didn't change any
> tracked files while verifying.
>
> `htmd` is a library crate, so "run" here means the test suite and the
> examples. The three checks `AGENTS.md:52-59` says CI requires:
>
> | Command                              | Result                                                                                                                                  |
> | ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
> | `cargo build`                        | Finished the `dev` build of `htmd v0.5.5`, exit 0                                                                                       |
> | `cargo clippy -q -- --deny warnings` | No warnings, exit 0                                                                                                                     |
> | `cargo test`                         | Exit 0, no failures: 36 unit tests passed (1 ignored), 227 integration tests passed across 10 files in `tests/`, and 4 doc tests passed |
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

Prompt used to write the spec section
-------------------------------------

Prompting claude-opus-5-5 through the Kasper CLI agent with:

> Add a "Subscript" section to the end of translation_approach.md specifying how
> htmd translates `<sub>` to pulldown-cmark 0.13.4's `~a~` syntax. Follow the
> conventions of the existing sections, base every rule on pulldown-cmark's
> source, and keep it concise. Include an examples table formatted like this:
>
> | Description             | HTML in            | Faithful expected  |
> | ----------------------- | ------------------ | ------------------ |
> | Simple subscript        | `<sub>a</sub>`     | `~a~`              |
> | Intraword subscript     | `H<sub>2</sub>O`   | `H<sub>2</sub>O`   |

3. Review rounds
----------------

Each round used a fresh reviewer with the same prompt (shown in part 4). We
applied only small, targeted edits, never the reviewer's rewrites. When
reviewers disagreed, we tested the claim against pulldown-cmark 0.13.4.

The table was made by prompting claude-opus-5-5 through the Kasper CLI agent
with:

> From the six review rounds of the Subscript section in this conversation,
> make a GitHub-flavored Markdown table with the columns "Round", "Findings",
> and "Main changes made". "Findings" is the count the reviewer reported in
> that round. "Main changes made" is one sentence: the main changes applied in
> that round, separated by semicolons, with rejected findings in parentheses.
> For the last round, write "Findings below, all applied". Align the columns,
> like this example:
>
> | Round | Findings | Main changes made                                                                              |
> | ----- | -------- | ---------------------------------------------------------------------------------------------- |
> | 1     | 15       | Rule 1 vs rule 2 contradiction; added NFC, apostrophe and hyphen variants, digits inside words |
> | 2     | 11       | Stress marks (`ма́ма`) no longer split words; rule 2 written as a grammar                        |
> | 3     | 10       | NFC, not NFKC (ligature `ﬁ`); final sigma in `lower()`                                         |
> | 4     | 5        | Findings below, all applied                                                                    |

Output:

| Round | Findings | Main changes made                                                                                                                                                                                                                                  |
| ----- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 7        | Step order contradicted the interior-whitespace row; neighbors now come from the emitted Markdown, not sibling nodes; nested `<sub>` through a `<span>`; code, math, and image `alt` text handled by the `~` escaping rule; whitespace defined as `char::is_whitespace` |
| 2     | 5        | An unescaped `~` neighbor merges into `~~`; Pure-mode nested subscripts and trailing `<br>`; autolinks exempted from `\~` escaping (HTML-block escaping finding rejected as out of scope)                                                          |
| 3     | 6        | Pure `<sub><br></sub>` left a bare `~~`; only single `~` escaped, so `~~~` tests keep passing; punctuation defined (includes symbols, checked against pulldown-cmark); corrected the explanation for `~a~~b~` (line-start escape finding rejected as an implementation detail) |
| 4     | 6        | `~a~~b~` actually reads as text (the round 3 reviewer was wrong; verified); runs of two escaped too (`<sub>a~~b</sub>`, strikethrough); code fence at line start; `^` escaped when superscript is also enabled                                     |
| 5     | 5        | `\~~~` leaves a live `~~` that blocks the closing `~`; Pure combines adjacent subscripts (`~ab~`); "Unicode whitespace" renamed to the `char::is_whitespace` set; hard-break reason depends on `BrStyle`; a backslash neighbor escapes the opener |
| 6     | 4        | Findings in part 4, all applied                                                                                                                                                                                                                                         |
| 7     | 2        | Step 5 now says it needs the emitted Markdown around the element, unlike the emphasis section; step 3's reason corrected (an unmatched `~~` run, not a balanced strikethrough, blocks the closing `~`)                                                                  |

4. Round 6 review
-----------------

Prompting claude-opus-5-5 through the Kasper CLI agent, with a read-only
reviewer sub-agent given the system prompt and prompt below, produced the
following output; strikethrough would mark output we deemed incorrect or
irrelevant, but none was needed. We reproduced findings 1, 2, and 4 with
pulldown-cmark 0.13.4, and finding 3 is a real gap in the spec text. The
reviewer's "not verified" note is also resolved: pulldown-cmark's
`is_punctuation('©')` is true.

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

5. Last review (round 7)
------------------------

Prompting Claude Sonnet 5.5 (`claude-sonnet-5-5`), with a fresh read-only
reviewer sub-agent (the `Explore` agent) given the prompt below in one message,
produced the following output. Round 7 reviewed the section as
it stands on `subscript-htmd` (`c98556c`), after all round 6 findings were
applied. Unlike rounds 1 to 6, it used Claude Sonnet 5.5 rather than
`claude-opus-5-5`. The session ran at high effort; the sub-agent's own effort
level was not set separately.

The prompt keeps the system prompt and the task of part 4, with four changes:
it names `translation_approach.md` by path, since there is no `.review-ref/`
copy of the pulldown-cmark source, so the reviewer fetched v0.13.4 from GitHub;
it forbids changing the repository; it allows "Findings: 0"; and it asks for a
list of what the reviewer checked. The local scratch directory is shortened to
`<scratchpad>` below.

Prompt:

> You are a strict technical reviewer of a design spec for htmd, a Rust
> HTML-to-Markdown converter. Report only concrete errors: factual mistakes,
> contradictions, wrong examples, and missing edge cases that would produce
> incorrect output. Do not propose rewrites, style changes, or new prose. Each
> finding: one or two sentences, with the evidence.
>
> You are strictly READ-ONLY: do not create, edit, or delete any file in the
> repo, and do not run git commands that change anything. You may write scratch
> files only under `<scratchpad>/review7/` (download reference sources there).
>
> Task: Review the section named "Subscript" in
> `translation_approach.md` (the last section of that file, starts at the line
> `### Subscript`).
>
> Context: the rest of that file is the established spec; read the "Translating
> HTML nodes", "Emphasis and strong emphasis", and "Math" sections for
> conventions (Faithful vs Pure mode, serialization as HTML, adjacent-element
> runs). The target reader is pulldown-cmark 0.13.4. There is no Rust toolchain
> on this machine, so fetch the reference source from GitHub
> (`raw.githubusercontent.com/pulldown-cmark/pulldown-cmark/v0.13.4/pulldown-cmark/...`):
> `src/firstpass.rs` (`delim_run_can_open` / `delim_run_can_close` near line
> 2254; tilde run scanning near line 935), `src/parse.rs` (tilde handling near
> line 988 and `InlineStack::find_match` near line 1627), `src/scanners.rs` and
> `src/puncttable.rs` (`is_punctuation`), and `specs/super_sub.txt` and
> `specs/strikethrough.txt`. Use curl via Bash or WebFetch. Verify every claim
> in the section against that source; the htmd code itself (`src/dom_walker.rs`,
> `src/element_handler/`) is a reference for how text is escaped today.
>
> Return a numbered list of findings, at most 12, most severe first, with
> file:line evidence from the pulldown-cmark source. Then a one-line count:
> "Findings: N". If you find nothing wrong, say so and give "Findings: 0". Also
> list briefly what you checked and found correct.

The output below is the reviewer's report, unedited apart from the shortened
scratch path. No finding is struck out, because none has been shown to be
wrong, but neither is fully confirmed. Finding 1 is confirmed by reading the
spec: the emphasis section does say that examining the text before and after
an element is not implemented. Finding 2 has not been reproduced with
pulldown-cmark 0.13.4, because no Rust toolchain was available for this round,
but its reasoning from the source is consistent with the part 4 findings. Both
findings were then applied to the spec with small edits: step 3 now says an
unmatched `~~` run blocks the closing `~`, and step 5 now says it needs the
emitted Markdown around the element, unlike the emphasis section.

Output:

> Review of the "Subscript" section (`translation_approach.md:736-818`) against
> pulldown-cmark v0.13.4, source copied to `<scratchpad>/review7/`. I found one
> contradiction with the established spec and one wrong rationale. No wrong
> table row and no wrong expected output.
>
> 1. **Step 5 contradicts the emphasis section.** Step 5
>    (`translation_approach.md:790-795`) needs the emitted Markdown character
>    before the opening `~` and after the closing `~`. The after-character may
>    come from a later sibling, including a pass-through `<span>` (`:794-795`).
>    The emphasis section says (`:509-518`) that examining text "preceding and
>    following" is "not implemented" and that "the current implementation makes
>    this difficult to determine". That section therefore chose the conservative
>    edge-character rule. The Subscript section assumes the lookahead exists
>    without saying the convention changed.
>
> 2. **The step 3 rationale for `~~` is wrong as stated.** Step 3
>    (`:785-788`) says a `~~` run inside the content, "such as a strikethrough
>    written by another handler", "stops the closing `~` from matching". A
>    balanced `~~b~~` does not stop it.
>    * In `~a ~~b~~ c~`, the `~~` closer matches its `~~` opener. `find_match`
>      (`parse.rs:1627-1660`) then calls `truncate(matching_ix)`
>      (`parse.rs:1587-1593`) and sets no lower bound. The final `~` still
>      matches the stack entry at index 0, giving `<sub>a <del>b</del> c</sub>`.
>    * The `super_sub.txt` example `~This ~~is stricken.~` shows the same: an
>      opening `~~` inside is harmless.
>    * What blocks the closer is a failed closer-capable run. Examples are a
>      `~~` closer with no opener, as in `~a b~~ c~`, and a stray `^`. A failed
>      match calls `set_lowerbound` (`parse.rs:1597-1599`), which sets
>      `lower_bounds[TILDES] = stack.len()`.
>    * A strikethrough at the content edge (`<sub><del>a</del></sub>`) does
>      fail, but only because `~` plus `~~` adjoin to make a run of three, which
>      is text (`firstpass.rs:935`). The step 3 rule still produces safe output.
>      Only its stated reason is wrong.
>
> Findings: 2
>
> Checked and found correct:
>
> * **Opener rule** (`firstpass.rs:2254-2295`): returns false if there is no next
>   character or the next character is whitespace. It returns true at `ix == 0`.
>   Otherwise the previous character must be whitespace or punctuation. The
>   `prev == '~'` branch at `:2289` is redundant because `~` is itself
>   punctuation.
> * **Closer rule** (`firstpass.rs:2300-2337`): `ix == 0` gives false,
>   whitespace before gives false, and end of text gives true. A single `~`
>   closes when the next character is whitespace or punctuation, or when the
>   previous character is `~` (`:2332`).
> * **"Start of line" and "end of line"** match the code. `ix` is relative to
>   the line start passed to `parse_line` (`firstpass.rs:772`). The character
>   after a line end is a newline, which is whitespace.
> * **Whitespace** is `char::is_whitespace`, which includes U+00A0.
> * **Punctuation:** `is_punctuation` includes ASCII punctuation plus the Unicode
>   P and S categories. I decoded the `PUNCT_TAB` and mask tables: `©`, `€`, `™`
>   and emoji return true, and U+00A0 returns false.
> * **Run handling:** `is_valid_seq` (`firstpass.rs:935`) makes a run of three or
>   more text. Runs pair only with equal `run_length` (`parse.rs:1640`). A pair
>   of `~~` is Strikethrough, or Text without `ENABLE_STRIKETHROUGH`, and a
>   single `~` is Subscript, then Strikethrough, then Text (`parse.rs:988-1013`).
> * **Escapes:** `\~` is honored (`super_sub.txt` "Backslash escapes"). A `~`
>   after an escaped `\~` can still open and close, so `~a\~~` and `~\~a~` work.
> * **Neighbouring tildes:** `~a~~b~` reads as text. The `~~` run fails to match
>   and sets the lower bound.
> * **`~a.~.b~ c~`:** the inner `~` is both open and close (prev `.`, next `.`)
>   and closes the outer subscript.
> * **`~a^ b~` without escaping** reads as text. A failed `^` closer sets the
>   shared TILDES bound. The `\^` escape is needed only when superscript is
>   enabled, since `^` is registered only then (`firstpass.rs:2371`).
> * **Rest of the section:**
>   * All 12 table rows are consistent with the code.
>   * The `\~\~` escaping of runs of one or two, and of every `~` in a run of
>     three or more at line start, are consistent with the code. After an
>     escaped first tilde the leftover `~~` would be a valid run.
>   * The `firstpass.rs#L2254-L2337` range is right.
>   * Pure mode's hard-break handling is right: a closing `~` at line start has
>     whitespace before it, and with `TwoSpaces` an opening `~` is followed by
>     whitespace.
>   * Faithful adjacent subscripts match the emphasis section's run rule.
>   * The lower-bound clamping in `truncate` keeps a failed closer before an
>     enclosing opener from blocking a later subscript.
