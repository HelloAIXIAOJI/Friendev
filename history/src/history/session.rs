use super::management;
use super::persistence;
use super::types::Message;
use crate::ui::get_i18n;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// Chat session containing messages and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: Uuid,
    pub working_directory: PathBuf,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ChatSession {
    /// Create a new session
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

    /// Add a message to the session
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    /// Save session to disk
    pub fn save(&self) -> Result<()> {
        persistence::save_session(self)
    }

    /// Load session from disk
    pub fn load(id: Uuid) -> Result<Self> {
        persistence::load_session(id)
    }

    /// List all sessions sorted by most recent first
    pub fn list_all() -> Result<Vec<ChatSession>> {
        persistence::list_all_sessions()
    }

    /// Get session summary from first user message
    pub fn summary(&self) -> String {
        let i18n = get_i18n();
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
            .unwrap_or_else(|| i18n.get("history_new_chat_summary"));

        first_user_msg
    }

    /// Delete this session from disk
    pub fn delete(&self) -> Result<()> {
        management::delete_session(self)
    }

    /// Automatically delete all sessions with 0 messages
    pub fn cleanup_empty_sessions() -> Result<()> {
        management::cleanup_empty_sessions()
    }
}
