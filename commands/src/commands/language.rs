use anyhow::Result;

use crate::config::Config;
use crate::i18n::{is_language_supported, supported_languages_str, I18n};

/// Handle /language command
pub fn handle_language_command(parts: &[&str], config: &mut Config, i18n: &I18n) -> Result<()> {
    match parts.get(1) {
        Some(&"ui") => {
            if let Some(lang) = parts.get(2) {
                if !is_language_supported(lang) {
                    println!(
                        "\n\x1b[31m[✗] {}:\x1b[0m {}\n",
                        i18n.get("error"),
                        i18n.get("lang_ui_unsupported").replace("{}", lang)
                    );
                    println!(
                        "\x1b[33m[!] {}:\x1b[0m {}\n",
                        i18n.get("lang_supported_label"),
                        supported_languages_str()
                    );
                } else {
                    config.update_ui_language(lang.to_string())?;
                    let new_i18n = I18n::new(lang);
                    println!(
                        "\n\x1b[32m[OK]\x1b[0m {} {}\n",
                        new_i18n.get("ui_language_set"),
                        lang
                    );
                }
            } else {
                println!(
                    "\n\x1b[36m[>]\x1b[0m {}: {}\n",
                    i18n.get("current_ui_lang"),
                    config.ui_language
                );
                println!(
                    "\x1b[33m[!] {}:\x1b[0m /language ui <lang>",
                    i18n.get("usage")
                );
                println!(
                    "    {} {}\n",
                    i18n.get("lang_supported_label"),
                    supported_languages_str()
                );
            }
        }
        Some(&"ai") => {
            if let Some(lang) = parts.get(2) {
                config.update_ai_language(lang.to_string())?;
                println!(
                    "\n\x1b[32m[OK]\x1b[0m {} {}\n",
                    i18n.get("ai_language_set"),
                    lang
                );
            } else {
                println!(
                    "\n\x1b[36m[>]\x1b[0m {}: {}\n",
                    i18n.get("current_ai_lang"),
                    config.ai_language
                );
                println!(
                    "\x1b[33m[!] {}:\x1b[0m /language ai <lang>",
                    i18n.get("usage")
                );
                println!("    {}\n", i18n.get("supported_languages"));
            }
        }
        _ => {
            println!("\n\x1b[33m[?] {}:\x1b[0m", i18n.get("help_language"));
            println!(
                "    \x1b[36m/language\x1b[0m ui <lang>  {}",
                i18n.get("cmd_language_ui")
            );
            println!(
                "    \x1b[36m/language\x1b[0m ai <lang>  {}",
                i18n.get("cmd_language_ai")
            );
            println!(
                "\n    {} {}\n",
                i18n.get("lang_supported_ui_label"),
                supported_languages_str()
            );
        }
    }
    Ok(())
}
