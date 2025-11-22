use crate::protocol::{Request, Response};
use crate::AppService;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

pub async fn serve<S>(service: S, addr: SocketAddr, verbose: bool) -> Result<()>
where
    S: AppService + Send + 'static,
{
    let listener = TcpListener::bind(addr).await?;
    let shared = Arc::new(Mutex::new(service));

    loop {
        let (stream, peer) = listener.accept().await?;
        let service = shared.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, service, verbose).await {
                if verbose {
                    eprintln!("[RPC] Connection with {} failed: {}", peer, e);
                }
            }
        });
    }
}

async fn handle_connection<S>(
    stream: TcpStream,
    service: Arc<Mutex<S>>,
    verbose: bool,
) -> Result<()>
where
    S: AppService + Send + 'static,
{
    let (reader, mut writer) = stream.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let request: Request = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                if verbose {
                    eprintln!("[RPC] Failed to parse request: {}", e);
                }
                continue;
            }
        };
        let response = match request {
            Request::HandleUserInput {
                line,
                verbose: req_verbose,
            } => {
                let service_clone = service.clone();
                let log_errors = verbose || req_verbose;
                tokio::spawn(async move {
                    let mut guard = service_clone.lock().await;
                    if let Err(e) = guard.handle_user_input(&line).await {
                        if log_errors {
                            eprintln!("[RPC] handle_user_input error: {}", e);
                        }
                    }
                });
                Response::Ok
            }
            Request::GetMessage { key } => {
                let guard = service.lock().await;
                match guard.get_message(&key).await {
                    Ok(value) => Response::Message { value },
                    Err(e) => Response::Error {
                        message: e.to_string(),
                    },
                }
            }
            Request::StreamSubscribe {
                verbose: _req_verbose,
            } => {
                let mut guard = service.lock().await;
                match guard.next_event().await {
                    Some(event) => Response::StreamEvent { event },
                    None => Response::Ok,
                }
            }
        };

        let mut payload = serde_json::to_vec(&response)?;
        payload.push(b'\n');
        writer.write_all(&payload).await?;
    }

    Ok(())
}
