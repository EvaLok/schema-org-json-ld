## 1. [state-integrity] Connectivity-test session was recorded as merged when GitHub shows it was closed unmerged

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:5600-5607
**Evidence**: The cycle-386 connectivity-test session for issue `#1874` / PR `#1875` is stored with `"status": "merged"` and `"merged_at": "2026-03-27T22:17:14Z"`. The same cycle's metrics were then incremented to `merged: 538` and `produced_pr: 549` (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:5828-5841`). GitHub metadata for PR `#1875` shows the opposite outcome: `merged: false`, `state: closed`, `closed_at: 2026-03-27T22:15:49Z`. This means cycle 386 permanently inflated the merged-PR history even though `state-invariants` still passed.
**Recommendation**: Reconcile agent-session outcomes against the actual GitHub PR state before marking a session as merged or incrementing merge metrics. This session should be reclassified as a non-merged resolution, and the derived counters corrected.

## 2. [worklog-accuracy] Self-modifications section omitted part of the cycle-386 docs commit

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-27/222123-cycle-386-copilot-restored-outage-resolved.md:23-25
**Evidence**: The self-modifications section lists only the cycle-377 worklog rename. But the cycle-386 docs commit (`8b0856a`) also created `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-27/222123-copilot-restored-outage-resolved.md` as a redirect stub (`Moved to 222123-cycle-386-copilot-restored-outage-resolved.md`) and rewrote the cycle-377 worklog link in `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-27.md`. The worklog therefore understates its own documentation-side changes.
**Recommendation**: When reporting self-modifications, include every infrastructure/docs artifact changed by the cycle's own docs commit, including redirect stubs and journal link rewrites, or explicitly state that the list is partial.

## 3. [process-adherence] The worklog hides that the cycle proceeded past a failed pipeline gate

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-27/222123-cycle-386-copilot-restored-outage-resolved.md:27-31
**Evidence**: The published worklog reports `Pipeline status: PASS (3 warnings)`. But issue `#1873` step comments for cycle 386 recorded `Warning: pipeline-check failed: pipeline-check failed with status 1` at Step 0 and `Pipeline check failed (see warnings)` at Step 4 before the cycle continued. That failure matters because the review instructions cap complacency at 3/5 whenever a blocking-level gate is overridden, yet neither the worklog nor the journal explicitly records that an override happened.
**Recommendation**: If a cycle continues after a failed startup gate, record that failure and override explicitly in the worklog/journal so later reviewers can apply the correct complacency cap without reconstructing the issue timeline from comments.

Complacency score: **3/5**.

Justification: the score is capped at 3/5 because cycle 386 proceeded after a recorded `pipeline-check` failure. The cycle did some things right — receipts match `cycle-receipts`, `state-invariants` and `metric-snapshot` pass, and the issue has 26 per-step comments — but the incorrect merged-session record and incomplete documentation show real drift remained even in a short recovery cycle.
