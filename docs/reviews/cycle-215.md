# Cycle 215 Review

## Findings

## 1. [worklog-accuracy] The worklog's "current state" and receipt table were hand-massaged away from the committed state

**File**: docs/worklog/2026-03-10/062644-cycle-215-summary.md:33-55
**Evidence**: The worklog says there was `1` in-flight agent session and lists `b8e635d` as the `cycle-start` receipt. But `git show --stat b8e635d` fails outright, while the actual cycle-start commit in the same history window is `c9993cb` (`state(cycle-start): begin cycle 215, issue #956 [cycle 215]`). The in-flight count is also not a faithful state snapshot: `docs/state.json` records `copilot_metrics.in_flight: 2` and still contains live rows for `#954` and `#957` (`docs/state.json:2507-2520,2677-2689`), while `write-entry` would normally populate the worklog from `copilot_metrics.in_flight` unless `--in-flight` was overridden or the file was edited afterward (`tools/rust/crates/write-entry/src/main.rs:290-294,387-396`). This is not a trustworthy tool-derived worklog block.
**Recommendation**: Stop hand-assembling the worklog state block and receipt table. Generate them only from the committed `docs/state.json` plus verified git receipts, and fail the cycle if any listed SHA does not resolve or if any current-state field disagrees with the state snapshot used to render the worklog.

## 2. [state-integrity] `docs/state.json` still carries a duplicate live dispatch for closed issue #954

**File**: docs/state.json:2500-2520
**Evidence**: The state ledger contains two separate entries for issue `#954`: one merged row for PR `#955` and another row that is still `in_flight`. Git history shows how that happened: `337c862` dispatched `#954` once as `[Cycle Review] Cycle 214 end-of-cycle review`, `bbaeace` dispatched the same issue again three seconds later as `Cycle 214 review`, and `ab72688` later recorded the merge of PR `#955` without removing the duplicate open row. Issue `#954` itself is closed, so keeping a second `in_flight` session in state is stale ledger data, not a real active task.
**Recommendation**: Treat `agent_sessions` as a per-issue ledger instead of an append-only bag. Reject duplicate dispatches for the same open issue, or reconcile by issue number/latest terminal status during merge so a closed issue cannot remain `in_flight`.

## 3. [metrics-drift] The dispatched "structural fix" for metrics drift is too narrow for the failure that still exists

**File**: docs/journal/2026-03-10.md:139-149
**Evidence**: The journal says the recurrence-escalation response was to dispatch issue `#957` so derived rates are "always consistent after any dispatch." But the issue spec for `#957` only proposes running `derive-metrics --apply` after `record-dispatch`; it does not address duplicate or stale `agent_sessions`. That gap already matters in the current cycle: `derive-metrics` derives `total_dispatches` from raw `agent_sessions.len()` and increments `in_flight` for every row whose status is `in_flight` or `dispatched` (`tools/rust/crates/derive-metrics/src/main.rs:110-154`), so rerunning it on the current state still reports `in_flight: 2` because the stale duplicate `#954` row remains. The cycle escalated the recurring symptom, but it scoped the fix too narrowly to repair the live integrity problem.
**Recommendation**: Expand or replace `#957` so the fix reconciles `agent_sessions` by issue identity or latest status, not just by rerunning percentage derivation. Add an integration test that dispatches the same issue twice and then merges it, and require the derived metrics to collapse that to one resolved session.

## 4. [review-evidence] The cycle again claimed PRs were reviewed without leaving an auditable review trail

**File**: docs/worklog/2026-03-10/062644-cycle-215-summary.md:18-27
**Evidence**: The worklog lists PRs `#951`, `#953`, and `#955` under `PRs reviewed`. GitHub does not support that claim with a durable review trail: `get_reviews` returns no review objects for all three PRs, PR comments are empty on `#951` and `#955`, and `#953` has only an owner issue comment about tab indentation plus a bot reply, not a formal approval or change-request review. The check evidence is also weak: PRs `#951` and `#955` show only a `claude-review` check run, and PR `#953` shows no check runs via the PR head at all. The cycle may have done real manual review, but it did not record one in a way that another reviewer can audit.
**Recommendation**: Only say a PR was "reviewed" when there is a recorded GitHub review or an explicit worklog note describing the review that was performed. Keep review evidence separate from post-merge confidence or bot commentary.

## Complacency score

4/5 — This cycle did not do nothing: PR #953 really did go through a useful revision round, and the final indentation cleanup was clean. But the cycle still narrated more control than it actually had:

- the worklog state block and receipt table are not reliable enough to audit directly,
- the committed state still counts a closed review issue as `in_flight`,
- the "structural fix" dispatched for recurring metrics drift does not cover the stale-session failure already present in the same cycle, and
- the worklog continues to say PRs were reviewed without leaving review evidence another operator can verify.

That is not total fabrication, but it is still a complacent cycle: the story of rigor is cleaner than the state ledger and receipt trail underneath it.

## Recommendations

1. Make worklog generation fail when any receipt SHA is unresolved or any rendered current-state field disagrees with `docs/state.json`.
2. Add an `agent_sessions` invariant that forbids an issue from being simultaneously terminal (`merged`/`closed`) and `in_flight`.
3. Broaden the metrics-drift fix to reconcile duplicate/stale sessions, not just recompute percentages after dispatch.
4. Require an auditable PR review artifact before listing a PR under `PRs reviewed`.
