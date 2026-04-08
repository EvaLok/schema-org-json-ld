# Cycle 461 Review

## 1. [code-quality] The new chronic-category-currency gate silently skips malformed structural-fix entries

**File**: `tools/rust/crates/pipeline-check/src/main.rs:2916-2923`, `tools/rust/crates/pipeline-check/src/main.rs:7891-8003`
**Evidence**: PR #2305 added `chronic-category-currency`, but the core loop does `let Some(verification_cycle) = ... else { continue; }` for `chosen_path == "structural-fix"`. If a structural-fix entry is missing or has a non-numeric `verification_cycle`, the gate quietly ignores it instead of surfacing bad state. The added tests cover warn/fail/pass thresholds and skipping non-structural entries, but there is no regression test for the malformed-entry path. That makes the new blocking gate fail open on invalid data.
**Recommendation**: Treat missing or invalid `verification_cycle` on structural-fix entries as a blocking `Error`/`Fail`, and add a regression test that proves malformed chronic entries cannot bypass the gate.

## 2. [state-integrity] Cycle 461 promoted dispatch #2310 to “resolved” and “verified” before the fix had even landed

**File**: `docs/worklog/2026-04-08/215638-cycle-461-review-processing-dispatch-chronic-structural-fix-close-out.md:5-10`, `docs/state.json:6946-6952`, `docs/state.json:7653-7657`, `docs/state.json:8068-8073`, `docs/state.json:8095-8100`, `docs/state.json:13631-13658`
**Evidence**: First, the worklog says cycle 456 deferred `worklog-accuracy` was “auto-resolved via cycle 460 F1 dispatch,” and the deferred-findings ledger marks that item `resolved: true` with `resolved_ref: "docs/reviews/cycle-460.md"`. Second, cycle 460’s review history records those findings as `dispatch_created`, not actioned, and dispatch `#2310` is still only an `in_flight` session in cycle 461 state. Third, the same premature promotion appears in chronic tracking: `process-adherence` and `state-integrity` were bumped to `verification_cycle: 461` even though their own rationales say PR `#2311` is still in flight and “Cycle 462 will verify after #2311 lands.” Together, those records convert an unmerged proposal into canonical “resolved/verified” state.
**Recommendation**: Do not mark deferred findings resolved or chronic structural fixes verified until the cited fix has merged and a later artifact demonstrates it worked. If same-cycle bookkeeping must note the dispatch, record it as pending rather than resolved/verified.

## 3. [process-adherence] The recorded C5.5 step-comment audit points at cycle 460’s issue, not cycle 461’s

**File**: `docs/worklog/2026-04-08/215638-cycle-461-review-processing-dispatch-chronic-structural-fix-close-out.md:33-35`
**Evidence**: The worklog presents the C5.5 rerun as the cycle’s authoritative gate story. But the actual C5.5 comment on issue `#2309` shows `step-comments` auditing `issue #2300` (“found 26 unique step comments ...”), while `current-cycle-steps` in the same payload correctly audits `issue #2309`. The cycle got lucky because both issues currently have 26 step comments, but the evidence used for close-out did not actually prove that the dedicated step-comment check ran against the current cycle issue.
**Recommendation**: Fix `pipeline-check` so `step-comments` and `current-cycle-steps` derive the same current-cycle issue number, and do not rely on the step-comment substep as close-out evidence until that mismatch is eliminated.

## Complacency score

**2/5** — Cycle 461 did real work: it ran the documented tools, kept a visible step trail, and recorded the temporary C5.5 failure before rerunning. But it also merged a new blocking gate that fails open on malformed state, converted in-flight dispatch `#2310` into “resolved/verified” ledger truth before any merge proved it, and accepted close-out evidence whose step-comment audit referenced the wrong issue. Because the cycle had a blocking C5.5 failure before rerun, the score is capped below 4/5 anyway; the evidence supports staying at 2/5 rather than stretching to 3/5.
