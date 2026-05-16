# Cycle 161 detail _notes — v2-state-dispatch-policy-enforcement design scope + X5 commit-governance scope clarification

**Date:** 2026-05-16
**Cycle issue:** [#2970](https://github.com/EvaLok/schema-org-json-ld/issues/2970)
**Composition:** two-track, HARDENING-AT-14 post cycle 151 single-track exception (10 consecutive post-exception; 15 of 16 in arc 146-161)
**Forward priorities source:** [`cycle-160-reconciler-completeness-metadata-and-c10-amendment.md`](cycle-160-reconciler-completeness-metadata-and-c10-amendment.md#forward-priorities-for-cycle-161)

## Cycle 161 input

Session-start checks (all single-purpose-bash-invocation form):

- **Audit HEAD:** `8285b7d3` — unchanged from cycle 157 absorption (audit cycle 221). No new audit content cycle 222 yet. The audit's cycle 222 watch items name potential first substantive Phase 3 audit-engagement filing as a candidate; no advance this cycle.
- **Input-from-eva:** 4 standing (#2937, #2794, #2741, #808). No new directives since cycle 152 absorption of #2937.
- **Open-issue count:** 5 (cycle issue + 4 standing input-from-eva). SECTION 6b housekeeping NOT invoked (no closure candidates beyond the cycle issue itself).
- **In-flight dispatches:** 0.

## Composition rationale

Two-track composition continues. Cycle 158 was symmetric-closure (both new priorities closed in N+1); cycle 159 was asymmetric-closure (gate-bounded #15 deferred); cycle 160 was mixed-closure (different deferral ages); cycle 161 is **paired-closure** — Track 1 closes a single priority that is intrinsically paired (#3 + #4 share an architectural problem, so they close together as a single design scope rather than as two separate closures).

This expands the pattern catalog:
- Symmetric closure (cycle 158): N produces 2, N+1 closes the same 2.
- Asymmetric closure (cycle 159): N produces 2, N+1 closes 1 of them, the other is gate-bounded.
- Mixed closure (cycle 160): N+1 closes priorities of different deferral ages.
- **Paired closure (cycle 161):** N+1 closes 2 priorities as a single piece of work because they share an underlying problem.

Track 2 is a long-deferred bounded textual fix (X5 carveout, deferred since cycle 155).

## Track 1 — v2-state-dispatch-policy-enforcement design scope

**Closes cycle 160 forward priorities #3 + #4** as a single paired closure. Cycle 160's note explicitly flagged this as design-scope-shape:

> the actual `add` happens in v1 `record-dispatch` (frozen zone), not v2-state-dispatch-sync. A design scope cycle may be the right shape rather than direct code work.

This document settles the naming with a three-layer ownership clarification.

### Target file

[`docs/redesign/_notes/v2-state-dispatch-policy-enforcement.md`](v2-state-dispatch-policy-enforcement.md) — new, 196 lines, direct-push commit [`c5e4bb27`](https://github.com/EvaLok/schema-org-json-ld/commit/c5e4bb27).

### Three-layer ownership (the key contribution)

| Operation | Owner | Zone | Modification path |
|---|---|---|---|
| Append (new dispatch) | v1 `record-dispatch` | FROZEN | workflow-change PR (heavy) |
| Transition (in_flight → terminal) | v2 `v2-state-dispatch-sync` | direct-push | direct |
| Enforce (cross-axis ceiling + halt) | v2 `v2-state-audit` + `v2-cycle-runner` session-start wiring | direct-push | direct |

The cycle 158 policy line "Owner: v2-state-dispatch-sync" was ambiguous; this scope makes the writer/transitioner/enforcer split explicit.

### Live population analysis (cycle 161 measurement)

`jq` on `docs/state.json.agent_sessions`:

| Status | Count | Share | Lifecycle |
|---|---|---|---|
| merged | 819 | 88.1% | TERMINAL |
| failed | 62 | 6.7% | TERMINAL |
| closed_without_pr | 25 | 2.7% | TERMINAL |
| closed | 15 | 1.6% | TERMINAL |
| closed_without_merge | 6 | 0.6% | TERMINAL |
| reviewed_awaiting_eva | 2 | 0.2% | LIVE |
| in_flight | 1 | 0.1% | LIVE |
| **Total** | **930** | 100% | — |

**Key observation:** 922 of 930 (99.1%) are terminal; only 3 are live. The threshold model needs to reflect this — flat thresholds against total count conflate live-operational-state from archival-pending-state.

### Recalibration: live+total split thresholds

| Tier | LIVE (operational anomaly) | TOTAL (archival neglect) | Rationale |
|---|---|---|---|
| Advisory | 100 | 1000 | Bounded concurrency / first archival trigger |
| Mandatory | 200 | (drives archival; total tracks archival debt) | Operator action |
| Hard | 500 | 2000 | Halt at session-start |

Cycle 161 current state under this model: LIVE=3 (well below advisory), TOTAL=930 (between mandatory and hard).

### Archival recommendation: Option B (separate sweep tool)

Three options sketched in the design scope (§4.2). Recommended:
- **Option B**: `v2-state-dispatch-archive` periodic age-based sweep tool, separate from v2-state-dispatch-sync.
- Rejected: Option A (transition-time archival inside v2-state-dispatch-sync — scope expansion), Option C (in-place compaction — destroys audit-trail value).

### Implementation order

1. **Cycle 162+**: wire v2-state-audit at v2-cycle-runner session-start with `--halt-on-hard` flag.
2. **Cycle 163+**: build `v2-state-dispatch-archive` per Option B.
3. **Cycle 164+**: backlog archival run (819 merged → archive; live → ~111).
4. **Cycle 165+**: v2-state-retention-policy.md §4 Axis 6 patch.

Step 1 is the smallest next bite — re-uses existing tooling, no new policy decisions, single-cycle shape.

### What the design scope does NOT do (its own §7)

10 explicit non-doings — see the design scope file itself.

## Track 2 — X5 commit-governance scope clarification

**Closes cycle 160 forward priority #1** (deferred since cycle 155 absorption of PR #2961 cycle 152 v2-cycle-runner critique X5 finding, AGREE-WITH-CARVEOUT verdict).

### Target file

[`tools/rust/crates/v2-cycle-runner/src/main.rs`](../../tools/rust/crates/v2-cycle-runner/src/main.rs) — modified, +31 LOC (header comment block above `use clap::...`). Single direct-push commit [`aeb2b0f8`](https://github.com/EvaLok/schema-org-json-ld/commit/aeb2b0f8).

### What the header declares

The header explicitly names:
- `commit-shape / commit-governance discipline` as **OUT OF SCOPE for v2-cycle-runner**.
- The cycle 155 AGREE-WITH-CARVEOUT verdict and the CORE-DESIGN-PRINCIPLE rationale (separation between runner's step-execution responsibility and sibling-tool responsibilities).
- Forward-pointer to a hypothetical `v2-commit-discipline-check` sibling tool if commit-shape verification becomes wanted later.
- Cross-references to: cycle 149 design scope, cycle 152 critique X5 (line 146), cycle 155 absorption verdict (line 101), and the cycle 161 closure here.

### Pattern match

Matches the `//` module-header pattern used by `v2-state-audit` and `v2-dispatch-status`. No conflict with the existing `#[command(about = ...)]` macro description (preserved unchanged).

### Verification

- `cargo build -p v2-cycle-runner --manifest-path tools/rust/Cargo.toml`: clean.
- `cargo test -p v2-cycle-runner --manifest-path tools/rust/Cargo.toml`: 36 unit + 2 integration = 38/38 green.
- `cargo clippy -p v2-cycle-runner --manifest-path tools/rust/Cargo.toml --all-targets -- -D warnings`: clean.

No code semantics change — comment-only addition.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 design scope `v2-state-dispatch-policy-enforcement.md` | 196 lines (+0 -0 net add) | `c5e4bb27` |
| Track 2 header in `v2-cycle-runner/src/main.rs` | +31 LOC | `aeb2b0f8` |
| `cycle-161-dispatch-policy-enforcement-and-x5-scope.md` (this _notes) | ~285 lines | cycle-close |
| Total cycle 161 textual + code output | ~510+ lines | 3 commits |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 161 datapoints:
- Design-scope shape (Track 1): ~200 lines — consistent with cycle 158 v2-state-retention-policy.md (230 lines) and v2-critique-task-class-taxonomy.md (199 lines). Predictable range now: 200-230 lines.
- Bounded textual shape (Track 2): +31 LOC. Smaller than cycle 160 Track 2 C10 amendment (+3 LOC) was on lines; this is bigger because it adds a full header block. Cross-cycle range for "header block addition" shape is now established.

## Pattern updates this cycle

- **`paired-closure-of-two-cycle-N-priorities-as-single-piece-of-work`** NOVEL@1 cycle 161. Cycle 160 forward priorities #3 + #4 closed as a single design scope because they share an architectural problem (state-json-dispatches axis enforcement). Distinct from symmetric closure (where 2 separate pieces of work close together) and from paired ACT-NOW+WITH-CARVEOUT verdict (cycle 155 C10 had paired verdict but separate closure). Recurrence test: any cycle where the forward-priority list contains two items that share an underlying problem.
- **`design-scope-as-priority-reframing-mechanism`** NOVEL@1 cycle 161. Cycle 160's #3 framing was "v2-state-dispatch-sync refuse-to-write hard-threshold enforcement"; cycle 161 design scope reframes to "v2-state-audit session-start wiring" because the named owner is structurally not the appender. Pattern: design scopes can REFRAME forward priorities, not only EXECUTE them. The forward-priority text becomes a hypothesis to test against the actual code surface.
- **`three-layer-ownership-clarification-as-design-pattern`** NOVEL@1 cycle 161. Writer / transitioner / enforcer split for state surfaces with multiple operations. Applicable beyond agent_sessions: any state surface with append + transition + audit operations may benefit. Candidate for v2-state-retention-policy.md §2 generalization in a future patch.
- **`live-vs-total-threshold-split`** NOVEL@1 cycle 161. Recalibration approach that distinguishes operational anomaly (live count) from archival neglect (total count). Applicable to any append-only axis with terminal-state lifecycle.
- **`module-header-scope-declaration-pattern`** RECURRENCE-AT-3 cycle 161. v2-state-audit + v2-dispatch-status + (now) v2-cycle-runner all use `//`-header scope declarations. Pattern: for v2 crates, the module header is the canonical location for OUT-OF-SCOPE declarations.
- **`agree-act-now-bounded-fix-via-tracksecond-pattern`** RECURRENCE-AT-6 cycle 161 (150, 155, 156, 159, 160, 161). Pattern: bounded ACT-NOW critique findings excel as Track 2 work.
- **`two-track-composition` HARDENING-AT-14** cycle 161. 10 consecutive post cycle 151 exception (152-161). Combined arc: 146-150 (5) + 151 single-track exception + 152-161 (10) = 15 of 16 cycles two-track.
- **`tool-extraction-surfaces-prior-cycle-errors` BROADENED** cycle 161. Cycles 156, 159 surfaced errors via tool extraction; cycle 160 via schema-bump; cycle 161 via design-scope-as-reframing. Pattern broadens: any structurally-novel operation against existing code surfaces can surface prior-cycle imprecisions.

## Process honoring

- **46th consecutive cycle of HONORING named forward priority** (cycles 115-161). Cycle 160 named #3 + #4 (paired, Track 1) + #1 (X5, Track 2); cycle 161 closes all 3.
- **74th bottleneck-asynchronous cycle** (78-161).
- **51st non-per-candidate-sharpening cycle** (111-161).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 161).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start comment, 2 commit messages, cycle-close ephemerals).
- Cycle 133 clarification preserved: 3 cargo invocations cycle 161 (build + test + clippy on v2-cycle-runner for Track 2 header verification; bounded-purpose).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `8285b7d3`).
- **Cycle 137 lessons re-validated cycle 161 15-cycle-running** (137 + 147-161). All `gh api` / `gh issue list` / `gh issue comment` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 161 10-cycle-running** (149-161 minus 158/160-text-only). All cargo via `--manifest-path tools/rust/Cargo.toml`.
- Cycle 151 date-test-value lesson preserved (no new date tests cycle 161).
- Cycle 152 disk-format-read-source lesson preserved.
- Cycle 153 multi-Edit-fallback lesson preserved (not exercised cycle 161; single Edit on Track 2).
- Cycle 154 `gh api graphql` subprocess pattern preserved (not exercised cycle 161).
- Cycle 155 dispatch-return-detection lesson preserved (v2-dispatch-status not invoked cycle 161; no in-flight dispatches).
- Cycle 156 anti-overstatement audit enumeration discipline preserved (this _notes has explicit "What cycle 161 does NOT do" with 16 items below).
- Cycle 157 audit-HEAD-check at session-start preserved (HEAD unchanged).
- Cycle 158 symmetric-closure pattern preserved; cycle 159 asymmetric (gate-bounded); cycle 160 mixed-closure (different deferral ages); cycle 161 paired-closure (NOVEL@1).
- Cycle 159 test-pattern-mirror-existing-control-flow lesson preserved (not exercised; Track 2 was comment-only, no new tests added).
- Cycle 160 schema-duplication observation preserved (not exercised cycle 161; design-scope and comment-only work).
- Journal-immutability discipline preserved (cycle 161 will append NEW section via Edit anchor at end of cycle 160 section; cycle 148-160 sections NOT back-edited).
- **Two-track composition continued** — cycle 161 is 10th consecutive post cycle 151 exception (HARDENING-AT-14).
- **SECTION 6b list housekeeping NOT invoked cycle 161** — 5 currently-open issues (cycle issue + 4 standing input-from-eva); no closure candidates.

## In-session issues and recoveries

- **No in-session errors encountered cycle 161.** All file operations clean. Both Track commits + pushes clean. Session-start checks (audit HEAD, input-from-eva count, open issues) all single-purpose bash invocations.
- All Edit / Write operations clean cycle 161 (1 Edit on v2-cycle-runner main.rs for Track 2 header; 4 Writes for design scope + 3 ephemerals).
- All `gh api` / `gh issue list` operations clean cycle 161.
- All `cargo build` / `cargo test` / `cargo clippy` operations clean cycle 161.
- All `git add` / `git commit` / `git push` operations clean cycle 161.

## Forward priorities for cycle 162+

Renumbered list (cycle 160 inherited + cycle 161 closures applied + cycle 161 new sub-priorities from Track 1 design scope §6):

1. **v2-state-audit session-start wiring in v2-cycle-runner** (NEW from cycle 161 Track 1 §6 / supersedes cycle 159 priority #5 reframing): add `state-audit-on-start` step before super-step phase advance; halt with `halt_reason=state-bound-exceeded` if `v2-state-audit` returns exit 3. New halt class. Estimate: 1 cycle (similar shape to cycle 159 C2 phase-marker).

2. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** (from cycle 148 absorption, was cycle 160 priority #2). Carries forward. <1 cycle.

3. **v2-state-dispatch-archive tool design scope** (NEW from cycle 161 Track 1 §6): bounded scope for the archive sweep tool. ~200-250 lines design-scope-textual (cycle 158 shape).

4. **v2-state-dispatch-archive implementation** (NEW from cycle 161 Track 1 §6): build the crate, ~1000 LOC + tests (cycle 159 v2-state-audit shape).

5. **Backlog archival run** (NEW from cycle 161 Track 1 §6): one-time invocation of v2-state-dispatch-archive against the 819-merged backlog.

6. **v2-state-retention-policy.md §4 Axis 6 recalibration patch** (NEW from cycle 161 Track 1 §6): apply live+total split thresholds once steps 1-5 above land; remove cycle 159 storage-key-clarification "forward work" stub.

7. **TOOL-SCOPED `v2-channel-router` enforcement extension design scope** (C1 / L3.1 from cycle 148 absorption, was cycle 160 priority #5). Carries forward.

8. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (L2.5 from cycle 148 absorption, was cycle 160 priority #6). Carries forward.

9. **`status` + `verify` v2-cycle-runner subcommands** (was cycle 160 priority #7). Carries forward.

10. **AGREE-DEFER queue (post-real-role-session-measurement)** (was cycle 160 priority #8). Hold for live-claude-code-spawn evidence (still SCAFFOLD).

11. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 160 priority #9).

12. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 160 priority #10).

13. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 160 priority #11).

14. **Audit-engagement substantive-focal single-track variant** (was cycle 160 priority #12). Gate = audit HEAD changes.

15. **Per-axis archival mechanism design scope** (was cycle 160 priority #13). Gate-bounded.

16. **Missing Integration Scenario 3 — super-step-out-of-order live integration test** (was cycle 160 priority #14, unblocked by cycle 160 C10 amendment). <1 cycle if paired.

17. **Cycle 120 L2 preserved** (no recursive annotation of `2-selection.md`).

**Cycle 161 forward priorities CLOSED:**
- Cycle 160 priorities #3 + #4 (paired into Track 1 design scope).
- Cycle 160 priority #1 (X5 carveout, Track 2 header).

**Cycle 161 new sub-priorities produced from Track 1 design scope §6:** 5 items (cycle 162+ priorities #1, #3, #4, #5, #6 above).

## What cycle 161 does NOT do (anti-overstatement audit)

1. Does NOT modify v1 `record-dispatch` (frozen zone preserved).
2. Does NOT wire v2-state-audit into v2-cycle-runner session-start (cycle 162+ priority #1).
3. Does NOT build v2-state-dispatch-archive (cycle 163+ priorities #3 + #4).
4. Does NOT run backlog archival against the 819-merged backlog (cycle 164+ priority #5).
5. Does NOT patch `v2-state-retention-policy.md` §4 Axis 6 thresholds (cycle 165+ priority #6).
6. Does NOT modify `.github/workflows/` or this orchestrator prompt.
7. Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
8. Does NOT close any issue beyond the cycle issue itself.
9. Does NOT dispatch any new Copilot work (no in-flight dispatches; nothing to wake on).
10. Does NOT alter the existing `Phase` enum or any cycle 159 Track 2 surface area.
11. Does NOT alter v2-state-audit or v2-state-retention-policy doc content (Track 1 only ADDS a new design scope file).
12. Does NOT add new tests to v2-cycle-runner (Track 2 is comment-only).
13. Does NOT change the `RunReport` / `StepTrace` schema (Track 2 explicitly preserves them).
14. Does NOT propose modifying the dispatch-entry schema (status enum, fields, etc.); Track 1 recalibration is over thresholds, not data shape.
15. Does NOT address axes 1-5 from cycle 158 policy (Track 1 is Axis-6-specific).
16. Does NOT pre-design halt-recovery flow for `state-bound-exceeded` (operator-review-required is the cycle 158 stance; cycle 162+ wiring may revisit).
17. Does NOT escalate any cycle 161 decision to Eva (EVA-DEFAULT-AUTONOMY: design-space questions resolved within the cycle).

## Cycle 161 ARTIFACTS

- `docs/redesign/_notes/v2-state-dispatch-policy-enforcement.md` — new (Track 1, 196 lines, commit `c5e4bb27`).
- `tools/rust/crates/v2-cycle-runner/src/main.rs` — modified (Track 2, +31 LOC, commit `aeb2b0f8`).
- `docs/redesign/_notes/cycle-161-dispatch-policy-enforcement-and-x5-scope.md` — new (this cycle's detail _notes, ~285 lines).
- Journal section in `docs/journal/2026-05-16.md` — appended via Edit anchor at end of cycle 160 section; cycle 148-160 sections NOT back-edited.
- `.scratch/cycle161-session-start.md` — session-start comment body (ephemeral).
- `.scratch/cycle161-track1-msg.txt` — Track 1 commit message (ephemeral).
- `.scratch/cycle161-track2-msg.txt` — Track 2 commit message (ephemeral).
- `.scratch/cycle161-session-end.md` — session-end comment (ephemeral; authored cycle-close).
- `.scratch/cycle161-issue-close.md` — cycle issue close comment (ephemeral; authored cycle-close).
- 2 Track-side direct-push commits: `c5e4bb27` (Track 1) + `aeb2b0f8` (Track 2).
- 1 cycle-close commit (this _notes + journal section + ephemerals).
- 0 issues closed cycle 161 (no candidates; cycle issue itself closes per existing convention).
- 0 dispatches cycle 161.
- 3 cargo invocations cycle 161 (build + test + clippy on v2-cycle-runner for Track 2 verification).
- ~6 GitHub API operations cycle 161 (audit HEAD, input-from-eva list, open issues list, agent_sessions count via jq locally, session-start comment post, cycle close).
- 1 Edit on `tools/rust/crates/v2-cycle-runner/src/main.rs` (Track 2 header).
- 0 Edits on `tools/rust/crates/v2-state-audit/src/main.rs`.
- 0 Edits on `tools/rust/crates/v2-state-dispatch-sync/src/main.rs`.
- 0 Edits on `tools/rust/crates/v2-dispatch-status/src/main.rs`.
- 0 Edits on `docs/state.json` (read-only `jq` for population analysis).
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.
- 0 Edits on `docs/redesign/_notes/v2-state-retention-policy.md` (patches deferred to cycle 165+ per Track 1 §4.3 ordering).
