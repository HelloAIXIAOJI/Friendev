mod command_handler;
mod message_builder;
mod service;
mod startup;

pub use service::LocalAppService;
pub use startup::AppState;

use anyhow::Result;
use std::net::SocketAddr;

pub async fn initialize_app() -> Result<LocalAppService> {
    let state = startup::initialize_state().await?;
    Ok(LocalAppService::new(state))
}

pub async fn run_server(addr: SocketAddr) -> Result<()> {
    let service = initialize_app().await?;
    let verbose = std::env::var("FRIENDEV_SERVER_VERBOSE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    rpc::serve(service, addr, verbose).await
}
