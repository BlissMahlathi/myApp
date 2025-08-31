//! Participant management types for session membership.

use serde::{Deserialize, Serialize};

/// Represents a participant in a rehearsal session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    /// Unique participant identifier.
    pub id: String,
    /// Human-readable display name.
    pub display_name: String,
}
