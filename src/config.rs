use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_url: String,
    pub current_model: String,
    #[serde(default = "default_ui_language")]
    pub ui_language: String,
    #[serde(default = "default_ai_language")]
    pub ai_language: String,
}

fn default_ui_language() -> String {
    "en".to_string()
}

fn default_ai_language() -> String {
    "en".to_string()
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("无法获取配置目录"))?
            .join("friendev");
        fs::create_dir_all(&config_dir)?;
        Ok(config_dir)
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.json"))
    }

    pub fn load() -> Result<Option<Self>> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(Some(config))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn initialize() -> Result<Self> {
        println!("欢迎使用 Friendev！首次使用需要初始化配置。\n");
        
        let api_key = dialoguer::Input::<String>::new()
            .with_prompt("请输入 OpenAI API Key")
            .interact_text()?;

        let api_url = dialoguer::Input::<String>::new()
            .with_prompt("请输入 OpenAI API URL")
            .default("https://api.openai.com/v1".to_string())
            .interact_text()?;

        let current_model = dialoguer::Input::<String>::new()
            .with_prompt("请输入默认模型")
            .default("gpt-4".to_string())
            .interact_text()?;

        let config = Config {
            api_key,
            api_url,
            current_model,
            ui_language: "en".to_string(),
            ai_language: "en".to_string(),
        };

        config.save()?;
        println!("\n✓ 配置已保存！\n");
        Ok(config)
    }

    pub fn update_model(&mut self, model: String) -> Result<()> {
        self.current_model = model;
        self.save()
    }
    
    pub fn update_ui_language(&mut self, language: String) -> Result<()> {
        self.ui_language = language;
        self.save()
    }
    
    pub fn update_ai_language(&mut self, language: String) -> Result<()> {
        self.ai_language = language;
        self.save()
    }
}
