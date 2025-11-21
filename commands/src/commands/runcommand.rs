use anyhow::Result;

use crate::i18n::I18n;

/// Handle /runcommand command
pub fn handle_run_command_command(parts: &[&str], i18n: &I18n) -> Result<()> {
    match parts.get(1) {
        Some(&"list") => match crate::tools::command_manager::CommandConfig::load() {
            Ok(config) => {
                let commands = config.list_always_approve_commands();

                if commands.is_empty() {
                    println!(
                        "\n\x1b[90m[i] {}\x1b[0m\n",
                        i18n.get("runcommand_no_commands")
                    );
                } else {
                    println!("\n\x1b[1;33m{}:\x1b[0m", i18n.get("runcommand_list_header"));
                    for (i, cmd) in commands.iter().enumerate() {
                        println!("  \x1b[32m[{}]\x1b[0m {}", i + 1, cmd);
                    }
                    println!();
                }
            }
            Err(e) => eprintln!(
                "\n\x1b[31m[X] {}:\x1b[0m {}\n",
                i18n.get("runcommand_load_config_failed"),
                e
            ),
        },
        Some(&"add") => {
            if let Some(cmd) = parts.get(2) {
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!(
                            "\n\x1b[31m[X] {}:\x1b[0m {}\n",
                            i18n.get("runcommand_load_config_failed"),
                            e
                        );
                        return Ok(());
                    }
                };

                if config.add_always_approve_command(cmd) {
                    config.save()?;
                    println!(
                        "\n\x1b[32m[OK]\x1b[0m {}\n",
                        i18n.get("runcommand_add_ok").replace("{}", cmd)
                    );
                } else {
                    println!(
                        "\n\x1b[33m[!] {}\n",
                        i18n.get("runcommand_add_exists").replace("{}", cmd)
                    );
                }
            } else {
                println!(
                    "\n\x1b[33m[!] {}:\x1b[0m /runcommand add <command>\n",
                    i18n.get("usage")
                );
            }
        }
        Some(&"del") | Some(&"remove") => {
            if let Some(cmd) = parts.get(2) {
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!(
                            "\n\x1b[31m[X] {}:\x1b[0m {}\n",
                            i18n.get("runcommand_load_config_failed"),
                            e
                        );
                        return Ok(());
                    }
                };

                if config.remove_always_approve_command(cmd) {
                    config.save()?;
                    println!(
                        "\n\x1b[32m[OK]\x1b[0m {}\n",
                        i18n.get("runcommand_del_ok").replace("{}", cmd)
                    );
                } else {
                    println!(
                        "\n\x1b[33m[!] {}\n",
                        i18n.get("runcommand_del_not_found").replace("{}", cmd)
                    );
                }
            } else {
                println!(
                    "\n\x1b[33m[!] {}:\x1b[0m /runcommand del <command>\n",
                    i18n.get("usage")
                );
            }
        }
        Some(&"info") => {
            if let Some(id_str) = parts.get(2) {
                match crate::tools::command_manager::CommandConfig::load() {
                    Ok(config) => {
                        if let Some(cmd) = config.get_background_command(id_str) {
                            println!("\n\x1b[1;33m{}:\x1b[0m", i18n.get("runcommand_info_header"));
                            println!("  {} {}", i18n.get("runcommand_info_id"), cmd.id);
                            println!("  {} {}", i18n.get("runcommand_info_command"), cmd.command);
                            println!("  {} {}", i18n.get("runcommand_info_status"), cmd.status);
                            println!(
                                "  {} {}",
                                i18n.get("runcommand_info_started"),
                                cmd.start_time.format("%Y-%m-%d %H:%M:%S UTC")
                            );

                            if let Some(code) = cmd.exit_code {
                                println!("  {} {}", i18n.get("runcommand_info_exit_code"), code);
                            }

                            if let Some(output) = &cmd.output {
                                println!(
                                    "\n\x1b[1;33m{}:\x1b[0m",
                                    i18n.get("runcommand_info_output")
                                );
                                println!("  {}", output.replace("\n", "\n  "));
                            }

                            println!();
                        } else {
                            println!(
                                "\n\x1b[31m[X] {}\x1b[0m\n",
                                i18n.get("runcommand_info_not_found").replace("{}", id_str)
                            );
                        }
                    }
                    Err(e) => eprintln!(
                        "\n\x1b[31m[X] {}:\x1b[0m {}\n",
                        i18n.get("runcommand_load_config_failed"),
                        e
                    ),
                }
            } else {
                println!(
                    "\n\x1b[33m[!] {}:\x1b[0m /runcommand info <id>\n",
                    i18n.get("usage")
                );
            }
        }
        _ => {
            println!(
                "\n\x1b[33m[?] {}:\x1b[0m",
                i18n.get("runcommand_help_header")
            );
            println!(
                "    \x1b[36m/runcommand\x1b[0m list        {}",
                i18n.get("cmd_runcommand_list")
            );
            println!(
                "    \x1b[36m/runcommand\x1b[0m add <cmd>   {}",
                i18n.get("cmd_runcommand_add")
            );
            println!(
                "    \x1b[36m/runcommand\x1b[0m del <cmd>   {}",
                i18n.get("cmd_runcommand_del")
            );
            println!(
                "    \x1b[36m/runcommand\x1b[0m info <id>   {}\n",
                i18n.get("cmd_runcommand_info")
            );
        }
    }
    Ok(())
}
