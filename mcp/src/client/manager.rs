use std::collections::HashMap;
use std::sync::Arc;
use rmcp::{service::RunningService, RoleClient};

#[derive(Clone)]
pub struct ClientManager {
    pub(crate) clients: HashMap<String, Arc<RunningService<RoleClient, ()>>>,
}

impl ClientManager {
    pub fn new() -> Self {
        Self { clients: HashMap::new() }
    }

    pub fn list_servers(&self) -> Vec<String> {
        let mut names: Vec<String> = self.clients.keys().cloned().collect();
        names.sort();
        names
    }
}
