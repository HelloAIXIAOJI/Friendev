use anyhow::Result;

use crate::history::ChatSession;
use crate::i18n::I18n;
use agents::generate_agents_analysis_prompt;

/// Handle /agents.md command - returns prompt (auto-sent to AI by main.rs)
pub async fn handle_agents_md_command(session: &ChatSession, i18n: &I18n) -> Result<String> {
    println!(
        "\n\x1b[33m[*] {}\x1b[0m",
        i18n.get("agents_analyzing_project")
    );

    // Generate analysis prompt
    let analysis_prompt = generate_agents_analysis_prompt(&session.working_directory)?;

    println!("\x1b[32m[OK]\x1b[0m {}\n", i18n.get("agents_sending_to_ai"));

    Ok(analysis_prompt)
}
