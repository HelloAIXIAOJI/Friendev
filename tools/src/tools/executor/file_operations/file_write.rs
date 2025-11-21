use anyhow::Result;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use super::file_common::{handle_approval_with_details, normalize_path};
use crate::tools::args::FileWriteArgs;
use crate::tools::types::ToolResult;
use crate::ui::get_i18n;

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

    // 处理审批流程
    let action_desc = if mode == "append" {
        let tmpl = i18n.get("file_write_append_action");
        tmpl.replace("{}", &target_path.display().to_string())
    } else {
        let tmpl = i18n.get("file_write_overwrite_action");
        tmpl.replace("{}", &target_path.display().to_string())
    };

    if let Some(err) = handle_approval_with_details(
        "file_write",
        &action_desc,
        Some(&args.content),
        &target_path.display().to_string(),
        &args.content,
        require_approval,
    )
    .await?
    {
        return Ok(ToolResult::error(err));
    }

    // 创建父目录（如果不存在）
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 根据模式写入或追加
    if mode == "append" {
        execute_append_mode(&target_path, &args.content)
    } else {
        execute_overwrite_mode(&target_path, &args.content)
    }
}

fn execute_append_mode(target_path: &Path, content: &str) -> Result<ToolResult> {
    let i18n = crate::ui::get_i18n();
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

    let i18n = crate::ui::get_i18n();

    let brief_tmpl = i18n.get("file_write_overwrite_brief");
    let brief = brief_tmpl.replace("{}", &content.len().to_string());

    let output_tmpl = i18n.get("file_write_overwrite_output");
    let output = output_tmpl
        .replacen("{}", &target_path.display().to_string(), 1)
        .replacen("{}", &content.len().to_string(), 1);
    Ok(ToolResult::ok(brief, output))
}
