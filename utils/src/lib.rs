//! Common utilities and helper functions shared across the platform.
//!
//! This crate provides reusable utilities for:
//! - Application versioning and metadata
//! - Time and timestamp helpers (future)
//! - Configuration loading utilities (future)

/// Returns the application version string.
pub fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the application name.
pub fn app_name() -> &'static str {
    "Rehearsal Coordination Platform"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_version() {
        let version = app_version();
        assert!(!version.is_empty());
        assert!(version.contains('.'));
    }

    #[test]
    fn test_app_name() {
        let name = app_name();
        assert_eq!(name, "Rehearsal Coordination Platform");
    }
}
