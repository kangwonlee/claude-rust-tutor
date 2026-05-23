# claude-rust-tutor

Multi-provider LLM-based coding tutor in Rust. Port of [`kangwonlee/gemini-python-tutor`](https://github.com/kangwonlee/gemini-python-tutor), naming follows the convention "primary contributing LLM + impl language + tutor."

**Status:** v0.1 scaffold. Compiles; no actual feedback generation yet. Implementation in progress.

## Role

Drop-in feedback generator for autograded assignments. Reads:
- one or more `pytest-json-report`-shape JSON files (test results — from pytest, or from `cargo nextest` via an adapter),
- one or more student code files,
- a `README.md` with assignment instructions,

… and emits a markdown feedback block to stdout (which the calling CI workflow captures into `$GITHUB_STEP_SUMMARY` + uploads as an artifact).

## Distribution model

Published as a multi-arch static binary inside a `FROM scratch` carrier image at `ghcr.io/kangwonlee/claude-rust-tutor:vX.Y.Z`. Downstream grader images consume it with a multi-stage `COPY`:

```dockerfile
COPY --from=ghcr.io/kangwonlee/claude-rust-tutor:vX.Y.Z /claude-rust-tutor /usr/local/bin/
```

No python runtime needed in the grader image.

## Provider support

Same five providers as `gemini-python-tutor`, default order picks whichever API key is set:

| Provider | Env var (selector) | Default model |
|---|---|---|
| Gemini (default) | `INPUT_GEMINI-API-KEY` | `gemini-2.5-flash` |
| Claude | `INPUT_CLAUDE_API_KEY` | `claude-sonnet-4-20250514` |
| Grok | `INPUT_GROK-API-KEY` | `grok-code-fast` |
| NVIDIA NIM | `INPUT_NVIDIA-API-KEY` | `google/gemma-2-9b-it` |
| Perplexity | `INPUT_PERPLEXITY-API-KEY` | `sonar` |

## Env var contract

Identical to `gemini-python-tutor` for drop-in replaceability:

| Env var | Meaning |
|---|---|
| `INPUT_REPORT-FILES` | Comma-separated paths to `pytest-json-report`-shape JSONs |
| `INPUT_STUDENT-FILES` | Comma-separated paths to student source files |
| `INPUT_README-PATH` | Path to assignment README |
| `INPUT_EXPLANATION-IN` | Locale name (e.g. `Korean`, `English`) |
| `INPUT_MODEL` | Optional model override |
| `INPUT_FAIL-EXPECTED` | `true` to assert failures expected (default `false`) |
| `INPUT_<PROVIDER>_API_KEY` | API key for the chosen provider |

## License

BSD-3-Clause + Do Not Harm. See [`LICENSE`](./LICENSE).

## Credits

- Architecture, prompt design, security posture (UID 1001 mount/permissions): inherited from `gemini-python-tutor`, which itself was substantially shaped by contributions from Gemini, Grok, and other LLMs.
- This Rust port: implementation mostly with Claude assistance (hence the name).
