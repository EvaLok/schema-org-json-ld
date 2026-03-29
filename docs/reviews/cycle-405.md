# Cycle 405 Review

## 1. [code-quality] The new timestamp fallback is fail-open even though the code and PR describe it as fail-closed

**File**: tools/rust/crates/pipeline-check/src/main.rs:183
**Evidence**: The default `CommandRunner::fetch_issue_comments_with_timestamps` implementation wraps the legacy body-only result as `vec![(comment_bodies, String::new())]`. `collect_step_comment_timestamps` then skips empty `created_at` values entirely at lines 1276-1277, so any runner that relies on the default implementation silently disables the temporal-ordering check instead of failing closed. The inline comment says this path keeps the check "fail-closed," but the behavior is the opposite. The new temporal-ordering tests also never cover this compatibility path: each one overrides `fetch_issue_comments_with_timestamps` and panics if the body-only fallback is used (for example lines 8347-8349, 8435-8437, 8524-8526, and 8593-8595).
**Recommendation**: Make the compatibility path explicit. Either fail or warn when `current-cycle-steps` has to use the body-only fallback, or update the comment/PR narrative to admit that the fallback is fail-open for backward compatibility and add a regression test that exercises that path.

## 2. [worklog-accuracy] The worklog says no issues were processed even though it records two issue closures in the same document

**File**: docs/worklog/2026-03-29/162259-cycle-405-temporal-ordering-merge-housekeeping-cleanup.md:5
**Evidence**: The worklog's "What was done" section says cycle 405 closed agent-task `#1975` and audit-inbound `#1974` (line 6), but the dedicated "Issues processed" section says `None.` (line 16). GitHub issue metadata confirms both closures happened during the cycle: `#1975` closed at `2026-03-29T16:19:21Z` and `#1974` closed at `2026-03-29T16:19:50Z`.
**Recommendation**: Reconcile the section semantics. If closed issues count as processed, list them under "Issues processed." If the section is meant to exclude merged/closed dispatch artifacts, rename it so the worklog no longer contradicts itself.

## 3. [worklog-accuracy] The published pipeline status records the early C1 snapshot instead of the actual C5.5 gate result

**File**: docs/worklog/2026-03-29/162259-cycle-405-temporal-ordering-merge-housekeeping-cleanup.md:25
**Evidence**: The worklog says `Pipeline status: FAIL at C1`. The cycle issue trail shows that was a deliberate early snapshot: Step C3 says the worklog was written with the `FAIL at C1` status, and Step C5.5 later reports that the inner pipeline at gate time had `current-cycle-steps` passing with all 20 pre-gate steps present. At C5.5 the only remaining blocking item was the cycle-404 `step-comments` cascade plus a `doc-validation` cascade caused by the stale worklog text itself. So the worklog is not a faithful record of the final auditable gate state.
**Recommendation**: Record the final C5.5 result explicitly in the worklog, either by writing the worklog after C5.5 or by appending a post-gate addendum that preserves the earlier C1 snapshot while also stating the actual close-out outcome.

## 4. [journal-quality] The journal claims the C5.5-reporting commitment was followed even though the worklog still failed that observable

**File**: docs/journal/2026-03-29.md:256
**Evidence**: The previous commitment quoted in the cycle 405 entry says: `Address worklog-accuracy finding by ensuring future worklogs report exact C5.5 gate outcomes.` The follow-through then marks that commitment as `FOLLOWED` because the worklog reported the `C1` result honestly (line 259). That does not satisfy the stated observable. The Step C5.5 comment on issue `#1977` explicitly says `doc-validation` cascaded because the worklog still reported `FAIL at C1` while the pipeline at C5.5 time said PASS for the current-cycle checks.
**Recommendation**: Mark this commitment as not followed or deferred, and create a concrete remediation artifact (issue, dispatch, or merged fix) for the close-out/worklog timing defect instead of redefining the observable after the fact.

## 5. [state-integrity] The chronic-category freshness marker was not refreshed even though cycle 405 says it performed a new chronic-category analysis

**File**: docs/state.json:6426
**Evidence**: `field_inventory.fields.review_agent.chronic_category_responses.last_refreshed` is still `cycle 402`, even though its cadence says it should be refreshed `when chronic categories are detected (5+ in last 6 reviews)`. Cycle 405 explicitly claims that analysis happened: the worklog says `4 chronic categories in last 6 reviews` (line 8), and the journal says `worklog-accuracy and process-adherence are chronic across 5/6 recent reviews` (line 263). A direct count of review headings across `docs/reviews/cycle-399.md` through `docs/reviews/cycle-404.md` shows four categories at 5/6 occurrences: `worklog-accuracy`, `state-integrity`, `process-adherence`, and `journal-quality`.
**Recommendation**: Refresh the `review_agent.chronic_category_responses` inventory marker whenever this analysis is performed, and update the corresponding response entries when the cycle claims a new chronic-category assessment.

## 6. [complacency-detection] The chronic-category analysis is narrative-only and does not produce concrete mitigation for all four chronic categories

**File**: docs/journal/2026-03-29.md:263
**Evidence**: The worklog says cycle 405 processed `4 chronic categories in last 6 reviews`, but the journal narrows the narrative to two categories (`worklog-accuracy` and `process-adherence`) and the next-cycle commitments are only: (1) process the review findings and (2) monitor whether the new temporal-ordering check helps. There is no concrete mitigation, verification step, or new artifact for the equally chronic `state-integrity` and `journal-quality` categories, and no evidence that the cycle's analysis updated any chronic-response state entry. That makes the analysis look performative rather than operationalized.
**Recommendation**: When a cycle claims a chronic-category analysis, require an artifact-backed output per chronic category: an updated state response entry, a filed issue/dispatch, or an explicit documented decision to recalibrate with verification criteria.

## Complacency score

**2/5** — The score is capped below 4 because the cycle's own close-out reported a `FAIL`/`CASCADE` at C5.5. Beyond that cap, cycle 405 still normalized a known worklog defect into a claimed success condition, left the chronic-category analysis mostly narrative, and published documentation that contradicts both GitHub issue activity and the final gate state.
