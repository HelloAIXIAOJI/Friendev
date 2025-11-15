use anyhow::Result;
use super::types::SearchResult;
use super::duckduckgo;
use super::bing;
use super::client::create_client;

/// Graceful fallback search: try DuckDuckGo first, fall back to Bing
pub async fn search_auto(keywords: &str, max_results: usize) -> Result<Vec<SearchResult>> {
    let client = create_client();
    
    // Try DuckDuckGo first
    match duckduckgo::search_duckduckgo(&client, keywords, max_results).await {
        Ok(results) => return Ok(results),
        Err(e) => {
            eprintln!("\nDuckDuckGo ERROR: {} \n Try Bing...", e);
        }
    }
    
    // Fall back to Bing if DuckDuckGo fails
    bing::search_bing(&client, keywords, max_results).await
}
