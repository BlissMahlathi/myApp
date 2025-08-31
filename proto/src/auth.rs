//! Authentication exchange types for JWT token workflows.

use serde::{Deserialize, Serialize};

/// Request to exchange authorization code for session token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthExchangeRequest {
    /// Authorization code (pre-shared or QR-derived).
    pub code: String,
}

/// Response containing session token and expiration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthExchangeResponse {
    /// JWT session token for WebSocket authentication.
    pub token: String,
    /// Token expiration timestamp (ISO 8601 format).
    pub expires_at: String,
}
