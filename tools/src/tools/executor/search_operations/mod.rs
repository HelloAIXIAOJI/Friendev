use anyhow::Result;

use crate::tools::types::ToolResult;

mod search_auto;
mod search_bing;
mod search_common;
mod search_duckduckgo;

pub async fn execute_search_auto(arguments: &str) -> Result<ToolResult> {
    search_auto::execute_search_auto(arguments).await
}

pub async fn execute_search_duckduckgo(arguments: &str) -> Result<ToolResult> {
    search_duckduckgo::execute_search_duckduckgo(arguments).await
}

pub async fn execute_search_bing(arguments: &str) -> Result<ToolResult> {
    search_bing::execute_search_bing(arguments).await
}
