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
                        <a class="docs-nav-link" href="#connecting">"Connecting Your AI Tool"</a>
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
                             from the same port."
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
                                <tr>
                                    <td><code>"MENTISDB_DATA_DIR"</code></td>
                                    <td><code>"~/.mentisdb"</code></td>
                                    <td>"Where chain data is stored on disk"</td>
                                </tr>
                                <tr>
                                    <td><code>"MENTISDB_PORT"</code></td>
                                    <td><code>"9471"</code></td>
                                    <td>"Port for the HTTP server"</td>
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
                                <tr>
                                    <td><code>"MENTISDB_DEFAULT_CHAIN"</code></td>
                                    <td><code>"default"</code></td>
                                    <td>
                                        "The chain key used when no "
                                        <code>"chain_key"</code>
                                        " is specified"
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </section>

                    // ── Connecting Your AI Tool ──────────────────────────────
                    <section class="docs-section" id="connecting">
                        <h2 id="connecting">"Connecting Your AI Tool"</h2>
                        <p>"Once the daemon is running, connect your AI coding tool via MCP:"</p>

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

                        <h3>"Qwen Code"</h3>
                        <div class="code-block">
                            <code>
                                "qwen mcp add --transport http mentisdb http://127.0.0.1:9471"
                            </code>
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
