# Pyoway Rust Conventions

## Toolchain

- **Channel:** `1.95.0` (pinned in `rust-toolchain.toml`)
- **Edition:** `2024`
- **Components:** rustc, cargo, clippy, rustfmt, rust-analyzer, rust-src, rust-docs
- **Targets:** `wasm32-unknown-unknown` (for WASM frontend)

## Lint Configuration

Workspace-level lints in `Cargo.toml`:

```toml
[workspace.lints.rust]
unsafe_code = "deny"                  # No unsafe allowed
missing_docs = "deny"                 # All public items must be documented
rust_2024_compatibility = "deny"

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
cargo = { level = "warn", priority = -1 }
unwrap_used = "warn"
expect_used = "warn"
missing_docs_in_private_items = "deny"
missing_errors_doc = "deny"
must_use_candidate = "warn"
```

Additionally, `.cargo/config.toml` sets `RUSTFLAGS` for all targets:

```toml
rustflags = ["-D", "unsafe_code", "-W", "clippy::pedantic", "-W", "clippy::cargo", "-W", "clippy::nursery"]
```

### Allowed Clippy Exceptions (via `-A` in CI)

- `clippy::multiple_crate_versions` — Common in workspace projects, hard to avoid
- `clippy::must_use_candidate` — Noisy on Leptos component functions

## Code Style

### Documentation

- All public items MUST have doc comments (`///`)
- Private items SHOULD have doc comments (enforced by `missing_docs_in_private_items = "deny"`)
- Functions returning `Result` MUST document error variants (`missing_errors_doc = "deny"`)
- Use `//!` for module-level documentation

### Error Handling

- Use `thiserror` for library error types (see `landing-server/src/error.rs` for the pattern)
- Use `anyhow` is NOT used; prefer custom error enums
- Avoid `unwrap()` and `expect()` — mark with `#[allow(clippy::unwrap_used)]` when unavoidable and add a comment explaining why safe
- Functions returning `Result` should have descriptive error types

### LLM-Friendly Patterns

For better AI agent code generation:

```rust
// ✅ Prefer explicit types on public signatures
pub fn process_items(items: &[Item]) -> Result<Vec<ProcessedItem>, ProcessError>

// ✅ Use expect() over unwrap() with descriptive messages
let config = AppConfig::from_env().expect("Failed to load configuration");

// ✅ Use named structs over tuples for function returns
pub struct ParseResult {
    pub value: i32,
    pub rest: &str,
}

// ✅ Prefer explicit type annotations over inference where it crosses boundaries
let items: Vec<&Item> = collection.iter().filter(|x: &&Item| x.is_valid()).collect();

// ✅ Use #[must_use] on functions returning values
#[must_use]
pub fn compute_score(input: &str) -> Score
```

### Module Organization

- Each component gets its own file in a `components/` directory
- Re-export public items in `mod.rs` / `lib.rs`
- Keep files under 500 lines where possible
- Use `// --- section ---` markers in long files for navigation

## Cargo Profiles

```toml
[profile.dev]
opt-level = 1  # Faster dev builds while maintaining debuggability

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
strip = "symbols"
```

## CI Quality Gates

- `cargo fmt --check --all` — Formatted code
- `cargo clippy --workspace --all-targets -- -D warnings` — Lint-clean
- `cargo check --workspace` — Compiles without errors
- `cargo nextest run` — All tests pass
- `cargo deny check` — No license/advisory issues
- `cargo audit` — No security advisories
- `cargo machete` — No unused dependencies
