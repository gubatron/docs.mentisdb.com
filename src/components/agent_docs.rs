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
