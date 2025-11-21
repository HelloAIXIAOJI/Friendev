use super::defaults;
use serde::{Deserialize, Serialize};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_url: String,
    pub current_model: String,
    #[serde(default = "defaults::default_ui_language")]
    pub ui_language: String,
    #[serde(default = "defaults::default_ai_language")]
    pub ai_language: String,
    #[serde(default = "defaults::default_max_retries")]
    pub max_retries: u32,
    #[serde(default = "defaults::default_retry_delay_ms")]
    pub retry_delay_ms: u64,
}
