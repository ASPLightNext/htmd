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
* Dustin Johnson: Verified the Windows build and tested subscript boundary cases with pulldown-cmark 0.13.4, then clarified the specification’s whitespace and fallback rules. The original round 8 proposed three example rows; a later rerun recorded in part 6, whose runner is not recorded, found that none were needed.

2. Output of "Verify that you can compile and run the project in the current git repo."
----------------------------------------------------------------------------------------

#### Khamzin Daulet

Record note: the output below was added in commit `f371119` (git author
ar_matzhan). It replaced the output Daulet Khamzin committed in `092b833`,
which that commit records as a Claude Sonnet 5.5 (high effort) run on HEAD
`c98556c`; that text remains in the repository history. Who ran the output
below, and at what effort, is not recorded.

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
| 7 (original) | 2 | Step 5 now says it needs the emitted Markdown around the element, unlike the emphasis section; step 3's reason corrected (an unmatched `~~` run, not a balanced strikethrough, blocks the closing `~`). Claude Sonnet 5.5 run committed in `092b833`; part 5 now holds the rerun that replaced its transcript. |
| 7 (rerun) | 5 | Findings in part 5 (Opus rerun on the current section); 1, 2, 4 and 5 applied, 3 not (it changes no output) |
| 8 (original) | 3 | Clarified which emitted characters determine subscript validity, specified where edge whitespace stays on fallback, and added three boundary examples. Dustin Johnson's run, committed in `a31bed2`; part 6 now holds the rerun that replaced its output. |
| 8 (rerun) | 0 | Rerun against the whitespace-only rule: every case behaves as specified with no neighbor inspection; the emitted-neighbor proposal was moved to a not-implemented note. |
| 9     | 1        | Step 3 now checks the content after text-node tildes are escaped, so it no longer contradicts the `x<sub>␣a~b␣</sub>y` → `x␣~a\~b~␣y` row; only an unescaped `~` (from another handler, or a code span, math, or autolink) forces the fallback. No transcript of this round is recorded in this file. |

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

Record note: this transcript was added in commit `f371119` (git author
ar_matzhan). The round 7 run it replaced was committed by Daulet Khamzin in
`092b833`, which records it as a Claude Sonnet 5.5 run on `c98556c` with two
findings; that transcript remains in the repository history. Who ran the rerun
below is not recorded.

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

## 6. Review round 8 — prompt by Dustin Johnson, output from a later rerun

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

Record note: the output below is a rerun added in commit `f371119` (git
author ar_matzhan). Who ran it, and with which tool, model, and effort, is not
recorded; the Tool and Model lines above describe Dustin Johnson's original
run. That original output, which reported three actionable findings against
the draft specification before htmd implemented subscripts, was committed in
`a31bed2` and remains in the repository history.

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

Finalized the Subscript specification at `a5326db`, used Claude to implement the feature and regression tests committed as `49f3b7d` and to perform implementation review rounds 2–4, requested fresh reviews of the team's replacement at `4cb0627`, and verified both implementations on Windows.

#### Specification review rounds

These rounds cover only the final submission; draft review rounds are recorded above. Round 1 reviewed the Subscript section at `a31bed2`, round 2 the saved file later committed as `a5326db`, and round 3 commit `4cb0627`; round 4 reviewed the corrected attachment based on that commit.

| Round | Necessary findings | Optional/trivial findings | Main changes made |
| --- | ---: | ---: | --- |
| 1 | 3 | 6 | Added block-descendant fallback and its precedence over Pure-mode combining, defined adjacency independently of the existing combining implementation, and clarified escaping complete tilde runs. |
| 2 | 0 | 4 | Confirmed that all three necessary findings were resolved; only optional wording suggestions remained, and no further changes were made. |
| 3 | 3 | 4 | Reviewed `4cb0627` against the instructor's PR comments; proposed three wording corrections, applied afterward (see "Reviews of `4cb0627`" below). |
| 4 | 0 | 0 new; 4 carried forward | Reviewed the corrected specification; S1–S3 were applied exactly and no necessary findings remain. |

#### Specification review round 2 (`a5326db`)

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

