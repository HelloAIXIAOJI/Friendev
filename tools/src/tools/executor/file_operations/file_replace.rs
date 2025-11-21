use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

use super::super::utils::normalize_whitespace;
use super::file_common::{handle_approval_with_details, normalize_path};
use crate::tools::args::FileReplaceArgs;
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

    // 生成预览内容
    let preview = generate_preview(&args);

    // 处理审批流程
    let file_content = fs::read_to_string(&target_path)?;
    let detailed_changes = generate_detailed_changes(&file_content, &args);

    if let Some(err) = handle_approval_with_details(
        "file_replace",
        &target_path.display().to_string(),
        Some(&preview),
        &target_path.display().to_string(),
        &detailed_changes,
        require_approval,
    )
    .await?
    {
        return Ok(ToolResult::error(err));
    }

    // 读取文件并处理
    let mut content = fs::read_to_string(&target_path)?;

    // 检测换行符风格
    let uses_crlf = content.contains("\r\n");

    // 关键：规范化换行符为 Unix \n
    content = content.replace("\r\n", "\n");
    let original_content = content.clone();

    // 应用所有编辑
    let (replacements_made, failed_edits) = apply_edits(&mut content, &args);

    // 检查是否有修改
    if content == original_content {
        let error_msg = generate_error_diagnostics(&failed_edits, &content);
        return Ok(ToolResult::error(error_msg));
    }

    // 写回文件
    let final_content = if uses_crlf {
        content.replace("\n", "\r\n")
    } else {
        content
    };
    fs::write(&target_path, &final_content)?;

    let brief = format!(
        "应用了 {} 个编辑，{} 个替换",
        args.edits.len(),
        replacements_made
    );
    let output = format!(
        "文件已更新: {}\n应用了 {} 个编辑\n共进行了 {} 个替换",
        target_path.display(),
        args.edits.len(),
        replacements_made
    );

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

fn generate_detailed_changes(file_content: &str, args: &FileReplaceArgs) -> String {
    let mut detailed_changes = String::new();
    detailed_changes.push_str("\n=== 当前文件内容 ===\n");
    detailed_changes.push_str(file_content);
    detailed_changes.push_str("\n\n=== 计划进行的更改 ===\n");

    for (i, edit) in args.edits.iter().enumerate() {
        detailed_changes.push_str(&format!("\n编辑 #{}:\n", i + 1));

        if edit.replace_all {
            detailed_changes.push_str(&format!("  替换所有出现的: '{}'", edit.old));
        } else {
            detailed_changes.push_str(&format!("  替换第一次出现的: '{}'", edit.old));
        }

        if edit.new.contains('\n') || edit.new.chars().count() > 50 {
            detailed_changes.push_str("\n  替换为 (多行):\n");
            for line in edit.new.lines() {
                detailed_changes.push_str(&format!("    {}\n", line));
            }
        } else {
            detailed_changes.push_str(&format!("\n  替换为: '{}'", edit.new));
        }
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
    let mut error_msg = String::from("未找到要替换的字符串。诊断信息：\n");

    for (idx, search_str) in failed_edits.iter() {
        error_msg.push_str(&format!("\n编辑 #{}:\n", idx + 1));
        error_msg.push_str(&format!(
            "  搜索字符串长度: {} 字符\n",
            search_str.chars().count()
        ));
        error_msg.push_str(&format!(
            "  搜索字符串 (前100字符): {}\n",
            if search_str.chars().count() > 100 {
                search_str.chars().take(100).collect::<String>()
            } else {
                search_str.clone()
            }
        ));
        error_msg.push_str(&format!("  包含换行符: {}\n", search_str.contains('\n')));
        error_msg.push_str(&format!("  包含 \\r\\n: {}\n", search_str.contains("\r\n")));

        // 尝试找相似的内容作为建议
        let mut suggestions = Vec::new();
        for line in content.lines() {
            if line.contains(&search_str.trim()) {
                suggestions.push(line);
            }
        }

        if !suggestions.is_empty() && suggestions.len() <= 3 {
            error_msg.push_str("  文件中发现相似内容（可能是空格/换行符差异）:\n");
            for sugg in suggestions.iter().take(3) {
                error_msg.push_str(&format!("    {}\n", sugg));
            }
        }
    }

    error_msg.push_str("\n提示：检查以下可能的问题:\n");
    error_msg.push_str("  1. 行结束符差异 (Windows \\r\\n vs Unix \\n)\n");
    error_msg.push_str("  2. 前后有额外空格\n");
    error_msg.push_str("  3. 缩进使用了不同的制表符或空格\n");
    error_msg.push_str("  4. 特殊字符编码差异\n");

    error_msg
}
