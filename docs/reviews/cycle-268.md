# Cycle 268 Review

## 1. [tool-correctness] `verify-review-events` accepts any review event as sufficient evidence for code PRs

**File**: tools/rust/crates/verify-review-events/src/main.rs:547
**Evidence**: `fetch_pull_request_reviews()` counts every object returned by `gh api repos/EvaLok/schema-org-json-ld/pulls/{N}/reviews` (`count: entries.len()` at lines 547-568), and the verifier then treats any nonzero count as success for code PRs (`verified = !classification.expects_reviews() || review_data.count > 0` at lines 289-305). The code never inspects review `state`, reviewer identity, or whether the review happened before merge. That means `COMMENTED`, `CHANGES_REQUESTED`, `DISMISSED`, self-authored, or post-merge reviews can all satisfy the “verified” check. Cycle 268’s journal then overstates the result by claiming the tool “cannot be gamed” (`docs/journal/2026-03-15.md:249-252`), even though this logic is gameable.
**Recommendation**: Treat only the intended review states as valid evidence (at minimum filter by `APPROVED` if approval is the policy), reject self-reviews and post-merge reviews, and add regression tests that cover comment-only, changes-requested, dismissed, and post-merge review events.

## 2. [tool-correctness] Missing PR discovery still lets entire cycles auto-advance as “verified”

**File**: tools/rust/crates/verify-review-events/src/main.rs:246
**Evidence**: `collect_pull_requests()` only looks at merged entries already present in `agent_sessions` and silently drops anything without a usable PR number or cycle mapping (`continue` at lines 260-277). It does not cross-check GitHub for merged PRs in the cycle window, even though the original issue explicitly allowed that fallback approach. After that, `compute_safe_advance()` treats a cycle with no discovered PRs as verified via `.unwrap_or(true)` (lines 637-659), and the test suite locks that behavior in with `compute_safe_advance_advances_when_no_prs_are_in_range()` (`src/main.rs:930-933`). The result is fail-open behavior: if a merged PR is absent from `agent_sessions`, mis-timestamped, or otherwise skipped, the cycle still advances.
**Recommendation**: Fail closed when a checked cycle has no discovered PRs unless GitHub has been queried to confirm there were no merged PRs in that window, or add a second discovery path that enumerates merged PRs from GitHub and reconciles them against `agent_sessions`.

## 3. [state-integrity] Cycle 268 closed chronic `review-evidence` before the verifier proved the hard case

**File**: docs/state.json:4153
**Evidence**: The chronic `review-evidence` entry was marked `status: "verified"` with the rationale that the structural fix is complete because `verify-review-events` “verified cycles 266-268 and advanced marker to 268 with genuine evidence” (`docs/state.json:4153-4159`). But re-running `bash tools/verify-review-events --json` from the pre-apply cycle state at commit `4a48d9b` showed only three PRs in scope: `#1284` (docs), `#1287` (docs), and `#1290` (tooling). No code PR in the verified range exercised the path that is supposed to require formal review events. Despite that, the worklog says the cycle “resolved chronic state-integrity and review-evidence” (`docs/worklog/2026-03-15/143359-cycle-268-verify-review-events-merged-chronic-state-integrity-resolved.md:9-10`), and the journal says the tool “cannot be gamed” (`docs/journal/2026-03-15.md:249-252`). With findings 1 and 2 still true, that closure claim is ahead of the evidence.
**Recommendation**: Downgrade chronic `review-evidence` back to in-progress or explicitly mark it as only partially resolved until the verifier is hardened and either exercised against a real code PR or covered by tests that prove the code-PR review path behaves correctly.

## Complacency score

**2/5** — Cycle 268 did real corrective work: it reverted prior marker drift, merged a real GitHub-backed verification tool, and the corrected schema counts appear accurate. But it also merged that tool on the first clean pass, declared a 25-cycle chronic problem resolved immediately, and used “genuine evidence” / “cannot be gamed” language before the tool’s strictness was proven. That is not a blocking-gate override, so the score cap does not apply, but it is still a clear case of closing the narrative faster than the evidence warranted.
