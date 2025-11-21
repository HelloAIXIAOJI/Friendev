use super::persistence::{list_all_sessions, session_path};
use super::session::ChatSession;
use crate::ui::get_i18n;
use anyhow::Result;
use std::fs;

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
        let i18n = get_i18n();
        println!(
            "\x1b[33m[*] {}\x1b[0m",
            i18n.get("history_cleanup_empty")
                .replace("{}", &deleted_count.to_string())
        );
    }

    Ok(())
}
