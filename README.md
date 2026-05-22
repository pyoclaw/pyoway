# Pyoway

[![CI](https://github.com/pyoclaw/pyoway/actions/workflows/ci.yml/badge.svg)](https://github.com/pyoclaw/pyoway/actions/workflows/ci.yml)

**Pyoway** is a personal website and knowledge base built entirely with modern Rust tooling.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Pyoway Project                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐  │
│  │ landing-     │  │ landing-     │  │ docs/    │  │
│  │ server       │  │ frontend     │  │ (mdBook) │  │
│  │ (Axum)       │  │ (Leptos WASM)│  │          │  │
│  └──────┬───────┘  └──────┬───────┘  └────┬─────┘  │
└─────────┼──────────────────┼────────────────┼────────┘
          │                  │                │
          ▼                  ▼                ▼
   ┌──────────────┐  ┌──────────────┐  ┌─────────────┐
   │ Axum Server  │  │ Trunk WASM   │  │ mdBook      │
   │ :8080       │  │ Bundle      │  │ (static     │
   │             │  │ (dist/)     │  │  HTML)      │
   └──────┬───────┘  └──────┬───────┘  └──────┬──────┘
          │                  │                  │
          ▼                  ▼                  ▼
   ┌────────────────────────────────────────────────┐
   │         Deployment Targets                     │
   │  Landing: pyoway.dev                           │
   │  Docs:    docs.pyoway.dev                      │
   └────────────────────────────────────────────────┘
```

## Tech Stack

| Component | Technology |
|---|---|
| **Web Server** | [Axum](https://github.com/tokio-rs/axum) |
| **Frontend** | [Leptos](https://leptos.dev/) (WASM) |
| **Styling** | [Tailwind CSS](https://tailwindcss.com/) v4 |
| **Docs** | [mdBook](https://rust-lang.github.io/mdBook/) |
| **CI/CD** | GitHub Actions |

## Quick Start

### Prerequisites

- Rust 1.95.0+ (`rustup install 1.95.0`)
- WASM target (`rustup target add wasm32-unknown-unknown`)
- [Trunk](https://trunkrs.dev/) (`cargo install trunk`)
- [mdBook](https://rust-lang.github.io/mdBook/) (`cargo install mdbook`)
- Node.js + npm (for Tailwind CSS)

### Setup

```bash
# Clone the repository
git clone git@github.com:pyoclaw/pyoway.git
cd pyoway

# Copy environment config
cp .env.example .env

# Build the frontend WASM bundle
cd landing-frontend && trunk build --release && cd ..

# Run the server
cargo run -p landing-server
```

Visit `http://localhost:8080` in your browser.

### Development

```bash
# Check workspace
cargo check --workspace

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings

# Run tests
cargo test --workspace

# Build docs
mdbook build docs
```

### Docker

```bash
docker compose up --build
```

## Project Structure

```
pyoway/
├── landing-server/          # Axum HTTP server
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # Server entrypoint
│       ├── config.rs        # Environment configuration
│       └── error.rs         # Error types
├── landing-frontend/        # Leptos WASM frontend
│   ├── Cargo.toml
│   ├── index.html           # HTML shell
│   ├── Trunk.toml           # Trunk bundler config
│   └── src/
│       ├── main.rs          # WASM entrypoint
│       ├── lib.rs           # Root component
│       └── components/      # UI components
├── docs/                    # mdBook documentation site
│   ├── book.toml
│   ├── src/
│   └── theme/
├── .github/workflows/       # CI/CD pipelines
├── Cargo.toml               # Workspace root
├── Dockerfile
├── docker-compose.yml
└── Justfile                 # Command shortcuts
```

## License

MIT
