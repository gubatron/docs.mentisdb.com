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
                                <code>"MENTISDB_DEFAULT_CHAIN"</code>
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
