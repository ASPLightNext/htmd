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
  and ran review rounds 1–6.
* Zhexenbiyev Arsen: Created the team's fork of htmd and the `subscript-htmd`
  branch, and opened the team's pull request upstream.
* Khamzin Daulet: Verified that the project builds and passes the CI checks
  (`cargo build`, `cargo clippy`, `cargo test`) on the branch, recorded the
  output below, and ran review round 7 of the Subscript section (part 5).
* Dustin Johnson: Verified the Windows build and tested subscript boundary cases with pulldown-cmark 0.13.4, then clarified the specification’s whitespace and fallback rules. The original round 8 proposed three example rows; its rerun (part 6) found that none were needed.

2. Output of "Verify that you can compile and run the project in the current git repo."
----------------------------------------------------------------------------------------

#### Khamzin Daulet

Prompting claude-opus-5-5 through the Kasper CLI agent with `Verify that you
can compile and run the project in the current git repo.` produced:

> All three CI checks pass, and the two examples I ran succeeded. I used
> toolchain cargo 1.99.1 / rustc 1.99.1 (`aarch64-apple-darwin`).
>
> HEAD is `ebd8de4` on branch `subscript-htmd`. The tracked files with
> uncommitted changes are `src/element_handler/sub.rs`,
> `tests/subscript_tests.rs`, `translation_approach.md`, and `submission.md`;
> the checks below ran against that working tree. I didn't change any tracked
> files while verifying.
>
> `htmd` is a library crate, so "run" here means the test suite and the
> examples. The three checks CI requires:
>
> | Command                              | Result                                                                                                                                  |
> | ------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
> | `cargo build`                        | Finished the `dev` build, exit 0                                                                                                        |
> | `cargo clippy -q -- --deny warnings` | No warnings, exit 0                                                                                                                     |
> | `cargo test`                         | Exit 0, no failures: 38 unit tests passed (1 ignored), 244 integration tests passed across 11 files in `tests/` (17 of them in `subscript_tests.rs`), and 4 doc tests passed |
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

#### Dustin Johnson

> Verified successfully on Windows:
>
> - `cargo build` — passed
> - `cargo clippy -- --deny warnings` — passed
> - `cargo test` — 267 passed, 1 ignored
> - `cargo run --example simple` — ran successfully
>
> I installed the Rust stable toolchain and MSVC C++ Build Tools needed to compile. The repository working tree remains unchanged.

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

Rounds 1–6 each used a fresh reviewer with the same prompt (shown in part 4);
round 7 used a variant of it (part 5), and round 8 its own prompt (part 6). We
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

Rows 7–9 were added to the table by hand after the later rounds.

Output:

| Round | Findings | Main changes made                                                                                                                                                                                                                                  |
| ----- | -------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 7        | Step order contradicted the interior-whitespace row; neighbors now come from the emitted Markdown, not sibling nodes; nested `<sub>` through a `<span>`; code, math, and image `alt` text handled by the `~` escaping rule; whitespace defined as `char::is_whitespace` |
| 2     | 5        | An unescaped `~` neighbor merges into `~~`; Pure-mode nested subscripts and trailing `<br>`; autolinks exempted from `\~` escaping (HTML-block escaping finding rejected as out of scope)                                                          |
| 3     | 6        | Pure `<sub><br></sub>` left a bare `~~`; only single `~` escaped, so `~~~` tests keep passing; punctuation defined (includes symbols, checked against pulldown-cmark); corrected the explanation for `~a~~b~` (line-start escape finding rejected as an implementation detail) |
| 4     | 6        | `~a~~b~` actually reads as text (the round 3 reviewer was wrong; verified); runs of two escaped too (`<sub>a~~b</sub>`, strikethrough); code fence at line start; `^` escaped when superscript is also enabled                                     |
| 5     | 5        | `\~~~` leaves a live `~~` that blocks the closing `~`; Pure combines adjacent subscripts (`~ab~`); "Unicode whitespace" renamed to the `char::is_whitespace` set; hard-break reason depends on `BrStyle`; a backslash neighbor escapes the opener |
| 6     | 4        | Findings in part 4, all applied                                                                                                                                                                                                                                         |
| 7     | 5        | Findings in part 5 (Opus rerun on the current section); 1, 2, 4 and 5 applied, 3 not (it changes no output)                                                                                                                                                                                                |
| 8     | 0        | Rerun against the whitespace-only rule: every case behaves as specified with no neighbor inspection; the emitted-neighbor proposal was moved to a not-implemented note.                                                                                                |
| 9     | 1        | Step 3 now checks the content after text-node tildes are escaped, so it no longer contradicts the `x<sub>␣a~b␣</sub>y` → `x␣~a\~b~␣y` row; only an unescaped `~` (from another handler, or a code span, math, or autolink) forces the fallback.                                                           |

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

