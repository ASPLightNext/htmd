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
* Dustin Johnson: Verified the Windows build and tested subscript boundary cases with pulldown-cmark 0.13.4, then clarified the specification’s whitespace and fallback rules and added three examples.

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

| 8 | 3 | Clarified which emitted characters determine subscript validity, specified where edge whitespace stays on fallback, and added three boundary examples. |

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

With `ENABLE_SUBSCRIPT` enabled, a single `~` opens a subscript when it is followed by a non-whitespace character and preceded by the start of the line, whitespace, or punctuation. It closes one when preceded by a non-whitespace character and followed by whitespace, punctuation, or the end of the line. A tilde inside the original HTML does not determine this: the parser checks characters in the Markdown candidate, including the translated subscript content and the output of neighboring elements.

The intended order is: trim the translated content’s edge whitespace, move that whitespace outside the would-be `~…~`, then test the delimiter boundaries in the prospective Markdown output for the current mode. For example, `H<sub> 2 </sub>O` becomes `H ~2~ O`, so both delimiters have whitespace neighbors and the parser recognizes the subscript.

There are two ambiguities worth resolving in the current wording:

- Step 5 says to inspect “emitted Markdown,” but does not explicitly say this means the *prospective, mode-specific* output after step 4—not the neighboring characters in the input HTML.
- If the candidate fails and Faithful mode falls back to HTML, steps 4–5 do not say whether the fallback retains the original edge whitespace inside `<sub>` or keeps it moved outside. Both readings are plausible; the parser tests below show their different output.

The single-tilde flanking rules themselves are already stated clearly. Nothing in these tests requires changing the existing choices about attributes, nesting, or escaping.

#### 2. Parser verification

`Cargo.lock` pins `pulldown-cmark` **0.13.4**; `cargo tree -i pulldown-cmark` also reported `pulldown-cmark v0.13.4`. The temporary harness enabled **`ENABLE_SUBSCRIPT | ENABLE_STRIKETHROUGH`**; all other parser options were disabled. It detected `Event::Start(Tag::Subscript)` and rendered the events to HTML. These are parser results—not htmd conversion results. A raw-HTML fallback can render a `<sub>` element even when there is no Markdown `Tag::Subscript` event.

In the table, `␣` denotes a literal space; spaces passed to the harness were ordinary U+0020 spaces.

| Input / case | Candidate Markdown tested | Parser observation |
| --- | --- | --- |
| `H<sub>2</sub>O`, Faithful fallback | `H<sub>2</sub>O` | No Markdown subscript event; raw HTML renders `<sub>2</sub>`. |
| Same, hypothetical Markdown form | `H~2~O` | No subscript event; tildes remain literal. `H` blocks opening and `O` blocks closing. |
| `H<sub> 2 </sub>O` | `H␣~2~␣O` | Subscript event; rendered output is `H␣<sub>2</sub>␣O`. The moved spaces remain outside the subscript. |
| `H<sub> 2</sub>O`, fallback retaining original whitespace | `H<sub>␣2</sub>O` | No Markdown subscript event; raw HTML retains the space inside `<sub>`. |
| Same, fallback after moving whitespace | `H␣<sub>2</sub>O` | No Markdown subscript event; the space is outside `<sub>`. |
| Same, hypothetical Markdown form | `H␣~2~O` | No subscript event; `O` blocks closing. |
| `H<sub>2 </sub>O`, fallback retaining original whitespace | `H<sub>2␣</sub>O` | No Markdown subscript event; raw HTML retains the space inside `<sub>`. |
| Same, fallback after moving whitespace | `H<sub>2</sub>␣O` | No Markdown subscript event; the space is outside `<sub>`. |
| Same, hypothetical Markdown form | `H~2~␣O` | No subscript event; `H` blocks opening. |
| `x (<sub>i</sub>) y` | `x (~i~) y` | Subscript event; rendered output is `x␣(<sub>i</sub>)␣y`. Parentheses are acceptable punctuation neighbors. |
| `H<sub>2</sub><span>O</span>`, Faithful | `H<sub>2</sub><span>O</span>` | No Markdown subscript event; raw `<sub>` and `<span>` HTML render. `H` blocks opening. |
| Same, Faithful hypothetical Markdown form | `H~2~<span>O</span>` | No subscript event; `H` still blocks opening. The emitted character after the closing tilde would be `<`. |
| Same, Pure | `H2O` | No Markdown subscript event; the subscript is emitted as plain content and the span’s text passes through. |
| Same, Pure hypothetical Markdown form | `H~2~O` | No subscript event; the emitted neighbor is `O` (and `H` also blocks opening). |

For the span case, the difference between modes is real even though `H` already makes this particular subscript fail: Faithful emits `<span>O</span>`, while Pure emits the text `O`. A more discriminating candidate, `(~i~<span>O</span>)`, produced a Markdown subscript event; its Pure text-neighbor counterpart `(~i~O)` did not.

