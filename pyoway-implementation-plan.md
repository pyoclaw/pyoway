# Pyoway — Implementation Plan

> **Status:** Planning  
> **Based on:** `pyoway-spec.md` (Final Draft)  
> **Total Phases:** 4 (12 Epics, ~40+ individual tasks)  
> **Priority Order:** Workspace scaffold → Config files → Backend → Frontend → Docs → CI/CD → Polish

---

## Phase 0: Prerequisites & Tooling Installation

> **Goal:** Install all required tools before any code is written. Run once per development machine.

| # | Task | Command | Depends On | Est. Time |
|---|---|---|---|---|
| 0.1 | Install WASM target | `rustup target add wasm32-unknown-unknown` | — | 1m |
| 0.2 | Install core tools | `cargo install trunk wasm-pack mdbook` | — | 20m |
| 0.3 | Install quality tools | `cargo install cargo-machete cargo-spellcheck typos-cli cargo-geiger cargo-public-api` | — | 15m |
| 0.4 | Install test tools | `cargo install cargo-nextest cargo-llvm-cov cargo-mutants` | — | 10m |
| 0.5 | Install dep tools | `cargo install cargo-deny cargo-audit cargo-outdated cargo-sort cargo-semver-checks` | — | 15m |
| 0.6 | Install build/release tools | `cargo install cargo-dist cargo-wizard cargo-chef cargo-binstall cargo-expand cargo-pgo` | — | 20m |
| 0.7 | Install DX tools | `cargo install cargo-watch cargo-modules cargo-husky cargo-tally cargo-generate` | — | 10m |
| 0.8 | Install Tailwind CSS | `npm init -y && npm install -D tailwindcss @tailwindcss/cli` | — | 2m |
| 0.9 | Verify all tools | `cargo machete --version && cargo nextest --version && cargo deny --version && cargo audit --version` | 0.2–0.8 | 1m |

> **⏱ Total Phase 0:** ~94 minutes (mostly compile time, can run in background)

---

## Phase 1: Workspace Scaffold & Configuration

> **Goal:** Create the project skeleton — all config files, no application code yet.  
> **Priority:** HIGHEST — everything builds on this.

### Epic 1.1 — GitHub Repository

| # | Task | Key Details | Output Files | Validation |
|---|---|---|---|---|
| 1.1.1 | Create private GitHub repo | `gh repo create pyoway --private` | Remote: `github.com/pyoclaw/pyoway` | `gh repo view pyoway` |
| 1.1.2 | Initialize local git repo | `cd /home/pyo/workspace/pyoway && git init` | `.git/` | `git status` |
| 1.1.3 | Create `.gitignore` | Rust + WASM + mdBook + Node + IDE entries | `.gitignore` | Verify via `git check-ignore -v target/` |
| 1.1.4 | Set remote origin | `git remote add origin git@github.com:pyoclaw/pyoway.git` | `.git/config` | `git remote -v` |

### Epic 1.2 — Rust Workspace Configuration

| # | Task | Key Details | Output Files | Validation |
|---|---|---|---|---|
| 1.2.1 | Create workspace `Cargo.toml` | `[workspace] resolver = "2" members = ["landing-server", "landing-frontend"] edition = "2024"` | `Cargo.toml` | `cargo metadata --format-version 1` |
| 1.2.2 | Add workspace lints | `[lints.workspace] unsafe_code = "deny" clippy::pedantic = "warn" ...` | `Cargo.toml` (updated) | — |
| 1.2.3 | Create `rust-toolchain.toml` | Pin to `1.95.0`, components: `rustc, cargo, clippy, rustfmt, rust-analyzer, rust-src, rust-docs`, targets: `wasm32-unknown-unknown` | `rust-toolchain.toml` | `rustup show` |
| 1.2.4 | Create `.cargo/config.toml` | `rustflags = ["-D", "unsafe_code", "-W", "clippy::pedantic"]`, profiles (dev opt-level=1, release lto=fat, strip=symbols) | `.cargo/config.toml` | `cargo check` (must succeed) |

### Epic 1.3 — Tooling Configuration Files

