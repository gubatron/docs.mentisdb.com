use leptos::prelude::*;

/// Top navigation bar for docs.mentisdb.com.
///
/// Renders the MentisDB wordmark, a visual separator, the "Docs" label,
/// section links for each audience, a GitHub link, and a back-link to
/// the main marketing site. Also renders a dark/light theme toggle.
#[component]
pub fn DocsNavBar() -> impl IntoView {
    let is_dark = use_context::<RwSignal<bool>>().expect("theme signal not provided via context");

    view! {
        <nav class="navbar">
            <div class="container">
                <div class="navbar-left">
                    <a href="/" class="navbar-logo">
                        <img
                            src=move || if is_dark.get() { "/logo.svg" } else { "/logo-dark.svg" }
                            class="logo-mark"
                            width="40"
                            height="26"
                            alt="MentisDB logo"
                            aria-hidden="true"
                        />
                        "Mentis"<span class="accent">"DB"</span>
                    </a>
                    <span class="navbar-sep">"|"</span>
                    <span class="navbar-section">"Docs"</span>
                </div>
                <div class="navbar-links">
                    <a href="/user"      class="navbar-link">"For Users"</a>
                    <a href="/agent"     class="navbar-link">"For Agents"</a>
                    <a href="/developer" class="navbar-link">"For Developers"</a>
                    <a
                        href="https://github.com/cloudllm-ai/mentisdb"
                        class="navbar-link"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "GitHub"
                    </a>
                    <a href="https://mentisdb.com" class="navbar-cta">
                        "← mentisdb.com"
                    </a>
                    <button
                        class="theme-toggle"
                        aria-label="Toggle light/dark theme"
                        on:click=move |_| is_dark.update(|d| *d = !*d)
                    >
                        {move || if is_dark.get() { "☀️" } else { "🌙" }}
                    </button>
                </div>
            </div>
        </nav>
    }
}
