mod defaults;
mod paths;
mod persistence;
mod setup;
mod types;
mod updates;

use anyhow::Result;

// Re-export public API
pub use types::Config;

impl Config {
    /// Get or create config directory
    pub fn config_dir() -> Result<std::path::PathBuf> {
        paths::config_dir()
    }

    /// Get config file path
    pub fn config_path() -> Result<std::path::PathBuf> {
        paths::config_path()
    }

    /// Load configuration from disk
    pub fn load() -> Result<Option<Self>> {
        persistence::load_config()
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<()> {
        persistence::save_config(self)
    }

    /// Initialize configuration through interactive prompts
    pub fn initialize() -> Result<Self> {
        setup::initialize_config()
    }

    /// Update the current model
    pub fn update_model(&mut self, model: String) -> Result<()> {
        updates::update_model(self, model)
    }

    /// Update the UI language
    pub fn update_ui_language(&mut self, language: String) -> Result<()> {
        updates::update_ui_language(self, language)
    }

    /// Update the AI language
    pub fn update_ai_language(&mut self, language: String) -> Result<()> {
        updates::update_ai_language(self, language)
    }
}
