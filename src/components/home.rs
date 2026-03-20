use leptos::prelude::*;

// ---------------------------------------------------------------------------
// Internal sub-component — one audience card
// ---------------------------------------------------------------------------

/// A single clickable card on the audience-selector landing page.
///
/// The entire card is wrapped in `<a href>` so the full surface area is
/// interactive without any JS — plain progressive-enhancement HTML.
#[component]
fn AudienceCard(
    /// Root-relative href the card links to (e.g. "/user")
    href: &'static str,
    /// Large emoji icon rendered above the title
    icon: &'static str,
    /// Bold heading inside the card
    title: &'static str,
    /// Short descriptive paragraph below the heading
    subtitle: &'static str,
    /// Accent call-to-action text at the bottom of the card
    cta: &'static str,
) -> impl IntoView {
    view! {
        <a href=href class="audience-card">
            <div class="audience-icon">{icon}</div>
            <div class="audience-title">{title}</div>
            <p class="audience-subtitle">{subtitle}</p>
            <span class="audience-cta">{cta}</span>
        </a>
    }
}

// ---------------------------------------------------------------------------
// Public component — full landing page
// ---------------------------------------------------------------------------

/// Audience-selector landing page for docs.mentisdb.com.
///
/// Visitors are greeted with a brief hero headline and three equal-width
/// cards — one for each docs audience: end-users, autonomous agents, and
/// Rust developers integrating MentisDB into their applications.
///
/// The page is entirely static; no signals are needed.
#[component]
pub fn DocsHome() -> impl IntoView {
    view! {
        <div class="docs-home">
            <div class="container">

                // ── Hero ────────────────────────────────────────────────
                <div class="docs-hero">
                    <h1>"MentisDB Documentation"</h1>
                    <p class="docs-hero-sub">"Choose your path:"</p>
                </div>

                // ── Audience grid ────────────────────────────────────────
                <div class="audience-grid">

                    <AudienceCard
                        href="/user"
                        icon="🛠️"
                        title="I'm a User"
                        subtitle="Setting up MentisDB, configuring the daemon, using the skills \
                                  registry, and securing memories with cryptographic signatures."
                        cta="User Guide →"
                    />

                    <AudienceCard
                        href="/agent"
                        icon="🤖"
                        title="I'm an Agent"
                        subtitle="How to read from and write to MentisDB effectively. Thought \
                                  types, roles, tags, retrieval patterns, context window protocols."
                        cta="Agent Guide →"
                    />

                    <AudienceCard
                        href="/developer"
                        icon="⚙️"
                        title="I'm a Developer"
                        subtitle="Integrating MentisDB into your Rust application. Full API \
                                  reference, storage adapters, MCP server, REST endpoints."
                        cta="API Reference →"
                    />

                </div>
            </div>
        </div>
    }
}
