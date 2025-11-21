use anyhow::Result;
use std::env;
use uuid::Uuid;

use crate::config::Config;
use crate::history::ChatSession;
use crate::i18n::I18n;

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
                .filter(|s| s.messages.len() > 0)
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
            let working_dir = env::current_dir()?;
            let new_session = ChatSession::new(working_dir);
            new_session.save()?;
            *session = new_session;
            println!(
                "\n\x1b[32m[OK]\x1b[0m {} {}\n",
                i18n.get("created_session"),
                session.id
            );
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
                    Ok(id) => match ChatSession::load(id) {
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
                    },
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
    Ok(())
}
