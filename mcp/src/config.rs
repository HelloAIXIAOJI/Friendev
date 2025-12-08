use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;
use config::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum McpServerConfig {
    // New format with explicit transport
    Sse {
        transport: SseTransport,
        url: String,
        /// Bearer token for authentication
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_token: Option<String>,
        /// Custom headers to include with requests
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
    },
    Http {
        transport: HttpTransport,
        url: String,
        /// Bearer token for authentication  
        #[serde(skip_serializing_if = "Option::is_none")]
        auth_token: Option<String>,
        /// Custom headers to include with requests
        #[serde(skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
        /// Allow stateless connections (default: true)
        #[serde(skip_serializing_if = "Option::is_none")]
        stateless: Option<bool>,
    },
    ChildProcess {
        transport: ChildProcessTransport,
        command: String,
        #[serde(default)]
        args: Vec<String>,
        env: Option<HashMap<String, String>>,
    },
    // Legacy format (backward compatibility) - no transport field
    Legacy {
        command: String,
        #[serde(default)]
        args: Vec<String>,
        env: Option<HashMap<String, String>>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum SseTransport {
    Sse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ChildProcessTransport {
    ChildProcess,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum HttpTransport {
    Http,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct McpConfig {
    #[serde(rename = "mcpServers")]
    pub mcp_servers: HashMap<String, McpServerConfig>,
}

impl McpConfig {
    pub fn load() -> Result<Self> {
        let mut config = McpConfig::default();
        let friendev_config_dir = Config::config_dir()?;
        let mcp_config_dir = friendev_config_dir.join("mcps");

        if !mcp_config_dir.exists() {
             return Ok(config);
        }

        let entries = std::fs::read_dir(mcp_config_dir)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = std::fs::read_to_string(&path)?;
                // We assume the JSON file content structure matches McpConfig (has mcpServers key)
                // or is it just the map?
                // The prompt says "define MCP servers (one json can define multiple)".
                // Usually `mcpServers` is the key.
                match serde_json::from_str::<McpConfig>(&content) {
                    Ok(partial_config) => {
                        config.mcp_servers.extend(partial_config.mcp_servers);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(config)
    }

    pub fn create_new(name: &str) -> Result<std::path::PathBuf> {
        let friendev_config_dir = Config::config_dir()?;
        let mcp_config_dir = friendev_config_dir.join("mcps");

        if !mcp_config_dir.exists() {
            std::fs::create_dir_all(&mcp_config_dir)?;
        }

        let mut path = mcp_config_dir.join(name);
        if path.extension().is_none() {
            path.set_extension("json");
        }
        
        if path.exists() {
             anyhow::bail!("File already exists: {}", path.display());
        }

        let empty_config = serde_json::json!({
            "mcpServers": {}
        });

        let content = serde_json::to_string_pretty(&empty_config)?;
        std::fs::write(&path, content)?;

        Ok(path)
    }

    /// Get the MCP configuration directory path
    pub fn config_dir() -> Result<std::path::PathBuf> {
        let friendev_config_dir = Config::config_dir()?;
        Ok(friendev_config_dir.join("mcps"))
    }
}