**What I ran:** `cargo tree -i pulldown-cmark` reported version 0.13.4. I created and removed a temporary crate whose manifest pinned `pulldown-cmark = "=0.13.4"`.

*[Dustin’s note: The command list was blank in the copied response, so the exact harness command is not recorded.]*

The harness used `Parser::new_ext(markdown, Options::ENABLE_SUBSCRIPT | Options::ENABLE_STRIKETHROUGH)`, detected `Event::Start(Tag::Subscript)`, and called `pulldown_cmark::html::push_html`. It tested the candidates listed above. The additional-candidate run exited 0 with a harmless unused-import warning.

#### 3. Proposed specification change

I recommend replacing steps 4–5 with the following. This makes the characters under inspection, fallback behavior, and mode difference explicit while leaving the other existing decisions intact:

> 4. For a Markdown candidate, trim leading and trailing whitespace from the translated content and place those characters immediately outside the would-be `~content~` pair.
> 5. Test that candidate against the complete prospective Markdown output for the current mode. Inspect the emitted characters immediately adjacent to each tilde, including the first and last emitted characters of the content and output from neighboring elements; do not use adjacent characters from the input HTML. Apply the single-tilde flanking rules above, and retain the fallback when an unescaped `~` or `\` is immediately outside the pair. If either delimiter is invalid, Faithful mode serializes the original `<sub>` element as HTML, preserving its original edge whitespace; Pure mode emits the relocated edge whitespace and the trimmed content without tildes. A letter immediately before the opening tilde or after the closing tilde blocks that delimiter, and a neighboring tilde merges with it: `~a~~b~` reads as text. A neighbor can come from any element, including a `<span>` whose emitted Markdown differs by mode.

Three useful additions to the existing table:

| Description | HTML in | Faithful expected |
| --- | --- | --- |
| Leading edge whitespace with an intraword neighbor | `H<sub>␣2</sub>O` | `H<sub>␣2</sub>O` |
| Trailing edge whitespace with an intraword neighbor | `H<sub>2␣</sub>O` | `H<sub>2␣</sub>O` |
| Mode-dependent span neighbor | `(<sub>i</sub><span>O</span>)` | `(~i~<span>O</span>)` |

For the first two rows, Pure would emit `H␣2O` and `H2␣O`, respectively: no HTML and no subscript markup. For the span row, Pure would emit `(iO)`; Faithful’s HTML span leaves `<` next to the closing tilde, while Pure’s text neighbor `O` blocks it.

#### 4. Review record

**Actionable findings: 3**

1. Clarify that boundary checks use the complete prospective Markdown output for the active mode, after edge-whitespace relocation—not input-HTML neighbors.
2. Specify what happens to edge whitespace when step 5 falls back: retain it inside the original Faithful HTML element; in Pure, emit it outside the plain content.
3. Add asymmetric edge-whitespace and mode-dependent span examples; the current table does not cover those cases.

**Commands and results:** `cargo tree -i pulldown-cmark` → `pulldown-cmark v0.13.4`; the temporary pinned-parser harness → exit 0, with the observations tabulated here; `git status --short` → `M subscript_submission.md`. That submission-file change was already present and left untouched. I did not edit `translation_approach.md` or any shared files.

**Review-round summary (proposed):** Clarifies mode-specific delimiter checks and edge-whitespace handling on fallback, and adds asymmetric-whitespace and mode-dependent-span examples.

## Final submission

### Dustin Johnson

GitHub userid: `drj228`

#### Contributions

Finalized the Subscript specification through two review rounds, used Claude to implement the feature and regression tests, addressed implementation findings across four review rounds, and verified formatting, Build, Clippy, and all tests on Windows.

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

Model: Claude Opus 5.5, Medium effort.

Tool: Claude browser chat.

Reviewed version: the original repository archive with the revised implementation patch, the NBSP fix, and the mixed-whitespace fix applied in order. This implementation was subsequently committed as `49f3b7d`.

Prompt:

```text
Review the Subscript implementation with all three patches applied in order:
1. Revised htmd-subscript.patch containing the marker-width and caret fixes.
2. htmd-subscript-nbsp-fix.patch.
3. htmd-subscript-mixed-ws-fix.patch.

Confirm this exact version before reviewing. Read AGENTS.md.

Review only the Subscript implementation and its necessary shared-code changes against the finalized specification.

Check whether the mixed-whitespace finding is resolved: a Faithful fallback must retain only the single shared ASCII space outside the element and restore the rest of the edge run inside it. Check both leading and trailing runs containing one or multiple NBSPs, plus the existing ASCII-only and NBSP-only cases.

Check the fix for new correctness problems. Do not repeat resolved findings without concrete counterevidence. Keep previously reported optional suggestions separate unless new evidence demonstrates a substantive failure.

Normal Windows checks passed on this version:
cargo fmt --check
cargo build --locked
cargo clippy --locked -- --deny warnings
cargo test --locked

Results: 39 unit tests passed, 1 ignored; 249 integration tests passed, including 22 Subscript tests; 4 documentation tests passed. These are my results, not commands you executed.

