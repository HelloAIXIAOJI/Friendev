use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

use super::super::utils::normalize_whitespace;
use super::file_common::normalize_path;
use crate::tools::args::FileReplaceArgs;
use crate::tools::indexer::Indexer;
use crate::tools::types::ToolResult;

pub async fn execute_file_replace(
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    let args: FileReplaceArgs = serde_json::from_str(arguments)?;

    let target_path = normalize_path(&args.path, working_dir);

    // 验证文件存在
    if !target_path.exists() {
        return Ok(ToolResult::error(format!(
            "文件不存在: {}",
            target_path.display()
        )));
    }

    if !target_path.is_file() {
        return Ok(ToolResult::error(format!(
            "不是文件: {}",
            target_path.display()
        )));
    }

    // 读取文件并处理
    let mut content = fs::read_to_string(&target_path)?;
    let original_content = content.clone();

    // 检测换行符风格
    let uses_crlf = content.contains("\r\n");

    // 关键：规范化换行符为 Unix \n
    content = content.replace("\r\n", "\n");
    let processing_content = content.clone();

    // 应用所有编辑
    let (replacements_made, failed_edits) = apply_edits(&mut content, &args);

    // 检查是否有修改
    if content == processing_content {
        let error_msg = generate_error_diagnostics(&failed_edits, &processing_content);
        return Ok(ToolResult::error(error_msg));
    }

    if require_approval {
        let details = generate_detailed_changes(&original_content, &args);
        if !super::file_common::check_file_action_approval(
            "file_replace",
            &target_path,
            Some(&details),
        )? {
            let i18n = ui::get_i18n();
            return Ok(ToolResult::error(i18n.get("approval_rejected")));
        }
    }

    // 写回文件
    let final_content = if uses_crlf {
        content.replace("\n", "\r\n")
    } else {
        content
    };
    fs::write(&target_path, &final_content)?;

    // Auto-hook: Update outline index
    if let Ok(indexer) = Indexer::new(working_dir) {
        let _ = indexer.index_file(&target_path, working_dir, false, false).await;
    }

    let i18n = ui::get_i18n();
    let brief_tmpl = i18n.get("file_replace_success");
    // Note: replace keys: {} for edits count, {} for replacements count (positional replacement depends on string order)
    // The key is: "Applied {} edits, total {} replacements in {1}" which is a bit mixed.
    // Let's assume standard positional replacement or .replace chaining.
    // Brief template: "Applied {} edits, total {} replacements in {1}"
    // We need to be careful about argument order.
    // Let's use direct .replace for placeholders if they are unique, or positional if supported.
    // Rust's String.replace matches all occurrences.
    // For "Applied {} edits...", first {} is edits, second {} is replacements.
    // But .replace("{}", ...) will replace ALL {} with first arg.
    // We need replacen.
    
    let brief = brief_tmpl
        .replacen("{}", &args.edits.len().to_string(), 1)
        .replacen("{}", &replacements_made.to_string(), 1)
        .replace("{1}", &target_path.display().to_string());

    let output = brief.clone(); // Use same message for detailed output for now, or create a separate key if needed.

    Ok(ToolResult::ok(brief, output))
}

fn generate_preview(args: &FileReplaceArgs) -> String {
    let preview = args
        .edits
        .iter()
        .take(3)
        .map(|e| {
            let old_preview = if e.old.chars().count() > 40 {
                let truncated: String = e.old.chars().take(40).collect();
                format!("{}...", truncated)
            } else {
                e.old.clone()
            };
            let new_preview = if e.new.chars().count() > 40 {
                let truncated: String = e.new.chars().take(40).collect();
                format!("{}...", truncated)
            } else {
                e.new.clone()
            };
            format!("- Replace: {}\n  With: {}", old_preview, new_preview)
        })
        .collect::<Vec<_>>()
        .join("\n");

    if args.edits.len() > 3 {
        format!("{}\n... and {} more edits", preview, args.edits.len() - 3)
    } else {
        preview
    }
}

fn generate_detailed_changes(_file_content: &str, args: &FileReplaceArgs) -> String {
    let mut detailed_changes = String::new();

    for (i, edit) in args.edits.iter().enumerate() {
        detailed_changes.push_str(&format!("@@ Edit #{} @@\n", i + 1));

        if edit.replace_all {
            detailed_changes.push_str("Type: Replace All\n");
        } else {
            detailed_changes.push_str("Type: Replace First\n");
        }

        detailed_changes.push_str("--- Search Pattern\n");
        for line in edit.old.lines() {
            detailed_changes.push_str(&format!("-{}\n", line));
        }

        detailed_changes.push_str("+++ Replacement\n");
        for line in edit.new.lines() {
            detailed_changes.push_str(&format!("+{}\n", line));
        }
        detailed_changes.push_str("\n");
    }

    detailed_changes
}

