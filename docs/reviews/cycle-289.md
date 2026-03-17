# Cycle 289 Review

## 1. [receipt-integrity] Batch 1 fixed post-receipt scope only partway, so the generator and validator still disagree about docs receipts

**File**: tools/rust/crates/cycle-receipts/src/main.rs:12-13,108-127
**Evidence**:
- PR `#1406` added `POST_RECEIPT_COMMIT_PREFIXES` for only `state(clean-cycle` and `state(record-dispatch):`, and `collect_receipts()` now filters only those subjects from the canonical receipt stream.
- Running `bash tools/cycle-receipts --cycle 289 --repo-root .` on the committed repo still returns ten receipts, including `a48418e docs(cycle-289): worklog, journal entries [cycle 289]`.
- The rest of the cycle documentation describes a different scope: `docs/worklog/2026-03-17/085233-cycle-289-batch-1-merged-batch-2-redispatched-batch-3-dispatched.md:46-48` says docs and record-dispatch commits are structurally excluded, and `tools/rust/crates/receipt-validate/src/main.rs:181-183` still hard-codes `docs(cycle-...)` as structurally excluded during validation.
- Cycle 289 therefore still depends on a split rule set: the generator says the docs receipt exists, while the validator and published worklog say it does not count.
**Recommendation**: Make receipt scope consistent across `cycle-receipts`, `receipt-validate`, and the published worklog note. Either exclude docs commits in the generator too, or stop describing them as structurally excluded in the validator/worklog.

## 2. [worklog-accuracy] The worklog logs concrete issue activity and then publishes `Issues processed: None.`

**File**: docs/worklog/2026-03-17/085233-cycle-289-batch-1-merged-batch-2-redispatched-batch-3-dispatched.md:3-23
**Evidence**:
- The `What was done` section records explicit issue activity in the same artifact: it closed audit-inbound `#1403`, redispatched Batch 2 as `#1413`, and dispatched Batch 3 as `#1411` (lines 5-13).
- `docs/state.json:3952-3964` also records `#1413` and `#1411` as new in-flight agent sessions for cycle 289.
- GitHub issue metadata confirms that `#1403` was closed during this cycle (`closed_at: 2026-03-17T08:30:47Z`), so this was not just planned work — an issue was actually processed.
- Despite that, the dedicated `Issues processed` section still says `None.` (lines 20-22), repeating the exact manual-summary defect Phase 2 item 6 was supposed to eliminate.
**Recommendation**: Stop hand-writing the `Issues processed` block. Derive it from committed state and issue activity so the section cannot contradict the worklog body.

## 3. [journal-quality] The cycle 289 follow-through misreports commitment 2 and omits the action that actually satisfied it

**File**: docs/journal/2026-03-17.md:159-173
**Evidence**:
- The previous cycle's second commitment was explicit: `Dispatch Phase 2 Batch 3 (items 1 and 2) next cycle` (lines 161-163).
- The follow-through sentence then claims commitment 2 was satisfied because `Batch 2 redispatched due to conflicts` (line 165). That is a different action tied to a different batch.
- The cycle did in fact dispatch Batch 3: the worklog records `Dispatched Batch 3 #1411` at `docs/worklog/2026-03-17/085233-cycle-289-batch-1-merged-batch-2-redispatched-batch-3-dispatched.md:8-9`.
- So the journal did not merely use imprecise wording; it swapped the numbered commitment's real completion event for a separate remedial action and therefore broke the commitment-to-outcome chain it is supposed to document.
**Recommendation**: Map each numbered commitment to the exact observed result. If the cycle also did extra work such as a redispatch, record that separately instead of counting it as completion of a different commitment.

## 4. [state-integrity] Cycle-scoped state was marked refreshed in cycle 289 even though stale values were carried forward

**File**: docs/state.json:4184-4299
**Evidence**:
- `cycle_phase` now says cycle 290 is in `work`, but `completed_at` is still `2026-03-16T22:39:05Z`, older than the current cycle and older than `phase_entered_at` (`2026-03-17T14:25:19Z`) at lines 4184-4189.
- The stale timestamp was already carried through cycle start: commit `69a543f state(cycle-start)` changed the phase from `complete` to `work` without clearing `completed_at`, so cycle 289 refreshed the phase marker while preserving an out-of-phase completion timestamp.
- `eva_input_issues.closed_this_cycle` still contains `[1350]` at lines 4255-4257, but GitHub issue `#1350` was closed on `2026-03-16T12:33:26Z`, before cycle 289 began.
- The field inventory nevertheless marks `eva_input_issues.closed_this_cycle` as refreshed in cycle 289 at lines 4297-4299, even though the cycle-scoped list was not reset to this cycle's reality.
**Recommendation**: Treat cycle-scoped fields as data that must be reset, not just re-labeled. Close-out/state-invariant tooling should fail when `last_refreshed` advances for stale `closed_this_cycle` data or when `cycle_phase.completed_at` conflicts with the active phase.

## Complacency score

**3/5** — Cycle 289 did real work and closed with a PASS pipeline, so the override cap does not apply. But the close-out artifacts still show meaningful complacency: the receipt-scope fix remained split across tools, the worklog published `Issues processed: None.` despite clear issue activity, the journal misreported which commitment was actually fulfilled, and `state.json` advanced freshness markers over stale cycle-scoped values. That is more than incidental wording drift; it is a repeat pattern of documentation and state being treated as “close enough” after the operational work was done.
