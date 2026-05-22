# Pyoway — Project Specification

> **Status:** Final Draft Spec  
> **Last Updated:** 2025-06-18  
> **Author:** Generated via user interview & research

---

## 1. Project Overview

**Pyoway** is a personal website + blog project built entirely with modern Rust tooling. It consists of two independent services:

1. **Landing Page** — A bold, colorful personal site served via Actix-web with a Leptos + Tailwind WASM frontend.
2. **Documentation Site** — A personal blog / knowledge base generated with mdBook from Markdown, hosted separately on a custom subdomain.

---

## 2. Repository

| Field | Value |
|---|---|
| **Name** | `pyoway` |
| **Visibility** | Private |
| **Owner** | `pyoclaw` (personal GitHub account) |
| **Creation tool** | `gh repo create pyoway --private` |
| **Local root** | `/home/pyo/workspace/pyoway` |

---

## 3. Workspace Structure

A Cargo workspace at the repository root, using **Rust 2024 edition**.

### 3.1 Member Crates

| Crate | Type | Description |
|---|---|---|
| `landing-server` | Binary | Actix-web HTTP server (full-featured: middleware, WebSockets, security headers, error handling) |
| `landing-frontend` | WASM binary | Leptos + Tailwind CSS frontend, compiled to WASM via Trunk |
| `docs/` (directory, not a crate) | mdBook site | Markdown-based blog/knowledge base, built with `mdbook` CLI |

### 3.2 Root Workspace `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = ["landing-server", "landing-frontend"]
edition = "2024"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
```

### 3.3 Toolchain

| Tool | Version / Source |
|---|---|
| **Rust** | 1.95.0 (installed via rustup) |
| **Cargo** | 1.95.0 |
| **Edition** | 2024 |
| **Lints** | `rust-2024-compatibility`, `clippy::pedantic`, `unsafe_code = "deny"` |

---

## 4. Dependencies

### 4.1 `landing-server` (Actix-web)

| Crate | Version | Purpose |
|---|---|---|
| `actix-web` | 4.13.0 | HTTP server framework |
| `actix-files` | latest | Static file serving for WASM bundle |
| `actix-cors` | latest | CORS middleware |
| `actix-web-flash-messages` | latest | Flash messages / session |
| `tokio` | 1.x (full features) | Async runtime |
| `serde` / `serde_json` | latest | JSON serialization |
| `tower-http` | latest | Compression & security headers |
| `tracing` / `tracing-subscriber` | latest | Structured logging |
| `mime_guess` | latest | MIME type detection for static files |
| `dotenv` | latest | Config from `.env` |

**Middleware stack:**
- CORS (permissive for dev, locked down in prod)
- Compression (br/gzip)
- Request logging (tracing)
- Security headers (HSTS, CSP, X-Frame-Options, etc.)
- Custom error handler

### 4.2 `landing-frontend` (Leptos + Tailwind)

| Crate | Version | Purpose |
|---|---|---|
| `leptos` | 0.9.0-alpha (latest) | Reactive WASM framework |
| `leptos_meta` | 0.9.x | HTML `<head>` management (title, meta, links) |
| `leptos_router` | 0.9.x | Client-side routing |
| `wasm-bindgen` | latest | JS interop |
| `console_error_panic_hook` | latest | Better WASM panics |
| `web-sys` | latest | Browser API bindings |

**Build tools:**
- `trunk` 0.22.0-beta.1 — WASM bundler and dev server
- `tailwindcss` v4 — utility-first CSS (via `npm` or standalone binary)

### 4.3 Documentation (mdBook)

- **Tool:** `mdbook` 0.5.3 (installed via `cargo install mdbook`)
- **Theme:** Custom (bold & colorful, matching the landing page aesthetic)
- **Features:** Search, code highlighting, print, custom CSS overrides

---

## 5. Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    GitHub (pyoclaw/pyoway)               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ landing-     │  │ landing-     │  │ docs/        │  │
│  │ server       │  │ frontend     │  │ (mdBook src) │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
└─────────┼──────────────────┼──────────────────┼─────────┘
          │                  │                  │
          ▼                  ▼                  ▼
   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
   │ Actix-web   │   │ Trunk WASM  │   │ mdbook      │
   │ Server :8080│   │ Bundle      │   │ (static     │
   │            │   │ (dist/)     │   │  HTML output)│
   └──────┬──────┘   └──────┬──────┘   └──────┬──────┘
          │                  │                  │
          ▼                  ▼                  ▼
   ┌────────────────────────────────────────────────────┐
   │               Deployment Targets                   │
   │  Landing: custom-domain.com (or subdomain)         │
   │  Docs:    docs.custom-domain.com (separate host)   │
   └────────────────────────────────────────────────────┘
```

