use anyhow::Result;
use std::env;
use uuid::Uuid;

use crate::api::ApiClient;
use crate::config::Config;
use crate::history::ChatSession;
use crate::i18n::I18n;

/// 处理命令，返回 Ok(()) 如果成功处理，Err 如果发生错误
pub async fn handle_command(
    command: &str,
    config: &mut Config,
    session: &mut ChatSession,
    api_client: &mut ApiClient,
) -> Result<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let i18n = I18n::new(&config.ui_language);
    
    match parts.get(0) {
        Some(&"/exit") => {
            println!("\n\x1b[36m{}\x1b[0m\n", i18n.get("goodbye"));
            std::process::exit(0);
        }
        Some(&"/help") => {
            print_help(&i18n);
        }
        Some(&"/model") => {
            handle_model_command(&parts, config, api_client, &i18n).await?;
        }
        Some(&"/history") => {
            handle_history_command(&parts, config, session, &i18n)?;
        }
        Some(&"/language") | Some(&"/lang") => {
            handle_language_command(&parts, config, &i18n)?;
        }
        Some(&"/agents.md") => {
            handle_agents_md_command(session, &i18n).await?;
        }
        Some(&"/runcommand") => {
            handle_run_command_command(&parts, &i18n)?;
        }
        _ => {
            println!("\n\x1b[31m[X] {}: {}\x1b[0m\n", i18n.get("unknown_command"), command);
        }
    }
    
    Ok(())
}

/// 处理 /model 命令
async fn handle_model_command(
    parts: &[&str],
    config: &mut Config,
    api_client: &mut ApiClient,
    i18n: &I18n,
) -> Result<()> {
    match parts.get(1) {
        Some(&"list") => {
            println!("\n\x1b[36m[*] {}\x1b[0m", i18n.get("loading_models"));
            match api_client.list_models().await {
                Ok(models) => {
                    println!("\n\x1b[1;33m{}:\x1b[0m", i18n.get("available_models"));
                    for (i, model) in models.iter().enumerate() {
                        if model == &config.current_model {
                            println!("  \x1b[32m[*]\x1b[0m \x1b[1m{}\x1b[0m. {}", i + 1, model);
                        } else {
                            println!("  \x1b[90m[ ]\x1b[0m {}. {}", i + 1, model);
                        }
                    }
                    println!();
                }
                Err(e) => eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}", i18n.get("failed_load_models"), e),
            }
        }
        Some(&"switch") => {
            if let Some(model_name) = parts.get(2) {
                config.update_model(model_name.to_string())?;
                // 重新创建 API 客户端以使用新模型
                *api_client = ApiClient::new(config.clone());
                println!("\n\x1b[32m[OK]\x1b[0m {} \x1b[1m{}\x1b[0m\n", i18n.get("switched_model"), model_name);
            } else {
                println!("\n\x1b[33m[!] {}:\x1b[0m /model switch <model_name>\n", i18n.get("usage"));
            }
        }
        _ => {
            println!("\n\x1b[33m[?] {}:\x1b[0m", i18n.get("help_model"));
            println!("    \x1b[36m/model\x1b[0m list          {}", i18n.get("cmd_model_list"));
            println!("    \x1b[36m/model\x1b[0m switch <name> {}\n", i18n.get("cmd_model_switch"));
        }
    }
    Ok(())
}

/// 处理 /history 命令
fn handle_history_command(
    parts: &[&str],
    _config: &mut Config,
    session: &mut ChatSession,
    i18n: &I18n,
) -> Result<()> {
    match parts.get(1) {
        Some(&"list") => {
            let sessions = ChatSession::list_all()?;
            // Filter out sessions with 0 messages
            let filtered_sessions: Vec<_> = sessions.into_iter()
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
            println!("\n\x1b[32m[OK]\x1b[0m {} {}\n", i18n.get("created_session"), session.id);
        }
        Some(&"del") | Some(&"delete") => {
            if let Some(id_str) = parts.get(2) {
                match Uuid::parse_str(id_str) {
                    Ok(id) => {
                        if id == session.id {
                            eprintln!("\n\x1b[31m[X] {}\x1b[0m\n", i18n.get("cannot_delete_current"));
                        } else {
                            match ChatSession::load(id) {
                                Ok(s) => {
                                    s.delete()?;
                                    println!("\n\x1b[32m[OK]\x1b[0m {} {}\n", i18n.get("deleted_session"), id);
                                }
                                Err(e) => eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("failed_load_session"), e),
                            }
                        }
                    }
                    Err(_) => eprintln!("\n\x1b[31m[X] {}\x1b[0m\n", i18n.get("invalid_uuid")),
                }
            } else {
                println!("\n\x1b[33m[!] {}:\x1b[0m /history del <id>\n", i18n.get("usage"));
            }
        }
        Some(&"switch") => {
            if let Some(id_str) = parts.get(2) {
                match Uuid::parse_str(id_str) {
                    Ok(id) => {
                        match ChatSession::load(id) {
                            Ok(loaded_session) => {
                                *session = loaded_session;
                                println!("\n\x1b[32m[OK]\x1b[0m {}: {}", i18n.get("switched_session"), session.id);
                                println!("     \x1b[36m[DIR]\x1b[0m \x1b[2m{}\x1b[0m\n", session.working_directory.display());
                            }
                            Err(e) => eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("failed_load_session"), e),
                        }
                    }
                    Err(_) => eprintln!("\n\x1b[31m[X] {}\x1b[0m\n", i18n.get("invalid_uuid")),
                }
            } else {
                println!("\n\x1b[33m[!] {}:\x1b[0m /history switch <id>\n", i18n.get("usage"));
            }
        }
        _ => {
            println!("\n\x1b[33m[?] {}:\x1b[0m", i18n.get("help_history"));
            println!("    \x1b[36m/history\x1b[0m list        {}", i18n.get("cmd_history_list"));
            println!("    \x1b[36m/history\x1b[0m new         {}", i18n.get("cmd_history_new"));
            println!("    \x1b[36m/history\x1b[0m switch <id> {}", i18n.get("cmd_history_switch"));
            println!("    \x1b[36m/history\x1b[0m del <id>    {}\n", i18n.get("cmd_history_del"));
        }
    }
    Ok(())
}

