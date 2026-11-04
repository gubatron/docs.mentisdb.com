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
                            <a href="#0.12"              class="docs-nav-link">"0.12 Library Changes"</a>
                            <a href="#0.11"              class="docs-nav-link">"0.11 Library Changes"</a>
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
                            <code>r#"mentisdb = "0.12.0""#</code>
                        </div>
                        <p>
                            "The minimum supported Rust version is 1.90 ("
                            <code>"rust-version"</code>
                            " in the crate manifest; CI checks it)."
                        </p>
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

                        <h3>"All 45 MCP Tools"</h3>
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
                                    <td><code>"chain_key"</code>, <code>"text"</code>, <code>"as_of"</code>, <code>"scope"</code>, <code>"enable_reranking"</code>, <code>"rerank_k"</code>, <code>"concepts_any"</code>, <code>"roles"</code>, <code>"limit"</code>, <code>"compact"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_federated_search"</code></td>
                                    <td>"Ranked retrieval over several chains at once, merged into one list. Each chain is authorized and filtered for the caller on its own"</td>
                                    <td><code>"chain_keys"</code>, <code>"text"</code>, <code>"limit"</code>, <code>"graph"</code>, <code>"as_of"</code>, <code>"include_invalidated"</code>, <code>"compact"</code></td>
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
                                    <td><code>"mentisdb_migration_status"</code></td>
                                    <td>"Report the background storage-format migration state of every chain that is, was, or failed to be migrated: phase, percent done, attempts and error. Global token only"</td>
                                    <td></td>
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
                                    <td><code>"mentisdb_get_access_policy"</code></td>
                                    <td>"Return the chain's access policy as the caller may see it: mode, sequence, key-set commitment and segment boundaries for everyone; the allowed keys and record hashes only for a reader who proved an authorized key"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_verify_signatures"</code></td>
                                    <td>"Re-verify the signed thoughts and governance records the caller may see; per-thought status (valid, unsigned, invalid), signature version and totals"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_standing_context"</code></td>
                                    <td>"Render durable, age-independent memory (constraints, preferences, user traits, goals, decisions, lessons, newest handoff and checkpoint) as a prompt block. Call at session start before mentisdb_recent_context"</td>
                                    <td><code>"chain_key"</code>, <code>"agent_id"</code>, <code>"types"</code>, <code>"since"</code>, <code>"as_of"</code>, <code>"include_invalidated"</code>, <code>"per_type_limit"</code>, <code>"limit"</code></td>
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
                                    <td><code>"mentisdb_delete_skill"</code></td>
                                    <td>"Permanently remove a skill and all of its versions. After delete the same skill_id can be uploaded again. Prefer revoke when you need an audit row."</td>
                                    <td><code>"skill_id"</code>, <code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_head"</code></td>
                                    <td>"Return head metadata, the latest thought at the current chain tip, and integrity state"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_summary_candidates"</code></td>
                                    <td>"Suggest groups of thoughts that are candidates for summarization"</td>
                                    <td><code>"chain_key"</code>, <code>"text"</code>, <code>"limit"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_register_webhook"</code></td>
                                    <td>"Register a webhook to receive HTTP POST notifications when thoughts are appended. Delivery is fire-and-forget with exponential backoff (up to 3 attempts)"</td>
                                    <td><code>"url"</code>, <code>"chain_key_filter"</code>, <code>"thought_type_filter"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_list_webhooks"</code></td>
                                    <td>"List all registered webhooks"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_delete_webhook"</code></td>
                                    <td>"Remove a webhook registration by its UUID"</td>
                                    <td><code>"id"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_list_entity_types"</code></td>
                                    <td>"List the entity types registered on a chain"</td>
                                    <td><code>"chain_key"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_upsert_entity_type"</code></td>
                                    <td>"Create or update an entity type in the per-chain registry"</td>
                                    <td><code>"chain_key"</code>, <code>"entity_type"</code></td>
                                </tr>
                                <tr>
                                    <td><code>"mentisdb_extract_memories"</code></td>
                                    <td>"Turn free-form text into typed memory candidates with the configured LLM. Nothing is appended; needs the llm-extraction feature"</td>
                                    <td><code>"text"</code>, <code>"chain_key"</code>, <code>"agent_id"</code>, <code>"prompt_template"</code></td>
                                </tr>
                            </tbody>
                        </table>
                        <p>
                            "Since 0.11, registering or revoking agent keys, disabling agents and \
                             changing a chain's access policy are "
                            <strong>"operator-only"</strong>
                            ": "
                            <code>"mentisdb_add_agent_key"</code>
                            ", "
                            <code>"mentisdb_revoke_agent_key"</code>
                            ", "
                            <code>"mentisdb_disable_agent"</code>
                            " and "
                            <code>"mentisdb_set_access_policy"</code>
                            " are no longer offered, and a "
                            <code>"status"</code>
                            " change through "
                            <code>"mentisdb_upsert_agent"</code>
                            " is refused (restating the current status is still accepted). \
                             The operator uses the dashboard behind its PIN or the local "
                            <code>"mentisdb"</code>
                            " command line; see "
                            <a href="/user#public-private-chains">"Public & Private Chains"</a>
                            ". A tool call against a chain that is still being migrated returns a \
                             tool error with the retry delay and percent done; retry after it."
                        </p>

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
                        <p>
                            "The legacy CloudLLM endpoint "
                            <code>"POST /tools/execute"</code>
                            " takes "
                            <code>"tool"</code>
                            " + "
                            <code>"parameters"</code>
                            "; JSON-RPC "
                            <code>"tools/call"</code>
                            " takes "
                            <code>"name"</code>
                            " + "
                            <code>"arguments"</code>
                            ". Authorization checks exactly the fields the transport executes, and a \
                             payload that mixes both shapes is rejected with 400."
                        </p>

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
                                    <td>"Canonical flat ranked retrieval with lexical + vector + graph-aware score breakdowns (hybrid when managed sidecars are available). Supports as_of, scope, enable_reranking, and rerank_k parameters, and the ranking knobs resolve_successors, collapse_duplicates, vector_fusion, synonym_weight, df_scope and df_cutoffs (bad values are a 400). Pass compact: true for smaller hits"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/context-bundles"</code></td>
                                    <td>"Seed-anchored grouped support context for agent reasoning and dashboard inspection. Supports as_of and scope parameters"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/v1/chains"</code></td>
                                    <td>"List available chain keys; each chain summary carries "<code>"derived_backlog"</code>" (0.12)"</td>
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
                                    <td><code>"/v1/standing-context"</code></td>
                                    <td>"Durable, age-independent memory as a prompt block plus typed entries (entries, duplicates_collapsed, prompt). Load at session start before recent context"</td>
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
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/federated-search"</code></td>
                                    <td>"Ranked search fused across several chains (chain_keys). Accepts compact: true"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/summary-candidates"</code></td>
                                    <td>"Suggest groups of thoughts that are candidates for summarization"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/extract-memories"</code></td>
                                    <td>"Turn raw text into typed memory candidates with the configured LLM"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/entity-types"</code></td>
                                    <td>"List entity types defined on a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/entity-types/upsert"</code></td>
                                    <td>"Create or update an entity type on a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/v1/webhooks"</code></td>
                                    <td>"List registered webhooks"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/webhooks"</code></td>
                                    <td>"Register a webhook (url, optional chain_key_filter, optional thought_type_filter)"</td>
                                </tr>
                                <tr>
                                    <td><code>"DELETE"</code></td>
                                    <td><code>"/v1/webhooks/{id}"</code></td>
                                    <td>"Delete a webhook by id"</td>
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
                                    <td><code>"/v1/skills/delete"</code></td>
                                    <td>"Permanently delete a skill and all versions so the skill_id can be reused"</td>
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
                                    <td>"Answers 403 (operator-only since 0.11): register keys from the dashboard or with mentisdb agent key add"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/keys/revoke"</code></td>
                                    <td>"Answers 403 (operator-only since 0.11): mentisdb agent key revoke"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/agents/disable"</code></td>
                                    <td>"Answers 403 (operator-only since 0.11): mentisdb agent disable / activate"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/vectors/rebuild"</code></td>
                                    <td>"Rebuild vector sidecar indexes for a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/chains/merge"</code></td>
                                    <td>"Merge all thoughts from a source chain into a target, then delete the source. Refused for a source with private segments, governance records or signed thoughts"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/access/policy"</code></td>
                                    <td>"A chain's access policy as the caller may see it: mode, sequence, key-set commitment and segment boundaries; keys and record hashes only for an authorized reader"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/access/verify"</code></td>
                                    <td>"Re-verify the signatures and governance records the caller may see"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/access/policy/set"</code></td>
                                    <td>"Answers 403 (operator-only since 0.11): mentisdb chain access set|allow|deny"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/v1/migrations"</code></td>
                                    <td>"Background migration state of every chain (global token)"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/migrations/retry"</code></td>
                                    <td>"Clear a chain's migration quarantine so it is migrated again; body chain_key (global token)"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/migrations/skip"</code></td>
                                    <td>"Leave a chain unmigrated and unavailable until retried; body chain_key (global token)"</td>
                                </tr>
                            </tbody>
                        </table>

                        <h3 id="rest-errors">"Chain Resolution and Error Statuses"</h3>
                        <p>
                            "Only writes create a chain whose "
                            <code>"chain_key"</code>
                            " does not exist yet: "
                            <code>"/v1/bootstrap"</code>
                            ", "
                            <code>"/v1/thoughts"</code>
                            ", "
                            <code>"/v1/retrospectives"</code>
                            ", "
                            <code>"/v1/import-markdown"</code>
                            ", "
                            <code>"/v1/agents/upsert"</code>
                            ", "
                            <code>"/v1/entity-types/upsert"</code>
                            " and the target of "
                            <code>"/v1/chains/merge"</code>
                            ". Every other route returns 404 for an unknown chain and persists nothing; \
                             the configured default chain is the only one created on demand for reads. \
                             Chain keys must be non-empty, at most 256 bytes and free of control \
                             characters (400 otherwise). The source, target and branch keys of "
                            <code>"/v1/chains/merge"</code>
                            " and "
                            <code>"/v1/chains/branch"</code>
                            " are trimmed of surrounding whitespace before that check. The MCP tools \
                             behave the same way."
                        </p>
                        <table class="api-table">
                            <thead>
                                <tr>
                                    <th>"Status"</th>
                                    <th>"Meaning"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr><td><code>"400"</code></td><td>"Invalid input: malformed JSON, bad UUID, unknown thought type or role, invalid chain key, an unknown ranking-knob value, a v2-signed append built on an older head (fetch the head and sign again), a result window (offset + limit) above 10,000 on ranked, lexical or federated search or context bundles, or a standing-context per_type_limit above 100"</td></tr>
                                <tr><td><code>"401"</code></td><td>"Missing or invalid bearer token, a token whose scope does not cover the operation, or an X-Mentis-Auth read proof that is present but invalid (bad signature, clock more than 60 s off, reused nonce)"</td></tr>
                                <tr><td><code>"403"</code></td><td>"Permission denied: missing or invalid signature on a signed skill upload, a write the chain's access policy does not accept (the body does not say whether the agent or key exists), or an operator-only route"</td></tr>
                                <tr><td><code>"404"</code></td><td>"Unknown chain or other missing resource"</td></tr>
                                <tr><td><code>"408"</code></td><td>"A request body stopped arriving for MENTISDB_BODY_IDLE_TIMEOUT_SECS, or a small or anonymous body missed its total deadline (twice that)"</td></tr>
                                <tr><td><code>"409"</code></td><td>"Conflict, e.g. branching into a chain key that already exists"</td></tr>
                                <tr><td><code>"413"</code></td><td>"Request body over the route limit (2 MiB; MENTISDB_MAX_BODY_BYTES, default 64 MiB, on the import routes for an authenticated caller), or over 1 MiB for an X-Mentis-Auth read proof that does not also send X-Mentis-Content-Sha256 (the SHA-256 the proof signs; the CLI and pymentisdb send it, so a large memory import works)"</td></tr>
                                <tr><td><code>"500"</code></td><td>"Server fault; the body says only \"internal server error\" and details go to the daemon log"</td></tr>
                                <tr><td><code>"501"</code></td><td>"/v1/extract-memories on a build without the llm-extraction feature"</td></tr>
                                <tr><td><code>"503"</code></td><td>"The chain is being migrated (Retry-After header, retry_after_secs and percent done in the body) or failed to migrate (the body names the fix, e.g. mentisdb migrate retry --chain KEY); also, with Retry-After, when the in-flight body budget is exhausted"</td></tr>
                            </tbody>
                        </table>
                        <p>
                            "With bearer-token access enabled, "
                            <code>"GET /v1/chains"</code>
                            ", "
                            <code>"GET /v1/webhooks"</code>
                            ", "
                            <code>"DELETE /v1/webhooks/{id}"</code>
                            ", "
                            <code>"/v1/skills/{upload,deprecate,revoke,delete}"</code>
                            " and "
                            <code>"/v1/admin/flush"</code>
                            " require a global token; "
                            <code>"POST /v1/webhooks"</code>
                            " with a chain-scoped token requires a "
                            <code>"chain_key_filter"</code>
                            " inside its scope."
                        </p>
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

                        <h3>"REST Example: compact ranked search"</h3>
                        <p>
                            "Ranked and federated search accept an opt-in "<code>"compact: true"</code>
                            " (MCP "<code>"mentisdb_ranked_search"</code>" / "<code>"mentisdb_federated_search"</code>
                            ", REST "<code>"/v1/ranked-search"</code>" / "<code>"/v1/federated-search"</code>
                            ", CLI "<code>"--compact"</code>"). A compact hit is "<code>"chain_key"</code>
                            ", the total "<code>"score"</code>" as a number and the thought without "
                            <code>"hash"</code>", "<code>"prev_hash"</code>", "<code>"thought_signature"</code>
                            ", "<code>"signing_key_id"</code>", "<code>"schema_version"</code>", "<code>"agent_name"</code>
                            ", "<code>"agent_owner"</code>" and top-level null or empty fields; "
                            <code>"matched_terms"</code>", "<code>"match_sources"</code>" and the score components are left out. "
                            "The "<code>"provenance"</code>" object, the version fields ("<code>"version_status"</code>
                            ", "<code>"successors"</code>", "<code>"current_version"</code>", "<code>"successor_of"</code>
                            ", "<code>"collapsed_duplicates"</code>") and graph provenance are always kept, and the "
                            "public/private boundary is applied before compaction. Fields are dropped by name from a "
                            "fixed list, so any field added to a thought later is kept by default. "
                            "Skip it when you need hashes or signatures to verify a thought or per-component scores; "
                            <code>"POST /v1/thought"</code>" returns the full record of any hit."
                        </p>
                        <div class="code-block">
                            <pre><code>{r#"POST /v1/ranked-search
{"chain_key": "project", "text": "job queue storage", "limit": 5, "compact": true}

// Full response (default)
{
  "backend": "hybrid",
  "total": 1,
  "results": [
    {
      "chain_key": "project",
      "score": {
        "confidence": 0.0, "graph": 0.0, "importance": 0.17541419, "lexical": 1.9490464,
        "recency": 0.05, "relation": 0.0, "rrf": 0.0, "seed_support": 0.0,
        "session_cohesion": 0.0, "total": 5.932654, "vector": 0.34079674
      },
      "matched_terms": ["job", "queue", "storag"],
      "match_sources": ["content", "tags", "concepts"],
      "graph_distance": null, "graph_path": null,
      "graph_relation_kinds": [], "graph_seed_paths": 0,
      "thought": {
        "agent_id": "builder", "agent_name": "Builder", "agent_owner": "platform",
        "concepts": ["job queue"], "confidence": null,
        "content": "Use Postgres for the job queue; revisit if throughput exceeds 5k jobs per second.",
        "entity_type": null,
        "hash": "31de8f264b1588a29be30a278a24e8623cf51b8f3ce8483a7e52f69ce80e21e1",
        "id": "1182d44f-3834-4ff0-9121-eab986b1a4f5",
        "importance": 0.800000011920929, "index": 0, "prev_hash": "",
        "provenance": {
          "segment": "public", "signature": "unsigned", "signature_version": null,
          "signed": false, "trust": "untrusted_data", "verified_agent": null
        },
        "refs": [], "relations": [], "role": "Memory", "schema_version": 3,
        "session_id": null, "signing_key_id": null, "source_episode": null,
        "tags": ["storage"], "thought_signature": null, "thought_type": "Decision",
        "timestamp": "2026-11-01T23:03:19.669167Z"
      }
    }
  ]
}

// Same request with "compact": true
{
  "backend": "hybrid",
  "total": 1,
  "results": [
    {
      "chain_key": "project",
      "score": 5.932654,
      "thought": {
        "agent_id": "builder",
        "concepts": ["job queue"],
        "content": "Use Postgres for the job queue; revisit if throughput exceeds 5k jobs per second.",
        "id": "1182d44f-3834-4ff0-9121-eab986b1a4f5",
        "importance": 0.800000011920929, "index": 0,
        "provenance": {
          "segment": "public", "signature": "unsigned", "signature_version": null,
          "signed": false, "trust": "untrusted_data", "verified_agent": null
        },
        "role": "Memory", "tags": ["storage"], "thought_type": "Decision",
        "timestamp": "2026-11-01T23:03:19.669167Z"
      }
    }
  ]
}"#}</code></pre>
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

                        <h3>"REST Example: POST /v1/webhooks"</h3>
                        <div class="code-block">
                            <pre><code>
    "POST /v1/webhooks HTTP/1.1\n\
    Host: 127.0.0.1:9472\n\
    Content-Type: application/json\n\
    \n\
    {\n\
    \"url\": \"https://myapp.example.com/mentisdb-webhook\",\n\
    \"chain_key_filter\": \"default\",\n\
    \"thought_type_filter\": [\"Decision\", \"Constraint\"]\n\
    }\n\
    \n\
    // Response:\n\
    {\n\
    \"webhook\": {\n\
    \"id\": \"6f1c2a9e-...\",\n\
    \"url\": \"https://myapp.example.com/mentisdb-webhook\",\n\
    \"chain_key_filter\": \"default\",\n\
    \"thought_type_filter\": [\"Decision\", \"Constraint\"],\n\
    \"created_at\": \"2026-04-14T12:00:00Z\",\n\
    \"active\": true\n\
    }\n\
    }"</code></pre>
                        </div>
                        <p>
                            "Targets must be public http or https endpoints: loopback, private, link-local, \
                             CGNAT, documentation, multicast and IPv4-embedding IPv6 addresses are rejected, \
                             also after DNS resolution, and redirects are not followed. After 5 consecutive \
                             failed deliveries a webhook is paused for 5 minutes."
                        </p>
                        <p>
                            "Self-hosted operators can open specific private ranges with "
                            <code>"MENTISDB_WEBHOOK_ALLOWED_CIDRS"</code>
                            " (e.g. a Tailscale tailnet); see "
                            <a href="/user#webhook-allowed-cidrs">"Private webhook targets"</a>
                            "."
                        </p>

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
                             "Failed logins are rate-limited per client (an IPv4 address or IPv6 /64) with a global \
                              budget and backoff to prevent brute-force enumeration; \
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

                         <h3>"Invalidation filtering (default retrieval)"</h3>
                         <p>
                             "Thoughts that are the target of a later "
                             <code>"Supersedes"</code>
                             ", "
                             <code>"Corrects"</code>
                             ", or "
                             <code>"Invalidates"</code>
                             " relation are recorded in "
                             <code>"invalidated_thought_ids"</code>
                             " and "
                             <strong>"excluded by default"</strong>
                             " from "
                             <code>"MentisDb::query"</code>
                             ", "
                             <code>"query_ranked"</code>
                             ", context bundles, recent context, and related REST/MCP search. \
                              Pass "
                             <code>"include_invalidated: true"</code>
                             " (crate builders: "
                             <code>"with_include_invalidated(true)"</code>
                             ") for audit. Point-in-time "
                             <code>"as_of"</code>
                             " still returns thoughts that were valid at that timestamp."
                         </p>
                         <p>
                             "Auto-dedup is "
                             <strong>"off"</strong>
                             " unless "
                             <code>"MENTISDB_DEDUP_THRESHOLD"</code>
                             " is set (recommended "
                             <code>"0.85"</code>
                             " for long-lived agent chains). Without dedup or agent-written \
                              invalidation edges, the set stays empty and search cannot hide noise."
                         </p>

                        // ── 0.12 Library Changes ────────────────────────────
                        <h2 id="0.12">"0.12 Library Changes"</h2>
                        <p>
                            "MentisDB 0.12.0 is a minor-version release because public library types \
                             changed. Sidecar files, their digests, persisted HNSW graphs, the wire \
                             protocol and the CLI are unchanged."
                        </p>

                        <h3>"Breaking: one flat vector store (VectorRow)"</h3>
                        <p>
                            "Every vector of a managed sidecar now lives once, in 128-row, 64-byte-aligned \
                             chunks shared by the sidecar entries, the exact index, the HNSW graph and the \
                             implicit-edge scanner. Exact search runs a lane-parallel dot product over it \
                             (about 4.3x faster at 100,000 vectors; scores within "<code>"1e-5"</code>
                            " of the old cosine, so near-ties may swap). Signature changes:"
                        </p>
                        <ul>
                            <li>
                                <code>"VectorSidecarEntry::vector"</code>" is a "
                                <code>"mentisdb::search::flat_store::VectorRow"</code>
                                " (dereferences to "<code>"&[f32]"</code>"), not a "<code>"Vec<f32>"</code>
                                ". Use "<code>".to_vec()"</code>" for a "<code>"Vec"</code>" and "
                                <code>"VectorRow::standalone(&values)"</code>" to make one."
                            </li>
                            <li>
                                <code>"VectorSidecarEntry::new"</code>" takes "<code>"impl Into<VectorRow>"</code>
                                ": "<code>"Vec<f32>"</code>" and "<code>"&[f32]"</code>" convert, but a bare \
                                 "<code>".collect()"</code>" whose type is only inferred no longer compiles; \
                                 annotate it."
                            </li>
                            <li>
                                <code>"VectorSidecar"</code>" has a private field: build it with "
                                <code>"VectorSidecar::build"</code>", not a struct literal."
                            </li>
                            <li>
                                "New: "<code>"search::flat_store"</code>" ("<code>"VectorStore"</code>", "
                                <code>"VectorRow"</code>", "<code>"dot"</code>", "<code>"DOT_EPSILON"</code>"), "
                                <code>"VectorIndex::{from_rows, upsert_row}"</code>", "
                                <code>"VectorSearchBackend::upsert_row"</code>" (provided, implementors keep \
                                 compiling) and "<code>"HnswBackend::{from_rows, share_cached_rows}"</code>"."
                            </li>
                        </ul>
                        <div class="code-block">
                            <pre><code>{r#"use mentisdb::search::VectorSidecarEntry;
use mentisdb::search::flat_store::VectorRow;

let entry = VectorSidecarEntry::new(id, index, hash, vec![0.1, 0.2, 0.3]); // unchanged
let owned: Vec<f32> = entry.vector.to_vec();   // 0.11: entry.vector.clone()
let slice: &[f32] = &entry.vector;            // unchanged
let values: Vec<f32> = (0..3).map(|i| i as f32).collect(); // annotate inferred types
let entry2 = VectorSidecarEntry::new(id, index, hash, values);
let literal = VectorSidecarEntry {
    thought_id: id, thought_index: index, thought_hash: hash.to_string(),
    vector: VectorRow::standalone(&[0.1, 0.2, 0.3]),   // 0.11: vec![0.1, 0.2, 0.3]
};"#}</code></pre>
                        </div>
                        <p>
                            <code>"entry.vector.clone()"</code>
                            " still compiles but now returns a "<code>"VectorRow"</code>
                            " (a pointer copy), not a "<code>"Vec<f32>"</code>"."
                        </p>

                        <h3>"Breaking: MentisDbServiceConfig is #[non_exhaustive]"</h3>
                        <p>
                            "It can no longer be built with a struct literal; the fields stay readable and \
                             assignable. Build it with "<code>"MentisDbServiceConfig::new(..)"</code>
                            " and the "<code>"with_*"</code>" builders. The body-admission settings are "
                            <code>"with_max_body_bytes"</code>" (import routes, clamped to 1 MiB..1 GiB), "
                            <code>"with_max_inflight_body_bytes"</code>" and "<code>"with_body_idle_timeout"</code>"."
                        </p>
                        <div class="code-block">
                            <pre><code>{r#"use mentisdb::server::MentisDbServiceConfig;
use mentisdb::StorageAdapterKind;
use std::time::Duration;

let config = MentisDbServiceConfig::new("/var/lib/mentisdb".into(), "project", StorageAdapterKind::Binary)
    .with_verbose(true)
    .with_max_body_bytes(128 * 1024 * 1024)
    .with_max_inflight_body_bytes(512 * 1024 * 1024)
    .with_body_idle_timeout(Duration::from_secs(60));"#}</code></pre>
                        </div>

                        <h3>"Request body admission"</h3>
                        <p>
                            "An outermost layer ("<code>"server_body"</code>") decides before reading a body: \
                             the bearer token is checked from the headers first; ordinary routes and every MCP \
                             request take at most "<code>"SMALL_BODY_BYTES"</code>" (2 MiB); only the routes in "
                            <code>"LARGE_BODY_ROUTES"</code>" take up to the configured cap, from an \
                             authenticated caller. Imports are reserved from their declared length against the \
                             in-flight budget; other requests are charged per arriving byte from a fixed "
                            <code>"SMALL_POOL_BYTES"</code>" (64 MiB) pool, of which an anonymous peer may hold "
                            <code>"ANONYMOUS_PEER_BYTES"</code>" (8 MiB). Stalled bodies are 408, an exhausted \
                             budget 503 with Retry-After. Read proofs that send "<code>"X-Mentis-Content-Sha256"</code>
                            " are verified before the body is read and the body is hashed as it streams."
                        </p>

                        <h3>"Derived-state backlog"</h3>
                        <p>
                            <code>"MentisDb::derived_backlog"</code>" returns a "<code>"DerivedBacklog"</code>
                            " summing a chain's managed sidecars: "
                            <code>"embeddings_in_flight"</code>" (requested, not committed), "
                            <code>"embeddings_parked"</code>" (ready, waiting for an earlier thought's \
                             vector), "
                            <code>"embeddings_missing"</code>" (waiting to be embedded again) and "
                            <code>"implicit_edges"</code>" (applied vectors whose edges have not reached \
                             the overlay). All are zero when the chain is caught up. "
                            <code>"GET /v1/chains"</code>" and the MCP chain listing report it per chain as "
                            <code>"derived_backlog"</code>
                            ", with zeros for a chain that is not open or whose private segments are \
                             hidden from the caller."
                        </p>
                        <div class="code-block">
                            <pre><code>{r#""derived_backlog": {
  "embeddings_in_flight": 0, "embeddings_parked": 0,
  "embeddings_missing": 0, "implicit_edges": 312
}"#}</code></pre>
                        </div>
                        <p>
                            "The implicit-edge queue holds positions, not vectors, so a backlog never \
                             grows memory by more than a few bytes, never blocks an append and never \
                             drops an edge. The scan sustains about 155 appends per second at 100,000 \
                             thoughts and 720 at 20,000 (4 threads, 384 dimensions); faster appends make \
                             "<code>"implicit_edges"</code>" grow, and it drains when they slow down."
                        </p>

                        // ── 0.11 Library Changes ────────────────────────────
                        <h2 id="0.11">"0.11 Library Changes"</h2>
                        <p>
                            "MentisDB 0.11.0 changes what default retrieval returns, adds public and \
                             private chains, and moves format upgrades into a background migration \
                             manager. Full rustdoc is on "
                            <a href="https://docs.rs/mentisdb/latest/mentisdb/" target="_blank" rel="noopener noreferrer">"docs.rs"</a>
                            "; the design of the access model is in "
                            <a href="https://github.com/CloudLLM-ai/mentisdb/blob/master/docs/CHAIN_ACCESS.md" target="_blank" rel="noopener noreferrer">"docs/CHAIN_ACCESS.md"</a>
                            "."
                        </p>

                        <h3>"Valid-version retrieval (module lineage)"</h3>
                        <p>
                            "Lineage is derived when a chain opens; no migration is needed. A superseded \
                             thought that would have ranked is replaced by its live successor ("
                            <code>"RankedSearchHit::version.successor_of"</code>
                            " names what it stands in for). A thought that a newer Correction, \
                             AssumptionInvalidated or Decision only references, or that anything "
                            <code>"Contradicts"</code>
                            ", stays visible as "
                            <code>"VersionStatus::Contested"</code>
                            " with the contesting thought promoted above it. New: "
                            <code>"MentisDb::{version_status, current_version, version_info, hit_json, append_advice}"</code>
                            ", "
                            <code>"VersionStatus"</code>
                            ", "
                            <code>"AppendAdvice"</code>
                            " and "
                            <code>"lineage::cross_chain_successor"</code>
                            " for a relation that carries a "
                            <code>"chain_key"</code>
                            " (applied wherever one read path sees both chains: branch search and \
                             federated search). Every thought JSON carries "
                            <code>"version_status"</code>
                            ", "
                            <code>"successors"</code>
                            " and "
                            <code>"current_version"</code>
                            " when it is not plainly current."
                        </p>
                        <div class="code-block">
                            <pre><code>{r#"use mentisdb::{RankedSearchQuery, VersionStatus};

let hits = chain.query_ranked(&RankedSearchQuery::new().with_text("release policy Fridays"));
// The live successor is returned in place of the superseded policy.
assert_eq!(hits.hits[0].version.successor_of, vec![old.id]);
assert_eq!(chain.version_status(old.id), VersionStatus::Superseded);"#}</code></pre>
                        </div>

                        <h3>"RankedSearchQuery defaults changed"</h3>
                        <p>
                            <code>"RankedSearchQuery::new()"</code>
                            " now applies relative vector fusion, near-duplicate collapse at Jaccard 0.9 \
                             (text queries), successor redirection and thesaurus expansion at weight 0.3 \
                             (the server default; the library used to apply none). Result lists and "
                            <code>"total_candidates"</code>
                            " differ from 0.10. Opt out per query:"
                        </p>
                        <div class="code-block">
                            <pre><code>{r#"use mentisdb::{RankedSearchQuery, VectorFusion};

let query = RankedSearchQuery::new()
    .with_text("deploy window")
    .with_vector_fusion(VectorFusion::Absolute) // the 0.10 blend
    .with_collapse_duplicates(None)             // keep near-duplicates
    .with_resolve_successors(false)             // plain hiding of superseded thoughts
    .with_thesaurus(false);                     // literal terms only"#}</code></pre>
                        </div>
                        <p>
                            "Process-wide: "
                            <code>"MENTISDB_VECTOR_FUSION=absolute"</code>
                            ", "
                            <code>"MENTISDB_COLLAPSE_DUPLICATES=off"</code>
                            ". "
                            <code>"with_df_scope"</code>
                            " and "
                            <code>"with_df_cutoffs"</code>
                            " choose how the lexical stop-word gate measures document frequency ("
                            <code>"DfScope::Global"</code>
                            " stays the default). Over REST and MCP the same knobs are request fields: "
                            <code>"resolve_successors"</code>
                            ", "
                            <code>"collapse_duplicates"</code>
                            " (0 disables), "
                            <code>"vector_fusion"</code>
                            ", "
                            <code>"synonym_weight"</code>
                            " (0 disables), "
                            <code>"df_scope"</code>
                            " and "
                            <code>"df_cutoffs"</code>
                            "."
                        </p>

                        <h3>"Standing context (module standing)"</h3>
                        <p>
                            <code>"MentisDb::standing_context(&StandingContextQuery)"</code>
                            " returns the durable half of memory regardless of age: live Constraint, \
                             PreferenceUpdate, UserTrait, RelationshipUpdate, Goal, Decision and \
                             LessonLearned thoughts, ranked by importance then recency, restatements \
                             collapsed, capped at 8 per type and 40 in total, plus the newest Handoff \
                             and Checkpoint/Summary. The "
                            <code>"StandingContext"</code>
                            " result has "
                            <code>"entries"</code>
                            ", "
                            <code>"duplicates_collapsed"</code>
                            " and a ready-to-load "
                            <code>"prompt"</code>
                            ". A superseded constraint or decision is replaced by its successor."
                        </p>

                        <h3>"Public and private chains"</h3>
                        <ul>
                            <li>
                                <code>"mentisdb::audience::{Audience, Segment, SegmentMap}"</code>
                                ": every read path takes an audience. "
                                <code>"ThoughtQuery::audience"</code>
                                ", "
                                <code>"StandingContextQuery::audience"</code>
                                " and "
                                <code>"*_for(audience)"</code>
                                " variants ("
                                <code>"get_thought_for"</code>
                                ", "
                                <code>"thought_json_for"</code>
                                ", "
                                <code>"hit_json_for"</code>
                                ", "
                                <code>"append_advice_for"</code>
                                ", ...). The library default is "
                                <code>"Audience::Authorized"</code>
                                ": a program that holds the data directory holds every byte."
                            </li>
                            <li>
                                <code>"mentisdb::read_auth::{sign_request, ReadSigner}"</code>
                                " build the "
                                <code>"X-Mentis-Auth"</code>
                                " read-proof header for HTTP clients. To send a body over 1 MiB also send "
                                <code>"X-Mentis-Content-Sha256"</code>
                                " ("
                                <code>"body_digest"</code>
                                ", "
                                <code>"content_digest_header_value"</code>
                                "): the server verifies the proof from the declared digest before reading the \
                                 body and hashes the body as it streams, failing the request at its end on a \
                                 mismatch, before any handler has run."
                            </li>
                            <li>
                                <code>"mentisdb::operator::apply"</code>
                                " runs operator commands (mode changes, allowed keys, key registry, \
                                 agent status) signed with "
                                <code>"<data dir>/operator/operator.key"</code>
                                "; "
                                <code>"MentisDb::add_agent_key_as_operator"</code>
                                " registers a key the operator vouches for. "
                                <code>"MentisDb::add_agent_key"</code>
                                " still requires a "
                                <code>"&KeyRegistrationProof"</code>
                                " (proof of possession) and a valid curve point."
                            </li>
                            <li>
                                <code>"MentisDb::verification"</code>
                                " reports the open-time check of governed chains (quarantined thoughts, \
                                 rejected policy records)."
                            </li>
                            <li>
                                <code>"AccessError::StaleHead"</code>
                                " is returned for a v2-signed append built on an older head (fetch the \
                                 head and sign again); a forged signature stays "
                                <code>"BadSignature"</code>
                                "."
                            </li>
                            <li>
                                "A stored zero-length signature is a placeholder and counts as unsigned \
                                 everywhere; new writes normalize an empty signature to none. "
                                <code>"Thought::signature_bytes"</code>
                                " returns the real signature or nothing."
                            </li>
                        </ul>

                        <h3>"Storage and validation"</h3>
                        <ul>
                            <li>
                                <code>"mentisdb::validate_chain_key"</code>
                                " guards every entry point (library open, branch, restore): non-empty, at \
                                 most 256 bytes, no control characters."
                            </li>
                            <li>
                                <code>"MentisDb::defer_next_embedding"</code>
                                " lets a caller embed outside the chain's write lock, as the daemon now \
                                 does for every append (lock hold per append about 0.5 ms instead of \
                                 about 9.5 ms with a 5 ms model). Library callers that do not opt in are \
                                 unchanged."
                            </li>
                            <li>
                                "The daemon migrates legacy chains in the background through a journaled, \
                                 backed-up manager. A library user that opens a legacy chain directly \
                                 keeps the in-place upgrade, now with a one-time copy of the original under "
                                <code>"migrations/backup/in-place/"</code>
                                ". A chain or data directory written by a newer release is refused with a \
                                 clear message."
                            </li>
                            <li>
                                "Chains from March and April 2026 that stamp schema 4 on schema-3 records \
                                 open again: schema 4 is an accepted historical alias, so the next thought \
                                 schema is 5."
                            </li>
                        </ul>

                        <h3>"Deprecated"</h3>
                        <p>
                            <code>"RankedSearchQuery::{llm_rerank, llm_model, llm_rerank_top_k, with_llm_reranking}"</code>
                            " and "
                            <code>"RankedSearchScore::llm_score"</code>
                            " are no-ops (LLM reranking was never implemented), and "
                            <code>"search::ppr"</code>
                            " is never called by ranked search. Both are marked "
                            <code>"#[deprecated]"</code>
                            " and will be removed in the next breaking release."
                        </p>

                        <h3>"Benchmarks"</h3>
                        <p>
                            "Full LoCoMo-10P and LongMemEval protocol run on the 0.11 defaults (fresh \
                             ingest, fastembed-minilm): LoCoMo-10P R@10 76.7% (73.5% in 0.10.9.54), \
                             LongMemEval R@5 69.8% (68.0%). See "
                            <a href="https://github.com/CloudLLM-ai/mentisdb/tree/master/results/benchmarks/2026-11-01-full-protocol" target="_blank" rel="noopener noreferrer">"results/benchmarks/2026-11-01-full-protocol"</a>
                            "."
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
                                " — durable mode. Appends are queued to the bounded background writer and the request only returns after the writer flushes them. Concurrent appends may share a short group-commit window; a lone append flushes immediately. Since 0.10.9 commits are fsynced only with MENTISDB_FSYNC=1, so benchmark both settings when touching the writer."
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
