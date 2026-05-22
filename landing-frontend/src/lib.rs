//! Root component and application shell for the Pyoway landing page.

use leptos::prelude::*;
use leptos_meta::{Html, Link, Meta, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

pub mod components;

/// Root application component.
#[allow(clippy::must_use_candidate)]
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:class="scroll-smooth" />
        <Title text="Pyoway" />
        <Meta name="description" content="Pyoway — Personal website and knowledge base built with Rust" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Link rel="preconnect" href="https://fonts.googleapis.com" />
        <Link rel="preconnect" href="https://fonts.gstatic.com" attr:crossorigin="" />
        <Link
            rel="stylesheet"
            href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=JetBrains+Mono:wght@400&display=swap"
        />

        <Router>
            <Routes fallback=|| view! { <div>"Page not found"</div> }>
                <Route path=StaticSegment("") view=HomePage />
            </Routes>
        </Router>
    }
}

/// Home page — single-page scroll with all sections.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-[#0a0a0f] text-[#f8fafc] font-['Inter',system-ui,sans-serif]">
            <components::Nav />
            <main>
                <components::Hero />
                <components::Features />
                <components::DocsLink />
            </main>
            <components::Footer />
        </div>
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;

    #[test]
    fn home_page_renders_nav() {
        let html = view! { <HomePage /> }.to_html();
        assert!(html.contains("pyoway"));
        assert!(html.contains("Features"));
    }

    #[test]
    fn home_page_renders_hero() {
        let html = view! { <HomePage /> }.to_html();
        assert!(html.contains("View My Work"));
        assert!(html.contains("built with Rust"));
    }

    #[test]
    fn home_page_renders_features_section() {
        let html = view! { <HomePage /> }.to_html();
        assert!(html.contains("Lightning Fast"));
        assert!(html.contains("Pure Rust"));
        assert!(html.contains("CI/CD Ready"));
    }

    #[test]
    fn home_page_renders_docs_link() {
        let html = view! { <HomePage /> }.to_html();
        assert!(html.contains("Dive Into the"));
        assert!(html.contains("Visit the Knowledge Base"));
    }

    #[test]
    fn home_page_renders_footer() {
        let html = view! { <HomePage /> }.to_html();
        assert!(html.contains("GitHub"));
        assert!(html.contains("All rights reserved"));
    }
}