### 5.1 Request Flow (Landing Page)

```
Browser ──HTTPS──► Actix-web ──serve static──► dist/ (Trunk WASM bundle)
                       │
                       ├── GET /  ───► landing-frontend WASM app
                       ├── GET /:route ──► SPA fallback to index.html
                       ├── POST /api/* ──► API handlers (future)
                       └── WebSocket /ws/* ──► WS handlers (future)
```

### 5.2 Documentation Site

- Built independently via `mdbook build`
- Output: `docs/book/` (static HTML, CSS, JS, search index)
- Hosted separately on a custom subdomain (e.g., `docs.pyoway.dev`)
- Deployment: User handles their own hosting (e.g., Netlify, Vercel, DigitalOcean)

---

## 6. Landing Page Design

### 6.1 Design Inspiration

- **Primary:** Vercel (vercel.com) — dark gradient hero, clean grid layouts, bold typography
- **Vibe:** Bold & colorful, modern, tech-forward

### 6.2 Color Palette (Preliminary)

| Role | Color | Hex |
|---|---|---|
| Background | Deep midnight | `#0a0a0f` |
| Surface | Dark card | `#12121a` |
| Primary accent | Electric blue | `#3b82f6` |
| Secondary accent | Violet | `#8b5cf6` |
| Tertiary accent | Cyan | `#06b6d4` |
| Text primary | White | `#f8fafc` |
| Text muted | Slate | `#94a3b8` |
| Success | Emerald | `#10b981` |
| Gradient hero | Blue → Violet | `linear-gradient(to right, #3b82f6, #8b5cf6)` |

### 6.3 Typography

- **Headings:** Inter, or system-ui stack
- **Body:** Inter, or system-ui stack
- **Monospace:** JetBrains Mono (for code snippets)
- **Scale:** Modular scale with 1.25 ratio

### 6.4 Page Sections

All sections on a single-page scroll:

1. **Hero**
   - Name + one-line tagline describing the person/project
   - Bold gradient background with animated particles/shapes
   - Call-to-action button (e.g., "View My Work" → scrolls to features/docs)
   - Subtle scroll-down indicator

2. **Features / Highlights**
   - 3-4 cards showcasing key aspects (e.g., skills, projects, interests)
   - Grid layout with hover effects, subtle glow on cards
   - Each card has an icon, title, and brief description

3. **Docs Link / Knowledge Base Preview**
   - Prominent section linking to the mdBook documentation site
   - "Visit the Knowledge Base" CTA with arrow
   - Context: "Thoughts, tutorials, and deep dives"

4. **Footer**
   - Links: GitHub, Twitter/X, Email, Docs
   - Copyright line
   - "Built with Rust 🦀" credit

### 6.5 Micro-interactions & Animations

- **Hero:** Subtle particle/shape animation in the background (WebGL or CSS)
- **Cards:** Hover lift + glow effect, smooth transitions (`transition: all 0.3s ease`)
- **Scroll:** Smooth scroll behavior, optional scroll-triggered fade-in for sections
- **Navigation:** Smooth scroll to section, active state indicator
- **Loading:** Skeleton screen or animated logo while WASM loads

---

## 7. Workspace Tooling & Skills (Recommended)

