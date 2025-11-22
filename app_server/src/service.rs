use super::{command_handler, AppState};
use anyhow::Result;
use async_trait::async_trait;
use rpc::protocol::StreamEvent;
use rpc::AppService;
use std::collections::VecDeque;

pub struct LocalAppService {
    state: AppState,
    events: VecDeque<StreamEvent>,
}

impl LocalAppService {
    pub fn new(state: AppState) -> Self {
        Self {
            state,
            events: VecDeque::new(),
        }
    }

    pub fn push_event(&mut self, event: StreamEvent) {
        self.events.push_back(event);
    }
}

#[async_trait]
impl AppService for LocalAppService {
    async fn handle_user_input(&mut self, line: &str) -> Result<()> {
        match command_handler::handle_user_input(line, &mut self.state, &mut self.events).await {
            Ok(()) => Ok(()),
            Err(e) => {
                self.push_event(StreamEvent::OutputLine(format!("[ERROR] {}", e)));
                Err(e)
            }
        }
    }

    async fn get_message(&self, key: &str) -> Result<String> {
        Ok(self.state.i18n.get(key))
    }

    async fn next_event(&mut self) -> Option<StreamEvent> {
        self.events.pop_front()
    }
}