| # | Task | Key Details | Output Files | Validation |
|---|---|---|---|---|
| 1.3.1 | Create `deny.toml` | Advisory severity, license allowlist (MIT, Apache-2.0, BSD-3-Clause, Unicode-DFS-2016), ban duplicate versions | `deny.toml` | `cargo deny check` |
| 1.3.2 | Create `.github/clippy.toml` | `doc-valid-idents`, `allowed-duplicate-crates` | `.github/clippy.toml` | `cargo clippy` |
| 1.3.3 | Create `.env.example` | `HOST=127.0.0.1`, `PORT=8080`, `RUST_LOG=info`, `CORS_ORIGIN=http://localhost:8080` | `.env.example` | Manual review |
| 1.3.4 | Create initial `README.md` | Project name, description, quick start, tech stack badges, docs link | `README.md` | Manual review |

### Epic 1.4 — VS Code / Editor Configuration (LLM-Optimized)

| # | Task | Key Details | Output Files | Validation |
|---|---|---|---|---|
| 1.4.1 | Create `.vscode/extensions.json` | 7 recommended extensions (rust-analyzer, even-better-toml, tailwind, lldb, dependi, gh-actions, spell-checker) | `.vscode/extensions.json` | Manual review |
| 1.4.2 | Create `.vscode/settings.json` | rust-analyzer config (check=clippy, all features, proc macros, inlay hints), TOML formatting, editor rules | `.vscode/settings.json` | Manual review |
| 1.4.3 | Validate LLM-optimized settings | Ensure inlay hints, proc macro attrs, closure return hints are enabled | `.vscode/settings.json` | Manual review |

### Epic 1.5 — Crate Scaffolds (Empty Stubs)

| # | Task | Key Details | Output Files | Validation |
|---|---|---|---|---|
| 1.5.1 | Scaffold `landing-server` | `cargo init landing-server --edition 2024`; add `actix-web = "4.13"`, `tokio = { version = "1", features = ["full"] }`, `serde`, `serde_json`, `tracing`, `tracing-subscriber`, `actix-files`, `actix-cors`, `tower-http`, `dotenv` to `[dependencies]`; add workspace lints reference | `landing-server/Cargo.toml`, `landing-server/src/main.rs` | `cargo check -p landing-server` |
| 1.5.2 | Scaffold `landing-frontend` | `cargo init landing-frontend --edition 2024 --lib`; add `leptos`, `leptos_meta`, `leptos_router`, `wasm-bindgen`, `console_error_panic_hook`, `web-sys`; add crate-type = ["cdylib", "rlib"]; add workspace lints reference | `landing-frontend/Cargo.toml`, `landing-frontend/src/lib.rs` | `cargo check -p landing-frontend` (after wasm target) |
| 1.5.3 | Initialize mdBook skeleton | `mdbook init docs`; configure `docs/book.toml` with title "Pyoway Knowledge Base", custom theme dir | `docs/book.toml`, `docs/src/SUMMARY.md`, `docs/src/intro.md` | `mdbook build docs` |

> **⏱ Total Phase 1:** ~15 minutes (mostly config, fast)

---

## Phase 2: Landing Page Backend (`landing-server`)

> **Goal:** A production-ready Actix-web server that serves the WASM bundle and API routes.

### Epic 2.1 — Server Foundation

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 2.1.1 | Implement `config.rs` | Load env vars via dotenv, struct `AppConfig` with `host`, `port`, `cors_origin`, `log_level` | `landing-server/src/config.rs` | Unit tests |
| 2.1.2 | Implement `errors.rs` | Custom `AppError` enum, `ResponseError` impl for each variant, JSON error response format | `landing-server/src/errors.rs` | Unit tests |
| 2.1.3 | Implement `middleware.rs` | CORS middleware (permissive in dev), compression (br/gzip), request logging (tracing), security headers (HSTS, CSP, X-Frame-Options, X-Content-Type-Options), custom error handler middleware | `landing-server/src/middleware.rs` | Integration test |
| 2.1.4 | Implement `routes.rs` | `GET /` → serve `index.html`, `GET /{path:.*}` → static files from `dist/`, `GET /health` → health check endpoint, SPA fallback (unknown paths → index.html) | `landing-server/src/routes.rs` | Unit + integration test |
| 2.1.5 | Implement `main.rs` | Wire up: dotenv init → tracing subscriber → parse config → build App (middleware stack + routes) → bind server → graceful shutdown with tokio signal | `landing-server/src/main.rs` | `cargo run` + curl test |
| 2.1.6 | Verify compilation & clippy | `cargo check -p landing-server && cargo clippy -p landing-server` | — | Passes clean |