These rounds cover only the final submission. Rounds 1–4 reviewed the implementation committed as `49f3b7d`; round 5 reviewed its replacement at `4cb0627`, and round 6 checked the unchanged replacement against the corrected specification.

| Round | Necessary findings | Optional/trivial findings | Main changes made |
| --- | ---: | --- | --- |
| 1 | 2 | Not separately counted | Corrected width calculations to preserve genuine marker-range characters and implemented caret escaping inside subscript text. |
| 2 | 1 | 3 | Corrected Faithful fallback whitespace-sharing checks so non-breaking spaces are not treated as merged ASCII spaces. |
| 3 | 1 | 0 new; 3 carried forward | Corrected mixed space/NBSP edge runs so only the single shared ASCII space remains outside a Faithful fallback. |
| 4 | 0 | 0 new; 3 carried forward | Confirmed mixed-whitespace handling and existing regression cases; only the earlier optional suggestions remained. |
| 5 | 1 | 1 | Reviewed `4cb0627`: the code matches the specification and needs no change; proposed untracking `.review-ref/`, applied afterward (see "Reviews of `4cb0627`" below). |
| 6 | 0 | 0 new; 1 carried forward | Reviewed the unchanged `4cb0627` code against the corrected specification; it still matches, and Windows verification with `--locked` passed (286 tests, 1 ignored). |

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

#### Implementation review round 4 (`49f3b7d`)

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
htmd's own output. The `.review-ref/` files cited in parts 4 and 5 are
verbatim copies of pulldown-cmark v0.13.4's `src/firstpass.rs`, `src/parse.rs`,
`src/puncttable.rs`, `src/scanners.rs`, `specs/strikethrough.txt`, and
`specs/super_sub.txt`, which remain available at that upstream tag.

#### Reviews of `4cb0627` (specification round 3, implementation round 5)

Model: Claude Opus 5.5, Medium effort. Tool: Claude browser chat, with a cloud
workspace; VS Code was not used for these reviews.

Inspected version: commit `4cb0627`, the head of PR #10, fetched with git. The
attached `translation_approach.md` and `submission.md` matched it, apart from a
missing final newline in the specification. This session could not open the
pull request, so Dustin Johnson supplied the instructor's comments, retrieved
through ChatGPT's GitHub connection, without their line anchors.

Prompts, in order (the second continued the first after Claude reported that it
could not read the pull request):

