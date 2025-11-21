use super::session::ChatSession;
use crate::config::Config;
use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

/// Get or create sessions directory
pub fn sessions_dir() -> Result<PathBuf> {
    let dir = Config::config_dir()?.join("sessions");
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Get path for a specific session file
pub fn session_path(id: Uuid) -> Result<PathBuf> {
    Ok(sessions_dir()?.join(format!("{}.json", id)))
}

/// Save a session to disk
pub fn save_session(session: &ChatSession) -> Result<()> {
    let path = session_path(session.id)?;
    let content = serde_json::to_string_pretty(session)?;
    fs::write(path, content)?;
    Ok(())
}

/// Load a session from disk
pub fn load_session(id: Uuid) -> Result<ChatSession> {
    let path = session_path(id)?;
    let content = fs::read_to_string(path)?;
    let session: ChatSession = serde_json::from_str(&content)?;
    Ok(session)
}

/// List all sessions sorted by most recent first
pub fn list_all_sessions() -> Result<Vec<ChatSession>> {
    let dir = sessions_dir()?;
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
