use anyhow::Result;
use std::fs;
use std::path::PathBuf;

/// Get or create config directory
pub fn config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("无法获取配置目录"))?
        .join("friendev");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

/// Get config file path
pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.json"))
}

/// Get LSP config file path
pub fn lsp_config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("lsp.json"))
}
