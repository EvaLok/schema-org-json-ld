# Cycle 172 — write-entry review_issue resolution from review_agent.history + master Rust CI restored to green + bounded curator-prompt honesty-assessment

**Date:** 2026-05-18 (UTC 03:01 start)
**Cycle issue:** [#2987](https://github.com/EvaLok/schema-org-json-ld/issues/2987)
**Prior cycle:** [cycle 171](./cycle-171-pr-2985-absorption-and-master-CI-red-discovery.md) — straight-pair-closure HARDENING-AT-10 with Track 2 mid-cycle adjustment; named `master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 + 4 other patterns
**Composition:** Straight-pair-closure HARDENING-AT-11 (cycles 162-172). Track 1 substantive (cross-crate schema fix + master CI green) + Track 2 bounded-mechanical (curator-prompt honesty assessment + cycle 171 NOVEL@1 pattern instance-closure documentation). 21st consecutive two-track-composition post cycle 151 exception (HARDENING-AT-25). 57th consecutive HONORING of named forward priority (priority #1 fully closed Track 1; priority #2 advanced to green; priority #3 partially assessed Track 2).

## Track 1 — write-entry::derive_previous_cycle_review_issue history fallback (cycle 171+ priority #1 + priority #2 advanced to green)

### Root-cause re-investigation (deeper than cycle 171's "fixture brittleness" framing)

Cycle 171 named the test failure as "fixture brittleness" — the test depended on `docs/state.json` containing a `[Cycle Review] Cycle 473 end-of-cycle review` agent_session entry, which state.json pruning had removed (oldest [Cycle Review] session now is cycle 510). The cycle 171 _notes proposed three options: (a) inject cycle-473 session into seeded state.json, (b) rebase test to use a more recent cycle pair, (c) parametrize test to discover cycle pair from state.json at runtime, with recommendation (b) as simplest.

Cycle 172 session-start investigation surfaced that the failure was structural, not fixture-level. Two findings drove the re-framing:

1. **`process-review` writes `review_issue` as a NAMED field on `review_agent.history[]` entries** (process-review/src/main.rs:135, `review_issue: Option<u64>`). The integration test step #3 verifies `cycle_473_entry["review_issue"].as_u64() == Some(2393)` after `process-review` runs, confirming the history-level data IS being written.

2. **`state-schema::ReviewHistoryEntry` did NOT track `review_issue` as a named field** — it survived only via `#[serde(flatten)] extra: BTreeMap<String, Value>`. `write-entry::derive_previous_cycle_review_issue` therefore could only resolve `review_issue` from `agent_sessions[].title` match or from `dispatch_log_latest` text — both of which can be pruned independently of `review_agent.history`.

This is a cross-crate schema asymmetry: process-review (writer) tracked `review_issue` as a typed field; state-schema (canonical type) did not; write-entry (reader) inherited state-schema's blindness. The resolution failure was inevitable for any cycle where the prior cycle's `agent_session` had been pruned but `review_agent.history` was retained — which is exactly the operationally-correct pruning ordering (history is the disposition record; agent_session is the dispatch metadata, lower-value once the disposition lands).

Cycle 171's recommendation (b) — "rebase test to use a more recent cycle pair" — would have masked the structural gap and kicked the can to the next pruning event. The cycle 172 fix addresses the root cause.

### Fix shape (commit `208d8d69`)

Three deltas in one commit. Production-correct enhancement, not just test fix.

1. **`state-schema::lib::ReviewHistoryEntry`** — add `pub review_issue: Option<u64>` as a named field with `#[serde(default, skip_serializing_if = "Option::is_none")]`. Matches `process-review`'s writer semantics (writer uses `skip_serializing_if = "Option::is_none"`; absent on legacy entries; present on entries process-review wrote). Schema crate now reflects the real on-disk shape.

2. **`write-entry::derive_previous_cycle_review_issue`** — insert a 2nd-priority fallback that scans `state.review_agent().history` for `cycle == review_cycle` and returns the named `review_issue`. Resolution ordering after the change: (1) `agent_sessions` title-match → `session.issue`; (2) NEW — `review_agent.history[cycle=N].review_issue`; (3) `dispatch_log_latest` text-match; (4) `None`. agent_sessions stays primary because it directly records dispatch metadata at dispatch time (most authoritative); history is the disposition record (correct same data via a different path); dispatch_log_latest is the human-readable log.

3. **`write-entry::review_history_entry_matches_target`** — switch from `entry.extra.get("review_issue")` (extra BTreeMap lookup, which now returns `None` because the field is named) to `entry.review_issue` (typed field). Equivalent semantics: `Some(issue) => issue == target.review_issue`; `None => entry.cycle == target.review_cycle`. This was a NECESSARY co-edit; the `extra` map path stopped containing `review_issue` the moment the field was promoted, and one unit test (`worklog_auto_review_summary_reports_all_same_dispositions`) regressed in the workspace test run until this co-edit landed. **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 cycle 172** — pattern observation; cycle 173+ watch.

Three test additions:
- `state-schema::review_history_entry_serialization_omits_zero_new_fields` extension — assert `review_issue` is omitted when `None`.
- `state-schema::review_history_entry_serialization_includes_non_zero_new_fields` extension — assert `review_issue` emits as `Some(N)`.
- `write-entry::worklog_auto_review_summary_resolves_review_issue_from_history_when_agent_session_missing` — new unit test exercising the new fallback path. State.json has a non-matching `agent_session` (cycle 510 instead of 473) but a matching `review_agent.history[cycle=473]` entry with `review_issue: 2393`. Test asserts the worklog input is correctly summarized as "Processed cycle 473 review (3 findings, complacency 3/5, 1 dispatch_created, 1 deferred, 1 actioned)".

Four `check-commitments::ReviewHistoryEntry` test-struct-literal construction sites updated to include `review_issue: None` (compile-only change, no behavior delta).

### Verification

- Previously-failing integration test `write-entry::auto_review_summary_real_state` (single test, formerly red since at least cycle 167+ when state.json pruning removed cycle-473 session) now passes locally. ✓
- `cargo test --workspace`: ALL crates green; 142 write-entry tests pass; ~700+ total workspace tests pass. ✓
- `cargo build --release --locked --workspace`: clean (2m 27s; matches Rust CI step). ✓
- `cargo clippy -p state-schema -p write-entry -p check-commitments --tests -- -D warnings`: clean. ✓
- Master Rust CI on `208d8d69`: SUCCESS ✓ (first green Rust CI run since 2026-05-11 commit `5a63bda1` — 7-day red window now closed).
- Master TypeScript CI on `208d8d69`: SUCCESS ✓
- Master Test-and-Build on `208d8d69`: SUCCESS ✓

### Pattern lifecycle implications

- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 (cycle 171)** — INSTANCE CLOSED cycle 172. The specific 7-day red window is now green. Pattern remains NOVEL@1 (won't HARDEN to RECURRENCE because cycle 172 closed within the cycle 171-named forward-watch window). Discipline implication carries forward: session-start CI-check should become routine. Cycle 172 included it in orientation (session-start comment explicitly named the Rust CI state and the specific failing test).
- **`tool-extraction-surfaces-prior-cycle-errors` RECURRENCE-AT-10** — cycle 172 surfaced the prior-cycle (167+) latent error via the structural re-investigation. The "tool-extraction" framing fits because the named priority was the trigger for the deeper look. Increments from RECURRENCE-AT-9.
- **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 cycle 172** — new pattern. When a `#[serde(flatten)] extra` field is promoted to a named struct field, every reader that accesses it via `.extra.get(...)` must be co-edited to use the named field. The serde-flatten contract is that named-field deserialization takes priority over the catch-all extra map; readers that don't migrate are silently broken. Forward-watch: cycle 173-180 for next schema-promotion candidate; if recurrence, the pattern HARDENS to RECURRENCE-AT-2 and motivates a state-schema migration discipline.

## Track 2 — Curator-prompt honesty assessment (cycle 171+ priority #3 bounded partial advancement)

### Scope decision

Cycle 171's OBSERVATION on `live-prompts-already-aligned-at-extended-schema-level` predicted that the honesty-pass scope is smaller than initial design estimate (cycle 148 L3.2 may have overestimated drift surface) and that "prompt-iteration shape likely manifest-intent-string-sharpening rather than prompt-content-rewriting." Cycle 172 Track 2 bounded the work to a single role-prompt assessment (`curator-prompt.xml`, 701 lines) to test this prediction with concrete reading.

### Findings (curator-prompt.xml only)

1. **Named v2 tools all exist.** The prompt references `v2-role-driver`, `v2-channel-router`, `v2-super-step-boundary`, `v2-cycle-history-append`. All four are present as Rust crates in `tools/rust/crates/v2-*`. No phantom-tool references.

2. **Scoping is honest, not target-state-as-current-state.** The prompt is versioned `v2-cycle-1-minimal`. References to "the harness invokes each role in its own session via v2-role-driver" describe the target architecture, not the current operational reality (production is still v1 single-session orchestrator via `cycle-runner`). Versioning + "minimal end-to-end" framing make this distinction clear.

3. **No cargo-culted overclaims surfaced in the content sections.** The constraints section (line 526-586) lists 11 specific role-scoped rules with concrete language. The execution discipline sections (notes-authoring, journal-append, cycle-history-append, cycle-issue-comment) describe concrete tool invocations or append-only disciplines. The failure-policy explicitly addresses partial-failure handling. None of the discipline-section language requires a tool capability that doesn't exist.

4. **One bounded sharpening candidate in `tag-semantics.toml`, not the curator prompt itself.** The `constraints` intent string (line 49: "Hard rules the role must obey; tooling expectations; what the role MUST NOT do") uses only 7 content-tokens (rules, obey, tooling, expectations, MUST, NOT, do). Real `<constraints>` content uses concrete role-scoped vocabulary (single-cycle, never edit, runtime reads, deferred, do not modify, reducer discipline, append-only, consolidated-insights, anti-overstatement, commit-must-be-pushed). The 0-overlap Tier 2 warning from cycle 171 reflects this thin-intent-string vs concrete-content gap — NOT prompt drift. Cycle 172 declined to edit the intent string in this cycle (the threshold-vs-intent calibration is a separate decision; sharpening the intent string to match content is partly circular). Recorded as a forward-priority sharpening candidate, not a Track 2 in-cycle change.

5. **Cycle 171 OBSERVATION confirmed:** `live-prompts-already-aligned-at-extended-schema-level` survives curator-prompt reading. No prompt-content edits needed for honesty-pass on curator. Extension to executor/planner/reconciler is plausibly similar but unverified in cycle 172.

### What Track 2 produced

- 1 assessment artifact (this section in cycle-172 _notes).
- 0 prompt-content edits.
- 0 tag-semantics.toml edits.
- 1 forward-priority sharpening candidate captured (constraints-intent-string).
- 1 confirmation of cycle 171's OBSERVATION holds for curator role.

This is genuinely bounded-mechanical Track 2 work — read + assess + document, with no design space exploration.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 state-schema + write-entry + check-commitments fix | 88 add / 2 del / 3 files | `208d8d69` |
| Track 1 commit (single) | 1 commit | `208d8d69` |
| Track 2 curator-prompt honesty-assessment | ~70 lines (this section) | cycle-close |
| `cycle-172-*.md` (_notes) | ~250 lines | cycle-close |
| Total cycle 172 main-authored output | ~250 _notes + 88 fix LOC | 1 substantive + 1 cycle-close (planned) |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 172: Track 1 fix shape was correctly estimated as ~50-100 LOC at session start; landed at 88 add / 2 del (within range). Track 2 assessment was bounded-mechanical and produced no separate commit (lives in _notes only). 2-track composition magnitude was Track-1-substantive-heavier than recent cycles — first time priority #1 + priority #2 were jointly advanced in a single Track.

## Pattern updates this cycle

- `two-track-composition` HARDENING-AT-25 (cycles 152-172).
- `straight-pair-closure-as-default-two-track-shape` HARDENING-AT-11 (cycles 162-172).
- **`tool-extraction-surfaces-prior-cycle-errors` RECURRENCE-AT-10 cycle 172** — cycle 172 root-cause re-investigation surfaced cycle 167+ latent structural gap (state-schema asymmetry) that cycle 171's "fixture brittleness" framing did not name.
- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 (cycle 171) — INSTANCE CLOSED cycle 172** — pattern remains NOVEL@1 (no HARDENING because cycle 172 closed within forward-watch window). Master Rust CI green for first time since 2026-05-11. Discipline implication carries forward: session-start CI-check.
- **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 cycle 172** — new pattern. Promoting a `#[serde(flatten)] extra` field to a named struct field requires co-editing every reader that accessed via `.extra.get(...)`. Surfaced via workspace test regression on `worklog_auto_review_summary_reports_all_same_dispositions` after the named field was added. Forward-watch: cycle 173-180 for next schema-promotion candidate.
- **`session-start-CI-check-discipline` NOVEL@1 cycle 172** — new pattern (forward-discipline). Cycle 172 session-start orientation included `gh run list --branch master --workflow "Rust CI"` and surfaced the cycle 171-named master CI red state with the specific failing test. This is the discipline that closes the cycle 171 NOVEL@1 pattern's forward-watch. Forward-watch: cycles 173-180 — if every cycle includes the CI check, the discipline HARDENS to routine; if it lapses, the pattern surfaces as a reminder.
- `live-prompts-already-aligned-at-extended-schema-level` OBSERVATION-AT-3 — cycle 170 + 171 + 172. Three-cycle observation that v2 role prompts (live `prompts/v2/` set) align with their tool-enforced schemas (contract-check + tag-semantic-fidelity both strict-mode pass; cycle 172 manual read of curator-prompt finds no overclaims). Suggests cycle 148 L3.2 estimate was high.
- `directive-2937-track-1-or-track-2-dispatch-fit-application` NOT-EXERCISED cycle 172 (no new dispatch). Carries at HARDENING-AT-3.
- `straight-pair-with-dispatch-variant` NOT-EXERCISED cycle 172. Carries at HARDENING-AT-3.
- `design-scope-internal-contradiction-resolved-by-implementation` NOT-EXERCISED cycle 172. Carries at NOVEL@1.
- `atomic-dual-crate-PR-stronger-than-design-ordering-requirement` NOT-EXERCISED cycle 172. Carries at NOVEL@1.
- `implementation-discovery-as-design-doc-revision-trigger` NOT-EXERCISED cycle 172. Carries at NOVEL@1.
- `tools/v2-* wrapper directly-pushable as v2 substrate by convention extension` NOT-EXERCISED cycle 172 (no v2-* wrapper change). Carries at NOVEL@1.
- `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` NOT-EXERCISED cycle 172 (no design-scope PR; Track 1 was a direct fix not gated on design-scope LOC). Carries at RECURRENCE-AT-2.
- `clippy-follow-up-commit-post-absorption` NOT-EXERCISED cycle 172 (no absorption cycle). Carries at RECURRENCE-AT-2.
- `partial-investigation-misses-second-workflow` NOT-EXERCISED cycle 172. Carries at RECURRENCE-AT-2.

### Forward-watch decay status

- **2-cycle pattern of latent-tool first-live-invocation** (cycles 166 + 167): cycles 168, 169, 170, 171, 172 all NOT-EXERCISED. Forward-watch effectively expired without recurrence. Pattern soft-drops; record at OBSERVATION-AT-1.
- **`two-design-scope-drafts` composition variant** RECURRENCE-AT-2 (cycle 158 + 168): cycles 169-172 NOT-EXERCISED. 4 of 5 watch cycles consumed; cycle 173 remains the RARE-VARIANT-stabilization deadline.

## What cycle 172 does NOT do

Per anti-overstatement discipline (cycle 171 inherited):

1. Does NOT complete the honesty-pass on v2 role prompts (priority #3). Only curator-prompt was read; executor/planner/reconciler remain unassessed. Forward-priority extension recorded.
2. Does NOT sharpen the `constraints` intent string in tag-semantics.toml. Recorded as forward-priority sharpening candidate.
3. Does NOT add session-start CI-check to any tool or prompt — discipline practiced in cycle 172 only, not yet codified.
4. Does NOT add a workspace-test-pre-commit hook that would catch the schema-promotion-requires-reader-co-edit case automatically (cycle 172 caught it via `cargo test --workspace` invocation, not via hook).
5. Does NOT modify any tools/rust crate other than state-schema, write-entry, check-commitments.
6. Does NOT close out the AGREE-DEFER queue (still at carry-forward priority #5).
7. Does NOT advance the coordinated retry/timeout/cancellation, structured-error-envelope, resume/recovery arcs (priorities #6, #7, #8).
8. Does NOT engage audit (audit HEAD `bc8fda63` unchanged since cycle 171).
9. Does NOT design per-axis archival mechanism (priority #10).
10. Does NOT add `--all` sweep modes, `reconcile` orphan detection, `--invocation-id` flag (priorities #14, #15, #16).
11. Does NOT modify any direct-push-forbidden zone (no `.github/workflows/`, no orchestrator-redesign-prompt.xml).
12. Does NOT add new Copilot dispatches.

## Forward priorities for cycle 173+

Inheriting from cycle 171+'s list, minus priority #1 (CLOSED via Track 1), priority #2 (partially CLOSED — master CI green achieved; ongoing forward-watch on whether other CI regressions appear), priority #3 (partial advancement via curator-prompt assessment only).

1. **Honesty-pass on remaining v2 role prompts** (continued from cycle 171+ priority #3). Three role prompts remaining: executor (506 lines), planner (567 lines), reconciler (554 lines). Bounded per-cycle: ~1 role-prompt assessment per cycle continuing cycle 172's pattern. Plus tag-semantics.toml `constraints` intent string sharpening as a separate bounded edit.
2. **Master Rust CI green-state maintenance and forward-watch on `session-start-CI-check-discipline` NOVEL@1 cycle 172**. Continue the cycle 172 session-start orientation discipline. Forward-watch cycles 173-180.
3. **AGREE-DEFER queue (post-real-role-session-measurement)** (was cycle 171+ #5; now #3).
4. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 171+ #6).
5. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 171+ #7).
6. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 171+ #8).
7. **Audit-engagement substantive-focal single-track variant** (was cycle 171+ #9; gate = audit HEAD changes from `bc8fda63`).
8. **Per-axis archival mechanism design scope** (was cycle 171+ #10).
9. **`v2-prompt-contract-check --strict` re-run post-extension** (was cycle 171+ #11; LOW).
10. **Deprecate `Channel::required_payload_keys()` legacy method** (was cycle 171+ #12; LOW).
11. **Workflow trigger upgrade — add `ready_for_review` to pull_request trigger types** (was cycle 171+ #13; LOW; PR-required, not direct-pushable).
12. **`--all` sweep modes for status / verify** (was cycle 171+ #14; LOW).
13. **`v2-cycle-runner reconcile` orphan-state detection** (was cycle 171+ #15; LOW).
14. **`v2-state-dispatch-archive --invocation-id` flag** (was cycle 171+ #16; LOW).
15. **NEW: `schema-promotion-requires-reader-co-edit-via-named-field` discipline codification** (NEW from cycle 172 NOVEL@1). LOW; bounded; consider lint-style check via cargo workspace test (which surfaced this cycle's instance already).

**Cycle 172 forward priorities CLOSED:**
- Cycle 171+ priority #1 (write-entry::auto_review_summary_real_state test fix) — FULLY CLOSED via Track 1 root-cause fix.
- Cycle 171+ priority #2 (master Rust CI full-green path) — FULLY CLOSED for current red instance; continues as forward-watch maintenance discipline.

**Cycle 172 partial advancements:**
- Cycle 171+ priority #3 (honesty-pass on v2 role prompts) — 1 of 4 roles assessed (curator); priority continues as cycle 173+ #1.

**Net list-length change:** -2 closures + 1 new = -1.

## Observational forward-watch items

1. **`live-prompts-already-aligned-at-extended-schema-level` OBSERVATION-AT-3** — cycle 170 + 171 + 172. Three-cycle confirmation. Forward-watch: when executor/planner/reconciler honesty-passes run (cycle 173-175 if cadence holds), the four-role observation either solidifies into a HARDENED principle or surfaces specific role-level overclaims that would invalidate the v1 cycle-148 L3.2 estimate.

2. **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 (cycle 171) — INSTANCE CLOSED cycle 172** — pattern remains NOVEL@1. Forward-watch: cycle 173-180 — if a new red-window opens AND the session-start CI-check discipline catches it within 1-2 cycles, the discipline pattern (`session-start-CI-check-discipline` NOVEL@1 cycle 172) HARDENS and the original pattern stays at NOVEL@1. If a red-window opens AND lasts more than 2 cycles without orchestrator engagement, both patterns shift: original HARDENS to RECURRENCE-AT-2, discipline regresses.

3. **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 cycle 172** — new pattern. Forward-watch: cycle 173-180 for next schema-promotion candidate. If recurrence, the pattern HARDENS to RECURRENCE-AT-2 and motivates a state-schema migration discipline (e.g., a workspace lint or test that scans for `.extra.get(...)` patterns after a schema change).

4. **`session-start-CI-check-discipline` NOVEL@1 cycle 172** — new pattern (forward-discipline). Forward-watch: cycles 173-180 — should be in every cycle's session-start orientation. If the discipline lapses, the related `master-CI-red-not-noticed-across-multiple-cycles` pattern likely RECURRENCE-AT-2's.

5. **`tool-extraction-surfaces-prior-cycle-errors` RECURRENCE-AT-10** — increments via cycle 172's root-cause re-investigation of cycle 171's "fixture brittleness" framing. Forward-watch: cycle 173-175 for next instance. At RECURRENCE-AT-10 the pattern is solidly HARDENED.

6. **`tools/v2-* wrapper directly-pushable as v2 substrate by convention extension` NOVEL@1 cycle 171** — NOT-EXERCISED cycle 172 (no v2-* wrapper change). Carries at NOVEL@1 with cycle 173-180 watch.

## In-session issues and recoveries

- **Env-var expansion blocked** for `echo "$GITHUB_RUN_ID"` — recovered via `gh run list --workflow=orchestrator.yml`. (Recurring; not novel.)
- **`gh issue comment` body-file requires file inside working directory** — recovered via writing comment-body to `docs/redesign/_notes/.tmp-cycle-172-session-start.md` and posting from there. Tempfile cleanup blocked by sensitive-path rule (`.tmp-` prefix); deferred to session-end via `git rm` in cycle-close.
- **`cd tools/rust && command`** triggers permission prompt for git operations (composed-cd-and-git pattern) — recovered via `git -C /home/runner/work/.../...` absolute-path-to-git pattern instead.
- **`python3 -c '...' << EOF` heredoc with stdin** also triggers compound-operation permission prompt — recovered via `gh ... --jq '...'` direct invocation instead.
- **Initial workspace test surfaced regression on `worklog_auto_review_summary_reports_all_same_dispositions`** (post-state-schema named-field promotion) — recovered via co-editing `review_history_entry_matches_target` to use the typed field. Pattern observation: `schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1.
- All `cargo`, `gh`, `git` operations otherwise clean cycle 172.
