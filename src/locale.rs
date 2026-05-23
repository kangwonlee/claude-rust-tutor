//! Locale strings — embedded into the binary at compile time via `include_dir`.
//!
//! Mirrors gemini-python-tutor/locale/*.json. Each file is a flat JSON object
//! with keys: directive, report_header, report_footer, instruction_start,
//! instruction_end, homework_start, homework_end.

use include_dir::{Dir, include_dir};
use std::collections::HashMap;

static LOCALES: Dir = include_dir!("$CARGO_MANIFEST_DIR/locale");

#[derive(Debug)]
pub struct Locale {
    pub directive: String,
    pub report_header: String,
    pub report_footer: String,
    pub instruction_start: String,
    pub instruction_end: String,
    pub homework_start: String,
    pub homework_end: String,
}

pub fn load(name: &str) -> anyhow::Result<Locale> {
    let filename = format!("{name}.json");
    let file = LOCALES
        .get_file(&filename)
        .ok_or_else(|| anyhow::anyhow!("locale file not found: {filename}"))?;
    let s = std::str::from_utf8(file.contents())?;
    let map: HashMap<String, String> = serde_json::from_str(s)?;
    let get = |k: &str| -> anyhow::Result<String> {
        map.get(k)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("locale {name} missing key: {k}"))
    };
    Ok(Locale {
        directive: get("directive")?,
        report_header: get("report_header")?,
        report_footer: get("report_footer")?,
        instruction_start: get("instruction_start")?,
        instruction_end: get("instruction_end")?,
        homework_start: get("homework_start")?,
        homework_end: get("homework_end")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_english() {
        let l = load("English").expect("English locale");
        assert!(!l.directive.is_empty());
    }

    #[test]
    fn loads_korean() {
        let l = load("Korean").expect("Korean locale");
        assert!(!l.directive.is_empty());
    }

    #[test]
    fn missing_locale_fails() {
        assert!(load("Klingon").is_err());
    }
}
