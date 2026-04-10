# Cycle 473 Review

## 1. [code-change-quality] PR #2385 shipped `--auto-review-summary` with lookup logic that depends on missing state fields

**File**: tools/rust/crates/write-entry/src/main.rs:1352-1383, tools/rust/crates/write-entry/src/main.rs:5423-5523, docs/journal/2026-04-10.md:176-180
**Evidence**: Lines 1352-1383 implement the new lookup and only succeed when a `review_agent.history` entry carries top-level `issue` or `review_issue` fields matching `previous_cycle_issue`. Lines 5423-5523 test only hand-written fixtures that fabricate exactly those fields, but the real cycle 470-472 history entries in `docs/state.json` do not carry them. When run against cycle 473 state, `bash tools/write-entry worklog --repo-root . --title 'Cycle 473 probe' --auto-review-summary --dry-run --pipeline 'PASS' --publish-gate 'published'` fails immediately with `Error: review_agent.history has no entry matching previous_cycle_issue 2381`. The cycle 473 journal separately confirms the same first-runtime failure and names the missing issue linkage as the cause.
**Recommendation**: Make `process-review` persist the review issue on history entries or derive the prior review entry without relying on nonexistent fields, and add an integration test that exercises `--auto-review-summary` against the real `docs/state.json` shape instead of a hand-written fixture.

## 2. [journal-quality] The journal marked the cycle-start verification commitment MET even though its own runtime observable failed

**File**: docs/journal/2026-04-10.md:169-172, docs/state.json:8109-8111
**Evidence**: Commitment 1 for cycle 473 required two observables: `(a)` the `cycle-start` test passes and `(b)` `closed_this_cycle` is `[]` after cycle-start runs. The follow-through marks the commitment `MET`, but in the same sentence admits cycle-start ran before PR #2383 merged, so stale `#2340` still carried into this cycle. The state snapshot at `docs/state.json:8109-8111` still shows `eva_input_issues.closed_this_cycle: [2340]`. That means observable `(b)` was not met, so the commitment was not fully satisfied under the journal’s own criteria.
**Recommendation**: Score commitment follow-through observable-by-observable. If any promised runtime check is still false, mark the commitment partial/deferred instead of `MET`, and carry the unmet observable into the next cycle explicitly.

## 3. [state-integrity] The review ledger upgraded a dispatched fix to `actioned` before the fix had actually landed

**File**: docs/state.json:14366-14392, docs/worklog/2026-04-10/213352-cycle-473-review-processed-3-prs-merged-2-dispatches-3-deferrals-resolved.md:7-9, docs/state.json:7223-7234
**Evidence**: Cycle 472’s review-history entry records the `state-integrity` finding as `actioned` and says “dropped deferral state fixed.” But cycle 473’s own worklog shows the only action taken for that defect was dispatching `#2389`, and `docs/state.json` still lists `#2389` as `in_flight` rather than merged. Issue `#2389` is the structural fix request to start setting `resolved: true` on dropped deferrals, so the defect was still waiting on implementation at cycle close. The ledger therefore promoted a dispatch to a completed fix, which breaks the review history’s ability to distinguish findings that merely have follow-up work queued from findings that are actually fixed.
**Recommendation**: Keep review-history disposition at `dispatch_created` until the corrective PR merges and the deferred-finding row is demonstrably repaired in state, and add an invariant that rejects `actioned` review-history notes when the cited fix issue is still only `in_flight`.

## Complacency score

3/5 — cycle 473 did the mechanical review work: receipts reproduce cleanly, `state-invariants` and `metric-snapshot` pass, and issue `#2388` has the expected 26 step comments. But the cycle still repeated chronic truthfulness problems in higher-value artifacts: a merged feature failed on first real use because tests modeled nonexistent state, the journal called an unmet observable `MET`, and the review ledger inflated a newly dispatched fix into an `actioned` one. That is too much narrative optimism for a cycle that was explicitly trying to close chronic review-fidelity gaps.
