use anyhow::Result;
use async_trait::async_trait;

/// High-level interface exposed by the application server.
#[async_trait]
pub trait AppService: Send {
    /// Process a user-provided line (command or message).
    async fn handle_user_input(&mut self, line: &str) -> Result<()>;

    /// Retrieve a localized message for the given key.
    async fn get_message(&self, key: &str) -> Result<String>;

    /// Poll next streaming event if available.
    async fn next_event(&mut self) -> Option<crate::protocol::StreamEvent>;
}
