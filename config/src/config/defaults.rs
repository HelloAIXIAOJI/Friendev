/// Default maximum retries for API requests
pub fn default_max_retries() -> u32 {
    3
}

/// Default retry delay in milliseconds
pub fn default_retry_delay_ms() -> u64 {
    300
}

use crate::i18n::SUPPORTED_LANGUAGES;

/// Default UI language (first supported language)
pub fn default_ui_language() -> String {
    SUPPORTED_LANGUAGES[0].to_string()
}

/// Default AI language (first supported language)
pub fn default_ai_language() -> String {
    SUPPORTED_LANGUAGES[0].to_string()
}
