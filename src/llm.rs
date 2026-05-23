//! LLM client + provider configs — to be filled in v0.1.1.
//!
//! Port of llm_client.py + llm_configs.py.
//!
//! Architecture (matches python):
//!   - `LlmConfig` trait — three methods per provider:
//!       * `headers() -> HeaderMap`
//!       * `body(prompt) -> serde_json::Value`
//!       * `parse(response: serde_json::Value) -> anyhow::Result<String>`
//!   - Generic `call_api(config, prompt)` does POST + 429 retry with
//!     exponential backoff + timeout, calls `config.parse` on success.
//!   - Provider selection from env: which `INPUT_*_API_KEY` is set determines
//!     the active provider; explicit `INPUT_MODEL` can override.

#![allow(dead_code)] // scaffold

pub mod client;
pub mod providers;

pub use client::call_api;
pub use providers::{Provider, select_from_env};