```text
I am finalizing our team’s htmd Subscript project. Review the attached submission.md and translation_approach.md using Claude Opus 5.5, Medium effort.

I want one thorough, consolidated review with the smallest complete set of necessary corrections. Before responding, check your proposed corrections against each other and against the examples so that fixing one issue does not introduce another. Do not stop after the first finding.

SOURCE OF TRUTH

The attachments are our current saved files. Read them directly. Do not substitute an earlier attachment or clone.

Read the instructor’s review comments and team replies here:
https://github.com/bjones1/htmd/pull/10

Use the current pull-request source code when needed to check whether the implementation still matches the revised specification. Record the commit you inspected. Do not edit files, post comments, commit, or push.

If you cannot access GitHub, identify all missing information in one request before proposing changes. Do not guess what the instructor meant or assume a teammate’s “fixed” reply proves correctness.

REVIEW SCOPE

Review only:
- The Subscript specification.
- Shared specification rules directly relevant to Subscript.
- The Subscript implementation and affected helpers/tests.
- Relevant submission.md records.

Preserve other team members’ work and historical review transcripts.

CHECK ALL INSTRUCTOR FEEDBACK

Confirm that the current work:
1. Escapes literal tildes in text nodes.
2. Uses “move” rather than “trim” for relocated whitespace.
3. Clearly distinguishes Pure and Faithful behavior.
4. Follows the existing emphasis/strong-emphasis strategy.
5. Addresses the objection that checking characters outside the subscript is impractical.
6. Keeps superscript outside this project’s scope.
7. Defines ASCII versus Unicode whitespace where needed.
8. Explicitly translates subscript children from HTML into Markdown.
9. Clearly attributes each person’s contributions and verification.
10. Accurately identifies the LLM and actual Rust verification.

IMPORTANT IMPLEMENTATION CHECK

Our earlier implementation uses deferred private-use markers to check final emitted characters around subscript delimiters. Determine whether the current specification and instructor’s requested approach still permit that design.

Do not resolve a conflict merely by rewriting the specification to justify existing code. Prefer the smallest change consistent with the instructor’s direction and the project’s existing architecture.

Check whether later specification changes made any implementation behavior, test expectation, or submission claim outdated.

SUBMISSION RECORDS

Check that:
- Final specification review tables exclude draft rounds.
- Specification and implementation reviews have separate tables.
- Counts and summaries match the recorded reviews.
- Last-review prompts, responses, and model information are accurately recorded.
- Historical reviews remain unchanged, with later developments explained separately.
- No contribution, review round, test result, or successful check is invented.

The browser reviews used Claude Opus 5.5, Medium effort. VS Code was used separately for local editing and commands. Preserve any accurately documented use of other tools in earlier rounds.

BEFORE RETURNING YOUR ANSWER

Mentally apply the complete proposed correction set and review the resulting specification again. Check its rules, examples, mode behavior, implementation implications, and submission wording together.

Separate demonstrated correctness problems and unmet instructor requirements from optional style preferences. Do not expand scope or add speculative requirements. Do not recommend changes merely because you prefer different wording.

OUTPUT

Provide one consolidated answer containing:

1. A brief verdict: ready, or necessary corrections remain.
2. An instructor-feedback checklist showing addressed, not addressed, or not verifiable, with evidence.
3. Every necessary correction, grouped by file. For each:
   - Exact unique text to find with Ctrl+F.
   - Exact replacement text in a copyable code block.
   - A brief reason.
   - Any dependent source-code, test, or submission changes.
4. If code changes are necessary, provide one combined minimal patch against the exact inspected commit, including meaningful regression tests. Keep it separate from documentation replacements.
5. A short, ordered application and verification sequence.
6. Optional suggestions listed separately, without including them in the required edits.
7. Necessary-finding count and a one-sentence review-table summary.

Clearly distinguish inspection from execution. Claim tests passed only if you actually ran them, and disclose any substituted dependencies or other build limitations.

Do not claim that a proposed correction has already been applied or reviewed in our local repository. Do not fabricate a final zero-findings review for changes that have not yet been applied and checked.
```

