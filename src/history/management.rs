use anyhow::Result;
use std::fs;
use super::session::ChatSession;
use super::persistence::{session_path, list_all_sessions};

/// Delete a session
pub fn delete_session(session: &ChatSession) -> Result<()> {
    let path = session_path(session.id)?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Automatically delete all sessions with 0 messages
pub fn cleanup_empty_sessions() -> Result<()> {
    let sessions = list_all_sessions()?;
    let mut deleted_count = 0;
    
    for session in sessions {
        if session.messages.is_empty() {
            delete_session(&session)?;
            deleted_count += 1;
        }
    }
    
    if deleted_count > 0 {
        println!("\x1b[33m[*] Cleaned up {} empty session(s)\x1b[0m", deleted_count);
    }
    
    Ok(())
}
