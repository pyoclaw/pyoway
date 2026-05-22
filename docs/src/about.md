# About Pyoway

Pyoway is a personal project built entirely with Rust — from the web server to the frontend
to the documentation tooling.

## Tech Stack

| Component | Technology |
|---|---|
| Web Server | Axum (Rust) |
| Frontend | Leptos (WASM) + Tailwind CSS |
| Documentation | mdBook |
| CI/CD | GitHub Actions |

## Architecture

The project is split into three independent parts:

1. **Landing Server** — An Axum HTTP server serving the WASM frontend bundle
2. **Landing Frontend** — A Leptos SPA compiled to WebAssembly
3. **Documentation Site** — An mdBook-generated knowledge base hosted separately

## Source Code

The full source is available on [GitHub](https://github.com/pyoclaw/pyoway).
