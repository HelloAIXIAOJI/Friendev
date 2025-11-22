use crate::protocol::{Request, Response, StreamEvent};
use crate::AppService;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct RemoteAppService {
    addr: SocketAddr,
    verbose: bool,
}

impl RemoteAppService {
    pub fn new(addr: SocketAddr, verbose: bool) -> Self {
        Self { addr, verbose }
    }

    async fn send_request(&self, request: Request) -> Result<Response> {
        let mut stream = TcpStream::connect(self.addr).await?;
        let mut payload = serde_json::to_vec(&request)?;
        payload.push(b'\n');
        stream.write_all(&payload).await?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        if line.is_empty() {
            return Err(anyhow!("empty response from server"));
        }
        Ok(serde_json::from_str(&line)?)
    }

    pub async fn connect(addr: SocketAddr, verbose: bool) -> Result<Self> {
        let client = Self::new(addr, verbose);
        // quick ping to ensure availability
        let _ = client
            .send_request(Request::GetMessage {
                key: "ping".to_string(),
            })
            .await
            .map_err(|e| anyhow!("failed to connect to server: {}", e))?;
        Ok(client)
    }
}

#[async_trait]
impl AppService for RemoteAppService {
    async fn handle_user_input(&mut self, line: &str) -> Result<()> {
        match self
            .send_request(Request::HandleUserInput {
                line: line.to_string(),
                verbose: self.verbose,
            })
            .await?
        {
            Response::Ok => Ok(()),
            Response::Error { message } => Err(anyhow!(message)),
            Response::Message { .. } => Err(anyhow!("unexpected response")),
            Response::StreamEvent { .. } => Err(anyhow!("unexpected stream response")),
        }
    }

    async fn get_message(&self, key: &str) -> Result<String> {
        match self
            .send_request(Request::GetMessage {
                key: key.to_string(),
            })
            .await?
        {
            Response::Message { value } => Ok(value),
            Response::Ok => Ok(String::new()),
            Response::Error { message } => Err(anyhow!(message)),
            Response::StreamEvent { .. } => Err(anyhow!("unexpected stream response")),
        }
    }

    async fn next_event(&mut self) -> Option<StreamEvent> {
        match self
            .send_request(Request::StreamSubscribe {
                verbose: self.verbose,
            })
            .await
        {
            Ok(Response::StreamEvent { event }) => Some(event),
            Ok(Response::Ok) => None,
            Ok(Response::Message { .. }) => None,
            Ok(Response::Error { message }) => {
                eprintln!("[client] stream error: {}", message);
                None
            }
            Err(e) => {
                eprintln!("[client] stream request failed: {}", e);
                None
            }
        }
    }
}
