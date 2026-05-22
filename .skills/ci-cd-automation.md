# Pyoway CI/CD & Automation Conventions

## GitHub Actions Workflows

### CI (`.github/workflows/ci.yml`)

**Triggers:** push/PR to `main`

**Jobs:**
| Job Name | What it does |
|---|---|
| `Check & Lint` | `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo check` |
| `Test Server` | `cargo test -p landing-server` |
| `Test Frontend` | `cargo test -p landing-frontend` |
| `Security Audit` | `cargo deny check`, `cargo audit`, `cargo machete` |

**Key patterns:**
- Uses `RUST_TOOLCHAIN: "1.95.0"` env var (centralized)
- Uses `Swatinem/rust-cache@v2` in all jobs for fast rebuilds
- Each job installs its own Rust toolchain via `actions-rust-lang/setup-rust-toolchain@v1`

### Build & Deploy Docs (`.github/workflows/build-docs.yml`)

**Triggers:** push to `main` with `docs/**` changes, PR with `docs/**` changes, manual

**Jobs:**
1. `build` — Install mdBook via `cargo install mdbook`, build with `mdbook build docs`, run integration tests, upload `docs/book/` as Pages artifact
2. `deploy` — Deploys to GitHub Pages using `actions/deploy-pages@v4` (only on push/manual, not PR)

**Concurrency:** `pages-${{ github.ref }}` with `cancel-in-progress: true`

### Build Landing Page (`.github/workflows/build-landing.yml`)

**Triggers:** version tags (`v*`), manual

**Steps:**
1. Install Rust toolchain with WASM target
2. Install Trunk (`cargo install trunk`)
3. Setup Node.js + npm cache
4. Install Tailwind CSS (`npm ci` in `landing-frontend/`)
5. Build with `trunk build --release`
6. Upload `dist/` as artifact

### Dependabot Auto-Merge (`.github/workflows/dependabot-auto-merge.yml`)

**Triggers:** PR opened/reopened/synchronized

**Logic:**
1. Only runs for PRs from `dependabot[bot]`
2. Checks the update type via `dependabot/fetch-metadata@v1`
3. Only minor/patch updates are auto-approved and auto-merged
4. Uses `gh pr review --approve` + `gh pr merge --auto --squash`

## Branch Protection

Configured via `.github/scripts/setup-branch-protection.sh` (API-based).

**Required checks:**
- Check & Lint
- Test Server
- Test Frontend
- Security Audit

**Rules:**
- 1 required review approval (stale reviews dismissed)
- Enforce for admins
- Strict mode (branches must be up to date)
- Dependabot bypasses review requirement (configured in GitHub UI)

## Dependabot Configuration

**Config:** `.github/dependabot.yml`

- **Cargo updates:** Weekly (Monday), minor/patch grouped together, wasm-bindgen ignored (must stay synced with Trunk)
- **GitHub Actions updates:** Weekly, minor/patch grouped
- Labels: `dependencies` + `rust` / `ci`
- PR limit: 3 open at a time

## PR Labeling

**Config:** `.github/labeler.yml`, `.github/workflows/labeler.yml`

Auto-labels PRs based on changed paths using `actions/labeler@v6`:

| Label | When |
|---|---|
| `docs` | `docs/**` |
| `frontend` | `landing-frontend/**` |
| `server` | `landing-server/**` |
| `ci/cd` | `.github/**` |
| `infrastructure` | Dockerfile, docker-compose.yml, config files |
| `documentation` | README.md, LICENSE, etc. |
| `dependencies` | Any Cargo.toml / Cargo.lock |

## Local Development (Justfile)

```bash
just check      # cargo check --workspace
just clippy     # cargo clippy with -D warnings
just fmt        # cargo fmt --check
just test       # cargo nextest run
just audit      # cargo deny check + cargo audit
just machete    # cargo machete
just ci         # Runs all CI checks locally
```

## Key Patterns to Follow

When adding new workflows or modifying existing ones:

1. **Use `RUST_TOOLCHAIN` env var** — Don't hardcode the version
2. **Add Rust cache** — `Swatinem/rust-cache@v2` after toolchain install
3. **Scope permissions** — Use `permissions:` at the job level, not workflow level, for least privilege
4. **Path filters** — Use `paths:` on push triggers to avoid unnecessary runs
5. **Concurrency groups** — Cancel in-progress runs for the same branch/ref
6. **Check names must match** — Branch protection requires exact job name match (e.g., "Check & Lint", "Test Server", "Test Frontend", "Security Audit")
