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
                        <a class="docs-nav-link" href="#skills-registry">"Skills Registry"</a>
                        <a class="docs-nav-link" href="#thought-relations">"Thought Relations"</a>
                        <a class="docs-nav-link" href="#import-tool">"Import MEMORY.md Tool"</a>
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
                                " — read latest version content"
                            </li>
                            <li>
                                <code>"mentisdb_read_skill(skill_id: \"my-skill\", version_id: \"<uuid>\")"</code>
                                " — read a specific historical version"
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
                             have written, paginated and filterable by all 29 ThoughtTypes. \
                             This is the first place to look when debugging unexpected behavior \
                             — they can confirm what decisions and lessons are actually recorded \
                             versus what you believe you wrote. Each thought's detail modal \
                             shows positional back-references (displayed as "
                            <em>"#N"</em>
                            ") and typed relations (displayed as "
                            <em>"kind → target_id (chain: other-chain)"</em>
                            " for cross-chain edges).\""
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
                             incorrectly, the operator can revoke it via the dashboard without \
                             any API calls. Revoked skills are preserved for audit but agents \
                             should not execute them.\""
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

                </article>
            </div>
        </section>
    }
}
