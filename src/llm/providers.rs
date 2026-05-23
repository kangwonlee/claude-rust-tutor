//! Provider configs — to be filled in v0.1.1.
//!
//! One config per provider, each implementing `LlmConfig`. Mirrors
//! llm_configs.py's 5 dataclasses: GeminiConfig (default), GrokConfig,
//! NvidiaNIMConfig, ClaudeConfig, PerplexityConfig.

#![allow(dead_code)] // scaffold

use std::env;

#[derive(Debug)]
pub enum Provider {
    Gemini { api_key: String, model: String },
    Claude { api_key: String, model: String },
    Grok { api_key: String, model: String },
    NvidiaNim { api_key: String, model: String },
    Perplexity { api_key: String, model: String },
}

/// Mirror python's `get_model_key_from_env()` — first non-empty key wins,
/// in the fallback order from llm_utils.py.
pub fn select_from_env() -> anyhow::Result<Provider> {
    let model_override = env::var("INPUT_MODEL").ok().filter(|s| !s.is_empty());

    // Order matches python tutor's fallback chain.
    if let Some(key) = nonempty("INPUT_GEMINI-API-KEY") {
        return Ok(Provider::Gemini {
            api_key: key,
            model: model_override.unwrap_or_else(|| "gemini-2.5-flash".into()),
        });
    }
    if let Some(key) = nonempty("INPUT_CLAUDE_API_KEY") {
        return Ok(Provider::Claude {
            api_key: key,
            model: model_override.unwrap_or_else(|| "claude-sonnet-4-20250514".into()),
        });
    }
    if let Some(key) = nonempty("INPUT_GROK-API-KEY") {
        return Ok(Provider::Grok {
            api_key: key,
            model: model_override.unwrap_or_else(|| "grok-code-fast".into()),
        });
    }
    if let Some(key) = nonempty("INPUT_NVIDIA-API-KEY") {
        return Ok(Provider::NvidiaNim {
            api_key: key,
            model: model_override.unwrap_or_else(|| "google/gemma-2-9b-it".into()),
        });
    }
    if let Some(key) = nonempty("INPUT_PERPLEXITY-API-KEY") {
        return Ok(Provider::Perplexity {
            api_key: key,
            model: model_override.unwrap_or_else(|| "sonar".into()),
        });
    }
    anyhow::bail!("no LLM API key set in env (INPUT_*_API_KEY)")
}

fn nonempty(name: &str) -> Option<String> {
    env::var(name).ok().filter(|s| !s.trim().is_empty())
}
