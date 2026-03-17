# Cycle 290 Review

## 1. [code-change-quality] Batch 3's cycle-receipts fixtures still are not production-realistic full SHAs

**File**: tools/rust/crates/cycle-receipts/src/main.rs:364-382,451-482,486-590
**Evidence**: `parse_git_log_line()` reads a full commit SHA and derives `short_sha` from the first 7 characters of that full value. But the merged Batch 3 tests still hard-code impossible "full" SHAs such as `abcdef1234567890` and `abc1234def567890` in the render and dedup fixtures. That means PR #1412 fixed the short-SHA/link mismatch, but it did not make the fixtures truly match production parser output as claimed in the PR body and worklog.
**Recommendation**: Replace the hand-written 16-character pseudo-hashes with real 40-character fixture SHAs, or better, build the render fixtures from `parse_git_log_line()` so the tests exercise the same full-SHA shape production uses.

## 2. [pr-review-quality] The cycle recorded Batch 2 as "revision requested" without a formal GitHub review object

**File**: docs/worklog/2026-03-17/144922-cycle-290-review-processed-batch-3-merged-batch-2-revision-requested.md:8-10
**Evidence**: The worklog says Batch 2 was reviewed and a revision was requested. The underlying GitHub PR metadata for `#1414` shows the bug report and rebase request were posted as plain PR comments, while `get_reviews` returned no review objects and `get_review_comments` returned no review threads. So the bug catch itself was real and the instructions were clear, but the repository does not contain the stronger `CHANGES_REQUESTED` audit trail the narrative implies.
**Recommendation**: When a PR must not merge until a defect is fixed, submit the feedback as a formal GitHub review in addition to, or instead of, issue-style comments so the review state is machine-visible and auditable.

## 3. [receipt-integrity] The published receipt table still omits canonical cycle 290 receipts while claiming validation passed

**File**: docs/worklog/2026-03-17/144922-cycle-290-review-processed-batch-3-merged-batch-2-revision-requested.md:39-50
**Evidence**: The worklog note says receipt scope was validated by `receipt-validate` and excludes only docs and record-dispatch commits. But `bash tools/cycle-receipts --cycle 290 --repo-root .` now returns 9 receipts, including `234cd44 fix(worklog): add missing process-merge receipt 4e634e2 [cycle 290]` and `c47613b state(derive-metrics): reconcile copilot_metrics after #1405 status fix [cycle 290]`. `bash tools/receipt-validate --cycle 290 --worklog ...` then fails with `Genuinely missing: 2` for exactly those two commits. Neither commit is a docs or record-dispatch receipt, so the worklog's published scope note and receipt table are still incomplete at cycle end.
**Recommendation**: Regenerate or amend the worklog receipt section after any additional tool-generated commits land, and do not leave the `Validated by receipt-validate` note in place unless the final published artifact still passes validation.

## 4. [worklog-accuracy] The "Current state" block was published against unreconciled metrics and not the state snapshot it shipped with

**File**: docs/worklog/2026-03-17/144922-cycle-290-review-processed-batch-3-merged-batch-2-revision-requested.md:27-32
**Evidence**: The worklog says the current state has `1` in-flight session and `439 dispatches, 427 merged`. But the docs commit that introduced this worklog (`64f3ab8`) still had `docs/state.json` with `copilot_metrics.in_flight = 2`, `closed_without_pr = 3`, and `resolved = 437`. One minute later, commit `c47613b state(derive-metrics): reconcile copilot_metrics after #1405 status fix [cycle 290]` changed those values to `in_flight = 1`, `closed_without_pr = 4`, and `resolved = 438`. So the published worklog was not actually sourced from the committed state it was released with.
**Recommendation**: Do not publish the worklog until any manual state edits have been followed by `derive-metrics`, or derive the Current state block directly from the exact `docs/state.json` blob being committed in the same receipt.

