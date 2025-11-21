use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

use crate::tools::types::ToolResult;
use crate::tools::types::{approve_action_for_session, is_action_approved};
use crate::ui::get_i18n;
use crate::ui::prompt_approval;

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
pub fn verify_file_exists(path: &Path) -> Result<ToolResult> {
    let i18n = get_i18n();
    if !path.exists() {
        let tmpl = i18n.get("file_not_exist");
        return Ok(ToolResult::error(format!(
            "{}",
            tmpl.replace("{}", &path.display().to_string())
        )));
    }
    if !path.is_file() {
        let tmpl = i18n.get("file_not_file");
        return Ok(ToolResult::error(format!(
            "{}",
            tmpl.replace("{}", &path.display().to_string())
        )));
    }
    Ok(ToolResult::ok(String::new(), String::new()))
}

/// 验证目录存在
pub fn verify_dir_exists(path: &Path) -> Result<ToolResult> {
    let i18n = get_i18n();
    if !path.exists() {
        let tmpl = i18n.get("file_path_not_exist");
        return Ok(ToolResult::error(format!(
            "{}",
            tmpl.replace("{}", &path.display().to_string())
        )));
    }
    if !path.is_dir() {
        let tmpl = i18n.get("file_not_directory");
        return Ok(ToolResult::error(format!(
            "{}",
            tmpl.replace("{}", &path.display().to_string())
        )));
    }
    Ok(ToolResult::ok(String::new(), String::new()))
}

/// 显示审批对话框并返回用户决策
pub async fn request_approval(
    tool_name: &str,
    description: &str,
    preview: Option<&str>,
) -> Result<(bool, bool, bool)> {
    Ok(prompt_approval(tool_name, description, preview)?)
}

/// 处理审批流程
pub async fn handle_approval_flow(
    tool_id: &str,
    approval_desc: &str,
    preview: Option<&str>,
    require_approval: bool,
) -> Result<Option<String>> {
    if !require_approval {
        return Ok(None);
    }

    if is_action_approved(tool_id) {
        return Ok(None);
    }

    let (approved, always, _view_details) =
        request_approval(tool_id, approval_desc, preview).await?;

    if !approved {
        let i18n = get_i18n();
        return Ok(Some(i18n.get("approval_user_rejected")));
    }

    if always {
        approve_action_for_session(tool_id);
    }

    Ok(None)
}

/// 处理带详细内容展示的审批流程
pub async fn handle_approval_with_details(
    tool_id: &str,
    approval_desc: &str,
    preview: Option<&str>,
    detail_title: &str,
    detail_content: &str,
    require_approval: bool,
) -> Result<Option<String>> {
    if !require_approval {
        return Ok(None);
    }

    if is_action_approved(tool_id) {
        return Ok(None);
    }

    let (approved, always, view_details) =
        request_approval(tool_id, approval_desc, preview).await?;

    // 如果用户选择查看详细信息
    if view_details {
        let continue_operation =
            crate::ui::show_detailed_content(tool_id, detail_title, detail_content)?;

        if !continue_operation {
            let i18n = get_i18n();
            return Ok(Some(i18n.get("approval_user_cancelled")));
        }
    }

    if !approved {
        let i18n = get_i18n();
        return Ok(Some(i18n.get("approval_user_rejected")));
    }

    if always {
        approve_action_for_session(tool_id);
    }

    Ok(None)
}
