use anyhow::Result;
use std::path::Path;

use crate::tools::types::ToolResult;

mod file_common;
mod file_diff_edit;
mod file_list;
mod file_read;
mod file_replace;
mod file_write;

pub async fn execute_file_list(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    file_list::execute_file_list(arguments, working_dir).await
}

pub async fn execute_file_read(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    file_read::execute_file_read(arguments, working_dir).await
}

pub async fn execute_file_write(
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    file_write::execute_file_write(arguments, working_dir, require_approval).await
}

pub async fn execute_file_replace(
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    file_replace::execute_file_replace(arguments, working_dir, require_approval).await
}

pub async fn execute_file_diff_edit(
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    file_diff_edit::execute_file_diff_edit(arguments, working_dir, require_approval).await
}
