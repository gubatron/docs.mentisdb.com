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
                        <a class="docs-nav-link" href="#thought-roles">"Thought Roles"</a>
                        <a class="docs-nav-link" href="#retrieval">"Retrieval Patterns"</a>
                        <a class="docs-nav-link" href="#context-protocol">"Context Window Protocol"</a>
                        <a class="docs-nav-link" href="#fleet">"Fleet Coordination"</a>
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
                    </section>

                    // ── Choosing Thought Types ───────────────────────────────
                    <section class="docs-section" id="thought-types">
                        <h2 id="thought-types">"Choosing Thought Types"</h2>
                        <p>"Use the semantic type that matches the memory's job:"</p>
                        <div class="thought-type-grid">
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Decision"</code></span>
                                <p>"Chosen design or implementation direction"</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Constraint"</code></span>
                                <p>"Hard boundary that must not drift"</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"LessonLearned"</code></span>
                                <p>"Retrospective rule distilled from a failure"</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Insight"</code></span>
                                <p>"Non-obvious technical lesson"</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Correction"</code></span>
                                <p>"An earlier assumption was wrong; this replaces it"</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Summary"</code></span>
                                <p>
                                    "Compressed state; pair with role "
                                    <code>"Checkpoint"</code>
                                    " for restart points"
                                </p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"PreferenceUpdate"</code></span>
                                <p>"Stable team or user preference"</p>
                            </div>
                            <div class="thought-type-card">
                                <span class="thought-type-name"><code>"Question"</code></span>
                                <p>"Unresolved issue worth preserving"</p>
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
                            "At the start of every session, call "
                            <code>"mentisdb_recent_context(last_n=30)"</code>
                            ". This gives you the team's latest decisions, active constraints, \
                             and lessons learned."
                        </p>

                        <h3>"Search by project first"</h3>
                        <p>
                            "Use tags to narrow: search by project tag first, then by subsystem. \
                             Example: "
                            <code>"tags_any=[\"myproject\"]"</code>
                            " + "
                            <code>"thought_types=[\"Decision\",\"Constraint\"]"</code>
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
                            <code>"thought_types=[\"Correction\"]"</code>
                            " to make sure you're not acting on outdated assumptions."
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

                </article>
            </div>
        </section>
    }
}
