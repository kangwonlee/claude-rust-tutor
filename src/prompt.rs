//! Port of gemini-python-tutor/prompt.py — to be filled in v0.1.1.
//!
//! Will build the final prompt string from:
//!   - test report JSONs (extract `tests[].outcome != passed/skipped`
//!     + `longrepr` / `stderr` from `pytest-json-report` shape)
//!   - student source files (sanitized, wrapped with `# begin:` markers)
//!   - README (with common-content markers stripped)
//!   - locale strings (section headers in chosen language)
//!
//! Top-level signature matches the python `engineering()` function.

#![allow(dead_code)] // scaffold

use std::path::Path;

/// Build the LLM-ready prompt string.
///
/// Returns `(n_failed_tests, prompt)`. Mirrors `prompt.engineering(...)`.
pub fn build(
    _report_paths: &[&Path],
    _student_files: &[&Path],
    _readme: &Path,
    _locale_name: &str,
) -> anyhow::Result<(usize, String)> {
    // TODO v0.1.1
    anyhow::bail!("prompt::build not yet implemented")
}
