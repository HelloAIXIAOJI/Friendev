use colored::Colorize;
use std::io::{self, Write};
use std::sync::OnceLock;

use super::get_i18n;
use i18n::I18n;

type ReviewHandler = dyn Fn(&ReviewRequest) -> io::Result<()> + Send + Sync + 'static;

static REVIEW_HANDLER: OnceLock<Box<ReviewHandler>> = OnceLock::new();

pub struct ReviewRequest<'a> {
    pub action: &'a str,
    pub subject: &'a str,
    pub preview: Option<&'a str>,
}

pub fn set_review_handler<F>(handler: F)
where
    F: Fn(&ReviewRequest) -> io::Result<()> + Send + Sync + 'static,
{
    let _ = REVIEW_HANDLER.set(Box::new(handler));
}

/// 用户审批提示
/// 返回 (approved, always, view_details)
pub fn prompt_approval(
    action: &str,
    file_path: &str,
    content_preview: Option<&str>,
) -> io::Result<(bool, bool, bool)> {
    use std::path::Path;

    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file_path);

    let i18n = get_i18n();

    println!();
    println!("{}", i18n.get("approval_title").yellow());
    println!(
        "{}",
        format!("    {} {}:", action, i18n.get("approval_action_wants")).yellow()
    );
    println!("{}", format!("      {}", file_name).yellow().bold());

    if let Some(preview) = content_preview {
        println!("{}", i18n.get("approval_empty_line").yellow());
        println!(
            "{}",
            format!("    {}", i18n.get("approval_content_preview")).yellow()
        );

        let lines: Vec<&str> = preview.lines().take(5).collect();
        for line in lines {
            let truncated = if line.chars().count() > 35 {
                let shortened: String = line.chars().take(35).collect();
                format!("{}...", shortened)
            } else {
                line.to_string()
            };
            println!("{}", format!("      {}", truncated).bright_black());
        }

        let total_lines = preview.lines().count();
        if total_lines > 5 {
            println!(
                "{}",
                format!("      ... ({} more lines)", total_lines - 5).bright_black()
            );
        }
    }

    println!("{}", i18n.get("approval_empty_line").yellow());
    println!(
        "{}",
        format!("    {}", i18n.get("approval_choice_hint")).yellow()
    );

    let separator = i18n.get("approval_separator");
    let prompt_label = i18n.get("approval_choice_prompt");

    loop {
        println!("{}", separator.as_str().yellow());
        print!("  {} ", prompt_label.as_str().bright_cyan());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let choice = input.trim().to_lowercase();
        match choice.as_str() {
            "y" | "yes" => return Ok((true, false, false)),
            "i" | "info" => {
                return Ok((true, false, true));
            }
            "a" | "always" => {
                println!("  {} {}", "✓".green(), i18n.get("approval_always_approved"));
                return Ok((true, true, false));
            }
            "r" | "review" => {
                let request = ReviewRequest {
                    action,
                    subject: file_path,
                    preview: content_preview,
                };

                if let Some(handler) = REVIEW_HANDLER.get() {
                    if let Err(err) = handler(&request) {
                        println!(
                            "  {} {}",
                            "!".yellow(),
                            i18n.get("approval_review_error")
                                .replace("{}", &err.to_string())
                        );
                        continue;
                    }

                    let approved = prompt_review_decision(&i18n)?;
                    return Ok((approved, false, false));
                } else {
                    println!(
                        "  {} {}",
                        "!".yellow(),
                        i18n.get("approval_review_unavailable")
                    );
                }
            }
            _ => {
                println!("  {} {}", "✗".red(), i18n.get("approval_rejected"));
                return Ok((false, false, false));
            }
        }
    }
}

fn prompt_review_decision(i18n: &I18n) -> io::Result<bool> {
    println!();
    println!("  {}", i18n.get("approval_review_followup"));

    loop {
        let prompt = i18n.get("approval_review_decision_prompt");
        print!("  {} ", prompt.bright_cyan());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!(
                "  {} {}",
                "!".yellow(),
                i18n.get("approval_review_invalid_choice")
            ),
        }
    }
}

/// 显示详细内容
pub fn show_detailed_content(action: &str, file_path: &str, content: &str) -> io::Result<bool> {
    use std::path::Path;

    // 提取文件名
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file_path);

    let i18n = get_i18n();

    println!();
    println!("{}", i18n.get("details_title").cyan());
    println!(
        "{}",
        format!("    {} {}", i18n.get("details_tool"), action).cyan()
    );
    println!(
        "{}",
        format!("    {} {}", i18n.get("details_file"), file_name)
            .cyan()
            .bold()
    );
    println!("{}", i18n.get("details_separator").cyan());
    println!();

    // 显示完整内容，使用终端友好的格式
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let line_num = format!("{:3}:", i + 1).bright_black();
        println!("  {} {}", line_num, line);
    }

    println!();
    println!("{}", i18n.get("details_separator").cyan());
    println!(
        "{}",
        format!("    {}", i18n.get("details_choice_hint")).cyan()
    );
    println!("{}", "  ──────────────────────────────────────────".cyan());
    print!("  {} ", i18n.get("details_choice_prompt").bright_cyan());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice = input.trim().to_lowercase();
    match choice.as_str() {
        "c" | "continue" => Ok(true),
        _ => Ok(false),
    }
}
