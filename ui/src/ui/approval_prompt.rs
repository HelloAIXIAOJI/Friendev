use colored::Colorize;
use std::io::{self, Write};

use crate::ui::get_i18n;

/// 用户审批提示
/// 返回 (approved, always, view_details)
pub fn prompt_approval(
    action: &str,
    file_path: &str,
    content_preview: Option<&str>,
) -> io::Result<(bool, bool, bool)> {
    use std::path::Path;

    // 提取文件名
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

    // 显示内容预览
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
    println!("{}", i18n.get("approval_separator").yellow());
    print!("  {} ", i18n.get("approval_choice_prompt").bright_cyan());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice = input.trim().to_lowercase();
    match choice.as_str() {
        "y" | "yes" => Ok((true, false, false)),
        "i" | "info" => {
            Ok((true, false, true)) // 返回 true, false, true 表示需要查看详细信息
        }
        "a" | "always" => {
            println!("  {} {}", "✓".green(), i18n.get("approval_always_approved"));
            Ok((true, true, false)) // 返回 true, true, false 表示 always
        }
        _ => {
            println!("  {} {}", "✗".red(), i18n.get("approval_rejected"));
            Ok((false, false, false))
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
