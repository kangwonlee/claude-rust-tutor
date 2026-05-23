//! claude-rust-tutor
//!
//! Reads env-var inputs (test report files, student code files, README,
//! API keys, locale, model), constructs a prompt, calls an LLM, prints
//! markdown feedback to stdout.
//!
//! Drop-in replacement for `gemini-python-tutor/entrypoint.py` — same env
//! var contract, same output destination (`$GITHUB_STEP_SUMMARY` appended
//! by the calling workflow).

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

mod llm;
mod locale;
mod prompt;
mod sanitize;

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

    let report_refs: Vec<&std::path::Path> = inputs.report_files.iter().map(PathBuf::as_path).collect();
    let student_refs: Vec<&std::path::Path> = inputs.student_files.iter().map(PathBuf::as_path).collect();

    let (n_failed, prompt_text) = match prompt::build(
        &report_refs,
        &student_refs,
        &inputs.readme,
        &inputs.explanation_in,
    ) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("prompt build error: {e}");
            return ExitCode::from(3);
        }
    };
    log::info!("prompt built: {n_failed} failed test(s), {} chars", prompt_text.len());

    let provider = match llm::select_from_env() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("provider selection error: {e}");
            return ExitCode::from(4);
        }
    };
    log::info!("provider: {}", provider.label());

    match llm::call_api(&provider, &prompt_text) {
        Ok(feedback) => {
            println!("{feedback}");
            // Mirror python tutor's exit-code policy: zero on success even if
            // tests failed (the autograder reports score separately).
            ExitCode::from(0)
        }
        Err(e) => {
            eprintln!("LLM call error: {e}");
            ExitCode::from(5)
        }
    }
}

#[derive(Debug)]
struct Inputs {
    report_files: Vec<PathBuf>,
    student_files: Vec<PathBuf>,
    readme: PathBuf,
    explanation_in: String,
    github_repo: String,
    fail_expected: bool,
    model_override: Option<String>,
}

impl Inputs {
    fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            report_files: split_csv(&env::var("INPUT_REPORT-FILES")?).into_iter().map(PathBuf::from).collect(),
            student_files: split_csv(&env::var("INPUT_STUDENT-FILES")?).into_iter().map(PathBuf::from).collect(),
            readme: PathBuf::from(env::var("INPUT_README-PATH")?),
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
