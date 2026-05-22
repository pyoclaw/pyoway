# Pyoway — GitHub Copilot Instructions

This file provides context for GitHub Copilot when working on the Pyoway project.

## Project Overview

Pyoway is a personal website and knowledge base built with Rust. It uses:
- **Axum 0.8** — HTTP server backend
- **Leptos 0.7** — WASM frontend framework
- **Tailwind CSS v4** — Utility-first styling
- **Trunk** — WASM bundler
- **mdBook** — Documentation site

## Coding Rules

### Rust
- Edition 2024, toolchain 1.95.0
- `unsafe_code = "deny"` — no unsafe Rust allowed
- `missing_docs = "deny"` — all public items must be documented
- `clippy::pedantic` at warn level
- Use `thiserror` for error types, not `anyhow`
- Use `tracing` for logging, not `log`
- Prefer explicit types on function signatures over inference

### Frontend (Leptos)
- Each component is one file in `src/components/` with tests in the same file
- Use `view! { ... }` macro for templates
- Use inline SVG for icons
- All styling via Tailwind utility classes
- Color palette: bg `#0a0a0f`, surface `#12121a`, accent blue `#3b82f6`

### Backend (Axum)
- Router built in `main.rs::build_router()`
- Config from env vars via `config.rs` with `from_env()` / `from_raw()` pattern
- Error types implement `IntoResponse`
- `handle_frontend` handles static files + SPA fallback

### Tests
- Tests are in the same file as the code, inside `#[cfg(test)] mod tests`
- Frontend tests use `leptos::tachys::view::RenderHtml::to_html()`
- Backend tests use `tower::ServiceExt::oneshot` with Axum routers

## Key Files

| File | Purpose |
|---|---|
| `Cargo.toml` | Workspace root |
| `landing-server/src/main.rs` | Server entrypoint + routing |
| `landing-frontend/src/lib.rs` | Root Leptos App component |
| `.cargo/config.toml` | Rustflags + profiles |
| `rust-toolchain.toml` | Pinned toolchain |
| `.github/workflows/ci.yml` | CI pipeline |
| `.github/labeler.yml` | PR labeling rules |

## Do NOT

- Do not add JavaScript frameworks (this is a Rust/WASM project)
- Do not use `unwrap()` in production code (use `expect()` with a message or handle errors)
- Do not add `anyhow` as a dependency (use `thiserror`)
- Do not change the `.cargo/config.toml` `rustflags` without updating all workflows
- Do not edit `mcp.json` or `.skills/` files unless asked to
