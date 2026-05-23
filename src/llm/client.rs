//! Generic HTTP retry client — to be filled in v0.1.1.
//!
//! Port of llm_client.py::LLMAPIClient.call_api. Single sequential request,
//! 429 → exponential backoff (base 5s × 2^attempt), max 3 retries, 60s timeout.

#![allow(dead_code)] // scaffold

use super::providers::Provider;

pub fn call_api(_provider: &Provider, _prompt: &str) -> anyhow::Result<String> {
    // TODO v0.1.1
    anyhow::bail!("client::call_api not yet implemented")
}