5. Review (round 7)
------------------------

Prompting claude-opus-5-5 through the Kasper CLI agent, with a fresh read-only
reviewer sub-agent given the system prompt and prompt below, produced the
following output. This round replaces an earlier round 7 run with Claude Sonnet
5.5; it reviewed the section as it stands in the working tree on
`subscript-htmd` (HEAD `ebd8de4`, with uncommitted changes), so it postdates the
changes recorded in rounds 8 and 9. The sub-agent's effort level was not set
separately.

The system prompt is the one in part 4. The prompt keeps the task of part 4,
with three changes: it names `translation_approach.md` by path and points to
both the `.review-ref/` copy and the full pulldown-cmark 0.13.4 crate source;
it forbids changing any file; and it allows "Findings: 0" and asks for a list
of what the reviewer checked.

Prompt:

> You are strictly READ-ONLY: do not create, edit, or delete any file.
>
> Task: Review the section named "Subscript" in `translation_approach.md` (the
> last section of that file, starts at the line `### Subscript`).
>
> Context: the rest of that file is the established spec; read the "Translating
> HTML nodes", "Emphasis and strong emphasis", and "Math" sections for
> conventions (Faithful vs Pure mode, serialization as HTML, adjacent-element
> runs). The target reader is pulldown-cmark 0.13.4. Its reference source is in
> `.review-ref/` (firstpass.rs, parse.rs, scanners.rs, puncttable.rs,
> super_sub.txt, strikethrough.txt) and the full crate is in the local cargo
> registry: `src/firstpass.rs` (`delim_run_can_open` / `delim_run_can_close`
> near line 2254; tilde run scanning near line 935), `src/parse.rs` (tilde
> handling near line 988 and `InlineStack::find_match` near line 1627),
> `src/scanners.rs` and `src/puncttable.rs` (`is_punctuation`), and
> `specs/super_sub.txt` and `specs/strikethrough.txt`. Verify every claim in
> the section against that source; the htmd code itself (`src/dom_walker.rs`,
> `src/element_handler/`, including `sub.rs`) is a reference for how text is
> escaped today.
>
> Return a numbered list of findings, at most 12, most severe first, with
> file:line evidence from the pulldown-cmark source. Then a one-line count:
> "Findings: N". If you find nothing wrong, say so and give "Findings: 0". Also
> list briefly what you checked and found correct.

The output below is the reviewer's report, unedited. We spot-checked findings 2
and 5 against the sources: `super_sub.txt` reads `~This ~~is stricken.~` as
`<sub>This ~~is stricken.</sub>`, and `src/util/escape.rs:218-227` escapes only
runs of one or two in `alt` text. Findings 1, 2, 4, and 5 were later applied
to the spec (finding 5 by documenting that runs of three or more in `alt` text
are left as they are); finding 3 was not, since it changes no output.

Output:

