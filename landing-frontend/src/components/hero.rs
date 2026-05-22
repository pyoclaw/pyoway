//! Hero section component.

use leptos::prelude::*;

/// Full-viewport hero with gradient background and animated particles.
#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative min-h-screen flex items-center justify-center overflow-hidden bg-gradient-to-br from-blue-600 via-violet-600 to-cyan-500">
            {/* Animated gradient orbs */}
            <div class="absolute inset-0 overflow-hidden">
                <div class="absolute -top-40 -left-40 w-96 h-96 bg-blue-400/20 rounded-full blur-3xl animate-pulse" />
                <div class="absolute -bottom-40 -right-40 w-96 h-96 bg-violet-400/20 rounded-full blur-3xl animate-pulse delay-1000" />
                <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-72 h-72 bg-cyan-400/10 rounded-full blur-3xl animate-pulse delay-500" />
            </div>

            {/* Content */}
            <div class="relative z-10 text-center px-6 max-w-3xl">
                <h1 class="text-5xl md:text-7xl font-bold tracking-tight mb-6 bg-clip-text text-transparent bg-gradient-to-r from-white via-blue-100 to-violet-100">
                    Pyoway
                </h1>
                <p class="text-lg md:text-xl text-white/80 mb-10 max-w-xl mx-auto leading-relaxed">
                    Personal website and knowledge base -- built with Rust, powered by WASM.
                </p>
                <a
                    href="#features"
                    class="inline-flex items-center gap-2 px-8 py-4 bg-white/10 backdrop-blur-sm border border-white/20 rounded-full text-white font-medium hover:bg-white/20 transition-all duration-300 hover:shadow-lg hover:shadow-white/10"
                >
                    "View My Work"
                    <svg class="w-4 h-4 animate-bounce" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
                    </svg>
                </a>
            </div>

            {/* Scroll indicator */}
            <div class="absolute bottom-10 left-1/2 -translate-x-1/2 flex flex-col items-center gap-2 text-white/50 text-sm">
                <span>Scroll</span>
                <div class="w-6 h-10 border-2 border-white/30 rounded-full flex justify-center">
                    <div class="w-1 h-3 bg-white/60 rounded-full mt-2 animate-bounce" />
                </div>
            </div>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;

    #[test]
    fn renders_heading() {
        let html = view! { <Hero /> }.to_html();
        assert!(html.contains("Pyoway"));
    }

    #[test]
    fn renders_description() {
        let html = view! { <Hero /> }.to_html();
        assert!(html.contains("built with Rust"));
        assert!(html.contains("WASM"));
    }

    #[test]
    fn renders_cta_button() {
        let html = view! { <Hero /> }.to_html();
        assert!(html.contains("View My Work"));
        assert!(html.contains("#features"));
    }

    #[test]
    fn renders_scroll_indicator() {
        let html = view! { <Hero /> }.to_html();
        assert!(html.contains("Scroll"));
    }

    #[test]
    fn has_section_element() {
        let html = view! { <Hero /> }.to_html();
        assert!(html.contains("<section"));
        assert!(html.contains("</section>"));
    }
}
