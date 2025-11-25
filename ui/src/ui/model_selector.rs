use dialoguer::{theme::ColorfulTheme, Select};

use super::get_i18n;

/// Interactive model selector
/// Returns the selected model name, or None if user cancelled
pub fn select_model(
    models: &[String],
    current_model: &str,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let i18n = get_i18n();
    
    // Find the index of the current model
    let default_index = models
        .iter()
        .position(|m| m == current_model)
        .unwrap_or(0);
    
    // Create selection items with visual indicators
    let items: Vec<String> = models
        .iter()
        .map(|model| {
            if model == current_model {
                format!("✓ {} (current)", model)
            } else {
                format!("  {}", model)
            }
        })
        .collect();
    
    // Build the selector with custom theme
    let theme = ColorfulTheme {
        active_item_style: console::Style::new().cyan().bold(),
        active_item_prefix: console::style("▶".to_string()).cyan(),
        inactive_item_prefix: console::style(" ".to_string()),
        checked_item_prefix: console::style("✓".to_string()).green(),
        unchecked_item_prefix: console::style(" ".to_string()),
        prompt_prefix: console::style("?".to_string()).yellow().bold(),
        prompt_style: console::Style::new().bold(),
        ..ColorfulTheme::default()
    };
    
    let selection = Select::with_theme(&theme)
        .with_prompt(&i18n.get("model_selector_prompt"))
        .items(&items)
        .default(default_index)
        .interact_opt()?;
    
    Ok(selection.map(|idx| models[idx].clone()))
}

/// Show a simple model list (non-interactive fallback)
pub fn print_model_list(models: &[String], current_model: &str) {
    let i18n = get_i18n();
    
    println!("\n\x1b[1;33m{}:\x1b[0m", i18n.get("available_models"));
    for (i, model) in models.iter().enumerate() {
        if model == current_model {
            println!("  \x1b[32m[*]\x1b[0m \x1b[1m{}.\x1b[0m {}", i + 1, model);
        } else {
            println!("  \x1b[90m[ ]\x1b[0m {}. {}", i + 1, model);
        }
    }
    println!();
}
