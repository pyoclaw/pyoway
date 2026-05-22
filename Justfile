# Pyoway development commands

# Default: run all checks
default: check

# Check entire workspace
check:
    cargo check --workspace

# Run clippy on entire workspace
clippy:
    cargo clippy --workspace --all-targets -- -D warnings -A clippy::multiple_crate_versions -A clippy::must_use_candidate

# Format check
fmt:
    cargo fmt --check --all

# Run all tests
test:
    cargo nextest run

# Run all tests (fallback to cargo test)
test-all:
    cargo nextest run || cargo test

# Run security audit
audit:
    cargo deny check
    cargo audit --ignore RUSTSEC-2024-0436

# Check for unused deps
machete:
    cargo machete

# Build frontend WASM bundle
build-frontend:
    cd landing-frontend && trunk build --release

# Build mdBook docs
build-docs:
    mdbook build docs

# Test mdBook docs build output
test-docs:
    bash docs/test-docs.sh

# Serve docs locally
serve-docs:
    mdbook serve docs

# Run full CI check locally
ci: fmt clippy check test audit machete

# Clean build artifacts
clean:
    cargo clean
    rm -rf landing-frontend/dist docs/book

# Install essential development tools
install-tools:
    cargo install cargo-nextest cargo-deny cargo-audit cargo-machete cargo-watch

# Watch and check on changes
watch:
    cargo watch -x check
