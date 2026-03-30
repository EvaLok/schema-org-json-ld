# Cycle 411 Review

## 1. [worklog-accuracy] Worklog omits the second forward-work dispatch even though it was committed after the dispatch existed

**File**: docs/worklog/2026-03-30/065519-cycle-411-frozen-commit-fix-audit-circuit-breaker-article-dispatch.md:7
**Evidence**: The published worklog says cycle 411 dispatched only `#2011`, and its `Issues processed` list names only `#345`, `#2011`, and `#2010` (lines 7 and 16-18). But `docs/state.json` now records a second in-flight cycle-411 dispatch, `#2014`, at lines 6055-6062, and issue `#2009` contains a `Duplicate Run Notice` saying the duplicate run dispatched `#2014`. This omission was not unavoidable: commit `b9ede45d` (`state(dispatch): record #2014...`) landed at `07:03:23Z`, and the docs commit that published this worklog (`2cb7a56`) did not land until `07:08:34Z`, so the worklog was still being edited after the second dispatch existed.
**Recommendation**: When a duplicate run adds substantive cycle work before the docs commit is finalized, update the worklog narrative and issue list to include it. If the process intends to freeze the worklog before duplicate-run activity, then later duplicate runs should stop instead of mutating cycle state after publication.

## 2. [journal-quality] The cycle 411 follow-through section grades a different commitment than the one it quotes

**File**: docs/journal/2026-03-30.md:111
**Evidence**: The cycle 411 entry quotes two prior commitments at lines 113-114: review `#2003` when Copilot completes, and verify that the self-modifications section says `None` when the scoped diff is empty. But the actual follow-through sentence at line 116 evaluates a different commitment entirely: `Disclose pipeline exclusions in worklog status lines (F4, deadline c415)`. That F4 commitment appears in the resumed cycle 410 note at line 95, not in the quoted block above it. As written, the journal never states whether the two quoted commitments were actually met.
**Recommendation**: Make the follow-through section grade the exact commitments it quotes, or update the quoted block so it reflects the latest commitment set being evaluated. Each quoted observable should receive an explicit disposition.

## 3. [state-integrity] `last_cycle` still summarizes a one-dispatch cycle while the live ledger shows two in-flight sessions

**File**: docs/state.json:6680
**Evidence**: The state ledger lists two in-flight cycle-411 agent sessions: `#2011` at lines 6046-6053 and `#2014` at lines 6055-6062, and the top-level counter is `in_flight_sessions: 2` at line 6679. But `last_cycle.summary` at line 6684 still says cycle 411 only `dispatched #2011`, and `last_cycle.timestamp` / `last_eva_comment_check` remain `2026-03-30T06:54:59Z` at lines 6685 and 6687, which predates the second dispatch and the later close-out comments on issue `#2009`. The file therefore mixes a post-duplicate-run ledger with a pre-duplicate-run cycle summary.
**Recommendation**: Keep `last_cycle.summary`, `last_cycle.timestamp`, and related freshness fields synchronized with the final cycle ledger. If post-close-out duplicate-run writes are allowed, the summary metadata must be refreshed as well; otherwise, block those writes and keep the cycle snapshot immutable.

## 4. [test-gap] The frozen-commit fix changed a subtle code path without adding a regression test for the resumed-cycle scenario

**File**: tools/rust/crates/pipeline-check/src/main.rs:1961
**Evidence**: Direct push `f50d26d6` changed `frozen_commit_status_for_date()` from `git show --stat` to `git ls-tree` specifically to handle resumed cycles where docs exist in the commit tree but not in that commit's diff. The commit touched only production code in `tools/rust/crates/pipeline-check/src/main.rs` (plus the cycle 410 worklog), with no test changes in the same commit. The existing frozen-commit tests at lines 5854-6065 cover baseline pass/fail cases and missing artifacts, but none construct the exact regression scenario described in the new code comment at lines 1961-1962: docs committed earlier in the cycle, followed by a later cycle-tagged commit whose diff does not mention them.
**Recommendation**: Add a focused regression test that creates a temporary repo with docs committed first and a later cycle-tagged commit that changes something else, then assert that `verify_frozen_commit_for_date()` passes by inspecting the commit tree rather than the diff.

## Complacency score

**3/5** — Capped at 3/5 because cycle 411 explicitly bypassed a blocking/publish-level gate during `record-dispatch` (`pipeline gate bypassed` is recorded in commit `5d872be`). The cycle did contain real work and the final pipeline gate passed, but chronic categories were not truly under control: `worklog-accuracy` and `state-integrity` both drifted again once the duplicate run added `#2014`, and the journal still broke its own commitment-audit trail by grading the wrong prior promise.
