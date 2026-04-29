use leptos::prelude::*;

/// UserDocs — full installation and usage guide for end users running MentisDB locally.
#[component]
pub fn UserDocs() -> impl IntoView {
    view! {
                <section class="docs-page">
                    <div class="container docs-layout">

                        // ── Left sidebar ──────────────────────────────────────────────
                        <aside class="docs-sidebar">
                            <nav class="docs-nav">
                                <a class="docs-nav-link" href="#installation">"Installation"</a>
                                <a class="docs-nav-link" href="#running">"Running the Daemon"</a>
                                <a class="docs-nav-link" href="#configuration">"Configuration"</a>
                                <a class="docs-nav-link" href="#priming">"Priming Your Agent"</a>
                                <a class="docs-nav-link" href="#self-update">"Self-Update"</a>
                                <a class="docs-nav-link" href="#https">"HTTPS & TLS"</a>
                                <a class="docs-nav-link" href="#dashboard">"Web Dashboard"</a>
                                <a class="docs-nav-link" href="#import-memory-md">"Import MEMORY.md"</a>
                                <a class="docs-nav-link" href="#backup-restore">"Backup & Restore"</a>
                                <a class="docs-nav-link" href="#connecting">"Connecting AI Tools"</a>
                                <a class="docs-nav-link" href="#connecting-automated">"Automated Setup"</a>
                                <a class="docs-nav-link" href="#connecting-manual">"Manual MCP Config"</a>
                                <a class="docs-nav-link" href="#chain-topologies">"Chain Topologies"</a>
                                <a class="docs-nav-link" href="#fleet-coordination">"Fleet Coordination"</a>
                                <a class="docs-nav-link" href="#skills-registry">"The Skills Registry"</a>
                                <a class="docs-nav-link" href="#signatures">"Cryptographic Signatures"</a>
                                <a class="docs-nav-link" href="#memory-scopes">"Memory Scopes"</a>
                                <a class="docs-nav-link" href="#temporal-queries">"Temporal Queries"</a>
                                <a class="docs-nav-link" href="#cli-subcommands">"CLI Subcommands"</a>
                            </nav>
                        </aside>

                        // ── Main content ──────────────────────────────────────────────
                        <article class="docs-content">

                            // ── Installation ─────────────────────────────────────────
                            <section class="docs-section" id="installation">
                                <h1>"User Guide"</h1>
                                <h2 id="installation">"Installation"</h2>
                                <p>"MentisDB requires Rust. If you don't have it:"</p>
                                <div class="code-block">
                                    <code>"curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"</code>
                                </div>
                                <p>"Then install MentisDB:"</p>
                                <div class="code-block">
                                    <code>"cargo install mentisdb"</code>
                                </div>
                            </section>

                            // ── Running the Daemon ───────────────────────────────────
                            <section class="docs-section" id="running">
                                <h2 id="running">"Running the Daemon"</h2>
                                <p>"Start the daemon — it listens on port 9471 by default:"</p>
                                <div class="code-block">
                                    <code>"mentisdbd"</code>
                                </div>
                                <p>"To keep it running after closing your terminal:"</p>
                                <div class="code-block">
                                    <code>"nohup mentisdbd &"</code>
                                </div>
                                <p>
                                    "The daemon serves both MCP (for AI tools) and REST endpoints \
                                     plus an HTTPS dashboard for human operators."
                                </p>
                                <p>"CLI subcommands for quick operations without an MCP client:"</p>
                                <div class="code-block">
                                    <pre><code>{r#"mentisdbd add "The sky is blue"
mentisdbd search "cache invalidation" --limit 5
mentisdbd agents"#}</code></pre>
                                </div>
                            </section>

                            // ── Configuration ────────────────────────────────────────
                            <section class="docs-section" id="configuration">
                                <h2 id="configuration">"Configuration"</h2>
                                <p>"MentisDB is configured via environment variables:"</p>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Variable"</th>
                                            <th>"Default"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Storage"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DIR"</code></td>
                                            <td><code>"~/.cloudllm/mentisdb"</code></td>
                                            <td>"Where chain data and TLS files are stored on disk"</td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <code>"MENTISDB_DEFAULT_CHAIN_KEY"</code>
                                                <br/>
                                                <small><em>"(deprecated alias: " <code>"MENTISDB_DEFAULT_KEY"</code> ")"</em></small>
                                            </td>
                                            <td><code>"borganism-brain"</code></td>
                                            <td>
                                                "The chain key used when no "
                                                <code>"chain_key"</code>
                                                " is specified"
                                            </td>
                                        </tr>
        <tr>
                                            <td><code>"MENTISDB_STORAGE_ADAPTER"</code></td>
                                            <td><code>"binary"</code></td>
                                             <td>
                                                 "Storage format for new chains. Only "
                                                 <code>"binary"</code>
                                                 " is accepted; the JSONL adapter can no longer be \
                                                  used to create new chains. Existing JSONL chains \
                                                  remain fully readable and migratable — the adapter \
                                                  is retained for reading and migrating legacy data only"
                                             </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_GROUP_COMMIT_MS"</code></td>
                                            <td><code>"2"</code></td>
                                            <td>
                                                "Group-commit window in milliseconds for the background \
                                                 binary writer. The writer batches appends within this \
                                                 window before flushing to disk. Lower values = lower \
                                                 latency; higher values = better throughput"
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_AUTO_FLUSH"</code></td>
                                            <td><code>"true"</code></td>
                                            <td>
                                                "Flush to disk on every write. Set to "
                                                <code>"false"</code>
                                                " for higher throughput (less durability)"
                                            </td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Limits"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MAX_THOUGHT_PAYLOAD_BYTES"</code></td>
                                            <td><code>"10 MB"</code></td>
                                            <td>
                                                "Maximum size of a single thought payload. Lowered from \
                                                 64 MB as DoS protection. Oversized writes are rejected \
                                                 with an error"
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MAX_SKILL_SIZE_BYTES"</code></td>
                                            <td><code>"1 MB"</code></td>
                                            <td>
                                                "Upload size limit for a single skill file. Exceeding \
                                                 this limit rejects the upload"
                                            </td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Networking — HTTP"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_BIND_HOST"</code></td>
                                            <td><code>"127.0.0.1"</code></td>
                                            <td>
                                                "IP address the server binds to. Use "
                                                <code>"0.0.0.0"</code>
                                                " for network-wide access (combine with a dashboard PIN)"
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_MCP_PORT"</code></td>
                                            <td><code>"9471"</code></td>
                                            <td>"HTTP MCP server port"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_REST_PORT"</code></td>
                                            <td><code>"9472"</code></td>
                                            <td>"HTTP REST API port"</td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Networking — HTTPS"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_HTTPS_MCP_PORT"</code></td>
                                            <td><code>"9473"</code></td>
                                            <td>"HTTPS MCP server port. Set to 0 to disable"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_HTTPS_REST_PORT"</code></td>
                                            <td><code>"9474"</code></td>
                                            <td>"HTTPS REST API port. Set to 0 to disable"</td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"TLS"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_TLS_CERT"</code></td>
                                            <td><code>"~/.cloudllm/mentisdb/tls/cert.pem"</code></td>
                                            <td>"Path to the TLS certificate PEM (auto-generated on first start)"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_TLS_KEY"</code></td>
                                            <td><code>"~/.cloudllm/mentisdb/tls/key.pem"</code></td>
                                            <td>"Path to the TLS private key PEM (auto-generated on first start)"</td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Logging"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_VERBOSE"</code></td>
                                            <td><code>"true"</code></td>
                                            <td>"Enable verbose startup and request logging"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_LOG_FILE"</code></td>
                                            <td><em>"unset"</em></td>
                                            <td>"Optional path to write logs to a file"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"RUST_LOG"</code></td>
                                            <td><code>"info"</code></td>
                                            <td>"Log level filter (trace, debug, info, warn, error)"</td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Audio"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_STARTUP_SOUND"</code></td>
                                            <td><code>"true"</code></td>
                                            <td>"Play a 4-note jingle on startup"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_THOUGHT_SOUNDS"</code></td>
                                            <td><code>"false"</code></td>
                                            <td>"Play a unique sound for each ThoughtType on append"</td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Web Dashboard"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DASHBOARD_PORT"</code></td>
                                            <td><code>"9475"</code></td>
                                            <td>"Dashboard HTTPS port. Set to 0 to disable"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DASHBOARD_PIN"</code></td>
                                            <td><em>"unset"</em></td>
                                            <td>"Optional PIN to gate dashboard access. Unset = open (localhost only)"</td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Daemon Self-Update"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_UPDATE_CHECK"</code></td>
                                            <td><code>"true"</code></td>
                                            <td>"Background GitHub release check for mentisdbd. Set it to false to disable the startup-time prompt."</td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_UPDATE_REPO"</code></td>
                                            <td><code>"CloudLLM-ai/mentisdb"</code></td>
                                            <td>"Optional owner/repo override for the GitHub release source used by the updater."</td>
                                        </tr>
                                        <tr class="config-group-header">
                                            <td colspan="3"><strong>"Deduplication"</strong></td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DEDUP_THRESHOLD"</code></td>
                                            <td><em>"unset"</em></td>
                                            <td>
                                                "Jaccard threshold for auto-dedup on append (0.0–1.0). \
                                                 When set, MentisDB compares new thoughts against recent \
                                                 memories and auto-Supersedes duplicates above this \
                                                 threshold. Disabled when unset."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DEDUP_SCAN_WINDOW"</code></td>
                                            <td><code>"64"</code></td>
                                            <td>
                                                "Number of recent thoughts to scan for dedup comparison. \
                                                 Only used when "
                                                <code>"MENTISDB_DEDUP_THRESHOLD"</code>
                                                " is set."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </section>

                            // ── Environment Variables Reference ──────────────────────
                            <section class="docs-section" id="environment-variables">
                                <h2 id="environment-variables">"Environment Variables Reference"</h2>
                                <p>
                                    "Every MentisDB environment variable, grouped by concern. \
                                     Each row lists the default, a one-sentence purpose, and a \
                                     concrete example with a rationale for when you'd reach for \
                                     it. All variables are read once at "
                                    <code>"mentisdbd"</code>
                                    " startup unless noted otherwise."
                                </p>

                                <h3>"Storage"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Variable"</th>
                                            <th>"Default"</th>
                                            <th>"Purpose"</th>
                                            <th>"Why / example"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"MENTISDB_DIR"</code></td>
                                            <td><code>"~/.cloudllm/mentisdb"</code></td>
                                            <td>
                                                "Root directory where chain files, the chain \
                                                 registry, the skill registry, and TLS certs live."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_DIR=/mnt/ssd/mentisdb"</code>
                                                " — point storage at a large fast disk in production."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td>
                                                <code>"MENTISDB_DEFAULT_CHAIN_KEY"</code>
                                                <br/>
                                                <small><em>
                                                    "(deprecated alias: "
                                                    <code>"MENTISDB_DEFAULT_KEY"</code>
                                                    ")"
                                                </em></small>
                                            </td>
                                            <td><code>"borganism-brain"</code></td>
                                            <td>
                                                "Chain used when a request omits "
                                                <code>"chain_key"</code>
                                                "."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_DEFAULT_CHAIN_KEY=team-core"</code>
                                                " — make a shared team chain the implicit default."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_STORAGE_ADAPTER"</code></td>
                                            <td><code>"binary"</code></td>
                                            <td>
                                                "Storage format for new chains; only "
                                                <code>"binary"</code>
                                                " is supported for new chains."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_STORAGE_ADAPTER=binary"</code>
                                                " — set explicitly in ops manifests for future-proofing."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"Behaviour"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Variable"</th>
                                            <th>"Default"</th>
                                            <th>"Purpose"</th>
                                            <th>"Why / example"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"MENTISDB_AUTO_FLUSH"</code></td>
                                            <td><code>"true"</code></td>
                                            <td>
                                                <code>"true"</code>
                                                " fsyncs on every append (strict durability); "
                                                <code>"false"</code>
                                                " batches up to 16 records in a background writer \
                                                 (higher throughput, up to 15 records may be lost \
                                                 on hard crash)."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_AUTO_FLUSH=false"</code>
                                                " — for write-heavy multi-agent hubs where throughput \
                                                 matters more than last-second durability."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_GROUP_COMMIT_MS"</code></td>
                                            <td><code>"2"</code></td>
                                            <td>
                                                "Window in milliseconds for the strict-mode writer \
                                                 to coalesce concurrent appends into one flush."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_GROUP_COMMIT_MS=5"</code>
                                                " — bigger window amortises fsync cost at the price \
                                                 of 3ms extra p99 latency."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DEDUP_THRESHOLD"</code></td>
                                            <td><em>"unset (disabled)"</em></td>
                                            <td>
                                                "Jaccard similarity threshold in "
                                                <code>"[0.0, 1.0]"</code>
                                                " for auto-emitting "
                                                <code>"Supersedes"</code>
                                                " relations on near-duplicate appends."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_DEDUP_THRESHOLD=0.85"</code>
                                                " — collapse near-identical retrospectives."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DEDUP_SCAN_WINDOW"</code></td>
                                            <td><code>"64"</code></td>
                                            <td>
                                                "How many recent thoughts to scan when dedup is enabled."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_DEDUP_SCAN_WINDOW=128"</code>
                                                " — widen the comparison window on chatty chains \
                                                 where near-duplicates arrive in bursts."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_VERBOSE"</code></td>
                                            <td><code>"true"</code></td>
                                            <td>
                                                "Log each MentisDB operation to the "
                                                <code>"mentisdb::interaction"</code>
                                                " target."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_VERBOSE=false"</code>
                                                " in production where you've attached your own \
                                                 observability and don't need the built-in stream."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_LOG_FILE"</code></td>
                                            <td><em>"unset (stdout only)"</em></td>
                                            <td>
                                                "Optional file path for interaction logs."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_LOG_FILE=/var/log/mentisdb/interactions.log"</code>
                                                " — persist the interaction stream for later audit \
                                                 or log-shipping."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"Networking / TLS"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Variable"</th>
                                            <th>"Default"</th>
                                            <th>"Purpose"</th>
                                            <th>"Why / example"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"MENTISDB_BIND_HOST"</code></td>
                                            <td><code>"127.0.0.1"</code></td>
                                            <td>
                                                "IP address for all server sockets."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_BIND_HOST=0.0.0.0"</code>
                                                " — bind to all interfaces (use with care; firewall \
                                                 or set a dashboard PIN first)."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_MCP_PORT"</code></td>
                                            <td><code>"9471"</code></td>
                                            <td>"HTTP MCP server port."</td>
                                            <td>
                                                <code>"MENTISDB_MCP_PORT=19471"</code>
                                                " — move off the default when running a second \
                                                 daemon on the same host."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_REST_PORT"</code></td>
                                            <td><code>"9472"</code></td>
                                            <td>"HTTP REST server port."</td>
                                            <td>
                                                <code>"MENTISDB_REST_PORT=19472"</code>
                                                " — align with the MCP port offset when running a \
                                                 side-by-side instance."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_HTTPS_MCP_PORT"</code></td>
                                            <td><code>"9473"</code></td>
                                            <td>
                                                "HTTPS MCP port; set to "
                                                <code>"0"</code>
                                                " to disable."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_HTTPS_MCP_PORT=0"</code>
                                                " — turn off HTTPS MCP entirely on a fully-internal host."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_HTTPS_REST_PORT"</code></td>
                                            <td><code>"9474"</code></td>
                                            <td>
                                                "HTTPS REST port; set to "
                                                <code>"0"</code>
                                                " to disable."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_HTTPS_REST_PORT=0"</code>
                                                " — disable HTTPS REST when terminating TLS at a reverse proxy."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_TLS_CERT"</code></td>
                                            <td><code>"<MENTISDB_DIR>/tls/cert.pem"</code></td>
                                            <td>
                                                "PEM cert path. Auto-generated on first boot if absent."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_TLS_CERT=/etc/ssl/mentisdb/fullchain.pem"</code>
                                                " — swap in a CA-signed cert instead of the default self-signed one."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_TLS_KEY"</code></td>
                                            <td><code>"<MENTISDB_DIR>/tls/key.pem"</code></td>
                                            <td>"PEM key path."</td>
                                            <td>
                                                <code>"MENTISDB_TLS_KEY=/etc/ssl/mentisdb/privkey.pem"</code>
                                                " — pair with a CA-signed cert from a secrets mount."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"Dashboard"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Variable"</th>
                                            <th>"Default"</th>
                                            <th>"Purpose"</th>
                                            <th>"Why / example"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"MENTISDB_DASHBOARD_PORT"</code></td>
                                            <td><code>"9475"</code></td>
                                            <td>
                                                "HTTPS-only dashboard port; set to "
                                                <code>"0"</code>
                                                " to disable entirely."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_DASHBOARD_PORT=0"</code>
                                                " — turn the dashboard off on headless servers where \
                                                 only the MCP/REST APIs are needed."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_DASHBOARD_PIN"</code></td>
                                            <td><em>"unset"</em></td>
                                            <td>
                                                "Shared PIN required to access the dashboard; empty \
                                                 string is treated as absent."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_DASHBOARD_PIN=8472-9471"</code>
                                                " — required whenever you bind the dashboard off "
                                                <code>"127.0.0.1"</code>
                                                "."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"Updates"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Variable"</th>
                                            <th>"Default"</th>
                                            <th>"Purpose"</th>
                                            <th>"Why / example"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"MENTISDB_UPDATE_CHECK"</code></td>
                                            <td><code>"true"</code></td>
                                            <td>
                                                "Enable/disable the automatic update check on daemon startup."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_UPDATE_CHECK=false"</code>
                                                " in CI — keeps test runs deterministic and offline."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_UPDATE_REPO"</code></td>
                                            <td><code>"CloudLLM-ai/mentisdb"</code></td>
                                            <td>
                                                "Override the GitHub repo the updater polls (useful for forks)."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_UPDATE_REPO=my-org/mentisdb-fork"</code>
                                                " — track an internal fork's release channel instead of upstream."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"Audio"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Variable"</th>
                                            <th>"Default"</th>
                                            <th>"Purpose"</th>
                                            <th>"Why / example"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"MENTISDB_STARTUP_SOUND"</code></td>
                                            <td><em>"unset (off)"</em></td>
                                            <td>
                                                "Set to "
                                                <code>"1"</code>
                                                "/"
                                                <code>"true"</code>
                                                " to play the startup chime."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_STARTUP_SOUND=true"</code>
                                                " on a dev workstation — audible cue that the daemon \
                                                 is up after a restart."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"MENTISDB_THOUGHT_SOUNDS"</code></td>
                                            <td><em>"unset (off)"</em></td>
                                            <td>
                                                "Set to "
                                                <code>"1"</code>
                                                "/"
                                                <code>"true"</code>
                                                " to play a per-thought-type sound on each append."
                                            </td>
                                            <td>
                                                <code>"MENTISDB_THOUGHT_SOUNDS=true"</code>
                                                " — audible feedback while pair-programming with an \
                                                 agent; leave off on shared machines."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </section>

                            // ── Priming Your Agent ───────────────────────────────────
                            <section class="docs-section" id="priming">
                                <h2 id="priming">"Priming Your Agent"</h2>
                                <p>
                                    "Priming an agent for MentisDB is one sentence. Just type:"
                                </p>
                                <div class="code-block">
                                    <pre><code>"prime yourself for optimal mentisdb usage"</code></pre>
                                </div>
                                <p>
                                    "That is all. The agent handles the rest automatically:"
                                </p>
                                <ol>
                                    <li>
                                        <strong>"Detect and select chain"</strong>
                                        " — calls "
                                        <code>"mentisdb_list_chains"</code>
                                        " to see what chains exist, then picks the one whose name best matches \
                                         the current project, repository, or working folder. If no chains exist \
                                         yet, it asks you to name a new one before proceeding."
                                    </li>
                                    <li>
                                        <strong>"Bootstrap"</strong>
                                        " — calls "
                                        <code>"mentisdb_bootstrap"</code>
                                        " with the chosen chain key to create or reopen that chain, then \
                                         calls "
                                        <code>"mentisdb_list_agents"</code>
                                        " and reuses the existing specialist that best matches the current \
                                         task, followed by "
                                        <code>"mentisdb_recent_context"</code>
                                        " to reload prior state."
                                    </li>
                                    <li>
                                        <strong>"Load skill rules"</strong>
                                        " — reads "
                                        <code>"mentisdb://skill/core"</code>
                                        " so it knows exactly how to write thoughts, search, and manage context"
                                    </li>
                                    <li>
                                        <strong>"Self-seed"</strong>
                                        " — writes a Summary checkpoint so every future session recovers state \
                                         automatically without needing to re-prime"
                                    </li>
                                </ol>
                                <div class="docs-callout-tip">
                                    <p>
                                        <strong>"First time?"</strong>
                                        " If no memory chains exist yet, the agent will briefly explain what a \
                                         chain is and ask what to name it — usually your project or repository \
                                         name works well."
                                    </p>
                                </div>
                                <div class="docs-callout-tip">
                                    <p>
                                        <strong>"Tip:"</strong>
                                        " Add this phrase to your tool's system-prompt or project instructions \
                                         file and every new agent session will prime itself automatically — \
                                         you will never need to type it again."
                                    </p>
                                </div>

                                <h3>"Additional tips"</h3>
                                <ul>
                                    <li>
                                        "Give the agent a stable "
                                        <code>"agent_id"</code>
                                        " — this is how its memories are attributed and retrieved later"
                                    </li>
                                    <li>
                                        "Tell it which "
                                        <code>"chain_key"</code>
                                        " to use if you run multiple chains (e.g. one per project)"
                                    </li>
                                    <li>
                                        "Instruct it to load memories "
                                        <em>"before starting"</em>
                                        " — not after it has already made decisions"
                                    </li>
                                    <li>
                                        "If your client supports MCP resources, tell the agent to read "
                                        <code>"mentisdb://skill/core"</code>
                                        " first; only use "
                                        <code>"GET /mentisdb_skill_md"</code>
                                        " as a fallback for non-MCP or limited clients"
                                    </li>
                                    <li>
                                        "If you do not specify a chain up front, tell the agent to prefer \
                                         a chain whose name matches the current repo or working folder"
                                    </li>
                                    <li>
                                        "Ask it to write a "
                                        <code>"Summary"</code>
                                        " checkpoint before compacting its context or ending a long session"
                                    </li>
                                </ul>
                            </section>

                            // ── Self-Update ────────────────────────────────────────
                            <section class="docs-section" id="self-update">
                                <h2 id="self-update">"Self-Update"</h2>
                                <p>
                                    "MentisDB's daemon checks GitHub for a newer release after startup \
                                     by default. On an interactive terminal, the daemon finishes \
                                     booting, then shows an ASCII prompt asking whether it should \
                                     update itself with "
                                    <code>"cargo install"</code>
                                    "."
                                </p>
                                <div class="code-block">
                                    <code>"MENTISDB_UPDATE_CHECK=0 mentisdbd"</code>
                                </div>
                                <p>
                                    "Version comparison uses the first three numeric components only. \
                                     That means a release tag like "
                                    <code>"0.6.1.14"</code>
                                    " is treated as core version "
                                    <code>"0.6.1"</code>
                                    ", and the fourth number is only the monotonically increasing \
                                     release counter."
                                </p>
                                <p>
                                    "If the terminal is non-interactive, MentisDB never blocks on stdin. \
                                     It prints the exact "
                                    <code>"cargo install --git ... --tag ..."</code>
                                    " command you can run manually instead."
                                </p>
                                <p>
                                    "Use "
                                    <code>"MENTISDB_UPDATE_CHECK=0"</code>
                                    " when you want a quiet daemon with no release prompt."
                                </p>
                            </section>

                            // ── HTTPS & TLS ──────────────────────────────────────────
                            <section class="docs-section" id="https">
                                <h2 id="https">"HTTPS & TLS"</h2>
                                <p>
                                    "MentisDB automatically generates a self-signed TLS certificate on \
                                     first startup using "
                                    <code>"rcgen"</code>
                                    ". This enables encrypted connections on two dedicated HTTPS ports \
                                     alongside the plain HTTP ports. No manual certificate management \
                                     is required."
                                </p>

                                <h3>"Port Map"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Port"</th>
                                            <th>"Protocol"</th>
                                            <th>"Purpose"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"9471"</code></td>
                                            <td>"HTTP"</td>
                                            <td>"MCP server"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"9472"</code></td>
                                            <td>"HTTP"</td>
                                            <td>"REST API"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"9473"</code></td>
                                            <td>"HTTPS"</td>
                                            <td>"MCP server (TLS)"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"9474"</code></td>
                                            <td>"HTTPS"</td>
                                            <td>"REST API (TLS)"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"9475"</code></td>
                                            <td>"HTTPS"</td>
                                            <td>"Web Dashboard"</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"The my.mentisdb.com Hostname"</h3>
                                <p>
                                    <code>"my.mentisdb.com"</code>
                                    " is a public DNS A-record that resolves to "
                                    <code>"127.0.0.1"</code>
                                    ". The auto-generated certificate includes it as a Subject \
                                     Alternative Name alongside "
                                    <code>"localhost"</code>
                                    " and "
                                    <code>"127.0.0.1"</code>
                                    ". Once you trust the certificate on your machine, you can use \
                                     "
                                    <code>"https://my.mentisdb.com:9473"</code>
                                    " as a friendly, stable hostname in MCP configs — no port-forwarding \
                                     or extra DNS setup required."
                                </p>

                                <h3>"Trusting the Self-Signed Certificate"</h3>
                                <p>
                                    "The certificate is saved to "
                                    <code>"~/.cloudllm/mentisdb/tls/cert.pem"</code>
                                    " on first startup. Run the appropriate command once per machine:"
                                </p>

                                <h4>"macOS"</h4>
                                <div class="code-block">
                                    <pre><code>{r#"sudo security add-trusted-cert -d -r trustRoot \
            -k /Library/Keychains/System.keychain \
            ~/.cloudllm/mentisdb/tls/cert.pem"#}</code></pre>
                                </div>

                                <h4>"Linux"</h4>
                                <div class="code-block">
                                    <pre><code>{r#"sudo cp ~/.cloudllm/mentisdb/tls/cert.pem \
    /usr/local/share/ca-certificates/mentisdb.crt
sudo update-ca-certificates"#}</code></pre>
                                </div>

                                <h4>"Windows"</h4>
                                <p>
                                    "Double-click "
                                    <code>"cert.pem"</code>
                                    " → "
                                    <em>"Install Certificate"</em>
                                    " → Local Machine → Trusted Root Certification Authorities → Finish."
                                </p>

                                <div class="docs-callout docs-callout-tip">
                                    "After trusting the certificate, restart your MCP client (e.g. Claude \
                                     Desktop) and use "
                                    <code>"https://my.mentisdb.com:9473"</code>
                                    " as the server URL."
                                </div>

                                <h3>"Disabling HTTPS"</h3>
                                <p>"Set both HTTPS ports to 0 to run HTTP-only:"</p>
                                <div class="code-block">
                                    <pre><code>{r#"MENTISDB_HTTPS_MCP_PORT=0
            MENTISDB_HTTPS_REST_PORT=0"#}</code></pre>
                                </div>
                            </section>

                            // ── Web Dashboard ────────────────────────────────────────
                            <section class="docs-section" id="dashboard">
                                <h2 id="dashboard">"Web Dashboard"</h2>
                                <p>
                                    "The web dashboard is a self-contained single-page application \
                                     embedded directly in the MentisDB binary — no npm, no separate \
                                     process, no installation required. Open it in any browser at:"
                                </p>
                                <div class="code-block">
                                    <code>"https://127.0.0.1:9475/dashboard"</code>
                                </div>
                                <p>
                                    "The version number is displayed in the nav header. The dashboard \
                                     connects to the same daemon you already have running, and it keeps \
                                     showing newly appended thoughts while the daemon stays up — no \
                                     restart is required to see fresh chain counts or an agent's latest \
                                     thoughts."
                                </p>

                                <div class="docs-callout docs-callout-warning">
                                    <strong>"Security note: "</strong>
                                    "The dashboard is a read/write admin interface. Protect it with a \
                                     PIN ("
                                    <code>"MENTISDB_DASHBOARD_PIN"</code>
                                    ") or keep "
                                    <code>"MENTISDB_BIND_HOST=127.0.0.1"</code>
                                    " (the default) so it is never exposed to the network."
                                </div>

    <h3>"PIN Protection"</h3>
                            <p>
                                "Set "
                                <code>"MENTISDB_DASHBOARD_PIN"</code>
                                " to any string. A login page appears automatically. Leave it unset \
                                 for open (localhost-only) access."
                            </p>
                            <div class="code-block">
                                <code>"MENTISDB_DASHBOARD_PIN=my-secret-pin mentisdbd"</code>
                            </div>
                            <p>
                                "PIN verification uses constant-time comparison ("
                                <code>"subtle::ConstantTimeEq"</code>
                                ") to prevent timing attacks — the server does not leak information \
                                 about how many characters matched."
                            </p>

                            <h3>"Login Rate Limiting"</h3>
                            <p>
                                "The "
                                <code>"/dashboard/login"</code>
                                " endpoint enforces rate limiting: 5 failed PIN attempts per IP \
                                 address within a 5-minute window results in "
                                <strong>"HTTP 429 Too Many Requests"</strong>
                                ". Successful attempts reset the counter."
                            </p>

                            <h3>"Disabling the Dashboard"</h3>
                                <div class="code-block">
                                    <code>"MENTISDB_DASHBOARD_PORT=0 mentisdbd"</code>
                                </div>

                                <h3>"Sections"</h3>

                                <h4>"Chain Manager"</h4>
                                <p>
                                    "Lists all chains with live thought counts and agent counts. Click \
                                     a chain to open its Thought Explorer. You can bootstrap a new chain \
                                     or delete an existing one (a type-to-confirm safety gate prevents \
                                     accidental deletion). Click "
                                    <em>"↺ Refresh"</em>
                                    " to reload live counts."
                                </p>

                                <h4>"Thought Explorer"</h4>
                                <p>
                                    "Paginated table of all thoughts in a chain. Filter by any of the \
                                     30 ThoughtTypes using the grouped filter panel — each type is shown \
                                     with a coloured badge. The explorer also supports chain-scoped text \
                                     search plus a live agent dropdown; when text search is active it \
                                     returns hybrid ranked results and grouped context bundles instead of \
                                     a plain substring list. If a managed vector sidecar is enabled for \
                                     the chain, the ranking transparently blends lexical, graph, and \
                                     semantic vector signals. Click any row to open a detail modal showing \
                                     the full thought content, metadata, positional back-references \
                                     (displayed as "
                                    <em>"#N"</em>
                                    "), and typed relations (displayed as "
                                    <em>"kind → target_id (chain: other-chain)"</em>
                                    " for cross-chain edges), plus ranked-search provenance such as \
                                     score breakdowns, matched terms, graph distance, and bundle support \
                                     preview when the row came from search."
                                </p>

                                <h4>"Vector Sidecars"</h4>
                                <p>
                                    "Each chain page also has a "
                                    <strong>"Vector Sidecars"</strong>
                                    " panel for the daemon's managed embedding indexes. By default \
                                     `mentisdbd` keeps a local `local-text-v1` sidecar in sync for each \
                                     chain it opens. Operators can expand the panel to inspect freshness, \
                                     indexed-thought counts, and the sidecar path; disable or re-enable \
                                     append-time sync; run "
                                    <strong>"Sync now"</strong>
                                    "; or "
                                    <strong>"Rebuild from scratch"</strong>
                                    " after an explicit delete-and-recreate confirmation."
                                </p>

                                <h4>"Agent Manager"</h4>
                                <p>
                                    "All registered agents grouped by chain. Click an agent to open \
                                     its detail page where you can edit its display name, description, \
                                     and owner; revoke or re-activate it; add or revoke Ed25519 signing \
                                     keys; browse its most recent thoughts; or copy that agent's memories \
                                     into another chain while preserving its metadata."
                                </p>

                                <h4>"Skills Registry"</h4>
                                <p>
                                    "Browse all skills with their version count and lifecycle status \
                                     (active, deprecated, revoked). Click a skill to view it with three \
                                     tabs: "
                                    <strong>"Rendered"</strong>
                                    " (formatted Markdown), "
                                    <strong>"Source"</strong>
                                    " (raw text), and "
                                    <strong>"Diff"</strong>
                                    " (side-by-side version comparison). Revoke or deprecate a skill \
                                     directly from the UI with a confirmation step."
                                </p>

                                <h3>"Dashboard API"</h3>
                                <p>
                                    "The dashboard UI communicates with the daemon through a set of \
                                     internal browser APIs under "
                                    <code>"/dashboard/api/"</code>
                                    ". These endpoints are "
                                    <strong>"not"</strong>
                                    " intended for external scripting — they are an implementation \
                                     detail of the single-page dashboard. All endpoints require PIN \
                                     authentication when "
                                    <code>"MENTISDB_DASHBOARD_PIN"</code>
                                    " is set."
                                </p>

                                <h4>"Chains"</h4>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Method"</th>
                                            <th>"Path"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/chains"</code></td>
                                            <td>"List all chains with live thought and agent counts"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/chains"</code></td>
                                            <td>"Bootstrap a new chain; appends a Summary checkpoint if empty"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/chains/{chain_key}"</code></td>
                                            <td>"Chain detail including vector sidecar statuses"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"DELETE"</code></td>
                                            <td><code>"/chains/{chain_key}"</code></td>
                                            <td>"Permanently delete a chain and deregister it"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/chains/merge"</code></td>
                                            <td>"Merge all thoughts from source into target chain, then delete source"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/chains/{chain_key}/import-markdown"</code></td>
                                            <td>"Import a MEMORY.md-formatted document as new thoughts"</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"Vector Sidecars"</h4>
                                <table class="config-table">
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
                                            <td><code>"/chains/{chain_key}/vectors/{provider_key}/enable"</code></td>
                                            <td>"Enable append-time sync for a managed vector sidecar"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/chains/{chain_key}/vectors/{provider_key}/disable"</code></td>
                                            <td>"Disable append-time sync for a managed vector sidecar"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/chains/{chain_key}/vectors/{provider_key}/sync"</code></td>
                                            <td>"Run an immediate sync pass for a managed vector sidecar"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/chains/{chain_key}/vectors/{provider_key}/rebuild"</code></td>
                                            <td>"Rebuild a vector sidecar from scratch (requires confirm_delete)"</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"Thoughts & Search"</h4>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Method"</th>
                                            <th>"Path"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/chains/{chain_key}/thoughts"</code></td>
                                            <td>"Paginated thought listing, filterable by ThoughtType and scope"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/chains/{chain_key}/search"</code></td>
                                            <td>"Ranked hybrid search with context bundles when text is provided. Supports as_of for point-in-time queries and scope for scope-filtered results"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/chains/{chain_key}/search/bundles"</code></td>
                                            <td>"Seed-anchored context bundles for a search query. Supports as_of and scope parameters"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/chains/{chain_key}/search/agents"</code></td>
                                            <td>"Live thought authors merged with registry display names"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/thoughts/{chain_key}/{thought_id}"</code></td>
                                            <td>"Retrieve a single thought by UUID"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/chains/{chain_key}/agents/{agent_id}/thoughts"</code></td>
                                            <td>"Paginated thoughts authored by a specific agent"</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"Agents"</h4>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Method"</th>
                                            <th>"Path"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/agents"</code></td>
                                            <td>"All registered agents across all chains, grouped by chain"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/agents"</code></td>
                                            <td>"Create or update an agent registry entry"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/agents/{chain_key}"</code></td>
                                            <td>"All agents registered on a specific chain"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}"</code></td>
                                            <td>"Single agent detail with live thought count"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"PATCH"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}"</code></td>
                                            <td>"Update display name, description, or owner of an agent"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}/revoke"</code></td>
                                            <td>"Mark an agent as revoked"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}/activate"</code></td>
                                            <td>"Mark an agent as active"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}/keys"</code></td>
                                            <td>"Register a new Ed25519 public verification key on an agent"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"DELETE"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}/keys/{key_id}"</code></td>
                                            <td>"Revoke a public key from an agent"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}/memory-markdown"</code></td>
                                            <td>"Export an agent's thoughts as a MEMORY.md Markdown document"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/agents/{chain_key}/{agent_id}/copy-to/{target_chain_key}"</code></td>
                                            <td>"Copy all of an agent's thoughts to another chain"</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"Skills"</h4>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Method"</th>
                                            <th>"Path"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/skills"</code></td>
                                            <td>"List all registered skills with version count and status"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/skills"</code></td>
                                            <td>"Upload a new skill version (Markdown or JSON)"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/skills/{skill_id}"</code></td>
                                            <td>"Get skill summary and latest content"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/skills/{skill_id}/versions"</code></td>
                                            <td>"Full version history for a skill"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/skills/{skill_id}/diff"</code></td>
                                            <td>"Unified diff between two skill versions"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/skills/{skill_id}/revoke"</code></td>
                                            <td>"Mark a skill as revoked (content preserved for audit)"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"POST"</code></td>
                                            <td><code>"/skills/{skill_id}/deprecate"</code></td>
                                            <td>"Mark a skill as deprecated"</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"Version"</h4>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Method"</th>
                                            <th>"Path"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"GET"</code></td>
                                            <td><code>"/version"</code></td>
                                            <td>"Returns the crate version baked in at compile time"</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <div class="docs-callout docs-callout-warning">
                                    <strong>"Note: "</strong>
                                    "All paths are relative to "
                                    <code>"/dashboard/api"</code>
                                    ". When a PIN is set, every endpoint requires either an "
                                    <code>"Authorization: Bearer &lt;pin&gt;"</code>
                                    " header or a valid "
                                    <code>"mentisdb_pin"</code>
                                    " cookie. Without a PIN the dashboard is open on localhost."
                                </div>
                            </section>


                            // ── Import MEMORY.md ─────────────────────────────────────
                            <section class="docs-section" id="import-memory-md">
                                <h2 id="import-memory-md">"Import MEMORY.md"</h2>
                                <p>
                                    "The chain detail page in the dashboard has a "
                                    <strong>"📥 Import MEMORY.md"</strong>
                                    " button. Use it to bulk-import an existing Markdown memory file \
                                     into the chain — no CLI or API calls required."
                                </p>

                                <h3>"How to use it"</h3>
                                <ol>
                                    <li>
                                        "Open the dashboard at "
                                        <code>"https://127.0.0.1:9475/dashboard"</code>
                                        " and click into a chain."
                                    </li>
                                    <li>
                                        "Click "
                                        <strong>"📥 Import MEMORY.md"</strong>
                                        " — a modal appears."
                                    </li>
                                    <li>
                                        "Paste your Markdown content into the text area."
                                    </li>
                                    <li>
                                        "Enter a "
                                        <strong>"Default Agent ID"</strong>
                                        " — all imported thoughts are attributed to this agent. \
                                         The agent must already be registered in the chain."
                                    </li>
                                    <li>
                                        "Click "
                                        <strong>"Import"</strong>
                                        " — the dashboard reports how many thoughts were created."
                                    </li>
                                </ol>

                                <h3>"Expected format"</h3>
                                <p>
                                    "Any Markdown file where each top-level or second-level heading \
                                     introduces a distinct memory. The "
                                    <code>"mentisdb_memory_markdown"</code>
                                    " MCP tool produces this format automatically, as does any \
                                     MentisDB export. Example:"
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#"## LessonLearned — 2025-01-10
Use `signal()` not `create_signal()` in Leptos 0.7.
All `create_*` APIs were removed in the 0.7 redesign.

