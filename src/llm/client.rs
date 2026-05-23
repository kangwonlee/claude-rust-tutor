//! Port of llm_client.py::LLMAPIClient.call_api.
//!
//! Single sequential POST; on HTTP 429 retries with exponential backoff
//! (5s × 2^attempt), max 3 retries, 60s per-request timeout.

use std::thread;
use std::time::Duration;

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;

use super::providers::Provider;

const TIMEOUT_SEC: u64 = 60;
const BASE_DELAY_SEC: f64 = 5.0;
const MAX_RETRIES: u32 = 3;

pub fn call_api(provider: &Provider, prompt: &str) -> anyhow::Result<String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(TIMEOUT_SEC))
        .build()?;

    let mut headers = HeaderMap::new();
    for (k, v) in provider.headers() {
        headers.insert(
            HeaderName::from_bytes(k.as_bytes())
                .map_err(|e| anyhow::anyhow!("bad header name {k}: {e}"))?,
            HeaderValue::from_str(&v)
                .map_err(|e| anyhow::anyhow!("bad header value for {k}: {e}"))?,
        );
    }

    let url = provider.api_url();
    let body = provider.body(prompt);

    for attempt in 0..=MAX_RETRIES {
        let resp = match client.post(&url).headers(headers.clone()).json(&body).send() {
            Ok(r) => r,
            Err(e) if e.is_timeout() => {
                anyhow::bail!("request timed out after {TIMEOUT_SEC}s: {e}");
            }
            Err(e) => {
                anyhow::bail!("network error: {e}");
            }
        };

        let status = resp.status().as_u16();
        if status == 200 {
            let parsed: Value = resp
                .json()
                .map_err(|e| anyhow::anyhow!("response not JSON: {e}"))?;
            return provider.parse(&parsed);
        }
        if status == 429 {
            if attempt == MAX_RETRIES {
                anyhow::bail!("rate limit (429) — exhausted {MAX_RETRIES} retries");
            }
            let delay = BASE_DELAY_SEC * (1u32 << attempt) as f64;
            log::warn!(
                "rate limit (429); sleeping {delay:.1}s (attempt {}/{MAX_RETRIES})",
                attempt + 1
            );
            thread::sleep(Duration::from_secs_f64(delay));
            continue;
        }
        let body_text = resp.text().unwrap_or_default();
        anyhow::bail!("API request failed: status {status} body {body_text}");
    }
    anyhow::bail!("retry loop exited unexpectedly")
}
