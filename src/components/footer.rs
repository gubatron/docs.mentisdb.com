use leptos::prelude::*;

/// Site-wide footer for docs.mentisdb.com.
///
/// Contains navigation links mirroring the navbar, the GitHub repository
/// link, a back-link to the main site, and the copyright / license line.
#[component]
pub fn DocsFooter() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container">
                <div class="footer-links">
                    <a href="/"          class="footer-link">"Home"</a>
                    <a href="/user"      class="footer-link">"User Docs"</a>
                    <a href="/agent"     class="footer-link">"Agent Docs"</a>
                    <a href="/developer" class="footer-link">"Developer Docs"</a>
                    <a
                        href="https://github.com/cloudllm-ai/mentisdb"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="footer-link"
                    >
                        "GitHub"
                    </a>
                    <a href="https://mentisdb.com" class="footer-link">
                        "mentisdb.com"
                    </a>
                </div>
                <p class="footer-copy">"© 2026 MentisDB · MIT License"</p>
            </div>
        </footer>
    }
}
