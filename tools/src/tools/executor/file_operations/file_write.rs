use anyhow::Result;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::file_common::normalize_path;
use crate::tools::args::FileWriteArgs;
use crate::tools::indexer::Indexer;
use crate::types::ToolResult;
use ui::get_i18n;

pub async fn execute_file_write(
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    let args: FileWriteArgs = serde_json::from_str(arguments)?;

    let target_path = normalize_path(&args.path, working_dir);
    let i18n = get_i18n();

    // 验证 mode 参数
    let mode = args.mode.as_str();
    if mode != "overwrite" && mode != "append" {
        let tmpl = i18n.get("file_write_invalid_mode");
        return Ok(ToolResult::error(tmpl.replace("{}", mode)));
    }

    if require_approval {
        let details = generate_detailed_changes(&target_path, &args);
        if !super::file_common::check_file_action_approval(
            "file_write",
            &target_path,
            Some(&details),
        )? {
            let i18n = get_i18n();
            return Ok(ToolResult::error(i18n.get("approval_rejected")));
        }
    }

    // 创建父目录（如果不存在）
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 根据模式写入或追加
    let result = if mode == "append" {
        execute_append_mode(&target_path, &args.content)
    } else {
        execute_overwrite_mode(&target_path, &args.content)
    };

    // Auto-hook: Update outline index if write was successful
    if result.is_ok() {
        if let Ok(indexer) = Indexer::new(working_dir) {
            let _ = indexer.index_file(&target_path, working_dir, false, false).await;
        }
    }

    result
}

fn execute_append_mode(target_path: &Path, content: &str) -> Result<ToolResult> {
    let i18n = ui::get_i18n();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(target_path)?;
    file.write_all(content.as_bytes())?;

    let file_size = target_path.metadata()?.len();

    let brief_tmpl = i18n.get("file_write_append_brief");
    let brief = brief_tmpl.replace("{}", &content.len().to_string());

    let output_tmpl = i18n.get("file_write_append_output");
    let output = output_tmpl
        .replacen("{}", &target_path.display().to_string(), 1)
        .replacen("{}", &content.len().to_string(), 1)
        .replacen("{}", &file_size.to_string(), 1);
    Ok(ToolResult::ok(brief, output))
}

fn execute_overwrite_mode(target_path: &Path, content: &str) -> Result<ToolResult> {
    fs::write(target_path, content)?;

    let i18n = ui::get_i18n();

    let brief_tmpl = i18n.get("file_write_overwrite_brief");
    let brief = brief_tmpl.replace("{}", &content.len().to_string());

    let output_tmpl = i18n.get("file_write_overwrite_output");
    let output = output_tmpl
        .replacen("{}", &target_path.display().to_string(), 1)
        .replacen("{}", &content.len().to_string(), 1);
    Ok(ToolResult::ok(brief, output))
}

fn generate_detailed_changes(target_path: &Path, args: &FileWriteArgs) -> String {
    let mut detailed_changes = String::new();

    if args.mode == "append" {
        detailed_changes.push_str("@@ Append to file @@\n");
        // If file exists, we might want to show the last few lines of the existing file for context?
        // For now, just show what's being added as +
        if target_path.exists() {
             if let Ok(content) = fs::read_to_string(target_path) {
                 let lines: Vec<&str> = content.lines().collect();
                 if !lines.is_empty() {
                      let context_lines = if lines.len() > 3 { &lines[lines.len()-3..] } else { &lines[..] };
                      for line in context_lines {
                          detailed_changes.push_str(&format!(" {}\n", line));
                      }
                 }
             }
        }

        for line in args.content.lines() {
            detailed_changes.push_str(&format!("+{}\n", line));
        }
    } else {
        // Overwrite mode
        detailed_changes.push_str("@@ Overwrite file @@\n");
        
        if target_path.exists() {
             if let Ok(content) = fs::read_to_string(target_path) {
                 for line in content.lines() {
                     detailed_changes.push_str(&format!("-{}\n", line));
                 }
             }
        } else {
             detailed_changes.push_str("--- /dev/null\n");
        }
        
        detailed_changes.push_str(&format!("+++ {}\n", target_path.display()));
        for line in args.content.lines() {
            detailed_changes.push_str(&format!("+{}\n", line));
        }
    }

    detailed_changes
}
