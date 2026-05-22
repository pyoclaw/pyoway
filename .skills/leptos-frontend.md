# Pyoway Leptos Frontend Conventions

## Architecture

The frontend is a Leptos 0.7 WASM application compiled via Trunk and served by the Axum backend.

### Build Pipeline

```
Tailwind CSS (npm) → .css
Leptos code (Rust) → Trunk → WASM bundle (.wasm + .js glue)
                         → dist/ directory
```

The server serves `dist/` as static files via the `handle_frontend` fallback route.

**Build command:** `cd landing-frontend && trunk build --release`

### Dev server

```bash
cd landing-frontend && trunk serve
```

Trunk's dev server handles both the WASM rebuild and Tailwind CSS compilation on file changes.

## Component Patterns

### Component Signature

Every component uses Leptos's `#[component]` macro with explicit `impl IntoView`:

```rust
/// Navigation bar component.
#[component]
pub fn Nav() -> impl IntoView {
    view! { ... }
}
```

Props are passed as function parameters:

```rust
#[component]
fn FeatureCard(
    /// Emoji or icon character for the card.
    icon: &'static str,
    /// Card heading text.
    title: &'static str,
    /// Card body description.
    description: &'static str,
) -> impl IntoView {
    view! { ... }
}
```

All component params MUST have doc comments.

### Module Registration

Every component is registered in `components/mod.rs`:

```rust
mod nav;
pub use nav::Nav;

mod hero;
pub use hero::Hero;
// ... etc.
```

### Component Pattern Rules

1. **One component per file** — named after the component
2. **Tests in the same file** — `#[cfg(test)] mod tests { ... }` at the bottom
3. **CSS classes as string literals** — Tailwind utility classes in the `view!` macro
4. **SVGs inline** — Icons are embedded as `<svg>` elements, not external assets

### View Layout

```rust
view! {
    <section class="...">
        <div class="...">
            <h2>"Heading"</h2>
            <p>"Description"</p>
        </div>
    </section>
}
```

## Styling Conventions

- **Tailwind CSS v4** — All styling is done via Tailwind utility classes
- **No custom CSS** — The only CSS file is `styles/tailwind.css` with `@tailwind` directives
- **Color palette** (defined in `tailwind.css`):
  - Background: `#0a0a0f` (deep midnight)
  - Surface: `#12121a` (dark card)
  - Primary: `#3b82f6` (electric blue)
  - Secondary: `#8b5cf6` (violet)
  - Tertiary: `#06b6d4` (cyan)
  - Text: `#f8fafc` (white)
  - Muted: `#94a3b8` (slate)
- **Font:** Inter (headings + body), JetBrains Mono (code)

## Testing

- Tests use `leptos::tachys::view::RenderHtml` to render components to HTML strings
- Assertions use `assert!(html.contains("..."))` to check rendered output
- Tests are compiled and run as native (not WASM) via `cargo test -p landing-frontend`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;

    #[test]
    fn renders_heading() {
        let html = view! { <Hero /> }.to_html();
        assert!(html.contains("Pyoway"));
    }
}
```

## Key Do's and Don'ts

### DO
- Use `view! { ... }` macro for all HTML
- Use inline SVG for icons
- Use Tailwind `animate-*` classes for animations (e.g., `animate-pulse`, `animate-bounce`)
- Add `#[allow(clippy::must_use_candidate)]` on component functions when clippy complains
- Add `#[allow(clippy::unwrap_used)]` on test modules only
- Use `transition-all duration-300` for hover effects

### DON'T
- Don't import external JS libraries
- Don't use `dangerous_inner_html` unless absolutely necessary
- Don't leave unnecessary `unwrap()` in non-test code
- Don't use raw `<style>` tags — use Tailwind classes instead
- Don't put component tests in separate test files — keep them in the component file