All cargo tools listed below are intended to be **installed globally** (`cargo install <tool>`) and used across the entire workspace. They are organized by category.

### 7.1 Code Quality & Linting

| Tool | Version | Install | Purpose |
|---|---|---|---|
| **clippy** | (bundled with Rust) | `rustup component add clippy` | Official Rust linter — pedantic, nursery, perf, correctness lints |
| **cargo-machete** | 0.9.2 | `cargo install cargo-machete` | Detects unused dependencies in Cargo.toml files |
| **cargo-spellcheck** | 0.15.7 | `cargo install cargo-spellcheck` | Spell-checks doc comments and rustdoc |
| **cargo-geiger** | 0.13.0 | `cargo install cargo-geiger` | Detects `unsafe` usage across the crate graph |
| **cargo-public-api** | 0.51.0 | `cargo install cargo-public-api` | Lists/diffs the public API surface |
| **typos-cli** | 1.46.2 | `cargo install typos-cli` | Fast spell checker for source code |

### 7.2 Testing & Coverage

| Tool | Version | Install | Purpose |
|---|---|---|---|
| **rstest** | 0.26.1 | *(dev-dependency)* | Fixture-based testing, table tests, parametrized tests |
| **cargo-nextest** | 0.9.136 | `cargo install cargo-nextest` | Next-gen test runner — fast parallel execution, per-test timeout |
| **cargo-llvm-cov** | 0.8.7 | `cargo install cargo-llvm-cov` | LLVM-native code coverage |
| **cargo-mutants** | 27.0.0 | `cargo install cargo-mutants` | Mutation testing — verifies test suite quality |

### 7.3 Dependency Management

| Tool | Version | Install | Purpose |
|---|---|---|---|
| **cargo-deny** | 0.19.6 | `cargo install cargo-deny` | Audits deps for licenses, security advisories, duplicates |
| **cargo-audit** | 0.22.1 | `cargo install cargo-audit` | Checks dep tree against RustSec Advisory Database |
| **cargo-outdated** | 0.19.0 | `cargo install cargo-outdated` | Shows outdated deps with semver-compatible update suggestions |
| **cargo-sort** | 2.1.4 | `cargo install cargo-sort` | Sorts dependency lists alphabetically in Cargo.toml |
| **cargo-semver-checks** | 0.47.0 | `cargo install cargo-semver-checks` | Lints API changes for semver violations |

### 7.4 Build & Release

| Tool | Version | Install | Purpose |
|---|---|---|---|
| **cargo-dist** | 0.31.0 | `cargo install cargo-dist` | Generates CI pipelines for building/publishing binaries |
| **cargo-wizard** | 0.2.3 | `cargo install cargo-wizard` | Applies Cargo profile templates (dev, release, size, speed) |
| **cargo-chef** | 0.1.77 | `cargo install cargo-chef` | Docker layer caching for Rust builds |
| **cargo-binstall** | 1.19.1 | `cargo install cargo-binstall` | Installs Rust binaries from GitHub releases (skips compilation) |
| **cargo-expand** | 1.0.122 | `cargo install cargo-expand` | Expands Rust macros |
| **cargo-pgo** | 0.3.0 | `cargo install cargo-pgo` | Profile-Guided Optimization for release binaries |

### 7.5 Developer Experience

| Tool | Version | Install | Purpose |
|---|---|---|---|
| **cargo-watch** | 8.5.3 | `cargo install cargo-watch` | Watches files and re-runs commands on change |
| **cargo-modules** | 0.26.0 | `cargo install cargo-modules` | Visualizes crate module structure as a tree |
| **cargo-husky** | 1.5.0 | `cargo install cargo-husky` | Git hooks for Cargo — runs clippy/test/fmt on commit |
| **cargo-tally** | 1.0.74 | `cargo install cargo-tally` | Generates dependency usage graphs |
| **cargo-generate** | 0.23.9 | `cargo install cargo-generate` | Template-based project generation |

### 7.6 WASM / Platform Specific

