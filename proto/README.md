# Proto Crate

This crate provides shared protocol types for the rehearsal coordination system.

## Purpose

The `proto` crate defines the core data structures used across the system for:

- **Authentication**: JWT exchange request/response types for session token workflows
- **WebSocket Frames**: Typed message envelopes for routing different kinds of real-time events
- **Session State**: Participant management and snapshot synchronization types

## Key Types

- `AuthExchangeRequest` / `AuthExchangeResponse`: JWT token exchange DTOs
- `FrameKind` / `FrameEnvelope`: WebSocket message routing and payload wrapper
- `Participant`: Session member representation
- `Snapshot`: Complete session state for client synchronization

All types implement `Serialize` and `Deserialize` for JSON wire format compatibility.

## Usage

This crate is intended to be shared between the server and any future client implementations to ensure consistent protocol definitions.