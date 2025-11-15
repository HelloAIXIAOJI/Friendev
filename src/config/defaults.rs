/// Default maximum retries for API requests
pub fn default_max_retries() -> u32 {
    3
}

/// Default retry delay in milliseconds
pub fn default_retry_delay_ms() -> u64 {
    300
}

/// Default UI language
pub fn default_ui_language() -> String {
    "en".to_string()
}

/// Default AI language
pub fn default_ai_language() -> String {
    "en".to_string()
}
