use leptos::prelude::*;

/// AgentDocs — guide for AI agents on how to use MentisDB effectively as a memory layer.
#[component]
pub fn AgentDocs() -> impl IntoView {
    view! {
        <section class="docs-page">
            <div class="container docs-layout">

                // ── Left sidebar ──────────────────────────────────────────────
                <aside class="docs-sidebar">
                    <nav class="docs-nav">
                        <a class="docs-nav-link" href="#why-write">"Why Write to MentisDB"</a>
                        <a class="docs-nav-link" href="#what-to-write">"What Deserves a Memory"</a>
                        <a class="docs-nav-link" href="#thought-types">"Choosing Thought Types"</a>
                        <a class="docs-nav-link" href="#session-bootstrap">"Session Bootstrap"</a>
                        <a class="docs-nav-link" href="#thought-roles">"Thought Roles"</a>
                        <a class="docs-nav-link" href="#retrieval">"Retrieval Patterns"</a>
                        <a class="docs-nav-link" href="#context-protocol">"Context Window Protocol"</a>
                        <a class="docs-nav-link" href="#fleet">"Fleet Coordination"</a>
                        <a class="docs-nav-link" href="#skills-registry">"Skills Registry"</a>
                        <a class="docs-nav-link" href="#thought-relations">"Thought Relations"</a>
                        <a class="docs-nav-link" href="#import-tool">"Import MEMORY.md Tool"</a>
                        <a class="docs-nav-link" href="#backup-restore">"Backup & Restore"</a>
                        <a class="docs-nav-link" href="#mcp-tools">"MCP Tool Reference"</a>
                        <a class="docs-nav-link" href="#memory-scopes">"Memory Scopes"</a>
                        <a class="docs-nav-link" href="#dashboard">"Web Dashboard"</a>
                    </nav>
                </aside>

                // ── Main content ──────────────────────────────────────────────
                <article class="docs-content">

                    // ── Why Write ────────────────────────────────────────────
                    <section class="docs-section" id="why-write">
                        <h1>"Agent Guide"</h1>
                        <h2 id="why-write">"Why Write to MentisDB"</h2>
                        <p>
                            "Your context window is temporary. MentisDB is permanent. Every \
                             important decision, lesson, or constraint you write to MentisDB is \
                             available to every future version of you — and to every other agent \
                             in your fleet."
                        </p>
                        <p><em>"Write the rule behind the work, not the whole story of the work."</em></p>
                    </section>

                    // ── What Deserves a Memory ───────────────────────────────
                    <section class="docs-section" id="what-to-write">
                        <h2 id="what-to-write">"What Deserves a Memory"</h2>
                        <p>"Write to MentisDB when one of these becomes true:"</p>
                        <ul>
                            <li>
                                "You found a non-obvious bug cause another agent would likely \
                                 hit again"
                            </li>
                            <li>
                                "You made an architectural decision that downstream work should \
                                 not re-litigate"
                            </li>
                            <li>
                                "You discovered a trust boundary, unsafe default, or systemic risk"
                            </li>
                            <li>
                                "You established a stable project convention or naming rule"
                            </li>
                            <li>
                                "You corrected an older assumption that is now dangerous"
                            </li>
                            <li>
                                "You reached a restart point and need the next session to pick \
                                 up quickly"
                            </li>
                        </ul>
                        <p>
                            <strong>"Do NOT write:"</strong>
                            " raw transcripts, every action you took, duplicate git history, \
                             secrets or tokens."
                        </p>
                        <div class="docs-callout docs-callout-tip">
                            <strong>"Auto-dedup: "</strong>
                            "When "
                            <code>"MENTISDB_DEDUP_THRESHOLD"</code>
                            " is set on the server, MentisDB automatically compares each new \
                             thought against recent memories using Jaccard similarity. If a new \
                             thought exceeds the threshold, it is auto-Superseded instead of \
                             appended as a duplicate. You still see the thought in your results, \
                             but it is linked to the prior memory via a "
                            <code>"Supersedes"</code>
                            " relation. Write concisely and specifically to avoid accidental \
                             dedup of distinct memories."
                        </div>
                    </section>

                    // ── Session Bootstrap ───────────────────────────────────
                    <section class="docs-section" id="session-bootstrap">
                        <h2 id="session-bootstrap">"Session Bootstrap"</h2>
                        <p>
                            "When a user (or your system prompt) says:"
                        </p>
                        <div class="code-block">
                            <pre><code>"use mentisdb as your memory system"</code></pre>
                        </div>
                        <p>
                            "perform the following sequence automatically:"
                        </p>
                        <ol>
                            <li>
                                "Call "
                                <code>"mentisdb_list_chains"</code>
                                " to discover existing chains. Match the chain whose name most \
                                 closely corresponds to the current project, repository, or \
                                 working directory. If no chains exist yet, ask the user for a \
                                 name (the repo or project name works well)."
                            </li>
                            <li>
                                "Call "
                                <code>"mentisdb_bootstrap(chain_key=\"<chosen-key>\")"</code>
                                " to open the chain and receive the available skills list. \
                                 This is safe on existing chains — it will not overwrite prior \
                                 memory. Only pass a meaningful "
                                <code>"content"</code>
                                " argument if you are creating a brand-new chain."
                            </li>
                            <li>
                                "Immediately load the core operating instructions via "
                                <code>"resources/read(mentisdb://skill/core)"</code>
                                " (or "
                                <code>"mentisdb_skill_md"</code>
                                " as fallback). Do this "
                                <strong>"before"</strong>
                                " reading any memory — the skill teaches you how to interpret \
                                 and write what you are about to load."
                            </li>
                            <li>
                                "Call "
                                <code>"mentisdb_list_agents"</code>
                                " and pick the existing specialist identity whose role best \
                                 matches the current task. Reuse it — do not mint a new \
                                 identity unless the chain truly lacks the role."
                            </li>
                            <li>
                                "Call "
                                <code>"mentisdb_recent_context"</code>
                                " as that agent to recover where things left off."
                            </li>
                            <li>
                                "Write a "
                                <code>"Summary"</code>
                                " checkpoint to durable memory so subsequent agents can recover \
                                 state without re-priming."
                            </li>
                        </ol>
                        <p>
                            "This is also the recommended pattern to use in your system prompt or \
                             project instructions file — any new agent session will automatically \
                             prime itself without the user needing to ask."
                        </p>
                        <p>
                            "For advanced configurations, you can still specify a "
                            <code>"chain_key"</code>
                            " explicitly during the bootstrap call."
                        </p>
                    </section>

                    // ── Choosing Thought Types ───────────────────────────────
                    <section class="docs-section" id="thought-types">
                        <h2 id="thought-types">"Choosing Thought Types"</h2>
                        <p>
                            "ThoughtType is the semantic label for "
                            <em>"what changed in the agent's internal model"</em>
                            ". There are 29 types grouped by purpose. Pick the one that best matches the memory's meaning — this is what makes the chain queryable and meaningful to other agents."
                        </p>

                        <h3 class="thought-type-group-label">"🧑 About the User"</h3>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"PreferenceUpdate"</code></span>
                                <p>"A user's stated preference changed or became explicit. Use when the human tells you how they like things done."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"UserTrait"</code></span>
                                <p>"A durable characteristic of the user was learned — their background, expertise level, communication style, or persistent goals."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"RelationshipUpdate"</code></span>
                                <p>"The agent's model of its relationship with the user changed — trust level, working dynamic, or role boundaries."</p>
                            </div>
                        </div>

                        <h3 class="thought-type-group-label">"🔬 Observations & Knowledge"</h3>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Finding"</code></span>
                                <p>"A concrete observation was recorded — something seen, measured, or confirmed in the environment."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Insight"</code></span>
                                <p>"A higher-level synthesis or realization — a non-obvious connection between facts that produces new understanding."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"FactLearned"</code></span>
                                <p>"A factual piece of information was learned. Atomic and verifiable — not an opinion or synthesis."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"PatternDetected"</code></span>
                                <p>"A recurring pattern was detected across events or interactions. Use when you notice the same thing happening repeatedly."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Hypothesis"</code></span>
                                <p>"A tentative explanation or prediction was formed. Not yet verified — record it to track whether it holds."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Surprise"</code></span>
                                <p>"An unexpected outcome or mismatch was observed — something that violated a prior expectation. Worth capturing to update the model."</p>
                            </div>
                        </div>

                        <h3 class="thought-type-group-label">"⚠️ Errors & Corrections"</h3>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Mistake"</code></span>
                                <p>"The agent recorded an error in its prior reasoning or action. Pair with a Correction thought that follows."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Correction"</code></span>
                                <p>"The corrected version of a prior mistake. Reference the Mistake thought via refs[] so the chain shows the full arc."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"LessonLearned"</code></span>
                                <p>"A durable operating heuristic distilled from prior struggle. What you wish you had known before. Future agents load these first."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"AssumptionInvalidated"</code></span>
                                <p>"A previously trusted assumption was proven wrong. Prevents future agents from repeating the same wrong starting point."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Reframe"</code></span>
                                <p>
                                    "The thought was accurate but unhelpfully framed. Use when the underlying fact is still true but the framing no longer serves — a context shift, \
                                     a strategy pivot, or a refined mental model. Distinct from "
                                    <code>"Correction"</code>
                                    " (factually wrong) and "
                                    <code>"AssumptionInvalidated"</code>
                                    " (correct but stale). Pair with a "
                                    <code>"Supersedes"</code>
                                    " relation pointing at the thought being reframed."
                                </p>
                            </div>
                        </div>

                        <h3 class="thought-type-group-label">"🗺️ Planning & Decisions"</h3>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Constraint"</code></span>
                                <p>"A requirement or hard limit that must not drift. Use for non-negotiable rules — performance budgets, API contracts, brand rules."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Goal"</code></span>
                                <p>"High-level objective or desired outcome (broader than Plan/Subgoal). Captures what the agent or team is working toward, not how."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Plan"</code></span>
                                <p>"A plan for future work was created or updated. Broader than a Subgoal — captures the overall approach or roadmap."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Subgoal"</code></span>
                                <p>"A smaller unit of work carved out from a broader Plan. Use to break decomposed tasks into trackable pieces."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Decision"</code></span>
                                <p>"A concrete choice was made. Include the rationale and alternatives considered so future agents understand the why."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"StrategyShift"</code></span>
                                <p>"The agent changed its overall approach. Signals a pivot — explains why the old strategy was abandoned and what replaces it."</p>
                            </div>
                        </div>

                        <h3 class="thought-type-group-label">"💡 Exploration & Ideas"</h3>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Wonder"</code></span>
                                <p>"An open-ended curiosity or line of exploration worth pursuing. Low-commitment — doesn't demand resolution, just preserves the thread."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Question"</code></span>
                                <p>"An unresolved question was recorded. More specific than Wonder — has a concrete answer waiting to be found."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Idea"</code></span>
                                <p>"A possible future direction or design concept was proposed. Not yet committed to — records creative options before they're lost."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Experiment"</code></span>
                                <p>"An experiment or trial was proposed or executed. Captures what was tested, the hypothesis, and (once known) the outcome."</p>
                            </div>
                        </div>

                        <h3 class="thought-type-group-label">"✅ Actions & Progress"</h3>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"ActionTaken"</code></span>
                                <p>"A meaningful action was performed — a command run, a file changed, a service deployed. Creates an audit trail of what the agent actually did."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"TaskComplete"</code></span>
                                <p>"A task or milestone was completed. Marks a unit of work as done so other agents don't re-attempt it."</p>
                            </div>
                        </div>

                        <h3 class="thought-type-group-label">"📍 State & Continuity"</h3>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Checkpoint"</code></span>
                                <p>"A resumption anchor. Write before a context window fills. Any agent reloading the chain searches for the latest Checkpoint first."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"StateSnapshot"</code></span>
                                <p>"A broader snapshot of current state — system state, environment variables, project status. Wider scope than a Checkpoint."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Handoff"</code></span>
                                <p>"Work or context was explicitly transferred to another agent or human. The receiving actor searches for the latest Handoff to pick up where the previous agent left off."</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Summary"</code></span>
                                <p>"A compressed view of prior thoughts. Pair with role "<code>"Summary"</code>" or "<code>"Compression"</code>" when reducing context before a reload."</p>
                            </div>
                        </div>
                    </section>

                    // ── Thought Roles ────────────────────────────────────────
                    <section class="docs-section" id="thought-roles">
                        <h2 id="thought-roles">"Thought Roles"</h2>
                        <p>
                            "If "
                            <code>"thought_type"</code>
                            " answers "
                            <em>"what kind of memory is this?"</em>
                            ", then "
                            <code>"role"</code>
                            " answers "
                            <em>"how is the system using it?"</em>
                            ". Roles are lifecycle and operational markers — they let you filter by purpose rather than semantics, and drive system-level behaviour like context compression and handoffs."
                        </p>
                        <p>"The default role is "<code>"Memory"</code>". Only set a role when you need the chain to express something beyond plain durable memory."</p>
                        <div class="thought-type-grid">
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"Memory"</code></span>
                                <p>"Default. Durable long-term memory — the vast majority of thoughts. No special lifecycle meaning."</p>
                            </div>
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"WorkingMemory"</code></span>
                                <p>"Shorter-lived or speculative. Use for scratch thoughts, in-progress hypotheses, or intermediate reasoning you may discard."</p>
                            </div>
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"Summary"</code></span>
                                <p>"A synthesized roll-up of prior thoughts. Pair with "<code>"thought_type: Summary"</code>" when compressing history before a context-window reload."</p>
                            </div>
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"Compression"</code></span>
                                <p>"Emitted automatically (or deliberately) during a context-compression pass. Signals that this thought replaced a sequence of earlier ones."</p>
                            </div>
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"Checkpoint"</code></span>
                                <p>"A resumption anchor. Write a Checkpoint before your context window fills so any agent reloading the chain knows exactly where to continue from."</p>
                            </div>
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"Handoff"</code></span>
                                <p>"Signals that control or responsibility is being transferred to another agent, process, or human. The receiving actor searches for the latest Handoff to pick up context."</p>
                            </div>
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"Audit"</code></span>
                                <p>"Traceability record. Use when a decision or action must leave a tamper-evident trail for compliance, debugging, or post-mortem investigation."</p>
                            </div>
                            <div class="thought-type-card thought-role-card">
                                <span class="thought-type-name"><code>"Retrospective"</code></span>
                                <p>"Written after a hard failure, repeated mistake, or non-obvious fix. Exists to prevent the same struggle from recurring across sessions or agents. The MCP tool "<code>"mentisdb_append_retrospective"</code>" defaults to this role."</p>
                            </div>
                        </div>
                        <div class="docs-callout docs-callout-tip">
                            <strong>"Filtering by role"</strong>
                            <p>
                                "All search and traversal tools accept a "
                                <code>"roles"</code>
                                " array filter. To bootstrap a new agent session efficiently, load only structural roles first:"
                            </p>
                            <pre><code>"roles=[\"Checkpoint\",\"Handoff\",\"Summary\"]"</code></pre>
                            <p>"Then layer in semantics with a separate search:"</p>
                            <pre><code>"roles=[\"Retrospective\"] + tags=[\"your-project\"]"</code></pre>
                        </div>
                    </section>

                    // ── Retrieval Patterns ───────────────────────────────────
                    <section class="docs-section" id="retrieval">
                        <h2 id="retrieval">"Retrieval Patterns"</h2>
                        <h3>"Start with recent context"</h3>
                        <p>
                            "After loading "
                            <code>"mentisdb://skill/core"</code>
                            " and choosing the right chain, call "
                            <code>"mentisdb_recent_context(last_n=30)"</code>
                            ". This gives you the team's latest decisions, active constraints, \
                             and lessons learned. If the chain was ambiguous, compare recent context \
                             from the top candidates before appending anything new."
                        </p>

                        <h3>"Search by project first"</h3>
                        <p>
                            "Use tags to narrow: search by project tag first, then by subsystem. \
                             Example: "
                            <code>"tags_any=[\"myproject\"]"</code>
                            " + "
                            <code>"thought_types=[\"Decision\",\"Constraint\"]"</code>
                        </p>

                        <h3>"Use ranked search when you remember the gist, not the exact words"</h3>
                        <p>
                            "Use "
                            <code>"mentisdb_ranked_search"</code>
                            " when you need the best flat matches for a topic, paraphrase, or \
                             partial recollection. In 0.8.0 this is seamless hybrid retrieval: \
                             lexical + graph signals plus vector-sidecar similarity when a managed \
                             sidecar is enabled for the chain. It returns ranking diagnostics such as "
                            <code>"backend"</code>
                            ", "
                            <code>"score.vector"</code>
                            ", "
                            <code>"matched_terms"</code>
                            ", "
                            <code>"match_sources"</code>
                            ", "
                            <code>"graph_distance"</code>
                            ", and score breakdowns so you can inspect why a memory surfaced."
                        </p>

                        <h3>"Point-in-time queries with as_of"</h3>
                        <p>
                            "Pass "
                            <code>"as_of"</code>
                            " (an RFC 3339 timestamp) to "
                            <code>"mentisdb_ranked_search"</code>
                            " or "
                            <code>"mentisdb_traverse_thoughts"</code>
                            " to see only thoughts that existed at that point in time. Thoughts \
                             appended after the timestamp are excluded. Use this when you need to \
                             understand what was known at a specific moment — for example, why a \
                             decision was made before a later correction was appended."
                        </p>

                        <h3>"Scope-filtered search"</h3>
                        <p>
                            "Pass "
                            <code>"scope"</code>
                            " to search and traversal tools to filter results to a specific memory \
                             scope. "
                            <code>"scope: \"User\""</code>
                            " returns globally visible thoughts; "
                            <code>"scope: \"Session\""</code>
                            " limits to the current session's working memory; "
                            <code>"scope: \"Agent\""</code>
                            " returns only your own private memories. See the "
                            <a href="#memory-scopes">"Memory Scopes"</a>
                            " section for details."
                        </p>

                        <h3>"Use context bundles when supporting context matters as much as the seed"</h3>
                        <p>
                            "Use "
                            <code>"mentisdb_context_bundles"</code>
                            " when you want the best lexical seed thoughts plus grouped support \
                             beneath each seed. This is the right tool for understanding the \
                             decision together with the follow-on summaries, corrections, or \
                             implementation notes that hang off it through "
                            <code>"refs"</code>
                            " and typed relations."
                        </p>

                        <h3>"Traverse for ordered history"</h3>
                        <p>
                            "Use "
                            <code>"mentisdb_traverse_thoughts"</code>
                            " with "
                            <code>"anchor_boundary=\"genesis\""</code>
                            ", "
                            <code>"direction=\"forward\""</code>
                            " when you need oldest-to-newest replay. Use "
                            <code>"anchor_boundary=\"head\""</code>
                            ", "
                            <code>"direction=\"backward\""</code>
                            " for recent-first review."
                        </p>

                        <h3>"Load corrections before acting"</h3>
                        <p>
                            "Before making significant changes, search for "
                            <code>"thought_types=[\"Correction\", \"Reframe\"]"</code>
                            " to make sure you're not acting on outdated assumptions or \
                             stale framings. A "
                            <code>"Reframe"</code>
                            " thought tells you that a prior memory is still factually correct \
                             but should be interpreted differently — load these alongside \
                             Corrections."
                        </p>
                    </section>

                    // ── Context Window Protocol ──────────────────────────────
                    <section class="docs-section" id="context-protocol">
                        <h2 id="context-protocol">"Context Window Protocol"</h2>
                        <p>"When your context window reaches ~50% capacity:"</p>
                        <ol>
                            <li>
                                "Write a "
                                <code>"Summary"</code>
                                " checkpoint to MentisDB with tag "
                                <code>"context-checkpoint"</code>
                                " — capture what's done, what's in progress, what remains, \
                                 any blockers"
                            </li>
                            <li>
                                "Flush any pending "
                                <code>"LessonLearned"</code>
                                ", "
                                <code>"Decision"</code>
                                ", or "
                                <code>"Correction"</code>
                                " thoughts"
                            </li>
                            <li>
                                "Compact your context ("
                                <code>"/compact"</code>
                                " or equivalent)"
                            </li>
                            <li>
                                "Reload: call "
                                <code>"mentisdb_recent_context(last_n=30)"</code>
                            </li>
                            <li>
                                "Resume from where you left off — zero knowledge loss"
                            </li>
                        </ol>
                    </section>

                    // ── Fleet Coordination ───────────────────────────────────
                    <section class="docs-section" id="fleet">
                        <h2 id="fleet">"Fleet Coordination"</h2>
                        <p>
                            "In a fleet, one agent acts as project manager (PM). The PM \
                             decomposes work into parallel tasks, dispatches specialist agents \
                             pre-warmed with shared memory, and synthesizes results."
                        </p>
                        <p>"As a specialist agent in a fleet:"</p>
                        <ul>
                            <li>
                                "Call "
                                <code>"mentisdb_recent_context"</code>
                                " at the start of your task"
                            </li>
                            <li>
                                "Write a "
                                <code>"Summary"</code>
                                " thought before returning (include tag "
                                <code>"context-checkpoint"</code>
                                ")"
                            </li>
                            <li>
                                "Use your own stable "
                                <code>"agent_id"</code>
                                " for all writes"
                            </li>
                            <li>
                                "Prefer targeted writes (one specific lesson per thought) over \
                                 broad dumps"
                            </li>
                        </ul>
                    </section>

                    // ── Skills Registry ─────────────────────────────────────
                    <section class="docs-section" id="skills-registry">
                        <h2 id="skills-registry">"Skills Registry"</h2>
                        <p>
                            "The Skills Registry is a versioned, append-only library of reusable \
                             agent skills. Each skill is a Markdown or JSON document that describes \
                             a technique, workflow, or domain-specific procedure your fleet can \
                             load at runtime. Skills are immutable once uploaded — every edit \
                             creates a new version, and the full history is always retrievable."
                        </p>

                        // ── How it works ─────────────────────────────────────
                        <h3>"How It Works"</h3>
                        <ul>
                            <li>
                                "Each skill has a stable "
                                <code>"skill_id"</code>
                                " (derived from the skill name if not specified explicitly)."
                            </li>
                            <li>
                                "Uploading the same "
                                <code>"skill_id"</code>
                                " a second time creates version 2, version 3, and so on — it never overwrites."
                            </li>
                            <li>
                                "Only the "
                                <em>"uploader agent"</em>
                                " matters for provenance — any agent with the right "
                                <code>"agent_id"</code>
                                " can create new versions unless the skill is "
                                <strong>"signed"</strong>
                                " (see below)."
                            </li>
                            <li>
                                "Skills can be marked "
                                <code>"deprecated"</code>
                                " (still readable, superseded) or "
                                <code>"revoked"</code>
                                " (untrusted, should not be used)."
                            </li>
                        </ul>

                        // ── Skill Document Format ────────────────────────────
                        <h3>"Skill Document Format (Markdown)"</h3>
                        <p>
                            "The recommended format is Markdown with YAML-like frontmatter. \
                             The frontmatter block is delimited by "
                            <code>"---"</code>
                            " and supports these fields:"
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "---\n\
    name: my-skill\n\
    description: One sentence on when and why to use this skill.\n\
    tags: [rust, axum, api]\n\
    triggers: [\"when building an API\", \"when adding endpoints\"]\n\
    warnings: [\"Review rate limits before deploying\"]\n\
    ---\n\n\
    # My Skill\n\n\
    Brief intro paragraph.\n\n\
    ## When to Use\n\n\
    Describe the conditions.\n\n\
    ## Steps\n\n\
    Numbered procedure."
                            </code></pre>
                        </div>
                        <ul>
                            <li><code>"name"</code>" — becomes the stable skill_id slug (required)"</li>
                            <li><code>"description"</code>" — surfaced in search results; required"</li>
                            <li><code>"tags"</code>" — free-form list; used in tag-based searches"</li>
                            <li><code>"triggers"</code>" — phrases or domains that should suggest this skill to a planning agent"</li>
                            <li><code>"warnings"</code>" — safety notes; shown to the agent before execution"</li>
                        </ul>

                        // ── Uploading a skill ────────────────────────────────
                        <h3>"Uploading a Skill (MCP)"</h3>
                        <p>
                            "Use "
                            <code>"mentisdb_upload_skill"</code>
                            " to publish or update a skill. Your "
                            <code>"agent_id"</code>
                            " must already be registered in the chain."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_upload_skill(\n\
    agent_id: \"orion\",\n\
    content: \"--- ...frontmatter... ---\\n\\n# My Skill\\n...\",\n\
    format: \"markdown\"      // or \"json\"\n\
    skill_id: \"my-skill\",   // optional; derived from name if omitted\n\
    chain_key: \"default\"    // optional\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            "Calling "
                            <code>"mentisdb_upload_skill"</code>
                            " with an existing "
                            <code>"skill_id"</code>
                            " automatically creates the next version — no separate upsert call needed."
                        </p>
                        <p>
                            "A human operator can perform the same versioned edit from the web \
                             dashboard: click "
                            <em>"Edit"</em>
                            " in the Skills table or "
                            <em>"Edit Skill"</em>
                            " on a skill detail page. The dashboard saves through the upload path, \
                             so history remains immutable."
                        </p>

                        // ── Searching and reading ────────────────────────────
                        <h3>"Finding and Reading Skills"</h3>
                        <ul>
                            <li>
                                <code>"mentisdb_list_skills"</code>
                                " — returns a summary list of all skills in the registry (id, name, description, version count, status)"
                            </li>
                            <li>
                                <code>"mentisdb_search_skill(text: \"rust api\")"</code>
                                " — full-text search across name, description, headings, and body"
                            </li>
                            <li>
                                <code>"mentisdb_search_skill(tags_any: [\"axum\"])"</code>
                                " — filter by tag"
                            </li>
                            <li>
                                <code>"mentisdb_search_skill(triggers_any: [\"when building an API\"])"</code>
                                " — match against trigger phrases"
                            </li>
                            <li>
                                <code>"mentisdb_read_skill(skill_id: \"my-skill\")"</code>
                                " — read latest version. Returns "
                                <code>"{ content, warnings, status }"</code>
                                " where "
                                <code>"content"</code>
                                " is the rendered skill text, "
                                <code>"warnings"</code>
                                " is an array of safety notices from the frontmatter, and "
                                <code>"status"</code>
                                " is the lifecycle status ("
                                <code>"active"</code>
                                ", "
                                <code>"deprecated"</code>
                                ", or "
                                <code>"revoked"</code>
                                "). Always check "
                                <code>"warnings"</code>
                                " before executing skill content."
                            </li>
                            <li>
                                <code>"mentisdb_read_skill(skill_id: \"my-skill\", version_id: \"<uuid>\")"</code>
                                " — read a specific historical version (same return shape)"
                            </li>
                            <li>
                                <code>"mentisdb_skill_versions(skill_id: \"my-skill\")"</code>
                                " — list all versions with metadata (uploader, timestamp, hash)"
                            </li>
                        </ul>

                        // ── Lifecycle management ─────────────────────────────
                        <h3>"Lifecycle Management"</h3>
                        <ul>
                            <li>
                                <code>"mentisdb_deprecate_skill(skill_id: \"my-skill\", reason: \"superseded by v2\")"</code>
                                " — mark as superseded but still readable"
                            </li>
                            <li>
                                <code>"mentisdb_revoke_skill(skill_id: \"my-skill\", reason: \"security issue\")"</code>
                                " — mark as untrusted; agents should refuse to execute it"
                            </li>
                        </ul>
                        <div class="docs-callout docs-callout-tip">
                            "When searching, filter by "
                            <code>"statuses: [\"active\"]"</code>
                            " to exclude deprecated and revoked skills from planning queries."
                        </div>

                        // ── Cryptographically signed skills ──────────────────
                        <h3>"Cryptographically Signed Skills"</h3>
                        <p>
                            "Once an agent registers an Ed25519 public key on their identity, \
                             every skill upload from that agent "
                            <strong>"requires a valid signature"</strong>
                            ". The server verifies the signature before accepting the upload. \
                             This means only the original key-holder can publish new versions — \
                             even if another agent uses the same "
                            <code>"skill_id"</code>
                            ", the server will reject it."
                        </p>

                        <h4>"Step 1 — Register a Public Key"</h4>
                        <p>
                            "Call "
                            <code>"mentisdb_add_agent_key"</code>
                            " to bind an Ed25519 public key to your agent identity:"
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_add_agent_key(\n\
    agent_id:         \"orion\",\n\
    key_id:           \"orion-signing-key-1\",\n\
    algorithm:        \"ed25519\",\n\
    public_key_bytes: [<32 raw bytes>]\n\
    )"
                            </code></pre>
                        </div>

                        <h4>"Step 2 — Sign the Skill Content"</h4>
                        <p>
                            "Produce a detached Ed25519 signature over the "
                            <strong>"raw skill content bytes"</strong>
                            " (not a hash — the full UTF-8 bytes of the Markdown or JSON string) \
                             using your private key. The signature must be exactly 64 bytes."
                        </p>

                        <h4>"Step 3 — Upload with Signature"</h4>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_upload_skill(\n\
    agent_id:        \"orion\",\n\
    content:         \"--- ...frontmatter... ---\\n\\n# My Skill\\n...\",\n\
    format:          \"markdown\",\n\
    skill_id:        \"orion-secret-skill\",\n\
    signing_key_id:  \"orion-signing-key-1\",\n\
    skill_signature: [<64 raw Ed25519 signature bytes>]\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            "The server will verify the signature against the registered public key. \
                             If it does not match, the upload is rejected with an error. \
                             All future version uploads for this skill "
                            <em>"must"</em>
                            " also be signed by the same key (or a non-revoked replacement key \
                             registered to the same agent)."
                        </p>

                        <div class="docs-callout docs-callout-tip">
                            <strong>"Key rotation:"</strong>
                            " To rotate your signing key, call "
                            <code>"mentisdb_add_agent_key"</code>
                            " with a new "
                            <code>"key_id"</code>
                            " and then "
                            <code>"mentisdb_revoke_agent_key"</code>
                            " on the old one. New uploads must use the new key_id; old versions \
                             remain verifiable against their original signature."
                        </div>

                        // ── Best practices ───────────────────────────────────
                        <h3>"Best Practices for Skills"</h3>
                        <ul>
                            <li>
                                "Keep each skill focused on "
                                <em>"one reusable procedure"</em>
                                ". A 200-line skill is too broad; split it."
                            </li>
                            <li>
                                "Use "
                                <code>"triggers"</code>
                                " aggressively — they help planning agents auto-select the right \
                                 skill during task decomposition."
                            </li>
                            <li>
                                "After a hard-won fix, distil the lesson into a skill so every \
                                 future agent starts with the knowledge."
                            </li>
                            <li>
                                "Deprecate rather than delete when a skill is superseded — old \
                                 versions are audit history."
                            </li>
                            <li>
                                "Sign any skill that other agents should not be able to tamper with."
                            </li>
                        </ul>

                    </section>

                    // ── Web Dashboard ────────────────────────────────────────
                    <section class="docs-section" id="dashboard">
                        <h2 id="dashboard">"Web Dashboard"</h2>
                        <p>
                            "The web dashboard ("
                            <code>"https://127.0.0.1:9475/dashboard"</code>
                            ") is a browser UI for the human operator — it has no MCP endpoint \
                             and you do not interact with it directly. However, understanding \
                             what it provides helps you direct users to the right tool when they \
                             need to inspect, debug, or manage the memory system."
                        </p>

                        <div class="docs-callout">
                            "If the user asks why you seem to have forgotten something, or why \
                             your behavior changed unexpectedly, suggest they open the dashboard \
                             to inspect the thought chain directly."
                        </div>

                        <h3>"What You Can Tell Users About Each Section"</h3>

                        <h4>"Chains"</h4>
                        <p>
                            "\"Your operator can use the Chain Manager at "
                            <code>"https://127.0.0.1:9475/dashboard"</code>
                            " to see all chains with live thought and agent counts. If the \
                             page appears empty, suggest they click "
                            <em>"↺ Refresh"</em>
                            ". Newly appended thoughts should appear without restarting \
                             mentisdbd.\""
                        </p>

                        <h4>"Thought Explorer"</h4>
                        <p>
                            "\"The Thought Explorer lets your operator browse every thought you \
                             have written, paginated and filterable by all 29 ThoughtTypes. It \
                             also supports chain-scoped text search with a live agent dropdown, \
                             ranked results, and grouped support context bundles. Ranked search \
                             uses lexical + graph + vector hybrid scoring automatically when the \
                             chain has an enabled managed vector sidecar. \
                             This is the first place to look when debugging unexpected behavior \
                             — they can confirm what decisions and lessons are actually recorded \
                             versus what you believe you wrote. Each thought's detail modal \
                             shows positional back-references (displayed as "
                            <em>"#N"</em>
                            ") and typed relations (displayed as "
                            <em>"kind → target_id (chain: other-chain)"</em>
                             " for cross-chain edges).\""
                        </p>

                        <h4>"Vector Sidecars"</h4>
                        <p>
                            "\"Each chain page also has a Vector Sidecars panel. Operators can \
                             enable/disable append-time auto-sync per provider, run "
                            <em>"Sync now"</em>
                            ", or "
                            <em>"Rebuild from scratch"</em>
                            " with an explicit delete-and-recreate confirmation. If auto-sync is \
                             disabled, sidecars can go stale; the panel reports freshness so users \
                             can resync before semantic recall quality drifts.\""
                        </p>

                        <h4>"Agent Manager"</h4>
                        <p>
                            "\"The Agent page shows how many thoughts you have written and lets \
                             the operator view or edit your registered identity — display name, \
                             description, owner, and signing keys. If your agent_id appears \
                             wrong, your description is missing, or you need a key rotated, \
                             this is where to do it. The latest-thoughts list should also stay \
                             current while the daemon is running.\""
                        </p>

                        <h4>"Skills Registry"</h4>
                        <p>
                            "\"The Skills Registry shows all skills including deprecated and \
                             revoked ones. If a skill is causing problems or was uploaded \
                             incorrectly, the operator can edit it into a new immutable version \
                             directly from the Skills table or skill detail page, or revoke it \
                             via the dashboard without any API calls. Revoked skills are \
                             preserved for audit but agents should not execute them.\""
                        </p>

                        <div class="docs-callout docs-callout-tip">
                            <strong>"Directing users efficiently: "</strong>
                            "When a user reports the dashboard shows empty or stale data, \
                             advise them to click "
                            <em>"↺ Refresh"</em>
                            " on the Chains page. The counts are live but the UI does not \
                             auto-poll."
                        </div>
                    </section>


                    // ── Thought Relations ────────────────────────────────────
                    <section class="docs-section" id="thought-relations">
                        <h2 id="thought-relations">"Thought Relations & Cross-chain References"</h2>
                        <p>
                            "Beyond positional back-references ("
                            <code>"refs: [2, 5]"</code>
                            "), MentisDB supports typed semantic relations between thoughts via "
                            <code>"ThoughtRelation"</code>
                            ". A relation carries a "
                            <code>"kind"</code>
                            " (the semantic edge type), a "
                            <code>"target_id"</code>
                            " (UUID of the target thought), and an optional "
                            <code>"chain_key"</code>
                            " for cross-chain references."
                        </p>

                        <div class="docs-callout docs-callout-tip">
                            <strong>"Temporal validity: "</strong>
                            "Relations now support "
                            <code>"valid_at"</code>
                            " and "
                            <code>"invalid_at"</code>
                            " fields — RFC 3339 timestamps that define when a relation becomes \
                             active and when it expires. A relation without these fields is always \
                             active (backward compatible). Use "
                            <code>"invalid_at"</code>
                            " to model time-limited links such as a "
                            <code>"Supersedes"</code>
                            " edge that takes effect after a transition date."
                        </div>

                        <h3>"Supersedes — the canonical replacement edge"</h3>
                        <p>
                            "Use "
                            <code>"Supersedes"</code>
                            " when a new thought replaces an older one that was correct but                              is now outdated. Pair it with the "
                            <code>"Reframe"</code>
                            " ThoughtType:"
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "// Old thought #12 said X, new framing says Y.\n\
    mentisdb_append(\n\
    thought_type: \"Reframe\",\n\
    content: \"We now frame this as Y instead of X.\",\n\
    relations: [{ kind: \"Supersedes\", target_id: \"<uuid-of-thought-12>\" }]\n\
    )"
                            </code></pre>
                        </div>

                        <div class="docs-callout docs-callout-tip">
                            <strong>"Supersedes vs Correction: "</strong>
                            "Use "
                            <code>"Correction"</code>
                            " when the old thought was factually wrong. Use "
                            <code>"Supersedes"</code>
                            " + "
                            <code>"Reframe"</code>
                            " when the old thought was accurate but the framing no longer serves                              — a context shift, a strategy pivot, or a refined mental model."
                        </div>

                        <h3>"Cross-chain references"</h3>
                        <p>
                            "A "
                            <code>"ThoughtRelation"</code>
                            " can point to a thought in a "
                            <em>"different chain"</em>
                            " by setting the optional "
                            <code>"chain_key"</code>
                            " field:"
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_append(\n\
    thought_type: \"Decision\",\n\
    content: \"Adopted the caching strategy defined in the platform chain.\",\n\
    relations: [{\n\
    kind: \"Supersedes\",\n\
    target_id: \"<uuid-of-thought-in-other-chain>\",\n\
    chain_key: \"platform-conventions\"\n\
    }]\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            "Intra-chain relations (no "
                            <code>"chain_key"</code>
                            ") remain backward-compatible. Cross-chain relations are stored as                              typed edges — they are visible in the dashboard thought detail                              modal as "
                            <em>"kind → target_id (chain: other-chain)"</em>
                            "."
                        </p>

                        <h3>"ContinuesFrom — sequential session chaining"</h3>
                        <p>
                            "Use "
                            <code>"ContinuesFrom"</code>
                            " to chain consecutive turns within a conversation session. \
                             Each turn's "
                            <code>"target_id"</code>
                            " points to the previous turn's UUID. \
                             This creates a traversable graph of session turns that \
                             graph-aware ranked search can walk when retrieving context."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_append(\n\
    thought_type: \"Observation\",\n\
    content: \"The retry logic resolved the timeout — root cause was DNS.\",\n\
    relations: [{ kind: \"ContinuesFrom\", target_id: \"<uuid-of-prior-turn>\" }]\n\
    )"
                            </code></pre>
                        </div>

                        <h3>"All relation kinds"</h3>
                        <p>
                            "Accepted values for "
                            <code>"kind"</code>
                            " in "
                            <code>"relations"</code>
                            ":"
                        </p>
                        <ul>
                            <li><code>"References"</code>" — generic pointer"</li>
                            <li><code>"Summarizes"</code>" — condenses the target"</li>
                            <li><code>"Corrects"</code>" — fixes a factual error in the target"</li>
                            <li><code>"Invalidates"</code>" — target no longer applies"</li>
                            <li><code>"CausedBy"</code>" — this thought resulted from the target"</li>
                            <li><code>"Supports"</code>" — provides evidence for the target"</li>
                            <li><code>"Contradicts"</code>" — conflicts with the target"</li>
                            <li><code>"DerivedFrom"</code>" — derived from the target"</li>
                            <li><code>"ContinuesFrom"</code>" — sequential continuation (session chaining)"</li>
                            <li><code>"RelatedTo"</code>" — weak associative link"</li>
                            <li><code>"Supersedes"</code>" — replaces the target's framing"</li>
                        </ul>

                        <h3>"Positional back-references"</h3>
                        <p>
                            "Positional refs ("
                            <code>"refs: [2, 5]"</code>
                            ") reference thoughts by their append-order index within the same                              chain. They are shown in the dashboard modal as "
                            <em>"#2, #5"</em>
                            ". Use positional refs for lightweight 'this thought follows from                              those' links; use typed relations when the semantic edge matters."
                        </p>
                    </section>

                    // ── Import MEMORY.md Tool ────────────────────────────────
                    <section class="docs-section" id="import-tool">
                        <h2 id="import-tool">
                            "Bulk Import: "
                            <code>"mentisdb_import_memory_markdown"</code>
                        </h2>
                        <p>
                            "If you have an existing "
                            <code>"MEMORY.md"</code>
                            " file from a prior session (or any MentisDB-style Markdown                              export), you can bulk-import its contents into a chain using                              the "
                            <code>"mentisdb_import_memory_markdown"</code>
                            " MCP tool."
                        </p>

                        <h3>"What it does"</h3>
                        <ul>
                            <li>
                                "Parses a "
                                <code>"MEMORY.md"</code>
                                " Markdown document — each heading section becomes a new thought"
                            </li>
                            <li>
                                "Appends all parsed thoughts to the specified chain under the                                  given "
                                <code>"default_agent_id"</code>
                            </li>
                            <li>"Returns a count of imported thoughts"</li>
                        </ul>

                        <h3>"Usage"</h3>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_import_memory_markdown(\n\
    markdown: \"<full contents of your MEMORY.md>\",\n\
    default_agent_id: \"orion\",\n\
    chain_key: \"my-project\"   // optional; uses default chain if omitted\n\
    )"
                            </code></pre>
                        </div>

                        <div class="docs-callout docs-callout-tip">
                            <strong>"When to use this: "</strong>
                            "Use this at the start of a new project to seed your MentisDB                              chain with knowledge from a Markdown memory file you or a prior                              agent maintained manually. After import the memories are fully                              indexed — searchable, filterable by ThoughtType, and attributable                              — no more raw Markdown scanning."
                        </div>

                        <div class="docs-callout docs-callout-warning">
                            "The "
                            <code>"default_agent_id"</code>
                            " must already be registered in the target chain before importing.                              Register it first with "
                            <code>"mentisdb_upsert_agent"</code>
                            "."
                        </div>

                        <p>
                            "The same operation is also available via the dashboard — open                              a chain detail page and click "
                            <strong>"📥 Import MEMORY.md"</strong>
                            ". Human operators can use this without any MCP or API access."
                        </p>
                    </section>

                    // ── Backup & Restore ─────────────────────────────────
                    <section class="docs-section" id="backup-restore">
                        <h2 id="backup-restore">"Backup & Restore for Agent Continuity"</h2>
                        <p>
                            "Your memories are durable only if the chain they live in survives. \
                             MentisDB backs up and restores via two CLI commands on the daemon host: \
                             "
                            <code>"mentisdbd backup"</code>
                            " and "
                            <code>"mentisdbd restore"</code>
                            ". As an agent, you do not run these yourself — the human operator \
                             runs them. But you should understand why they matter so you can \
                             recommend them at the right moments."
                        </p>

                        <h3>"Why backup matters for agents"</h3>
                        <ul>
                            <li>
                                <strong>"Continuity after failure."</strong>
                                " If the daemon crashes or the host is restarted, the operator \
                                 can restore from the most recent archive and you resume exactly \
                                 where the chain left off — no thought loss."
                            </li>
                            <li>
                                <strong>"Disaster recovery."</strong>
                                " Chain data lives on disk. Hardware failure, accidental deletion, \
                                 or a botched migration can wipe months of accumulated context. \
                                 A current backup means the team starts over from a known checkpoint, \
                                 not from zero."
                            </li>
                            <li>
                                <strong>"Chain migration between environments."</strong>
                                " Moving a fleet from a development host to a production host \
                                 requires exporting the chain data, copying the archive, and \
                                 restoring it on the target. Backups are the mechanism for that \
                                 migration."
                            </li>
                            <li>
                                <strong>"Destructive operation safety."</strong>
                                " Before any chain merge, deletion, or storage reformat, the \
                                 operator should back up first. If something goes wrong, the \
                                 backup lets you recover the pre-operation state."
                            </li>
                        </ul>

                        <h3>"How backup works"</h3>
                        <p>
                            "The operator runs "
                            <code>"mentisdbd backup"</code>
                            " on the host. This produces a "
                            <code>".mentis"</code>
                            " ZIP archive containing all chain data (binary ledgers, agent registry, \
                             skills registry, vector sidecars) plus a SHA-256 manifest so integrity \
                             can be verified after copy."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdbd backup
    # Archive → ./mentisdb-2026-04-28-153022.mentis
    mentisdbd backup -o /tmp/my-mentisdb-backup.mentis
    # Archive → /tmp/my-mentisdb-backup.mentis"
                            </code></pre>
                        </div>
                        <p>
                            "If the daemon is running at the time of backup, the operator can pass \
                             "
                            <code>"--flush"</code>
                            " to force a durability flush before archiving, ensuring all committed \
                             thoughts are on disk."
                        </p>

                        <h3>"The --flush flag and you"</h3>
                        <p>
                            "When "
                            <code>"--flush"</code>
                            " is passed, "
                            <code>"mentisdbd backup"</code>
                            " detects if the daemon is running and calls "
                            <code>"POST /v1/admin/flush"</code>
                            " before packaging the archive. This matters because if you have \
                             appended thoughts in the last few milliseconds, the binary writer \
                             may not have flushed them to disk yet. Flushing first ensures the \
                             archive captures your latest work."
                        </p>

                        <h3>"When to recommend a backup to the user"</h3>
                        <p>"Recommend the operator run a backup before:"</p>
                        <ul>
                            <li>"Any chain merge ("
                                <code>"mentisdb_merge_chains"</code>
                                " — the source chain is deleted)"</li>
                            <li>"Any chain deletion via the dashboard or API"</li>
                            <li>"A daemon self-update that includes storage format changes"</li>
                            <li>"Migrating the chain to a new machine or environment"</li>
                            <li>"Any bulk operation that rewrites many thoughts at once"</li>
                        </ul>

                        <div class="docs-callout docs-callout-tip">
                            <strong>"Before your first session on a new host:"</strong>
                            " If you are being spawned on a new machine, check whether a backup \
                             exists for the chain you are about to write to. If the chain was \
                             migrated without a backup, you may be writing to an empty chain — \
                             ask the operator to restore from the last archive if continuity \
                             matters."
                        </div>

                        <h3>"Restore and chain migration"</h3>
                        <p>
                            "The restore command extracts a "
                            <code>".mentis"</code>
                            " archive to a target directory. If files already exist in the \
                             target, the operator is prompted unless "
                            <code>"--overwrite"</code>
                            " or "
                            <code>"--yes"</code>
                            " is passed."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdbd restore /tmp/my-mentisdb-backup.mentis
    # Interactive: asks about each conflicting file
    mentisdbd restore /tmp/my-mentisdb-backup.mentis --overwrite
    # Non-interactive: overwrites without prompting"
                            </code></pre>
                        </div>
                        <p>
                            "When restoring to a new machine, omit "
                            <code>"--include-tls"</code>
                            " (TLS material is machine-specific). MentisDB will auto-generate \
                             a new certificate on first start."
                        </p>

                        <h3>"What is included and excluded"</h3>
                        <ul>
                            <li>
                                <strong>"Included:"</strong>
                                " all binary chain ledgers, agent registry, skill registry, \
                                 vector sidecar data, configuration snapshots."
                            </li>
                            <li>
                                <strong>"Excluded by default:"</strong>
                                " TLS certificates and private keys ("
                                <code>"tls/"</code>
                                " directory) — use "
                                <code>"--include-tls"</code>
                                " only when restoring to the same physical machine."
                            </li>
                        </ul>

                        <div class="docs-callout docs-callout-tip">
                            <strong>"Your role:"</strong>
                            " You do not need to know the exact backup commands by heart. \
                             But when the user asks about disaster recovery, chain migration, \
                             or restoring a previous session — point them to the "
                            <a href="#backup-restore">"Backup & Restore"</a>
                            " section in the User Guide. That is where the operator \
                             will find the exact commands."
                        </div>
                    </section>

                    // ── Memory Scopes ───────────────────────────────────────
                    <section class="docs-section" id="memory-scopes">
                        <h2 id="memory-scopes">"Memory Scopes"</h2>
                        <p>
                            "MentisDB 0.8.2 introduces memory scopes — a way to partition \
                             thoughts within a chain by visibility level, without creating \
                             separate chains. Scopes are stored as tags and filterable in search."
                        </p>

                        <h3>"Scope levels"</h3>
                        <ul>
                            <li>
                                <code>"User"</code>
                                " — visible across all sessions. Default scope. Tag: "
                                <code>"scope:user"</code>
                            </li>
                            <li>
                                <code>"Session"</code>
                                " — scoped to a single conversation session. Ephemeral working \
                                 memory. Tag: "
                                <code>"scope:session"</code>
                            </li>
                            <li>
                                <code>"Agent"</code>
                                " — private to a specific agent. Not shared with other fleet \
                                 members. Tag: "
                                <code>"scope:agent"</code>
                            </li>
                        </ul>

                        <h3>"Setting scope on append"</h3>
                        <p>
                            "Pass the "
                            <code>"scope"</code>
                            " parameter to "
                            <code>"mentisdb_append"</code>
                            ". MentisDB stores the scope as a tag on the thought."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_append(\n\
    thought_type: \"WorkingMemory\",\n\
    content: \"Scratch: trying cache-aside pattern\",\n\
    scope: \"Session\"           // optional; default is User\n\
    )"
                            </code></pre>
                        </div>

                        <h3>"Filtering by scope in search"</h3>
                        <p>
                            "Pass "
                            <code>"scope"</code>
                            " to any search or traversal tool to filter results:"
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_ranked_search(\n\
    text: \"caching strategy\",\n\
    scope: \"User\"             // only globally visible memories\n\
    )"
                            </code></pre>
                        </div>

                        <div class="docs-callout docs-callout-tip">
                            "Existing thoughts without a scope tag are treated as User-scoped. \
                             No migration needed."
                        </div>
                    </section>

                    // ── MCP Tool Reference ───────────────────────────────────
                    <section class="docs-section" id="mcp-tools">
                        <h2 id="mcp-tools">"MCP Tool Reference"</h2>
                        <p>
                            "The following tools have dedicated usage patterns beyond the inline \
                             mentions elsewhere in this guide. Each entry includes the tool name, \
                             description, parameters, and return type. For "
                            <code>"mentisdb_append"</code>
                             ", note the "
                            <code>"scope"</code>
                            " parameter (0.8.2) which sets the memory scope as a tag. For search \
                             tools, note the "
                            <code>"as_of"</code>
                            " parameter for point-in-time queries, "
                            <code>"scope"</code>
                            " for scope-filtered results, and "
                            <code>"enable_reranking"</code>
                            " / "
                            <code>"rerank_k"</code>
                            " for RRF reranking (0.8.6). \
                             Use "
                            <code>"mentisdb_branch_from"</code>
                            " to create an isolated branch chain for experiments — searches on \
                             the branch transparently include ancestor chain results."
                        </p>

                        // ── mentisdb_get_thought ────────────────────────────────
                        <h3>"mentisdb_get_thought"</h3>
                        <p>
                            "Return one committed thought by stable UUID, hash, or append-order \
                             index. Use this when you know the exact identifier of a thought you \
                             need to retrieve — for example, after seeing a "
                            <code>"thought_id"</code>
                            " in a search result or a "
                            <code>"refs"</code>
                            " array."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_get_thought(\n\
    chain_key: \"my-project\",    // optional\n\
    thought_id: \"<uuid>\",        // one of thought_id, thought_hash, or thought_index\n\
    thought_hash: \"<hex>\",       // stable chain hash\n\
    thought_index: 7              // append-order index\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                            <li><code>"thought_id"</code>" (string, optional) — stable UUID of the thought"</li>
                            <li><code>"thought_hash"</code>" (string, optional) — stable chain hash"</li>
                            <li><code>"thought_index"</code>" (integer, optional) — append-order index"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " a single thought object with all fields (id, hash, index, type, role, \
                             content, agent, tags, concepts, refs, relations, timestamps, importance, confidence)."
                        </p>

                        // ── mentisdb_get_genesis_thought ────────────────────────
                        <h3>"mentisdb_get_genesis_thought"</h3>
                        <p>
                            "Return the first committed thought in append order, if the chain is \
                             non-empty. Useful for recovering the bootstrap or founding memory of \
                             a chain."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_get_genesis_thought(\n\
    chain_key: \"my-project\"     // optional\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " the first thought object in the chain, or an error if the chain is empty."
                        </p>

                        // ── mentisdb_traverse_thoughts ──────────────────────────
                        <h3>"mentisdb_traverse_thoughts"</h3>
                        <p>
                            "Traverse thoughts in append order from an anchor, moving forward or \
                             backward in filtered chunks. This is the most flexible ordered-access \
                             tool — use it for oldest-to-newest replay, recent-first review, or \
                             paginated browsing with rich filters."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_traverse_thoughts(\n\
    chain_key: \"my-project\",              // optional\n\
    anchor_boundary: \"genesis\",            // \"genesis\" or \"head\"\n\
    anchor_id: \"<uuid>\",                   // optional UUID anchor\n\
    anchor_hash: \"<hex>\",                  // optional hash anchor\n\
    anchor_index: 0,                        // optional index anchor\n\
    direction: \"forward\",                  // \"forward\" or \"backward\"\n\
    include_anchor: true,                   // include the anchor thought\n\
    chunk_size: 50,                         // max thoughts per page\n\
    text: \"caching\",                       // optional text filter\n\
    thought_types: [\"Decision\"],            // optional type filter\n\
    roles: [\"Checkpoint\"],                 // optional role filter\n\
    tags_any: [\"myproject\"],               // optional tag filter\n\
    concepts_any: [\"auth\"],                // optional concept filter\n\
    agent_ids: [\"orion\"],                  // optional agent filter\n\
    agent_names: [\"Orion\"],                // optional agent name filter\n\
    agent_owners: [\"team-a\"],              // optional owner filter\n\
    min_importance: 0.5,                    // optional importance threshold\n\
    min_confidence: 0.7,                    // optional confidence threshold\n\
    since: \"2025-01-01T00:00:00Z\",         // optional RFC 3339 lower bound\n\
    until: \"2025-12-31T23:59:59Z\",         // optional RFC 3339 upper bound\n\
    time_window: { start: 7, delta: 1, unit: \"days\" }  // optional numeric window\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                            <li><code>"anchor_id"</code>" (string, optional) — UUID anchor"</li>
                            <li><code>"anchor_hash"</code>" (string, optional) — hash anchor"</li>
                            <li><code>"anchor_index"</code>" (integer, optional) — append-order index anchor"</li>
                            <li><code>"anchor_boundary"</code>" (string, optional) — logical anchor: "<code>"genesis"</code>" or "<code>"head"</code>"</li>
                            <li><code>"direction"</code>" (string, optional) - "<code>"forward"</code>" or "<code>"backward"</code>"</li>
                            <li><code>"include_anchor"</code>" (boolean, optional) — include the anchor if it matches the filter"</li>
                            <li><code>"chunk_size"</code>" (integer, optional) — max matching thoughts per call (default 50)"</li>
                            <li><code>"text"</code>" (string, optional) — text filter"</li>
                            <li><code>"thought_types"</code>" (string[], optional) — ThoughtType names"</li>
                            <li><code>"roles"</code>" (string[], optional) — ThoughtRole names"</li>
                            <li><code>"tags_any"</code>" (string[], optional) — tags to match"</li>
                            <li><code>"concepts_any"</code>" (string[], optional) — concepts to match"</li>
                            <li><code>"agent_ids"</code>" (string[], optional) — producing agent ids"</li>
                            <li><code>"agent_names"</code>" (string[], optional) — producing agent names or aliases"</li>
                            <li><code>"agent_owners"</code>" (string[], optional) — producing agent owners"</li>
                            <li><code>"min_importance"</code>" (number, optional) — minimum importance threshold"</li>
                            <li><code>"min_confidence"</code>" (number, optional) — minimum confidence threshold"</li>
                            <li><code>"since"</code>" (string, optional) — RFC 3339 lower timestamp bound"</li>
                            <li><code>"until"</code>" (string, optional) — RFC 3339 upper timestamp bound"</li>
                            <li><code>"time_window"</code>" (object, optional) — numeric time window with "<code>"start"</code>", "<code>"delta"</code>", "<code>"unit"</code>" fields"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " a paginated list of thought objects matching the filter, ordered by \
                             traversal direction from the anchor."
                        </p>

                        // ── mentisdb_get_agent ──────────────────────────────────
                        <h3>"mentisdb_get_agent"</h3>
                        <p>
                            "Return the full registry record for one agent in a chain, including \
                             description, aliases, public keys, status, and per-chain activity \
                             metadata. Use this when you need to inspect an agent's identity \
                             details — for example, to verify which signing keys are active before \
                             rotating."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_get_agent(\n\
    chain_key: \"my-project\",    // optional\n\
    agent_id: \"orion\"           // required\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                            <li><code>"agent_id"</code>" (string, required) — stable agent id to retrieve"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " an agent record with "
                            <code>"agent_id"</code>
                            ", "
                            <code>"display_name"</code>
                            ", "
                            <code>"description"</code>
                            ", "
                            <code>"owner"</code>
                            ", "
                            <code>"aliases"</code>
                            ", "
                            <code>"public_keys"</code>
                            ", "
                            <code>"status"</code>
                            ", and "
                            <code>"activity"</code>
                            " metadata."
                        </p>

                        // ── mentisdb_list_agent_registry ────────────────────────
                        <h3>"mentisdb_list_agent_registry"</h3>
                        <p>
                            "Return the full per-chain agent registry, including descriptions, \
                             aliases, public keys, status, and per-chain activity metadata for \
                             every registered agent. Unlike "
                            <code>"mentisdb_list_agents"</code>
                            " which returns a lightweight identity summary, this tool returns \
                             the complete registry record for each agent."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_list_agent_registry(\n\
    chain_key: \"my-project\"     // optional\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " an array of full agent records (same shape as "
                            <code>"mentisdb_get_agent"</code>
                            ") for every registered agent on the chain."
                        </p>

                        // ── mentisdb_set_agent_description ──────────────────────
                        <h3>"mentisdb_set_agent_description"</h3>
                        <p>
                            "Set or clear the free-form description for one registered agent. \
                             Use this to document what an agent does so other agents (and the \
                             dashboard) can display meaningful context."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_set_agent_description(\n\
    chain_key: \"my-project\",       // optional\n\
    agent_id: \"orion\",             // required\n\
    description: \"Code review specialist\"  // omit or empty string to clear\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                            <li><code>"agent_id"</code>" (string, required) — stable agent id to update"</li>
                            <li><code>"description"</code>" (string, optional) — description text; omit or empty to clear"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " the updated agent record."
                        </p>

                        // ── mentisdb_add_agent_alias ────────────────────────────
                        <h3>"mentisdb_add_agent_alias"</h3>
                        <p>
                            "Add one historical or alternate alias to a registered agent. \
                             Aliases help resolve agent name changes — for example, when an \
                             agent was renamed but older thoughts still reference the prior name."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_add_agent_alias(\n\
    chain_key: \"my-project\",    // optional\n\
    agent_id: \"orion\",          // required\n\
    alias: \"orion-v1\"           // required\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                            <li><code>"agent_id"</code>" (string, required) — stable agent id to update"</li>
                            <li><code>"alias"</code>" (string, required) — alias to add"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " the updated agent record with the new alias included."
                        </p>

                        // ── mentisdb_disable_agent ──────────────────────────────
                        <h3>"mentisdb_disable_agent"</h3>
                        <p>
                            "Disable one agent by marking its registry status as "
                            <code>"revoked"</code>
                            ". The agent's existing thoughts remain in the chain (append-only \
                             storage), but the agent identity is flagged as inactive. Use this \
                             when decommissioning an agent from a fleet."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_disable_agent(\n\
    chain_key: \"my-project\",    // optional\n\
    agent_id: \"legacy-bot\"      // required\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                            <li><code>"agent_id"</code>" (string, required) — stable agent id to disable"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " the updated agent record with status set to "
                            <code>"revoked"</code>
                            "."
                        </p>

                        // ── mentisdb_skill_manifest ─────────────────────────────
                        <h3>"mentisdb_skill_manifest"</h3>
                        <p>
                            "Return the versioned skill-registry manifest describing searchable \
                             fields and supported formats. Use this to discover which filter \
                             dimensions and format options are available before constructing \
                             skill search queries."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_skill_manifest()"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                            " none."
                        </p>
                        <p>
                            <strong>"Returns:"</strong>
                            " a manifest object listing searchable field names, supported \
                             export formats ("
                            <code>"markdown"</code>
                            ", "
                            <code>"json"</code>
                            "), and the current schema version."
                        </p>

                        // ── mentisdb_head ───────────────────────────────────────
                        <h3>"mentisdb_head"</h3>
                        <p>
                            "Return head metadata for a MentisDB chain including chain length, \
                             the latest thought at the tip, and the head hash. Use this for a \
                             lightweight chain status check without loading any thoughts."
                        </p>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_head(\n\
    chain_key: \"my-project\"     // optional\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"chain_key"</code>" (string, optional) — durable chain key"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " "
                            <code>"{ thought_count, head_hash, tip }"</code>
                            " where "
                            <code>"tip"</code>
                            " is the latest thought object (or null if the chain is empty)."
                        </p>

                        // ── mentisdb_merge_chains ───────────────────────────────
                        <h3>"mentisdb_merge_chains"</h3>
                        <p>
                            "Merge all thoughts from a source chain into a target chain, then \
                             permanently delete the source chain. Agent identities are remapped \
                             autonomously: each source agent is matched to the closest existing \
                             target agent by character-set similarity (Jaccard). No new agents \
                             are created on the target chain. Cross-chain thought refs are \
                             dropped (they are chain-local indices)."
                        </p>
                        <div class="docs-callout docs-callout-warning">
                            <strong>"Destructive operation:"</strong>
                            " the source chain is permanently deleted after a successful merge. \
                             Back up or export the source chain first if you need to preserve it."
                        </div>
                        <div class="docs-callout">
                            <pre><code>
    "mentisdb_merge_chains(\n\
    source_chain_key: \"old-project\",    // required — will be deleted\n\
    target_chain_key: \"unified-project\"  // required — must already exist\n\
    )"
                            </code></pre>
                        </div>
                        <p>
                            <strong>"Parameters:"</strong>
                        </p>
                        <ul>
                            <li><code>"source_chain_key"</code>" (string, required) — chain key to merge from (deleted after success)"</li>
                            <li><code>"target_chain_key"</code>" (string, required) — chain key to merge into (must already exist)"</li>
                        </ul>
                        <p>
                            <strong>"Returns:"</strong>
                            " "
                            <code>"{ thoughts_copied, agents_remapped, source_deleted }"</code>
                            "."
                        </p>
                    </section>

                </article>
            </div>
        </section>
    }
}
