# Pyoway Dependency Management

## Workspace Dependencies

The workspace has two crate members:
- `landing-server` (Axum HTTP server)
- `landing-frontend` (Leptos WASM frontend)

Each crate inherits workspace lints via `[lints] workspace = true` in their `Cargo.toml`.

## Adding Dependencies

### New dependency checklist

1. Add the dependency to the correct crate's `Cargo.toml`
2. Run `cargo check` to verify it resolves
3. Run `cargo machete` to check for unused deps
4. Run `cargo deny check` to verify license compliance
5. Run `cargo audit` to check for advisories

### Version pinning

- Use semantic versioning ranges (e.g., `"0.8"` for Axum, `"1"` for tokio)
- Pin exact versions only when necessary (e.g., `wasm-bindgen` must stay synced with Trunk's pinned version)
- Avoid `*` version constraints

### Cargo-deny configuration (`deny.toml`)

```toml
[advisories]
unmaintained = "warn"
yanked = "deny"

[licenses]
allow = ["MIT", "Apache-2.0", "BSD-3-Clause", "BSL-1.0", "Unicode-3.0"]

[bans]
multiple-versions = "deny"
# Skip entries documented with reasons
```

When adding a new dependency:
- Check `deny.toml` licenses — if the dep uses a new license, add it to the allowlist
- If the dep causes duplicate versions that can't be resolved, add a skip entry with a clear reason

## Dependabot

Dependabot checks for updates weekly (Mondays). Minor/patch updates are grouped into single PRs. wasm-bindgen is excluded from auto-updates.

### Update flow for Dependabot PRs

```
Dependabot opens PR
  → Labeler adds `dependencies` label
  → CI runs (all 4 checks)
  → Auto-merge workflow approves + enables auto-merge
  → CI passes → PR auto-merges
```

## .cargo/config.toml

```toml
[profile.dev]
opt-level = 1  # Balance speed and debuggability

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = "symbols"
```

The `rustflags` in this file set workspace-wide Clippy lints.

## Key Rules

1. **No `unsafe` dependencies** — `unsafe_code = "deny"` prevents any crate from using unsafe
2. **No yanked crates** — `[advisories] yanked = "deny"` in deny.toml
3. **No duplicate crate versions** — `[bans] multiple-versions = "deny"` with documented skip entries
4. **Licenses must be in allowlist** — MIT, Apache-2.0, BSD-3-Clause, BSL-1.0, Unicode-3.0
5. **wasm-bindgen is pinned** — Newer versions may break Trunk compatibility