> 1. **Step 2 gives the wrong reason for treating a Pure-mode hard break as
>    whitespace (lines 809–811).** For a trailing `\⏎` break, `trim_end` already
>    moves the `⏎` outside the tildes. What actually goes wrong is the leftover
>    backslash: it escapes the closing tilde (`␣~a\~⏎`), so the subscript never
>    closes. The scanner treats `\` before ASCII punctuation as an escape and
>    skips that character (`.review-ref/firstpass.rs:909-913`); htmd's own
>    comment says the same (`src/element_handler/sub.rs:125-126`). "A closing
>    `~` cannot start a line" is true (`firstpass.rs:2307-2309`), but the split
>    never produces that layout. The `BrStyle::TwoSpaces` half is moot: `␣␣⏎` is
>    all `char::is_whitespace`, so the plain split already moves it outside. The
>    rule is right; its stated reason is not.
>
> 2. **Step 3.3 overstates what an unmatched `~~` does (lines 818–819).** The
>    text says an unmatched `~~` run inside "stops the closing `~` from
>    matching", but a `~~` that can only open does not. `find_match` skips stack
>    entries whose run length differs (`.review-ref/parse.rs:1640-1642`),
>    matches the outer `~`, and turns the skipped `~~` into text
>    (`parse.rs:1651-1657`). The spec's own test shows this:
>    `~This ~~is stricken.~` reads as `<sub>This ~~is stricken.</sub>`
>    (`.review-ref/super_sub.txt:31-34`). Only a `~~` that can close and finds
>    no partner blocks: it sets the shared tilde lower bound (`parse.rs:1661`,
>    `1613-1614`, `1596-1597`), which then hides the outer `~` opener. The
>    conservative rule is still safe; the explanation is wrong.
>
> 3. **The opening rule (lines 752–754) leaves out the tilde exception, which
>    the spec gives only for closing.** A single `~` directly after a `~` can
>    open even when the character before it is not whitespace or punctuation,
>    provided the next character is not punctuation (`firstpass.rs:2289-2291`).
>    Also, a `~` with no character after it can never open
>    (`firstpass.rs:2261-2265`). The wording "the character after it is not
>    whitespace" does not say what happens at the end of the line. Neither gap
>    changes htmd's output, because htmd only writes a `~` delimiter with
>    whitespace outside it.
>
> 4. **Step 3.3 also rejects tildes that pulldown-cmark never reads as
>    delimiters (lines 815–816).** The spec exempts code spans, math, autolinks
>    and (by the existing link rules) link destinations from tilde escaping
>    (line 795–796), so their `~` stay unescaped in the content. The step's
>    claim that only another handler such as `<del>` writes an unescaped `~`
>    misses these, and any such tilde makes the check fail. So
>    `x<sub>␣<code>a~b</code>␣</sub>y` is serialized in Faithful mode, and in
>    Pure mode (step 4) it loses its subscript entirely. The table has no row
>    for this. I did not trace pulldown-cmark's code-span scanning to cite a
>    line showing these tildes are inert.
>
> 5. **The text and the code disagree on image `alt` text inside a subscript
>    (lines 799–800).** The spec says every `~` of a run of three or more is
>    escaped inside subscript content, but the `alt` escaper escapes only runs
>    of one or two and never checks whether it is inside a subscript
>    (`src/util/escape.rs:218-227`; compare the text-node path,
>    `src/dom_walker.rs:546`). So `<sub>␣<img alt="a~~~b">␣</sub>` leaves
>    unescaped tildes, step 3.3 rejects it, and Pure mode drops the subscript.
>    A run of three is plain text to the reader (`firstpass.rs:935-937`), so the
>    rule's own reason does not hold here.
>
> Findings: 5
>
> Checked and found correct against the pulldown-cmark source:
>
> * Option behavior: with subscript on, `~a~` reads as `<sub>`; without it, as
>   strikethrough if that option is on, else text (`parse.rs:988-1010`). The `~`
>   scanner is only active when one of the two options is set
>   (`firstpass.rs:2365-2369`).
> * Runs: a run of three or more is never a delimiter (`firstpass.rs:935-937`);
>   runs pair only with runs of the same length (`parse.rs:1640`); a run of two
>   is strikethrough or text (`parse.rs:988-996`).
> * Opening: next character must not be whitespace; start of line opens;
>   otherwise the previous character must be whitespace or punctuation
>   (`firstpass.rs:2266-2271`, `2293-2294`).
> * Closing: line start never closes; the previous character must not be
>   whitespace; end of line, whitespace/punctuation after, or a `~` before all
>   close (`firstpass.rs:2307-2336`).
> * Whitespace is `char::is_whitespace`, which includes U+00A0.
> * `is_punctuation` covers symbols: `©` (U+00A9) is set in the mask `0xdbfe`
>   (`puncttable.rs:767`, `1500-1512`).
> * `\~` is honored inside a subscript (`super_sub.txt:19-23`), and
>   `␣~a\~\~b~␣` is the correct escape of `<sub>␣a~~b␣</sub>`.
> * Every table row except the cases noted above, including nested subscripts,
>   subscript holding emphasis, and the paragraphs row (type-7 block opener).
> * Faithful and Pure steps 1–4 match `src/element_handler/sub.rs` and the
>   tilde escaping in `src/dom_walker.rs:537-553`.
> * `H~2~O` cannot open (`firstpass.rs:2293`); `x␣~i~␣y` would read as a
>   subscript.
> * The firstpass.rs#L2254-L2337 link range matches the two delimiter
>   functions.
>
> Not verified:
>
> * That pulldown-cmark scans inlines line by line, so offset 0 means the start
>   of a line. I relied on this for the "start of the line" and "a closing `~`
>   cannot start a line" claims.
> * I could not open the crate under `~/.cargo` (outside the workspace) and
>   used the `.review-ref` copies throughout.

## 6. Last review (round 8) — Dustin Johnson

**Tool:** VS Code AI chat, Agent mode

**Model and effort:** Exact model and effort setting not recorded.

### Prompt used

```text
Help me develop a small, evidence-backed revision to the Subscript section
of translation_approach.md for our ECE 4793 draft. My contribution is to
verify and clarify how whitespace and neighboring characters affect the
decision to emit Markdown subscript or fall back to HTML.

