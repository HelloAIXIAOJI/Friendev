use anyhow::Result;
use std::fs;
use std::path::Path;

use super::file_common::normalize_path;
use crate::tools::args::FileDiffEditArgs;
use crate::tools::indexer::Indexer;
use crate::tools::types::ToolResult;

pub async fn execute_file_diff_edit(
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    let args: FileDiffEditArgs = serde_json::from_str(arguments)?;

    let target_path = normalize_path(&args.path, working_dir);

    // 验证文件存在
    if !target_path.exists() {
        return Ok(ToolResult::error(format!(
            "File does not exist: {}",
            target_path.display()
        )));
    }

    if !target_path.is_file() {
        return Ok(ToolResult::error(format!(
            "Not a file: {}",
            target_path.display()
        )));
    }

    // 读取文件
    let content = fs::read_to_string(&target_path)?;
    let original_content = content.clone();
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // 检测换行符风格
    let uses_crlf = content.contains("\r\n");

    // 应用所有 hunk（从后到前，避免行号偏移）
    let mut hunks = args.hunks.clone();
    hunks.sort_by(|a, b| b.start_line.cmp(&a.start_line));

    // 记录所有修改的行范围，用于后续的验证输出
    let mut modified_ranges = Vec::new();

    for hunk in hunks.iter() {
        if hunk.start_line == 0 {
            return Ok(ToolResult::error("Line numbers must start from 1".to_string()));
        }

        let start_idx = hunk.start_line - 1;
        let end_idx = std::cmp::min(start_idx + hunk.num_lines, lines.len());

        if start_idx > lines.len() {
            return Ok(ToolResult::error(format!(
                "Line number out of range: {}, total lines in file: {}",
                hunk.start_line,
                lines.len()
            )));
        }

        // 记录修改范围（用于后续提取上下文）
        modified_ranges.push((start_idx, start_idx + hunk.new_content.lines().count()));

        // 构建新的行列表
        let mut new_lines = Vec::new();
        new_lines.extend_from_slice(&lines[..start_idx]);
        new_lines.extend(hunk.new_content.lines().map(|s| s.to_string()));
        new_lines.extend_from_slice(&lines[end_idx..]);

        lines = new_lines;
    }

    if require_approval {
        let details = generate_detailed_changes(&original_content, &args);
        if !super::file_common::check_file_action_approval(
            "file_diff_edit",
            &target_path,
            Some(&details),
        )? {
            let i18n = ui::get_i18n();
            return Ok(ToolResult::error(i18n.get("approval_rejected")));
        }
    }

    // 重建文件内容
    let new_content = lines.join("\n");
    let final_content = if uses_crlf {
        new_content.replace("\n", "\r\n")
    } else {
        new_content
    };

    fs::write(&target_path, &final_content)?;

    // Auto-hook: Update outline index
    if let Ok(indexer) = Indexer::new(working_dir) {
        let _ = indexer.index_file(&target_path, working_dir, false, false).await;
    }

    // 核心：直接从内容生成上下文，不再重新读取文件
    let actual_lines: Vec<&str> = final_content.lines().collect();

    // 生成 diff_merge_result：合并所有修改范围的上下文
    let diff_merge_result = generate_diff_result(&actual_lines, &modified_ranges);

    let brief = format!("Applied {} hunks", args.hunks.len());
    let output = format!(
        "File updated: {}\nApplied {} diff hunks\n\n{}",
        target_path.display(),
        args.hunks.len(),
        diff_merge_result
    );

    let verification_prompt = "Please verify the DIFF merge result above. Check if all modifications are correct and there are no syntax errors (e.g., unclosed brackets, misaligned indentation). If everything looks good, you may continue. If there are any issues, describe the problem clearly.";

    Ok(ToolResult {
        success: true,
        brief,
        message: output,
        verification_required: true,
        verification_message: Some(verification_prompt.to_string()),
    })
}

fn generate_preview(args: &FileDiffEditArgs) -> String {
    let preview = args
        .hunks
        .iter()
        .take(3)
        .map(|h| {
            format!(
                "- Line {}: {} lines → {} chars",
                h.start_line,
                h.num_lines,
                h.new_content.chars().count()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    if args.hunks.len() > 3 {
        format!("{}\n... and {} more hunks", preview, args.hunks.len() - 3)
    } else {
        preview
    }
}

fn generate_detailed_changes(file_content: &str, args: &FileDiffEditArgs) -> String {
    let mut detailed_changes = String::new();
    let lines: Vec<&str> = file_content.lines().collect();

    for (i, hunk) in args.hunks.iter().enumerate() {
        detailed_changes.push_str(&format!("@@ Hunk #{} @@\n", i + 1));
        
        let start_line = hunk.start_line;
        let num_lines = hunk.num_lines;
        let original_start_idx = if start_line > 0 { start_line - 1 } else { 0 };
        
        // Context before (3 lines)
        let context_start = if original_start_idx >= 3 { original_start_idx - 3 } else { 0 };
        for idx in context_start..original_start_idx {
            if idx < lines.len() {
                detailed_changes.push_str(&format!(" {}\n", lines[idx]));
            }
        }

        // Removed lines
        for idx in 0..num_lines {
            let curr_idx = original_start_idx + idx;
            if curr_idx < lines.len() {
                detailed_changes.push_str(&format!("-{}\n", lines[curr_idx]));
            }
        }

        // Added lines
        for line in hunk.new_content.lines() {
            detailed_changes.push_str(&format!("+{}\n", line));
        }
        
        // Context after (3 lines)
        let end_idx = original_start_idx + num_lines;
        for idx in end_idx..(end_idx + 3) {
            if idx < lines.len() {
                detailed_changes.push_str(&format!(" {}\n", lines[idx]));
            }
        }
        
        detailed_changes.push_str("\n");
    }

    detailed_changes
}

fn generate_diff_result(actual_lines: &[&str], modified_ranges: &[(usize, usize)]) -> String {
    let mut diff_merge_result = String::new();
    diff_merge_result.push_str("==== DIFF MERGE RESULT (from actual file) ====\n\n");

    // 合并所有修改范围，避免重复
    let mut all_context_ranges = Vec::new();
    for (mod_start, mod_end) in modified_ranges.iter() {
        let context_start = if *mod_start >= 3 { *mod_start - 3 } else { 0 };
        let context_end = std::cmp::min(*mod_end + 3, actual_lines.len());
        all_context_ranges.push((context_start, context_end));
    }

    // 合并重叠的范围
    all_context_ranges.sort();
    let mut merged_ranges = Vec::new();
    for (start, end) in all_context_ranges {
        if let Some((_last_start, last_end)) = merged_ranges.last_mut() {
            if start <= *last_end {
                *last_end = std::cmp::max(*last_end, end);
            } else {
                merged_ranges.push((start, end));
            }
        } else {
            merged_ranges.push((start, end));
        }
    }

    // 生成合并后的上下文输出
    for (range_start, range_end) in merged_ranges {
        for line_idx in range_start..range_end {
            if line_idx < actual_lines.len() {
                diff_merge_result.push_str(&format!(
                    "Line {:4}: {}\n",
                    line_idx + 1,
                    actual_lines[line_idx]
                ));
            }
        }
        diff_merge_result.push('\n');
    }

    diff_merge_result
}
