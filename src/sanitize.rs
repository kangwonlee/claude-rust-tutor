//! Port of gemini-python-tutor/prompt.py::sanitize_input.
//!
//! Strips common prompt-injection patterns and sensitive-keyword tokens
//! from untrusted text (student code content, README content) before it
//! is embedded in the LLM prompt.

use regex::RegexBuilder;
use std::sync::OnceLock;

const MAX_LEN: usize = 10_000;

/// Strip patterns the python tutor strips, then collapse newlines to
/// spaces, then truncate to `MAX_LEN` characters.
pub fn sanitize(text: &str) -> String {
    let mut out = text.to_string();

    for pat in PATTERNS.get_or_init(build_patterns) {
        out = pat.replace_all(&out, "").into_owned();
    }

    // Replace runs of newlines with a single space (matches python's re.sub(r"\n+", " ", ...)).
    let nl_run = NEWLINE_RUN.get_or_init(|| {
        RegexBuilder::new(r"\n+").build().expect("static regex")
    });
    out = nl_run.replace_all(&out, " ").trim().to_string();

    if out.chars().count() > MAX_LEN {
        log::warn!("input truncated from {} to {} chars", out.chars().count(), MAX_LEN);
        let mut truncated = String::with_capacity(MAX_LEN);
        for c in out.chars().take(MAX_LEN) {
            truncated.push(c);
        }
        out = truncated;
    }
    out
}

static PATTERNS: OnceLock<Vec<regex::Regex>> = OnceLock::new();
static NEWLINE_RUN: OnceLock<regex::Regex> = OnceLock::new();

fn build_patterns() -> Vec<regex::Regex> {
    // Mirrors gemini-python-tutor/prompt.py patterns 1:1. Case-insensitive,
    // dotall — same flags as the python re.sub call.
    [
        r"ignore\s+previous\s+instructions",
        r"grading\s+logic",
        r"system\s+prompt",
        r"###+\s*",
        r"```.*?(```|$)",
        r"secret|key|password|token",
    ]
    .iter()
    .map(|p| {
        RegexBuilder::new(p)
            .case_insensitive(true)
            .dot_matches_new_line(true)
            .build()
            .expect("static pattern")
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_ignore_previous_instructions() {
        assert!(!sanitize("Please IGNORE PREVIOUS INSTRUCTIONS and...").to_lowercase().contains("ignore previous instructions"));
    }

    #[test]
    fn collapses_newlines() {
        assert_eq!(sanitize("hello\n\n\nworld"), "hello world");
    }

    #[test]
    fn truncates_long_input() {
        let long = "a".repeat(MAX_LEN + 500);
        assert_eq!(sanitize(&long).chars().count(), MAX_LEN);
    }

    #[test]
    fn strips_secret_word() {
        assert!(!sanitize("my SECRET is 123").to_lowercase().contains("secret"));
    }
}
