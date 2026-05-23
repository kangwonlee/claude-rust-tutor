//! claude-rust-tutor
//!
//! Reads env-var inputs (test report files, student code files, README,
//! API keys, locale, model), constructs a prompt, calls an LLM, prints
//! markdown feedback to stdout.
//!
//! Drop-in replacement for `gemini-python-tutor/entrypoint.py` — same env
//! var contract, same output destination (`$GITHUB_STEP_SUMMARY` appended
//! by the calling workflow).
//!
//! v0.1 scaffold: env parsing wired; provider dispatch + prompt + HTTP
//! TBD in subsequent commits.

use std::env;
use std::process::ExitCode;

mod locale;
mod sanitize;
mod prompt;
mod llm;

fn main() -> ExitCode {
    env_logger::init();

    let inputs = match Inputs::from_env() {
        Ok(i) => i,
        Err(e) => {
            eprintln!("env error: {e}");
            return ExitCode::from(2);
        }
    };

    log::info!("report files: {:?}", inputs.report_files);
    log::info!("student files: {:?}", inputs.student_files);
    log::info!("readme: {:?}", inputs.readme);
    log::info!("explanation in: {}", inputs.explanation_in);

    // TODO v0.1.1+: build prompt, pick provider, call LLM, emit feedback.
    eprintln!("claude-rust-tutor v0.1.0 scaffold — provider dispatch not yet implemented");
    ExitCode::from(0)
}

#[derive(Debug)]
struct Inputs {
    report_files: Vec<String>,
    student_files: Vec<String>,
    readme: String,
    explanation_in: String,
    github_repo: String,
    fail_expected: bool,
    model_override: Option<String>,
}

impl Inputs {
    fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            report_files: split_csv(&env::var("INPUT_REPORT-FILES")?),
            student_files: split_csv(&env::var("INPUT_STUDENT-FILES")?),
            readme: env::var("INPUT_README-PATH")?,
            explanation_in: env::var("INPUT_EXPLANATION-IN").unwrap_or_else(|_| "English".into()),
            github_repo: env::var("GITHUB_REPOSITORY").unwrap_or_else(|_| "unknown/repository".into()),
            fail_expected: env::var("INPUT_FAIL-EXPECTED")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
            model_override: env::var("INPUT_MODEL").ok().filter(|s| !s.is_empty()),
        })
    }
}

fn split_csv(s: &str) -> Vec<String> {
    s.split(',').map(|p| p.trim().to_string()).filter(|p| !p.is_empty()).collect()
}
