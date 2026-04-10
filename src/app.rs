use crate::components::{
    agent_docs::AgentDocs, developer_docs::DeveloperDocs, footer::DocsFooter, home::DocsHome,
    navbar::DocsNavBar, user_docs::UserDocs,
};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                // Anti-FOUC: apply saved theme before first paint
                <script>
                    "(function(){
                        var t = localStorage.getItem('theme') ||
                            (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
                        document.documentElement.setAttribute('data-theme', t);
                    })()"
                </script>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let is_dark = RwSignal::new(true);
    provide_context(is_dark);

    // Read saved preference from localStorage on mount (client-only, runs once)
    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        {
            let saved = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
                .and_then(|s| s.get_item("theme").ok().flatten());

            let dark = match saved.as_deref() {
                Some("light") => false,
                Some("dark") => true,
                _ => web_sys::window()
                    .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
                    .map(|mq| mq.matches())
                    .unwrap_or(true),
            };
            is_dark.set(dark);
        }
    });

    // Reactively apply theme attribute to <html> and persist to localStorage
    Effect::new(move |_| {
        let _dark = is_dark.get();
        #[cfg(not(feature = "ssr"))]
        {
            let theme = if _dark { "dark" } else { "light" };
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(html) = doc.document_element() {
                    let _ = html.set_attribute("data-theme", theme);
                }
            }
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok().flatten())
            {
                let _ = storage.set_item("theme", theme);
            }
        }
    });

    view! {
        <Stylesheet id="leptos" href="/pkg/docs-mentisdb-com.css"/>
        <Title text="MentisDB Docs"/>
        <Meta name="description" content="Documentation for MentisDB — the durable memory engine for AI agents."/>
        <Router>
            <DocsNavBar/>
            <main class="docs-main">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=DocsHome/>
                    <Route path=StaticSegment("user") view=UserDocs/>
                    <Route path=StaticSegment("agent") view=AgentDocs/>
                    <Route path=StaticSegment("developer") view=DeveloperDocs/>
                </Routes>
            </main>
            <DocsFooter/>
        </Router>
    }
}
