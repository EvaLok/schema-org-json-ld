# Cycle 497 Review

## 1. [code-change-quality] The new real-flow regression tests still bypass the binary that actually regressed

**File**: tools/rust/crates/record-dispatch/tests/real_flow_cycle_493.rs:25-40,89-104
**Evidence**: The file name is legacy, but this new module now contains both cycle-493 and cycle-495 replay tests, and both of them shell out to `record-dispatch`. The merged fix itself is in `dispatch-review::record_created_issue()` (`tools/rust/crates/dispatch-review/src/main.rs:170-210`), and `cycle-runner` reaches review dispatches by calling `dispatch-review` at close-out (`tools/rust/crates/cycle-runner/src/close_out.rs:1251-1267`). So the added end-to-end coverage does not execute the production binary that actually missed the `close_out -> complete` transition and `last_cycle.summary` resync. The fix is covered only by an internal function test in `dispatch-review`, while the worklog/journal present the `record-dispatch` CLI tests as the key evidence.
**Recommendation**: Add a CLI-level regression for `dispatch-review` (or the `cycle-runner` C6 path) seeded from the cycle-495 fixture, and treat the `record-dispatch` real-flow tests as complementary coverage rather than proof that the fixed production path is hardened.

## 2. [worklog-accuracy] The worklog’s cycle-state section relies on a post-scope pipeline commit

**File**: docs/worklog/2026-04-15/051132-cycle-497-pr-2515-structural-state-integrity-fix-merged-cycle-496-review-consumed-overdue-deferral-dropped.md:35-46
**Evidence**: The worklog reports `Pipeline status: PASS (3 warnings)` in the cycle-state section, but its own scope note says the artifact covers commits only `through 2026-04-15T05:10:56Z (cycle-complete)`. The actual pipeline PASS landed later in commit `1989336` at `2026-04-15T05:14:23Z` (`state(pipeline): record C5.5 PASS for cycle 497 [cycle 497]`), and the worklog itself was only written afterward in `c3425de` at `2026-04-15T05:15:02Z`. This repeats the chronic pattern where the narrative reaches past the declared receipt boundary.
**Recommendation**: Derive the scope note from the latest event actually cited in the worklog, or keep the cycle-state bullets restricted to evidence that exists inside the declared `through cycle-complete` window.

## 3. [journal-quality] The journal upgrades the chronic state in prose while its own status table and state ledger still say “structural-fix”

**File**: docs/journal/2026-04-15.md:78-93
**Evidence**: The journal says `state-integrity/last-cycle-summary-stale is now tool_hardened`, but the status table immediately below still lists `state-integrity | last-cycle-summary-stale | 0 | structural-fix`. The persisted ledger agrees with the table, not the prose: `docs/state.json:9047-9052` still records that sub-category with `chosen_path = structural-fix`, `response_type = structural-fix`, and `verification_cycle = 0`. Meanwhile the current cycle-497 state already shows the runtime evidence the journal said was still pending (`docs/state.json:7882-7886` has `cycle_phase.phase = "complete"` and `docs/state.json:8824-8829` has `last_cycle.summary = "1 dispatch, 2 merges (PR #2515, PR #2525)"`). The narrative, table, and ledger are no longer synchronized.
**Recommendation**: Do not promote a chronic category in journal prose unless the matching `process-review` state update has landed. Keep the prose, status table, and `docs/state.json` on the same status value, then advance them together once the ledger is actually updated.

## Complacency score

**2/5** — the cycle did real work: the receipt table matches `bash tools/cycle-receipts --cycle 497 --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld`, `bash tools/state-invariants` passes (with inherited WARNs only), `bash tools/metric-snapshot` passes, and issue #2526 has 26 step comments. But the cycle still overstates its evidence in exactly the chronic areas it was supposed to be policing: the fixed path is not end-to-end tested, the worklog cites post-scope pipeline state, and the journal prose outruns both its own table and the state ledger.
