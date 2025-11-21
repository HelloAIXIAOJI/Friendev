use serde::Serialize;

/// Search result containing title, URL, and snippet
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}
