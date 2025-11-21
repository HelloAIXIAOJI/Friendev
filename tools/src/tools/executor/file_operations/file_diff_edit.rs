use anyhow::Result;
use std::fs;
use std::path::Path;

use super::file_common::{handle_approval_with_details, normalize_path};
use crate::tools::args::FileDiffEditArgs;
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
        "file_diff_edit",
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

    // 读取文件
    let content = fs::read_to_string(&target_path)?;
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
            return Ok(ToolResult::error("行号必须从1开始".to_string()));
        }

        let start_idx = hunk.start_line - 1;
        let end_idx = std::cmp::min(start_idx + hunk.num_lines, lines.len());

        if start_idx > lines.len() {
            return Ok(ToolResult::error(format!(
                "行号超出范围: {}，文件总行数: {}",
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

    // 重建文件内容
    let new_content = lines.join("\n");
    let final_content = if uses_crlf {
        new_content.replace("\n", "\r\n")
    } else {
        new_content
    };

    fs::write(&target_path, &final_content)?;

    // 核心：直接从文件读取实际内容，并提取 ±3 行上下文
    let actual_content = fs::read_to_string(&target_path)?;
    let actual_lines: Vec<&str> = actual_content.lines().collect();

    // 生成 diff_merge_result：合并所有修改范围的上下文
    let diff_merge_result = generate_diff_result(&actual_lines, &modified_ranges);

    let brief = format!("应用了 {} 个 hunk", args.hunks.len());
    let output = format!(
        "文件已更新: {}\n应用了 {} 个 diff hunk\n\n{}",
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
    detailed_changes.push_str("\n=== 当前文件内容 ===\n");
    detailed_changes.push_str(file_content);
    detailed_changes.push_str("\n\n=== 计划进行的更改 ===\n");

    for (i, hunk) in args.hunks.iter().enumerate() {
        detailed_changes.push_str(&format!("\nHunk #{}\n", i + 1));
        detailed_changes.push_str(&format!("  行号: {}\n", hunk.start_line));
        detailed_changes.push_str(&format!("  原行数: {}\n", hunk.num_lines));
        detailed_changes.push_str("  新内容:\n");
        for line in hunk.new_content.lines() {
            detailed_changes.push_str(&format!("    {}\n", line));
        }
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
        diff_merge_result.push_str("\n");
    }

    diff_merge_result
}