### Epic 2.2 — Server Tests

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 2.2.1 | Add `rstest` dev-dep | `cargo add rstest --dev` | `landing-server/Cargo.toml` | `cargo check` |
| 2.2.2 | Write health check test | `GET /health` returns 200 + JSON `{"status": "ok"}` | Test file | `cargo nextest run` |
| 2.2.3 | Write 404 test | Unknown route returns index.html (SPA fallback) | Test file | `cargo nextest run` |
| 2.2.4 | Write config test | Invalid config panics, valid config parses | Test file | `cargo nextest run` |

> **⏱ Total Phase 2:** ~30 minutes

---

## Phase 3: Landing Page Frontend (`landing-frontend`)

> **Goal:** A stunning Leptos WASM SPA with Tailwind CSS, matching Vercel-inspired bold & colorful design.

### Epic 3.1 — Foundation & Build Setup

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 3.1.1 | Create `index.html` | Minimal HTML shell with `<meta charset="UTF-8">`, viewport meta, `<title>Pyoway</title>`, Google Fonts link for Inter + JetBrains Mono, `<div id="app"></div>`, module script link for WASM | `landing-frontend/index.html` | Manual review |
| 3.1.2 | Create `Trunk.toml` | Configure build: `release = true`, `output = "dist/"`, `public_url = "/"`, `hooks` for `tailwindcss -i` | `landing-frontend/Trunk.toml` | Manual review |
| 3.1.3 | Create `tailwind.css` | `@tailwind base; @tailwind components; @tailwind utilities;` + custom theme colors matching spec (midnight, electric blue, violet, cyan) + custom fonts | `landing-frontend/src/styles/tailwind.css` | Manual review |
| 3.1.4 | Implement `main.rs` | Leptos mount: `leptos::mount_to_body(|| view! { <App/> })` | `landing-frontend/src/main.rs` | `cargo check` |
| 3.1.5 | Implement `app.rs` | Root component with `<Router>`, `<Routes>`, scroll-to-top on navigation, meta tags | `landing-frontend/src/app.rs` | `cargo check` |
| 3.1.6 | Import fonts | Link Inter (weights 400, 500, 600, 700) and JetBrains Mono (weight 400) in `index.html` | `landing-frontend/index.html` | Manual review |

### Epic 3.2 — UI Components

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 3.2.1 | Implement `nav.rs` | Fixed top navigation bar, transparent → solid on scroll, links (Home, Docs, GitHub icon), smooth scroll behavior | `landing-frontend/src/components/nav.rs` | Visual review |
| 3.2.2 | Implement `hero.rs` | Full-viewport hero with gradient background (blue → violet), animated particles/shapes behind, name + tagline, CTA button "View My Work", scroll-down indicator (animated chevron), responsive text sizing | `landing-frontend/src/components/hero.rs` | Visual review |
| 3.2.3 | Implement `features.rs` | 3-4 feature cards in responsive grid (1 col mobile, 2 col tablet, 3-4 col desktop), each with icon (SVG/lucide), title, description, hover: lift + glow effect with `transition: all 0.3s ease` | `landing-frontend/src/components/features.rs` | Visual review |
| 3.2.4 | Implement `docs_link.rs` | Section with "Knowledge Base" heading, descriptive text "Thoughts, tutorials, and deep dives", CTA "Visit the Knowledge Base →" linking to external docs subdomain, subtle background accent | `landing-frontend/src/components/docs_link.rs` | Visual review |
| 3.2.5 | Implement `footer.rs` | Links row: GitHub, Twitter/X, Email, Docs; copyright line; "Built with Rust 🦀" credit; dark background matching spec | `landing-frontend/src/components/footer.rs` | Visual review |