```text
Proceed with one consolidated review of the current specification and implementation at the inspected commit 4cb0627. You have my authorization to perform both reviews now. Record only reviews you actually perform.

Here is the instructor’s feedback from PR #10, retrieved through ChatGPT’s GitHub connection. Each comment was part of the instructor’s Changes Requested review:

translation_approach.md:

1. “You should escape `~` found in text nodes.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571034707

2. “Use "move" instead of "trim", which usually means "remove."”
https://github.com/bjones1/htmd/pull/10#discussion_r4571045933

3. “Need two cases: one for Pure mode, another for Faithful mode. These should follow what htmd does for emphasis/strong emphasis.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571050745

4. “This is impractical -- htmd doesn't have easy access to characters outside the subscript block. See the spec for emphasis/strong emphasis.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571055186

5. “Superscript is outside the scope of this PR.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571072216

6. “What kind of whitespace -- ASCII or Unicode?”
https://github.com/bjones1/htmd/pull/10#discussion_r4571073896

7. “Awkward and a bit contradictory here. Move it closer / combine with Faithful mode approach.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571081094

8. “Missing: the contents of the `<sub>` element should be translated from HTML to Markdown.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571126263

submission.md:

9. “Which team member is this for? I only see Dustin's verification output.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571132893

10. “Use Opus if you can.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571139489

11. “You must install the Rust toolchain! Otherwise your LLM results are poor.”
https://github.com/bjones1/htmd/pull/10#discussion_r4571142883

Team replies from belivinge-2 say:
- Tilde escaping was fixed.
- “Trim” was replaced with “move.”
- The rule now follows emphasis/strong emphasis.
- Superscript was removed from the specification.
- Rust was installed and the LLM rerun.
- A shared Approach section now precedes both modes.
- Sonnet usage was replaced with Opus.
- Pure and Faithful cases were separated.
- The verification section was intended for Daulet Khamzin.
- Children are translated to Markdown in an inline context, with text tildes escaped.

These are claims in replies, not independent proof. Check them against the inspected files. ChatGPT did not retrieve the original line anchors, so do not invent their exact locations. If a missing anchor changes your conclusion, identify that specific limitation.

FACTS ABOUT MY RECORDS

I am Dustin Johnson, GitHub drj228.

My final specification and implementation reviews used Claude Opus 5.5, Medium effort, in a separate browser session. VS Code was used separately for editing and local commands.

My implementation commit was 49f3b7d. My submission-record commit was adec166. The earlier implementation reviews apply to that earlier implementation, not automatically to its replacement.

I do not know who ran the newer Part 2, Part 5, Part 6 or round 9 reviews, their exact settings, or whether their complete transcripts exist. Do not attribute them to me or invent those facts. I have not confirmed agreement to replace historical transcripts.

HOW TO PROCEED

1. Complete a fresh specification review and a fresh implementation review of the current inspected version. Evaluate them against the instructor’s comments above.

2. Keep accurate historical reviews clearly labeled with the version they covered. Treat later reruns as separate records. Do not silently restore or delete teammates’ records. Provide targeted corrections that preserve available evidence and identify any remaining attribution questions for the team.

3. Do not let unknown historical attribution block the current technical reviews. Separate what you can complete now from facts only a teammate can confirm.

4. Review the tracked .review-ref directory: determine whether it is required by the project or build. Recommend removing it if it is unnecessary scratch material. Do not assume removal is safe without checking references.

5. Return one consolidated correction set with exact unique Ctrl+F text and replacement blocks. If implementation changes are necessary, provide one minimal incremental patch against the inspected commit with regression tests.

6. Check all proposed changes together before responding. Avoid optional rewrites. Clearly distinguish necessary corrections, optional suggestions, and unresolved factual questions.

7. Provide the exact submission.md additions for the two reviews you actually perform, including the model, prompt, findings, inspected commit and execution limitations. Do not record proposed fixes as already applied or claim a post-fix review occurred unless it actually did.

Do not commit, push, or post replies. Do not modify historical test results to match the current build. Keep earlier Windows verification attached to the earlier version and report current verification separately.
```

Executed, in a substitute offline setup rather than a normal build: crates.io
was unreachable, so the dependencies were GitHub checkouts at the locked
versions, `criterion` was dropped, and the lock was re-resolved; nothing ran
with `--locked`, and nothing ran on Windows.

* `cargo build`, `cargo clippy -- --deny warnings`, and `cargo fmt --check`:
  clean.
* `cargo test`: 38 unit tests passed (1 ignored), 244 integration tests passed
  (17 in `subscript_tests.rs`), and 4 doc tests passed.
* All 16 rows of the Subscript table matched htmd's output in both modes.
* 24 further inputs were converted with htmd and read back with
  pulldown-cmark 0.13.4 (`ENABLE_SUBSCRIPT | ENABLE_STRIKETHROUGH`); every
  `~…~` that htmd wrote read as a subscript.
* With `.review-ref/` removed, the build and all 286 tests still passed.

Specification review (round 3): 3 necessary findings, 4 optional.

1. Step 2 still says "trimming" for a line ending that moves outside the
   tildes (instructor comment 2).
2. "Pure mode always writes [emphasis] with its delimiters" is false: Pure mode
   drops the delimiters of an empty or whitespace-only emphasis.
3. The shared Approach step 1 translates every subscript's children in an
   inline context, while each mode's step 1 handles a subscript holding a block
   differently (Pure translates its children in the incoming context, as the
   last table row shows); the order was not stated.

Optional: the antecedent of "This choice" in the type 7 bullet; "spaces" in the
tilde-run rule could say ASCII spaces; the emphasis section's "run of any
inline element" is wider than htmd's run code, so adjacent subscripts whose
first holds an element are written as two subscripts; "step 3 above" inside
each mode's own step 3 could name the Approach.

