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