| Tool | Version | Install | Purpose |
|---|---|---|---|
| **wasm-pack** | (latest) | `cargo install wasm-pack` | WASM build toolchain |
| **trunk** | 0.22.0-beta.1 | `cargo install trunk` | WASM web bundler — dev server, asset pipeline |
| **cargo-lambda** | 1.9.1 | `cargo install cargo-lambda` | AWS Lambda deployment (future) |

### 7.7 Configuration Files for Tooling

```toml
# .cargo/config.toml — Cargo project config
[target.x86_64-unknown-linux-gnu]
rustflags = ["-D", "unsafe_code", "-W", "clippy::pedantic", "-W", "clippy::cargo", "-W", "clippy::nursery"]

[profile.dev]
opt-level = 1  # Faster dev builds while maintaining debuggability

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = "symbols"
```

```yaml
# .github/clippy.toml — Clippy configuration
doc-valid-idents = ["Pyoway", "Leptos", "Actix", "WASM", "mdBook"]
allowed-duplicate-crates = ["syn", "quote", "proc-macro2"]
```

```toml
# deny.toml — cargo-deny configuration
[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "deny"

[licenses]
unlicensed = "deny"
allow = ["MIT", "Apache-2.0", "BSD-3-Clause", "Unicode-DFS-2016"]

[bans]
multiple-versions = "deny"
skip = [
  { name = "syn", allow-registry = ["*"] },
]
```

### 7.8 One-Shot Install Script

```bash
# Install all recommended tools at once
cargo install cargo-machete cargo-spellcheck typos-cli cargo-geiger cargo-public-api
cargo install cargo-nextest cargo-llvm-cov cargo-mutants
cargo install cargo-deny cargo-audit cargo-outdated cargo-sort cargo-semver-checks
cargo install cargo-dist cargo-wizard cargo-chef cargo-binstall cargo-expand cargo-pgo
cargo install cargo-watch cargo-modules cargo-husky cargo-tally cargo-generate
```

---

## 8. LLM-Enhancing Tools & Techniques for Rust

These tools and configurations help LLMs (like Codebuff, Copilot, Cursor, etc.) work more effectively with Rust code.

### 8.1 rust-analyzer Configuration (LLM-Optimized)

```jsonc
// .vscode/settings.json — rust-analyzer config optimized for LLM-assisted coding
{
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.procMacro.attributes.enable": true,
  "rust-analyzer.completions.addCallParenthesis": true,
  "rust-analyzer.completions.addCallArgumentSnippets": true,
  "rust-analyzer.imports.granularity.group": "module",
  "rust-analyzer.rustfmt.extraArgs": ["+stable"],
  "rust-analyzer.semanticHighlighting.enable": true,
  "rust-analyzer.callInfo.full": true,
  "rust-analyzer.linkedProjects": ["Cargo.toml"],
  "rust-analyzer.diagnostics.disabled": ["unresolved-proc-macro"],
  // Inlay hints for maximum LLM context
  "rust-analyzer.inlayHints.chainingHints.enable": true,
  "rust-analyzer.inlayHints.parameterHints.enable": true,
  "rust-analyzer.inlayHints.typeHints.enable": true,
  "rust-analyzer.inlayHints.closingBraceHints.enable": true,
  "rust-analyzer.inlayHints.closureReturnTypeHints.enable": "always",
  "rust-analyzer.inlayHints.maxLength": 120,
}
```

### 8.2 LLM-Friendly Rust Coding Patterns

LLMs understand Rust better when code is explicitly typed and documented:

```rust
// ❌ Inference-heavy (LLMs struggle to reason)
let items = collection.iter().filter(|x| x.is_valid()).collect();

// ✅ Explicit types (LLMs can precisely reason)
let items: Vec<&Item> = collection
    .iter()
    .filter(|x: &&Item| x.is_valid())
    .collect();
```

