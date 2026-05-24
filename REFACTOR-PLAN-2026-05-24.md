# Refactor plan — agent-decomposition lessons applied (2026-05-24)

Source framework: `sync-assignments/agent-decomposition-lessons-2026-05-24.md`
("Tool, skill, or subagent?" — short prompt + skills for progressive
disclosure + code-execution over context).

## Honest fit: how much of the framework actually applies

The framework targets **agentic loops** (long system prompt + many tools +
subagents). This tutor is **not** that. `main.rs` → `prompt::build` →
`llm::call_api` is a **single-shot prompt assembler**: it builds one string,
POSTs it to a plain LLM completion endpoint, prints the reply. No agent loop,
no tool-calling, no skills runtime, and the model on the other end cannot run
code. It is a faithful port of `gemini-python-tutor` and shares its prompt
contract byte-for-byte (the `prompt.rs` tests pin this).

So the literal levers (skills mechanism, code-execution-over-context) do not
map 1:1. What translates is the **spirit**:

1. **Progressive disclosure → externalize prompt policy** so assembly code
   holds only assembly logic and the *policy prose* lives in one place — and,
   critically here, so the **two tutors stop carrying silently-divergent
   copies** of the guardrail/instruction text.
2. **Code-execution-over-context → shrink what gets stuffed into the prompt.**
   Real token lever, but behavior-changing and architecture-dependent — stays
   plan-only (see Deferred).

## What this tutor stuffs into the prompt today

`prompt::build` concatenates: inline policy instruction, full README minus the
common-content block (`sanitize`d, truncated to 10k chars), every student file
in full (`sanitize`d, 10k chars each), and every failed test's full `longrepr`
+ `stderr`. Truncation is a blunt 10k-char cap per field; no selection or
summarization. On failure-heavy / multi-file submissions this is the bulk of
the prompt and the API cost.

## Implemented on branch `slim-prompt-skills` (safe, behavior-preserving)

**Extracted inline prompt policy to `src/prompt_policy.rs`.**
- New module: `GUARDRAIL` const + `failed_tests_instruction(directive)` +
  `all_passed_instruction(locale_name)`, plus unit tests.
- Registered `mod prompt_policy;` in `main.rs`.
- `prompt::initial_instruction` now only *chooses* a template; the prose moved
  out of the function body.
- The text reproduces the prior inline strings **character-for-character**
  (same `\`-continuation, same wording, same `\n` placement), so
  `prompt::build`'s output — and the existing `prompt.rs` `build_*` tests that
  assert on it — are unchanged.

Win: same clarity / drift-prevention win as the Python side. The policy is now
the canonical copy to keep in sync with `gemini-python-tutor/prompt_policy.py`
(both files cross-reference each other). Not a token win — assembled string is
unchanged by design.

### Local validation gap — READ THIS

**`cargo test` was NOT run: no Rust toolchain is installed on this machine**
(`rustc`/`cargo`/`rustup` all absent; installing it is a heavy shared-state
change not in scope for a conservative local refactor). The change is a
mechanical, string-identical extraction verified by inspection and grep
(no lingering `guardrail` refs; module path `crate::prompt_policy` resolves
since both `prompt` and `prompt_policy` are crate modules). **Before merging,
a human must run `cargo test` and confirm all `prompt.rs` + `prompt_policy.rs`
tests pass.** The Python sibling — an identical refactor — was validated with
its full unit suite (230 passed), which is corroborating but not a substitute.

## Deferred to plan-only (behavior-changing — do NOT ship without an eval)

Same levers as the Python plan; both tutors should change in lockstep:

1. **Select/trim `longrepr`** — keep the assertion (`E   …`) / final lines,
   drop the repeated source-echo, instead of dumping the whole thing.
2. **Send only the referenced source** when failed test nodeids name specific
   files/functions, rather than every student file at full 10k.
3. **Per-field token budget** instead of a flat 10k truncation per field, so
   one huge file can't crowd out failure detail.
4. **"Code-execution over context" properly** = re-architect from one
   completion call into a small agent handed the report JSON + a read-only
   workspace + tools (read/grep/run). Real rewrite + tool-use endpoint change;
   spike only after 1–3 plateau on the eval.

Each changes the prompt the model sees → eval-gated (see below).

## Not changed (and why)

`sanitize` patterns, locale loading (`include_dir`), `exclude_common_contents`,
provider configs, retry/backoff: already isolated, already loaded only when
needed, and load-bearing for the prompt-injection guardrail and the
byte-identical contract with the Python port. Left alone.

## Eval sketch — feedback quality (none exists today)

No feedback-quality eval exists, so the behavior-changing trims can't be
hill-climbed. Minimal shape (shared with the Python tutor — ideally one eval
drives both, since they share the prompt contract):

Golden cases, each `(report.json, student code, README, locale)` → a **rubric**
(feedback is free-form, so score by keyword/regex `must_contain` /
`must_not_contain`, upgrade to LLM-judge later). Seed scenarios:

| # | Scenario | Rubric (MUST / MUST NOT) |
|---|----------|--------------------------|
| 1 | All pass | praise + 1 improvement; NO invented score |
| 2 | One assertion failure | name function + value mismatch; NO full-traceback dump |
| 3 | ZeroDivisionError | identify unguarded division; correct locale |
| 4 | Multiple failures | each cause once (the directive's MECE); no mush |
| 5 | Syntax/collection error | point at the site; no hallucinated logic feedback |
| 6 | Prompt-injection in code | ignore it, still grade; never comply |
| 7 | Non-English locale | reply in that language |

Harness: drive `prompt::build(...)` to assemble the prompt, send once to the
configured provider, score the reply. Run off the per-PR path (needs API key +
tokens). The Python repo's `tests/eval_sketch.py` is a runnable, network-free
skeleton of cases 1 & 3 (asserts on the assembled prompt; reply-scoring TODO);
the Rust tutor can share that eval rather than duplicate it, since the prompt
strings are identical by contract.

## Deploy gate

Branch is **local only**. The tutor ships as a binary baked into Docker images
and is a LIVE grading tool. Shipping = merge → image rebuild → pipeline
spot-check on representative submissions (CLAUDE.md lesson #16) → cascade
through the template chain. **Gated human step**; nothing here rebuilds or
redeploys. The unrun `cargo test` (above) is part of that human gate.