## 5. [worklog-accuracy] The next-steps section says Phase 2 items 1-2 remain even though Batch 3 had just merged them

**File**: docs/worklog/2026-03-17/144922-cycle-290-review-processed-batch-3-merged-batch-2-revision-requested.md:34-37
**Evidence**: The worklog's second next step says `Phase 2 items 1-2 remain after all Batch merges`. But Eva directive `#1401` defines items 1 and 2 as the Batch 3 work, issue `#1411` is explicitly titled `Phase 2 Batch 3: verify-review-events code-PR tests, cycle-receipts test realism`, and PR `#1412` merged that exact batch during cycle 290. Those items were the ones that had just been completed, not the ones still remaining.
**Recommendation**: Derive the remaining Phase 2 scope from the open Phase 2 issues/PRs after merges, or name the still-open items directly instead of referring to already-merged item numbers.

## 6. [state-integrity] `cycle_phase.completed_at` is still stale and contradictory after the cycle 1405 state fix

**File**: docs/state.json:3937-3942,4187-4190
**Evidence**: Cycle 290 manually marked issue `#1405` as `failed`, but the same state snapshot still shows `cycle_phase.phase = "close_out"` with `completed_at = "2026-03-16T22:39:05Z"`, which predates the current `phase_entered_at = "2026-03-17T14:48:53Z"`. Phase 2 item 9 and issue `#1405` were supposed to make `transition_cycle_phase()` clear `completed_at` on non-`complete` phases; instead, the cycle closed with a phase block that still mixes a stale completion timestamp into an active close-out phase.
**Recommendation**: Treat `completed_at` as a semantic invariant, not just a typed field. `state-invariants` or the close-out flow should fail whenever `completed_at` is present outside `complete`, especially after a cycle explicitly touched the state to reflect Batch 2's failure/redispatch path.

## 7. [state-integrity] `review_events_verified_through_cycle` was marked refreshed for cycle 290 even though the value was not advanced to the tool's own verified result

**File**: docs/state.json:4371-4373,6836-6837
**Evidence**: Field inventory marks `review_events_verified_through_cycle` as refreshed in cycle 290, but the actual value remains `289`. Running `bash tools/verify-review-events` on the committed repo reports both merged cycle 290 PRs (`#1412` tooling and `#1416` docs-only) as verified and ends with `Safe to advance marker to 290.` So cycle 290 refreshed the freshness marker without updating the underlying verified-through value to match the tool's result.
**Recommendation**: Make the close-out flow fail if `last_refreshed` advances for `review_events_verified_through_cycle` without either updating the value or recording an explicit reason why the verification tool did not advance it.

## 8. [journal-quality] The cycle 290 commitments are not fully auditable because commitment 2 has no observable completion condition

**File**: docs/journal/2026-03-17.md:213-216
**Evidence**: The journal ends with `Assess remaining Phase 2 items after all batches are merged`. That is an intent, not a concrete commitment with an observable done condition. It also inherits the worklog's already-false premise that items 1-2 still remain after Batch 3 merged them. Compared with the repository's stated goal of concrete follow-through, a reviewer next cycle cannot tell exactly what evidence would satisfy this commitment.
**Recommendation**: Phrase commitments as auditable outcomes tied to specific artifacts or issue states, such as naming the exact remaining Phase 2 items to confirm closed/open or the exact dispatch/merge decision expected next cycle.

## Complacency score

**4/5** — Cycle 290 did one important thing right: it caught a real bug in PR `#1414` before merge. But the close-out artifacts still show a strong "good enough" pattern. The worklog published a receipt table that now fails `receipt-validate`, a Current state block that did not match the state snapshot committed with it, a next-step statement that misidentified already-finished Phase 2 items as remaining work, and state freshness markers that were advanced without matching the actual verified values. That is more than wording drift; it shows the operational review was taken seriously, but the documentation/state closure was still treated as something that could be patched after publication.
