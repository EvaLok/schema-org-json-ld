# Cycle 456 Review

## 1. [test-gap] The same-cycle close-out hotfix shipped without regression tests for the new state-driven branches

**File**: `tools/rust/crates/cycle-runner/src/close_out.rs:141-177,2285-2348`; `tools/rust/crates/pipeline-check/src/main.rs:2219-2238,5826-5994`
**Evidence**:
- Direct push `2517af4b` changed `detect_prior_gate_failures` to read `/tool_pipeline/c5_5_initial_result` from `docs/state.json` and added a new `FAIL→PASS` allowance in `worklog_immutability_status_for_date`.
- The `close_out.rs` tests in the same file still cover only comment parsing (`detect_prior_gate_failures_parses_c4_1_fail_step_comments`, `...c5_5...`, and the empty case). They do not exercise the new state-based source-of-truth path or the `cycle != current_cycle` guard that now gates whether a prior failure is published.
- The `pipeline-check` tests around `worklog_immutability` cover resume annotations, bare status changes, exact matches, and post-cycle edits, but there is no test for the new `FAIL→PASS (... initially failed ...)` exception.
- `cargo test -p cycle-runner` and `cargo test -p pipeline-check` both pass, which confirms the old suites still run, but it also shows the hotfix was accepted without any new regression covering the just-added branches.
**Recommendation**: Add focused tests for the new state-based C5.5 failure detection, for rejecting a previous cycle’s `c5_5_initial_result`, and for the new `FAIL→PASS` immutability exception before treating this hotfix as the durable fix path.

## 2. [worklog-accuracy] The published worklog under-reports the local unblock that rewrote the artifact itself

**File**: `docs/worklog/2026-04-07/215638-cycle-456-review-f1-f2-f4-dispatched-as-2274-field-inventory-refresh-pipeline-check-immutability-fix.md:8-29`
**Evidence**:
- The worklog says the only local tool fix was `pipeline-check::worklog_immutability_status_for_date` and its self-modification section lists only `tools/rust/crates/pipeline-check/src/main.rs`.
- But the direct push immediately before the docs refresh, `2517af4b`, modified **both** `tools/rust/crates/cycle-runner/src/close_out.rs` and `tools/rust/crates/pipeline-check/src/main.rs`.
- The follow-up docs commit `46c19e44` then rewrote this same worklog from `## Pre-dispatch state` / `FAIL (3 warnings, 2 blocking: ...)` to `## Cycle state` / `FAIL→PASS (...)` plus `Close-out gate failures`.
- So the final artifact depends on an unreported same-cycle `cycle-runner` hotfix that changed the close-out rendering, not just on the narrower `pipeline-check` tweak the worklog describes.
**Recommendation**: When a post-`cycle-complete` unblock changes close-out code and then republishes the cycle artifact, list every modified infrastructure file and explicitly note that the published worklog was regenerated off that hotfix.

## 3. [journal-quality] The journal still overstates progress while claiming strict per-criterion grading

**File**: `docs/journal/2026-04-07.md:169-188`
**Evidence**:
- The carry-forward commitment at line 169 defines four observable checks `(a)` through `(d)`, including `(c) no second commit modifies the worklog after C5 commits` and `(d) grep -RF fixup_latest_worklog_in_flight ... returns matches only in record-dispatch and dispatch-task binary call sites`.
- The follow-through at line 171 discusses only part of `(a)` plus a separate C4.1 observation; it never grades `(c)` or `(d)` even though the entry claims this cycle demonstrates “strict per-criterion grading.”
- Git history shows `46c19e44 docs(cycle-456): worklog, journal, and state updates [cycle 456]` modified the cycle 456 worklog after the close-out sequence, so criterion `(c)` was not met and should have been recorded explicitly.
- Line 175 says the cycle “Cleared the chronic field-inventory WARN,” but `bash tools/pipeline-check --cycle 456 --json` still reports `field-inventory` as `warn`, and `docs/state.json:7752-7755` still shows `step_comment_acknowledged_gaps` at `last_refreshed: cycle 445`.
**Recommendation**: Grade each stated criterion individually, record failed criteria instead of collapsing them into a summary label, and describe field-inventory as “reduced to one remaining warning” until the tool output actually turns PASS.

## Complacency score

**2/5** — The cycle did real work: receipts resolve, step coverage on `#2273` is complete, and the repository validations pass. But the cycle still had to hotfix its own close-out after a blocking gate failure, shipped that hotfix without branch-specific regression coverage, under-reported the infrastructure changes that rewrote the published worklog, and then claimed stricter journal discipline than the artifact actually shows. That is movement, but not trustworthy closure.
