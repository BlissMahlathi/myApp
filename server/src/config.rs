//! Configuration management for server settings.

/// Server configuration placeholder.
/// TODO: Implement environment variable loading and validation.
#[derive(Debug)]
pub struct Config {
    /// Server bind port.
    pub port: u16,
    /// Log level setting.
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            log_level: "info".to_string(),
        }
    }
}
