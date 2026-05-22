//! Features and highlights cards section.

use leptos::prelude::*;

/// A single feature card.
#[component]
fn FeatureCard(
    /// Emoji or icon character for the card.
    icon: &'static str,
    /// Card heading text.
    title: &'static str,
    /// Card body description.
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="group relative p-8 rounded-2xl bg-[#12121a] border border-white/5 hover:border-blue-500/30 transition-all duration-300 hover:shadow-lg hover:shadow-blue-500/5 hover:-translate-y-1">
            <div class="text-3xl mb-4">{icon}</div>
            <h3 class="text-xl font-semibold mb-3 text-white">{title}</h3>
            <p class="text-slate-400 leading-relaxed text-sm">{description}</p>
            <div class="absolute inset-0 rounded-2xl bg-gradient-to-br from-blue-500/0 via-violet-500/0 to-cyan-500/0 group-hover:from-blue-500/5 group-hover:via-violet-500/5 group-hover:to-cyan-500/5 transition-all duration-300 pointer-events-none" />
        </div>
    }
}

/// Features section with a responsive grid of cards.
#[component]
pub fn Features() -> impl IntoView {
    view! {
        <section id="features" class="py-24 px-6 bg-[#0a0a0f]">
            <div class="max-w-6xl mx-auto">
                <div class="text-center mb-16">
                    <h2 class="text-3xl md:text-4xl font-bold mb-4">
                        Features
                    </h2>
                    <p class="text-slate-400 max-w-lg mx-auto">
                        Built with modern tools for performance and developer experience.
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    <FeatureCard
                        icon="⚡"
                        title="Lightning Fast"
                        description="Compiled to WebAssembly via Leptos for near-native performance in the browser."
                    />
                    <FeatureCard
                        icon="🦀"
                        title="Pure Rust"
                        description="End-to-end Rust — from the Axum web server to the WASM frontend and build tooling."
                    />
                    <FeatureCard
                        icon="📖"
                        title="Knowledge Base"
                        description="A full documentation site built with mdBook, featuring categorized articles and blog posts."
                    />
                    <FeatureCard
                        icon="🎨"
                        title="Bold Design"
                        description="Vercel-inspired dark gradient aesthetic with micro-interactions and smooth animations."
                    />
                    <FeatureCard
                        icon="🔒"
                        title="Secure by Default"
                        description="Security headers, CORS policies, and unsafe code denied at the compiler level."
                    />
                    <FeatureCard
                        icon="🚀"
                        title="CI/CD Ready"
                        description="Automated quality gates, dependency auditing, and build pipelines with GitHub Actions."
                    />
                </div>
            </div>
        </section>
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;

    #[test]
    fn renders_section_heading() {
        let html = view! { <Features /> }.to_html();
        assert!(html.contains("Features"));
    }

    #[test]
    fn renders_section_subtitle() {
        let html = view! { <Features /> }.to_html();
        assert!(html.contains("Built with modern tools"));
    }

    #[test]
    fn renders_all_feature_cards() {
        let html = view! { <Features /> }.to_html();
        // All six card titles
        assert!(html.contains("Lightning Fast"));
        assert!(html.contains("Pure Rust"));
        assert!(html.contains("Knowledge Base"));
        assert!(html.contains("Bold Design"));
        assert!(html.contains("Secure by Default"));
        assert!(html.contains("CI/CD Ready"));
    }

    #[test]
    fn renders_feature_icons() {
        let html = view! { <Features /> }.to_html();
        assert!(html.contains("⚡"));
        assert!(html.contains("🦀"));
        assert!(html.contains("📖"));
        assert!(html.contains("🎨"));
        assert!(html.contains("🔒"));
        assert!(html.contains("🚀"));
    }

    #[test]
    fn renders_feature_descriptions() {
        let html = view! { <Features /> }.to_html();
        assert!(html.contains("Compiled to WebAssembly"));
        assert!(html.contains("End-to-end Rust"));
        assert!(html.contains("full documentation site"));
    }

    #[test]
    fn has_section_id() {
        let html = view! { <Features /> }.to_html();
        assert!(html.contains("id=\"features\""));
    }
}
