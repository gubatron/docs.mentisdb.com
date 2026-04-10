use leptos::prelude::*;

/// Blog post: MentisDB 0.8.3 — Migration Hotfix
#[component]
pub fn Blog083Hotfix() -> impl IntoView {
    view! {
        <section class="docs-page">
            <div class="container docs-layout">

                // ── Left sidebar ──────────────────────────────────────────────
                <aside class="docs-sidebar">
                    <nav class="docs-nav">
                        <a class="docs-nav-link" href="#apology">"We're Sorry"</a>
                        <a class="docs-nav-link" href="#what-happened">"What Happened"</a>
                        <a class="docs-nav-link" href="#the-fix">"The Fix"</a>
                        <a class="docs-nav-link" href="#reassurance">"No Data Lost"</a>
                        <a class="docs-nav-link" href="#migration-tests">"Migration Tests"</a>
                        <a class="docs-nav-link" href="#whitepaper">"WHITEPAPER.md Rewrite"</a>
                        <a class="docs-nav-link" href="#upgrade">"How to Upgrade"</a>
                    </nav>
                </aside>

                // ── Main content ──────────────────────────────────────────────
                <article class="docs-content">

                    <h1>"MentisDB 0.8.3 — Migration Hotfix"</h1>
                    <p class="docs-hero-sub">"April 10, 2026"</p>

                    // ── Apology ─────────────────────────────────────────────
                    <h2 id="apology">"We're Sorry"</h2>
                    <p>
                        "If you upgraded from 0.8.1 to 0.8.2 and saw your daemon crash with \
                         an "
                        <code>"UnexpectedVariant"</code>
                        " or "
                        <code>"UUID parsing failed: invalid length"</code>
                        " error — we sincerely apologize. That should never have happened. \
                         Your memory chains are safe, and the fix is a single "
                        <code>"cargo update"</code>
                        " away."
                    </p>

                    // ── What Happened ───────────────────────────────────────
                    <h2 id="what-happened">"What Happened"</h2>
                    <p>
                        "MentisDB stores thoughts in a binary format with an internal schema \
                         version (V0, V1, V2, V3). The "
                        <code>"load_binary_thoughts()"</code>
                        " function in 0.8.2 read only the "
                        <strong>"first"</strong>
                        " thought's "
                        <code>"schema_version"</code>
                        " varint and then applied that single format to the entire chain."
                    </p>
                    <p>
                        "This worked fine for homogeneous chains — all V2 thoughts, for \
                         instance. But if a chain contained thoughts written by different \
                         daemon versions — say, V1 thoughts from an older release followed \
                         by V2 thoughts appended by a newer daemon — the deserializer used \
                         the V1 layout for every thought. V2 thoughts have a different struct \
                         shape (3-field "
                        <code>"ThoughtRelation"</code>
                        " vs. V1's 2-field), so the bincode decoder would hit a variant \
                         discriminant it didn't recognize or try to read a UUID from the \
                         wrong byte offset. The result: a hard crash on chain open."
                    </p>
                    <p>
                        "In practical terms, any user who ran pre-0.8.2 MentisDB (V1 schema) \
                         and then upgraded to 0.8.2 (V2 schema) would have a chain where \
                         the first thought was V1 and later thoughts were V2. Opening that \
                         chain in 0.8.2 was impossible."
                    </p>

                    // ── The Fix ─────────────────────────────────────────────
                    <h2 id="the-fix">"The Fix"</h2>
                    <p>
                        "The single-schema-path approach has been replaced with "
                        <code>"load_binary_thoughts_per_thought()"</code>
                        ". Each thought's payload is now individually versioned: the loader \
                         reads each record, peeks its "
                        <code>"schema_version"</code>
                        " varint, and dispatches to the correct legacy deserializer — V0/V1 \
                         with 2-field "
                        <code>"ThoughtRelation"</code>
                        ", V2 with 3-field, or V3 direct. After deserialization, the hash \
                         chain is rebuilt for all migrated thoughts."
                    </p>
                    <p>
                        "The dead "
                        <code>"migrate_v2_thoughts()"</code>
                        " function has been removed; its logic is now inline in the \
                         per-thought path."
                    </p>

                    // ── Reassurance ──────────────────────────────────────────
                    <h2 id="reassurance">"No Data Lost — Just Reopen"</h2>
                    <p>
                        "If you hit this crash: "
                        <strong>"just open MentisDB again with 0.8.3."</strong>
                        " The per-thought loader will correctly read each thought regardless \
                         of its schema version, and the entire chain will be persisted in V3 \
                         format. Future opens will use the native V3 fast path — no \
                         re-migration needed."
                    </p>
                    <p>
                        "No data was lost. No manual intervention is required. The fix is \
                         automatic and transparent."
                    </p>

                    // ── Migration Tests ──────────────────────────────────────
                    <h2 id="migration-tests">"11 New Migration Tests"</h2>
                    <p>
                        "To prevent any regression in schema-migration code, 0.8.3 adds \
                         comprehensive coverage:"
                    </p>
                    <ul>
                        <li>
                            <code>"migrate_v2_chain_with_all_relation_kinds"</code>
                            " — all 11 "
                            <code>"ThoughtRelationKind"</code>
                            " variants, with and without "
                            <code>"chain_key"</code>
                            ", verifying "
                            <code>"valid_at"</code>
                            "/"
                            <code>"invalid_at"</code>
                            " are "
                            <code>"None"</code>
                            " after migration"
                        </li>
                        <li>
                            <code>"migrate_v1_chain_with_all_thought_types"</code>
                            " — all 30 "
                            <code>"ThoughtType"</code>
                            " variants survive V1→V3 migration"
                        </li>
                        <li>
                            <code>"migrate_v2_chain_with_all_thought_roles"</code>
                            " — all 8 "
                            <code>"ThoughtRole"</code>
                            " variants survive V2→V3 migration"
                        </li>
                        <li>
                            <code>"migrate_mixed_chain_with_v1_v2_v3_thoughts"</code>
                            " — per-thought detection across V1, V2, and V3 thoughts in the \
                             same chain (the exact scenario that caused the crash)"
                        </li>
                        <li>
                            <code>"migrate_v2_chain_with_signed_thought"</code>
                            " — "
                            <code>"signing_key_id"</code>
                            " and "
                            <code>"thought_signature"</code>
                            " preserved through migration"
                        </li>
                        <li>
                            <code>"migrate_v1_chain_with_relations"</code>
                            " — 2-field "
                            <code>"LegacyTestRelation"</code>
                            " migrates to 5-field "
                            <code>"ThoughtRelation"</code>
                            " with "
                            <code>"chain_key=None"</code>
                            ", "
                            <code>"valid_at=None"</code>
                            ", "
                            <code>"invalid_at=None"</code>
                        </li>
                    </ul>
                    <p>
                        "Together these tests cover all 30 ThoughtType variants, all 11 \
                         ThoughtRelationKind variants, all ThoughtRole variants, signed \
                         thoughts, and mixed V1/V2/V3 chains — ensuring the migration path \
                         is solid for every production scenario."
                    </p>

                    // ── Whitepaper ───────────────────────────────────────────
                    <h2 id="whitepaper">"WHITEPAPER.md Rewrite"</h2>
                    <p>
                        "The whitepaper has been completely rewritten with 12 sections \
                         covering the full feature set: architecture, schema evolution, \
                         semantic memory model, storage, query & retrieval, deduplication, \
                         CLI, MCP, benchmarks, competitive landscape, and future roadmap. \
                         Find it in the repository root as "
                        <code>"WHITEPAPER.md"</code>
                        "."
                    </p>

                    // ── How to Upgrade ───────────────────────────────────────
                    <h2 id="upgrade">"How to Upgrade"</h2>
                    <p>
                        "Pick either:"
                    </p>
                    <div class="code-block">
                        <pre><code>"cargo install mentisdb --version 0.8.3"</code></pre>
                    </div>
                    <p>"or, if you already have MentisDB in your Cargo workspace:"</p>
                    <div class="code-block">
                        <pre><code>"cargo update"</code></pre>
                    </div>
                    <p>
                        "Then restart "
                        <code>"mentisdbd"</code>
                        ". If you had a crashed chain, it will open correctly and persist \
                         in V3 format for all future opens."
                    </p>

                </article>
            </div>
        </section>
    }
}
