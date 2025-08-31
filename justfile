# Justfile for development commands
# Install with: cargo install just

# Default recipe to display help
default:
    @just --list

# Build all workspace crates
build:
    cargo build --workspace

# Run formatting on all code
fmt:
    cargo fmt --all

# Check formatting without making changes
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints
lint:
    cargo clippy --workspace -- -D warnings

# Run all tests
test:
    cargo test --workspace

# Run full CI pipeline locally
ci: fmt-check lint test

# Run the server binary
run-server:
    cargo run -p server

# Clean build artifacts
clean:
    cargo clean

# Install development dependencies
install-deps:
    rustup component add rustfmt clippy