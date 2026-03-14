# Cycle 251 Review

## 1. [worklog-accuracy] The merged `write-entry` PR missed the production `process-merge` subject format, so cycle 251 still published empty PR sections

**File**: tools/rust/crates/write-entry/src/main.rs:841-846
**Evidence**:
- The new PR extraction logic only treats a `#123` reference as a pull request when the preceding token is exactly `PR`, via `token.eq_ignore_ascii_case("pr")`.
- The added regression test only covers singular subjects like `state(process-merge): PR #537, PR #543 merged [cycle 154]` (`tools/rust/crates/write-entry/src/main.rs:2595-2605`).
- The real cycle-251 `process-merge` receipt subject is `state(process-merge): PRs #1199, #1197 merged [cycle 251]` (`git show --no-patch --format='%H %s' ac1ca9f`), so the new parser does not recognize either PR number.
- The symptom is visible in the published worklog: `PRs merged` and `PRs reviewed` both still say `None.` even though PRs `#1199` and `#1197` were merged this cycle (`docs/worklog/2026-03-14/030605-cycle-251-review-merge-dedup-fix.md:11-17`).
- The same worklog also omits the final docs receipt `e78aea3`, while `bash tools/cycle-receipts --cycle 251 --repo-root .` returns six receipts including that docs commit.
- `bash tools/validate-docs worklog --file docs/worklog/2026-03-14/030605-cycle-251-review-merge-dedup-fix.md --cycle 251 --repo-root .` still exited 0, so the validator did not catch either discrepancy.
**Recommendation**: Make PR extraction accept the real `process-merge` subject variants used in production (`PRs #...`, comma-separated lists, and other existing commit-message forms), add a regression test using the exact `ac1ca9f` subject shape, and extend `validate-docs worklog` so a worklog cannot pass when its PR sections or receipt table diverge from `cycle-receipts`.

## 2. [state-integrity] The dedup guard blocks any repeated issue number, not just the duplicate live-session case it was supposed to fix

**File**: tools/rust/crates/record-dispatch/src/lib.rs:250-268
**Evidence**: `apply_dispatch_patch()` now rejects a dispatch whenever *any* existing `agent_sessions` entry has the same issue number, regardless of whether the older session is already `merged`, `failed`, `closed_without_pr`, or otherwise terminal. The only added test covers that blanket rejection path (`tools/rust/crates/record-dispatch/src/lib.rs:583-600`); there is no regression test for the actual `dispatch-review` + `record-dispatch` double-write sequence and no coverage for a legitimate closed-then-re-dispatched issue. That is a stronger rule than the rest of the state model enforces: `state-invariants` only treats **duplicate in_flight sessions** for the same issue as an integrity failure (`tools/rust/crates/state-invariants/src/main.rs:1126-1140`). Cycle 251 therefore replaced one narrow duplicate-write bug with a broader per-issue uniqueness constraint that is not reflected in the invariant model and would prevent any future retry/re-dispatch flow on the same issue number.
**Recommendation**: Deduplicate by live/open session state rather than by raw issue number, or reconcile by issue identity plus latest terminal status. Add tests for both cases: (1) `dispatch-review` followed by `record-dispatch` for the same still-open review issue must fail cleanly, and (2) a terminal session for an issue must not prevent a later legitimate re-dispatch if the process ever needs one.

## 3. [complacency-detection] Cycle 251 recorded cycle 250 findings as actioned even though the current artifacts show the defect persisted

**File**: docs/state.json:5480-5491
**Evidence**:
- The new review-history entry says cycle 250 finding F1 (`worklog-accuracy`) was `actioned` because PR `#1197` merged and says F2 (`state-integrity`) was `actioned` because the dedup guard landed.
- The very next cycle's published worklog still exhibits the core F1 defect: its PR sections are empty and its receipt table is incomplete (`docs/worklog/2026-03-14/030605-cycle-251-review-merge-dedup-fix.md:11-17,43-51`).
- The cycle 251 journal overstates closure in the same direction: it says `Verified write-entry improvements address chronic root causes` and claims `PRs merged/reviewed sections now populate automatically from process-merge receipts`.
- The same journal entry then acknowledges that the disposition-language problem remains unresolved (`docs/journal/2026-03-14.md:57-74`).
- That means merged code and optimistic narrative were treated as proof of resolution before the current cycle artifacts actually demonstrated it.
**Recommendation**: Reclassify cycle 250 F1 as `actioned_failed` or `deferred` instead of `actioned`, narrow the cycle 251 journal language to the scope actually delivered, and require artifact-level regression verification before writing review-history notes that claim a chronic finding is resolved.

## Complacency score

**2/5** — Cycle 251 did real work: it merged the prior review artifact, merged a substantive tool PR, kept the state invariants passing, and posted a full set of 20 step comments on the cycle issue. But the cycle still published a worklog that contradicts the cycle receipts, shipped a dedup fix whose guard is broader than the invariant it was meant to satisfy, and then recorded the prior cycle's findings as actioned even though the new artifacts immediately disproved that closure. That is not maximum complacency, but it is still materially overstating success.
