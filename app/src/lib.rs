use anyhow::{anyhow, Result};
use rpc::{AppService, RemoteAppService};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::time::{sleep, Duration};

pub async fn initialize_app() -> Result<Box<dyn AppService>> {
    let args: Vec<String> = std::env::args().collect();
    let verbose = args.iter().any(|arg| arg == "--verbose");

    let addr: SocketAddr = std::env::var("FRIENDEV_SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:7878".to_string())
        .parse()?;

    launch_server(&addr)?;

    let mut last_err = None;
    for attempt in 0..10 {
        match RemoteAppService::connect(addr, verbose).await {
            Ok(remote) => {
                println!("\x1b[32m[OK]\x1b[0m Connected to server at {}", addr);
                return Ok(Box::new(remote));
            }
            Err(e) => {
                last_err = Some(e);
                sleep(Duration::from_millis(200 * (attempt + 1) as u64)).await;
            }
        }
    }

    Err(anyhow!(
        "Failed to connect to friendev-server at {}: {}",
        addr,
        last_err
            .map(|e| e.to_string())
            .unwrap_or_else(|| "unknown error".to_string())
    ))
}

fn launch_server(addr: &SocketAddr) -> Result<()> {
    let binary = resolve_server_binary()?;
    println!("\x1b[36m[*]\x1b[0m launching {}", binary.display());
    Command::new(&binary)
        .env("FRIENDEV_SERVER_ADDR", addr.to_string())
        .env_remove("FRIENDEV_SERVER_VERBOSE")
        .spawn()?;
    Ok(())
}

fn resolve_server_binary() -> Result<PathBuf> {
    let mut candidates = vec![
        PathBuf::from(format!("friendev-server{}", std::env::consts::EXE_SUFFIX)),
        PathBuf::from(format!("./friendev-server{}", std::env::consts::EXE_SUFFIX)),
    ];

    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(dir) = current_exe.parent() {
            candidates.push(dir.join(format!("friendev-server{}", std::env::consts::EXE_SUFFIX)));
        }
    }

    for candidate in candidates {
        if candidate.exists() && is_executable(&candidate) {
            return Ok(candidate);
        }
    }

    Err(anyhow!(
        "friendev-server binary not found in current directory"
    ))
}

fn is_executable(path: &Path) -> bool {
    path.is_file()
}

pub async fn run_repl(service: Box<dyn AppService>) -> Result<()> {
    app_client::run_repl(service).await
}
