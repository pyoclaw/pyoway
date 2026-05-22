//! Navigation bar component.

use leptos::prelude::*;

/// Fixed top navigation bar with transparency on scroll.
#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 right-0 z-50 bg-transparent backdrop-blur-sm border-b border-white/5">
            <div class="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
                <a href="/" class="text-lg font-bold tracking-tight hover:text-blue-400 transition-colors duration-300">
                    pyoway
                </a>
                <div class="flex items-center gap-6 text-sm text-slate-400">
                    <a href="#features" class="hover:text-white transition-colors duration-200">
                        Features
                    </a>
                    <a href="https://docs.pyoway.dev" target="_blank" rel="noopener noreferrer" class="hover:text-white transition-colors duration-200">
                        Docs
                    </a>
                    <a href="https://github.com/pyoclaw" target="_blank" rel="noopener noreferrer" class="hover:text-white transition-colors duration-200">
                        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z" />
                        </svg>
                    </a>
                </div>
            </div>
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;

    #[test]
    fn renders_brand_link() {
        let html = view! { <Nav /> }.to_html();
        assert!(html.contains("pyoway"));
        assert!(html.contains("href=\"/\""));
    }

    #[test]
    fn renders_nav_links() {
        let html = view! { <Nav /> }.to_html();
        assert!(html.contains("Features"));
        assert!(html.contains("Docs"));
    }

    #[test]
    fn renders_github_icon() {
        let html = view! { <Nav /> }.to_html();
        // GitHub SVG icon path data
        assert!(html.contains("M12 0C5.37 0"));
    }

    #[test]
    fn has_nav_element() {
        let html = view! { <Nav /> }.to_html();
        assert!(html.contains("<nav"));
        assert!(html.contains("</nav>"));
    }
}