/// 处理 /language 命令
fn handle_language_command(
    parts: &[&str],
    config: &mut Config,
    i18n: &I18n,
) -> Result<()> {
    match parts.get(1) {
        Some(&"ui") => {
            if let Some(lang) = parts.get(2) {
                config.update_ui_language(lang.to_string())?;
                let new_i18n = I18n::new(lang);
                println!("\n\x1b[32m[OK]\x1b[0m {} {}\n", new_i18n.get("ui_language_set"), lang);
            } else {
                println!("\n\x1b[36m[>]\x1b[0m {}: {}\n", i18n.get("current_ui_lang"), config.ui_language);
                println!("\x1b[33m[!] {}:\x1b[0m /language ui <lang>", i18n.get("usage"));
                println!("    {}\n", i18n.get("supported_languages"));
            }
        }
        Some(&"ai") => {
            if let Some(lang) = parts.get(2) {
                config.update_ai_language(lang.to_string())?;
                println!("\n\x1b[32m[OK]\x1b[0m {} {}\n", i18n.get("ai_language_set"), lang);
            } else {
                println!("\n\x1b[36m[>]\x1b[0m {}: {}\n", i18n.get("current_ai_lang"), config.ai_language);
                println!("\x1b[33m[!] {}:\x1b[0m /language ai <lang>", i18n.get("usage"));
                println!("    {}\n", i18n.get("supported_languages"));
            }
        }
        _ => {
            println!("\n\x1b[33m[?] {}:\x1b[0m", i18n.get("help_language"));
            println!("    \x1b[36m/language\x1b[0m ui <lang>  {}", i18n.get("cmd_language_ui"));
            println!("    \x1b[36m/language\x1b[0m ai <lang>  {}", i18n.get("cmd_language_ai"));
            println!("\n    {}\n", i18n.get("supported_languages"));
        }
    }
    Ok(())
}

