//! WebSocket frame types and envelopes for typed message routing.

use serde::{Deserialize, Serialize};

/// Frame type enumeration for WebSocket message routing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FrameKind {
    /// Heartbeat ping message.
    Ping,
    /// Heartbeat pong response.
    Pong,
    /// Authentication successful confirmation.
    AuthOk,
    /// Session state snapshot delivery.
    Snapshot,
    /// Error message with details.
    Error,
}

/// Envelope wrapper for all WebSocket messages with typed routing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameEnvelope {
    /// Message type for routing and deserialization.
    pub kind: FrameKind,
    /// JSON payload specific to the frame kind.
    pub payload: serde_json::Value,
}