**Guidelines for LLM-friendly Rust:**
1. **Prefer explicit types** on public function signatures and `let` bindings where inference crosses boundaries
2. **Document invariants** in doc comments (`///`) — this gets fed to the LLM as context
3. **Use `#[must_use]`** on functions returning values — prevents LLM from generating discarded-result code
4. **Avoid deeply nested closures** — break into named functions for LLM context windows
5. **Use named structs over tuples** for function returns — field names give LLMs semantic clues
6. **Use `expect()` over `unwrap()`** with meaningful messages — the LLM can read the expectation string

### 8.3 AI-Friendly Crate Choices

| Crate Type | Recommended | Why for LLMs |
|---|---|---|
| Error handling | `thiserror` + `anyhow` | Well-known patterns; extensive LLM training data |
| Async runtime | `tokio` | Most widely used; highest-quality LLM async code generation |
| Serialization | `serde` + `serde_json` | Ubiquitous; maximize LLM training coverage |
| Logging | `tracing` (not `log`) | Structured; LLMs generate spans/instrumentation correctly |
| HTTP client | `reqwest` | Dominant market share; best LLM generation quality |
| CLI args | `clap` (derive mode) | Derive macros parseable by LLMs; less boilerplate |

### 8.4 Codebuff-Specific Optimization

Codebuff reads the entire file context. Optimize project layout for LLM context windows:

- **Keep files under 500 lines** where possible — LLM context for a file is sharper
- **Use meaningful module names** — `src/features/hero.rs` tells the LLM more than `src/hero_component.rs`
- **Include `// --- section ---` markers** to help LLMs navigate long files
- **Explicit re-exports** in `mod.rs` / `lib.rs` give LLMs a quick module map
- **Use inline type annotations** on closures and complex iterators — prevents LLM type inference errors

---

## 9. MCP (Model Context Protocol) Tools & Servers

MCP standardizes how AI applications connect with external tools. Below are MCP tools relevant to Rust development.

### 9.1 What is MCP?

MCP is an open protocol that standardizes how AI applications (Codebuff, Claude Desktop, Cursor) connect with external data sources and tools. An **MCP server** exposes resources and tools; an **MCP client** (the AI assistant) uses them.

### 9.2 Recommended MCP Servers for This Project

| MCP Server | Purpose | Installation |
|---|---|---|
| **GitHub MCP** | Read/write GitHub repos, issues, PRs directly from the AI | `npx @modelcontextprotocol/server-github` |
| **Filesystem MCP** | Grant AI controlled access to specific project directories | `npx @modelcontextprotocol/server-filesystem` (built into Codebuff) |
| **Rust Analyzer MCP** *(community)* | Exposes rust-analyzer diagnostics/completions/hover to the AI | Via LSP-MCP bridge |

### 9.3 MCP Configuration (`.cursor/mcp.json` or similar)

```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      }
    },
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/home/pyo/workspace/pyoway"]
    }
  }
}
```

### 9.4 MCP Inspector

Available via: `npx @modelcontextprotocol/inspector`

This tool allows inspecting and debugging MCP server connections during development of custom MCP servers.

---

## 10. Steering Hints & Configuration

"Steering hints" are compiler directives, lint rules, and configuration that guide both human developers and AI assistants toward idiomatic, safe, and consistent Rust code.

### 10.1 rust-toolchain.toml

Pins the Rust version for reproducibility across environments and CI:

```toml
# rust-toolchain.toml
[toolchain]
channel = "1.95.0"
components = ["rustc", "cargo", "clippy", "rustfmt", "rust-analyzer", "rust-src", "rust-docs"]
targets = ["wasm32-unknown-unknown"]
profile = "minimal"
```

### 10.2 Cargo Workspace Lint Configuration

```toml
# In root Cargo.toml — workspace-level lint config (Rust 2024 edition)
[lints.workspace]
unsafe_code = "deny"
clippy::pedantic = "warn"
clippy::nursery = "warn"
clippy::cargo = "warn"
clippy::unwrap_used = "warn"
clippy::expect_used = "warn"
clippy::missing_docs_in_private_items = "deny"
clippy::missing_errors_doc = "deny"
clippy::must_use_candidate = "warn"
rust_2024_compatibility = "deny"
missing_docs = "deny"
```