Read the current Subscript section, especially steps 4–6 and the Pure-mode
paragraph. Use the rest of the document only to understand its terminology
and conventions. Check Cargo.lock for the pinned pulldown-cmark version and
use that version for parser verification. Use ENABLE_SUBSCRIPT and state
which other parser options you enable. Work in my local clone. Do not
implement the feature, rewrite the whole section, rename any files, or
commit or push changes.

1. Establish the rule before proposing edits.

   Explain in plain language when a single opening or closing tilde is
   recognized as a subscript delimiter. Distinguish characters in the
   original HTML from characters in the Markdown actually emitted. Explain
   the order of operations: trim the subscript content, move any edge
   whitespace as specified, then determine what touches each delimiter.
   Identify any point where the current wording allows two reasonable
   interpretations.

2. Verify concrete examples with the pinned Markdown parser.

   Start with these inputs:

   - H<sub>2</sub>O
   - H<sub> 2 </sub>O
   - H<sub> 2</sub>O
   - H<sub>2 </sub>O
   - x (<sub>i</sub>) y
   - H<sub>2</sub><span>O</span>

   For each input, derive the candidate Markdown from the current spec.
   Where the spec calls for HTML fallback, also try the Markdown form
   that would have been emitted without the fallback. Run the candidate
   Markdown through pulldown-cmark and record whether it produces a
   subscript and what visible spacing it preserves.

   For the span example, distinguish a mode that passes through the
   span's text from one that emits the span as HTML. Do not assume the
   emitted neighbor is the same in both modes.

   If htmd does not implement the proposed rule on this branch, test
   candidate Markdown directly with a temporary parser harness. Do not
   report a parser result as an htmd conversion result. Show what you
   actually ran. If you cannot execute a case, label it as an expectation.

