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
                    <p>"Pick either:"</p>
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

/// Blog post: MentisDB 0.8.5 — The Update That Almost Wasn't
#[component]
pub fn Blog085UpdateOdyssey() -> impl IntoView {
    view! {
        <section class="docs-page">
            <div class="container docs-layout">

                // ── Left sidebar ──────────────────────────────────────────────
                <aside class="docs-sidebar">
                    <nav class="docs-nav">
                        <a class="docs-nav-link" href="#the-bug">"The Bug"</a>
                        <a class="docs-nav-link" href="#catch-22">"The Catch-22"</a>
                        <a class="docs-nav-link" href="#fix-083">"0.8.3: Fixing the Migration"</a>
                        <a class="docs-nav-link" href="#fix-084">"0.8.4: Updates Before Migrations"</a>
                        <a class="docs-nav-link" href="#fix-085">"0.8.5: The Runtime-in-Runtime Panic"</a>
                        <a class="docs-nav-link" href="#lesson">"The Lesson"</a>
                        <a class="docs-nav-link" href="#upgrade">"How to Upgrade"</a>
                    </nav>
                </aside>

                // ── Main content ──────────────────────────────────────────────
                <article class="docs-content">

                    <h1>"MentisDB 0.8.5 — The Update That Almost Wasn't"</h1>
                    <p class="docs-hero-sub">"April 11, 2026"</p>

                    <p>
                        "What happens when your auto-updater can't run because the bug it's \
                         supposed to fix is blocking it? And then the fix for that creates \
                         another bug? This is the story of 72 hours, three hotfix releases, \
                         and the kind of recursive failure mode that makes distributed \
                         systems engineers laugh nervously."
                    </p>

                    // ── The Bug ─────────────────────────────────────────────
                    <h2 id="the-bug">"The Original Bug: V2→V3 Migration Crash"</h2>
                    <p>
                        "MentisDB 0.8.2 introduced schema V3 with 5-field "
                        <code>"ThoughtRelation"</code>
                        " (adding "
                        <code>"valid_at"</code>
                        " and "
                        <code>"invalid_at"</code>
                        "). The migration code to read older V1/V2 chains had a \
                         critical flaw: it peeked only the "
                        <strong>"first"</strong>
                        " thought's "
                        <code>"schema_version"</code>
                        " and applied that one format to the entire chain."
                    </p>
                    <p>
                        "If a chain contained V1 thoughts followed by V2 thoughts (common: \
                         any user who ran an older daemon, then upgraded), the deserializer \
                         would try to read V2 structs using V1 layout rules. Bincode would hit \
                         a variant discriminant it didn't recognize or read a UUID from the \
                         wrong byte offset. Hard crash. No fallback. No recovery."
                    </p>
                    <p>
                        "The error messages were cryptic: "
                        <code>"UnexpectedVariant { type_name: Option<T>, allowed: Range {min: \
                         0, max: 1}, found: 64 }"</code>
                        " or "
                        <code>"UUID parsing failed: invalid length: expected 16 bytes, found \
                         0"</code>
                        ". Users couldn't even "
                        <em>"open"</em>
                        " their chains."
                    </p>

                    // ── Catch-22 ─────────────────────────────────────────────
                    <h2 id="catch-22">"The Catch-22: Update Check Ran After the Crash"</h2>
                    <p>
                        "Here's where it got worse. MentisDB's startup sequence looked like \
                         this:"
                    </p>
                    <ol>
                        <li>"Load config"</li>
                        <li>"Migrate all chains (crashes here on broken chains)"</li>
                        <li>"Start servers"</li>
                        <li>"Check for updates (async, in background)"</li>
                    </ol>
                    <p>
                        "Step 2 crashes before step 4 ever runs. The update check that would \
                         install the fix "
                        <em>"never gets a chance to execute"</em>
                        ". The user is stuck on the broken version forever, because the \
                         broken version can't start, and the fix can't install itself."
                    </p>
                    <p>
                        "This is a classic upgrade Catch-22: the mechanism that delivers the \
                         fix depends on the system being healthy enough to run, but the \
                         system is broken precisely because it needs the fix."
                    </p>

                    // ── 0.8.3 ────────────────────────────────────────────────
                    <h2 id="fix-083">"0.8.3: Fixing the Migration"</h2>
                    <p>
                        "The first fix replaced the single-schema-path deserializer with "
                        <code>"load_binary_thoughts_per_thought()"</code>
                        ". Each thought's payload is now individually versioned: the loader \
                         reads each record, peeks its "
                        <code>"schema_version"</code>
                        " varint, and dispatches to the correct legacy deserializer. Hash \
                         chains are rebuilt after migration. Mixed V1/V2/V3 chains now \
                         load correctly."
                    </p>
                    <p>
                        "We also wrote 11 comprehensive migration tests covering all 30 "
                        <code>"ThoughtType"</code>
                        " variants, all 11 "
                        <code>"ThoughtRelationKind"</code>
                        " variants, all 8 "
                        <code>"ThoughtRole"</code>
                        " variants, signed thoughts, and mixed V1+V2+V3 chains."
                    </p>
                    <p>
                        "But users still couldn't get the fix if they were stuck on 0.8.2. \
                         The update check still ran "
                        <em>"after"</em>
                         " migrations."
                    </p>

                    // ── 0.8.4 ────────────────────────────────────────────────
                    <h2 id="fix-084">"0.8.4: Updates Before Migrations"</h2>
                    <p>
                        "0.8.4 moved the update check to "
                        <strong>"before"</strong>
                        " chain migrations. Now the startup sequence is:"
                    </p>
                    <ol>
                        <li>"Load config"</li>
                        <li>"Check for updates (synchronous, before touching chains)"</li>
                        <li>"If update found and user accepts: install and exit"</li>
                        <li>"Migrate all chains"</li>
                        <li>"Start servers"</li>
                    </ol>
                    <p>
                        "We also added two new CLI subcommands:"
                    </p>
                    <ul>
                        <li>
                            <code>"mentisdbd update"</code>
                            " — check for a newer release, prompt y/N, install if accepted"
                        </li>
                        <li>
                            <code>"mentisdbd force-update"</code>
                            " — install the latest release unconditionally, even if you \
                             think you're up to date. Useful for repairing a corrupted binary."
                        </li>
                    </ul>
                    <p>
                        "The auto-restart mechanism (which replaced the process in-place via "
                        <code>"exec()"</code>
                        ") was removed. It added complexity and the in-process restart \
                         could fail on some platforms. Now, if an update is installed, \
                         mentisdbd simply exits with a message to restart. Simpler and safer."
                    </p>

                    // ── 0.8.5 ────────────────────────────────────────────────
                    <h2 id="fix-085">"0.8.5: The Runtime-in-Runtime Panic"</h2>
                    <p>
                        "We tested "
                        <code>"mentisdbd update"</code>
                        " and "
                        <code>"mentisdbd force-update"</code>
                        " on a local build and they worked. Pushed 0.8.4. Users reported:"
                    </p>
                    <div class="code-block">
                        <pre><code>"thread 'main' panicked at src/bin/mentisdbd.rs:1404:31:\nCannot start a runtime from within a runtime. This happens because a function (like block_on) attempted to block the current thread while the thread is being used to drive asynchronous tasks."</code></pre>
                    </div>
                    <p>
                        "The new "
                        <code>"run_update_standalone()"</code>
                        " function needed to make an HTTP request to GitHub's releases API. \
                         It was called from "
                        <code>"main()"</code>
                         ", which is annotated with "
                        <code>"#[tokio::main]"</code>
                        ". Inside that async context, the function created a "
                        <em>"new"</em>
                        " tokio runtime with "
                        <code>"tokio::runtime::Builder::new_current_thread().enable_all().build()"</code>
                        " and then called "
                        <code>"rt.block_on(fetch_latest_release())"</code>
                        ". Tokio panicked because you cannot nest runtimes."
                    </p>
                    <p>
                        "The fix was simple: make "
                        <code>"run_update_standalone"</code>
                        " an "
                        <code>"async fn"</code>
                        " and "
                        <code>".await"</code>
                        " the HTTP call directly, since we're already inside a tokio runtime. \
                         Delete the "
                        <code>"rt.block_on()"</code>
                        " wrapper. Three lines removed, one keyword added."
                    </p>
                    <p>
                        "We added a test: "
                        <code>"parse_daemon_args_accepts_update_subcommands"</code>
                        " verifies that "
                        <code>"update"</code>
                        " parses to "
                        <code>"DaemonArgMode::Update"</code>
                        " and "
                        <code>"force-update"</code>
                        " parses to "
                        <code>"DaemonArgMode::ForceUpdate"</code>
                        ". Next time we add an async subcommand, we'll catch it in CI."
                    </p>

                    // ── The Lesson ───────────────────────────────────────────
                    <h2 id="lesson">"The Lesson"</h2>
                    <p>
                        "Three releases in 72 hours. Each one fixing a bug introduced by the \
                         previous fix. Here's what we learned:"
                    </p>
                    <ul>
                        <li>
                            <strong>"Never assume homogeneous schemas."</strong>
                            " Production chains contain thoughts written by different daemon \
                             versions. Per-thought version detection is the only safe approach."
                        </li>
                        <li>
                            <strong>"Update checks must run before anything that can crash."</strong>
                            " If the update mechanism depends on the system being healthy, \
                             the system can never recover from a crash. Move the check to \
                             the very first step."
                        </li>
                        <li>
                            <strong>"Never nest tokio runtimes."</strong>
                            " If you're inside "
                            <code>"#[tokio::main]"</code>
                            ", you're already on a runtime. Make the function async and \
                             .await directly. Don't create a second runtime."
                        </li>
                        <li>
                            <strong>"Test the full dispatch path, not just the happy path."</strong>
                            " Our unit tests for version comparison and argument parsing \
                             passed, but the "
                            <em>"actual execution path"</em>
                            " (async runtime context) was never exercised. Now we have \
                             parse tests that cover every subcommand."
                        </li>
                        <li>
                            <strong>"Release fast, but test on the actual binary."</strong>
                            " "
                            <code>"cargo test"</code>
                            " runs in a test harness. "
                            <code>"mentisdbd update"</code>
                            " runs in the real binary with the real tokio runtime. The \
                             runtime-in-runtime panic only manifests in production."
                        </li>
                    </ul>

                    // ── How to Upgrade ───────────────────────────────────────
                    <h2 id="upgrade">"How to Upgrade"</h2>
                    <p>
                        "If you're on any version before 0.8.5, the safest path is:"
                    </p>
                    <div class="code-block">
                        <pre><code>"cargo install mentisdb --force"</code></pre>
                    </div>
                    <p>
                        "The "
                        <code>"--force"</code>
                        " flag is required — "
                        <code>"cargo install"</code>
                        " will not overwrite an existing binary without it."
                    </p>
                    <p>"Or use the new subcommand (requires 0.8.5+):"</p>
                    <div class="code-block">
                        <pre><code>"mentisdbd force-update"</code></pre>
                    </div>
                    <p>
                        "Then restart "
                        <code>"mentisdbd"</code>
                        ". If you had chains from 0.8.1 or earlier, they will migrate \
                         automatically. No data loss. No manual intervention."
                    </p>

                </article>
            </div>
        </section>
    }
}
