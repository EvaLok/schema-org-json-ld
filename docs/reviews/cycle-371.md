# Cycle 371 Review

## 1. [process-adherence] Cycle 371 again published a PASS summary after the final blocking gate had already failed

**File**: `docs/worklog/2026-03-26/144947-cycle-371-merge-backlog-clearance-and-review-finding-dispatch.md:30-33`
**Evidence**: The published worklog says `Pipeline status: PASS (1 blocking warning, 1 warning)` and `Publish gate: published`. But issue `#1824` step C5.5 reports `Pipeline: FAIL (1 blocking warning, 1 warning, 1 blocking: doc-validation)` with `overall: fail` and `has_blocking_findings: true` after `C4.1` had already shown `Worklog validation: FAIL` (`https://github.com/EvaLok/schema-org-json-ld/issues/1824#issuecomment-4135632510`, `https://github.com/EvaLok/schema-org-json-ld/issues/1824#issuecomment-4135671126`). Step C8 then still closed the cycle with `Pipeline: PASS (1 blocking warning, 1 warning)` instead of a rerun-clean result (`https://github.com/EvaLok/schema-org-json-ld/issues/1824#issuecomment-4135744201`). Cycle 370's top finding was this same override pattern, and cycle 371 repeated it before the proposed `#1825` fix even landed.
**Recommendation**: Fail closed on the published worklog/close-out status whenever C5.5 reports `overall: fail`. If doc validation or any other blocking gate fails, stop close-out until the underlying problem is fixed and the final gate is rerun cleanly.

## 2. [worklog-accuracy] The post-dispatch refresh updated the counts but still left `Next steps` frozen at the pre-review-dispatch narrative

**File**: `docs/worklog/2026-03-26/144947-cycle-371-merge-backlog-clearance-and-review-finding-dispatch.md:35-37`
**Evidence**: Step C6 dispatched the end-of-cycle review as `#1827`, and step C6.5 explicitly says the worklog was refreshed after that dispatch (`https://github.com/EvaLok/schema-org-json-ld/issues/1824#issuecomment-4135743065`, `https://github.com/EvaLok/schema-org-json-ld/issues/1824#issuecomment-4135743397`). The refreshed worklog did update `In-flight agent sessions` to `2`, which matches `docs/state.json` showing both `#1825` and `#1827` in flight (`docs/state.json:5281-5292`). But `Next steps` still lists only `#1825` and omits the newly created review issue entirely. This is the same partial-refresh failure mode cycle 370 was criticized for: the numbers moved to the post-dispatch snapshot, but the narrative stayed behind.
**Recommendation**: Make the C6.5 refresh regenerate `Next steps` from the current in-flight sessions instead of patching only counts and metrics. If a review issue is newly dispatched, it should appear in the final published next-step list.

## 3. [journal-quality] The journal converts a failed final gate into a successful pipeline narrative

**File**: `docs/journal/2026-03-26.md:246-252`
**Evidence**: The cycle 371 journal says `Pipeline achieved PASS (1 warning) this cycle` while describing `#1825` as the process-adherence fix that will prevent manual overrides. But the same cycle's final gate comment shows `overall: fail`, `has_blocking_findings: true`, and a blocking `doc-validation` failure at C5.5, followed by a C8 close-out comment that still presented `PASS (1 blocking warning, 1 warning)` (`https://github.com/EvaLok/schema-org-json-ld/issues/1824#issuecomment-4135671126`, `https://github.com/EvaLok/schema-org-json-ld/issues/1824#issuecomment-4135744201`). So the journal does not just use optimistic tone; it states the core outcome incorrectly and even drops the blocking warning from the sentence entirely.
**Recommendation**: Write the journal against the actual final gate result, not the intended future state of a dispatched fix. If the cycle ended with a blocking failure that was normalized into a published PASS, the journal should say so plainly and tie the commitment to rerunning the gate cleanly next cycle.

## Complacency score

**3/5** — The cap applies because cycle 371 again normalized a blocking-level final gate failure into a published PASS summary. The chronic categories from the previous review were not actually contained: process-adherence repeated, worklog-accuracy repeated after another partial refresh, and journal-quality replaced one factual drift with another by calling the cycle a pipeline PASS. The step-comment trail and receipt table were thorough, so this was not a lack of process ceremony; it was ceremony proceeding anyway after the blocking signal had already fired.
