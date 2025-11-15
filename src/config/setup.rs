use anyhow::Result;
use super::types::Config;
use super::persistence;
use super::defaults;

/// Initialize configuration through interactive prompts
pub fn initialize_config() -> Result<Config> {
    println!("Welcome to Friendev! First-time use requires initialization configuration.\n");
    
    let api_key = dialoguer::Input::<String>::new()
        .with_prompt("Please enter OpenAI API Key")
        .interact_text()?;

    let api_url = dialoguer::Input::<String>::new()
        .with_prompt("Please enter OpenAI Base URL")
        .default("https://api.openai.com/v1".to_string())
        .interact_text()?;

    let current_model = dialoguer::Input::<String>::new()
        .with_prompt("Please enter the default model.")
        .default("gpt-4".to_string())
        .interact_text()?;

    let config = Config {
        api_key,
        api_url,
        current_model,
        ui_language: defaults::default_ui_language(),
        ai_language: defaults::default_ai_language(),
        max_retries: defaults::default_max_retries(),
        retry_delay_ms: defaults::default_retry_delay_ms(),
    };

    persistence::save_config(&config)?;
    println!("\n✓ 配置已保存！\n");
    Ok(config)
}
