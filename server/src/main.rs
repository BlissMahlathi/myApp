//! Rehearsal coordination server main entry point.

use server::{config::Config, logging};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init_logging();

    let config = Config::default();

    info!("🎵 Rehearsal Coordination Server Starting");
    info!("Server configuration: {:?}", config);
    info!("Server ready on port {}", config.port);

    // TODO: Add HTTP server setup with axum
    // TODO: Add WebSocket endpoint handling
    // TODO: Add graceful shutdown handling

    Ok(())
}
