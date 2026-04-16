# Cycle 504 Review

## 1. [worklog-accuracy] The worklog invents a dependency that the commit history disproves

**File**: docs/worklog/2026-04-16/075639-cycle-504-cycle-503-review-consumed-3-findings-deferred-f3-instance-fix-applied-via-stale-session-2549-reconciliation-pr-2553-merged-admin-override-no-new-dispatches-gate-still-blocked-by-2542.md:6
**Evidence**:
- The worklog says commit `eb3fedf0` added the missing `## Complacency score` heading to `docs/reviews/cycle-503.md` “so process-review's extract_score could parse.”
- The actual git ancestry shows the opposite order: `1ab9d3d` (`state(process-review): cycle 503 review consumed`) is already committed before `eb3fedf0` exists. `git rev-list --parents -n 1 1ab9d3d` returns parent `826e2a2`, while `git rev-list --parents -n 1 eb3fedf0` returns parent `1ab9d3d`.
- The heading edit was therefore a post-consumption cleanup on `master`, not a prerequisite for the `process-review` receipt the worklog cites.
**Recommendation**: Rewrite the narrative to match the real sequence. If `process-review` truly requires that heading, enforce it before merge and fail review artifacts that are not tool-consumable instead of retroactively patching them on `master`.

## 2. [receipt-integrity] The published receipt table omits the close-session receipt the same worklog uses as evidence

**File**: docs/worklog/2026-04-16/075639-cycle-504-cycle-503-review-consumed-3-findings-deferred-f3-instance-fix-applied-via-stale-session-2549-reconciliation-pr-2553-merged-admin-override-no-new-dispatches-gate-still-blocked-by-2542.md:6,34-43
**Evidence**:
- Line 6 relies on close-session receipt `826e2a2` to prove the cycle’s F3 instance fix (`in_flight_sessions 1->0` for stale issue `#2549`).
- `git show 826e2a2` confirms that this is a real cycle-504 state commit created at `2026-04-16 07:44:13Z`, well before cycle-complete at `2026-04-16 07:56:08Z`.
- The canonical command requested by the issue, `bash tools/cycle-receipts --cycle 504 --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld`, returns only four receipts: `1bf2340`, `c075c94`, `1ab9d3d`, and `d26dea7`. `826e2a2` is absent.
- The issue explicitly says only the docs commit and record-dispatch commit are structurally excluded from the receipt table. A same-cycle `close-session` state commit is neither.
**Recommendation**: Either teach `cycle-receipts`/the worklog generator to include same-cycle `close-session` receipts, or stop citing omitted receipts as authoritative evidence in the published worklog.

## 3. [journal-quality] The journal knowingly republishes the exact mixed-outcome collapse the prior review had just flagged

**File**: docs/journal/2026-04-16.md:141-149
**Evidence**:
- The cycle 504 journal records three distinct outcomes for the prior commitments: C1 met, C2 not met, and C3 not applicable.
- Despite that, the section still publishes the aggregate label `**Not followed.**`
- The entry itself admits this is the defect: line 149 says “The F2 mixed-outcome collapse is visible in this very journal entry's previous-commitment-status narrative.”
- Step `0.6` on issue `#2554` had already enumerated the three separate dispositions (`monitoring=will-be-met`, `conditional-on-resolution=not-met`, `conditional-on-24h=not-applicable`), so the cycle had the needed evidence and still collapsed it into one misleading status.
**Recommendation**: Stop emitting a single authoritative previous-commitment status when the cycle already has per-commitment outcomes. Change `write-entry journal` to record each commitment separately, and until that lands, avoid presenting the aggregate label as if it were precise.

## Complacency score

**2/5** — Cycle 504 was not silent: issue `#2554` has a full step-comment trail, CI on PR `#2553` passed, and current `state-invariants`/`metric-snapshot` are green. But the published artifacts still overstated what happened: the worklog reverses the actual order of a required hotfix, the receipt table omits a same-cycle state receipt it relies on as evidence, and the journal knowingly repeats the same mixed-outcome reporting defect it was supposed to be tightening. This cycle’s review mandate says any overridden FAIL or blocking-level gate would cap the score at **3/5**; even under that ceiling, the evidence here fits a lower **2/5**.
