use anyhow::Result;
use std::env;
use uuid::Uuid;
use dialoguer::{theme::ColorfulTheme, Select};

use ::history::ChatSession;
use config::Config;
use i18n::I18n;

/// Handle /history command
pub fn handle_history_command(
    parts: &[&str],
    _config: &mut Config,
    session: &mut ChatSession,
    i18n: &I18n,
) -> Result<()> {
    match parts.get(1) {
        Some(&"list") => {
            let sessions = ChatSession::list_all()?;
            let filtered_sessions: Vec<_> = sessions
                .into_iter()
                .filter(|s| {
                    !s.messages.is_empty() && s.working_directory == session.working_directory
                })
                .collect();

            if filtered_sessions.is_empty() {
                println!("\n\x1b[90m[i] {}\x1b[0m\n", i18n.get("no_history"));
            } else {
                println!("\n\x1b[1;33m{}:\x1b[0m", i18n.get("chat_history"));
                for (i, s) in filtered_sessions.iter().enumerate() {
                    if s.id == session.id {
                        println!(
                            "  \x1b[32m[*]\x1b[0m \x1b[1m{}\x1b[0m. \x1b[90m{}\x1b[0m\n      \x1b[36m>\x1b[0m {} \x1b[90m({} {})\x1b[0m\n      \x1b[2m{}\x1b[0m",
                            i + 1,
                            s.id,
                            s.summary(),
                            s.messages.len(),
                            i18n.get("messages"),
                            s.working_directory.display()
                        );
                    } else {
                        println!(
                            "  \x1b[90m[ ]\x1b[0m {}. \x1b[90m{}\x1b[0m\n      {}  \x1b[90m({} {})\x1b[0m\n      \x1b[2m{}\x1b[0m",
                            i + 1,
                            s.id,
                            s.summary(),
                            s.messages.len(),
                            i18n.get("messages"),
                            s.working_directory.display()
                        );
                    }
                }
                println!();
            }
        }
        Some(&"new") => {
            create_new_session(session, i18n)?;
        }
        Some(&"del") | Some(&"delete") => {
            if let Some(id_str) = parts.get(2) {
                match Uuid::parse_str(id_str) {
                    Ok(id) => {
                        if id == session.id {
                            eprintln!(
                                "\n\x1b[31m[X] {}\x1b[0m\n",
                                i18n.get("cannot_delete_current")
                            );
                        } else {
                            match ChatSession::load(id) {
                                Ok(s) => {
                                    s.delete()?;
                                    println!(
                                        "\n\x1b[32m[OK]\x1b[0m {} {}\n",
                                        i18n.get("deleted_session"),
                                        id
                                    );
                                }
                                Err(e) => {
                                    eprintln!(
                                        "\n\x1b[31m[X] {}:\x1b[0m {}\n",
                                        i18n.get("failed_load_session"),
                                        e
                                    )
                                }
                            }
                        }
                    }
                    Err(_) => eprintln!("\n\x1b[31m[X] {}\x1b[0m\n", i18n.get("invalid_uuid")),
                }
            } else {
                println!(
                    "\n\x1b[33m[!] {}:\x1b[0m /history del <id>\n",
                    i18n.get("usage")
                );
            }
        }
        Some(&"switch") => {
            if let Some(id_str) = parts.get(2) {
                match Uuid::parse_str(id_str) {
                    Ok(id) => switch_session(id, session, i18n),
                    Err(_) => eprintln!("\n\x1b[31m[X] {}\x1b[0m\n", i18n.get("invalid_uuid")),
                }
            } else {
                println!(
                    "\n\x1b[33m[!] {}:\x1b[0m /history switch <id>\n",
                    i18n.get("usage")
                );
            }
        }
        _ => {
            // Interactive mode if no subcommand
            if parts.len() == 1 {
                handle_interactive_history(session, i18n)?;
            } else {
                println!("\n\x1b[33m[?] {}:\x1b[0m", i18n.get("help_history"));
                println!(
                    "    \x1b[36m/history\x1b[0m list        {}",
                    i18n.get("cmd_history_list")
                );
                println!(
                    "    \x1b[36m/history\x1b[0m new         {}",
                    i18n.get("cmd_history_new")
                );
                println!(
                    "    \x1b[36m/history\x1b[0m switch <id> {}",
                    i18n.get("cmd_history_switch")
                );
                println!(
                    "    \x1b[36m/history\x1b[0m del <id>    {}\n",
                    i18n.get("cmd_history_del")
                );
            }
        }
    }
    Ok(())
}