fn apply_edits(content: &mut String, args: &FileReplaceArgs) -> (usize, Vec<(usize, String)>) {
    let mut replacements_made = 0;
    let mut failed_edits = Vec::new();

    for (edit_idx, edit) in args.edits.iter().enumerate() {
        if edit.regex {
            apply_regex_edit(
                content,
                edit,
                edit_idx,
                &mut replacements_made,
                &mut failed_edits,
            );
        } else {
            apply_string_edit(
                content,
                edit,
                edit_idx,
                &mut replacements_made,
                &mut failed_edits,
            );
        }
    }

    (replacements_made, failed_edits)
}

fn apply_regex_edit(
    content: &mut String,
    edit: &crate::tools::args::Edit,
    edit_idx: usize,
    replacements_made: &mut usize,
    failed_edits: &mut Vec<(usize, String)>,
) {
    match Regex::new(&edit.old) {
        Ok(re) => {
            let count = re.find_iter(content).count();
            if count > 0 {
                *content = if edit.replace_all {
                    *replacements_made += count;
                    re.replace_all(content, &edit.new).into_owned()
                } else {
                    *replacements_made += 1;
                    re.replace(content, &edit.new).into_owned()
                };
            } else {
                failed_edits.push((edit_idx, edit.old.clone()));
            }
        }
        Err(_) => {
            failed_edits.push((edit_idx, edit.old.clone()));
        }
    }
}

fn apply_string_edit(
    content: &mut String,
    edit: &crate::tools::args::Edit,
    edit_idx: usize,
    replacements_made: &mut usize,
    failed_edits: &mut Vec<(usize, String)>,
) {
    let search_pattern = if edit.normalize {
        normalize_whitespace(&edit.old)
    } else {
        edit.old.clone()
    };

    *content = if edit.replace_all {
        let count = if edit.normalize {
            let normalized_content = normalize_whitespace(content);
            normalized_content.matches(&search_pattern).count()
        } else {
            content.matches(&search_pattern).count()
        };
        *replacements_made += count;

        if edit.normalize {
            let normalized_content = normalize_whitespace(content);
            let normalized_result = normalized_content.replace(&search_pattern, &edit.new);
            normalized_result.replace("\n", "\r\n")
        } else {
            content.replace(&search_pattern, &edit.new)
        }
    } else {
        let found = if edit.normalize {
            normalize_whitespace(content).contains(&search_pattern)
        } else {
            content.contains(&search_pattern)
        };

        if found {
            *replacements_made += 1;
            if edit.normalize {
                let normalized_content = normalize_whitespace(content);
                let normalized_result = normalized_content.replacen(&search_pattern, &edit.new, 1);
                normalized_result.replace("\n", "\r\n")
            } else {
                content.replacen(&search_pattern, &edit.new, 1)
            }
        } else {
            failed_edits.push((edit_idx, edit.old.clone()));
            content.clone()
        }
    };
}

fn generate_error_diagnostics(failed_edits: &[(usize, String)], content: &str) -> String {
    let i18n = ui::get_i18n();
    let mut error_msg = i18n.get("replace_diag_not_found");
    error_msg.push('\n');

    for (idx, search_str) in failed_edits.iter() {
        error_msg.push_str(&format!("\n{}\n", i18n.get("replace_diag_edit_num").replace("{}", &(idx + 1).to_string())));
        error_msg.push_str(&format!(
            "  {}\n",
            i18n.get("replace_diag_len").replace("{}", &search_str.chars().count().to_string())
        ));
        error_msg.push_str(&format!(
            "  {}\n",
            i18n.get("replace_diag_preview").replace("{}", &if search_str.chars().count() > 100 {
                search_str.chars().take(100).collect::<String>()
            } else {
                search_str.clone()
            })
        ));
        error_msg.push_str(&format!("  {}\n", i18n.get("replace_diag_has_newline").replace("{}", &search_str.contains('\n').to_string())));
        error_msg.push_str(&format!("  {}\n", i18n.get("replace_diag_has_crlf").replace("{}", &search_str.contains("\r\n").to_string())));

        // 尝试找相似的内容作为建议
        let mut suggestions = Vec::new();
        for line in content.lines() {
            if line.contains(search_str.trim()) {
                suggestions.push(line);
            }
        }

        if !suggestions.is_empty() && suggestions.len() <= 3 {
            error_msg.push_str(&format!("  {}\n", i18n.get("replace_diag_similar")));
            for sugg in suggestions.iter().take(3) {
                error_msg.push_str(&format!("    {}\n", sugg));
            }
        }
    }

    error_msg.push_str(&format!("\n{}\n", i18n.get("replace_diag_hints")));

    error_msg
}

