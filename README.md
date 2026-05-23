# claude-rust-tutor

Multi-provider LLM-based coding tutor in Rust. Port of [`kangwonlee/gemini-python-tutor`](https://github.com/kangwonlee/gemini-python-tutor), naming follows the convention "primary contributing LLM + impl language + tutor."

**Status:** v0.1.0 shipped 2026-05-23 — `ghcr.io/kangwonlee/claude-rust-tutor:v0.1.0` (multi-arch, ~2.17 MB amd64). Full prompt builder + 5-provider HTTP client + 19 unit tests. Smoke test against a real LLM API key still pending.

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
| Gemini (default) | `INPUT_GEMINI_API_KEY` | `gemini-2.5-flash` |
| Claude | `INPUT_CLAUDE_API_KEY` | `claude-sonnet-4-20250514` |
| Grok | `INPUT_GROK_API_KEY` | `grok-code-fast` |
| NVIDIA NIM | `INPUT_NVIDIA_API_KEY` | `google/gemma-2-9b-it` |
| Perplexity | `INPUT_PERPLEXITY_API_KEY` | `sonar` |

## Env var contract

| Env var | Meaning |
|---|---|
| `INPUT_REPORT_FILES` | Comma-separated paths to `pytest-json-report`-shape JSONs |
| `INPUT_STUDENT_FILES` | Comma-separated paths to student source files |
| `INPUT_README_PATH` | Path to assignment README |
| `INPUT_EXPLANATION_IN` | Locale name (e.g. `Korean`, `English`) |
| `INPUT_MODEL` | Optional model override |
| `INPUT_FAIL_EXPECTED` | `true` to assert failures expected (default `false`) |
| `INPUT_<PROVIDER>_API_KEY` | API key for the chosen provider |

> **Names use underscores, not hyphens.** The Python tutor
> (`gemini-python-tutor`) runs as a GitHub composite *action*, so GitHub
> injects its `with:` inputs as `INPUT_REPORT-FILES` etc., and Python's
> `os.environ` reads those hyphenated names fine. This Rust port is invoked
> directly via `docker run -e`, and **`std::env::var` silently skips env
> names containing `-`** — a hyphenated name reads as
> `environment variable not found`. The caller (classroom.yml) passes the
> underscore form; keep the two in sync.

## License

BSD-3-Clause + Do Not Harm. See [`LICENSE`](./LICENSE).

## Credits

- Architecture, prompt design, security posture (UID 1001 mount/permissions): inherited from `gemini-python-tutor`, which itself was substantially shaped by contributions from Gemini, Grok, and other LLMs.
- This Rust port: implementation mostly with Claude assistance (hence the name).
