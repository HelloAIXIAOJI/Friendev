mod en_us;
mod zh_cn;
mod loader;

use std::collections::HashMap;
use loader::load_messages;

// Re-export language validation and supported languages
pub use loader::{is_language_supported, supported_languages_str};

/// Re-export SUPPORTED_LANGUAGES directly from loader (single source of truth)
pub use loader::SUPPORTED_LANGUAGES;

/// Internationalization context storing language and translations
#[derive(Debug, Clone)]
pub struct I18n {
    language: String,
    messages: HashMap<String, String>,
}

impl I18n {
    /// Create a new I18n context for the specified language
    pub fn new(language: &str) -> Self {
        let messages = load_messages(language);
        
        Self {
            language: language.to_string(),
            messages,
        }
    }

    /// Get a localized message by key
    /// Returns the message if found, otherwise returns a placeholder with the key
    pub fn get(&self, key: &str) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("[Missing: {}]", key))
    }
}
