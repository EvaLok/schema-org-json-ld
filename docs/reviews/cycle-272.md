# Cycle 272 Review

## 1. [worklog-accuracy] The worklog's processed-issues list swaps in PR #1317 and drops dispatched issue #1319

**File**: docs/worklog/2026-03-15/223004-cycle-272-3-dispatches-2-merges-review-processed.md:21-26
**Evidence**: The "Issues processed" section lists `#1315`, `#1317`, `#1312`, and `#1316`. This list incorrectly includes PR `#1317`, which was created from issue `#1315`, while omitting the infrastructure-consistency issue `#1319` that the same worklog says was dispatched in cycle 272 (`lines 7-11`). The ledger therefore mixes PR and issue numbers instead of recording the actual processed issues.
**Recommendation**: Generate the "Issues processed" section from issue-facing state transitions (`process-review`, `record-dispatch`, `process-audit`, `process-eva`) instead of merged PR numbers. The rendered list for cycle 272 should include `#1319` and should not substitute PR `#1317` for an issue.

## 2. [state-integrity] `last_cycle.summary` undercounts the cycle's merged PRs and disagrees with the cycle artifacts

**File**: docs/state.json:4123-4128
**Evidence**: `last_cycle.summary` says cycle 272 had "3 dispatches, 2 merges" and names only PR `#1317` and PR `#1318`. But `agent_sessions` records PR `#1313` as also merged in cycle 272 (`docs/state.json:3640-3646`), and the cycle worklog's "PRs merged" section lists all three merged PRs: `#1313`, `#1317`, and `#1318` (`docs/worklog/2026-03-15/223004-cycle-272-3-dispatches-2-merges-review-processed.md:15-19`). The repository therefore preserves two incompatible summaries of the same cycle.
**Recommendation**: Make `cycle-complete` derive `last_cycle.summary` from the same merged-session set used by the worklog and closing comment, or explicitly label the narrower count if it is meant to exclude review-artifact merges. As written, the summary silently understates cycle activity.

## 3. [process-adherence] The cycle 271 review history records one finding as ignored even though the orchestrator documented it as deferred and later dispatched a fix

**File**: docs/state.json:6214-6227
**Evidence**: The cycle 271 `review_agent.history` entry records `deferred: 0`, `dispatch_created: 2`, and `ignored: 1`. However, the orchestrator's disposition comment on review issue `#1312` says F1 and F2 were `dispatch_created`, F3 (`infrastructure-consistency`) was `deferred`, and F4 was `actioned`. The durable history should therefore show `deferred: 1` rather than `deferred: 0` and `ignored: 1`. Cycle 272 then dispatched issue `#1319` / commit `fba7475` to implement F3 after all, so recording that finding as ignored erases the outstanding obligation from the state history and misstates how the review was actually handled.
**Recommendation**: Repair the cycle 271 review-history entry so its disposition counts match the documented closure comment and later dispatch evidence (`deferred: 1` at minimum, or `dispatch_created: 3` if same-cycle follow-up dispatches are folded back into the consumed review). Add a consistency check so review-history dispositions cannot contradict the review-issue closure record.

## Complacency score

**3/5** — Cycle 272 did real work: it processed the prior review, merged the two targeted tool fixes, kept the pipeline green, and dispatched the remaining invariant work instead of ignoring it completely. But the cycle still let its audit trail drift: the worklog confuses issues with PRs, `last_cycle.summary` understates the number of merges, and the durable review-history entry rewrites a deferred finding into an ignored one. Those are not cosmetic mistakes — they distort the evidence the next cycle relies on to decide what happened and what still needs follow-through.
