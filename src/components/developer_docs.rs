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
                            <a href="#mcp-server"        class="docs-nav-link">"MCP Server"</a>
                            <a href="#rest-api"          class="docs-nav-link">"REST API"</a>
                            <a href="#schema"            class="docs-nav-link">"Schema Version"</a>
                            <a href="#import-api"        class="docs-nav-link">"Bulk Import API"</a>
                            <a href="#taxonomy"          class="docs-nav-link">"Thought Taxonomy"</a>
                            <a href="#thought-relations" class="docs-nav-link">"Thought Relations"</a>
                            <a href="#storage"           class="docs-nav-link">"Storage Adapters"</a>
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
                            <code>r#"mentisdb = "0.7""#</code>
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
                            " feature. The server exposes all MentisDB operations as MCP \
                             tools, making it compatible with any MCP-capable AI tool. The canonical \
                             onboarding path is not a separate URL: it happens during MCP "
                            <code>"initialize"</code>
                            " via startup instructions plus the embedded resource "
                            <code>"mentisdb://skill/core"</code>
                            " exposed through "
                            <code>"resources/list"</code>
                            " and "
                            <code>"resources/read"</code>
                            "."
                        </p>
                        <p>
                            "Default streamable HTTP endpoint: "
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
                                    <td><code>"/v1/thoughts"</code></td>
                                    <td>"Append a new thought to a chain"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/search"</code></td>
                                    <td>"Search/query thoughts"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/lexical-search"</code></td>
                                    <td>"Ranked lexical search with scores and matched-term diagnostics"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/ranked-search"</code></td>
                                    <td>"Canonical flat ranked retrieval with lexical + vector + graph-aware score breakdowns (hybrid when managed sidecars are available)"</td>
                                </tr>
                                <tr>
                                    <td><code>"POST"</code></td>
                                    <td><code>"/v1/context-bundles"</code></td>
                                    <td>"Seed-anchored grouped support context for agent reasoning and dashboard inspection"</td>
                                </tr>
                                <tr>
                                    <td><code>"GET"</code></td>
                                    <td><code>"/v1/chains"</code></td>
                                    <td>"List available chain keys"</td>
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
                            </tbody>
                        </table>
                        <p>
                            "For MCP-native agents, prefer the streamable HTTP root at "
                            <code>"POST /"</code>
                            " and let the agent bootstrap itself from the initialize instructions and resource catalog."
                        </p>
                        <p>
                            "For direct crate users, 0.7.2 keeps vector sidecars additive and \
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

                        // ── Schema Version ───────────────────────────────────
                        <h2 id="schema">"Schema Version"</h2>
                        <p>
                            "MentisDB 0.7.2 uses schema version 2 ("
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
                            <strong>"ThoughtType (29): "</strong>
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
