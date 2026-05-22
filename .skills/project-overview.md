# Pyoway — Project Overview

## What is Pyoway?

A personal website + knowledge base built entirely with modern Rust tooling. The project consists of two independent services:

1. **Landing Page** — Axum HTTP server serving a Leptos + Tailwind CSS WASM frontend
2. **Documentation Site** — mdBook-generated blog/knowledge base deployed to GitHub Pages

## Repository

- **URL:** `github.com/pyoclaw/pyoway`
- **Visibility:** Public
- **Default branch:** `main`
- **License:** MIT

## Tech Stack

| Component | Technology | Purpose |
|---|---|---|
| HTTP Server | [Axum](https://github.com/tokio-rs/axum) 0.8 | Serves WASM bundle, API routes |
| Frontend | [Leptos](https://leptos.dev/) 0.7 | Reactive WASM SPA |
| Styling | [Tailwind CSS](https://tailwindcss.com/) v4 | Utility-first CSS (via npm CLI) |
| Docs | [mdBook](https://rust-lang.github.io/mdBook/) | Static documentation site |
| Bundler | [Trunk](https://trunkrs.dev/) | WASM build + dev server |
| CI/CD | GitHub Actions | Quality gates, builds, deployments |
| Hosting (Landing) | Server binary via Docker | `pyoway.dev` |
| Hosting (Docs) | GitHub Pages | `pyoclaw.github.io/pyoway/` |

## Architecture

```
pyoway/
├── landing-server/          # Axum HTTP server (binary crate)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs          # Server entrypoint, routing, middleware
│       ├── config.rs        # Env-based configuration
│       └── error.rs         # Error types + HTTP response conversion
├── landing-frontend/        # Leptos WASM frontend (lib crate)
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── index.html           # HTML shell with loading splash
│   ├── Trunk.toml           # Trunk bundler config
│   ├── package.json         # Tailwind CSS npm dependency
│   ├── package-lock.json
│   └── src/
│       ├── main.rs          # WASM entrypoint
│       ├── lib.rs           # Root App component, routing
│       └── components/      # UI components
│           ├── mod.rs       # Re-exports
│           ├── nav.rs       # Fixed navigation bar
│           ├── hero.rs      # Full-viewport hero section
│           ├── features.rs  # Feature cards grid
│           ├── docs_link.rs # Knowledge Base CTA section
│           └── footer.rs    # Footer with links
├── docs/                    # mdBook documentation site
│   ├── book.toml
│   ├── src/                 # Markdown content
│   └── theme/               # Custom CSS overrides
├── .github/                 # CI/CD workflows + config
├── Cargo.toml               # Workspace root
├── Justfile                 # Development command shortcuts
├── Dockerfile               # Multi-stage build for server
└── docker-compose.yml       # Local deployment
```

## Workspace Structure

A Cargo workspace at the repository root using Rust 2024 edition:

```toml
[workspace]
resolver = "2"
members = ["landing-server", "landing-frontend"]
edition = "2024"
```

### Key Design Decisions

1. **Pure Rust everywhere** — No JavaScript framework; Leptos compiles to WASM
2. **No `unsafe` code** — Denied at the workspace level via `[lints.rust] unsafe_code = "deny"`
3. **Clippy pedantic** — Warn-level, with some allowed exceptions
4. **SPA fallback** — Unknown routes serve `index.html` for client-side routing
5. **Security-first** — HSTS, CSP, X-Frame-Options headers; path traversal sanitization
