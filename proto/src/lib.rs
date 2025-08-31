//! Protocol types for shared communication structures.
//!
//! This crate provides the core data types used across the system for:
//! - Authentication exchange (JWT tokens)
//! - WebSocket frame envelopes and message types
//! - Session snapshots and participant management
//!
//! All types derive Serialize/Deserialize for JSON wire format.

pub mod auth;
pub mod frame;
pub mod participant;
pub mod snapshot;

pub use auth::*;
pub use frame::*;
pub use participant::*;
pub use snapshot::*;