fn create_new_session(session: &mut ChatSession, i18n: &I18n) -> Result<()> {
    let working_dir = env::current_dir()?;
    let new_session = ChatSession::new(working_dir);
    new_session.save()?;
    *session = new_session;
    println!(
        "\n\x1b[32m[OK]\x1b[0m {} {}\n",
        i18n.get("created_session"),
        session.id
    );
    Ok(())
}

fn switch_session(id: Uuid, session: &mut ChatSession, i18n: &I18n) {
    match ChatSession::load(id) {
        Ok(loaded_session) => {
            *session = loaded_session;
            println!(
                "\n\x1b[32m[OK]\x1b[0m {}: {}",
                i18n.get("switched_session"),
                session.id
            );
            println!(
                "     \x1b[36m[DIR]\x1b[0m \x1b[2m{}\x1b[0m\n",
                session.working_directory.display()
            );
        }
        Err(e) => eprintln!(
            "\n\x1b[31m[X] {}:\x1b[0m {}\n",
            i18n.get("failed_load_session"),
            e
        ),
    }
}

fn handle_interactive_history(session: &mut ChatSession, i18n: &I18n) -> Result<()> {
    let sessions = ChatSession::list_all()?;
    let filtered_sessions: Vec<_> = sessions
        .into_iter()
        .filter(|s| {
            !s.messages.is_empty() && s.working_directory == session.working_directory
        })
        .collect();

    if filtered_sessions.is_empty() {
        println!("\n\x1b[90m[i] {}\x1b[0m\n", i18n.get("history_empty"));
        // Offer to create new session even if empty?
        // For now just return, or maybe prompt "Create new session? [Y/n]"
        // But let's stick to the menu style.
    }

    let mut items = Vec::new();
    // Option 0: New Session
    items.push(format!("✨ {}", i18n.get("history_new_session")));

    // Existing sessions
    for s in &filtered_sessions {
        let current_marker = if s.id == session.id {
            format!(" {} ", i18n.get("history_current_session"))
        } else {
            "".to_string()
        };
        
        let summary = s.summary();
        // Truncate summary for menu if too long
        // s.summary() already handles truncation and single line, but let's be safe for display
        let display_summary = if summary.chars().count() > 50 {
            let truncated: String = summary.chars().take(47).collect();
            format!("{}...", truncated)
        } else {
            summary
        };

        items.push(format!(
            "{} {}{}", 
            display_summary, 
            s.messages.len(), 
            i18n.get("messages"),
            // current_marker // Put current marker at end or beginning?
        ));
    }

    // Re-build items to include ID or more info if needed, but simple is better.
    // Let's make it cleaner.
    let mut menu_items = Vec::new();
    menu_items.push(format!("✨ \x1b[1m{}\x1b[0m", i18n.get("history_new_session")));

    for s in &filtered_sessions {
        let is_current = s.id == session.id;
        let prefix = if is_current { "\x1b[32m●\x1b[0m" } else { "○" };
        let summary = s.summary();
        let msgs = format!("({} {})", s.messages.len(), i18n.get("messages"));
        
        menu_items.push(format!("{} {} \x1b[90m{}\x1b[0m", prefix, summary, msgs));
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(i18n.get("history_menu_title"))
        .default(0)
        .items(&menu_items)
        .interact_opt()?;

    match selection {
        Some(0) => {
            create_new_session(session, i18n)?;
        }
        Some(index) => {
            // index 0 is New Session, so session index is index - 1
            if let Some(s) = filtered_sessions.get(index - 1) {
                if s.id != session.id {
                    switch_session(s.id, session, i18n);
                } else {
                    // Already on this session
                    println!("\n\x1b[90m[i] {}\x1b[0m\n", i18n.get("session_already_active"));
                }
            }
        }
        None => {
            // User cancelled
            println!("\n\x1b[90m[i] Cancelled\x1b[0m\n");
        }
    }

    Ok(())
}
