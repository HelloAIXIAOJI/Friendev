use super::defaults;
use super::persistence;
use super::types::Config;
use crate::i18n::{I18n, SUPPORTED_LANGUAGES};
use anyhow::Result;

/// Initialize configuration through interactive prompts
pub fn initialize_config() -> Result<Config> {
    // Use default language (en) for setup prompts
    let i18n = I18n::new("en");

    println!("\n{}", i18n.get("setup_welcome"));
    println!();

    // Step 1: UI Language (first)
    let ui_language_idx = dialoguer::Select::new()
        .with_prompt(i18n.get("setup_ui_language"))
        .default(0)
        .items(&SUPPORTED_LANGUAGES)
        .interact()?;
    let ui_language = SUPPORTED_LANGUAGES[ui_language_idx].to_string();

    // Update i18n with selected language for remaining prompts
    let i18n = I18n::new(&ui_language);

    // Step 2: API Key
    let api_key = dialoguer::Input::<String>::new()
        .with_prompt(i18n.get("setup_api_key"))
        .interact_text()?;

    // Step 3: API URL
    let api_url = dialoguer::Input::<String>::new()
        .with_prompt(i18n.get("setup_api_url"))
        .default("https://api.openai.com/v1".to_string())
        .interact_text()?;

    // Step 4: Default Model
    let current_model = dialoguer::Input::<String>::new()
        .with_prompt(i18n.get("setup_model"))
        .default("gpt-4".to_string())
        .interact_text()?;

    // Step 5: AI Language (last)
    let ai_language = dialoguer::Input::<String>::new()
        .with_prompt(i18n.get("setup_ai_language"))
        .default(SUPPORTED_LANGUAGES[0].to_string())
        .interact_text()?;

    let config = Config {
        api_key,
        api_url,
        current_model,
        ui_language,
        ai_language,
        max_retries: defaults::default_max_retries(),
        retry_delay_ms: defaults::default_retry_delay_ms(),
    };

    persistence::save_config(&config)?;
    println!("\n✓ {}\n", i18n.get("setup_saved"));
    Ok(config)
}
