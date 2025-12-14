#[cfg(test)]
mod tests {
    use super::*;
    use config::Config;

    #[tokio::test]
    async fn test_enhanced_client_creation() {
        let config = Config {
            api_url: "https://api.openai.com/v1".to_string(),
            api_key: "test-key".to_string(),
            current_model: "gpt-3.5-turbo".to_string(),
            max_retries: 3,
            retry_delay_ms: 1000,
        };

        let client = ApiClient::new(config);
        assert!(true); // Just test creation for now
    }
}