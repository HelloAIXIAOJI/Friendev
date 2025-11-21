use super::bing;
use super::client::create_client;
use super::duckduckgo;
use super::types::SearchResult;
use crate::ui::get_i18n;
use anyhow::Result;

/// Graceful fallback search: try DuckDuckGo first, fall back to Bing
pub async fn search_auto(keywords: &str, max_results: usize) -> Result<Vec<SearchResult>> {
    let client = create_client();

    // Try DuckDuckGo first
    match duckduckgo::search_duckduckgo(&client, keywords, max_results).await {
        Ok(results) => return Ok(results),
        Err(e) => {
            let i18n = get_i18n();
            eprintln!(
                "\n{}: {}\n{}",
                i18n.get("search_ddg_error_prefix"),
                e,
                i18n.get("search_try_bing")
            );
        }
    }

    // Fall back to Bing if DuckDuckGo fails
    bing::search_bing(&client, keywords, max_results).await
}
