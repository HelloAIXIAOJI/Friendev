use anyhow::Result;

use crate::api::ApiClient;
use crate::config::Config;
use crate::i18n::I18n;

/// Handle /model command
pub async fn handle_model_command(
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
                            println!("  \x1b[32m[*]\x1b[0m \x1b[1m{}.\x1b[0m {}", i + 1, model);
                        } else {
                            println!("  \x1b[90m[ ]\x1b[0m {}. {}", i + 1, model);
                        }
                    }
                    println!();
                }
                Err(e) => eprintln!(
                    "\n\x1b[31m[X] {}:\x1b[0m {}",
                    i18n.get("failed_load_models"),
                    e
                ),
            }
        }
        Some(&"switch") => {
            if let Some(model_name) = parts.get(2) {
                config.update_model(model_name.to_string())?;
                // Recreate API client with new model
                *api_client = ApiClient::new(config.clone());
                println!(
                    "\n\x1b[32m[OK]\x1b[0m {} \x1b[1m{}\x1b[0m\n",
                    i18n.get("switched_model"),
                    model_name
                );
            } else {
                println!(
                    "\n\x1b[33m[!] {}:\x1b[0m /model switch <model_name>\n",
                    i18n.get("usage")
                );
            }
        }
        _ => {
            println!("\n\x1b[33m[?] {}:\x1b[0m", i18n.get("help_model"));
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
