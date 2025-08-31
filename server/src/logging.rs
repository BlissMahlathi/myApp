//! Logging setup and configuration for structured observability.

use tracing::{info, Level};

/// Initialize structured logging for the server.
/// TODO: Add environment-based level configuration and structured output.
pub fn init_logging() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    info!("Logging initialized");
}
