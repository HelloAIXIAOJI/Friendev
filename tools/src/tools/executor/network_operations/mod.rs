use anyhow::Result;

use crate::tools::types::ToolResult;

mod fetch_content;

pub async fn execute_fetch_content(arguments: &str) -> Result<ToolResult> {
    fetch_content::execute_fetch_content(arguments).await
}