3. Propose the smallest useful specification change.

   Give me exact replacement text for any sentence you recommend
   changing, and suggest up to three example rows that add coverage beyond
   the existing table. Use the document's current style, including its
   visible-space notation where useful. State the expected Faithful output
   and explain any different Pure-mode behavior.

   Write the rule so someone implementing it can tell which characters to
   inspect and when to fall back. Keep the rule independent of a particular
   traversal or buffering strategy. Preserve existing decisions about
   attributes, nesting, and escaping unless a verified case directly
   contradicts one. If the rule is already clear and correct, say so and
   propose only examples that demonstrate something the table does not
   already cover.

4. Give me a review record.

   Return:
   - The exact commands run and their results.
   - A table of inputs, candidate Markdown, and observed parser behavior.
   - A numbered list of actionable spec findings, with the count.
   - The exact wording and example rows you recommend adding.
   - A one-sentence summary of the proposed change for our review-round
     table. Describe it as proposed until I apply it.

Do not edit the shared files yet. I want to review the evidence and decide
which changes to make.
```

### Review output

#### 1. Rule established

With `ENABLE_SUBSCRIPT` enabled, a single `~` opens a subscript when it is followed by a non-whitespace character and preceded by the start of the line, whitespace, or punctuation. It closes one when preceded by a non-whitespace character and followed by whitespace, punctuation, or the end of the line. The parser checks the characters of the emitted Markdown, not those of the input HTML.

The current specification does not inspect those neighbors. As with emphasis, htmd writes a subscript as Markdown only when the translated content has whitespace at both ends: that whitespace moves outside the tildes, so the opening `~` always follows whitespace and the closing `~` always precedes it, whatever the element's neighbors are. Otherwise, Faithful mode writes `<sub>`, the content with its edge whitespace, and `</sub>`; Pure mode writes the content with its edge whitespace and no tildes. Both former ambiguities are gone: no step refers to emitted neighbors, and the fallback keeps the edge whitespace inside the element (Faithful) or in place (Pure).

#### 2. Verification

`cargo tree -i pulldown-cmark` reports **0.13.4**. Because htmd now implements the rule, the rerun converts each input with htmd itself in both modes, then reads the result back with pulldown-cmark 0.13.4 and **`ENABLE_SUBSCRIPT | ENABLE_STRIKETHROUGH`** (all other options disabled), recording whether `Event::Start(Tag::Subscript)` occurs. A `<sub>` in the rendered HTML without that event comes from raw HTML, not from a Markdown subscript.

| Input | Faithful output | Pure output | Markdown subscript event | Matches spec |
| --- | --- | --- | --- | --- |
| `H<sub>2</sub>O` | `H<sub>2</sub>O` | `H2O` | No (both modes) | Yes: no edge whitespace |
| `H<sub>␣2␣</sub>O` | `H␣~2~␣O` | `H␣~2~␣O` | Yes (both modes); renders `H␣<sub>2</sub>␣O` | Yes |
| `H<sub>␣2</sub>O` | `H<sub>␣2</sub>O` | `H␣2O` | No (both modes) | Yes: trailing whitespace missing |
| `H<sub>2␣</sub>O` | `H<sub>2␣</sub>O` | `H2␣O` | No (both modes) | Yes: leading whitespace missing |
| `x␣(<sub>i</sub>)␣y` | `x␣(<sub>i</sub>)␣y` | `x␣(i)␣y` | No (both modes) | Yes: punctuation neighbors are not inspected |
| `H<sub>2</sub><span>O</span>` | `H<sub>2</sub><span>O</span>` | `H2O` | No (both modes) | Yes |
| `(<sub>i</sub><span>O</span>)` | `(<sub>i</sub><span>O</span>)` | `(iO)` | No (both modes) | Yes: the result no longer depends on the mode of a neighbor |
| `x␣<sub>i</sub>␣y` | `x␣<sub>i</sub>␣y` | `x␣i␣y` | No (both modes) | Yes: matches the "Subscript between words" row |
| `x<sub>␣a␣</sub>y` | `x␣~a~␣y` | `x␣~a~␣y` | Yes (both modes) | Yes: matches the "Edge whitespace" row |

In every case, a Markdown form appears exactly when the content has whitespace at both ends, and that form always produces a subscript event. No output depends on characters outside the element.

**What I ran:** `cargo tree -i pulldown-cmark` → `pulldown-cmark v0.13.4`. A temporary crate outside the repository (since removed) depended on `htmd` by path and on `pulldown-cmark = "=0.13.4"`; it converted the inputs above with `HtmlToMarkdown::builder().options(Options { translation_mode, ..Default::default() })`, parsed each result with `Parser::new_ext(markdown, Options::ENABLE_SUBSCRIPT | Options::ENABLE_STRIKETHROUGH)`, and rendered it with `pulldown_cmark::html::push_html`. `cargo run` exited 0. `cargo test` in the repository passed.

#### 3. Proposed specification change

None. The rule is clear and is independent of any traversal or buffering strategy. The existing table already covers the decisive cases ("Edge whitespace", "Subscript between words", "Intraword subscript", "Leading whitespace only", "Trailing whitespace only"). The mode-dependent span row proposed in the original round 8 is no longer meaningful, since neighbors are not inspected.

#### 4. Review record

**Actionable findings: 0**

**Commands and results:** `cargo tree -i pulldown-cmark` → `pulldown-cmark v0.13.4`; the temporary htmd + pinned-parser harness → exit 0, with the observations tabulated above; `cargo test` → all suites passed. No shared files were edited by this round.

**Review-round summary:** Rerun against the whitespace-only rule; every case behaves as specified with no neighbor inspection, so no changes were needed.

## Final submission

### Dustin Johnson

GitHub userid: `drj228`

#### Contributions

Finalized the Subscript specification through two review rounds, used Claude to implement the feature and regression tests, addressed implementation findings across four review rounds, and verified formatting, build, Clippy, and all tests on Windows.

#### Specification review rounds

These rounds cover only the final submission; draft review rounds are recorded above.

| Round | Necessary findings | Optional/trivial findings | Main changes made |
| --- | ---: | ---: | --- |
| 1 | 3 | 6 | Added block-descendant fallback and its precedence over Pure-mode combining, defined adjacency independently of the existing combining implementation, and clarified escaping complete tilde runs. |
| 2 | 0 | 4 | Confirmed that all three necessary findings were resolved; only optional wording suggestions remained, and no further changes were made. |

#### Last specification review

Model: Claude Opus 5.5, Medium effort.

Tool: Claude browser chat, using an attached copy of the updated specification.

The prompt below requested workspace access, but Claude could not access my computer and explicitly reported reviewing the attachment instead. Its parser tests used handwritten Markdown candidates, not htmd conversion output.

Prompt:

```text
Read the saved translation_approach.md directly from the current VS Code workspace, including its uncommitted changes. Do not clone the repository or use an earlier attachment.

