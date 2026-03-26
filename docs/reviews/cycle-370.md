# Cycle 370 Review

## 1. [process-adherence] The cycle closed out with a published `PASS` summary even though the final pipeline gate had already failed on a blocking finding

**File**: `docs/worklog/2026-03-26/122628-cycle-370.md:25-30`
**Evidence**: The published worklog says `Pipeline status: PASS (3 warnings)`. But the cycle issue’s final pipeline gate comment reports `Pipeline: FAIL (3 warnings, 1 blocking: current-cycle-steps)` with `has_blocking_findings: true` because issue `#1817` was still missing mandatory steps `[1, 3, 5, 6, C1, C2, C3]` at gate time (`https://github.com/EvaLok/schema-org-json-ld/issues/1817#issuecomment-4134313752`). The worklog was then explicitly “patched to PASS (3 warnings)” in step C3 (`https://github.com/EvaLok/schema-org-json-ld/issues/1817#issuecomment-4134321460`), and step C8 repeated that PASS summary without any posted rerun of the final gate after those missing comments were added (`https://github.com/EvaLok/schema-org-json-ld/issues/1817#issuecomment-4134330205`). This is a real gate override, not a harmless wording glitch.
**Recommendation**: Derive the published pipeline status only from the last successful C5.5 run after mandatory step comments exist. If C5.5 fails, keep the worklog/close-out status as FAIL and stop close-out until the gate is rerun cleanly.

## 2. [worklog-accuracy] The post-dispatch refresh updated the counts but left the worklog’s next steps stale against the final cycle state

**File**: `docs/worklog/2026-03-26/122628-cycle-370.md:27-34`
**Evidence**: After review dispatch, `docs/state.json` gained a third in-flight session for issue `#1822` (`docs/state.json:5267-5272`), and the follow-up refresh commit `72d8a7c` updated the worklog’s in-flight count from 2 to 3 and total dispatches from 588 to 589. But the same worklog still lists only `#1818` and `#1820` under `Next steps`, even though step C6 and step C8 both say the cycle review was dispatched as `#1822` (`https://github.com/EvaLok/schema-org-json-ld/issues/1817#issuecomment-4134329684`, `https://github.com/EvaLok/schema-org-json-ld/issues/1817#issuecomment-4134330205`). The refresh was therefore only partial: the numbers were patched, but the narrative stayed frozen at the pre-review-dispatch snapshot.
**Recommendation**: Make the C6.5 worklog refresh regenerate `Next steps` from the current in-flight sessions instead of patching only numeric fields. A refreshed state block that still omits the newly created in-flight review issue is still stale.

## 3. [journal-quality] The journal describes complacency as “improving” even though the score worsened from 2/5 to 3/5

**File**: `docs/journal/2026-03-26.md:214-220`
**Evidence**: The cycle 370 journal says, `Complacency scores improving (2/5 in cycle 368 to 3/5 in cycle 369).` That reverses the direction of the metric. The cycle 368 review ends at `2/5` (`docs/reviews/cycle-368.md:21-23`), while the cycle 369 review ends at `3/5` and explicitly says the score is capped there because the cycle normalized a blocking-level pipeline signal (`docs/reviews/cycle-369.md:21-23`). Calling `2/5 → 3/5` an improvement is not reflection; it is factual drift about the core health metric the journal is supposed to interpret.
**Recommendation**: Write score trends in an unambiguous direction (`rose`, `fell`, `worsened`, `improved`) and tie the interpretation back to the cited review rationale instead of optimistic shorthand.

## Complacency score

**3/5** — The cap applies because cycle 370 overrode a blocking-level final pipeline gate: step C5.5 on issue `#1817` reported `overall: fail` with `has_blocking_findings: true`, yet the published worklog and close-out still reported `PASS (3 warnings)`. The cycle also repeated the chronic worklog-refresh problem by updating counts without updating next steps, and the journal misread a worsening complacency score as improvement. Those are not cosmetic defects; they show the accountability layer is still being massaged after the fact instead of faithfully reporting what actually happened.
