use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: Uuid,
    pub working_directory: PathBuf,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatSession {
    pub fn new(working_directory: PathBuf) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            working_directory,
            messages: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    pub fn sessions_dir() -> Result<PathBuf> {
        let dir = Config::config_dir()?.join("sessions");
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    pub fn session_path(&self) -> Result<PathBuf> {
        Ok(Self::sessions_dir()?.join(format!("{}.json", self.id)))
    }

    pub fn save(&self) -> Result<()> {
        let path = self.session_path()?;
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load(id: Uuid) -> Result<Self> {
        let path = Self::sessions_dir()?.join(format!("{}.json", id));
        let content = fs::read_to_string(path)?;
        let session: ChatSession = serde_json::from_str(&content)?;
        Ok(session)
    }

    pub fn list_all() -> Result<Vec<ChatSession>> {
        let dir = Self::sessions_dir()?;
        let mut sessions = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(session) = serde_json::from_str::<ChatSession>(&content) {
                        sessions.push(session);
                    }
                }
            }
        }

        sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(sessions)
    }

    pub fn summary(&self) -> String {
        let first_user_msg = self
            .messages
            .iter()
            .find(|m| m.role == "user")
            .map(|m| {
                let content = m.content.trim();
                let char_count = content.chars().count();
                if char_count > 50 {
                    let truncated: String = content.chars().take(47).collect();
                    format!("{}...", truncated)
                } else {
                    content.to_string()
                }
            })
            .unwrap_or_else(|| "New Chat".to_string());

        first_user_msg
    }
    
    pub fn delete(&self) -> Result<()> {
        let path = self.session_path()?;
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
