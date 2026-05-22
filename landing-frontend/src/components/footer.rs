//! Footer component.

use leptos::prelude::*;

/// Site footer with links and credits.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="py-12 px-6 bg-[#0a0a0f] border-t border-white/5">
            <div class="max-w-6xl mx-auto">
                <div class="flex flex-col md:flex-row items-center justify-between gap-6 mb-8">
                    <div class="flex items-center gap-6 text-sm text-slate-400">
                        <a href="https://github.com/pyoclaw" target="_blank" rel="noopener noreferrer" class="hover:text-white transition-colors duration-200">
                            GitHub
                        </a>
                        <a href="https://twitter.com/pyoclaw" target="_blank" rel="noopener noreferrer" class="hover:text-white transition-colors duration-200">
                            Twitter / X
                        </a>
                        <a href="mailto:hello@pyoway.dev" class="hover:text-white transition-colors duration-200">
                            Email
                        </a>
                        <a href="https://docs.pyoway.dev" target="_blank" rel="noopener noreferrer" class="hover:text-white transition-colors duration-200">
                            Docs
                        </a>
                    </div>
                </div>

                <div class="text-center text-sm text-slate-500 space-y-2">
                    <p>&copy; 2026 Pyoway. All rights reserved.</p>
                    <p class="flex items-center justify-center gap-1">
                        "Built with "
                        <span class="text-xl">"🦀"</span>
                        " Rust"
                    </p>
                </div>
            </div>
        </footer>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::tachys::view::RenderHtml;

    #[test]
    fn renders_social_links() {
        let html = view! { <Footer /> }.to_html();
        assert!(html.contains("GitHub"));
        assert!(html.contains("Twitter"));
        assert!(html.contains("Email"));
        assert!(html.contains("Docs"));
    }

    #[test]
    fn renders_copyright() {
        let html = view! { <Footer /> }.to_html();
        assert!(html.contains("2026"));
        assert!(html.contains("All rights reserved"));
    }

    #[test]
    fn renders_rust_badge() {
        let html = view! { <Footer /> }.to_html();
        assert!(html.contains("Built with"));
        assert!(html.contains("Rust"));
    }

    #[test]
    fn has_footer_element() {
        let html = view! { <Footer /> }.to_html();
        assert!(html.contains("<footer"));
        assert!(html.contains("</footer>"));
    }

    #[test]
    fn links_open_in_new_tab() {
        let html = view! { <Footer /> }.to_html();
        assert!(html.contains("target=\"_blank\""));
        assert!(html.contains("rel=\"noopener noreferrer\""));
    }
}