Read AGENTS.md, then review only the Subscript section and its related shared-context bullet.

Check whether these findings are resolved:
1. Block descendants use the Faithful HTML-block fallback. Pure mode translates children in the incoming context, with this rule taking precedence over combining.
2. Adjacency is defined independently of can_combine and handles differing attributes and non-text children deterministically.
3. Every tilde in a run of three or more is escaped at text-node starts and inside subscript content.

Check for new substantive contradictions between the rules and examples. Verify uncertain parser behavior against pulldown-cmark 0.13.4 with ENABLE_SUBSCRIPT | ENABLE_STRIKETHROUGH. For block fallback cases, distinguish rendered HTML from its parsed DOM.

Do not edit any files, implement anything, commit, or push.

Separate necessary corrections from optional or trivial findings. Give a concrete counterexample and the smallest appropriate fix for each necessary correction. Do not repeat resolved findings without new evidence.

State the actual file path inspected and exactly what you executed versus inspected. Finish with finding counts and a one-sentence review-table summary. If only trivial suggestions remain, say so.
```

Claude’s response:

> The heading case keeps its structure. The one failing case (a subscript holding a `<p>` inside a paragraph) can’t come from real HTML, because the HTML parser closes the `<p>` first. I’ll clean up.
>
> I found **no necessary corrections** in the file you attached: all three findings are resolved and your rewording introduces no contradictions. Only four optional or trivial suggestions remain.
>
> ### Which file I reviewed
>
> I couldn’t read your VS Code workspace: no device-bridge tools are connected, so your computer isn’t reachable from here. I reviewed `/mnt/user-data/uploads/01b51335-attachment.txt`, the file you attached with this message, not a clone or an earlier attachment. It has 838 CRLF lines. I’m assuming it is your saved `translation_approach.md`, but I can’t confirm that.
>
> ### Inspected versus executed
>
> - **Inspected:**
>   - `AGENTS.md`, from the copy in my workspace taken at `a31bed2`.
>   - The type 7 bullet in “Translating HTML nodes”, the whole Subscript section, and the emphasis run rule.
>   - A diff of the attachment against the file I sent you last. It differs only in your rewording of the shared bullet, the two row names, step 1, the tilde and adjacency sentences, and the Pure paragraph.
> - **Executed:**
>   - A temporary harness on pulldown-cmark 0.13.4 (tag `v0.13.4`, `38e4d08`) with `ENABLE_SUBSCRIPT | ENABLE_STRIKETHROUGH` and no other options. It ran 10 handwritten Markdown inputs.
>   - Headless Chrome `--dump-dom` on 5 of the rendered results, to see the parsed DOM.
>   - No htmd conversions; htmd has no `<sub>` handler yet.
>   - All temporary files deleted.
>
> ### The three findings
>
> **1. Block descendants: resolved.**
>
> - **Faithful, block context:** step 1 now applies the HTML-block form before the other checks, and attributes no longer bypass it. The row’s `<sub>⏎<p>a</p><p>b</p></sub>` renders as itself, and its DOM matches the source except for a leading `⏎` text node.
> - **Faithful, inline context (new behavior):** this case now reaches the `~…~` steps. In a heading, `# ~<p>a</p>~` renders `<h1><sub><p>a</p></sub></h1>`, and the DOM keeps that structure. So does `# x ~<p>a</p><p>b</p>~ y`.
> - **Paragraphs:** inside a paragraph the same Markdown would break, but HTML parsing never produces `<p>…<sub><p>`. The case reachable through an inline ancestor such as `<em>` breaks with or without a subscript, as I reported last time.
> - **Pure:** the block rule takes precedence over combining. `a⏎⏎b` gives two paragraphs, and `a⏎⏎~b~` gives `<p>a</p><p><sub>b</sub></p>`.
>
> **2. Adjacency: resolved.** Adjacency no longer depends on `can_combine`, and the text explicitly covers differing attributes and children. Faithful mode serializes every member of a run, and Pure mode combines the eligible ones.
>
> **3. Tilde runs: resolved.** Your shorter wording keeps the rule. The parser confirms it: `*a*\~~~b c~~~*d*\~~~e` produces a strikethrough, while `*a*\~\~\~b c~~~*d*\~\~\~e` reads as plain text.
>
> ### Optional or trivial findings
>
> 1. **“This choice” now points at the wrong sentence.** In the type 7 bullet, your new sentence sits between “…the resulting context is inline.” and “This choice is a tradeoff”. “This choice” now seems to mean the subscript fallback rather than writing type 7 HTML as raw inlines. Moving the new sentence to the end of the bullet fixes it.
> 2. **“Immediately adjacent `<sub>` siblings” can be read two ways.** It could be taken to mean neighboring elements, ignoring whitespace text between them. For `<sub>a</sub> <sub>b</sub>`, that reading gives a different but valid result: Pure would write `~a b~` instead of `~a~ ~b~`, and both parse. Adding “with no node, not even whitespace text, between them” removes the doubt.
> 3. **“Apply the existing blank-line encoding rules” is vague.** Naming them would help: the type 6 `&#13;`/`&#10;` encoding of blank lines, and the blank line before and after an HTML block.
> 4. **“Every `~` is escaped at a text-node start” reads loosely.** It is clearer as “every `~` of a run of three or more that begins a text node, and of any such run inside subscript content”.
>
> ### Result
>
> - **Necessary corrections:** 0
> - **Optional or trivial:** 4
> - **Review-table summary:** All three findings are resolved and the rewording introduces no contradictions, so only trivial wording suggestions remain.

