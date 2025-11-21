use reqwest::Client;
use std::time::Duration;

/// Create a configured HTTP client for search requests
pub fn create_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()
        .unwrap_or_else(|_| Client::new())
}
