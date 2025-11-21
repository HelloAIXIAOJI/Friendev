use anyhow::Result;

use super::super::utils::limit_results;
use super::search_common::{create_search_error, create_search_result};
use crate::tools::args::SearchArgs;
use crate::tools::types::ToolResult;

pub async fn execute_search_auto(arguments: &str) -> Result<ToolResult> {
    let args: SearchArgs = serde_json::from_str(arguments)?;
    let max_results = limit_results(args.max_results);

    match crate::search_tool::search_auto(&args.keywords, max_results).await {
        Ok(results) => Ok(create_search_result(&args.keywords, &results, None)),
        Err(e) => Ok(create_search_error(&e.to_string(), None)),
    }
}
