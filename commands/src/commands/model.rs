use anyhow::Result;

use api::ApiClient;
use config::Config;
use i18n::I18n;
use ui::{enhanced_output, print_model_list, select_model};

/// Handle /model command
pub async fn handle_model_command(
    parts: &[&str],
    config: &mut Config,
    api_client: &mut ApiClient,
    i18n: &I18n,
) -> Result<()> {
    match parts.get(1) {
        Some(&"list") => {
            // Display model list (non-interactive)
            println!("\n\x1b[36m[*] {}\x1b[0m", i18n.get("loading_models"));
            match api_client.list_models().await {
                Ok(models) => {
                    print_model_list(&models, &config.current_model);
                }
                Err(e) => {
                    let error_msg = format!("{}: {}", i18n.get("failed_load_models"), e);
                    let _ = enhanced_output::print_error(&error_msg);
                }
            }
        }
        Some(&"switch") => {
            if let Some(model_name) = parts.get(2) {
                // Direct switch with model name
                config.update_model(model_name.to_string())?;
                *api_client = ApiClient::new(config.clone());
                
                let success_msg = format!("{} {}", i18n.get("switched_model"), model_name);
                let _ = enhanced_output::print_success(&success_msg);
            } else {
                println!(
                    "\n\x1b[33m[!] {}:\x1b[0m /model switch <model_name>\n",
                    i18n.get("usage")
                );
            }
        }
        None => {
            // Interactive model selector when just "/model" is entered
            println!("\n\x1b[36m[*] {}\x1b[0m\n", i18n.get("loading_models"));
            match api_client.list_models().await {
                Ok(models) => {
                    if models.is_empty() {
                        let _ = enhanced_output::print_warning(&i18n.get("no_models_available"));
                        return Ok(());
                    }
                    
                    // Show interactive selector
                    match select_model(&models, &config.current_model) {
                        Ok(Some(selected_model)) => {
                            // User selected a model
                            if selected_model == config.current_model {
                                println!(
                                    "\n\x1b[90m{} {}\x1b[0m\n",
                                    i18n.get("model_already_active"),
                                    selected_model
                                );
                            } else {
                                config.update_model(selected_model.clone())?;
                                *api_client = ApiClient::new(config.clone());
                                
                                let success_msg = format!("{} {}", i18n.get("switched_model"), selected_model);
                                let _ = enhanced_output::print_success(&success_msg);
                            }
                        }
                        Ok(None) => {
                            // User cancelled (pressed Esc)
                            println!("\n\x1b[90m{}\x1b[0m\n", i18n.get("model_selection_cancelled"));
                        }
                        Err(_e) => {
                            // Fallback to list view if interactive fails
                            eprintln!("\n\x1b[33m[!] {}\x1b[0m", i18n.get("interactive_mode_failed"));
                            print_model_list(&models, &config.current_model);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!("{}: {}", i18n.get("failed_load_models"), e);
                    let _ = enhanced_output::print_error(&error_msg);
                }
            }
        }
        _ => {
            // Unknown subcommand - show help
            println!("\n\x1b[33m[?] {}:\x1b[0m", i18n.get("help_model"));
            println!(
                "    \x1b[36m/model\x1b[0m               {}",
                i18n.get("cmd_model_interactive")
            );
            println!(
                "    \x1b[36m/model\x1b[0m list          {}",
                i18n.get("cmd_model_list")
            );
            println!(
                "    \x1b[36m/model\x1b[0m switch <name> {}\n",
                i18n.get("cmd_model_switch")
            );
        }
    }
    Ok(())
}
