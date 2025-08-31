//! Session snapshot types for state synchronization.

use crate::Participant;
use serde::{Deserialize, Serialize};

/// Complete session state snapshot for client synchronization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Current session participants.
    pub participants: Vec<Participant>,
}