### 10.3 CI Steering (Lint Gate Patterns)

```yaml
# .github/workflows/ci.yml — key steps
      - run: cargo fmt --check --all
      - run: cargo clippy --workspace --all-targets -- -D warnings
      - run: cargo machete
      - run: cargo audit
      - run: cargo nextest run
```

### 10.4 Editor & IDE Configuration

```jsonc
// .vscode/extensions.json — Recommended extensions
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tamasfe.even-better-toml",
    "bradlc.magic-tailwindcss",
    "vadimcn.vscode-lldb",
    "fill-labs.dependi",
    "github.vscode-github-actions",
    "streetsidesoftware.code-spell-checker"
  ]
}
```

```jsonc
// .vscode/settings.json — Full workspace settings
{
  // Rust-specific
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.check.extraArgs": ["--", "-D", "warnings"],
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.procMacro.enable": true,
  "rust-analyzer.inlayHints.typeHints.enable": true,
  "rust-analyzer.inlayHints.parameterHints.enable": true,
  "rust-analyzer.inlayHints.chainingHints.enable": true,
  "rust-analyzer.inlayHints.closingBraceHints.enable": true,
  "rust-analyzer.hoverActions.enable": true,

  // TOML
  "evenBetterToml.formatter.alignEntries": true,
  "evenBetterToml.formatter.reorderKeys": true,

  // General
  "files.trimTrailingWhitespace": true,
  "files.insertFinalNewline": true,
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "rust-lang.rust-analyzer",
  "editor.rulers": [100],

  // Spell check
  "cSpell.words": ["Pyoway", "Actix", "Leptos", "Tailwind", "mdBook", "Tokio", "Clap", "Serde"]
}
```

---

## 11. Documentation References

### 11.1 Official Rust References

| Resource | URL | Purpose |
|---|---|---|
| **Rust Book** | https://doc.rust-lang.org/book/ | Core Rust language reference |
| **Rust Edition Guide** | https://doc.rust-lang.org/nightly/edition-guide/rust-2024/ | Rust 2024 edition migration guide |
| **Rust API Guidelines** | https://rust-lang.github.io/api-guidelines/ | Standard API design patterns |
| **Rust Style Guide** | https://doc.rust-lang.org/nightly/style-guide/ | Code formatting conventions |
| **Rust Reference** | https://doc.rust-lang.org/reference/ | Language specification |
| **Rust by Example** | https://doc.rust-lang.org/rust-by-example/ | Practical code examples |
| **Cargo Book** | https://doc.rust-lang.org/cargo/ | Cargo build system & package management |
| **Rustdoc Book** | https://doc.rust-lang.org/rustdoc/ | Documentation generation |
| **Clippy Book** | https://doc.rust-lang.org/clippy/ | Linting rules reference |

### 11.2 Framework & Library Docs

| Resource | URL | Purpose |
|---|---|---|
| **Actix Web Docs** | https://actix.rs/docs/ | Actix-web framework reference |
| **actix-web API** | https://docs.rs/actix-web/latest/actix_web/ | API documentation |
| **Leptos Book** | https://book.leptos.dev/ | Leptos WASM framework guide |
| **Leptos API** | https://docs.rs/leptos/latest/leptos/ | Leptos API reference |
| **Tailwind CSS Docs** | https://tailwindcss.com/docs | Utility-first CSS reference |
| **Trunk Guide** | https://trunkrs.dev/ | WASM bundler docs |
| **mdBook Docs** | https://rust-lang.github.io/mdBook/ | Markdown book generation |
| **Tokio Tutorial** | https://tokio.rs/tokio/tutorial | Async runtime guide |
| **Serde Book** | https://serde.rs/ | Serialization framework |

### 11.3 Tooling & Best Practices

