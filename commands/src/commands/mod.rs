mod agents;
mod help;
mod history;
mod language;
mod model;
mod runcommand;

use anyhow::Result;

use ::history::ChatSession;
use api::ApiClient;
use config::Config;
use i18n::I18n;

pub use agents::handle_agents_md_command;
pub use help::print_help;

/// Handle command - returns Ok(()) if successfully processed, Err if error
pub async fn handle_command(
    command: &str,
    config: &mut Config,
    session: &mut ChatSession,
    api_client: &mut ApiClient,
) -> Result<()> {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let i18n = I18n::new(&config.ui_language);

    match parts.first() {
        Some(&"/exit") => {
            println!("\n\x1b[36m{}\x1b[0m\n", i18n.get("goodbye"));
            std::process::exit(0);
        }
        Some(&"/help") => {
            print_help(&i18n);
        }
        Some(&"/model") => {
            model::handle_model_command(&parts, config, api_client, &i18n).await?;
        }
        Some(&"/history") => {
            history::handle_history_command(&parts, config, session, &i18n)?;
        }
        Some(&"/language") | Some(&"/lang") => {
            language::handle_language_command(&parts, config, &i18n)?;
        }
        Some(&"/agents.md") => {
            handle_agents_md_command(session, &i18n).await?;
        }
        Some(&"/runcommand") => {
            runcommand::handle_run_command_command(&parts, &i18n)?;
        }
        _ => {
            println!(
                "\n\x1b[31m[X] {}: {}\x1b[0m\n",
                i18n.get("unknown_command"),
                command
            );
        }
    }

    Ok(())
}