Implementation review (round 5): no change to the code is needed; it matches
the specification. 1 necessary finding, 1 optional.

1. `.review-ref/` holds verbatim copies of six pulldown-cmark v0.13.4 files
   without that project's license notice. Nothing in the build, tests, or CI
   uses them, and `cargo package` would ship them in the crate.

Optional: the flag that marks subscript content is thread-local, so a custom
handler that runs a separate conversion inside a subscript would escape that
conversion's tildes too.

The review also proposed record corrections to this file: provenance notes for
parts 2, 5, and 6; labeled rows for the original and rerun rounds 7 and 8;
version labels for the final-submission review rounds. None of the proposed
corrections had been applied when this review ended, and no review of the
corrected files had been performed. Who ran the reruns in parts 2, 5, and 6,
and round 9, is not recorded here; those questions are for the team.


##### Complete Claude review response

The following is the complete response for the reviews recorded above.
The preceding findings summary accompanies this original response.

> I’ll do both reviews against `4cb0627` now. First I’ll pin down the evidence the submission corrections will cite:
>
> - which commits introduced and replaced each record;
> - whether anything references `.review-ref/`;
> - which spec versions my earlier reviews covered.
>
> The project builds and all 286 tests pass with `.review-ref/` removed. The packaging check printed zero files, which may just mean the command failed offline, so I’ll look at its output directly.
>
> The anchors I’ll use are all unique. Two more exact lines remain: the last row of each final-submission review table, where the new rows go.
>
> Necessary corrections remain: three wording fixes in the spec, removing the tracked `.review-ref/` folder, and several record corrections in `submission.md`. The code itself needs no change; it matches the spec and passes everything.
>
> The exact find-and-replace text is in `subscript-corrections.md`. I checked every find string against your attached files, and each matches exactly once. I applied all the edits together to scratch copies and reread the results, and nothing conflicts. The file also holds the record of these two reviews to add to `submission.md`, with your two prompts quoted verbatim.
>
> ### What I inspected and executed
>
> - **Version:** commit `4cb0627`, the head of PR #10, fetched with git. Your attached files match it.
> - **Instructor comments:** I used the text you supplied. I still couldn’t open the PR, so I don’t have the comments’ line anchors. Only comment 7 is affected; see the checklist.
> - **Executed,** in my substitute offline setup: dependencies from GitHub checkouts, `criterion` dropped, lock re-resolved, so nothing ran with `--locked` and nothing ran on Windows.&#x20;
>   - `cargo build`, `cargo clippy -- --deny warnings` and `cargo fmt --check` are clean.
>   - `cargo test`: 38 unit (1 ignored), 244 integration (17 subscript) and 4 doc tests pass.
>   - All 16 rows of the Subscript table match htmd’s output in both modes.
>   - I converted 24 more inputs (tables, headings, links, emphasis, blockquotes, list items, non-breaking and ideographic spaces, both hard-break styles, adjacency, code, math, alt text). Every `~…~` htmd wrote reads as a subscript in pulldown-cmark 0.13.4.
>   - With `.review-ref/` removed, the build and all 286 tests still pass.
>
> ### Instructor-feedback checklist
>
> | #  | Comment                                         | Status               | Evidence                                                                                                                                                                                                                                                    |
> | -- | ----------------------------------------------- | -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
> | 1  | Escape `~` in text nodes                        | Addressed            | Spec and code escape runs of one or two everywhere, and every tilde of a longer run at a text-node start or inside a subscript. A run of three or more in mid-sentence stays as is: it can’t act as a delimiter, and an existing test expects it unescaped. |
> | 2  | “Move”, not “trim”                              | One leftover         | Step 2 still says “trimming” for a line ending that is moved (S1). Pure step 1’s “trim” really removes newlines, so it is correct.                                                                                                                          |
> | 3  | Separate Pure and Faithful cases, like emphasis | Addressed            | Separate step lists for each mode, and a Pure column in the table.                                                                                                                                                                                          |
> | 4  | Don’t inspect characters outside the subscript  | Addressed            | No neighbor inspection. The marker design is gone from both spec and code.                                                                                                                                                                                  |
> | 5  | Superscript out of scope                        | Addressed            | No mention in the spec, and no caret handling in the code.                                                                                                                                                                                                  |
> | 6  | ASCII or Unicode whitespace?                    | Addressed            | Defined as `char::is_whitespace`, which is Unicode. One unqualified “spaces” remains; it’s optional unless the comment was anchored there.                                                                                                                  |
> | 7  | Awkward and contradictory                       | One leftover         | The shared step 1 contradicts each mode’s step 1 (S3). Without the anchor I can’t tell if this is the exact passage, but it is a contradiction either way.                                                                                                  |
> | 8  | Translate children to Markdown                  | Addressed            | Step 1 says so explicitly.                                                                                                                                                                                                                                  |
> | 9  | Whose verification is this?                     | Partly               | Part 2 now has a Khamzin Daulet heading, but its content was rewritten in a later commit by a different git author (U1).                                                                                                                                    |
> | 10 | Use Opus                                        | Partly               | Parts 2 and 5 are now Opus. The model for the part 6 rerun isn’t recorded.                                                                                                                                                                                  |
> | 11 | Install Rust                                    | Addressed for part 2 | Part 2 shows cargo 1.99.1. Who ran it isn’t recorded.                                                                                                                                                                                                       |
>
> The team’s replies are true for items 1–5 and 8. They’re only partly true for 2, 7 and 9–11, as the table shows.
>
> ### Necessary corrections
>
> **`translation_approach.md`**
>
> - **S1:** “trimming only the line ending” becomes “moving only the line ending” (comment 2).
> - **S2:** the claim that Pure mode “always writes” emphasis with its delimiters is false. htmd drops them for empty or whitespace-only emphasis; the paragraph is rewrapped with the fix.
> - **S3:** shared step 1 says every subscript’s children are translated in an inline context. Read in order, that would make the last table row’s Pure result `ab` instead of `a⏎⏎b`. The fix says each mode’s step 1 comes first, which is what the code does.
>
> **Repository**
>
> - **R1, untrack `.review-ref/`:** it holds verbatim copies of six pulldown-cmark v0.13.4 files, without pulldown-cmark’s license notice.&#x20;
>   - Nothing in the build, the tests or CI uses them. Only historical transcripts mention the folder.
>   - `cargo package` would ship them inside the crate.
>   - The file gives the commands to untrack the folder but keep it locally, plus a `.gitignore` line, and a note for `submission.md` saying where the upstream originals are.
>
> **`submission.md`** (these preserve the evidence; nothing is deleted or silently restored)
>
> - **U1, U2 and U3:** provenance notes for parts 2, 5 and 6. Each names the commit that introduced the current text (`f371119`, git author ar_matzhan) and the commit holding the original (`092b833` by Daulet Khamzin; `a31bed2` by you), and says the rerun’s runner isn’t recorded. Part 6’s heading and your contribution line no longer attribute the rerun to you.
> - **U4:** the original round 7 and 8 rows go back into the table, labeled “original”, beside the reruns labeled “rerun”. Round 9 is marked as having no recorded transcript.
> - **U5:** version labels for the final-submission reviews:&#x20;
>   - spec rounds 1–2 covered `a31bed2` and `a5326db`;
>   - implementation rounds 1–4 covered `49f3b7d`;
>   - the two “Last … review” headings are renamed to name those versions;
>   - your contributions line now names `a5326db` and `49f3b7d`.
> - **U6:** the record of today’s two reviews, to paste at the end of the file. It includes the model, both prompts, commit `4cb0627`, the build limitations, and the findings. It states that none of the corrections had been applied and no review of the corrected files has happened.
>
> **No code patch is needed.** The implementation matches the spec, including after the S1–S3 wording changes, which change no behavior.
>
> ### Apply and verify
>
> 1. Apply S1–S3 to the spec and U1–U6 to `submission.md` in VS Code, which keeps CRLF line endings.
> 2. Run `git rm -r --cached .review-ref` and add the `.gitignore` line.
> 3. Run `cargo fmt --check`, then `cargo build --locked`, `cargo clippy --locked -- --deny warnings` and `cargo test --locked`. Since no code changes, you should see the same 38 / 244 / 4 counts.
> 4. Run `git diff --check`, and check that `git status` lists only the spec, `submission.md` and `.gitignore`, plus the deleted `.review-ref/` files.
> 5. Settle the open questions below with your teammates, and update the notes if anyone confirms.
> 6. Run a fresh review of the corrected files and record it separately. These two reviews don’t count as a post-fix review.
>
> ### Optional suggestions
>
> - **Spec:**&#x20;
>   - “This choice” in the type 7 bullet now seems to refer to the subscript sentence.
>   - The tilde-run rule’s “spaces” could say “ASCII spaces”.
>   - The emphasis section’s “run of any inline element” is wider than htmd’s actual run handling. So `<sub> <em>a</em> </sub><sub> b </sub>` comes out as two Markdown subscripts; the result is still valid.
>   - “Step 3 above”, written inside each mode’s own step 3, could name the shared Approach.
> - **Code:** the flag that marks subscript content is thread-local. A custom handler that runs a separate conversion inside a subscript would also escape tildes in that conversion.
>
> ### Questions only the team can answer
>
> 1. Who ran the reruns now in parts 2, 5 and 6, and with what model and effort?
> 2. Did Daulet Khamzin and you agree to have your original transcripts replaced?
> 3. Does a transcript of round 9 exist anywhere?
> 4. Who wrote commits `6a34ffd` through `4cb0627`? Their git author is ar_matzhan; the PR replies come from belivinge-2. Whoever it was should get a contribution line.
> 5. Did anyone review the spec again after the part 5 rerun’s findings were applied?
>
> ### Result
>
> - **Necessary corrections:** 3 in the spec, 1 in the repository, and 6 record corrections in `submission.md`.
> - **No code change** is needed.
> - **Review-table summary:** At `4cb0627` the implementation matches the specification and passes all checks in a substitute offline build, but three spec wording fixes, untracking `.review-ref/`, and record corrections in `submission.md` are still needed.

