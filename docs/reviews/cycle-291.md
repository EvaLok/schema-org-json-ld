# Cycle 291 Review

## 1. [worklog-accuracy] The cycle declared all 9 Phase 2 items complete even though Batch 3's fixture-realism defect remains unresolved

**File**: docs/worklog/2026-03-17/165151-cycle-291-phase-2-complete.md:8-9
**Evidence**: The worklog says PR `#1414` completed items 3, 4, and 9 and therefore `All 9 Phase 2 items now complete`. But the merged Batch 3 code that supposedly finished item 2 still hard-codes impossible 16-character "full" SHAs in `tools/rust/crates/cycle-receipts/src/main.rs:451-482`, even though the production parser reads real full commit SHAs and derives the short SHA from them. Cycle 290's merged forensic review had already documented that exact realism gap in `docs/reviews/cycle-290.md:3-7`, and no cycle 291 merge changed those fixtures. So the cycle closed Eva directive `#1401`'s implementation scope as if item 2 were genuinely done while the previously identified defect remained in the tree.
**Recommendation**: Do not treat Phase 2 as fully complete until the remaining fixture-realism gap from item 2 is either fixed in `cycle-receipts` tests or explicitly accepted as an intentional residual risk in the directive/worklog.

## 2. [worklog-accuracy] The published worklog says no issues were processed even though cycle 291 closed two agent-task issues

**File**: docs/worklog/2026-03-17/165151-cycle-291-phase-2-complete.md:19-21
**Evidence**: The worklog's dedicated `Issues processed` block says `None.` Yet the committed state for the same cycle records issue `#1413` merged via PR `#1414` and issue `#1418` merged via PR `#1419` at `docs/state.json:3953-3988`, and GitHub issue metadata shows both issues closed during cycle 291 (`#1413` closed at `2026-03-17T16:48:40Z`, `#1418` closed at `2026-03-17T16:25:40Z`). This contradicts the merged `write-entry` auto-issues behavior itself: `tools/rust/crates/write-entry/src/main.rs:657-663,759-814` would derive active-cycle agent-session issues into the worklog when `--auto-issues` is used. The cycle therefore published the exact `Issues processed: None.` defect that Phase 2 items 4 and 6 were supposed to eliminate.
**Recommendation**: Generate the worklog with `--auto-issues` (or fail the close-out flow when the derived issue list is non-empty but the rendered section says `None.`) so the published artifact cannot discard merged issue activity.

## 3. [worklog-accuracy] The `Current state` block shipped stale pipeline and metrics data instead of the committed cycle 291 snapshot

**File**: docs/worklog/2026-03-17/165151-cycle-291-phase-2-complete.md:29-33
**Evidence**: The published worklog says `PASS (2 warnings: field-inventory staleness refreshed, step-comments cascade from 290)` and `440 dispatches, 433 PRs produced, 429 merged, 99.1% PR merge rate`. But the state snapshot committed with the docs artifact records `produced_pr = 434` and `pr_merge_rate = 98.8%` at `docs/state.json:4188-4196`, not `433` / `99.1%`. The warning count is stale too: the cycle had already landed `706f2cf state(field-inventory): refresh 19 after-change fields verified at cycle 291 [cycle 291]`, and running `bash tools/pipeline-check` on the committed repository returns `Overall: PASS (1 warning)` with only the step-comments cascade warning. So the cycle published a `Current state` block that did not match the state and pipeline outputs it was claiming to summarize.
**Recommendation**: Derive the `Current state` block from the exact committed `docs/state.json` snapshot and a fresh final-gate `pipeline-check` result instead of copying earlier intermediate values into the worklog.

## 4. [state-integrity] `review_agent.history` still says finding F6 was "commit pending" even though the same state snapshot already contains the committed repair

**File**: docs/state.json:6844-6859
**Evidence**: The latest `review_agent.history` entry says cycle 290 finding F6 was actioned as `cleared stale completed_at, commit pending`. But the same committed state already has `cycle_phase` with no stale `completed_at` and `review_events_verified_through_cycle = 290` at `docs/state.json:4198-4202,6854-6859`, and the action itself was committed earlier in cycle 291 as `a9999b3 state(review-processed): cycle 290 review actioned — cleared stale completed_at, advanced review_events to 290 [cycle 291]`. The review ledger therefore records the action as still pending after the commit has already landed, which makes the canonical history internally contradictory.
**Recommendation**: Write review-history notes from the post-commit state, or scrub "pending" language once the corresponding repair commit is included in the same state snapshot.

## 5. [process-integrity] The local `--ours` merge of PR #1414 did drop a real state.json structural change before a later refresh restored it

**File**: docs/journal/2026-03-17.md:239-241
**Evidence**: The journal says PR `#1414` was merged locally with `--ours` for `docs/state.json` because the PR's docs snapshot was stale. That conflict handling was not harmless bookkeeping: the PR head after conflict resolution (`git show 8ab4313:docs/state.json | jq '.field_inventory.fields.cycle_phase'`) included the new `notes` text documenting `completed_at`, but the local merge commit that landed in cycle 291 (`git show 2996b09:docs/state.json | jq '.field_inventory.fields.cycle_phase'`) had already dropped that `notes` field and kept only `cadence` / `last_refreshed`. A later field-inventory refresh reintroduced the note before publication, so the final state is fine, but the merge path demonstrably did lose one of the PR's intended state changes in transit.
**Recommendation**: When locally merging `docs/state.json`, diff the merged blob against the PR head for targeted structural entries instead of blanket-accepting `--ours`, and document any intentionally discarded state changes explicitly in the cycle artifact.

## Complacency score

**4/5** — Cycle 291 finished important work: it merged Batch 2, actioned two prior review findings, and left the final receipt table in a passing state. But the close-out artifacts still show a "good enough" closure pattern. The worklog overstated Phase 2 completion despite a previously documented open defect, erased two issues that were actually processed, published stale pipeline/metric values after the refreshes had already landed, and the canonical review history still describes one actioned finding as pending. The local-merge narrative also understates that `--ours` really did drop part of the PR's state change before another commit repaired it. That is too much self-reporting drift for a cycle that was explicitly about stabilization quality.
