# myApp - Rehearsal Coordination Platform

A real-time rehearsal coordination platform enabling ultra-low-latency team communication, authoritative tempo synchronization, and structured musical context for local network rehearsals.

## Workspace Layout

This repository uses a Cargo workspace structure with the following crates:

### Core Crates

- **`proto/`** - Shared protocol types and data structures
  - Authentication DTOs (JWT exchange)
  - WebSocket frame envelopes and message routing
  - Session snapshots and participant management
  - All types implement `Serialize`/`Deserialize` for JSON wire format

- **`server/`** - Rust/Tokio server runtime  
  - HTTP endpoints for authentication
  - WebSocket hub for real-time event fanout
  - Session and participant management
  - Tempo authority coordination (future)

- **`utils/`** - Common utilities and helper functions
  - Application versioning and metadata
  - Configuration loading helpers (future)
  - Time and timestamp utilities (future)

## Development

### Prerequisites

- Rust 1.80+ (automatically managed via `rust-toolchain.toml`)
- Optional: [just](https://github.com/casey/just) for convenient development commands

### Quick Start

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Start the server
cargo run -p server

# Format code
cargo fmt

# Run lints
cargo clippy --workspace
```

### Using Just (optional)

If you have `just` installed, you can use convenient shortcuts:

```bash
just build      # Build all crates
just test       # Run all tests  
just lint       # Run clippy
just fmt        # Format code
just ci         # Run full CI pipeline locally
just run-server # Start the server
```

Install just with: `cargo install just`

## Roadmap

The following issues outline the planned implementation phases:

- **Issue #2**: Implement /auth/exchange endpoint using proto auth DTOs
- **Issue #3**: Implement state store (may expand proto types)  
- **Issue #4**: Implement WebSocket base referencing FrameEnvelope and Snapshot
- **Issue #5**: Add heartbeat logic
- **Issue #7**: Flutter client bootstrap

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed system design, component relationships, and performance targets.

## License

MIT License - see [LICENSE](./LICENSE) for details.
