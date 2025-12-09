use super::persistence;
use super::types::Config;
use anyhow::{anyhow, Result};
use i18n::{is_language_supported, supported_languages_str};

/// Update the current model
pub fn update_model(config: &mut Config, model: String) -> Result<()> {
    config.current_model = model;
    persistence::save_config(config)
}

/// Update the UI language (only allows supported languages)
pub fn update_ui_language(config: &mut Config, language: String) -> Result<()> {
    if !is_language_supported(&language) {
        return Err(anyhow!(
            "Unsupported UI language: '{}'. Supported languages: {}",
            language,
            supported_languages_str()
        ));
    }
    config.ui_language = language;
    persistence::save_config(config)
}

/// Update the AI language
pub fn update_ai_language(config: &mut Config, language: String) -> Result<()> {
    config.ai_language = language;
    persistence::save_config(config)
}

/// Update the Shorekeeper (Smart Approval) model
pub fn update_shorekeeper_model(config: &mut Config, model: String) -> Result<()> {
    config.shorekeeper_model = Some(model);
    persistence::save_config(config)
}
