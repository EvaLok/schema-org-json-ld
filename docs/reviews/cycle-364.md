# Cycle 364 Review

## 1. [worklog-accuracy] The published cycle summary is still stale enough to fail the repository’s own validator against the final tree

**File**: docs/worklog/2026-03-26/002925-cycle-364-review-merge-journal-quality-fixes-dispatch.md:29-35
**Evidence**: The worklog’s “Pre-dispatch state” block says `In-flight agent sessions: 2` and `Copilot metrics: 572 dispatches, 513 merged`. The final committed state for cycle 364 says otherwise: `docs/state.json` records `in_flight_sessions: 3`, `copilot_metrics.in_flight: 3`, and `copilot_metrics.total_dispatches: 573` after review issue `#1783` was dispatched. Re-validating the published worklog against the final repository state fails with `in-flight agent sessions mismatch: worklog reports 2, state.json has 3`. The note on line 31 acknowledges a pre-dispatch snapshot, but the artifact is still published as the cycle summary and does not survive the repo’s own `validate-docs worklog` check once the cycle is actually complete.
**Recommendation**: Re-run worklog validation against the final committed state after review dispatch / phase completion, or regenerate the “Pre-dispatch state” block from final state before closing the cycle. If pre-dispatch snapshots are intentionally retained, the validator and artifact wording need to agree on that model instead of shipping a worklog that later fails validation.

## 2. [review-history-accuracy] Cycle 364’s authoritative review ledger under-records the journal-quality action it claims to have taken

**File**: docs/state.json:8985-9011
**Evidence**: The cycle 363 review history entry records `dispatch_created: 2`, but the per-finding dispositions only contain one `dispatch_created` item (`worklog-accuracy`) and still mark `journal-quality` as `deferred`. That does not match cycle 364’s own narrative. The worklog says `F3 dispatch_created ([#1779] + [#1781])` and the journal says the cycle dispatched twin fixes for the journal-quality finding: `#1779` (generation guard) and `#1781` (validation guard). The aggregate count therefore says two dispatches were created, while the per-finding ledger still leaves the relevant category as deferred. That makes chronic-category tracking less trustworthy precisely in an area the cycle claimed to be addressing.
**Recommendation**: Reconcile `review_agent.history.finding_dispositions` after same-cycle follow-up dispatches, or teach `process-review` / close-out tooling to record a disposition that matches the final cycle outcome. Aggregate counts and per-finding statuses should never disagree about whether a chronic finding was acted on.

## 3. [code-quality] The review-body generator filtered out same-cycle merges and seeded the review issue with false context

**File**: tools/rust/crates/cycle-runner/src/review_body.rs:210-232
**Evidence**: `get_merged_prs()` uses `last_cycle.timestamp` as the lower bound for “this cycle” merges and excludes any PR with `merged_at <= last_cycle_ts`. By the time the review body is generated for cycle 364, `last_cycle.timestamp` is already the current cycle’s close-out timestamp (`2026-03-26T00:34:20Z`), so the two real cycle-364 merges (`#1774`, `#1777`) are filtered out and the generated review issue body says `PRs merged: None` on issue `#1783`. That issue body is objectively false: the worklog lists both merged PRs, `docs/state.json` says `2 merges (PR #1774, PR #1777)`, and git history contains merge commits `ab2f664c` and `c8902ee0`.
**Recommendation**: Derive review-body merged PRs from cycle-tagged state for the just-completed cycle instead of comparing against `last_cycle.timestamp`, or use the previous-cycle boundary rather than the freshly written current-cycle timestamp. Add a regression test that generates a review body after `last_cycle` has already been updated for the current cycle and still expects same-cycle merges to appear.

## Complacency score

**2/5** — Cycle 364 did real work: it posted the expected 25 step comments on issue `#1778`, `state-invariants` and `metric-snapshot` passed, and it dispatched concrete follow-up issues for the cycle 363 journal defect. But the cycle still published a worklog that fails final-state validation, left the authoritative review ledger inconsistent with its own narrative, and dispatched the review issue with false “PRs merged: None” context. Those are not catastrophic failures, but they are exactly the kind of accuracy drift and chronic bookkeeping sloppiness that an adversarial review is supposed to catch.
