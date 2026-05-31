use leptos::prelude::*;

/// DeveloperDocs page — Rust library integration guide.
/// Primary CTA: send developers to https://docs.rs/mentisdb/latest/mentisdb/
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
                            <a href="#rest-api"          class="docs-nav-link">"REST API"</a>
                            <a href="#mcp-server"        class="docs-nav-link">"MCP Server"</a>
                            <a href="#schema"            class="docs-nav-link">"Schema Version"</a>
                            <a href="#import-api"        class="docs-nav-link">"Bulk Import API"</a>
                            <a href="#taxonomy"          class="docs-nav-link">"Thought Taxonomy"</a>
                            <a href="#thought-relations" class="docs-nav-link">"Thought Relations"</a>
                            <a href="#storage"           class="docs-nav-link">"Storage Adapters"</a>
                            <a href="#0.8.0"             class="docs-nav-link">"0.8.0 Improvements"</a>
                            <a href="#0.8.1"             class="docs-nav-link">"0.8.1 Improvements"</a>
                            <a href="#0.8.2"             class="docs-nav-link">"0.8.2 Features"</a>
                            <a href="#0.8.6"             class="docs-nav-link">"0.8.6 Features"</a>
                            <a href="#0.9.x"             class="docs-nav-link">"0.9.x Features"</a>
                            <a href="#benchmarking"      class="docs-nav-link">"Benchmarking"</a>
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
                            <code>r#"mentisdb = "0.9.1""#</code>
                        </div>
                        <p>
                            "The crate provides the full memory engine, skill registry, \
                             storage adapter interface, agent registry, and optional HTTP \
                             server stack (MCP + REST + dashboard) behind the "
                            <code>"server"</code>
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
                                <code>"Thought"</code>
                                ", "
                                <code>"ThoughtInput"</code>
                                ", "
                                <code>"ThoughtQuery"</code>
                                ", "
                                <code>"StorageAdapter"</code>
                                ", "
                                <code>"SkillRegistry"</code>
                                ", "
                                 <code>"BinaryStorageAdapter"</code>
                                 ", and the HTTP server."
                            </p>
                            <a
                                href="https://docs.rs/mentisdb/latest/mentisdb/"
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
                            <code>"server"</code>
                            " feature. All MentisDB operations are exposed as MCP tools \
                             over JSON-RPC 2.0 via HTTP, making it compatible with any \
                             MCP-capable AI tool. The canonical onboarding path is not a \
                             separate URL: it happens during the MCP "
                            <code>"initialize"</code>
                            " call via startup instructions plus the embedded resource "
                            <code>"mentisdb://skill/core"</code>
                            " exposed through "
                            <code>"resources/list"</code>
                            " and "
                            <code>"resources/read"</code>
                            "."
                        </p>
                        <p>
                            "Default HTTP endpoint: "
                            <code>"http://127.0.0.1:9471"</code>
                            " — HTTPS endpoint: "
                            <code>"https://127.0.0.1:9473"</code>
                            "."
                        </p>

                        <h3>"All 37 MCP Tools"</h3>
                        <table class="api-table">
                            <thead>
                                <tr>
                                    <th>"Tool"</th>
                                    <th>"Description"</th>
                                    <th>"Key Parameters"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td><code>"mentisdb_bootstrap"</code></td>
                                    <td>"Create a chain if needed and write one bootstrap checkpoint when it is empty"</td>
                                    <td><code>"chain_key"</code>, <code>"content"</code>, <code>"concepts"</code>, <code>"importance"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_append"</code></td>
                                    <td>"Append a durable semantic thought with optional tags, concepts, refs, scope, and signature metadata"</td>
                                    <td><code>"chain_key"</code>, <code>"thought_type"</code>, <code>"content"</code>, <code>"agent_id"</code>, <code>"tags"</code>, <code>"concepts"</code>, <code>"scope"</code>, <code>"importance"</code>, <code>"confidence"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_append_retrospective"</code></td>
                                    <td>"Append a guided retrospective memory to prevent future agents from repeating a hard failure"</td>
                                    <td><code>"chain_key"</code>, <code>"content"</code>, <code>"concepts"</code>, <code>"refs"</code>, <code>"importance"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_search"</code></td>
                                    <td>"Search thoughts by semantic filters, identity filters, time bounds, and scoring thresholds"</td>
                                    <td><code>"chain_key"</code>, <code>"text"</code>, <code>"thought_types"</code>, <code>"roles"</code>, <code>"agent_ids"</code>, <code>"since"</code>, <code>"until"</code>, <code>"min_importance"</code>, <code>"min_confidence"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_lexical_search"</code></td>
                                    <td>"Return flat ranked lexical matches with explainable term and field provenance"</td>
                                    <td><code>"chain_key"</code>, <code>"text"</code>, <code>"thought_types"</code>, <code>"limit"</code>, <code>"offset"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_ranked_search"</code></td>
                                    <td>"Return flat ranked results with lexical, graph-aware, or heuristic score breakdowns. Supports point-in-time queries and memory scope filtering"</td>
                                    <td><code>"chain_key"</code>, <code>"text"</code>, <code>"as_of"</code>, <code>"scope"</code>, <code>"enable_reranking"</code>, <code>"rerank_k"</code>, <code>"concepts_any"</code>, <code>"roles"</code>, <code>"limit"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_context_bundles"</code></td>
                                    <td>"Return seed-anchored grouped support context beneath the best lexical seeds"</td>
                                    <td><code>"chain_key"</code>, <code>"text"</code>, <code>"as_of"</code>, <code>"scope"</code>, <code>"graph"</code>, <code>"limit"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_list_chains"</code></td>
                                    <td>"List known chains with version, storage adapter, counts, and storage location"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_merge_chains"</code></td>
                                    <td>"Merge all thoughts from a source chain into a target chain, then permanently delete the source"</td>
                                    <td><code>"source_chain_key"</code>, <code>"target_chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_branch_from"</code></td>
                                    <td>"Create a new chain diverging from a thought on an existing chain. Searches on the branch transparently include ancestor results"</td>
                                    <td><code>"source_chain_key"</code>, <code>"branch_chain_key"</code>, <code>"branch_thought_id"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_list_agents"</code></td>
                                    <td>"List the distinct agent identities participating in one chain"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_get_agent"</code></td>
                                    <td>"Return one full agent registry record including status, aliases, description, keys, and per-chain activity metadata"</td>
                                    <td><code>"agent_id"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_list_agent_registry"</code></td>
                                    <td>"Return the full per-chain agent registry"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_upsert_agent"</code></td>
                                    <td>"Create or update a registry record before or after an agent writes thoughts"</td>
                                    <td><code>"agent_id"</code>, <code>"display_name"</code>, <code>"agent_owner"</code>, <code>"description"</code>, <code>"status"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_set_agent_description"</code></td>
                                    <td>"Set or clear the description stored for one registered agent"</td>
                                    <td><code>"agent_id"</code>, <code>"description"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_add_agent_alias"</code></td>
                                    <td>"Add a historical or alternate alias to a registered agent"</td>
                                    <td><code>"agent_id"</code>, <code>"alias"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_add_agent_key"</code></td>
                                    <td>"Add or replace one public verification key on a registered agent"</td>
                                    <td><code>"agent_id"</code>, <code>"key_id"</code>, <code>"algorithm"</code>, <code>"public_key_bytes"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_revoke_agent_key"</code></td>
                                    <td>"Revoke one previously registered public key"</td>
                                    <td><code>"agent_id"</code>, <code>"key_id"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_disable_agent"</code></td>
                                    <td>"Disable one agent by marking its registry status as revoked"</td>
                                    <td><code>"agent_id"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_recent_context"</code></td>
                                    <td>"Render recent thoughts into a prompt snippet for session resumption"</td>
                                    <td><code>"chain_key"</code>, <code>"last_n"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_memory_markdown"</code></td>
                                    <td>"Export a MEMORY.md-style Markdown view of the full chain or a filtered subset"</td>
                                    <td><code>"chain_key"</code>, <code>"agent_ids"</code>, <code>"thought_types"</code>, <code>"roles"</code>, <code>"since"</code>, <code>"until"</code>, <code>"limit"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_import_memory_markdown"</code></td>
                                    <td>"Import a MEMORY.md-formatted Markdown document into a target chain"</td>
                                    <td><code>"markdown"</code>, <code>"default_agent_id"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_get_thought"</code></td>
                                    <td>"Return one stored thought by stable id, chain index, or content hash"</td>
                                    <td><code>"thought_id"</code>, <code>"thought_index"</code>, <code>"thought_hash"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_get_genesis_thought"</code></td>
                                    <td>"Return the first thought ever recorded in the chain, if any"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_traverse_thoughts"</code></td>
                                    <td>"Traverse the chain forward or backward in append order from a chosen anchor, in chunks, with optional filters"</td>
                                    <td><code>"chain_key"</code>, <code>"anchor_id"</code>, <code>"anchor_index"</code>, <code>"anchor_hash"</code>, <code>"direction"</code>, <code>"chunk_size"</code>, <code>"thought_types"</code>, <code>"roles"</code>, <code>"since"</code>, <code>"until"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_skill_md"</code></td>
                                    <td>"Return the official embedded MENTISDB_SKILL.md Markdown file"</td>
                                    <td>(none)</td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_list_skills"</code></td>
                                    <td>"List versioned skill summaries from the skill registry"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_skill_manifest"</code></td>
                                    <td>"Return the versioned skill-registry manifest including searchable fields and supported formats"</td>
                                    <td>(none)</td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_upload_skill"</code></td>
                                    <td>"Upload a new immutable skill version from Markdown or JSON"</td>
                                    <td><code>"agent_id"</code>, <code>"skill_id"</code>, <code>"content"</code>, <code>"format"</code>, <code>"signing_key_id"</code>, <code>"skill_signature"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_search_skill"</code></td>
                                    <td>"Search skills by indexed metadata such as ids, names, tags, triggers, uploader identity, status, format, schema version, and time window"</td>
                                    <td><code>"chain_key"</code>, <code>"text"</code>, <code>"skill_ids"</code>, <code>"names"</code>, <code>"statuses"</code>, <code>"since"</code>, <code>"until"</code>, <code>"limit"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_read_skill"</code></td>
                                    <td>"Read one stored skill as Markdown or JSON. Responses include trust warnings for untrusted or malicious skill content"</td>
                                    <td><code>"skill_id"</code>, <code>"version_id"</code>, <code>"format"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_skill_versions"</code></td>
                                    <td>"List immutable uploaded versions for one skill"</td>
                                    <td><code>"skill_id"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_deprecate_skill"</code></td>
                                    <td>"Mark a skill as deprecated while preserving all prior versions"</td>
                                    <td><code>"skill_id"</code>, <code>"reason"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_revoke_skill"</code></td>
                                    <td>"Mark a skill as revoked while preserving audit history"</td>
                                    <td><code>"skill_id"</code>, <code>"reason"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_head"</code></td>
                                    <td>"Return head metadata, the latest thought at the current chain tip, and integrity state"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_register_webhook"</code></td>
                                    <td>"Register a webhook to receive HTTP POST notifications when thoughts are appended. Delivery is fire-and-forget with exponential backoff retries (up to 5 attempts)"</td>
                                    <td><code>"chain_key"</code>, <code>"url"</code>, <code>"event_types"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_list_webhooks"</code></td>
                                    <td>"List all registered webhooks"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_delete_webhook"</code></td>
                                    <td>"Remove a webhook registration by its UUID"</td>
                                    <td><code>"webhook_id"</code>, <code>"chain_key"</code></td>
                                </tr>
                            </tbody>
                        </table>

                        <h3>"JSON-RPC Request/Response Example"</h3>
                        <p>"MCP uses JSON-RPC 2.0 over HTTP. Send a POST to the MCP endpoint:"</p>
                        <div class="code-block">
                            <pre><code>
    "POST / HTTP/1.1\n\
    Host: 127.0.0.1:9471\n\
    Content-Type: application/json\n\
    \n\
    {\n\
    \"jsonrpc\": \"2.0\",\n\
    \"id\": 1,\n\
    \"method\": \"tools/call\",\n\
    \"params\": {\n\
    \"name\": \"mentisdb_append\",\n\
    \"arguments\": {\n\
      \"chain_key\": \"default\",\n\
      \"agent_id\": \"assistant\",\n\
      \"thought_type\": \"Insight\",\n\
      \"content\": \"Memory deduplication triggers when Jaccard similarity exceeds threshold\",\n\
      \"importance\": 0.7,\n\
      \"confidence\": 0.9,\n\
      \"tags\": [\"feature:dedup\"],\n\
      \"concepts\": [\"memory-dedup\", \"jaccard-similarity\"]\n\
    }\n\
    }\n\
    }\n\
    \n\
    // Response:\n\
    {\n\
    \"jsonrpc\": \"2.0\",\n\
    \"id\": 1,\n\
    \"result\": {\n\
    \"content\": [\n\
      {\n\
        \"type\": \"text\",\n\
        \"text\": \"Thought appended: a1b2c3... (index 42)\"\n\
      }\n\
    ]\n\
    }\n\
    }"</code></pre>
                        </div>

                        <h3>"MCP HTTP Ports"</h3>
                        <p>
                            "The MCP server listens on two ports:"
                        </p>
                        <ul>
                            <li>
                                <code>"http://127.0.0.1:9471"</code>
                                " — HTTP (unencrypted), suitable for local development"
                            </li>
                            <li>
                                <code>"https://127.0.0.1:9473"</code>
                                " — HTTPS (TLS), recommended for production"
                            </li>
                        </ul>
                        <p>
                            "The daemon binary ("
                            <code>"mentisdb"</code>
                            ") starts the MCP server automatically. For embedding in your own \
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
                                    <td><code>"/v1/thoughts"</code></td>
                                    <td>"Append a new thought to a chain. Supports scope parameter for memory scope tagging"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/search"</code></td>
                                    <td>"Search/query thoughts. Supports as_of for point-in-time queries and scope for scope filtering"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/lexical-search"</code></td>
                                    <td>"Ranked lexical search with scores and matched-term diagnostics"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/ranked-search"</code></td>
                                    <td>"Canonical flat ranked retrieval with lexical + vector + graph-aware score breakdowns (hybrid when managed sidecars are available). Supports as_of, scope, enable_reranking, and rerank_k parameters"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/context-bundles"</code></td>
                                    <td>"Seed-anchored grouped support context for agent reasoning and dashboard inspection. Supports as_of and scope parameters"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/v1/chains"</code></td>
                                    <td>"List available chain keys"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/chains/branch"</code></td>
                                    <td>"Create a branch chain diverging from a thought on an existing chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents"</code></td>
                                    <td>"List agents in a chain or inspect the registry"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/mentisdb_skill_md"</code></td>
                                    <td>"Compatibility fallback for clients that cannot use MCP resources; MCP-native clients should read mentisdb://skill/core after initialize"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/skills/upload"</code></td>
                                    <td>"Upload a skill version"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/skills/read"</code></td>
                                    <td>"Read a skill (latest or specific version)"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/recent-context"</code></td>
                                    <td>"Render a compact resumption prompt for recent, important context"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/memory-markdown"</code></td>
                                    <td>"Export a chain as MentisDB-flavored MEMORY.md markdown"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/head"</code></td>
                                    <td>"Return current chain head metadata and the latest thought"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/health"</code></td>
                                    <td>"Liveness health check"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/bootstrap"</code></td>
                                    <td>"Bootstrap a chain with an initial memory"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/retrospectives"</code></td>
                                    <td>"Append a guided retrospective thought"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/thought"</code></td>
                                    <td>"Retrieve a single thought by ID, hash, or index"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/thoughts/genesis"</code></td>
                                    <td>"Retrieve the first thought in a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/thoughts/traverse"</code></td>
                                    <td>"Traverse thoughts in append order with filtering and pagination"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/import-markdown"</code></td>
                                    <td>"Import MEMORY.md-formatted markdown into a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/v1/skills"</code></td>
                                    <td>"List uploaded skill summaries"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/v1/skills/manifest"</code></td>
                                    <td>"Return the skill registry manifest with searchable fields"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/skills/search"</code></td>
                                    <td>"Search the versioned skill registry by indexed fields"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/skills/versions"</code></td>
                                    <td>"List immutable versions for a stored skill"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/skills/deprecate"</code></td>
                                    <td>"Mark a stored skill as deprecated"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/skills/revoke"</code></td>
                                    <td>"Mark a stored skill as revoked"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agent"</code></td>
                                    <td>"Get a single agent record by ID"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agent-registry"</code></td>
                                    <td>"List the full agent registry for a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/upsert"</code></td>
                                    <td>"Create or update an agent record"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/description"</code></td>
                                    <td>"Set or clear an agent's description"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/aliases"</code></td>
                                    <td>"Add an alias to an agent"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/keys"</code></td>
                                    <td>"Add a verification key to an agent"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/keys/revoke"</code></td>
                                    <td>"Revoke a verification key from an agent"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/disable"</code></td>
                                    <td>"Disable (revoke) an agent"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/vectors/rebuild"</code></td>
                                    <td>"Rebuild vector sidecar indexes for a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/chains/merge"</code></td>
                                    <td>"Merge all thoughts from a source chain into a target, then delete the source"</td>
                                </tr>
                            </tbody>
                        </table>
                        <p>
                            "For MCP-native agents, prefer the streamable HTTP root at "
                            <code>"POST /"</code>
                            " and let the agent bootstrap itself from the initialize instructions and resource catalog."
                        </p>
                        <p>
                            "For direct crate users, 0.8.0 keeps vector sidecars additive and \
                             rebuildable, but ranked search now blends lexical, graph, and \
                             managed-sidecar vector signals automatically when sidecars are \
                             enabled. The append-only chain remains canonical; sidecars are \
                             derived acceleration state."
                        </p>
                        <p>
                            "The daemon now applies persisted managed-vector settings whenever \
                             it opens a chain. By default each chain gets the built-in \
                             local-text provider ("
                            <code>"local-text-v1"</code>
                            "), and ranked search across REST, MCP, and dashboard surfaces \
                              upgrades to hybrid scoring when that sidecar is available."
                        </p>

                        <h3>"REST Example: POST /v1/thoughts (Append Thought)"</h3>
                        <div class="code-block">
                            <pre><code>
    "POST /v1/thoughts HTTP/1.1\n\
    Host: 127.0.0.1:9471\n\
    Content-Type: application/json\n\
    \n\
    {\n\
    \"chain_key\": \"default\",\n\
    \"agent_id\": \"assistant\",\n\
    \"thought_type\": \"Insight\",\n\
    \"content\": \"Memory deduplication triggers when Jaccard similarity exceeds threshold\",\n\
    \"importance\": 0.7,\n\
    \"confidence\": 0.9,\n\
    \"tags\": [\"feature:dedup\"],\n\
    \"concepts\": [\"memory-dedup\", \"jaccard-similarity\"],\n\
    \"scope\": \"user\",\n\
    \"relations\": [\n\
    { \"kind\": \"ContinuesFrom\", \"target_id\": \"<uuid-of-prior-turn>\" }\n\
    ]\n\
    }\n\
    \n\
    // Response:\n\
    {\n\
    \"thought\": {\n\
    \"id\": \"a1b2c3d4-...\",\n\
    \"index\": 42,\n\
    \"hash\": \"9f14...\",\n\
    \"thought_type\": \"Insight\",\n\
    \"content\": \"Memory deduplication triggers when Jaccard similarity exceeds threshold\",\n\
    \"agent_id\": \"assistant\",\n\
    \"importance\": 0.7,\n\
    \"confidence\": 0.9,\n\
    \"tags\": [\"feature:dedup\"],\n\
    \"concepts\": [\"memory-dedup\", \"jaccard-similarity\"],\n\
    \"created_at\": \"2026-04-14T12:00:00Z\"\n\
    }\n\
    }"</code></pre>
                        </div>

                        <h3>"REST Example: POST /v1/ranked-search"</h3>
                        <div class="code-block">
                            <pre><code>
    "POST /v1/ranked-search HTTP/1.1\n\
    Host: 127.0.0.1:9471\n\
    Content-Type: application/json\n\
    \n\
    {\n\
    \"chain_key\": \"default\",\n\
    \"text\": \"how does memory deduplication work\",\n\
    \"scope\": \"user\",\n\
    \"limit\": 10,\n\
    \"enable_reranking\": true,\n\
    \"rerank_k\": 50\n\
    }\n\
    \n\
    // Response:\n\
    {\n\
    \"results\": [\n\
    {\n\
      \"thought\": {\n\
        \"id\": \"a1b2c3d4-...\",\n\
        \"thought_type\": \"Insight\",\n\
        \"content\": \"Memory deduplication triggers when Jaccard similarity exceeds threshold\",\n\
        \"importance\": 0.7\n\
      },\n\
      \"score\": 4.82,\n\
      \"lexical_score\": 1.2,\n\
      \"vector_score\": 3.1,\n\
      \"session_cohesion\": 0.4,\n\
      \"rank\": 1\n\
    }\n\
    ],\n\
    \"total\": 1,\n\
    \"query_time_ms\": 12\n\
    }"</code></pre>
                        </div>

                        <h3>"REST Example: POST /v1/context-bundles"</h3>
                        <div class="code-block">
                            <pre><code>
    "POST /v1/context-bundles HTTP/1.1\n\
    Host: 127.0.0.1:9471\n\
    Content-Type: application/json\n\
    \n\
    {\n\
    \"chain_key\": \"default\",\n\
    \"text\": \"memory deduplication decision\",\n\
    \"scope\": \"user\",\n\
    \"limit\": 5\n\
    }\n\
    \n\
    // Response:\n\
    {\n\
    \"bundles\": [\n\
    {\n\
      \"seed\": {\n\
        \"id\": \"a1b2c3d4-...\",\n\
        \"thought_type\": \"Decision\",\n\
        \"content\": \"Enable dedup at 0.85 Jaccard threshold\",\n\
        \"score\": 5.1\n\
      },\n\
      \"supporting\": [\n\
        {\n\
          \"id\": \"b2c3d4e5-...\",\n\
          \"thought_type\": \"LessonLearned\",\n\
          \"content\": \"Previous attempts at 0.95 threshold produced too many false negatives\",\n\
          \"chain_key\": \"default\"\n\
        }\n\
      ],\n\
      \"children\": []\n\
    }\n\
    ]\n\
    }"</code></pre>
                        </div>

                        <h3>"REST Example: POST /v1/webhooks/register"</h3>
                        <div class="code-block">
                            <pre><code>
    "POST /v1/webhooks/register HTTP/1.1\n\
    Host: 127.0.0.1:9471\n\
    Content-Type: application/json\n\
    \n\
    {\n\
    \"chain_key\": \"default\",\n\
    \"url\": \"https://myapp.example.com/mentisdb-webhook\",\n\
    \"event_types\": [\"thought.append\", \"thought.relation.added\"]\n\
    }\n\
    \n\
    // Response:\n\
    {\n\
    \"webhook\": {\n\
    \"id\": \"wh_abc123\",\n\
    \"chain_key\": \"default\",\n\
    \"url\": \"https://myapp.example.com/mentisdb-webhook\",\n\
    \"event_types\": [\"thought.append\", \"thought.relation.added\"],\n\
    \"created_at\": \"2026-04-14T12:00:00Z\",\n\
    \"status\": \"active\"\n\
    }\n\
    }"</code></pre>
                        </div>

                        // ── 0.8.0 Improvements ──────────────────────────────
                        <h2 id="0.8.0">"0.8.0 Search &amp; Storage Improvements"</h2>
                        <p>
                            "MentisDB 0.8.0 introduces five major improvements to the search \
                             and storage pipeline: Porter stemming, tiered vector-lexical \
                             fusion, importance weighting, bincode hashing, and managed \
                             sidecar entries with auto_sync."
                        </p>

                        <h3>"Porter Stemming (Normalizer v2)"</h3>
                        <p>
                            "The lexical tokenizer now applies English Porter stemming. \
                             Words like "
                            <code>"preferences"</code>
                            " → "
                            <code>"prefer"</code>
                            " and "
                            <code>"running"</code>
                            " → "
                            <code>"run"</code>
                            " now match in lexical search. Existing chains auto-reindex \
                             on first open — no manual migration step required."
                        </p>

                        <h3>"Tiered Vector-Lexical Fusion"</h3>
                        <p>
                            "Ranked search scoring replaces the flat addition model with \
                             a tiered boost that respects the relationship between lexical \
                             and vector signals:"
                        </p>
                        <ul>
                            <li>
                                "Lexical = 0 and vector &gt; 0: "
                                <code>"vector × 60"</code>
                                " — full boost for semantic-only hits"
                            </li>
                            <li>
                                "0 &lt; lexical &lt; 1.0 and vector &gt; 0: "
                                <code>"vector × (1 + 20 × fraction)"</code>
                                " — partial boost proportional to lexical overlap"
                            </li>
                            <li>
                                "Lexical ≥ 1.0: vector as-is — no boost, lexical dominates"
                            </li>
                        </ul>
                         <p>
                             "Failed logins are rate-limited per-IP to prevent brute-force enumeration; \
                              the dashboard is always served over TLS so the PIN is never transmitted in \
                              the clear on a local network."
                         </p>

                         <h3>"Automatic Thesaurus in Ranked Search (0.9.9)"</h3>
                         <p>
                             "The static thesaurus (~900 headwords + 300+ lemmas) now applies \
                              <strong>automatically by default</strong> to every ranked search query \
                              (REST <code>POST /v1/ranked-search</code>, MCP <code>mentisdb_ranked_search</code>, \
                              dashboard, and CLI). No client changes or extra parameters are required. \
                              With full embeddings this delivered the target LoCoMo-10P $R@10 = 72.6%. \
                              The expansion happens server-side via <code>apply_thesaurus_if_text</code> \
                              before BM25 scoring."
                         </p>

                         // ── Benchmarking ────────────────────────────────────
                        <h2 id="benchmarking">"Benchmarking"</h2>
                        <p>
                            "MentisDB ships with two benchmark styles:"
                        </p>
                        <ul>
                            <li>
                                <strong>"Criterion microbenchmarks"</strong>
                                " for in-process append, query, traversal, import, and skill-registry hot paths"
                            </li>
                            <li>
                                <strong>"HTTP concurrency benchmarks"</strong>
                                " for live "
                                <code>"mentisdb"</code>
                                " write/read waves under concurrent client load"
                            </li>
                        </ul>
                        <p>
                            "Run the full benchmark suite with:"
                        </p>
                        <div class="code-block">
                            <code>"make bench"</code>
                        </div>
                        <p>
                            "This currently runs "
                            <code>"cargo bench"</code>
                            " and saves stdout to "
                            <code>"/tmp/mentisdb_bench_results.txt"</code>
                            "."
                        </p>

                        <h3>"Durable vs Buffered Write Mode"</h3>
                        <p>
                            "Benchmark write-heavy changes in both persistence modes. They now behave differently enough that one run is not representative of the other:"
                        </p>
                        <ul>
                            <li>
                                <code>"MENTISDB_BENCH_AUTO_FLUSH=true"</code>
                                " — durable mode. Appends are queued to the bounded background writer and the request only returns after the writer flushes them. Concurrent appends may share a short group-commit window."
                            </li>
                            <li>
                                <code>"MENTISDB_BENCH_AUTO_FLUSH=false"</code>
                                " — buffered mode. Appends are queued and acknowledged before the worker flushes them, trading durability for higher write throughput."
                            </li>
                        </ul>
                        <div class="code-block">
                            <pre><code>
    "MENTISDB_BENCH_AUTO_FLUSH=true  cargo bench --bench http_concurrency\n\
    MENTISDB_BENCH_AUTO_FLUSH=false cargo bench --bench http_concurrency"
                            </code></pre>
                        </div>

                        <h3>"Criterion Baselines"</h3>
                        <p>
                            "Criterion benchmarks such as "
                            <code>"thought_chain"</code>
                            " and "
                            <code>"skill_registry"</code>
                            " automatically compare the current run against the saved baseline in "
                            <code>"target/criterion/"</code>
                            ". That is why the output includes messages like "
                            <code>"Performance has improved"</code>
                            ", "
                            <code>"Performance has regressed"</code>
                            ", or "
                            <code>"No change in performance detected"</code>
                            "."
                        </p>
                        <ul>
                            <li>
                                <code>"time: [low mid high]"</code>
                                " — lower is better"
                            </li>
                            <li>
                                <code>"thrpt: [low mid high]"</code>
                                " — higher is better"
                            </li>
                            <li>
                                <code>"change"</code>
                                " — percentage delta versus the saved baseline"
                            </li>
                            <li>
                                <code>"p &lt; 0.05"</code>
                                " — the change is statistically meaningful"
                            </li>
                        </ul>
                        <p>
                            "If you have changed the benchmark harness itself or want a fresh comparison, clear the old Criterion baseline before trusting the regression/improvement messages."
                        </p>

                        <h3>"HTTP Concurrency Baselines"</h3>
                        <p>
                            "The custom "
                            <code>"http_concurrency"</code>
                            " benchmark now persists its own baselines under "
                            <code>"target/http_concurrency/"</code>
                            " and prints delta tables on later runs. This means you no longer need to manually compare the Markdown output tables by eye."
                        </p>
                        <div class="code-block">
                            <pre><code>
    "MENTISDB_BENCH_AUTO_FLUSH=true  MENTISDB_BENCH_BASELINE=durable  cargo bench --bench http_concurrency\n\
    MENTISDB_BENCH_AUTO_FLUSH=false MENTISDB_BENCH_BASELINE=buffered cargo bench --bench http_concurrency"
                            </code></pre>
                        </div>
                        <p>
                            "Useful environment variables:"
                        </p>
                        <ul>
                            <li>
                                <code>"MENTISDB_BENCH_CONCURRENCY"</code>
                                " — comma-separated client counts such as "
                                <code>"100,1000,10000"</code>
                            </li>
                            <li>
                                <code>"MENTISDB_BENCH_AUTO_FLUSH"</code>
                                " — "
                                <code>"true"</code>
                                " for durable group commit, "
                                <code>"false"</code>
                                " for buffered throughput mode"
                            </li>
                            <li>
                                <code>"MENTISDB_BENCH_BASELINE"</code>
                                " — names the saved baseline file so you can keep separate durable, buffered, nightly, or branch-specific histories"
                            </li>
                        </ul>

                        <h3>"How to Read the HTTP Table"</h3>
                        <ul>
                            <li>
                                <code>"wall_ms"</code>
                                " — total time for the whole wave"
                            </li>
                            <li>
                                <code>"req/s"</code>
                                " — throughput"
                            </li>
                            <li>
                                <code>"p50_ms"</code>
                                ", "
                                <code>"p95_ms"</code>
                                ", "
                                <code>"p99_ms"</code>
                                " — median and tail latency"
                            </li>
                            <li>
                                <code>"errors"</code>
                                " — non-2xx responses or transport failures"
                            </li>
                        </ul>
                        <p>
                            "For write-path work, focus on the "
                            <code>"Write — POST /v1/thoughts"</code>
                            " table at high concurrency. Read-path changes should primarily move the "
                            <code>"Read — POST /v1/head"</code>
                            " table and the query/traversal Criterion benches."
                        </p>

                        <h3>"What to Benchmark After Storage Changes"</h3>
                        <ul>
                            <li>
                                <code>"cargo bench --bench thought_chain append_single -- --noplot"</code>
                                " — strict append latency"
                            </li>
                            <li>
                                <code>"cargo bench --bench thought_chain query_by_tag -- --noplot"</code>
                                " — indexed query path"
                            </li>
                            <li>
                                <code>"cargo bench --bench thought_chain traverse_filtered_miss_10 -- --noplot"</code>
                                " — full-scan traversal miss path"
                            </li>
                            <li>
                                <code>"cargo bench --bench http_concurrency"</code>
                                " — live daemon concurrency behavior"
                            </li>
                        </ul>
                        <p>
                            "A useful workflow is to measure a clean baseline on "
                            <code>"master"</code>
                            ", apply the storage change, rerun the same focused benches, and only then trust the wider "
                            <code>"make bench"</code>
                            " output."
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
