# Cycle 216 Review

## 1. [state-integrity] The cycle removed the known `#954` duplicate but left another duplicate session that still inflates metrics

**File**: docs/state.json:2283-2298
**Evidence**: `docs/state.json` still contains two merged entries for issue `#886`, both pointing to PR `#887` with only a four-second dispatch timestamp difference. `derive-metrics` derives `total_dispatches` from `agent_sessions.len()` and increments both `produced_pr` and `merged` for every matching row (`tools/rust/crates/derive-metrics/src/main.rs:110-154`). That means the duplicate is not harmless history. The currently committed state on this branch reports `(279 dispatches, 272 PRs produced, 270 merged)`, while a de-dup of just the `#886` duplicate drops those counts to `(278, 271, 269)`. Cycle 216's worklog says stale state was fixed by removing the duplicate `#954` session, but the ledger is still structurally dirty and the metrics are still being derived from that dirty ledger.
**Recommendation**: Reconcile the existing `#886` duplicate immediately and broaden the duplicate-session cleanup to scan the full ledger, not just the cycle's newest stale row. Make the invariant check fail whenever one issue/PR pair appears more than once unless the duplication is explicitly modeled as separate sessions.

## 2. [worklog-accuracy] The cycle 216 receipt table is not auditable because two listed receipts do not exist

**File**: docs/worklog/2026-03-10/082355-cycle-216-summary.md:42-52
**Evidence**: After `git fetch --unshallow origin`, `git rev-parse --verify 236a8cc^{commit}` and `git rev-parse --verify 9da9313^{commit}` still fail, so these are not shallow-clone misses. The surrounding cycle history shows plausible real commits instead: `a1eb170` is `state(cycle-start): begin cycle 216, issue #961 [cycle 216]`, and `e2fd23e` is `state(process-merge): PR #960 merged [cycle 216]`. The worklog therefore presents an audit trail that a reviewer cannot reproduce from the repository history.
**Recommendation**: Generate receipt tables from verified commit IDs only. Fail worklog generation if any receipt cannot be resolved locally with `git rev-parse --verify <sha>^{commit}` before the file is written.

## 3. [failure-semantics] PR #958's wrapper does not actually fail closed when `derive-metrics` fails

**File**: tools/record-dispatch:32-39
**Evidence**: The wrapper captures the `record-dispatch` binary output first, then runs `bash "$SCRIPT_DIR/derive-metrics" --apply`, and only then prints the stored success text. That suppresses misleading stdout, but it does not prevent partial state mutation: reproducing the added failure case with an invalid legacy status leaves a new committed `agent_sessions` row for issue `#603` and increments `copilot_metrics.total_dispatches` from `2` to `3` even though the wrapper exits `1` with `Error: agent_sessions[1].status has unsupported value 'mystery_status'`. The added shell test only checks exit code, stderr, and empty stdout (`tools/test-record-dispatch.sh:119-133`), so it never asserts that the repo is rolled back or remains internally consistent after failure. This contradicts the PR's own "fail-closed" claim.
**Recommendation**: Make the wrapper atomic: either run `derive-metrics` before committing the dispatch change, or snapshot and restore `docs/state.json`/git state when the follow-up step fails. Extend the shell test to assert that the failure path leaves no new dispatch commit and no extra `agent_sessions` row behind.

## Complacency score

4/5 — this cycle did some real work: issue #961 has startup/step comments, and PR #958 does have a formal GitHub approval review. But the cycle still narrated more control than it actually had. The "state cleanup" left another duplicate session in place, the worklog published two non-existent receipt hashes, and the merged "fail-closed" fix still leaves partially mutated state behind on its failure path. That is a materially complacent cycle even though some of the process evidence is real.