## Decision — 2025-01-11
Adopted binary storage adapter for production.
Rationale: binary is the only supported format for new chains."#}</code></pre>
                                </div>
                                <p>
                                    "After import the memories are fully indexed — searchable, \
                                     filterable by ThoughtType, and attributable by agent — exactly \
                                     like any thought appended via MCP or API."
                                </p>

                                <div class="docs-callout docs-callout-tip">
                                    <strong>"Migrating from a MEMORY.md workflow: "</strong>
                                    "If you previously kept project memory in a hand-edited "
                                    <code>"MEMORY.md"</code>
                                    " file, use this button once to seed your chain. From that point \
                                     forward let MentisDB be the source of truth — the agent writes \
                                     directly to the chain, and you export with "
                                    <code>"mentisdb_memory_markdown"</code>
                                    " whenever you need a portable snapshot."
                                </div>
                            </section>

                            // ── Backup & Restore ──────────────────────────────────
                            <section class="docs-section" id="backup-restore">
                                <h2 id="backup-restore">"Backup & Restore"</h2>
                                <p>
                                    "MentisDB stores all chain data on disk under "
                                    <code>"~/.cloudllm/mentisdb/"</code>
                                    ". Back up that directory at any time using the built-in "
                                    <code>"mentisdbd backup"</code>
                                    " command, and restore from a backup with "
                                    <code>"mentisdbd restore"</code>
                                    "."
                                </p>

                                <h3>"Archive format"</h3>
                                <p>
                                    "Backups are standard ZIP archives with a "
                                    <code>".mentis"</code>
                                    " extension. Each archive contains a SHA-256 manifest ("
                                    <code>"MENTISDB_MANIFEST.txt"</code>
                                    ") listing every file path and its digest, so integrity can be verified after download or copy."
                                </p>

                                <h3>"mentisdbd backup"</h3>
                                <div class="code-block">
                                    <code>"mentisdbd backup [-o &lt;path&gt;] [--dir &lt;path&gt;] [--flush] [--include-tls]"</code>
                                </div>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Argument"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"source_dir"</code></td>
                                            <td>"Path to the MentisDB data directory to back up (e.g. "
                                                <code>"~/.cloudllm/mentisdb"</code>
                                                ")"</td>
                                        </tr>
                                        <tr>
                                            <td><code>"output_path"</code></td>
                                            <td>"Optional output path for the "
                                                <code>".mentis"</code>
                                                " archive. Defaults to "
                                                <code>"~/.cloudllm/mentisdb/backup-&lt;timestamp&gt;.mentis"</code>
                                                "."</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"Options"</h4>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Flag"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"--flush"</code></td>
                                            <td>
                                                "Detects if "
                                                <code>"mentisdbd"</code>
                                                " is running on the local machine. If so, calls "
                                                <code>"POST /v1/admin/flush"</code>
                                                " to force a durability flush before archiving. The backup then proceeds with the daemon either stopped or freshly flushed. Use this to ensure the archive captures all committed thoughts."
                                            </td>
                                        </tr>
                                        <tr>
                                            <td><code>"--include-tls"</code></td>
                                            <td>
                                                "Include TLS certificate and private key files in the archive. By default these are excluded (they are machine-specific and should be re-generated on the target machine). This flag is opt-in so you consciously choose to include them."
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"mentisdbd restore"</h3>
                                <div class="code-block">
                                    <code>"mentisdbd restore &lt;archive.mentis&gt; [--dir &lt;path&gt;] [--overwrite] [--yes]"</code>
                                </div>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Argument"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"archive_path"</code></td>
                                            <td>"Path to the "
                                                <code>".mentis"</code>
                                                " archive to restore."</td>
                                        </tr>
                                        <tr>
                                            <td><code>"target_dir"</code></td>
                                            <td>"Directory to extract the archive into."</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"Options"</h4>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Flag"</th>
                                            <th>"Description"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"--overwrite"</code></td>
                                            <td>"Replace existing files without prompting when there is a conflict."</td>
                                        </tr>
                                        <tr>
                                            <td><code>"--yes"</code></td>
                                            <td>"Answer yes to all interactive prompts (equivalent to "
                                                <code>"--overwrite"</code>
                                                ")."</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"Interactive restore behavior"</h3>
                                <p>
                                    "During restore, if any file in the archive already exists in the target directory, "
                                    <code>"mentisdbd restore"</code>
                                    " prompts you to decide what to do with that file. Pass "
                                    <code>"--overwrite"</code>
                                    " or "
                                    <code>"--yes"</code>
                                    " to skip the prompt and overwrite unconditionally."
                                </p>

                                <h3>"Example commands"</h3>
                                <div class="code-block">
                                    <pre><code>{r#"# Create a backup (archive written to ./mentisdb-2026-04-28-153022.mentis)
mentisdbd backup

# Create a backup to a specific path
mentisdbd backup -o /tmp/my-mentisdb-backup.mentis

# Create a backup from a specific source directory
mentisdbd backup --dir ~/.cloudllm/mentisdb -o /tmp/backup.mentis

# Create a backup with a running daemon flush first
mentisdbd backup --flush

# Include TLS material in the backup (machine-specific — restore on same machine)
mentisdbd backup --include-tls

# Restore a backup (prompts for existing files, daemon must be stopped)
mentisdbd restore /tmp/my-mentisdb-backup.mentis

# Restore to a specific directory
mentisdbd restore /tmp/my-mentisdb-backup.mentis --dir ~/.cloudllm/mentisdb

# Restore, overwriting any conflicting files without prompting
mentisdbd restore /tmp/my-mentisdb-backup.mentis --overwrite"#}</code></pre>
                                </div>

                                <h3>"Security note on --include-tls"</h3>
                                <p>
                                    "TLS certificates and private keys are machine-specific. Including them in a backup is only appropriate when restoring to the same physical machine. If you restore to a new machine, omit "
                                    <code>"--include-tls"</code>
                                    " and let MentisDB auto-generate a fresh certificate on first start — then re-trust it on that machine."
                                </p>

                                <div class="docs-callout docs-callout-tip">
                                    <strong>"Back up before destructive operations."</strong>
                                    " Run "
                                    <code>"mentisdbd backup"</code>
                                    " before any chain merge, chain deletion, skill revocation, or daemon self-update that involves storage format changes. Backups take only seconds and let you recover exactly where you were if something goes wrong."
                                </div>
                            </section>

                            // ── Connecting AI Tools ──────────────────────────────────
                            <section class="docs-section" id="connecting">
                                <h2 id="connecting">"Connecting AI Tools"</h2>
                                <p>
                                    "Once the daemon is running, connect your AI tools via MCP. The fastest \
                                     path is to let MentisDB detect what is installed and configure it \
                                     automatically. Alternatively, configure any tool manually."
                                </p>

                                // ── Automated setup ──────────────────────────────────
                                <h3 id="connecting-automated">"Automated Setup (Recommended)"</h3>
                                <p>
                                    "MentisDB ships with two built-in commands that detect which clients are \
                                     installed on your machine and write the correct MCP configuration for you."
                                </p>

                                <h4>"Setup Wizard"</h4>
                                <p>
                                    "Interactive. Scans your machine, shows every detected tool, lets you \
                                     choose which to configure, and applies changes with your confirmation."
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd wizard"</code>
                                </div>
                                <p>
                                    "Accept all defaults and skip already-configured integrations (non-interactive):"
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd wizard --yes"</code>
                                </div>
                                <p>
                                    "Point all selected integrations at a custom MCP URL:"
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd wizard --url https://my.mentisdb.com:9473"</code>
                                </div>

                                <h4>"Setup One Agent"</h4>
                                <p>
                                    "Target a specific integration by name. Prints a plan first, then writes \
                                     the config file. Use "
                                    <code>"--dry-run"</code>
                                    " to preview without touching anything."
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd setup claude-code"</code>
                                </div>
                                <p>"Setup all detected agents at once:"</p>
                                <div class="code-block">
                                    <code>"mentisdbd setup all"</code>
                                </div>
                                <p>"Preview what would be written without writing anything:"</p>
                                <div class="code-block">
                                    <code>"mentisdbd setup all --dry-run"</code>
                                </div>
                                <p>"Use a custom MCP URL:"</p>
                                <div class="code-block">
                                    <code>"mentisdbd setup all --url https://my.mentisdb.com:9473"</code>
                                </div>

                                <h4>"Supported integrations"</h4>
                                <p>
                                    "Use any of these names with "
                                    <code>"mentisdbd setup"</code>
                                    ":"
                                </p>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Name"</th>
                                            <th>"Tool"</th>
                                            <th>"Config location"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"claude-code"</code></td>
                                            <td>"Claude Code CLI"</td>
                                            <td><code>"~/.claude.json"</code></td>
                                        </tr>
                                        <tr>
                                            <td><code>"claude-desktop"</code></td>
                                            <td>"Claude for Desktop"</td>
                                            <td><code>"~/Library/Application Support/Claude/claude_desktop_config.json"</code></td>
                                        </tr>
                                        <tr>
                                            <td><code>"codex"</code></td>
                                            <td>"OpenAI Codex CLI"</td>
                                            <td><code>"~/.codex/config.toml"</code></td>
                                        </tr>
                                        <tr>
                                            <td><code>"copilot"</code></td>
                                            <td>"GitHub Copilot CLI"</td>
                                            <td><code>"~/.copilot/mcp-config.json"</code></td>
                                        </tr>
                                        <tr>
                                            <td><code>"gemini"</code></td>
                                            <td>"Google Gemini CLI"</td>
                                            <td><code>"~/.gemini/settings.json"</code></td>
                                        </tr>
                                        <tr>
                                            <td><code>"opencode"</code></td>
                                            <td>"OpenCode"</td>
                                            <td><code>"~/.config/opencode/opencode.json"</code></td>
                                        </tr>
                                        <tr>
                                            <td><code>"qwen"</code></td>
                                            <td>"Qwen Code Assistant"</td>
                                            <td><code>"~/.qwen/settings.json"</code></td>
                                        </tr>
                                        <tr>
                                            <td><code>"vscode-copilot"</code></td>
                                            <td>"VS Code + Copilot"</td>
                                            <td>
                                                <code>"~/Library/Application Support/Code/User/mcp.json"</code>
                                                " (macOS) · "
                                                <code>"~/.config/Code/User/mcp.json"</code>
                                                " (Linux) · "
                                                <code>"%APPDATA%\\Code\\User\\mcp.json"</code>
                                                " (Windows)"
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h4>"How it works"</h4>
                                <p>
                                    "Both commands use "
                                    <code>"PathEnvironment"</code>
                                    " to resolve paths consistently — honouring "
                                    <code>"HOME"</code>
                                    ", "
                                    <code>"XDG_CONFIG_HOME"</code>
                                    ", "
                                    <code>"USERPROFILE"</code>
                                    ", and "
                                    <code>"APPDATA"</code>
                                    " on their respective platforms. They detect whether an integration is \
                                     already configured, is installed but unconfigured, or is not present, \
                                     and adapt their output accordingly. They never overwrite an existing \
                                     MentisDB entry without explicit confirmation."
                                </p>

                                // ── Manual setup ────────────────────────────────────
                                <h3 id="connecting-manual">"Manual MCP Configuration"</h3>
                                <p>
                                    "If you prefer to configure manually, or need a custom URL, copy the \
                                     relevant snippet below. All tools connect to "
                                    <code>"http://127.0.0.1:9471"</code>
                                    " by default (HTTP, localhost). After trusting the self-signed TLS \
                                     certificate you can use "
                                    <code>"https://my.mentisdb.com:9473"</code>
                                    " instead."
                                </p>

                                <h4>"Claude for Desktop"</h4>
                                <p>
                                    "Claude for Desktop supports two connection modes. "
                                    <strong>"Stdio mode (recommended)"</strong>
                                    " requires no daemon, no Node.js, and no mcp-remote — just point \
                                     Claude Desktop at the "
                                    <code>"mentisdbd"</code>
                                    " binary. The stdio process automatically detects a running daemon \
                                     and proxies to it, or launches one in the background if none is found."
                                </p>

                                <h5>"Option 1: Stdio mode (recommended)"</h5>
                                <p>"Edit your "
                                    <code>"claude_desktop_config.json"</code>
                                    " and add:"
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcpServers": {
    "mentisdb": {
      "command": "mentisdbd",
      "args": ["--mode", "stdio"]
    }
  }
}"#}</code></pre>
                                </div>
                                <p>"That's it. No daemon pre-start, no TLS config, no Node.js dependency. \
                                 The stdio process handles everything."</p>

                                <h5>"Option 2: HTTP via mcp-remote"</h5>
                                <p>
                                    "If you prefer the HTTP transport (e.g. for a remote daemon), use the "
                                    <a href="https://www.npmjs.com/package/mcp-remote" target="_blank" rel="noopener noreferrer">
                                        <code>"mcp-remote"</code>
                                    </a>
                                    " bridge. This requires Node.js >= 20."
                                </p>

                                <p>"Install mcp-remote globally:"</p>
                                <div class="code-block">
                                    <pre><code>"npm install -g mcp-remote"</code></pre>
                                </div>

                                <p>"Config file location by OS:"</p>
                                <ul>
                                    <li>
                                        <strong>"macOS: "</strong>
                                        <code>"~/Library/Application Support/Claude/claude_desktop_config.json"</code>
                                    </li>
                                    <li>
                                        <strong>"Windows: "</strong>
                                        <code>"%APPDATA%\\Claude\\claude_desktop_config.json"</code>
                                    </li>
                                    <li>
                                        <strong>"Linux: "</strong>
                                        <code>"~/.config/Claude/claude_desktop_config.json"</code>
                                    </li>
                                </ul>

                                <h5>"macOS (mcp-remote)"</h5>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcpServers": {
    "mentisdb": {
      "command": "/opt/homebrew/bin/mcp-remote",
      "args": ["https://my.mentisdb.com:9473"],
      "env": { "NODE_TLS_REJECT_UNAUTHORIZED": "0" }
    }
  }
}"#}</code></pre>
                                </div>
                                <div class="docs-callout">
                                    "The "
                                    <code>"NODE_TLS_REJECT_UNAUTHORIZED: \"0\""</code>
                                    " env var is required because MentisDB uses a self-signed TLS \
                                     certificate. Node.js rejects self-signed certs by default; this \
                                     env var disables that check for the mcp-remote process only. \
                                     Alternatively, add the MentisDB cert to your system keychain (see \
                                     the HTTPS & TLS section above) and remove the env block."
                                </div>
                                <p>
                                    "If you installed Node.js via "
                                    <code>"nvm"</code>
                                    " or a non-Homebrew path, find the mcp-remote binary with:"
                                </p>
                                <div class="code-block">
                                    <pre><code>"which mcp-remote"</code></pre>
                                </div>
                                <p>"and use that full path as the "
                                    <code>"command"</code>
                                    " value."
                                </p>

                                <h5>"Windows (mcp-remote)"</h5>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcpServers": {
    "mentisdb": {
      "command": "mcp-remote",
      "args": ["https://my.mentisdb.com:9473"],
      "env": { "NODE_TLS_REJECT_UNAUTHORIZED": "0" }
    }
  }
}"#}</code></pre>
                                </div>
                                <p>
                                    "On Windows, "
                                    <code>"npm install -g mcp-remote"</code>
                                    " places the binary on your PATH automatically. \
                                     If Claude Desktop cannot find it, supply the full path, e.g.:"
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#""command": "C:\\Users\\YourName\\AppData\\Roaming\\npm\\mcp-remote.cmd""#}</code></pre>
                                </div>

                                <h5>"Linux (mcp-remote)"</h5>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcpServers": {
    "mentisdb": {
      "command": "/usr/local/bin/mcp-remote",
      "args": ["https://my.mentisdb.com:9473"],
      "env": { "NODE_TLS_REJECT_UNAUTHORIZED": "0" }
    }
  }
}"#}</code></pre>
                                </div>
                                <p>
                                    "Verify the path with "
                                    <code>"which mcp-remote"</code>
                                    " and substitute it if different."
                                </p>

                                <div class="docs-callout">
                                    "Restart Claude for Desktop after saving the config file for \
                                     changes to take effect."
                                </div>

                                <h4>"Claude Code"</h4>
                                <p>
                                    "`mentisdbd setup claude-code` writes the MCP entry under "
                                    <code>"mcpServers.mentisdb"</code>
                                    " in "
                                    <code>"~/.claude.json"</code>
                                    " (or the platform-equivalent home directory on Windows). The older "
                                    <code>"~/.claude/mcp/mentisdb.json"</code>
                                    " file is treated as a legacy companion path, not the canonical target."
                                </p>
                                <div class="code-block">
                                    <code>
                                        "claude mcp add --transport http mentisdb http://127.0.0.1:9471"
                                    </code>
                                </div>

                                <h4>"OpenAI Codex"</h4>
                                <div class="code-block">
                                    <code>"codex mcp add mentisdb --url http://127.0.0.1:9471"</code>
                                </div>

                                <h4>"GitHub Copilot CLI"</h4>
                                <p>
                                    "Add to "
                                    <code>"~/.copilot/mcp-config.json"</code>
                                    ":"
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcpServers": {
    "mentisdb": {
      "type": "http",
      "url": "http://127.0.0.1:9471",
      "headers": {},
      "tools": ["*"]
    }
  }
}"#}</code></pre>
                                </div>
                                <p>"Or using HTTPS after trusting the certificate:"</p>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcpServers": {
    "mentisdb": {
      "type": "http",
      "url": "https://my.mentisdb.com:9473",
      "headers": {},
      "tools": ["*"]
    }
  }
}"#}</code></pre>
                                </div>

                                <h4>"Qwen Code"</h4>
                                <div class="code-block">
                                    <code>
                                        "qwen mcp add --transport http mentisdb http://127.0.0.1:9471"
                                    </code>
                                </div>

                                <h4>"OpenCode"</h4>
                                <p>
                                    "OpenCode stores MCP configuration in "
                                    <code>"~/.config/opencode/opencode.json"</code>
                                    " on Linux and macOS. Add the "
                                    <code>"mentisdb"</code>
                                    " block under the top-level "
                                    <code>"mcp"</code>
                                    " key:"
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcp": {
    "mentisdb": {
      "type": "remote",
      "url": "http://127.0.0.1:9471",
      "enabled": true
    }
  }
}"#}</code></pre>
                                </div>
                                <p>"Or using HTTPS after trusting the certificate:"</p>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcp": {
    "mentisdb": {
      "type": "remote",
      "url": "https://my.mentisdb.com:9473",
      "enabled": true
    }
  }
}"#}</code></pre>
                                </div>

                                <h4>"Google Gemini CLI"</h4>
                                <p>
                                    "Gemini CLI reads MCP server configuration from "
                                    <code>"~/.gemini/settings.json"</code>
                                    ". Add the "
                                    <code>"mentisdb"</code>
                                    " block under the top-level "
                                    <code>"mcpServers"</code>
                                    " key. The "
                                    <code>"httpUrl"</code>
                                    " field is required alongside "
                                    <code>"url"</code>
                                    " for HTTP transport:"
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "mcpServers": {
    "mentisdb": {
      "type": "http",
      "url": "http://127.0.0.1:9471",
      "httpUrl": "http://127.0.0.1:9471"
    }
  }
}"#}</code></pre>
                                </div>

                                <h4>"VS Code + Copilot"</h4>
                                <p>
                                    "VS Code stores MCP server configuration in "
                                    <code>"mcp.json"</code>
                                    " inside the VS Code user settings directory. The path varies by OS:"
                                </p>
                                <ul>
                                    <li>
                                        <strong>"macOS: "</strong>
                                        <code>"~/Library/Application Support/Code/User/mcp.json"</code>
                                    </li>
                                    <li>
                                        <strong>"Linux: "</strong>
                                        <code>"~/.config/Code/User/mcp.json"</code>
                                    </li>
                                    <li>
                                        <strong>"Windows: "</strong>
                                        <code>"%APPDATA%\\Code\\User\\mcp.json"</code>
                                    </li>
                                </ul>
                                <p>
                                    "Add the "
                                    <code>"mentisdb"</code>
                                    " block under the top-level "
                                    <code>"servers"</code>
                                    " key:"
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#"{
  "servers": {
    "mentisdb": {
      "type": "http",
      "url": "http://127.0.0.1:9471"
    }
  }
}"#}</code></pre>
                                </div>
                            </section>

                            // ── Chain Topologies ─────────────────────────────────────
                            <section class="docs-section" id="chain-topologies">
                                <h2 id="chain-topologies">"Chain Topologies"</h2>
                                <p>
                                    "MentisDB supports multiple named chains. Each chain is an \
                                     independent, append-only ledger. You choose how to map agents \
                                     to chains based on your team structure and privacy needs."
                                </p>

                                <h3>"One agent, one chain (simplest)"</h3>
                                <p>
                                    "A single agent uses the default chain for everything. All memories \
                                     accumulate in one place. Perfect for a solo developer with one \
                                     long-running assistant."
                                </p>
                                <div class="docs-callout">
                                    <code>"chain_key: \"default\""</code>
                                    " — one brain, one history."
                                </div>

                                <h3>"One agent, multiple chains (context isolation)"</h3>
                                <p>
                                    "The same agent writes to different chains depending on its current \
                                     assignment. Work memories stay scoped to the right project and don't \
                                     pollute unrelated contexts."
                                </p>
                                <div class="docs-callout">
                                    <p>"Orion writes project lessons to "
                                        <code>"\"project-alpha\""</code>
                                        " and company-wide conventions to "
                                        <code>"\"company-conventions\""</code>
                                        "."
                                    </p>
                                </div>
                                <ul>
                                    <li>"Each chain has its own agent registry, thought ledger, and skill registry"</li>
                                    <li>"An agent can read from and write to as many chains as needed"</li>
                                    <li>"Set "
                                        <code>"MENTISDB_DEFAULT_CHAIN_KEY"</code>
                                        " so the most-used chain requires no explicit "
                                        <code>"chain_key"</code>
                                        " parameter"
                                    </li>
                                </ul>

                                <h3>"Many agents, one shared chain (fleet / organisation)"</h3>
                                <p>
                                    "All agents write to the same chain. Every agent can optionally read \
                                     thoughts written by its peers by filtering on "
                                    <code>"agent_ids"</code>
                                    " in search queries. This is the foundation of fleet coordination."
                                </p>
                                <div class="docs-callout">
                                    <p>
                                        "Apollo, Orion, and Astro all write to "
                                        <code>"\"project-alpha\""</code>
                                        ". Each uses its own "
                                        <code>"agent_id"</code>
                                        ". A query without an agent filter returns memories from all three."
                                    </p>
                                </div>

                                <h3>"Many agents, many chains (per-team or per-tenant)"</h3>
                                <p>
                                    "Use separate chains for separate teams, departments, or clients. \
                                     Agents that work across teams carry context between chains by \
                                     reading from one and writing a "
                                    <code>"Summary"</code>
                                    " to another. No data leaks between chains unless you explicitly \
                                     bridge them."
                                </p>
                            </section>

                            // ── Fleet Coordination ───────────────────────────────────
                            <section class="docs-section" id="fleet-coordination">
                                <h2 id="fleet-coordination">"Fleet Coordination via the Thought Chain"</h2>
                                <p>
                                    "When your harness can spawn background sub-agents in parallel, \
                                     MentisDB's shared chain becomes a "
                                    <strong>"coordination primitive"</strong>
                                    " — a lightweight alternative to explicit message passing between \
                                     agents."
                                </p>
                                <div class="docs-video">
                                    <p class="docs-video-caption">
                                        "Prefer a walkthrough first? This tutorial shows the fleet \
                                         coordination flow end to end."
                                    </p>
                                    <div class="docs-video-frame">
                                        <iframe
                                            src="https://www.youtube.com/embed/cFvq4pVHSU8"
                                            title="MentisDB fleet coordination tutorial"
                                            loading="lazy"
                                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                                            referrerpolicy="strict-origin-when-cross-origin"
                                            allowfullscreen="true"
                                        ></iframe>
                                    </div>
                                </div>

                                <h3>"The pattern"</h3>
                                <ol>
                                    <li>
                                        "The "
                                        <strong>"PM agent"</strong>
                                        " decomposes the assignment into a task graph, writes each task \
                                         as a "
                                        <code>"Plan"</code>
                                        " or "
                                        <code>"Subgoal"</code>
                                        " thought tagged "
                                        <code>"\"task-pending\""</code>
                                        ", then spawns a specialist sub-agent per task."
                                    </li>
                                    <li>
                                        "Each "
                                        <strong>"specialist agent"</strong>
                                        " calls "
                                        <code>"mentisdb_recent_context"</code>
                                        " on start to load the task graph and any shared constraints, \
                                         then works autonomously."
                                    </li>
                                    <li>
                                        "When a specialist finishes, it writes a "
                                        <code>"TaskComplete"</code>
                                        " thought tagged "
                                        <code>"\"task-done\""</code>
                                        " (and any "
                                        <code>"LessonLearned"</code>
                                        " or "
                                        <code>"Decision"</code>
                                        " thoughts it accumulated)."
                                    </li>
                                    <li>
                                        "The "
                                        <strong>"PM agent"</strong>
                                        " polls the chain (or is notified) and queries "
                                        <code>"tags_any: [\"task-done\"]"</code>
                                        " to check completion status before unblocking dependent tasks."
                                    </li>
                                </ol>

                                <div class="docs-callout docs-callout-tip">
                                    "No message broker needed. The append-only chain "
                                    <em>"is"</em>
                                    " the bus. Every agent reads the same ledger; writes are immediately \
                                     visible to all readers."
                                </div>

                                <h3>"Avoiding conflicts in parallel agents"</h3>
                                <ul>
                                    <li>
                                        "Each agent uses its own "
                                        <code>"agent_id"</code>
                                        " — writes are attributed and never collide"
                                    </li>
                                    <li>
                                        "Agents doing independent work read only their own subtree \
                                         ("
                                        <code>"agent_ids: [\"my-id\"]"</code>
                                        ") and shared constraints ("
                                        <code>"tags_any: [\"constraint\", \"convention\"]"</code>
                                        ") — they don't need to read each other's full history"
                                    </li>
                                    <li>
                                        "Dependent tasks read their blocker's "
                                        <code>"TaskComplete"</code>
                                        " thought to extract outputs before starting"
                                    </li>
                                    <li>
                                        "The PM synthesises results by querying all "
                                        <code>"TaskComplete"</code>
                                        " thoughts in one call at the end"
                                    </li>
                                </ul>

                                <h3>"Example task graph"</h3>
                                <div class="docs-callout">
                                    <pre><code>
            "PM writes:\n\
            Plan  [tag: task-pending, id: design-schema]   → dispatches Orion\n\
            Plan  [tag: task-pending, id: write-tests]     → dispatches Apollo (blocks on design-schema)\n\
            Plan  [tag: task-pending, id: implement-api]   → dispatches Astro  (blocks on design-schema)\n\
            \n\
            Orion finishes, writes:\n\
            TaskComplete [tag: task-done, id: design-schema, content: \"...schema decisions...\"]\n\
            \n\
            PM queries tags_any=[\"task-done\"] → unblocks Apollo + Astro\n\
            Apollo + Astro run in parallel, each reading Orion's TaskComplete for shared context.\n\
            \n\
            PM final query: all TaskComplete thoughts → synthesises result."
                                    </code></pre>
                                </div>
                            </section>

                            // ── The Skills Registry ──────────────────────────────────
                            <section class="docs-section" id="skills-registry">
                                <h2 id="skills-registry">"The Skills Registry"</h2>
                                <p>
                                    "The skills registry is a versioned, immutable store for agent \
                                     instruction bundles (skill files). Think of it like git for your \
                                     agent's operating procedures."
                                </p>

                                <h3>"Uploading a skill"</h3>
                                <p>
                                    "Skills are uploaded as Markdown files. Each upload to an existing \
                                     "
                                    <code>"skill_id"</code>
                                    " creates a new immutable version (stored as a diff):"
                                </p>
                                <p>
                                    "Call "
                                    <code>"mentisdb_upload_skill"</code>
                                    " with three required fields: "
                                    <code>"agent_id"</code>
                                    " (the uploading agent's registered identity), "
                                    <code>"skill_id"</code>
                                    " (a stable slug like "
                                    <code>"\"my-project-conventions\""</code>
                                    "), and "
                                    <code>"content"</code>
                                    " (the raw Markdown of the skill file). If the agent has registered \
                                     public keys, also provide "
                                    <code>"signing_key_id"</code>
                                    " and "
                                    <code>"skill_signature"</code>
                                    " to create a cryptographically verified upload."
                                </p>

                                <h3>"Retrieving a skill"</h3>
                                <p>
                                    "Use "
                                    <code>"mentisdb_read_skill(skill_id)"</code>
                                    " to get the latest version, or pass "
                                    <code>"version_id"</code>
                                    " for a specific historical version. Full version history is always \
                                     preserved."
                                </p>
                            </section>

                            // ── Cryptographic Signatures ─────────────────────────────
                            <section class="docs-section" id="signatures">
                                <h2 id="signatures">"Cryptographic Signatures"</h2>
                                <p>
                                    "Agents with registered Ed25519 public keys must cryptographically \
                                     sign their skill uploads. This creates a verifiable, tamper-evident \
                                     record of authorship."
                                </p>

                                <h3>"Registering an agent key"</h3>
                                <p>
                                    "Use "
                                    <code>"mentisdb_add_agent_key"</code>
                                    " to register an Ed25519 public key for an agent. Once registered, \
                                     all uploads from that agent must include a valid signature over the \
                                     skill content."
                                </p>

                                <h3>"Why this matters"</h3>
                                <p>
                                    "Signed skills mean you always know which agent authored which \
                                     version. Combined with the immutable version history, this creates a \
                                     cryptographically auditable record of your fleet's institutional \
                                     knowledge."
                                </p>
                            </section>

                            // ── Memory Scopes ──────────────────────────────────────
                            <section class="docs-section" id="memory-scopes">
                                <h2 id="memory-scopes">"Memory Scopes"</h2>
                                <p>
                                    "MentisDB 0.8.2 introduces memory scopes — a lightweight way to \
                                     partition thoughts within a single chain. Scopes are stored as \
                                     tags on each thought and let you isolate memories by visibility \
                                     level without creating separate chains."
                                </p>

                                <h3>"Scope levels"</h3>
                                <table class="config-table">
                                    <thead>
                                        <tr>
                                            <th>"Scope"</th>
                                            <th>"Tag"</th>
                                            <th>"Purpose"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr>
                                            <td><code>"User"</code></td>
                                            <td><code>"scope:user"</code></td>
                                            <td>"Visible to the owning user across all sessions. Default scope for most memories."</td>
                                        </tr>
                                        <tr>
                                            <td><code>"Session"</code></td>
                                            <td><code>"scope:session"</code></td>
                                            <td>"Scoped to a single conversation session. Ephemeral working memory — scratch thoughts, in-progress hypotheses."</td>
                                        </tr>
                                        <tr>
                                            <td><code>"Agent"</code></td>
                                            <td><code>"scope:agent"</code></td>
                                            <td>"Private to a specific agent. Not shared with other agents in the fleet. Useful for internal heuristics or private state."</td>
                                        </tr>
                                    </tbody>
                                </table>

                                <h3>"Using scopes"</h3>
                                <p>
                                    "When appending a thought, set the "
                                    <code>"scope"</code>
                                    " parameter to one of "
                                    <code>"User"</code>
                                    ", "
                                    <code>"Session"</code>
                                    ", or "
                                    <code>"Agent"</code>
                                    ". MentisDB stores the scope as a tag (e.g. "
                                    <code>"scope:user"</code>
                                    ") on the thought. In search, use the "
                                    <code>"scope"</code>
                                    " parameter to filter results to a specific scope level."
                                </p>
                                <div class="docs-callout docs-callout-tip">
                                    <strong>"Backward compatible: "</strong>
                                    "Existing thoughts without a scope tag are treated as User-scoped. \
                                     You do not need to migrate or retag existing chains."
                                </div>
                            </section>

                            // ── Temporal Queries ────────────────────────────────────
                            <section class="docs-section" id="temporal-queries">
                                <h2 id="temporal-queries">"Temporal Queries"</h2>
                                <p>
                                    "MentisDB 0.8.2 adds temporal query support, allowing you to \
                                     query the chain as it existed at a specific point in time \
                                     and to set time-bounded validity on relations."
                                </p>

                                <h3>"Point-in-time queries with as_of"</h3>
                                <p>
                                    "Pass "
                                    <code>"as_of"</code>
                                    " (an RFC 3339 timestamp) to search and traversal tools to \
                                     see only thoughts that existed at that time. Thoughts appended \
                                     after the timestamp are excluded from results. This is useful \
                                     for auditing what an agent knew at a specific moment, or for \
                                     reproducing decisions made under a previous state of knowledge."
                                </p>
                                <div class="code-block">
                                    <pre><code>{r#"mentisdb_ranked_search(
  text: "caching strategy",
  as_of: "2025-12-01T00:00:00Z"
)"#}</code></pre>
                                </div>

                                <h3>"Temporal bounds on relations"</h3>
                                <p>
                                    "Thought relations now support "
                                    <code>"valid_at"</code>
                                    " and "
                                    <code>"invalid_at"</code>
                                    " fields — RFC 3339 timestamps that define when a relation \
                                     becomes active and when it expires. A relation is considered \
                                     active if the current time falls between "
                                    <code>"valid_at"</code>
                                    " and "
                                    <code>"invalid_at"</code>
                                    ". If neither field is set, the relation is always active (backward \
                                     compatible with existing chains)."
                                </p>
                                <div class="docs-callout docs-callout-tip">
                                    "Use "
                                    <code>"invalid_at"</code>
                                    " to model time-limited relationships — for example, a \
                                     "
                                    <code>"Supersedes"</code>
                                    " edge that only takes effect after a transition date, or a \
                                     "
                                    <code>"Supports"</code>
                                    " link that expires when a deprecation window closes."
                                </div>
                            </section>

                            // ── Advanced Retrieval ───────────────────────────────────
                            <section class="docs-section" id="advanced-retrieval">
                                <h2 id="advanced-retrieval">"Advanced Retrieval"</h2>
                                <p>
                                    "MentisDB layers four complementary signals over its append-only chain — "
                                    "lexical (BM25), dense-vector (cosine), graph BFS, and session cohesion — "
                                    "and fuses them into a single ranked result set. The full algorithmic pipeline "
                                    "is described in the "
                                    <a href="https://docs.mentisdb.com/ranked-search-pipeline">"ranked-search pipeline blog post"</a>
                                    " and the "
                                    <a href="https://github.com/CloudLLM-ai/mentisdb/blob/master/WHITEPAPER.md">"white paper"</a>
                                    "; this section summarises the knobs you can turn."
                                </p>

                                <h3 id="rrf-reranking">"Reciprocal Rank Fusion (RRF)"</h3>
                                <p>
                                    "Set "<code>"enable_reranking: true"</code>" on any "
                                    <code>"mentisdb_ranked_search"</code>
                                    " call to rerank the top "<code>"rerank_k"</code>" candidates (default 50) by merging "
                                    "three independent rankings — lexical-only, vector-only, and graph-only — through "
                                    "Reciprocal Rank Fusion with damping constant "<code>"k=60"</code>". RRF is robust when "
                                    "the absolute magnitudes of the component scores are not directly comparable."
                                </p>

                                <h3 id="context-bundles">"Context Bundles"</h3>
                                <p>
                                    <code>"mentisdb_context_bundles"</code>
                                    " returns seed-anchored grouped results instead of a flat list. Each bundle pairs "
                                    "one lexical seed with its graph-expanded neighbours in provenance order, so the "
                                    "agent can inspect "<em>"why"</em>" a supporting thought surfaced. Use bundles when "
                                    "you want to preserve evidence groupings rather than collapse everything into "
                                    "ranked rows."
                                </p>

                                <h3 id="vector-sidecars">"Dense-Vector Sidecars"</h3>
                                <p>
                                    "Vector state lives in rebuildable per-chain sidecars partitioned by chain, thought id, "
                                    "model, dimension, and embedding version. The daemon ships the "
                                    <code>"fastembed-minilm"</code>
                                    " provider by default — a 384-dimension MiniLM model running locally via ONNX, "
                                    "with no cloud dependency and no API key. Hybrid ranking blends lexical and cosine "
                                    "scores via a smooth exponential fusion that amplifies pure-semantic matches "
                                    "(~36×) and decays to additive composition as lexical evidence grows."
                                </p>

                                <h3 id="branching-chains">"Branching Chains"</h3>
                                <p>
                                    <code>"mentisdb_branch_from"</code>
                                    " forks a new chain from an existing thought. The branch receives a genesis thought "
                                    "with a "<code>"BranchesFrom"</code>" relation pointing at the fork point; ranked "
                                    "search on the branch transparently includes results from ancestor chains, so "
                                    "experimental or tenant-scoped branches can read shared context without "
                                    "cross-contaminating it. Ancestor discovery is transitive."
                                </p>

                                <h3 id="federated-search">"Federated Cross-Chain Search"</h3>
                                <p>
                                    <code>"mentisdb_federated_search"</code>
                                    " runs one ranked-search query across a list of chains concurrently and returns one "
                                    "merged, deduplicated, re-scored result set. Each hit carries the "
                                    <code>"chain_key"</code>
                                    " it originated from, so multi-agent hubs and cross-organisational memory "
                                    "aggregations can share a single query surface. Per-chain overrides let you "
                                    "apply different filters, limits, or RRF settings per chain."
                                </p>
                            </section>

                            // ── Entity Types & Provenance ────────────────────────────
                            <section class="docs-section" id="entity-types-and-provenance">
                                <h2 id="entity-types-and-provenance">"Entity Types & Provenance"</h2>

                                <h3 id="entity-types">"Per-Chain Entity Types"</h3>
                                <p>
                                    "Attach an "<code>"entity_type"</code>" label to any thought — e.g. "
                                    <code>"\"incident\""</code>", "<code>"\"customer\""</code>", "
                                    <code>"\"deploy\""</code>" — to categorise memory beyond free-form tags. Entity types "
                                    "are registered per chain through "
                                    <code>"mentisdb_upsert_entity_type"</code>
                                    " and discoverable via "
                                    <code>"mentisdb_list_entity_types"</code>
                                    "; each carries an optional description and a usage counter. Ranked search filters "
                                    "by entity type, and the dashboard explorer surfaces them as a first-class facet."
                                </p>

                                <h3 id="source-episode">"source_episode — Derived Memory Provenance"</h3>
                                <p>
                                    "When a thought is derived from a larger episode (a conversation turn, an ingested "
                                    "document, a batch job), set the optional "<code>"source_episode"</code>" field to a "
                                    "stable identifier so every derived thought can later be traced back to its source. "
                                    "Ranked search filters by "<code>"source_episode"</code>" exactly the same way it "
                                    "filters by agent or tag."
                                </p>
                            </section>

                            // ── Webhooks ─────────────────────────────────────────────
                            <section class="docs-section" id="webhooks">
                                <h2 id="webhooks">"Webhook Callbacks"</h2>
                                <p>
                                    "Register an HTTP endpoint that MentisDB will POST to whenever a thought is "
                                    "appended to a chain. Useful for syncing an external index, triggering downstream "
                                    "workflows, or mirroring writes to observability pipelines."
                                </p>
                                <p>
                                    "Delivery is fire-and-forget with exponential-backoff retries (up to 5 attempts). "
                                    "Registrations persist to "<code>"webhooks.json"</code>" next to the chain registry "
                                    "and survive daemon restarts. Fan-out is bounded by a queue and concurrency "
                                    "semaphore, so bursty appends cannot spawn unlimited outgoing tasks."
                                </p>
                                <p>"MCP tools: "
                                    <code>"mentisdb_register_webhook"</code>", "
                                    <code>"mentisdb_list_webhooks"</code>", "
                                    <code>"mentisdb_delete_webhook"</code>
                                    ". REST routes are documented in the developer guide."
                                </p>
                            </section>

                            // ── LLM-Extracted Memories ───────────────────────────────
                            <section class="docs-section" id="llm-extracted-memories">
                                <h2 id="llm-extracted-memories">"LLM-Extracted Memories"</h2>
                                <p>
                                    "Turn raw text — a chat transcript, a pasted document, a ticket comment — into a "
                                    "review-ready slate of typed thoughts. The "
                                    <code>"mentisdb_extract_memories"</code>
                                    " tool calls a configured OpenAI-compatible model, validates the JSON schema of the "
                                    "candidate thoughts, and returns them to the caller. "
                                    <strong>"Nothing is written to the chain until the caller explicitly appends the reviewed candidates."</strong>
                                </p>
                                <p>
                                    "Defaults to "<code>"gpt-4o"</code>", configurable via environment variables. The "
                                    "prompt enforces strict JSON output and the server validates schemas before return, "
                                    "so provider quirks (for example OpenAI-compatible endpoints that reject the "
                                    <code>"response_format"</code>
                                    " hint) cannot poison the chain. See "
                                    <a href="https://github.com/CloudLLM-ai/mentisdb/blob/master/docs/llm-extracted-memories-design.md">"llm-extracted-memories-design.md"</a>
                                    " for the full contract."
                                </p>
                            </section>

                            // ── Python Client ────────────────────────────────────────
                            <section class="docs-section" id="python-client">
                                <h2 id="python-client">"Python Client (pymentisdb)"</h2>
                                <p>
                                    <code>"pymentisdb"</code>
                                    " is the official Python client, published to PyPI. It wraps the REST surface with "
                                    "typed request and response objects and integrates natively with LangChain via "
                                    <code>"MentisDbMemory"</code>
                                    "."
                                </p>
                                <div class="code-block">
                                    <code>"pip install pymentisdb"</code>
                                </div>
                                <div class="code-block">
                                    <code>"from pymentisdb import MentisDbClient, ThoughtType\n\n\
client = MentisDbClient(\"http://127.0.0.1:9472\")\n\
client.append_thought(\n    chain_key=\"my-chain\",\n    agent_id=\"planner\",\n    thought_type=ThoughtType.DECISION,\n    content=\"Adopt LRU eviction for the response cache\",\n)\nhits = client.ranked_search(chain_key=\"my-chain\", text=\"cache eviction\", limit=5)"</code>
                                </div>
                                <p>"See the "
                                    <a href="https://pypi.org/project/pymentisdb/">"PyPI listing"</a>
                                    " and the "
                                    <a href="https://github.com/CloudLLM-ai/mentisdb/tree/master/pymentisdb">"pymentisdb/"</a>
                                    " folder for the full API surface, typed relations, context bundles, and a working "
                                    "LangChain example."
                                </p>
                            </section>

                            // ── CLI Subcommands ──────────────────────────────────────
                            <section class="docs-section" id="cli-subcommands">
                                <h2 id="cli-subcommands">"CLI Subcommands"</h2>
                                <p>
                                    "The "
                                    <code>"mentisdbd"</code>
                                    " binary includes subcommands for interacting with a running daemon
                                     from the terminal. These are useful for quick manual entries, scripting,
                                     and debugging — no MCP client or dashboard needed."
                                </p>
                                <p>"All three subcommands require a running daemon. They connect to the REST port (default "
                                    <code>"http://127.0.0.1:9472"</code>
                                    ")."
                                </p>

                                <h3>"add — Add a thought"</h3>
                                <p>
                                    "Adds a new thought to a chain. Use it for quick notes, scripted entries,
                                     or piping data into MentisDB."
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd add \"The sky is blue\""</code>
                                </div>
                                <div class="code-block">
                                    <code>"mentisdbd add \"Session fact\" --scope session --tag important"</code>
                                </div>
                                <div class="code-block">
                                    <code>"mentisdbd add \"Insight\" --type insight --agent my-agent"</code>
                                </div>
                                <table>
                                    <thead><tr><th>"Option"</th><th>"Description"</th></tr></thead>
                                    <tbody>
                                        <tr><td><code>"--type"</code></td><td>"Thought type (default: fact-learned)"</td></tr>
                                        <tr><td><code>"--scope"</code></td><td>"Memory scope: user, session, or agent"</td></tr>
                                        <tr><td><code>"--tag"</code></td><td>"Add a tag (repeatable)"</td></tr>
                                        <tr><td><code>"--agent"</code></td><td>"Agent ID for the thought"</td></tr>
                                        <tr><td><code>"--chain"</code></td><td>"Chain key (uses daemon default if omitted)"</td></tr>
                                        <tr><td><code>"--url"</code></td><td>"Daemon REST URL (default: http://127.0.0.1:9472)"</td></tr>
                                    </tbody>
                                </table>

                                <h3>"search — Search memories"</h3>
                                <p>
                                    "Searches thoughts using the same ranked retrieval engine as the REST API
                                     and MCP tools. Returns JSON with score breakdowns."
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd search \"cache invalidation\""</code>
                                </div>
                                <div class="code-block">
                                    <code>"mentisdbd search \"performance\" --limit 5 --scope session"</code>
                                </div>
                                <p>
                                    "Pipe results through "
                                    <code>"jq"</code>
                                    " for scripting:"
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd search \"deploy\" --limit 20 | jq '.hits[].thought.content'"</code>
                                </div>
                                <table>
                                    <thead><tr><th>"Option"</th><th>"Description"</th></tr></thead>
                                    <tbody>
                                        <tr><td><code>"--limit"</code></td><td>"Maximum results (default: 10)"</td></tr>
                                        <tr><td><code>"--scope"</code></td><td>"Filter by memory scope: user, session, or agent"</td></tr>
                                        <tr><td><code>"--chain"</code></td><td>"Chain key (uses daemon default if omitted)"</td></tr>
                                        <tr><td><code>"--url"</code></td><td>"Daemon REST URL (default: http://127.0.0.1:9472)"</td></tr>
                                    </tbody>
                                </table>

                                <h3>"agents — List registered agents"</h3>
                                <p>
                                    "Shows agent IDs, display names, status, and thought counts. Useful
                                     for auditing which agents have written to your MentisDB instance."
                                </p>
                                <div class="code-block">
                                    <code>"mentisdbd agents"</code>
                                </div>
                                <div class="code-block">
                                    <code>"mentisdbd agents --chain my-project"</code>
                                </div>
                                <table>
                                    <thead><tr><th>"Option"</th><th>"Description"</th></tr></thead>
                                    <tbody>
                                        <tr><td><code>"--chain"</code></td><td>"Chain key (uses daemon default if omitted)"</td></tr>
                                        <tr><td><code>"--url"</code></td><td>"Daemon REST URL (default: http://127.0.0.1:9472)"</td></tr>
                                    </tbody>
                                </table>
                            </section>

                        </article>
                    </div>
                </section>
            }
}
