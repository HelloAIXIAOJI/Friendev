use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

use crate::types::ToolResult;
use ui::get_i18n;

/// 规范化路径 - 处理相对路径和绝对路径
pub fn normalize_path(path_str: &str, working_dir: &Path) -> PathBuf {
    let p = Path::new(path_str);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        working_dir.join(p)
    }
}

/// 验证文件存在
#[allow(dead_code)]
pub fn verify_file_exists(path: &Path) -> Result<ToolResult> {
    let i18n = get_i18n();
    if !path.exists() {
        let tmpl = i18n.get("file_not_exist");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &path.display().to_string()).to_string(),
        ));
    }
    if !path.is_file() {
        let tmpl = i18n.get("file_not_file");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &path.display().to_string()).to_string(),
        ));
    }
    Ok(ToolResult::ok(String::new(), String::new()))
}

/// 验证目录存在
#[allow(dead_code)]
pub fn verify_dir_exists(path: &Path) -> Result<ToolResult> {
    let i18n = get_i18n();
    if !path.exists() {
        let tmpl = i18n.get("file_path_not_exist");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &path.display().to_string()).to_string(),
        ));
    }
    if !path.is_dir() {
        let tmpl = i18n.get("file_not_directory");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &path.display().to_string()).to_string(),
        ));
    }
    Ok(ToolResult::ok(String::new(), String::new()))
}

/// Check if file action is approved
pub fn check_file_action_approval(
    action: &str,
    path: &Path,
    preview: Option<&str>,
) -> Result<bool> {
    use crate::types::{approve_action_for_session, is_action_approved};
    use ui::prompt_approval;

    if is_action_approved(action) {
        return Ok(true);
    }

    let (approved, always, view_details) = prompt_approval(
        action,
        &path.display().to_string(),
        preview,
    )?;

    if view_details {
        let continue_op = ui::show_detailed_content(
            action,
            &path.display().to_string(),
            preview.unwrap_or("(No preview available)"),
        )?;
        if !continue_op {
            return Ok(false);
        }
        return Ok(true);
    }

    if !approved {
        return Ok(false);
    }

    if always {
        approve_action_for_session(action);
    }

    Ok(true)
}