#### Implementation review rounds

These rounds cover only the final submission.

| Round | Necessary findings | Optional/trivial findings | Main changes made |
| --- | ---: | --- | --- |
| 1 | 2 | Not separately counted | Corrected width calculations to preserve genuine marker-range characters and implemented caret escaping inside subscript text. |
| 2 | 1 | 3 | Corrected Faithful fallback whitespace-sharing checks so non-breaking spaces are not treated as merged ASCII spaces. |
| 3 | 1 | 0 new; 3 carried forward | Corrected mixed space/NBSP edge runs so only the single shared ASCII space remains outside a Faithful fallback. |
| 4 | 0 | 0 new; 3 carried forward | Confirmed mixed-whitespace handling and existing regression cases; only the earlier optional suggestions remained. |

The initial implementation and subsequent corrections were generated using Claude Opus 5.5, Medium effort, in Claude’s browser chat. Round 1 findings came from ChatGPT’s inspection of the generated patch; rounds 2–4 were performed by Claude.

Implementation commit: `49f3b7d`.

Windows verification after the final correction:

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Passed |
| `cargo build --locked` | Passed |
| `cargo clippy --locked -- --deny warnings` | Passed |
| `cargo test --locked` | Passed: 39 unit tests, 249 integration tests, and 4 documentation tests; 1 ignored |

