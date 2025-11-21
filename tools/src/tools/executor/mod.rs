use anyhow::Result;
use std::path::Path;

use crate::tools::types::ToolResult;
use crate::ui::get_i18n;

mod command_operations;
pub mod file_operations;
pub mod search_operations;
mod utils;

pub async fn execute_tool(
    name: &str,
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    match name {
        "file_list" => file_operations::execute_file_list(arguments, working_dir).await,
        "file_read" => file_operations::execute_file_read(arguments, working_dir).await,
        "file_write" => {
            file_operations::execute_file_write(arguments, working_dir, require_approval).await
        }
        "file_replace" => {
            file_operations::execute_file_replace(arguments, working_dir, require_approval).await
        }
        "file_diff_edit" => {
            file_operations::execute_file_diff_edit(arguments, working_dir, require_approval).await
        }
        "network_search_auto" => search_operations::execute_search_auto(arguments).await,
        "network_search_duckduckgo" => {
            search_operations::execute_search_duckduckgo(arguments).await
        }
        "network_search_bing" => search_operations::execute_search_bing(arguments).await,
        "run_command" => command_operations::execute_run_command(arguments, require_approval).await,
        _ => {
            let i18n = get_i18n();
            let tmpl = i18n.get("tool_unknown");
            Ok(ToolResult::error(tmpl.replace("{}", name)))
        }
    }
}
