# Cycle 154 _notes — v2-state-dispatch-sync Rust tool + list housekeeping

**Cycle:** 154 (two-track composition, 7th consecutive after cycle 151 single-track exception)
**Date:** 2026-05-15
**Cycle issue:** [#2963](https://github.com/EvaLok/schema-org-json-ld/issues/2963)
**Commits this cycle:** `218616a8` (Track 1 v2-state-dispatch-sync crate, 1058 LOC); cycle-close commit
**Predecessor:** cycle 153 (commit `47cf2fdc`, first end-to-end smoke + first measurement + state.json dispatch-tracking housekeeping)
**Reference forward priorities:** [`cycle-153-first-end-to-end-and-housekeeping.md`](cycle-153-first-end-to-end-and-housekeeping.md) §Forward priorities for cycle 154+ (#2 = state-dispatch-sync tool)

## Cycle composition

Two-track composition, 7th consecutive cycle since cycle 151 single-track exception. Cycle 153's forward priority list had:
- #1 (Dispatch #2960 critique absorption) — **blocked** (Copilot connected 10:59:04Z but produced 0 comments as of cycle 154 start)
- #2 (state-dispatch-sync Rust tool) — **next-up substantive focal**, picked as cycle 154 Track 1

**Track 1 (substantive focal):** `v2-state-dispatch-sync` Rust tool — extracts the cycle 153 Track 2 manual housekeeping pattern (11 sequential Edits on state.json) into a reusable tool. CORE-DESIGN-PRINCIPLE alignment: bounded-mechanical procedural work belongs in a tool, not in the orchestrator's per-cycle scratch space.

**Track 2 (bounded act-now):** list housekeeping per SECTION 6b cadence — closed 3 absorbed issues (#2879, #2881, #2849) with forward-link closing comments per SECTION 6b closure-discipline.

## Track 1 — v2-state-dispatch-sync Rust tool

### CLI shape

```
v2-state-dispatch-sync [--state-file PATH] [--repo OWNER/NAME] [--issues NN,NN] <subcommand>

Subcommands:
  audit  Read-only; print planned transitions for in_flight agent_sessions
  sync   Apply transitions to state.json + recount in_flight_sessions
```

Defaults: `--state-file docs/state.json`, `--repo EvaLok/schema-org-json-ld`. The `--issues` scope flag is a comma-separated subset.

### Architecture (3 layers)

1. **`GhClient` trait** abstracts the GitHub query (single method: `query_issue(owner, name, issue) -> IssueStatus`). Production `GhCliClient` subprocess-spawns `gh api graphql` with parameterized query. Tests use `MockGh` with HashMap-based canned responses.

2. **`compute_plan(state, client, owner, name, scope)`** is the pure orchestration function: scan agent_sessions, find in_flight entries (filter by scope if set), call `client.query_issue` per issue, build `Vec<Plan>` describing each transition.

3. **`apply_plan(state, plans)`** is the pure mutation function: takes the parsed state Value + plans, modifies agent_sessions entries in place, recomputes top-level `in_flight_sessions` count. Pure: state in → state out (no I/O, no network).

### GraphQL query

```graphql
query($owner: String!, $name: String!, $number: Int!) {
  repository(owner: $owner, name: $name) {
    issue(number: $number) {
      state
      closedByPullRequestsReferences(first: 10, includeClosedPrs: true, userLinkedOnly: false) {
        nodes { number merged mergedAt }
      }
    }
  }
}
```

Per-issue: 1 GraphQL call returns both the issue state and any merged-PR closure linkage. `userLinkedOnly: false` includes auto-linked references (e.g., "Closes #N" markup in PR body). The first merged PR in the returned list wins.

### Status-transition logic

For each in-flight issue NN:
- `state == OPEN` → unchanged (stays in_flight)
- `state == CLOSED`, `closedByPullRequestsReferences` empty or no merged PR → `closed_without_pr`
- `state == CLOSED`, merged PR found → `merged` + `pr=<N>` + `merged_at=<ts>`

On `merged` transition: `pr` and `merged_at` are written into the agent_sessions entry.
On `closed_without_pr` transition: any stale `pr` / `merged_at` fields are scrubbed (defensive).
On any transition: `in_flight_sessions` top-level count is recomputed by scanning the post-mutation array.

### Test coverage (26 unit tests)

| Layer | Tests |
|---|---|
| Repo parsing | 4 |
| `find_in_flight` | 3 |
| `parse_issue_response` (GraphQL parse) | 5 |
| `compute_plan` (with MockGh) | 4 |
| `apply_plan` (pure mutation) | 4 |
| State I/O roundtrip | 2 |
| `run()` orchestration (audit/sync/scoped) | 4 |

`apply_plan` tests cover: merged transition, closed_without_pr with stale-field scrubbing, preservation of unrelated entries, empty-plan count normalization. `run()` tests use MockGh + tempdir to drive the full audit/sync/scoped flows end-to-end without touching real state.json or hitting `gh`.

`cargo test -p v2-state-dispatch-sync` green (26/26). `cargo clippy --all-targets -D warnings` clean.

### Live smoke test against docs/state.json

```
$ cargo run --manifest-path tools/rust/Cargo.toml -p v2-state-dispatch-sync -- audit
Plan: 1 entries audited, 0 transitions, 1 unchanged

Unchanged (still open on GitHub):
  #2960: in_flight (no change)

Resulting in_flight_sessions count: 1
```

Verifies:
- Real `gh api graphql` subprocess call succeeds (no auth issues)
- Real GraphQL response parses correctly
- #2960 (the cycle 152 critique dispatch, currently the sole in-flight entry per cycle 153 Track 2 housekeeping) is correctly identified as OPEN
- No mutation to state.json on audit subcommand

### LOC + commit-shape

| File | LOC |
|---|---|
| `Cargo.toml` | 13 |
| `src/main.rs` (production code: ~290 LOC; tests: ~440 LOC) | 730 |
| **Crate total** | 743 |
| `Cargo.lock` delta | +11 (indexmap + hashbrown + equivalent for `preserve_order`) |
| **Net new lines committed** | 1058 |

Single direct-push commit `218616a8`. Direct-push zone per redesign prompt SECTION 2 (`tools/rust/crates/v2-*`).

Notable: tests/production ratio is roughly 60/40 (440 test LOC / ~290 production LOC). For a tool whose runtime mutations are non-reversible (writes to state.json), this ratio is the right discipline — pure-function `apply_plan` is exhaustively covered without any network/IO.

## Track 2 — list housekeeping (SECTION 6b)

Three absorbed issues closed with forward-link closing comments per SECTION 6b `closure-discipline` ("A closure without a forward-link is worse than leaving the issue open").

| Issue | Type | Forward-link |
|---|---|---|
| #2879 | dispatch-test (2026-05-08) | [`doc/adr/0016-copilot-dispatch-mechanism-graphql.md`](../../../doc/adr/0016-copilot-dispatch-mechanism-graphql.md) |
| #2881 | dispatch-test (2026-05-08) | [`doc/adr/0016-copilot-dispatch-mechanism-graphql.md`](../../../doc/adr/0016-copilot-dispatch-mechanism-graphql.md) |
| #2849 | audit-request (2026-05-05) | [`docs/redesign/_notes/cycle-85-audit-454-absorption.md`](cycle-85-audit-454-absorption.md) + audit-repo #454 |

#2879 + #2881 verified the Copilot dispatch mechanism that ADR 0016 then documented; mechanism now standard in `tools/dispatch-task`. #2849 audit-request was responded to by audit-repo #454 and absorbed cycle 85; the cluster framework went on to anchor Phase 2 candidate authoring cycles 87-91.

**SECTION 6b conservative-defaults exercised:** initially considered closing #2849 but verified via audit-repo #454 + cycle 85 _notes existence before committing to closure. The conservative default ("when in doubt, leave it open") was triggered then resolved by verification, not bypassed.

**Open count delta:** 9 → 6 open issues post-cycle-154. Remaining open: #2963 (cycle issue, closing now), #2960 (in-flight Copilot critique), #2937 (input-from-eva, standing constraint), #2794 (input-from-eva, Phase 1 firewall standing constraint), #2741 (input-from-eva, redesign mode active standing constraint), #808 (input-from-eva, pause language ports standing constraint). All remaining-open issues are either active in-flight work or standing input-from-eva constraints — neither category qualifies for SECTION 6b closure.

## Pattern updates this cycle

- **`two-track-composition` HARDENING-AT-7** cycle 154 (3rd consecutive resumed after cycle 151 single-track exception). Combined arc: 146-150 (5 consecutive) → 151 single-track exception → 152-154 resumed (3 consecutive post-exception).
- **`cycle-153-priority-#1-stays-blocked-when-async-dispatch-doesnt-return-in-N-cycles`** NOVEL@1 cycle 154. Forward-priority lists name #1 by substantive value, but priority #1 can stay blocked across multiple cycles when its dependency (a dispatch return) is async and outside main's control. Cycle 154 picked priority #2 instead. Pattern: forward-priority lists are not strict queues; they're ranked-by-value lists where the next-eligible item is taken. NOVEL@1; recurrence check next time a forward priority #1 stays blocked across consecutive cycles.
- **`tool-extraction-of-recently-surfaced-manual-housekeeping-pattern`** NOVEL@1 cycle 154. Cycle 153 Track 2 surfaced the state.json housekeeping pattern (11 manual Edits); cycle 154 Track 1 extracted it into a tool one cycle later. Single-cycle round-trip from "we did this manually and it was tedious" to "we have a tool that does this". CORE-DESIGN-PRINCIPLE in motion. Watch whether this round-trip recurs at single-cycle latency or stretches.
- **`gh-graphql-subprocess-pattern-for-rust-tools-querying-github`** NOVEL@1 cycle 154. Established the pattern: `Command::new("gh").args(["api", "graphql", "-f", "query=...", "-F", "var=val"]).output()` + `serde_json::from_slice` on stdout. Lighter than depending on `octocrab`; reuses the runner's already-authenticated `gh` CLI. Pattern is composable: any v2-* Rust tool that needs to query GitHub can use this same shape. Recurrence test: next v2-* Rust tool that needs a GitHub query.
- **`magnitude-prediction-precision-is-shape-dependent-not-flat`** new datapoint: tool-with-trait-abstraction-and-mock + GraphQL parse + state-mutation logic = 743 LOC at 60/40 test/production ratio. Previous datapoints were "stateful state-machine runner" shape (1843 LOC at minimal-3-subcommand + integration tests); this is a different shape (single-process tool with single external dependency). Band for this shape: ~600-900 LOC depending on test coverage discipline.
- **`pure-function-decomposition-enables-network-free-test-coverage`** NOVEL@1 cycle 154. Splitting `apply_plan` (pure mutation: state → state) from `compute_plan` (orchestration: state + client → plans) from `query_issue` (network: client → IssueStatus) enables exhaustive unit-test coverage of mutation logic with zero network dependency. Pattern: any tool whose mutations are non-reversible should isolate the mutation logic from the data-fetching logic for testability. NOVEL@1; recurrence test on next v2-* tool with a mutation surface.

## Forward priorities for cycle 155+

1. **Dispatch #2960 critique absorption** — STILL the highest-priority focal when the commit-as-file lands at `docs/redesign/_critique/cycle-152-v2-cycle-runner-adversarial-critique.md`. Per-finding evaluation per cycle 148 pattern. Likely cycle 155 substantive focal if Copilot returns. If still no return by cycle 155, escalate (per cycle 147 pattern of re-dispatching lost critiques).

2. **AGREE-ACT-NOW L1.3 + L3.6 reconciler completeness-metadata pairing** — bounded; prompt edit + v2-channel-router required-key extension. Next-up Track 1 candidate if dispatch return still blocked.

3. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** — document-only.

4. **AGREE-ACT-NOW X2 honest cycle-1-scope-redefinition** — document-only.

5. **AGREE-RECORD L1.2 + X4 side-channel architecture-notes doc** — document-only.

6. **TOOL-SCOPED v2-channel-router enforcement extension design scope** (C1 / L3.1).

7. **TOOL-SCOPED v2-prompt-tag-semantic-fidelity tool design scope** (L2.5).

8. **`status` + `verify` v2-cycle-runner subcommands** — cycle 156+ if needed.

9. **AGREE-DEFER queue (post-real-role-session-measurement)** — curator complexity rework, template-mirror architecture-revisit. Hold for live-claude-code-spawn evidence (still SCAFFOLD).

10. **Cycle 120 L2 preserved.**

**Cycle 153 priority #2 (state-dispatch-sync tool)** is CLOSED by cycle 154 Track 1.

## What cycle 154 does NOT do

- Does NOT modify `cycle-runner` (forbidden zone preserved during Phase 3).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT implement `status` / `verify` v2-cycle-runner subcommands (cycle 155+ if needed).
- Does NOT absorb dispatch #2960 critique (still not returned).
- Does NOT escalate dispatch #2960 (single-cycle non-return is within the cycle 147 pattern's tolerance; will reassess cycle 155 or 156 if still blocked).
- Does NOT spawn live Claude sessions (role-driver remains SCAFFOLD).
- Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Does NOT run `v2-state-dispatch-sync sync` against the live `docs/state.json` (audit was sufficient; sync would produce a no-op formatting-diff that's not worth landing).
- Does NOT escalate any cycle 154 decision to Eva (EVA-DEFAULT-AUTONOMY).

## Process honoring

- **39th consecutive cycle of HONORING named forward priority** (cycles 115-154). Cycle 153 named #2 = `state-dispatch-sync` Rust tool; cycle 154 Track 1 closes that priority.
- **67th bottleneck-asynchronous cycle** (78-154).
- **44th non-per-candidate-sharpening cycle** (111-154).
- Cycle 120 L2 preserved (`2-selection.md` untouched).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start comment, commit message, 3 closing comments, session-end comment, cycle-close ephemerals).
- Cycle 133 clarification preserved: cargo invocations cycle 154 are legitimate (1 build + 1 test + 1 clippy + 1 live audit run; not gratuitous).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `333a745d`).
- Cycle 137 lessons re-validated cycle 154 — `find -type f -name '*0016*'` succeeded (single-operation form), per-issue `gh issue close` operations were independent and ran cleanly. **8-cycle running validation** (137 + 147 + 148 + 149 + 150 + 151 + 153 + 154). Cycle 152 had no validation event; cycle 154 had several single-purpose-bash-invocation pattern uses.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 154** — all cargo invocations via `--manifest-path tools/rust/Cargo.toml`. **6-cycle running validation** (149 + 150 + 151 + 152 + 153 + 154).
- Cycle 151 date-test-value lesson preserved (no new date tests cycle 154).
- Cycle 152 disk-format-read-source lesson re-validated cycle 154 — the GraphQL response parsing was authored after sampling a real `gh api graphql` response shape; not assumed from schema. `parse_issue_response` tests cover open / closed-no-PR / closed-merged-PR / missing-issue / missing-data shapes explicitly.
- Cycle 153 multi-Edit-fallback lesson preserved (not exercised this cycle — the Rust tool sidesteps the bash redirect block entirely by using `std::fs::write` on `state.json`).
- Journal-immutability discipline preserved (cycle 154 appends NEW section via Edit anchor at end of cycle 153 section; cycle 148-153 sections NOT back-edited).
- Anti-overstatement audit explicit ("What cycle 154 does NOT do" enumeration above).
- **Two-track composition continued** — cycle 154 is the 3rd consecutive two-track cycle post cycle 151 exception (HARDENING-AT-7).
- **SECTION 6b conservative-defaults exercised** — verified #2849 absorption via audit-repo + cycle 85 _notes before committing to closure; did NOT close it on assumption alone.

## In-session issues and recoveries

- **One `find docs/decisions/` lookup failed** with exit 1 (directory doesn't exist). The actual ADR path is `doc/adr/` (note: singular, `doc/`, not `docs/`). Recovered via broader `find . -maxdepth 5 -name "*.md" -path "*adr*"`. Lesson: when looking for project-wide convention directories (ADR, RFC, decisions), don't assume the path; grep for references to confirm.
- **`grep -l ... docs/redesign/_notes/cycle-8[0-9]*.md cycle-9[0-9]*.md`** initially returned files from a different search angle (self-congratulation audit, cluster A-vs-G stress test, over-under-prescription audit — all from cycles 80-83) that wasn't the cycle 85 absorption note. The actual confirmation came from `gh api repos/.../issues/454` returning audit-repo #454's response body which named #2849 explicitly. Lesson: when verifying issue absorption, the audit-repo response itself is often the cleanest direct evidence.
- All `cargo build` / `cargo test` / `cargo clippy` / `cargo run` operations clean cycle 154 (after one fix: tempfile was used in tests but I'd omitted it from dev-dependencies in the initial Cargo.toml; one-line Edit added it).
- All `gh issue comment` / `gh issue close` / `gh api` operations clean cycle 154.
- All Write / Edit operations clean cycle 154.

## Cycle 154 ARTIFACTS

- `tools/rust/crates/v2-state-dispatch-sync/Cargo.toml` — new (15 LOC including blank line + dev-dependencies fix).
- `tools/rust/crates/v2-state-dispatch-sync/src/main.rs` — new (730 LOC: ~290 production + ~440 tests).
- `tools/rust/Cargo.lock` — modified (+11 lines for `indexmap`, `hashbrown`, `equivalent` deps added by serde_json `preserve_order` feature).
- `docs/redesign/_notes/cycle-154-state-dispatch-sync-tool-and-list-housekeeping.md` — new (this file).
- This journal section appended via Edit anchor at end of cycle 153 section; cycle 148-153 sections preserved.
- Track 1 single direct-push commit `218616a8` (the new crate + Cargo.lock delta).
- Cycle-close commit (this _notes + journal append).
- `.scratch/` ephemerals: `cycle154-session-start.md`, `cycle154-track1-commit-msg.txt`, `cycle154-close-2879.md`, `cycle154-close-2881.md`, `cycle154-close-2849.md`, `cycle154-session-end.md` (authored cycle-close), `cycle154-issue-close.md` (authored cycle-close).
- 0 modifications to existing v2-* crates (Track 1 is a brand-new crate; no edits to v2-cycle-runner / v2-channel-router / etc.).
- 3 issue closures with forward-link comments (#2879, #2881, #2849).
- 0 dispatches cycle 154 (no Copilot agent-task dispatches needed; #2960 still in-flight; cycle 152's dispatch count of 1 plus cycle 154's 0 means the 2-cycle dispatch rate is 1, well within input-from-eva #2937 directive's expected cadence).
- 4 cargo invocations cycle 154: build new crate; test new crate; clippy new crate; live audit run against state.json. All clean (after the one-line tempfile dep fix).
- 1 Edit on Cargo.toml (add tempfile to dev-dependencies — caught during first test run).
- 0 Edits on `docs/state.json` (the tool was built, not exercised in sync mode; audit verified 0 transitions needed).