| Resource | URL | Purpose |
|---|---|---|
| **Rustup Docs** | https://rustup.rs/ | Toolchain management |
| **rust-analyzer Manual** | https://rust-analyzer.github.io/manual.html | IDE integration reference |
| **cargo-deny Docs** | https://embarkstudios.github.io/cargo-deny/ | Dependency auditing |
| **Rust CI Guide** | https://github.com/actions-rust-lang/setup-rust-toolchain | GitHub Actions for Rust |
| **WASM Book** | https://rustwasm.github.io/docs/book/ | Rust + WASM guide |
| **WASM Pack Docs** | https://rustwasm.github.io/wasm-pack/book/ | WASM build tool |
| **Rust Design Patterns** | https://rust-unofficial.github.io/patterns/ | Idiomatic Rust patterns |
| **Rust Cookbook** | https://rust-lang-nursery.github.io/rust-cookbook/ | Common task recipes |
| **Awesome Rust** | https://github.com/rust-unofficial/awesome-rust | Curated Rust resources list |
| **Rust Security Advisory DB** | https://rustsec.org/ | Security vulnerability database |

### 11.4 MCP & AI-Assisted Development

| Resource | URL | Purpose |
|---|---|---|
| **MCP Specification** | https://modelcontextprotocol.io/ | MCP protocol specification |
| **MCP Servers (GitHub)** | https://github.com/modelcontextprotocol/servers | Official MCP server implementations |
| **Codebuff Docs** | https://codebuff.com/docs | Codebuff usage and configuration |
| **Claude MCP Docs** | https://docs.anthropic.com/en/docs/agents-and-tools/mcp | Anthropic's MCP documentation |

---

## 12. CI/CD (GitHub Actions)

### 12.1 Workflows

| Workflow | Trigger | Steps |
|---|---|---|
| **CI (check & test)** | push/PR to `main` | `cargo check`, `cargo clippy -- -D warnings`, `cargo nextest run`, `cargo fmt --check`, `cargo machete`, `cargo audit` |
| **Build landing** | tag `v*` or manual | `trunk build --release`, upload `dist/` as artifact |
| **Build docs** | push to `main` | `mdbook build docs/`, upload `docs/book/` as artifact |
| **Deploy docs** | workflow_dispatch | Placeholder — user connects their own hosting |

### 12.2 Quality Gates

- No `unsafe` code allowed (deny in workspace config)
- Clippy pedantic with no warnings
- Rust 2024 edition formatting
- All tests must pass
- No dependency advisories (cargo-audit)
- No unused dependencies (cargo-machete)

---

## 13. Directory Structure (Proposed)

```
pyoway/
├── .cargo/
│   └── config.toml               # Cargo project config (rustflags, profiles)
├── .github/
│   ├── clippy.toml               # Clippy configuration
│   └── workflows/
│       ├── ci.yml                 # Check, clippy, test, fmt, audit
│       ├── build-landing.yml      # Build WASM bundle
│       └── build-docs.yml         # Build mdBook site
├── .vscode/
│   ├── extensions.json            # Recommended extensions
│   └── settings.json              # Workspace settings
├── landing-server/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                # Server entrypoint, config
│       ├── middleware.rs          # CORS, compression, security headers, logging
│       ├── routes.rs              # Route definitions
│       ├── errors.rs              # Custom error types & handlers
│       └── config.rs              # Environment config
├── landing-frontend/
│   ├── Cargo.toml
│   ├── Trunk.toml                 # Trunk configuration
│   ├── src/
│   │   ├── main.rs                # Leptos app entrypoint
│   │   ├── app.rs                 # Root component, router
│   │   ├── components/
│   │   │   ├── hero.rs            # Hero section
│   │   │   ├── features.rs        # Features/highlights cards
│   │   │   ├── docs_link.rs       # Docs link section
│   │   │   ├── footer.rs          # Footer
│   │   │   └── nav.rs             # Navigation bar
│   │   └── styles/
│   │       └── tailwind.css       # Tailwind input with @tailwind directives
│   ├── index.html                 # HTML shell for WASM
│   └── public/                    # Static assets (images, favicon)
├── docs/
│   ├── book.toml                  # mdBook configuration
│   ├── src/
│   │   ├── SUMMARY.md             # Table of contents
│   │   ├── intro.md               # Introduction page
│   │   ├── blog/
│   │   │   ├── post-template.md   # Blog post template
│   │   │   └── hello-world.md     # Welcome/first post
│   │   ├── knowledge-base/
│   │   │   ├── rust/              # Rust-specific KB articles
│   │   │   ├── web-dev/           # Web development articles
│   │   │   └── tools/             # Tools & workflow articles
│   │   └── about.md               # About this site
│   └── theme/
│       └── custom.css             # Custom mdBook theme overrides
├── Cargo.toml                     # Workspace root
├── deny.toml                      # cargo-deny configuration
├── rust-toolchain.toml            # Pin Rust toolchain version
├── .env.example                   # Environment variables template
├── .gitignore
├── README.md
└── pyoway-spec.md                 # This file
```

