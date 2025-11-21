mod auto;
mod bing;
mod client;
mod duckduckgo;
mod html_parser;
mod types;

// Re-export public API
pub use auto::search_auto;
pub use types::SearchResult;

/// Search client for backward compatibility
pub struct SearchClient {
    client: reqwest::Client,
}

impl SearchClient {
    /// Create a new search client
    pub fn new() -> Self {
        Self {
            client: client::create_client(),
        }
    }

    /// Search using DuckDuckGo
    pub async fn search_duckduckgo(
        &self,
        keywords: &str,
        max_results: usize,
    ) -> anyhow::Result<Vec<SearchResult>> {
        duckduckgo::search_duckduckgo(&self.client, keywords, max_results).await
    }

    /// Search using Bing
    pub async fn search_bing(
        &self,
        keywords: &str,
        max_results: usize,
    ) -> anyhow::Result<Vec<SearchResult>> {
        bing::search_bing(&self.client, keywords, max_results).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_auto() {
        let results = search_auto("Rust programming", 3).await;
        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(!results.is_empty());
    }
}
