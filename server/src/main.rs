use anyhow::Result;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let addr =
        std::env::var("FRIENDEV_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".to_string());
    let addr: SocketAddr = addr.parse()?;

    println!("Starting Friendev server on {}", addr);
    app_server::run_server(addr).await
}
