use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    HandleUserInput { line: String, verbose: bool },
    GetMessage { key: String },
    StreamSubscribe { verbose: bool },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Ok,
    Error { message: String },
    Message { value: String },
    StreamEvent { event: StreamEvent },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamEvent {
    OutputLine(String),
    ToolStatus { id: String, status: String },
}
