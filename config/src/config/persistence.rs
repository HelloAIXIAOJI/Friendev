use super::paths;
use super::types::Config;
use anyhow::Result;
use std::fs;

/// Load configuration from disk
pub fn load_config() -> Result<Option<Config>> {
    let path = paths::config_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(Some(config))
}

/// Save configuration to disk
pub fn save_config(config: &Config) -> Result<()> {
    let path = paths::config_path()?;
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}
