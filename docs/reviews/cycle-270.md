# Cycle 270 Review

## 1. [tool-correctness] Dispatch-based fail-closed logic also blocks cycles that never produced a merged PR

**File**: tools/rust/crates/verify-review-events/src/main.rs:319-328,710-732
**Evidence**: `collect_dispatch_cycles()` records every agent session's `cycle` without checking the session `status`, and `compute_safe_advance()` then treats any checked cycle with dispatch activity but no discovered PRs as unverified. That closes the original fail-open hole, but it also collapses failed, `closed_without_pr`, `closed_without_merge`, and still-open sessions into the same bucket as “a merged PR existed but discovery missed it.” The tests only cover a synthetic dispatch-without-PR case (`tools/rust/crates/verify-review-events/src/main.rs:1118-1126`) and never exercise non-merged statuses, even though this repository already tracks non-merged outcomes in state (`docs/state.json:3796-3808`).
**Recommendation**: Distinguish “cycle had merged work we failed to discover” from “cycle had dispatches but no merge outcome.” Either drive the fail-closed decision from merged-session evidence / GitHub merged-PR queries instead of raw dispatch presence, or add status-aware tests for failed, `closed_without_pr`, `closed_without_merge`, and open sessions and document which statuses are supposed to block advancement.

## 2. [worklog-accuracy] The receipt table pads the receipt narrative by repeating the same commit under multiple labels

**File**: docs/worklog/2026-03-15/182731-cycle-270-2-merges-chronic-state-integrity-and-review-evidence-verified-clean-pipeline.md:41-58
**Evidence**: The worklog's “Commit receipts” section renders 12 rows, but four SHAs are repeated under alternate labels: `b7cba54` appears as both `cycle-tagged` and `review-history`, `7def8b4` as `cycle-tagged` and `state-fix`, `357bd0c` as `cycle-tagged` and `verify-review-events`, and `4c3522a` as `cycle-tagged` and `chronic-verified`. The underlying commits are real, but the table is not a faithful one-row-per-receipt rendering of the canonical receipt set, so the “8 receipts” close-out story depends on deduplicating the table by hand.
**Recommendation**: Render the exact `bash tools/cycle-receipts --cycle 270 --repo-root .` output in the worklog, or keep one canonical receipt row per SHA and add a separate alias/meaning column instead of repeating the same commit as multiple receipts.

## 3. [state-integrity] Cycle 270 re-verified chronic categories while its own rationale said the hard case was still pending

**File**: docs/state.json:4156-4187
**Evidence**: Both chronic entries were stamped with `verification_cycle: 270`, but the `state-integrity` rationale itself says “Code-PR hard case will be naturally tested at next schema implementation merge” (`docs/state.json:4159-4162`). The journal says the same thing even more plainly: “The hard case (code PRs requiring reviews) has not been exercised at runtime” (`docs/journal/2026-03-15.md:333-339`). That means the cycle closed `state-integrity` and `review-evidence` before the promised end-to-end proof had actually happened. Cycle 269 had already warned that the hard-case proof needed to be encoded before restoring the chronic markers (`docs/reviews/cycle-269.md:15-19`), and cycle 270 still closed the loop first and deferred the proof to “next schema implementation merge.”
**Recommendation**: Drop `verification_cycle` back to pending/in-progress until a merged code PR has been checked by the hardened verifier, or split the closure into two states (“tool hardened” vs “runtime code-PR path observed”) so the unresolved proof obligation stays visible.

## 4. [journal-quality] The follow-through commitment is reactive and does not define a success condition

**File**: docs/journal/2026-03-15.md:333-339
**Evidence**: The journal correctly admits that the hard case has not yet been exercised, but the only concrete commitment is: “If verify-review-events fails on the first code-PR cycle, treat as actioned_failed and re-investigate the filtering logic.” That is not an observable completion condition for the unresolved work. It specifies what to do after failure, but not when the rerun must happen, what evidence counts as success, or how the chronic categories should be handled if the first code-PR cycle passes.
**Recommendation**: Rewrite the commitment as a measurable check with explicit pass/fail outcomes, e.g. “On the first merged code PR after cycle 270, rerun `verify-review-events`; if the code PR is verified from pre-merge approvals, keep the category closed, otherwise reopen `review-evidence` / `state-integrity` and mark the follow-through actioned_failed.”

## Complacency score

**2/5** — The cycle did real corrective work: it merged the hardening PR, fixed stale merge metadata, and closed with a genuinely green C5.5 gate instead of repeating cycle 269’s gate override. But the cycle still massaged the receipt narrative, re-closed two chronic categories before the admitted hard case had actually run, and wrote a commitment that postpones the proof obligation instead of pinning it to an observable success check. That is not maximum complacency, but it is still closure language moving faster than evidence.
