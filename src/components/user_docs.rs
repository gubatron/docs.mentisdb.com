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
                        <a class="docs-nav-link" href="#self-update">"Self-Update"</a>
                        <a class="docs-nav-link" href="#https">"HTTPS & TLS"</a>
                        <a class="docs-nav-link" href="#dashboard">"Web Dashboard"</a>
                        <a class="docs-nav-link" href="#import-memory-md">"Import MEMORY.md"</a>
                        <a class="docs-nav-link" href="#connecting">"Connecting AI Tools"</a>
                        <a class="docs-nav-link" href="#priming">"Priming Your Agent"</a>
                        <a class="docs-nav-link" href="#chain-topologies">"Chain Topologies"</a>
                        <a class="docs-nav-link" href="#fleet-coordination">"Fleet Coordination"</a>
                        <a class="docs-nav-link" href="#skills-registry">"The Skills Registry"</a>
                        <a class="docs-nav-link" href="#signatures">"Cryptographic Signatures"</a>
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
                                    <td><code>"MENTISDB_DEFAULT_STORAGE_ADAPTER"</code></td>
                                    <td><code>"binary"</code></td>
                                    <td>
                                        "Storage format: "
                                        <code>"binary"</code>
                                        " (compact, default) or "
                                        <code>"jsonl"</code>
                                        " (human-readable)"
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
                            </tbody>
                        </table>
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
                             29 ThoughtTypes using the grouped filter panel — each type is shown \
                             with a coloured badge. Click any row to open a detail modal showing \
                             the full thought content, metadata, positional back-references \
                             (displayed as "
                            <em>"#N"</em>
                            "), and typed relations (displayed as "
                            <em>"kind → target_id (chain: other-chain)"</em>
                            " for cross-chain edges)."
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
Rationale: 3x smaller on-disk footprint vs JSONL."#}</code></pre>
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

                    // ── Connecting AI Tools ──────────────────────────────────
                    <section class="docs-section" id="connecting">
                        <h2 id="connecting">"Connecting AI Tools"</h2>
                        <p>"Once the daemon is running, connect your AI coding tool via MCP:"</p>

                        <h3>"Claude for Desktop"</h3>
                        <p>
                            "Claude for Desktop connects to MCP servers via the "
                            <code>"claude_desktop_config.json"</code>
                            " file. MentisDB uses "
                            <a href="https://www.npmjs.com/package/mcp-remote" target="_blank" rel="noopener noreferrer">
                                <code>"mcp-remote"</code>
                            </a>
                            " as the bridge between Claude Desktop and the local MentisDB HTTPS server."
                        </p>

                        <h4>"Step 1 — Install mcp-remote"</h4>
                        <p>"Install it globally with npm (Node.js required):"</p>
                        <div class="code-block">
                            <pre><code>"npm install -g mcp-remote"</code></pre>
                        </div>

                        <h4>"Step 2 — Edit the config file"</h4>
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

                        <h4>"macOS"</h4>
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

                        <h4>"Windows"</h4>
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

                        <h4>"Linux"</h4>
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

                        <h3>"Claude Code"</h3>
                        <div class="code-block">
                            <code>
                                "claude mcp add --transport http mentisdb http://127.0.0.1:9471"
                            </code>
                        </div>

                        <h3>"OpenAI Codex"</h3>
                        <div class="code-block">
                            <code>"codex mcp add mentisdb --url http://127.0.0.1:9471"</code>
                        </div>

                        <h3>"GitHub Copilot CLI"</h3>
                        <p>
                            "Add to "
                            <code>"~/.copilot/mcp-config.json"</code>
                            ":"
                        </p>
                        <div class="code-block">
                            <pre><code>{r#"{
  "servers": {
    "mentisdb": {
      "url": "http://127.0.0.1:9471",
      "type": "http"
    }
  }
}"#}</code></pre>
                        </div>
                        <p>"Or using HTTPS after trusting the certificate:"</p>
                        <div class="code-block">
                            <pre><code>{r#"{
  "servers": {
    "mentisdb": {
      "url": "https://my.mentisdb.com:9473",
      "type": "http"
    }
  }
}"#}</code></pre>
                        </div>

                        <h3>"Qwen Code"</h3>
                        <div class="code-block">
                            <code>
                                "qwen mcp add --transport http mentisdb http://127.0.0.1:9471"
                            </code>
                        </div>
                    </section>

                    // ── Priming Your Agent ───────────────────────────────────
                    <section class="docs-section" id="priming">
                        <h2 id="priming">"Priming Your Agent"</h2>
                        <p>
                            "Before your agent starts any task, tell it about MentisDB so it \
                             knows to write durable memories and load prior context. A simple \
                             opening message goes a long way:"
                        </p>
                        <div class="docs-callout">
                            <p>
                                <em>
                                    "\"You have access to MentisDB via MCP. At the start of each \
                                     session load recent context with mentisdb_recent_context. \
                                     During work, write important decisions, lessons, and \
                                     corrections as thoughts. Before your context fills up, write \
                                     a Summary checkpoint. Your agent_id is 'orion'.\""
                                </em>
                            </p>
                        </div>
                        <p>"You can make this permanent by adding it to your tool's system prompt or project instructions file. Tips:"</p>
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
                                "Ask it to write a "
                                <code>"Summary"</code>
                                " checkpoint before compacting its context or ending a long session"
                            </li>
                        </ul>
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

                </article>
            </div>
        </section>
    }
}