---

## 14. Setup Procedure (Initial)

```bash
# 1. Create GitHub private repo
gh repo create pyoway --private

# 2. Initialize local repo
cd /home/pyo/workspace/pyoway
git init
git remote add origin git@github.com:pyoclaw/pyoway.git

# 3. Install WASM target & tools
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-pack

# 4. Install Tailwind CSS (standalone CLI)
# Option A: npm
npm init -y && npm install -D tailwindcss @tailwindcss/cli
# Option B: standalone binary
# curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
# chmod +x tailwindcss-linux-x64 && mv tailwindcss-linux-x64 ~/.local/bin/tailwindcss

# 5. Install mdBook
cargo install mdbook

# 6. Install all recommended workspace tools
# (See section 7.8 for full command)

# 7. Create workspace structure
# (see directory structure above)

# 8. Initial commit
git add .
git commit -m "chore: initialize pyoway workspace"
git push -u origin main
```

---

## 15. Future Considerations (Out of Scope for v1)

- [ ] Contact form (Actix-web API endpoint + email integration)
- [ ] Blog RSS/Atom feed
- [ ] Full-text search on the landing page
- [ ] Analytics integration
- [ ] Docker build for landing-server
- [ ] Automate mdBook deployment (GitHub Pages or Netlify)
- [ ] Dark/light theme toggle
- [ ] i18n / multi-language support
- [ ] Custom MCP server for site-specific AI tooling

---

## 16. Questions Answered During Interview

| Question | Answer |
|---|---|
| Repo name | `pyoway` |
| Repo visibility | Private |
| GitHub owner | `pyoclaw` (personal) |
| Workspace location | Current directory (`/home/pyo/workspace/pyoway`) |
| Project purpose | Personal website + blog |
| SSG tool | mdBook |
| Landing + docs architecture | Separate services (different hosts) |
| Rust edition | 2024 |
| Design style | Bold & colorful (Vercel-inspired) |
| Landing page sections | Hero + Features + Docs link |
| Docs hosting | Separate subdomain (user-managed) |
| WASM framework | Leptos (latest) |
| CSS framework | Tailwind CSS v4 |
| Frontend approach | Pure Rust WASM |
| CI/CD | Include GitHub Actions workflows |
| WASM tooling setup | Include in initial setup |
| Docs content depth | Detailed skeleton with categorized sections |
| Hero content | Name + tagline + CTA button |
| Actix-web features | Full-featured: middleware, WebSockets, security headers, error handling |
| OS | Ubuntu 24.04 LTS (Noble) |
| Rust version | 1.95.0 |
| Design inspiration | Vercel (vercel.com) |
| WASM framework choice | Leptos (recommended + accepted) |
| LLM tools | rust-analyzer settings, explicit types, MCP servers |
| Cargo tools | Listed in 28 tools across 6 categories |
| MCP servers | GitHub, Filesystem, Rust Analyzer (community) |
