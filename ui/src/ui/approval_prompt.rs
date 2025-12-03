use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::{self};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use super::get_i18n;
use i18n::I18n;

type ReviewHandler = dyn Fn(&ReviewRequest) -> io::Result<bool> + Send + Sync + 'static;

static REVIEW_HANDLER: OnceLock<Box<ReviewHandler>> = OnceLock::new();
static SMART_APPROVAL_MODE: AtomicBool = AtomicBool::new(false);
static JURY_MODE: AtomicBool = AtomicBool::new(false);

/// Review request
pub struct ReviewRequest<'a> {
    pub action: &'a str,
    pub subject: &'a str,
    pub preview: Option<&'a str>,
    pub is_jury: bool,
}

/// Register review handler
pub fn set_review_handler<F>(handler: F)
where
    F: Fn(&ReviewRequest) -> io::Result<bool> + Send + Sync + 'static,
{
    let _ = REVIEW_HANDLER.set(Box::new(handler));
}

/// Set smart approval mode
pub fn set_smart_approval_mode(enabled: bool) {
    SMART_APPROVAL_MODE.store(enabled, Ordering::Relaxed);
}

/// Set jury mode
pub fn set_jury_mode(enabled: bool) {
    JURY_MODE.store(enabled, Ordering::Relaxed);
}

/// User approval prompt
/// Returns (approved, always, view_details)
pub fn prompt_approval(
    action: &str,
    file_path: &str,
    content_preview: Option<&str>,
) -> io::Result<(bool, bool, bool)> {
    use std::path::Path;

    let is_smart_mode = SMART_APPROVAL_MODE.load(Ordering::Relaxed);
    let is_jury_mode = JURY_MODE.load(Ordering::Relaxed);

    // Check for Smart Approval or Jury Mode
    if is_smart_mode || is_jury_mode {
        if let Some(handler) = REVIEW_HANDLER.get() {
            let request = ReviewRequest {
                action,
                subject: file_path,
                preview: content_preview,
                is_jury: is_jury_mode,
            };
            
            // In smart/jury mode, we delegate to the handler immediately
            if is_jury_mode {
                println!("\n{}", get_i18n().get("approval_jury_wait").yellow());
            } else {
                println!("\n{}", get_i18n().get("approval_review_wait").yellow());
            }

            match handler(&request) {
                Ok(true) => return Ok((true, false, false)),
                Ok(false) => {
                    println!("{}", get_i18n().get("approval_rejected").red());
                    return Ok((false, false, false));
                }
                Err(e) => {
                    println!("{} {}", "Review Error:".red(), e);
                    println!("Falling back to manual approval...");
                }
            }
        }
    }

    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file_path);

    let i18n = get_i18n();

    // Header
    println!();
    println!("{}", i18n.get("approval_title").yellow().bold());
    println!(
        "    {} {}",
        action.cyan(),
        i18n.get("approval_action_wants")
    );
    println!("      {}", file_name.yellow().bold());

    // Preview (Brief)
    if let Some(preview) = content_preview {
        println!();
        println!("    {}", i18n.get("approval_content_preview").blue());
        let lines: Vec<&str> = preview.lines().take(5).collect();
        for line in lines {
             let truncated = if line.chars().count() > 60 {
                 let shortened: String = line.chars().take(60).collect();
                 format!("{}...", shortened)
             } else {
                 line.to_string()
             };
            println!("      {}", truncated.bright_black());
        }
        if preview.lines().count() > 5 {
            println!("      ... ({} lines total)", preview.lines().count().to_string().bright_black());
        }
    }
    println!();

    let choices = vec![
        i18n.get("approval_opt_approve"),
        i18n.get("approval_opt_always"),
        i18n.get("approval_opt_details"),
        i18n.get("approval_opt_reject"),
    ];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(i18n.get("approval_choice_prompt"))
            .items(&choices)
            .default(0)
            .interact()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        match selection {
            0 => return Ok((true, false, false)), // Approve
            1 => {
                println!("  {} {}", "✓".green(), i18n.get("approval_always_approved"));
                return Ok((true, true, false)); // Always
            }
            2 => {
                // Check if we have a review handler
                if REVIEW_HANDLER.get().is_some() {
                     let request = ReviewRequest {
                        action,
                        subject: file_path,
                        preview: content_preview,
                        is_jury: false, // Manual request is always single review
                    };
                    
                    // Try to run review
                    if let Some(handler) = REVIEW_HANDLER.get() {
                        println!("{}", i18n.get("approval_review_wait").yellow());
                        if let Err(e) = handler(&request) {
                             println!("{} {}", "Error:".red(), e);
                             // If review fails, fall back to showing raw content?
                             // Or just let user decide again.
                        }
                    }

                    // After review (or attempt), ask for final decision
                    let approved = prompt_review_decision(&i18n)?;
                    return Ok((approved, false, false));
                } else {
                     // No review handler, treat as "Show Raw Details" request
                     return Ok((true, false, true));
                }
            }
            _ => {
                println!("  {} {}", "✗".red(), i18n.get("approval_rejected"));
                return Ok((false, false, false)); // Reject
            }
        }
    }
}

fn prompt_review_decision(i18n: &I18n) -> io::Result<bool> {
    println!();
    let choices = vec![
        i18n.get("approval_review_decision_yes"),
        i18n.get("approval_review_decision_no"),
    ];
    
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(i18n.get("approval_review_decision_prompt"))
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
    Ok(selection == 0)
}

pub fn show_detailed_content(action: &str, file_path: &str, content: &str) -> io::Result<bool> {
     use std::path::Path;
     let file_name = Path::new(file_path).file_name().and_then(|n| n.to_str()).unwrap_or(file_path);
     let i18n = get_i18n();
     
     println!();
     println!("{}", i18n.get("details_title").cyan());
     println!("    {} {}", i18n.get("details_tool"), action);
     println!("    {} {}", i18n.get("details_file"), file_name.cyan().bold());
     println!("{}", i18n.get("details_separator").cyan());
     
     let lines: Vec<&str> = content.lines().collect();
     // Simple pagination or limit could be added here, but for now, full output
     for (i, line) in lines.iter().enumerate() {
         if line.starts_with('+') && !line.starts_with("+++") {
             println!("{:3}: {}", i + 1, line.green());
         } else if line.starts_with('-') && !line.starts_with("---") {
             println!("{:3}: {}", i + 1, line.red());
         } else if line.starts_with("@@") {
             println!("{:3}: {}", i + 1, line.cyan());
         } else {
             println!("{:3}: {}", i + 1, line);
         }
     }
     println!("{}", i18n.get("details_separator").cyan());
     
     let hint_text = i18n.get("details_choice_hint");
     let choices = vec![
         hint_text.split(" / ").next().unwrap_or("Continue"),
         "Abort"
     ];
     
     let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(i18n.get("details_choice_prompt"))
        .items(&choices)
        .default(0)
        .interact()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        
     Ok(selection == 0)
}