Do not edit files, commit, or push. Report necessary corrections separately from optional or trivial findings. For each necessary correction, give a concrete input and the smallest appropriate fix.

State exactly what you inspected and executed, including dependency limitations. Finish with finding counts and a one-sentence review-table summary. If only optional or trivial suggestions remain, say so.
```

Claude’s response:

> I’ll confirm the attached third patch matches the one I delivered, apply all three in order to a fresh extraction, and check `AGENTS.md`.
>
> The mixed-whitespace finding is resolved, and I found **no new correctness problems**. Only the three optional suggestions from round 2 remain.
>
> ### Which version I reviewed
>
> - **The patch you attached** is byte-identical to the `htmd-subscript-mixed-ws-fix.patch` I delivered (SHA-256 `7fbedea5…`).
> - **The full chain:** I applied it to a fresh extraction of the archive after `htmd-subscript.patch` (`856c5bc3…`) and `htmd-subscript-nbsp-fix.patch` (`0f3b203b…`). All three applied cleanly with `patch -p1`, and the result matches the tree I fixed last round.
>
> ### What I inspected and executed
>
> - **Inspected:**
>   - `AGENTS.md`.
>   - The new fallback code in `resolve` and the sharing conditions in `sub.rs`.
>   - The walker’s space handling (`compress_whitespace`, `append_normalized_content` and `walk_text`), to check that the shared space is always the first character of a leading run or the last of a trailing one.
>   - The changed tests.
> - **Executed**, in my existing offline scratch setup with a fresh target directory:
>   - I converted 15 HTML inputs through htmd with a probe program.
>   - The focused tests (`fallback_shares`, `edge_whitespace`) pass.
>   - `cargo build`, `cargo clippy -- --deny warnings` and `cargo fmt --check` are clean.
>   - `cargo test --no-fail-fast`: 292 passed, 1 ignored.
> - **Dependency limitation:** crates.io isn’t reachable here, so the dependencies are GitHub checkouts at the locked versions. The setup drops `criterion` and re-resolves the lock offline, so nothing ran with `--locked`. Your Windows run is the one that verifies the real build.
>
> ### The mixed-whitespace finding is resolved
>
> Faithful fallbacks now keep only the shared ASCII space outside the element and put the rest of the run back inside:
>
> | Input | Output |
> | --- | --- |
> | `a <sub> &nbsp;b</sub>c` | `a <sub> \u{a0}b</sub>c` |
> | `H<sub>b&nbsp; </sub> c` | `H<sub>b\u{a0} </sub> c` |
> | `a <sub> &nbsp;&nbsp;&nbsp;b</sub>c` | `a <sub> \u{a0}\u{a0}\u{a0}b</sub>c` |
> | `H<sub>b&nbsp;&nbsp;&nbsp; </sub> c` | `H<sub>b\u{a0}\u{a0}\u{a0} </sub> c` |
> | `a <sub> &nbsp; b</sub>c` | `a <sub> \u{a0} b</sub>c` |
> | `a <sub>&nbsp; b</sub>c` | `a <sub>\u{a0} b</sub>c` |
> | `H<sub>b &nbsp;</sub> c` | `H<sub>b \u{a0}</sub> c` |
>
> In the last two rows the run doesn’t start (or end) with a plain space, so nothing is shared and the whole run goes inside.
>
> Earlier cases are unchanged:
>
> - **Plain spaces:** `a <sub> b</sub>c` and `a<sub>b </sub> c`.
> - **A non-breaking space beside the subscript:** `a\u{a0}<sub> b</sub>c` and `H<sub>b </sub>\u{a0}c`.
>
> ### Checked for new problems
>
> - **Slicing:** `lead[1..]` and `trail[..len-1]` are byte-safe, because sharing requires that character to be an ASCII space. For a run that is one shared space, the removed part is empty, so ASCII behavior is unchanged.
> - **Space flagged as shared but never merged:** a shared space that the walker didn’t actually merge can only happen at the start of a paragraph, where the previous text’s space was trimmed (`<p> <sub> &nbsp;b</sub>c</p>` gives ` <sub> \u{a0}b</sub>c`). That one outside space matches the existing convention (emphasis does the same), and CommonMark strips it, so the rendered output is unaffected.
> - **Accepted candidates are unaffected:** a subscript whose moved-out whitespace makes both tildes valid still becomes Markdown, as the spec requires (for example `x \u{a0}~a~\u{a0} y`).
> - **Pure mode** doesn’t use this code.
>
> ### Result
>
> - **Necessary corrections:** 0
> - **Optional or trivial:** 0 new. These still stand from round 2:
>   - an element neighbor’s space can be lost in a fallback;
>   - the subscript flag leaks into nested conversions;
>   - an unmatched marker could be left behind.
> - **Review-table summary:** The mixed-whitespace finding is resolved for leading and trailing runs with one or several non-breaking spaces, ASCII-only and NBSP-only cases still work, and only the earlier optional suggestions remain.

The three optional suggestions were left unchanged.