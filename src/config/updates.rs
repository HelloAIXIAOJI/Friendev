use anyhow::Result;
use super::types::Config;
use super::persistence;

/// Update the current model
pub fn update_model(config: &mut Config, model: String) -> Result<()> {
    config.current_model = model;
    persistence::save_config(config)
}

/// Update the UI language
pub fn update_ui_language(config: &mut Config, language: String) -> Result<()> {
    config.ui_language = language;
    persistence::save_config(config)
}

/// Update the AI language
pub fn update_ai_language(config: &mut Config, language: String) -> Result<()> {
    config.ai_language = language;
    persistence::save_config(config)
}
