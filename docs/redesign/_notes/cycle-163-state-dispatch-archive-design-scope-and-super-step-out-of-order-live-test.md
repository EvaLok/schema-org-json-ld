# Cycle 163 — Track 1: v2-state-dispatch-archive design scope + Track 2: Missing Integration Scenario 3 super-step-out-of-order live test

**Authoring cycle:** 163 (2026-05-17).
**Mode:** redesign (phase-3 prototype + integration tests).
**Forward priorities source:** [`cycle-162-state-audit-session-start-wiring-and-dispatch-brief-addendum.md`](cycle-162-state-audit-session-start-wiring-and-dispatch-brief-addendum.md#forward-priorities-for-cycle-163).
**Predecessors:** cycle 161 [`v2-state-dispatch-policy-enforcement.md`](v2-state-dispatch-policy-enforcement.md) §4.2 Option B + §4.3 step 2 (Track 1); cycle 160 [`cycle-160-reconciler-completeness-metadata-and-c10-amendment.md`](cycle-160-reconciler-completeness-metadata-and-c10-amendment.md) §"Track 2 — C10 (L2.5) design-scope amendment" (Track 2).

## Cycle shape

Two-track composition continues. Cycle 162 was straight-pair NOVEL@1; cycle 163 is **straight-pair RECURRENCE-AT-2** — 2 unrelated priorities from the renumbered list, no coupling reason beyond both being in the priority list. The two-track composition HARDENING extends to **HARDENING-AT-16** (12 consecutive post cycle 151 single-track exception).

Track 1 closes a NEW priority from cycle 161 §6 (renumbered to #1 in cycle 163+). Track 2 closes a long-deferred AGREE-DEFER priority from cycle 155 absorption (renumbered to #14), unblocked by cycle 160 Track 2 C10 amendment 3 cycles ago.

## Track 1 — v2-state-dispatch-archive design scope

**Closes cycle 162 forward priority #1** (was cycle 161 priority #3; design scope for the archival counterpart to v2-state-audit).

Authored as canonical file [`docs/redesign/_notes/v2-state-dispatch-archive.md`](v2-state-dispatch-archive.md) (new, 260 lines). Single direct-push commit [`596d6655`](https://github.com/EvaLok/schema-org-json-ld/commit/596d6655).

Captured the following architecture decisions:

1. **Single-purpose discipline** (§2): the tool moves terminal entries from `agent_sessions[]` to dated archive files. Does ONE thing. Rejects coupling with v2-state-audit (§9.1) and rejects expanding v2-state-dispatch-sync scope (cycle 161 §4.2 Option A).

2. **Status whitelist + age threshold** (§3.1, §3.2): 5 terminal statuses are archival-eligible (`merged` / `failed` / `closed_without_pr` / `closed` / `closed_without_merge`); 2 live statuses are never archived (`in_flight` / `reviewed_awaiting_eva`). Age cutoff defaults to 30 days; `--age-days` flag tunes it.

3. **Missing-timestamp handling** (§3.3): cycle 163 measurement found 3 of 819 merged entries have `merged_at: null`. Tool falls back to `dispatched_at`; if both are absent/unparseable, the entry is NOT archived (silently archiving an un-age-verifiable entry would trade audit-trail clarity for sweep completeness — the wrong trade for a forensic-record tool).

4. **Archive file schema** (§4): JSON with `archive_version: 1`, byte-for-byte entry preservation, dated filename (invocation date, not entry date), append-on-same-day. Versioned schema makes future migration safe.

5. **Atomicity via tempfile + rename + hash-check** (§6): archive file written BEFORE state mutation (re-runs are idempotent on failure between steps 4 and 5). Hash check at step 5 detects concurrent mutation; aborts with exit 3 preserving the archive.

6. **Lock file for v2-tool collisions; hash check for v1 record-dispatch collisions** (§6.3): v1 is frozen-zone so it doesn't honor the lock; hash check is the natural fallback. v2 tools (v2-state-dispatch-sync) acquire the lock.

7. **Ordering vs other v2-state-* tools** (§7): v2-state-audit at session-start reports; archive runs separately (operator-driven during redesign; cron-triggered post-cutover). v2-state-dispatch-sync transitions first, archive afterward.

Forward priorities produced (§8):
- Priority #1 (cycle 164+): v2-state-dispatch-archive implementation (~1000 LOC + tests, v2-state-audit shape).
- Priority #2: backlog archival run (one-time invocation against 819-merged).
- Priority #3: v2-state-retention-policy.md §4 Axis 6 recalibration patch (completes cycle 159 storage-key-clarification "forward work" stub).

Explicit non-doings (§10): 10 items, including "does NOT build the tool this cycle," "does NOT modify v1 record-dispatch," "does NOT pre-design the automated trigger surface."

## Track 2 — Missing Integration Scenario 3 super-step-out-of-order live test

**Closes cycle 162 forward priority #14** (was cycle 161 priority #16, was cycle 160 priority #14 newly unblocked, was cycle 155 absorption AGREE-DEFER pending C10 amendment).

Implementation: extends `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs` with one new test `live_run_halts_super_step_out_of_order_when_history_diverges` (+139 LOC). Modifies `tools/rust/crates/v2-super-step-boundary/src/main.rs` BoundaryError::OutOfOrderCycleStart Display to prepend `"super-step out of order: "` to its message (+2 -2 LOC net).

Single direct-push commit [`0df5b8cf`](https://github.com/EvaLok/schema-org-json-ld/commit/0df5b8cf).

Test asserts the dual cycle 160 C10 amendment invariants:

1. **State-mutation-PRESENT for runner-local observability state**: `state/v2-cycle-runner/last-cycle.json` exists post-halt with `status=out-of-order`, `halt_step=super-step-init`, `halt_reason=super-step-out-of-order`, `steps_attempted=1`.

2. **State-mutation-ABSENT for super-step state machine state**: `state/super-step-history.json` and `state/super-step.json` are byte-identical pre/post run. The boundary refused the cycle-start without mutating super-step state.

Test setup: pre-populate `super-step-history.json` with a synthetic cycle-5 completion record (leaves `super-step.json` as the empty sentinel from init), then run `v2-cycle-runner run --cycle 1`. v2-super-step-boundary's cycle-start step refuses with `OutOfOrderCycleStart` (cycles must be sequential: last completed 5, requested 1, expected 6).

### Implementation discovery: classifier-mismatch in pre-existing code

**The boundary error text did not classify as SuperStepOutOfOrder.** v2-cycle-runner's `classify_failure` requires stderr to contain "super-step" PLUS one of "out of order" / "ordering" / "wrong super-step" / "not at expected". v2-super-step-boundary's `OutOfOrderCycleStart` Display message was `"cycles must be sequential: last completed cycle was N, requested cycle M (expected K)"` — NO "super-step" prefix, so classify_failure fell through to `FailureClass::Transient`.

Without that match, the test could not assert `halt_reason=super-step-out-of-order` — the runner would have classified the failure as transient and retried once before halting as `halted` (not `out-of-order`).

The fix is minimal: prepend `"super-step out of order: "` to both arms of `OutOfOrderCycleStart` Display (both the `Some(n)` and `None` last_complete cases). The boundary's own error message self-identifies as the SuperStepOutOfOrder class — semantically honest because OutOfOrderCycleStart IS the only out-of-order error path in the boundary.

No existing test depended on the pre-change phrasing. Verified via grep: only the variant definition, comment, and Display arms reference the old text.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 design scope `v2-state-dispatch-archive.md` | 260 lines (new) | `596d6655` |
| Track 2 code change `v2-super-step-boundary/src/main.rs` | +2 -2 LOC net | `0df5b8cf` |
| Track 2 integration test `integration_cycle.rs` | +139 LOC (1 new test) | `0df5b8cf` |
| `cycle-163-state-dispatch-archive-design-scope-and-super-step-out-of-order-live-test.md` (_notes) | ~310 lines | cycle-close |
| Total cycle 163 textual + code output | ~712 lines | 3 commits |

Cycle 163 datapoints for `magnitude-prediction-precision-is-shape-dependent-not-flat`:
- Track 1 design-scope-textual: 260 lines (target was 200-250; +10 lines above upper bound, ~4% overshoot). The cycle 158 v2-state-retention-policy.md was 244 lines and cycle 161 v2-state-dispatch-policy-enforcement.md was ~197 lines. Cycle 163 is the largest design-scope-textual in this arc, driven by the additional content on atomicity (§6) and ordering vs other tools (§7) — concerns the predecessor scopes did not need to address at this depth.
- Track 2 code-with-test: 139 + 2 = 141 LOC. Substantially smaller than cycle 162 Track 1 (+338 LOC + 12 tests + 1 integ) — as expected for a "<1 cycle if paired" priority. Track 2 makes a tiny code change (text prefix) + adds a single integration test rather than restructuring code.

## Pattern updates this cycle

- **`boundary-error-text-classifier-mismatch-found-via-test-implementation`** NOVEL@1 cycle 163. Implementing the live integration test surfaced a runtime-classification gap that existed silently because nothing previously exercised the super-step-out-of-order path live. The boundary's stderr text did not match the runner's classify_failure keywords. Test-driven discovery of a quiet pre-existing bug. Recurrence test: any cycle where authoring a live integration test surfaces a runtime mismatch between two primitives that have unit tests but no end-to-end coverage.
- **`agree-defer-priority-becomes-live-after-blocker-clears`** NOVEL@1 cycle 163. Cycle 155 absorption found AGREE-DEFER on Missing Integration Scenario 3 pending C10 amendment. Cycle 160 cleared the blocker. Cycle 163 lands the implementation 3 cycles after the blocker cleared (not in the cycle the blocker cleared). Priority-lifecycle pattern: AGREE-DEFER → blocker-cleared → still-deferred-for-N-cycles → AGREE-ACT-NOW. Recurrence test: any AGREE-DEFER finding whose blocker has cleared but whose implementation lands more than 1 cycle later.
- **`straight-pair-closure-as-default-two-track-shape` RECURRENCE-AT-2** cycle 163 (162, 163). Cycle 162 named the shape NOVEL@1; cycle 163 RECURRES it.
- **`two-track-composition` HARDENING-AT-16** cycle 163. 12 consecutive post cycle 151 exception.
- **`agree-act-now-bounded-fix-via-tracksecond-pattern` NOT-EXERCISED** cycle 163 (Track 2 closes an AGREE-DEFER priority, not an AGREE-ACT-NOW residue). Carries forward at RECURRENCE-AT-7 from cycle 162.
- **`tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED** cycle 163 — Track 1 builds design scope, doesn't wire tools; Track 2 surfaces a different class (live-test-surfaces-runtime-classifier-gap, NOVEL@1 above). Carries forward at RECURRENCE-AT-6.
- **`pre-flight-vs-mid-cycle-halt-distinction` RECURRENCE-AT-2** cycle 163 (162, 163). Cycle 162 named the catalog-axis with state-bound-exceeded (pre-flight); cycle 163 Track 2 exercises super-step-out-of-order (mid-cycle, step 1) — different axis of the same catalog. The dual-position recurrence validates the structural distinction.
- **`fail-open-on-tool-internal-anomalies-but-fail-closed-on-policy-violations` NOT-EXERCISED** cycle 163. Carries forward at NOVEL@1 from cycle 162.

## Process honoring

- **48th consecutive cycle of HONORING named forward priority** (cycles 115-163). Cycle 162 named #1 (archive design scope, Track 1) + #14 (super-step-out-of-order live test, Track 2); cycle 163 closes both.
- **76th bottleneck-asynchronous cycle** (78-163).
- **53rd non-per-candidate-sharpening cycle** (111-163).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 163).
- Cycle 128 lesson preserved (.scratch/ via Write tool — 3 files this cycle: session-start, track1-msg, track2-msg).
- Cycle 133 clarification preserved: 3 cargo invocations cycle 163 (build + test + clippy on v2-super-step-boundary + v2-cycle-runner). All clean.
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `8285b7d3`).
- **Cycle 137 lessons re-validated cycle 163 17-cycle-running** (137 + 147-163). All `gh` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 163** — all cargo via `--manifest-path tools/rust/Cargo.toml`.
- Cycle 151 date-test-value lesson preserved (no new date tests).
- Cycle 152 disk-format-read-source lesson preserved (Track 1 design scope referenced live state.json measurement via jq — not by reading from any tool's serialized form).
- Cycle 153 multi-Edit-fallback lesson preserved (single Edit per file Track 2; not exercised Track 1 new file).
- Cycle 154 `gh api graphql` subprocess pattern preserved (not exercised cycle 163; no Copilot dispatch).
- Cycle 155 dispatch-return-detection lesson preserved.
- Cycle 156 anti-overstatement audit enumeration discipline preserved (cycle 163 _notes has explicit "What cycle 163 does NOT do" with 13 items).
- Cycle 157 audit-HEAD-check at session-start preserved.
- Cycle 158 / 159 / 160 / 161 / 162 closure pattern progression — straight-pair RECURRENCE-AT-2.
- Cycle 159 test-pattern-mirror lesson preserved (Track 2 integration test mirrors the cycle 162 `live_run_halts_with_state_bound_exceeded_when_audit_reports_hard` test shape; same TempDir setup, same primitive_bin_args, same exit-code + last-cycle.json + super-step-state assertions).
- Cycle 160 schema-duplication observation preserved (no new schema fields cycle 163).
- Cycle 161 paired-closure pattern preserved (acknowledged; cycle 163 structurally different — straight-pair).
- Cycle 162 state-audit session-start wiring preserved (integration test now exercises the pre-flight; passes against the cycle 162 wiring without change).
- Journal-immutability discipline preserved (cycle 163 is the first cycle of a new day so it creates `docs/journal/2026-05-17.md` from scratch; cycles 148-162 in `2026-05-16.md` NOT back-edited).
- **Two-track composition continued** — cycle 163 is 12th consecutive post cycle 151 exception (HARDENING-AT-16).
- **SECTION 6b list housekeeping NOT invoked cycle 163** — 5 currently-open issues (cycle issue + 4 standing input-from-eva); 0 open PRs; no closure candidates.

## In-session issues and recoveries

- **One in-session test-failure recovery cycle 163.** First test run reported `assertion failed: expected halt_class=super-step-out-of-order, got None`. Root cause: the JSON serialization of CycleReport uses `halt_reason` not `halt_class` as the field name (Rust struct field is `halt_class: Option<FailureClass>`; JSON output uses `halt_reason` via the `c.as_kebab()` conversion). The cycle 162 state-bound-exceeded test correctly asserts on `halt_reason`; my draft incorrectly used `halt_class`. Single-edit recovery: changed both the assertion field name and the diagnostic message. No code change needed in the runner — the failure was test-side only.
- All `git add` / `git commit` / `git push` operations clean cycle 163.
- All Edit / Write operations clean cycle 163.
- All `gh api` / `gh issue list` / `gh issue comment` operations clean cycle 163.

## Cycle 163 ARTIFACTS

- `docs/redesign/_notes/v2-state-dispatch-archive.md` — new (Track 1, 260 lines, commit `596d6655`).
- `tools/rust/crates/v2-super-step-boundary/src/main.rs` — modified (Track 2, +2 -2 LOC net, commit `0df5b8cf`).
- `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs` — modified (Track 2, +139 LOC, commit `0df5b8cf`).
- `docs/redesign/_notes/cycle-163-state-dispatch-archive-design-scope-and-super-step-out-of-order-live-test.md` — new (this file, ~310 lines, cycle-close).
- `docs/journal/2026-05-17.md` — new file (cycle 163 is first cycle of 2026-05-17, so `2026-05-16.md` is NOT back-edited).
- `.scratch/cycle163-*.{md,txt}` — ephemerals (session-start, Track 1 commit msg, Track 2 commit msg, session-end, issue-close).
- 2 Track-side direct-push commits: `596d6655` (Track 1) + `0df5b8cf` (Track 2).
- 1 cycle-close commit (this _notes + journal section + ephemerals).
- 0 issues closed cycle 163 (no candidates; cycle issue itself closes per existing convention).
- 0 dispatches cycle 163.
- 3 cargo invocations cycle 163 (build + test + clippy on v2-super-step-boundary + v2-cycle-runner). All clean.
- ~5 GitHub API operations cycle 163 (audit HEAD, input-from-eva list, open issues list + PR list, session-start comment post).
- 1 Edit on `tools/rust/crates/v2-super-step-boundary/src/main.rs` (Track 2 Display prefix).
- 1 Edit on `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs` (Track 2 test addition) + 1 test-fix Edit on same file.
- 1 Write of new file `docs/redesign/_notes/v2-state-dispatch-archive.md` (Track 1).
- 0 Edits on `tools/rust/crates/v2-cycle-runner/src/main.rs` (Track 2 needed no runner code change; the cycle 162 wiring was sufficient).
- 0 Edits on `tools/rust/crates/v2-state-audit/src/main.rs`.
- 0 Edits on `tools/rust/crates/v2-state-dispatch-sync/src/main.rs`.
- 0 Edits on `tools/rust/crates/v2-dispatch-status/src/main.rs`.
- 0 Edits on `docs/state.json`.
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.

## What cycle 163 does NOT do

1. Does NOT build v2-state-dispatch-archive (cycle 164+ priority #1; this cycle is design scope only).
2. Does NOT run a backlog archival (cycle 164+ priority #2; gated on build).
3. Does NOT patch v2-state-retention-policy.md §4 Axis 6 thresholds (cycle 165+ priority #3; gated on build + backlog run).
4. Does NOT modify v1 record-dispatch (frozen zone preserved).
5. Does NOT wire v2-state-dispatch-archive into v2-cycle-runner at session-start (Track 1 §10 #5: forward-future cycle).
6. Does NOT extend the test to assert on channel state files (Track 2 §3.1 invariant: super-step state files only; channel state mutation is downstream of step 1 which never executes).
7. Does NOT modify classify_failure (Track 2 fix lives at the boundary's Display, not at the runner's classifier — boundary's error message self-identifies).
8. Does NOT add unit tests to v2-super-step-boundary (the +2 -2 Display change is exercised by the new integration test; no boundary-side test depended on the old phrasing).
9. Does NOT exercise the `BoundaryError::OutOfOrderCycleStart { last_complete: None, ... }` arm in the test (the test sets up `last_complete: Some(5)`; the None arm is exercised by existing boundary unit tests).
10. Does NOT close cycle 161 priority #5 (v2-channel-router enforcement extension design scope) or #6 (v2-prompt-tag-semantic-fidelity tool design scope) — these remain as cycle 164+ priorities.
11. Does NOT advance Phase 2 candidate selection (`2-selection.md` untouched cycle 163; per cycle 120 L2).
12. Does NOT extend the halt-class catalog (cycle 162 extended to 5 entries; cycle 163 EXERCISES classes 4 + 5 but adds no new class).
13. Does NOT engage audit-repo cross-repo critique (audit HEAD unchanged at `8285b7d3` since 2026-05-16; no audit-engagement single-track variant exercised cycle 163).

## Forward priorities for cycle 164+

Inheriting from cycle 162's renumbered list, minus #1 + #14 (closed cycle 163), plus new sub-priorities from Track 1.

1. **v2-state-dispatch-archive implementation** (was cycle 163+ priority #2 from §8 above): build the crate per the cycle 163 design scope. ~1000 LOC + tests (v2-state-audit shape, cycle 159 precedent). One cycle for skeleton + happy-path tests, possibly one more cycle for atomicity edge cases (§6.2 hash-check, §6.3 lock-file).
2. **Backlog archival run** (was cycle 163+ priority #3): one-time invocation of v2-state-dispatch-archive against the live 819-merged backlog. Gated on priority #1.
3. **v2-state-retention-policy.md §4 Axis 6 recalibration patch** (was cycle 163+ priority #4): apply cycle 161 §4.1 live+total split thresholds once priorities #1-2 land. Remove cycle 159 "forward work" stub.
4. **TOOL-SCOPED `v2-channel-router` enforcement extension design scope** (was cycle 162 priority #5). Carries forward.
5. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (was cycle 162 priority #6). Carries forward.
6. **`status` + `verify` v2-cycle-runner subcommands** (was cycle 162 priority #7). Carries forward.
7. **AGREE-DEFER queue (post-real-role-session-measurement)** (was cycle 162 priority #8). Hold for live-claude-code-spawn evidence (still SCAFFOLD).
8. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 162 priority #9).
9. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 162 priority #10).
10. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 162 priority #11).
11. **Audit-engagement substantive-focal single-track variant** (was cycle 162 priority #12). Gate = audit HEAD changes.
12. **Per-axis archival mechanism design scope** (was cycle 162 priority #13). Gate-bounded; cycle 163 design scope §10 #7 named this explicitly (archival for axes 1-5 is per-primitive forward work; this cycle is Axis 6-specific).

**Cycle 163 forward priorities CLOSED:**
- Cycle 162 priority #1 (v2-state-dispatch-archive design scope, Track 1).
- Cycle 162 priority #14 (Missing Integration Scenario 3 super-step-out-of-order live test, Track 2).

**Cycle 163 new sub-priorities:** 0 net. Track 1 §8 produced 3 forward priorities, but priorities #1-3 above already inherit them from cycle 162 priority #2-4 (the cycle 161 §4.3 step-2-3-5 sequence). The cycle 162 inheritance and the cycle 163 Track 1 §8 are the same priorities re-stated; no double-counting.
