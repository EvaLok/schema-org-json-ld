# Cycle 398 Review

## 1. [worklog-accuracy] The published cycle 398 worklog was rewritten twice after the docs commit

**File**: docs/worklog/2026-03-28/223423-review-processed-invariant-fix-three-merges.md:30-34
**Evidence**: The final artifact now says `- **Pipeline status**: PASS (3 warnings)`.
`git log --follow -- docs/worklog/2026-03-28/223423-review-processed-invariant-fix-three-merges.md`
shows that the worklog was first published in `611fd20d`, then patched in `07072144`,
then rewritten again in `d8f63362`.
`git diff 611fd20d 07072144 -- docs/worklog/2026-03-28/223423-review-processed-invariant-fix-three-merges.md`
added a second line `- **Pipeline status (post-dispatch)**: PASS (3 warnings)` while
preserving the original `FAIL (2 warnings)` result.
`git diff 07072144 d8f63362 -- ...` then deleted both earlier lines and replaced them with
the single `PASS (3 warnings)` line now in the file.
That is exactly the audit-trail mutation cycle 397 was supposed to stop: the original
recorded C5 result is no longer visible in the published artifact.
**Recommendation**: Treat the first committed worklog as immutable. If post-C5 status must be preserved, append a separate addendum or distinct close-out artifact instead of editing the original `Pipeline status` line in place.

## 2. [review-consumption] Cycle 398 marked the worklog-accuracy finding as actioned even though PR #1931 only warns on the same behavior

**File**: docs/state.json:10284-10299
**Evidence**: `review_agent.history` records cycle 397 finding `worklog-accuracy` as
`actioned` with the note `PR #1931 merged (worklog immutability detection)`.
But the implementation in `tools/rust/crates/pipeline-check/src/main.rs:1622-1628`
makes `worklog-immutability` a `Severity::Warning` step.
The regression test at
`tools/rust/crates/pipeline-check/src/main.rs:5074-5112` explicitly expects a mutated
original pipeline-status line to return `StepStatus::Warn`, not fail.
Cycle 398 immediately exercised that escape hatch: the same worklog was mutated twice
after publication, and the close-out step comment for C5.5 reported
`worklog-immutability: WARN (pipeline status corrected from FAIL to PASS — expected mutation)`.
That is not an actioned finding; it is a detector that knowingly permits the reviewed
defect to recur in the very next cycle.
**Recommendation**: Reclassify the cycle 397 worklog-accuracy finding as still deferred or partially actioned, and either make original-line mutation blocking at C5.5 or move post-dispatch status into a tool-generated addendum that never rewrites the original line.

## 3. [state-integrity] Final state still contradicts itself about whether cycle 398 dispatched a review agent

**File**: docs/state.json:6326-6335,10314
**Evidence**: In the final cycle 398 state, `git show 3ccd2495:docs/state.json`
records `in_flight_sessions: 1` at line 6326 and `review_dispatch_consecutive: 2`
at line 10314.
Those values reflect the committed `state(record-dispatch): #1935 dispatched [cycle 398]`
receipt `d70ce64f`.
But `last_cycle.summary` at lines 6329-6333 still says
`0 dispatches, 3 merges (PR #1929, PR #1931, PR #1933)`.
This is the same stale-summary pattern already called out in earlier reviews
(`docs/reviews/cycle-309.md:9-13`, `docs/reviews/cycle-312.md:14-17`,
`docs/reviews/cycle-368.md:9-13`): the completed state claims a review dispatch is
in flight while the durable cycle summary still denies that any dispatch occurred.
**Recommendation**: Patch `last_cycle.summary` after `record-dispatch`, or split pre-dispatch close-out state from final completed-cycle state so the canonical `docs/state.json` snapshot cannot simultaneously record an in-flight review agent and `0 dispatches`.

## 4. [audit-cadence] The tool audit is 22 cycles overdue and the journal still treats that as a note instead of an enforced action

**File**: docs/state.json:6182-6185,6335; docs/journal/2026-03-28.md:419-439
**Evidence**: `field_inventory.fields.last_tool_audit_cycle` still says the field should
refresh `every 10 cycles`, while the top-level value remains `376` in cycle 398.
The journal explicitly acknowledges the miss as
`Deferred again — 4th consecutive miss, now 22 cycles overdue`.
It then repeats the same pattern by carrying a new commitment:
`Conduct tool audit (22 cycles overdue — if deferred a 5th time, dispatch to Copilot as an automated tool).`
This is no longer a one-off miss.
The cycle knows it has a chronic cadence failure, calls it `the most persistent process failure in the project`,
and still leaves it as another future promise instead of triggering an immediate enforcement path.
**Recommendation**: Stop tracking the tool audit as a soft journal intention. Dispatch it immediately or gate cycle completion on it once the 10-cycle cadence is exceeded, so the repo cannot keep accumulating “critical and must not slip again” language without any structural consequence.

## Complacency score

**4/5** — Cycle 398 did real work, but the review evidence shows a pattern of learning the words of prior findings without closing the control gap. The worklog mutation happened again immediately after merging a “fix” for worklog immutability, the same stale `last_cycle.summary` contradiction reappeared despite earlier review history, and the tool-audit miss advanced from “critical” language to a 22-cycle backlog with no enforcement. This is no longer isolated drift; it is repeated normalization of known defects.