#### Windows verification after the documentation corrections

Dustin Johnson ran these commands in the local Windows repository after applying
the documentation corrections and untracking `.review-ref/`, with the replacement
implementation from `4cb0627`. These results are separate from the earlier
verification of `49f3b7d`.

| Command | Result |
| --- | --- |
| `cargo fmt --check` | Passed; no formatting differences reported |
| `cargo build --locked` | Passed |
| `cargo clippy --locked -- --deny warnings` | Passed |
| `cargo test --locked` | Passed: 38 unit tests, 244 integration tests, and 4 documentation tests; 1 unit test ignored |

Total: 286 tests passed, 0 failed, and 1 ignored. The integration tests include
17 Subscript tests. This Windows run used the repository lockfile and normal
dependencies, rather than Claude's substitute offline dependency setup.

#### Last specification and implementation reviews

Specification round 4; implementation round 6.

Model: Claude Opus 5.5, Medium effort. Tool: Claude browser chat.

Reviewed inputs: the corrected specification and submission attachments, with
the Rust implementation unchanged from `4cb0627`. Claude did not rerun commands
in this round; the normal Windows verification is recorded above.

The response below found no necessary specification or implementation corrections
and three submission-record corrections. Those record corrections were applied
afterward, and these review rows, prompt, and response were then recorded; this
does not claim another LLM review of the resulting submission document.

