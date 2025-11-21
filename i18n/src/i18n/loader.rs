use super::{en_us, zh_cn};
use std::collections::HashMap;

/// Define supported languages and their message loaders
/// This macro ensures SUPPORTED_LANGUAGES and load_messages stay in sync
macro_rules! define_languages {
    ($($lang:expr => $loader:expr),* $(,)?) => {
        /// Supported UI languages - automatically derived from language definitions
        pub const SUPPORTED_LANGUAGES: &[&str] = &[$($lang),*];

        /// Load messages for the specified language
        pub fn load_messages(language: &str) -> HashMap<String, String> {
            match language {
                $($lang => $loader,)*
                _ => en_us::get_messages(),
            }
        }
    };
}

// Define all supported languages here (single source of truth)
define_languages!(
    "enus" => en_us::get_messages(),
    "1111" => en_us::get_messages(),
    "zhcn" => zh_cn::get_messages()
);

/// Check if a language is supported
pub fn is_language_supported(language: &str) -> bool {
    SUPPORTED_LANGUAGES.contains(&language)
}

/// Supported UI languages as formatted string
pub fn supported_languages_str() -> String {
    SUPPORTED_LANGUAGES.join(", ")
}