### Epic 3.3 — Animations & Polish

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 3.3.1 | Add scroll-triggered fade-in | Sections fade up on scroll using intersection observer (leptos_use or custom) | `landing-frontend/src/app.rs` | Visual review |
| 3.3.2 | Add hero particle animation | CSS or canvas-based animated gradient/orb effect behind the hero text | `landing-frontend/src/components/hero.rs` | Visual review |
| 3.3.3 | Add loading state | Skeleton/spinner shown while WASM is loading (in `index.html` before app mounts) | `landing-frontend/index.html` | Visual review |
| 3.3.4 | Verify build | `trunk build --release` completes successfully | `landing-frontend/dist/` | `trunk serve` + browser check |

> **⏱ Total Phase 3:** ~60 minutes (most design work)

---

## Phase 4: Documentation Site (mdBook)

> **Goal:** A well-structured, visually customized blog + knowledge base.

### Epic 4.1 — Configuration & Theme

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 4.1.1 | Configure `book.toml` | Title: "Pyoway Knowledge Base", author: "pyoclaw", multilingual=false, `[output.html]` with search, code highlighting, git-repository-url, edit-url-template | `docs/book.toml` | `mdbook build docs` |
| 4.1.2 | Create custom `theme/custom.css` | Override mdBook theme to match landing page colors (dark bg, blue/violet accents, Inter font), responsive adjustments | `docs/theme/custom.css` | `mdbook serve docs` + browser |

### Epic 4.2 — Content Structure

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 4.2.1 | Write `SUMMARY.md` | Full table of contents: Introduction, Blog (list), Knowledge Base (Rust, Web Dev, Tools), About | `docs/src/SUMMARY.md` | `mdbook build` |
| 4.2.2 | Write `intro.md` | Welcome page, what this site is about, quick navigation guide | `docs/src/intro.md` | Manual review |
| 4.2.3 | Write `about.md` | About the author, tech stack used, link to main site | `docs/src/about.md` | Manual review |
| 4.2.4 | Create blog section | `blog/post-template.md` (template with front matter), `blog/hello-world.md` (first post — "Welcome to Pyoway") | `docs/src/blog/*.md` | `mdbook build` |
| 4.2.5 | Create KB section | `knowledge-base/rust/` (placeholder "Why Rust?", "Leptos Notes"), `knowledge-base/web-dev/` (placeholder "Modern WASM"), `knowledge-base/tools/` (placeholder "Cargo Tools Guide") | `docs/src/knowledge-base/**/*.md` | `mdbook build` |

> **⏱ Total Phase 4:** ~30 minutes

---

## Phase 5: CI/CD & Automation

> **Goal:** Automated quality gates and builds for every push.

### Epic 5.1 — GitHub Actions Workflows

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 5.1.1 | Create `ci.yml` | Trigger: push/PR to main. Jobs: `fmt` (cargo fmt --check), `clippy` (cargo clippy -- -D warnings), `test` (cargo nextest run), `audit` (cargo audit), `machete` (cargo machete), `deny` (cargo deny check) | `.github/workflows/ci.yml` | Push to test |
| 5.1.2 | Create `build-landing.yml` | Trigger: tag `v*` or manual. Steps: install wasm target, trunk build --release, upload dist/ as artifact | `.github/workflows/build-landing.yml` | Manual trigger |
| 5.1.3 | Create `build-docs.yml` | Trigger: push to main. Steps: mdbook build docs/, upload book/ as artifact | `.github/workflows/build-docs.yml` | Push to test |

### Epic 5.2 — Local Git Hooks

| # | Task | Key Details | Files | Validation |
|---|---|---|---|---|
| 5.2.1 | Init cargo-husky | `cargo husky init` | `.husky/` | `git commit` test |
| 5.2.2 | Add pre-commit hook | Run `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check` | `.husky/pre-commit` | `cargo test` |

> **⏱ Total Phase 5:** ~20 minutes

---

## Phase 6: Integration & Smoke Testing

> **Goal:** End-to-end verification that everything works together.

### Epic 6.1 — Local Integration Test

