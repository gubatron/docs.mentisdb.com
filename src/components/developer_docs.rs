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
                            <code>"mentisdbd"</code>
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
                            "This ensures that pure semantic matches surface prominently, \
                             while thoughts already matched lexically are not artificially \
                             inflated by an additional vector boost."
                        </p>

                        <h3>"Importance Weight 3.0×"</h3>
                        <p>
                            "User-originated thoughts carry higher importance. At the default \
                             3.0× weight multiplier, a user thought (importance 0.8) contributes \
                             +2.4 to the final score versus an assistant thought (importance 0.2) \
                             at +0.6. This makes user-stated preferences and decisions rank \
                             above routine assistant summaries in retrieval results."
                        </p>

                        <h3>"Bincode Hashing"</h3>
                        <p>
                            <code>"compute_thought_hash()"</code>
                            " now uses bincode instead of "
                            <code>"serde_json"</code>
                            " for thought serialization, eliminating full JSON serialization \
                             per append. This reduces hash computation overhead and produces \
                             smaller, deterministic binary encodings. Existing chains auto-reindex \
                             on first open."
                        </p>

                        <h3>"ManagedSidecarEntry with auto_sync"</h3>
                        <p>
                            "Vector sidecar management uses "
                            <code>"ManagedSidecarEntry"</code>
                            " with an "
                            <code>"auto_sync"</code>
                            " flag that controls whether the sidecar rebuilds on startup. \
                             The new "
                            <code>"register_vector_sidecar_for_search()"</code>
                            " method registers a sidecar for search-only without triggering \
                             a rebuild, allowing faster daemon startup while keeping the \
                             sidecar available for ranked search fusion."
                        </p>

                        // ── 0.8.1 Improvements ──────────────────────────────
                        <h2 id="0.8.1">"0.8.1 Search Improvements"</h2>
                        <p>
                            "MentisDB 0.8.1 refines the scoring pipeline with session cohesion, \
                             smooth exponential fusion, and a tighter BM25 document-frequency cutoff. \
                             LongMemEval R@5 climbs from 65.0% to 67.6%; LoCoMo 2-persona R@10 \
                             reaches 88.7%."
                        </p>

                        <h3>"Session Cohesion Scoring"</h3>
                        <p>
                            "Thoughts within ±8 positions of a high-scoring lexical seed (score ≥ 3.0) \
                             receive a proximity boost up to 0.8, decaying linearly with distance. This \
                             surfaces evidence turns adjacent to the matching turn but sharing no lexical \
                             terms. The "
                            <code>"session_cohesion"</code>
                            " field is now included in ranked search score responses."
                        </p>

                        <h3>"Smooth Exponential Vector-Lexical Fusion"</h3>
                        <p>
                            "Replaces the 0.8.0 step-function tiers with a continuous decay: \
                             "
                            <code>"vector × (1 + 35 × exp(-lexical / 3.0))"</code>
                            ". Pure-semantic matches get ~36× amplification; by lexical = 3.0 the \
                             boost has decayed to ~12×; at lexical = 6.0 it is purely additive. \
                             This eliminates discontinuities between tiers."
                        </p>

                        <h3>"BM25 DF Cutoff 30%"</h3>
                        <p>
                            "Terms appearing in >30% of documents (corpus ≥ 20 docs) are skipped \
                             during scoring. This filters non-discriminative entity names without \
                             blanket stopword removal."
                        </p>

                        <h3>"NaN/Infinity Guard"</h3>
                        <p>
                            <code>"with_confidence()"</code>
                            " and "
                            <code>"with_importance()"</code>
                            " now reject non-finite floats. "
                            <code>"f32::NAN.clamp(0.0, 1.0)"</code>
                            " returns NaN in Rust, which previously crashed serde_json serialization \
                             when the dashboard tried to render affected thoughts."
                        </p>

                        // ── 0.8.2 Features ──────────────────────────────────
                        <h2 id="0.8.2">"0.8.2 Features"</h2>
                        <p>
                            "MentisDB 0.8.2 introduces four major features: temporal edge \
                             validity, memory deduplication, multi-level memory scopes, and \
                             CLI subcommands."
                        </p>

                        <h3>"Temporal Edge Validity"</h3>
                        <p>
                            <code>"ThoughtRelation"</code>
                            " now carries optional "
                            <code>"valid_at: Option&lt;DateTime&lt;Utc&gt;&gt;"</code>
                            " and "
                            <code>"invalid_at: Option&lt;DateTime&lt;Utc&gt;&gt;"</code>
                            " fields. A relation is considered active when the current time falls \
                             between these bounds. If neither is set the relation is always active \
                             (backward compatible with V2 chains). Schema V3 migration adds these \
                             fields transparently on first open."
                        </p>
                        <p>
                            "The "
                            <code>"as_of"</code>
                            " query parameter (RFC 3339 timestamp) is supported on ranked search, \
                             context bundles, and traversal endpoints. When provided, only thoughts \
                             appended at or before the timestamp are included in results. This \
                             enables point-in-time auditing and decision reproduction."
                        </p>
                        <p>
                            "When a relation's "
                            <code>"invalid_at"</code>
                            " has passed, the target thought is added to "
                            <code>"invalidated_thought_ids"</code>
                            " in the search response, allowing clients to filter or highlight \
                             stale edges."
                        </p>

                        <h3>"Memory Dedup"</h3>
                        <p>
                            "Automatic deduplication on append, controlled by two environment variables:"
                        </p>
                        <ul>
                            <li>
                                <code>"MENTISDB_DEDUP_THRESHOLD"</code>
                                " — Jaccard similarity threshold (0.0–1.0). When set, each append \
                                 compares the new thought's content against the last N thoughts. \
                                 If similarity exceeds the threshold, the new thought receives an \
                                 auto-"
                                <code>"Supersedes"</code>
                                " relation pointing at the most similar existing thought instead of \
                                 being appended as a duplicate. Disabled when unset."
                            </li>
                            <li>
                                <code>"MENTISDB_DEDUP_SCAN_WINDOW"</code>
                                " — how many recent thoughts to scan (default: 64). Only used when \
                                 MENTISDB_DEDUP_THRESHOLD is set."
                            </li>
                        </ul>
                        <p>
                            "The builder API exposes "
                            <code>".with_dedup_threshold(f64)"</code>
                            " and "
                            <code>".with_dedup_scan_window(usize)"</code>
                            " for programmatic control. When dedup fires, the append still succeeds \
                             but the resulting thought carries a "
                            <code>"Supersedes"</code>
                            " relation and the content is not duplicated in search results."
                        </p>

                        <h3>"Multi-Level Memory Scopes"</h3>
                        <p>
                            "The "
                            <code>"MemoryScope"</code>
                            " enum introduces three scope levels for thoughts within a single chain:"
                        </p>
                        <ul>
                            <li>
                                <code>"MemoryScope::User"</code>
                                " — globally visible across sessions (default, backward compatible)"
                            </li>
                            <li>
                                <code>"MemoryScope::Session"</code>
                                " — ephemeral working memory scoped to a single conversation"
                            </li>
                            <li>
                                <code>"MemoryScope::Agent"</code>
                                " — private to the authoring agent, not visible to other fleet members"
                            </li>
                        </ul>
                        <p>
                            "Scopes are stored as tags ("
                            <code>"scope:user"</code>
                            ", "
                            <code>"scope:session"</code>
                            ", "
                            <code>"scope:agent"</code>
                            ") on each thought. Set scope via "
                            <code>".with_scope(MemoryScope::Session)"</code>
                            " on the builder, or pass "
                            <code>"scope"</code>
                            " in MCP/REST append calls. Filter in search with the "
                            <code>"scope"</code>
                            " parameter."
                        </p>

                        <h3>"CLI Subcommands"</h3>
                        <p>
                            "The "
                            <code>"mentisdbd"</code>
                            " binary now supports inline subcommands for quick operations without \
                             an MCP client:"
                        </p>
                        <ul>
                            <li>
                                <code>"mentisdbd add \"content\""</code>
                                " — append a thought directly from the command line (uses "
                                <code>"ureq"</code>
                                " to POST to the local daemon)"
                            </li>
                            <li>
                                <code>"mentisdbd search \"query\" --limit 5"</code>
                                " — ranked search from the terminal"
                            </li>
                            <li>
                                <code>"mentisdbd agents"</code>
                                " — list registered agents across all chains"
                            </li>
                        </ul>
                        <p>
                            "These subcommands communicate with the running daemon over HTTP \
                             (via "
                            <code>"ureq"</code>
                            "), so the daemon must already be started. They are convenience \
                             shortcuts — all functionality remains available via MCP, REST, \
                             and the dashboard."
                        </p>

                        // ── Schema Version ───────────────────────────────────
                        <h2 id="schema">"Schema Version"</h2>
                        <p>
                            "MentisDB 0.8.2 uses schema version 3 ("
                            <code>"MENTISDB_SCHEMA_V3 = 3"</code>
                            "). V3 adds "
                            <code>"valid_at"</code>
                            " and "
                            <code>"invalid_at"</code>
                            " fields to "
                            <code>"ThoughtRelation"</code>
                            " for temporal edge validity. All new chains are created at V3 \
                             automatically. Legacy V2 chains (created before 0.8.2) are migrated \
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

                        // ── Thought Taxonomy ───────────────────────────────
                        <h2 id="taxonomy">"Thought Taxonomy"</h2>
                        <p>
                            "The canonical enum names below are accepted by the crate API, MCP \
                             tools, REST filters, and MEMORY.md import/export. "
                            <code>"ThoughtType"</code>
                            " carries semantic meaning; "
                            <code>"ThoughtRole"</code>
                            " carries operational meaning."
                        </p>
                        <p>
                            <strong>"ThoughtType (30): "</strong>
                            <code>"PreferenceUpdate"</code>
                            ", "
                            <code>"UserTrait"</code>
                            ", "
                            <code>"RelationshipUpdate"</code>
                            ", "
                            <code>"Finding"</code>
                            ", "
                            <code>"Insight"</code>
                            ", "
                            <code>"FactLearned"</code>
                            ", "
                            <code>"PatternDetected"</code>
                            ", "
                            <code>"Hypothesis"</code>
                            ", "
                            <code>"Mistake"</code>
                            ", "
                            <code>"Correction"</code>
                            ", "
                            <code>"LessonLearned"</code>
                            ", "
                            <code>"AssumptionInvalidated"</code>
                            ", "
                            <code>"Constraint"</code>
                            ", "
                            <code>"Goal"</code>
                            ", "
                            <code>"Plan"</code>
                            ", "
                            <code>"Subgoal"</code>
                            ", "
                            <code>"Decision"</code>
                            ", "
                            <code>"StrategyShift"</code>
                            ", "
                            <code>"Wonder"</code>
                            ", "
                            <code>"Question"</code>
                            ", "
                            <code>"Idea"</code>
                            ", "
                            <code>"Experiment"</code>
                            ", "
                            <code>"ActionTaken"</code>
                            ", "
                            <code>"TaskComplete"</code>
                            ", "
                            <code>"Checkpoint"</code>
                            ", "
                            <code>"StateSnapshot"</code>
                            ", "
                            <code>"Handoff"</code>
                            ", "
                            <code>"Summary"</code>
                            ", "
                            <code>"Surprise"</code>
                            ", and "
                            <code>"Reframe"</code>
                            "."
                        </p>
                        <p>
                            <strong>"ThoughtRole (8): "</strong>
                            <code>"Memory"</code>
                            ", "
                            <code>"WorkingMemory"</code>
                            ", "
                            <code>"Summary"</code>
                            ", "
                            <code>"Compression"</code>
                            ", "
                            <code>"Checkpoint"</code>
                            ", "
                            <code>"Handoff"</code>
                            ", "
                            <code>"Audit"</code>
                            ", and "
                            <code>"Retrospective"</code>
                            "."
                        </p>
                        <p>
                            "The Agent Guide contains the human-facing explanation of when to \
                             use each ThoughtType and ThoughtRole; docs.rs is the authoritative \
                             source for the Rust enum definitions and builder APIs."
                        </p>

                        <h3>"MemoryScope"</h3>
                        <p>
                            "0.8.2 introduces "
                            <code>"MemoryScope"</code>
                            " — a visibility partition within a single chain. Scopes are stored \
                             as tags on each thought:"
                        </p>
                        <ul>
                            <li>
                                <code>"MemoryScope::User"</code>
                                " — globally visible (default). Tag: "
                                <code>"scope:user"</code>
                            </li>
                            <li>
                                <code>"MemoryScope::Session"</code>
                                " — ephemeral, scoped to one conversation. Tag: "
                                <code>"scope:session"</code>
                            </li>
                            <li>
                                <code>"MemoryScope::Agent"</code>
                                " — private to the authoring agent. Tag: "
                                <code>"scope:agent"</code>
                            </li>
                        </ul>
                        <p>
                            "Set scope on the builder with "
                            <code>".with_scope(MemoryScope::Session)"</code>
                            " or pass "
                            <code>"scope"</code>
                            " in MCP/REST append calls. Filter in search with the "
                            <code>"scope"</code>
                            " parameter. Existing thoughts without a scope tag are treated as "
                            <code>"User"</code>
                            "-scoped — no migration required."
                        </p>

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
        valid_at: None,\n\
        invalid_at: None,\n\
    });\n\
    \n\
    // Relation with temporal bounds:\n\
    let input = ThoughtInput::new(ThoughtType::Decision, \"Adopted the caching strategy.\")\n\
    .with_relation(ThoughtRelation {\n\
        kind: ThoughtRelationKind::Supersedes,\n\
        target_id: old_thought_uuid,\n\
        chain_key: None,\n\
        valid_at: Some(\"2025-12-01T00:00:00Z\".parse().unwrap()),\n\
        invalid_at: None,\n\
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

                        <h3>"All relation kinds"</h3>
                        <p>
                            <code>"ThoughtRelationKind"</code>
                            " variants accepted by both the Rust API and "
                            <code>"POST /v1/thoughts"</code>
                            ":"
                        </p>
                        <ul>
                            <li><code>"References"</code>" — generic pointer to another thought"</li>
                            <li><code>"Summarizes"</code>" — this thought condenses the target"</li>
                            <li><code>"Corrects"</code>" — fixes a factual error in the target"</li>
                            <li><code>"Invalidates"</code>" — target is no longer applicable"</li>
                            <li><code>"CausedBy"</code>" — this thought resulted from the target"</li>
                            <li><code>"Supports"</code>" — this thought provides evidence for the target"</li>
                            <li><code>"Contradicts"</code>" — this thought conflicts with the target"</li>
                            <li><code>"DerivedFrom"</code>" — this thought is derived from the target"</li>
                            <li><code>"ContinuesFrom"</code>" — sequential continuation; used to chain consecutive session turns"</li>
                            <li><code>"RelatedTo"</code>" — weak associative link"</li>
                            <li><code>"Supersedes"</code>" — replaces the target's framing or approach"</li>
                        </ul>

                        <h3>"REST — relations in POST /v1/thoughts"</h3>
                        <p>
                            "Pass "
                            <code>"relations"</code>
                            " as a JSON array in the request body. \
                             Each element has a "
                            <code>"kind"</code>
                            " string, a "
                            <code>"target_id"</code>
                            " (UUID), and an optional "
                            <code>"chain_key"</code>
                            " for cross-chain edges:"
                        </p>
                        <div class="code-block">
                            <pre><code>
    "POST /v1/thoughts\n\
    {\n\
      \"agent_id\":     \"planner\",\n\
      \"thought_type\": \"Observation\",\n\
      \"content\":      \"Retry logic resolved the timeout — root cause was DNS.\",\n\
      \"refs\":         [41],\n\
      \"relations\": [\n\
        { \"kind\": \"ContinuesFrom\", \"target_id\": \"<uuid-of-prior-turn>\" },\n\
        { \"kind\": \"CausedBy\",      \"target_id\": \"<uuid>\", \"chain_key\": \"infra-ops\" }\n\
      ]\n\
    }"
                            </code></pre>
                        </div>

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
                                " — Default (and only supported adapter for new chains). \
                                 Compact binary format with write buffering. \
                                 Best for production."
                            </li>
                        </ul>
                        <p>
                            "Implement the "
                            <code>"StorageAdapter"</code>
                            " trait to plug in your own backend (S3, SQLite, etc.). \
                             See docs.rs for the trait definition."
                        </p>

                        // ── 0.8.6 Features ──────────────────────────────────
                        <h2 id="0.8.6">"0.8.6 Features"</h2>
                        <p>
                            "MentisDB 0.8.6 adds three retrieval features and a chain branching primitive:"
                        </p>

                        <h3>"RRF Reranking"</h3>
                        <p>
                            "Reciprocal Rank Fusion (RRF) is an opt-in reranking pass on ranked search. \
                             When "
                            <code>"enable_reranking"</code>
                            " is set, the engine produces separate lexical-only, vector-only, and \
                             graph-only rankings over the top "
                            <code>"rerank_k"</code>
                            " candidates (default 50), merges them via "
                            <code>"1/(k + rank)"</code>
                            " with k=60, and replaces the additive total with the RRF total. \
                             Non-rankable signals (importance, confidence, recency, session cohesion) \
                             are added back as small adjustments. Use RRF when lexical and vector \
                             signals disagree on top candidates."
                        </p>

                        <h3>"Irregular Verb Lemma Expansion"</h3>
                        <p>
                            "The query tokenizer now expands irregular English verbs to their base form \
                             (e.g. \"went\" → \"go\", \"gave\" → \"give\", \"ran\" → \"run\"). About \
                             170 mappings are included. Indexed content is not modified — expansion is \
                             query-time only."
                        </p>

                        <h3>"Memory Chain Branching"</h3>
                        <p>
                            "New "
                            <code>"ThoughtRelationKind::BranchesFrom"</code>
                            " enables cross-chain divergence. "
                            <code>"MentisDb::branch_from()"</code>
                            " creates a new chain with a genesis thought pointing back to the \
                             branch-point on the source chain. When searching a branch chain, \
                             the server transparently searches ancestor chains and merges results, \
                             annotated with "
                            <code>"chain_key"</code>
                            ". REST: "
                            <code>"POST /v1/chains/branch"</code>
                            ". MCP: "
                            <code>"mentisdb_branch_from"</code>
                            "."
                        </p>

                        <h3>"Per-Field BM25 DF Cutoffs"</h3>
                        <p>
                            "The "
                            <code>"Bm25DfCutoffs"</code>
                            " struct on "
                            <code>"LexicalQuery"</code>
                            " allows configuring separate document-frequency cutoff ratios per field \
                             (content, tags, concepts, agent_id, agent_registry). Terms whose global \
                             DF exceeds the cutoff for a given field are skipped for that field only. \
                             Default cutoffs: content=0.30, tags=0.30, concepts=0.30, agent_id=0.70, \
                             agent_registry=0.60."
                        </p>

                        // ── 0.9.x Features ──────────────────────────────────
                        <h2 id="0.9.x">"0.9.x Features"</h2>
                        <p>
                            "MentisDB 0.9.x adds four major features: LLM extraction, federated \
                             cross-chain search, an official Python client, and an improved setup experience."
                        </p>

                        <h3>"Opt-in LLM Extraction (0.9.1)"</h3>
                        <p>
                            "The "
                            <code>"extract_memories_from_text"</code>
                            " function uses GPT-4o (or any OpenAI-compatible endpoint) to convert raw \
                             agent text, conversation logs, or reasoning traces into structured \
                             "
                            <code>"ThoughtInput"</code>
                            " records. Extraction is opt-in behind the "
                            <code>"llm-extraction"</code>
                            " feature (enabled by default). Uses openai-rust2 with 3x retry on 429/5xx, \
                             60s timeout, temperature 0.1. Configure via "
                            <code>"OPENAI_API_KEY"</code>
                            ", "
                            <code>"LLM_BASE_URL"</code>
                            ", and "
                            <code>"LLM_MODEL"</code>
                            " env vars. The pipeline returns raw "
                            <code>"ExtractionResult"</code>
                            " — callers must review and validate thoughts before appending to protect \
                             the hash chain from untrusted LLM output."
                        </p>

                        <h3>"Federated Cross-Chain Search (0.9.1)"</h3>
                        <p>
                            "The "
                            <code>"BranchesFrom"</code>
                            " primitive enables fleet memory hierarchies. A project chain branches \
                             from a company-wide knowledge chain at a checkpoint. Ranked search on \
                             the branch transparently queries both the local chain and ancestor \
                             chains, annotated with "
                            <code>"chain_key"</code>
                            " so you know where each result originated. REST: \
                             "
                            <code>"POST /v1/chains/branch"</code>
                            ". MCP: "
                            <code>"mentisdb_branch_from"</code>
                            "."
                        </p>

                        <h3>"Python Client — pymentisdb (0.9.1)"</h3>
                        <p>
                            "pymentisdb is on PyPI. Full "
                            <code>"MentisDbClient"</code>
                            " with typed enums, LangChain "
                            <code>"MentisDbMemory"</code>
                            ", webhook management, skill registry, and memory import/export. Install:"
                        </p>
                        <div class="code-block">
                            <code>"pip install pymentisdb"</code>
                        </div>
                        <p>
                            "Or with LangChain:"
                        </p>
                        <div class="code-block">
                            <code>"pip install pymentisdb[langchain]"</code>
                        </div>

                        <h3>"Webhook Callbacks (0.9.1)"</h3>
                        <p>
                            "Register HTTP POST callbacks that fire when thoughts are appended. \
                             Useful for reactive agents, cross-system sync, or audit logging. REST: \
                             "
                            <code>"POST /v1/webhooks"</code>
                            ". MCP: "
                            <code>"mentisdb_register_webhook"</code>
                            ", "
                            <code>"mentisdb_list_webhooks"</code>
                            ", "
                            <code>"mentisdb_delete_webhook"</code>
                            ". Delivery is fire-and-forget with exponential backoff retries."
                        </p>

                        <h3>"Smart Stdio Mode — Auto-Detect Daemon (0.9.2)"</h3>
                        <p>
                            "The stdio MCP mode now detects if a daemon is already running on the \
                             configured MCP port via the health endpoint. If running, the stdio \
                             process acts as a lightweight proxy forwarding requests to the daemon's \
                             HTTP MCP endpoints. If not running, it launches the daemon in the \
                             background (nohup on Unix, start /B on Windows) in headless HTTP mode \
                             and proxies. This means Claude Desktop users get shared live state with \
                             zero configuration — just set the command to "
                            <code>"mentisdbd"</code>
                            " with args "
                            <code>"--mode stdio"</code>
                            "."
                        </p>

                        <h3>"Dashboard Skill Editing (0.9.7)"</h3>
                        <p>
                            "The web dashboard can now edit skills from both the Skills table and \
                             a skill detail page. Saving creates a new immutable skill version \
                             through the existing upload path, preserving audit history instead of \
                             mutating prior content."
                        </p>

                        <h3>"Wizard Brew-First Setup (0.9.1)"</h3>
                        <p>
                            "The interactive setup wizard "
                            <code>"mentisdbd setup"</code>
                            " now tries "
                            <code>"brew install mcp-remote"</code>
                            " before npm. It detects Homebrew-installed mcp-remote (which has a \
                             proper shebang pointing to the correct Node version) and writes the \
                             minimal Claude Desktop config automatically."
                        </p>

                        <h3>"Custom Ontology — entity_type (0.8.7)"</h3>
                        <p>
                            "Thoughts can carry an optional "
                            <code>"entity_type"</code>
                            " label (e.g. \"bug_report\", \"architecture_decision\"). The \
                             "
                            <code>"ThoughtQuery"</code>
                            " accepts "
                            <code>"entity_type"</code>
                            " as a filter. Entity types are auto-observed per chain and \
                             persisted in a "
                            <code>"chain_key-entity-types.json"</code>
                            " sidecar."
                        </p>

                        <h3>"Episode Provenance — source_episode (0.8.8)"</h3>
                        <p>
                            "Thoughts can carry a "
                            <code>"source_episode"</code>
                            " field pointing to the original thought, with a new \
                             "
                            <code>"DerivedFrom"</code>
                            " relation kind. The dashboard shows full provenance graphs."
                        </p>

                        <h3>"LLM-Based Reranking (0.8.8)"</h3>
                        <p>
                            "In addition to arithmetic RRF, "
                            <code>"RankedSearchQuery"</code>
                            " accepts an optional "
                            <code>"llm_rerank"</code>
                            " block that delegates the final rerank of the top-"
                            <code>"rerank_k"</code>
                            " candidates to an OpenAI-compatible model. The server sends the candidate \
                             thoughts and the original query, parses the model's ranked IDs, and blends \
                             the returned ordinal score (under the "
                            <code>"llm_score"</code>
                            " field on each hit) with the arithmetic signals as an additive tie-breaker. \
                             LLM reranking is strictly opt-in and disabled by default; the core pipeline \
                             remains LLM-free."
                        </p>

                        // ── Operations & Admin ─────────────────────────────
                        <h2 id="operations">"Operations & Admin"</h2>

                        <p>
                            "For the full list of environment variables that tune storage, \
                             behaviour, networking, TLS, the dashboard, updates, and audio \
                             cues, see the "
                            <a href="/user#environment-variables">
                                "Environment Variables Reference"
                            </a>
                            " in the user manual — every "
                            <code>"MENTISDB_*"</code>
                            " knob is documented there with defaults, purpose, and a concrete example."
                        </p>

                        <h3 id="admin-flush">"POST /v1/admin/flush"</h3>
                        <p>
                            "REST endpoint that walks every open chain and calls "
                            <code>"flush()"</code>
                            " on its storage adapter. Useful in buffered-write mode ("
                            <code>"MENTISDB_AUTO_FLUSH=false"</code>
                            ") where up to "
                            <code>"FLUSH_THRESHOLD − 1"</code>
                            " records can sit in the background-writer queue. The backup CLI calls this "
                            "automatically when it detects a running daemon; consistent on-disk state is \
                             a precondition for a reliable "<code>".mentis"</code>" archive."
                        </p>

                        <h3 id="backup-restore">"Backup & Restore (.mentis archive)"</h3>
                        <p>
                            <code>"mentisdbd backup"</code>
                            " produces a "<code>".mentis"</code>" ZIP archive covering every chain data file \
                             ("<code>"*.tcbin"</code>", "<code>"*.agents.json"</code>", "
                            <code>"*.entity-types.json"</code>", "<code>"*.vectors.*.json"</code>"), the \
                             global chain registry, the skill registry, and optionally TLS certificates. \
                             When a daemon is running the CLI first POSTs to "
                            <code>"/v1/admin/flush"</code>
                            " so pending writes are on disk before the archive is assembled."
                        </p>
                        <p>
                            <code>"mentisdbd restore"</code>
                            " reverses the operation. Path traversal is rejected server-side — manifest \
                             entries containing "<code>"../"</code>" or absolute paths fail with "
                            <code>"InvalidData"</code>
                            " before any file is written outside the target directory. Existing files are \
                             preserved by default; pass "<code>"--overwrite"</code>" to replace them, or \
                             answer "<code>"yes"</code>" to the interactive confirmation."
                        </p>

                        <h3 id="dashboard-pin">"Dashboard PIN"</h3>
                        <p>
                            "Set "<code>"MENTISDB_DASHBOARD_PIN"</code>" to require a shared PIN before the \
                             web dashboard accepts any request. An empty string is treated as absent. \
                             Failed logins are rate-limited per-IP to prevent brute-force enumeration; \
                             the dashboard is always served over TLS so the PIN is never transmitted in \
                             the clear on a local network."
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
                                <code>"mentisdbd"</code>
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
