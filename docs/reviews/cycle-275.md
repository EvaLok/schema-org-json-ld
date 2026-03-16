# Cycle 275 Review

## 1. [stale-metrics] The published current-state block was already stale when the worklog was committed

**File**: docs/worklog/2026-03-16/050548-cycle-275-3-merges-2-dispatches-review-processed-audit-accepted.md:34
**Evidence**: The worklog says cycle 275 closed with `414 dispatches, 406 PRs produced, 403 merged`. But `a6d9c01` ran before the docs commit `1276f61` and updated `docs/state.json` to `414 dispatches, 407 produced_pr, 404 merged, 412 resolved` (`docs/state.json:3941-3950`). `git log --reverse --since='2026-03-16T04:50:00Z' --until='2026-03-16T05:12:00Z'` shows `a6d9c01` before `1276f61`, so the worklog did not reflect the repository state it was supposedly summarizing.
**Recommendation**: Stop hand-copying the current-state metrics into the worklog. Generate that block directly from the post-fix `docs/state.json`, or fail docs generation if state changed after the last metric snapshot but before the worklog commit.

## 2. [receipt-integrity] The final receipt table remained broken even after the receipt-fix edits

**File**: docs/worklog/2026-03-16/050548-cycle-275-3-merges-2-dispatches-review-processed-audit-accepted.md:45
**Evidence**: First, the note says the `receipt-fix commit` and `record-dispatch commit` are structurally excluded, but the table immediately includes both record-dispatch receipts (`55a0dae`, `18a147d`) at lines 56-57. Second, `bash tools/receipt-validate --cycle 275 --worklog docs/worklog/2026-03-16/050548-cycle-275-3-merges-2-dispatches-review-processed-audit-accepted.md` fails on the published artifact with two genuinely missing receipts: `e4f3c46` and `8bc2e54`, the two commits that patched this same receipt section after the docs commit. Third, the worklog still certifies “Validated by receipt-validate at step C5.1” even though the current artifact does not pass that validator.
**Recommendation**: Re-run `receipt-validate` against the final worklog after every manual receipt edit, and generate the scope note from the actual included/excluded commit classes instead of hand-editing prose that can drift out of sync with the table.

## 3. [state-integrity] Cycle-complete shipped a stale review session and wrong derived metrics, then needed two repair commits to catch up

**File**: docs/state.json:3703
**Evidence**: The timeline is backwards for a supposedly closed-out state snapshot. GitHub issue `#1328` was already closed at `2026-03-16T03:03:41Z`, and the final state now shows it as `status: "merged"` with `pr: 1329` and `merged_at: "2026-03-16T03:03:38Z"` (`docs/state.json:3703-3708`). But `git diff 8d810b1 ca2b403 -- docs/state.json` shows cycle-complete had left that same session `in_flight` and counted `copilot_metrics.in_flight` as `3`. The next diff, `git diff ca2b403 a6d9c01 -- docs/state.json`, then had to repair the derived metrics from `403 merged / 406 produced / 411 resolved` to `404 / 407 / 412`.
**Recommendation**: Add a pre-close reconciliation that cross-checks merged review issues/PRs against `agent_sessions` and recomputes derived metrics before `cycle-complete` is allowed to pass. A cycle should not need two post-close state repairs to become accurate.

## 4. [journal-quality] The journal calls the cycle “clean” while omitting the post-close repairs that prove it was not

**File**: docs/journal/2026-03-16.md:92
**Evidence**: The cycle 275 journal frames the cycle as a “clean merge cycle with review accountability” and highlights that all three PRs were “one-and-done” and “ideal” (`docs/journal/2026-03-16.md:92-114`). But the same cycle needed `ca2b403` to fix a stale `in_flight` review session, `a6d9c01` to reconcile derived metrics, and then `e4f3c46`/`8bc2e54` to repair the worklog receipt section after publication. The reflection celebrates review smoothness, while skipping the operational failures that actually consumed the tail of the cycle.
**Recommendation**: Treat any post-`cycle-complete` repair commit as mandatory journal material. If a cycle needs state or artifact corrections after close-out, the reflection should name that breakage directly instead of presenting the cycle as clean.

## 5. [process-adherence] PR #1334 fixed `process-merge`, but the repository’s canonical instructions still teach the old broken invocation

**File**: COMPLETION_CHECKLIST.md:31
**Evidence**: PR #1334 added the optional `--merged-at` flag so `process-merge` can record the real GitHub merge timestamp (`tools/rust/crates/process-merge/src/main.rs:27-29,68-82,111-121`). But the checklist row that documents the canonical merge workflow still tells operators to run `bash tools/process-merge --prs 123,456 --issues 789,790` with no `--merged-at` at all. A repository search for `\-\-merged-at` only finds the flag in `tools/rust/crates/process-merge/src/main.rs` and its tests, not in workflow documentation such as `COMPLETION_CHECKLIST.md` or `STARTUP_CHECKLIST.md`. The fix exists in code, but the durable operating instructions still normalize the timestamp behavior that caused the review finding.
**Recommendation**: Update the checklist and any orchestrator instructions to require `--merged-at` with the actual GitHub `merged_at` value, or make `process-merge` fetch that timestamp itself so the safe path is the default rather than an undocumented optional extra.

## Complacency score

**2/5** — Cycle 275 did real maintenance work: it merged the review artifact, fixed `pipeline-check`’s denominator bug, accepted audit #262, and landed the `process-merge` timestamp flag. But the cycle still published stale current-state metrics, closed out with a wrong `agent_sessions`/metrics snapshot that needed two repair commits, and ended with a worklog receipt table that still does not match canonical receipt scope. That is not a disciplined close-out; it is a cycle that kept repairing its own bookkeeping after declaring itself clean.
