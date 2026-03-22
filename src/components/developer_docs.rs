use leptos::prelude::*;

/// DeveloperDocs page — Rust library integration guide.
/// Primary CTA: send developers to https://docs.rs/mentisdb/0.5.2/mentisdb/
#[component]
pub fn DeveloperDocs() -> impl IntoView {
    view! {
        <div class="docs-page">
            <div class="container">
                <div class="docs-layout">

                    // ── Sidebar ──────────────────────────────────────────────
                    <aside class="docs-sidebar">
                        <nav class="docs-nav">
                            <a href="#overview"          class="docs-nav-link">"Overview"</a>
                            <a href="#crate-docs"        class="docs-nav-link">"Crate Docs"</a>
                            <a href="#mcp-server"        class="docs-nav-link">"MCP Server"</a>
                            <a href="#rest-api"          class="docs-nav-link">"REST API"</a>
                            <a href="#schema"            class="docs-nav-link">"Schema Version"</a>
                            <a href="#import-api"        class="docs-nav-link">"Bulk Import API"</a>
                            <a href="#thought-relations" class="docs-nav-link">"Thought Relations"</a>
                            <a href="#storage"           class="docs-nav-link">"Storage Adapters"</a>
                            <a href="#contributing"      class="docs-nav-link">"Contributing"</a>
                        </nav>
                    </aside>

                    // ── Main content ─────────────────────────────────────────
                    <main class="docs-content">

                        // ── Overview ─────────────────────────────────────────
                        <h1>"Developer Documentation"</h1>
                        <h2 id="overview">"Overview"</h2>
                        <p>
                            "MentisDB is a standalone Rust crate available on crates.io. \
                             Add it to your project:"
                        </p>
                        <div class="code-block">
                            <code>r#"mentisdb = "0.5""#</code>
                        </div>
                        <p>
                            "The crate provides the full memory engine, skill registry, \
                             storage adapter interface, agent registry, and optional HTTP \
                             server (MCP + REST) behind the "
                            <code>"ssr"</code>
                            " feature flag."
                        </p>

                        // ── Full API Reference (PRIMARY SECTION) ─────────────
                        <h2 id="crate-docs">"Full API Reference"</h2>
                        <p>
                            "The complete, generated API documentation is hosted on docs.rs:"
                        </p>
                        <div class="docs-cta-card">
                            <h3>"docs.rs/mentisdb"</h3>
                            <p>
                                "Full rustdoc for all public types, traits, functions, and \
                                 modules. Includes "
                                <code>"MentisDb"</code>
                                ", "
                                <code>"ThoughtChain"</code>
                                ", "
                                <code>"StorageAdapter"</code>
                                ", "
                                <code>"SkillRegistry"</code>
                                ", "
                                <code>"BinaryStorageAdapter"</code>
                                ", "
                                <code>"JsonlStorageAdapter"</code>
                                ", and the HTTP server."
                            </p>
                            <a
                                href="https://docs.rs/mentisdb/0.5.2/mentisdb/"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="btn-primary"
                            >
                                "View on docs.rs →"
                            </a>
                        </div>

                        // ── MCP Server ───────────────────────────────────────
                        <h2 id="mcp-server">"MCP Server"</h2>
                        <p>
                            "MentisDB ships a built-in MCP (Model Context Protocol) HTTP \
                             server. Enable it with the "
                            <code>"ssr"</code>
                            " feature. The server exposes all MentisDB operations as MCP \
                             tools, making it compatible with any MCP-capable AI tool."
                        </p>
                        <p>
                            "Default endpoint: "
                            <code>"http://127.0.0.1:9471"</code>
                        </p>
                        <p>
                            "The daemon binary ("
                            <code>"mentisdbd"</code>
                            ") starts the server automatically. For embedding in your own \
                             Axum app, see the docs.rs API reference for server module details."
                        </p>

                        // ── REST API ─────────────────────────────────────────
                        <h2 id="rest-api">"REST API"</h2>
                        <p>
                            "The same daemon also serves a REST API alongside MCP. \
                             Key endpoints:"
                        </p>
                        <table class="api-table">
                            <thead>
                                <tr>
                                    <th>"Method"</th>
                                    <th>"Path"</th>
                                    <th>"Description"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/thoughts"</code></td>
                                    <td>"Append a new thought to a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/thoughts"</code></td>
                                    <td>"Search/query thoughts"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/chains"</code></td>
                                    <td>"List available chain keys"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/agents"</code></td>
                                    <td>"List agents in registry"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/skills"</code></td>
                                    <td>"Upload a skill version"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/skills/{id}"</code></td>
                                    <td>"Read a skill (latest or specific version)"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/import-markdown"</code></td>
                                    <td>"Bulk import a MEMORY.md into a chain"</td>
                                </tr>
                            </tbody>
                        </table>

                        // ── Schema Version ───────────────────────────────────
                        <h2 id="schema">"Schema Version"</h2>
                        <p>
                            "MentisDB 0.5.2 introduces schema version 2 ("
                            <code>"MENTISDB_SCHEMA_V2 = 2"</code>
                            "). All new chains are created at V2 automatically. \
                             Legacy V0 chains (created before 0.5.2) are migrated \
                             transparently on first open — no manual migration step \
                             and no data loss."
                        </p>

                        // ── Bulk Import API ──────────────────────────────────
                        <h2 id="import-api">"Bulk Import API"</h2>
                        <p>
                            "Import an existing "
                            <code>"MEMORY.md"</code>
                            " (or any MentisDB-style Markdown export) into a chain \
                             in one call:"
                        </p>
                        <div class="code-block">
                            <pre><code>
"let count = db\n\
    .import_from_memory_markdown(markdown_str, \"orion\")\n\
    .await?;\n\
println!(\"Imported {} thoughts\", count);"
                            </code></pre>
                        </div>
                        <p>
                            "The method parses the Markdown document, creates one thought \
                             per heading/section, and appends them all to the chain under \
                             "
                            <code>"default_agent_id"</code>
                            ". Returns the count of thoughts imported."
                        </p>
                        <p>"The same operation is also available via REST and MCP:"</p>
                        <ul>
                            <li>
                                "REST: "
                                <code>"POST /v1/import-markdown"</code>
                                " — body: "
                                <code>r#"{ "markdown": "...", "default_agent_id": "orion", "chain_key": "..." }"#</code>
                            </li>
                            <li>
                                "Dashboard: "
                                <code>"POST /dashboard/api/chains/{chain_key}/import-markdown"</code>
                            </li>
                            <li>
                                "MCP: "
                                <code>"mentisdb_import_memory_markdown(markdown, default_agent_id, chain_key)"</code>
                            </li>
                        </ul>

                        // ── Thought Relations ────────────────────────────────
                        <h2 id="thought-relations">"Thought Relations & Cross-chain References"</h2>
                        <p>
                            "A "
                            <code>"ThoughtRelation"</code>
                            " is a typed semantic edge between two thoughts. Attach relations \
                             to a "
                            <code>"ThoughtInput"</code>
                            " using the builder API:"
                        </p>
                        <div class="code-block">
                            <pre><code>
"// Intra-chain relation:\n\
let input = ThoughtInput::new(ThoughtType::Reframe, \"We now frame this as Y.\")\n\
    .with_relation(ThoughtRelation {\n\
        kind: ThoughtRelationKind::Supersedes,\n\
        target_id: old_thought_uuid,\n\
        chain_key: None,\n\
    });\n\
\n\
// Cross-chain relation — use the convenience builder:\n\
let input = ThoughtInput::new(ThoughtType::Decision, \"Adopted the shared convention.\")\n\
    .with_cross_chain_relation(\n\
        ThoughtRelationKind::Supersedes,\n\
        old_thought_uuid,\n\
        \"platform-conventions\",\n\
    );"
                            </code></pre>
                        </div>
                        <p>
                            "The "
                            <code>"chain_key: Option&lt;String&gt;"</code>
                            " field on "
                            <code>"ThoughtRelation"</code>
                            " makes cross-chain references first-class. \
                             Intra-chain relations ("
                            <code>"chain_key: None"</code>
                            ") remain fully backward-compatible with all existing code."
                        </p>
                        <h3>"ThoughtRelationKind::Supersedes"</h3>
                        <p>
                            "Use "
                            <code>"Supersedes"</code>
                            " to declare that a new thought replaces an older one that was \
                             correct but is now outdated or reframed. It is the canonical \
                             replacement edge — distinct from a "
                            <code>"Correction"</code>
                            " (which signals a factual error). Pair it with the "
                            <code>"Reframe"</code>
                            " ThoughtType for perspective or framing shifts."
                        </p>

                        // ── Storage Adapters ─────────────────────────────────
                        <h2 id="storage">"Storage Adapters"</h2>
                        <p>
                            "MentisDB separates the memory model from the storage backend \
                             via the "
                            <code>"StorageAdapter"</code>
                            " trait:"
                        </p>
                        <ul>
                            <li>
                                <strong>"BinaryStorageAdapter"</strong>
                                " — Default. Compact binary format with write buffering. \
                                 Best for production."
                            </li>
                            <li>
                                <strong>"JsonlStorageAdapter"</strong>
                                " — Line-oriented JSON. Human-readable, inspectable with \
                                 standard tools. Good for debugging."
                            </li>
                        </ul>
                        <p>
                            "Implement the "
                            <code>"StorageAdapter"</code>
                            " trait to plug in your own backend (S3, SQLite, etc.). \
                             See docs.rs for the trait definition."
                        </p>

                        // ── Contributing ─────────────────────────────────────
                        <h2 id="contributing">"Contributing"</h2>
                        <p>
                            "MentisDB is open source under the MIT license."
                        </p>
                        <div class="code-block">
                            <code>"git clone https://github.com/cloudllm-ai/mentisdb"</code>
                        </div>
                        <p>"Run the test suite:"</p>
                        <div class="code-block">
                            <code>"cargo test"</code>
                        </div>
                        <p>"Run benchmarks:"</p>
                        <div class="code-block">
                            <code>"make bench"</code>
                        </div>
                        <a
                            href="https://github.com/cloudllm-ai/mentisdb"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="btn-secondary"
                        >
                            "View on GitHub →"
                        </a>

                    </main>
                </div>
            </div>
        </div>
    }
}