Prompt:

```text
Perform the final post-correction review using these two attachments as authoritative.

Check that S1–S3 and U1–U6 were incorporated correctly, historical records remain accurately attributed, and no necessary contradictions remain. The Rust implementation is unchanged from 4cb0627.

The latest Windows verification passed:
cargo fmt --check
cargo build --locked
cargo clippy --locked -- --deny warnings
cargo test --locked

Results: 38 unit tests, 244 integration tests, and 4 documentation tests passed; 1 unit test ignored. The integration tests include 17 Subscript tests.

Review both corrected files completely within our established scope before responding. Report only necessary corrections and unmet assignment requirements; list optional suggestions separately. Do not propose stylistic rewrites or edit files.

Return separate specification and implementation conclusions, finding counts, and one-sentence review-table summaries. State which files and code version you inspected, distinguishing my Windows results from anything you executed.
```

Claude response (Markdown formatting normalized; findings and wording retained):

> I found no remaining problems in the specification or the code. I found 3 small corrections needed in `submission.md`, and I haven’t edited any files.
>
> **What I inspected and what I ran**
>
> - **Files:** your two attachments (`translation_approach.md` and `submission.md`). I compared each one line by line with the result I expected after S1–S3 and U1–U6.
> - **Code version:** `4cb0627`. I didn’t re-run anything this time, because you said the Rust code hasn’t changed.
> - **What I ran earlier, at `4cb0627`:** build, Clippy and fmt were clean, and 38 unit, 244 integration and 4 doc tests passed. All 16 Subscript table rows matched in both modes. Those runs used my substitute offline setup, so nothing ran with `--locked` or on Windows.
> - **Your Windows results** (fmt, build, Clippy and tests, all with `--locked`): I take these as you reported them. I didn’t run them. Their totals (286 passed, 1 ignored) match my earlier counts.
> - **Not checked:** I can’t confirm from these two files that `.review-ref/` is untracked and in `.gitignore` (R1). Your Windows section says it was done.
>
> ### Specification conclusion
>
> **0 necessary, 0 new optional.** S1, S2 and S3 are in exactly as proposed; the only other difference is a final newline. Nothing in the Subscript section or the shared rules contradicts anything else. The 4 optional suggestions from round 3 are unchanged.
>
> **Review-table summary:** “Reviewed the corrected specification; S1–S3 were applied exactly and no necessary findings remain.”
>
> ### Implementation conclusion
>
> **0 necessary, 0 new optional.** The code at `4cb0627` still matches the specification. S1–S3 changed wording only, not behavior. The optional thread-local note from round 5 still applies.
>
> **Review-table summary:** “Reviewed the unchanged `4cb0627` code against the corrected specification; it still matches, and Windows verification with `--locked` passed (286 tests, 1 ignored).”
>
> ### Necessary corrections to `submission.md`
>
> U1–U4 and U6 are in exactly as proposed. You rewrote the U5 contribution line and added two new sections, “Complete Claude review response” and “Windows verification after the documentation corrections”. Three corrections are needed, and each find string appears exactly once:
>
> 1. **The contribution line credits Claude with all the reviewing.** The file itself says round 1 findings came from ChatGPT, so it disagrees with itself.
>    - Find: used Claude to implement and review the feature and regression tests committed as `49f3b7d`
>    - Replace: used Claude to implement the feature and regression tests committed as `49f3b7d` and to perform implementation review rounds 2–4
> 2. **Two table rows still say “not yet applied”,** but your new Windows section says the corrections were applied. These rows describe what happened after each review, not a moment in time, so they now contradict it.
>    - Find: proposed three wording corrections, not yet applied
>    - Replace: proposed three wording corrections, applied afterward
>    - Find: proposed untracking `.review-ref/`, not yet applied
>    - Replace: proposed untracking `.review-ref/`, applied afterward
> 3. **The record of the `4cb0627` reviews says no review of the corrected files “has been performed”.** That stops being true once you record this review. The sentence’s first half is already in the past tense.
>    - Find: corrected files has been performed.
>    - Replace: corrected files had been performed.
>
> Your Part 2/5/6 provenance notes, the original and rerun rows, the label saying round 9 has no transcript, and the historical `49f3b7d` Windows results are all still attributed correctly. Nothing names who ran the reruns. The new Windows section keeps its results separate from the `49f3b7d` ones.
>
> **Still needed:** rows for this review (specification round 4, implementation round 6) and its prompt aren’t in the file yet. The two summaries above are written for those rows.
>
> ### Optional suggestions
>
> - Four lines in the quoted response end with `&#x20;`, left over from copying. It shows up only as a space, but you could delete it.
> - The quoted section is the reply as shown in chat, including progress notes. It doesn’t include `subscript-corrections.md`. A short note saying so would make “complete” exact.
> - The contribution line could mention your Windows verification of the corrected `4cb0627`.