The integration tests include 22 Subscript tests. These Windows checks used the normal repository dependencies and lockfile, rather than Claude’s substitute offline dependency setup.

#### Last implementation review

Model: Claude Opus 5.5, Medium effort. Tool: Claude browser chat.

Reviewed version: the original repository archive with three patches applied in
order (the revised implementation with the marker-width and caret fixes, the
NBSP fix, and the mixed-whitespace fix), later committed as `49f3b7d`.

The prompt asked Claude to confirm that exact version, check that a Faithful
fallback keeps only the single shared ASCII space outside the element for
leading and trailing runs with one or several NBSPs, look for new correctness
problems, and report necessary and optional findings separately, without
editing files.

Claude applied the three patches to a fresh extraction, converted 15 HTML inputs
through htmd, and ran `cargo build`, `cargo clippy -- --deny warnings`,
`cargo fmt --check`, and `cargo test --no-fail-fast` (292 passed, 1 ignored) in
an offline setup without `--locked`; the Windows run above verifies the real
build.

Result: 0 necessary corrections and 0 new optional ones. Three optional
suggestions from round 2 still stood and were left unchanged:

* an element neighbor's space can be lost in a fallback;
* the subscript flag leaks into nested conversions;
* an unmatched marker could be left behind.

#### Changes after the implementation reviews

The implementation reviewed above (`49f3b7d`) is not the one on the branch now.
Commit `6a34ffd` replaced it with the simpler rule of the final spec: a
subscript is written as `~…~` only when its content has whitespace at both
ends, with no neighbor inspection. As a result:

* caret escaping was removed; `^` is left alone (test `carets_are_left_alone`);
* the Faithful fallback no longer shares an ASCII space outside the element; it
  writes `<sub>`, the content with its edge whitespace, and `</sub>`, so the
  NBSP and mixed-whitespace fixes and the `fallback_shares` test are gone;
* the spec no longer defines adjacency on its own: adjacent subscripts follow
  the adjacent-element runs of the emphasis section, and a subscript with a
  block descendant is handled before any combining.

The test counts above (39 unit, 249 integration, 22 Subscript) are for
`49f3b7d`. On the current branch `cargo test` runs 38 unit tests (1 ignored),
244 integration tests in 11 files (17 in `subscript_tests.rs`), and 4 doc tests.

The final specification review above ran before `49f3b7d`, when htmd had no
`<sub>` handler; the round 8 rerun in part 6 ran after `6a34ffd`, against
htmd's own output.