| # | Task | Key Details | Validation |
|---|---|---|---|
| 6.1.1 | Full workspace check | `cargo check --workspace` | Passes clean |
| 6.1.2 | Full clippy pass | `cargo clippy --workspace --all-targets -- -D warnings` | Zero warnings |
| 6.1.3 | Run all tests | `cargo nextest run` | All green |
| 6.1.4 | Build WASM frontend | `cd landing-frontend && trunk build --release` | Produces `dist/` |
| 6.1.5 | Build mdBook docs | `mdbook build docs` | Produces `docs/book/` |
| 6.1.6 | Run cargo-deny | `cargo deny check` | All licenses OK |
| 6.1.7 | Run cargo-audit | `cargo audit` | No advisories |
| 6.1.8 | Run cargo-machete | `cargo machete` | No unused deps |
| 6.1.9 | Start server + verify | `cargo run -p landing-server` + `curl http://localhost:8080/` | Returns `index.html` |
| 6.1.10 | Verify docs serve | `mdbook serve docs` + `curl http://localhost:3000/` | Returns docs HTML |

### Epic 6.2 — Final Polish & Review

| # | Task | Key Details | Validation |
|---|---|---|---|
| 6.2.1 | Code review pass | Spawn code-reviewer-deepseek-flash on all changes | Review feedback addressed |
| 6.2.2 | README finalization | Add badges, CI status, setup instructions, architecture diagram reference | Manual review |
| 6.2.3 | Initial commit & push | `git add . && git commit -m "feat: initial pyoway scaffold" && git push -u origin main` | GitHub shows all files |

> **⏱ Total Phase 6:** ~20 minutes

---

## Phase 7: Future (Post-v1)

| # | Task | Priority |
|---|---|---|
| 7.1 | Add contact form API endpoint | Medium |
| 7.2 | Dockerfile + docker-compose for landing-server | High |
| 7.3 | Set up docs deployment (Netlify/Vercel/GitHub Pages) | High |
| 7.4 | Add blog RSS/Atom feed | Low |
| 7.5 | Dark/light theme toggle | Medium |
| 7.6 | Analytics integration (Plausible or Umami) | Low |
| 7.7 | Custom MCP server for site-specific AI tooling | Low |

---

## Dependency Graph

```
Phase 0 (Tools)
    │
    ▼
Phase 1 (Scaffold) ──────────────────────┐
    │                                     │
    ├──► Epic 1.1 (GitHub)                │
    ├──► Epic 1.2 (Workspace)            │
    ├──► Epic 1.3 (Config files)         │
    ├──► Epic 1.4 (VS Code)              │
    └──► Epic 1.5 (Crate stubs)          │
                                          │
Phase 2 (Backend) ◄───────────────────────┘
    │
    ├──► Epic 2.1 (Server foundation)
    └──► Epic 2.2 (Server tests)
    
Phase 3 (Frontend) ◄──────────────────────┐
    │                                     │
    ├──► Epic 3.1 (Foundation + build)    │
    ├──► Epic 3.2 (UI components)        │ Depends on 1.5.2
    └──► Epic 3.3 (Animations)           │

Phase 4 (Docs) ◄──────────────────────────┐
    │                                     │
    ├──► Epic 4.1 (Config + theme)       │ Depends on 1.5.3
    └──► Epic 4.2 (Content)              │

Phase 5 (CI/CD) ◄─────────────────────────┐
    │                                     │ Can start after 1.2
    ├──► Epic 5.1 (GitHub Actions)       │
    └──► Epic 5.2 (Git hooks)            │

Phase 6 (Integration) ◄───────────────────┘
    │                                     Depends on Phases 2-5
    ├──► Epic 6.1 (Smoke tests)
    └──► Epic 6.2 (Final polish)
```

---

## Execution Tips

### Parallelizable Tasks
- **Phase 0:** All cargo installs can run in parallel (different packages)
- **Phase 1 Epic 1.2 + 1.3 + 1.4:** Config files are independent
- **Phase 2 Epic 2.1:** All source files can be implemented in parallel
- **Phase 3 Epic 3.2:** All 5 components are independent
- **Phase 5 Epic 5.1:** All 3 workflow files are independent

### Validation Gates Per Phase
- End of Phase 1: `cargo check --workspace` passes
- End of Phase 2: `cargo nextest run` passes
- End of Phase 3: `trunk build --release` produces `dist/`
- End of Phase 4: `mdbook build docs` produces `docs/book/`
- End of Phase 6: All CI gates pass

### Time Estimate
- **Total:** ~4-5 hours (assuming no research delays)
- **Mostly compile time:** Phase 0 (~90m compile), Phase 2-3 (~30m compile)
- **Quickest wins:** Phase 1 (config files), Phase 4 (mdBook content)
