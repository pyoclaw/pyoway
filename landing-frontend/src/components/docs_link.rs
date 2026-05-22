//! Documentation link / knowledge base preview section.

use leptos::prelude::*;

/// Prominent section linking to the mdBook documentation site.
#[component]
pub fn DocsLink() -> impl IntoView {
    view! {
        <section class="py-24 px-6 bg-[#12121a]">
            <div class="max-w-6xl mx-auto text-center">
                <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-blue-500/10 border border-blue-500/20 text-blue-400 text-sm mb-8">
                    <span class="w-2 h-2 rounded-full bg-blue-400 animate-pulse" />
                    "Knowledge Base"
                </div>

                <h2 class="text-3xl md:text-4xl font-bold mb-4">
                    Dive Into the{" "}
                    <span class="bg-gradient-to-r from-blue-400 to-violet-400 bg-clip-text text-transparent">
                        Knowledge Base
                    </span>
                </h2>

                <p class="text-slate-400 text-lg mb-10 max-w-xl mx-auto leading-relaxed">
                    Thoughts, tutorials, and deep dives on Rust, web development, and the tools
                    that power Pyoway.
                </p>

                <a
                    href="https://docs.pyoway.dev"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-3 px-8 py-4 bg-gradient-to-r from-blue-600 to-violet-600 rounded-full text-white font-medium hover:from-blue-500 hover:to-violet-500 transition-all duration-300 hover:shadow-lg hover:shadow-blue-500/25"
                >
                    "Visit the Knowledge Base"
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 5l7 7m0 0l-7 7m7-7H3" />
                    </svg>
                </a>
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;

    #[test]
    fn renders_badge() {
        let html = view! { <DocsLink /> }.to_html();
        assert!(html.contains("Knowledge Base"));
    }

    #[test]
    fn renders_heading() {
        let html = view! { <DocsLink /> }.to_html();
        assert!(html.contains("Dive Into the"));
        assert!(html.contains("Knowledge Base"));
    }

    #[test]
    fn renders_description() {
        let html = view! { <DocsLink /> }.to_html();
        assert!(html.contains("Thoughts, tutorials, and deep dives"));
    }

    #[test]
    fn renders_cta_button() {
        let html = view! { <DocsLink /> }.to_html();
        assert!(html.contains("Visit the Knowledge Base"));
        assert!(html.contains("href=\"https://docs.pyoway.dev\""));
    }

    #[test]
    fn opens_in_new_tab() {
        let html = view! { <DocsLink /> }.to_html();
        assert!(html.contains("target=\"_blank\""));
        assert!(html.contains("rel=\"noopener noreferrer\""));
    }
}