/// 打印帮助信息
fn print_help(i18n: &I18n) {
    use colored::Colorize;
    
    println!("\n{}", i18n.get("help_title").bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_black());
    
    // 模型命令
    println!("\n{}", i18n.get("help_model").yellow().bold());
    println!("  {} {:25} {}", "·".bright_black(), "/model list".cyan(), i18n.get("cmd_model_list").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/model switch <name>".cyan(), i18n.get("cmd_model_switch").dimmed());
    
    // 历史命令
    println!("\n{}", i18n.get("help_history").yellow().bold());
    println!("  {} {:25} {}", "·".bright_black(), "/history list".cyan(), i18n.get("cmd_history_list").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/history new".cyan(), i18n.get("cmd_history_new").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/history switch <id>".cyan(), i18n.get("cmd_history_switch").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/history del <id>".cyan(), i18n.get("cmd_history_del").dimmed());
    
    // 语言命令
    println!("\n{}", i18n.get("help_language").yellow().bold());
    println!("  {} {:25} {}", "·".bright_black(), "/language ui <lang>".cyan(), i18n.get("cmd_language_ui").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/language ai <lang>".cyan(), i18n.get("cmd_language_ai").dimmed());
    
    // 其他命令
    println!("\n{}", i18n.get("help_other").yellow().bold());
    println!("  {} {:25} {}", "·".bright_black(), "/help".cyan(), i18n.get("cmd_help").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/exit".cyan(), i18n.get("cmd_exit").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/agents.md".cyan(), i18n.get("cmd_agents_md").dimmed());
    
    // 运行命令
    println!("\n{}", i18n.get("help_runcommand").yellow().bold());
    println!("  {} {:25} {}", "·".bright_black(), "/runcommand list".cyan(), i18n.get("cmd_runcommand_list").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/runcommand add <cmd>".cyan(), i18n.get("cmd_runcommand_add").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/runcommand del <cmd>".cyan(), i18n.get("cmd_runcommand_del").dimmed());
    println!("  {} {:25} {}", "·".bright_black(), "/runcommand info <id>".cyan(), i18n.get("cmd_runcommand_info").dimmed());
    
    println!("\n{}", "═".repeat(60).bright_black());
    println!();
}

/// 处理 /agents.md 命令 - 返回提示词（由 main.rs 自动发送给 AI）
pub async fn handle_agents_md_command(
    session: &ChatSession,
    _i18n: &I18n,
) -> Result<String> {
    println!("\n\x1b[33m[*] Analyzing project structure...\x1b[0m");
    
    // 生成分析提示词
    let analysis_prompt = crate::agents::generate_agents_analysis_prompt(&session.working_directory)?;
    
    println!("\x1b[32m[OK]\x1b[0m Sending to AI for AGENTS.md generation...\n");
    
    Ok(analysis_prompt)
}

/// 处理 /runcommand 命令
fn handle_run_command_command(parts: &[&str], _i18n: &I18n) -> Result<()> {
    match parts.get(1) {
        Some(&"list") => {
            match crate::tools::command_manager::CommandConfig::load() {
                Ok(config) => {
                    let commands = config.list_always_approve_commands();
                    
                    if commands.is_empty() {
                        println!("\n\x1b[90m[i] No commands require approval\x1b[0m\n");
                    } else {
                        println!("\n\x1b[1;33mCommands requiring approval:\x1b[0m");
                        for (i, cmd) in commands.iter().enumerate() {
                            println!("  \x1b[32m[{}]\x1b[0m {}", i + 1, cmd);
                        }
                        println!();
                    }
                }
                Err(e) => eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e),
            }
        }
        Some(&"add") => {
            if let Some(cmd) = parts.get(2) {
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e);
                        return Ok(());
                    }
                };
                
                if config.add_always_approve_command(cmd) {
                    config.save()?;
                    println!("\n\x1b[32m[OK]\x1b[0m Added '{}' to approval list\n", cmd);
                } else {
                    println!("\n\x1b[33m[!] '{}' is already in approval list\n", cmd);
                }
            } else {
                println!("\n\x1b[33m[!] Usage:\x1b[0m /runcommand add <command>\n");
            }
        }
        Some(&"del") | Some(&"remove") => {
            if let Some(cmd) = parts.get(2) {
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e);
                        return Ok(());
                    }
                };
                
                if config.remove_always_approve_command(cmd) {
                    config.save()?;
                    println!("\n\x1b[32m[OK]\x1b[0m Removed '{}' from approval list\n", cmd);
                } else {
                    println!("\n\x1b[33m[!] '{}' is not in approval list\n", cmd);
                }
            } else {
                println!("\n\x1b[33m[!] Usage:\x1b[0m /runcommand del <command>\n");
            }
        }
        Some(&"info") => {
            if let Some(id_str) = parts.get(2) {
                match crate::tools::command_manager::CommandConfig::load() {
                    Ok(config) => {
                        if let Some(cmd) = config.get_background_command(id_str) {
                            println!("\n\x1b[1;33mBackground Command Info:\x1b[0m");
                            println!("  ID: {}", cmd.id);
                            println!("  Command: {}", cmd.command);
                            println!("  Status: {}", cmd.status);
                            println!("  Started: {}", cmd.start_time.format("%Y-%m-%d %H:%M:%S UTC"));
                            
                            if let Some(code) = cmd.exit_code {
                                println!("  Exit Code: {}", code);
                            }
                            
                            if let Some(output) = &cmd.output {
                                println!("\n\x1b[1;33mOutput:\x1b[0m");
                                println!("  {}", output.replace("\n", "\n  "));
                            }
                            
                            println!();
                        } else {
                            println!("\n\x1b[31m[X] Command with ID '{}' not found\x1b[0m\n", id_str);
                        }
                    }
                    Err(e) => eprintln!("\n\x1b[31m[X] Failed to load command config:\x1b[0m {}\n", e),
                }
            } else {
                println!("\n\x1b[33m[!] Usage:\x1b[0m /runcommand info <id>\n");
            }
        }
        _ => {
            println!("\n\x1b[33m[?] Help for /runcommand:\x1b[0m");
            println!("    \x1b[36m/runcommand\x1b[0m list        List commands requiring approval");
            println!("    \x1b[36m/runcommand\x1b[0m add <cmd>   Add command to approval list");
            println!("    \x1b[36m/runcommand\x1b[0m del <cmd>   Remove command from approval list");
            println!("    \x1b[36m/runcommand\x1b[0m info <id>   Show background command details\n");
        }
    }
    Ok(())
}
