## 1. [code-change-quality] The cycle cleared a blocking C5.5 failure by widening the detector skip window instead of fixing the path

**File**: tools/rust/crates/pipeline-check/src/main.rs:3539-3545; docs/worklog/2026-04-19/060836-cycle-516-review-consumed-1-actioned-1-dispatch-created-1-deferred-f1-fix-dispatched-as-2600.md:28-33; docs/journal/2026-04-19.md:89-107
**Evidence**: `pipeline-check` now hard-codes `POST_DISPATCH_DELTA_FIRST_APPLICABLE_PREVIOUS_CYCLE = 517` and the surrounding comment explicitly says cycles 515 and 516 are being skipped because the production close-out path is still broken. The worklog and journal both admit C5.5 initially failed, that the real fix is only in-flight as #2600, and that the constant was bumped 515→517 so close-out could proceed. That means the cycle responded to a live blocking gate by moving the detector boundary forward, not by restoring the missing post-dispatch delta behavior in the production path.
**Recommendation**: Treat detector-boundary bumps as temporary waivers that must be recorded separately from the detector logic, and keep the gate fail-closed until the end-to-end close-out path is fixed and proven by runtime coverage.

## 2. [state-integrity] `review_events_verified_through_cycle` still advanced in a non-verify-review-events commit

**File**: docs/state.json:10984-10986,18439; docs/worklog/2026-04-19/060836-cycle-516-review-consumed-1-actioned-1-dispatch-created-1-deferred-f1-fix-dispatched-as-2600.md:42-50
**Evidence**: `docs/state.json` says `review_events_verified_through_cycle` is “managed by verify-review-events tool only” and now marks both the field and its freshness at cycle 516. But the cycle-516 receipt table contains no `state(verify-review-events)` receipt at all, and `git blame` points both the freshness marker and `review_events_verified_through_cycle: 516` to commit `0bdde482` (`state(pipeline): record initial C5.5 FAIL for cycle 516`), not to a verify-review-events state write. Cycle 516 even posted a C4.7 step comment saying verify-review-events succeeded, so the review-verification signal exists, but the persisted state provenance still does not match the commit that changed it.
**Recommendation**: Do not advance `review_events_verified_through_cycle` or its freshness marker from generic pipeline/state commits. Either emit a dedicated `state(verify-review-events): ...` receipt every cycle or stop claiming tool-only provenance in `field_inventory`.

## 3. [journal-quality] The new “Standing Eva blockers” section is wired to a state path that does not exist in production

**File**: tools/rust/crates/write-entry/src/main.rs:2992-2997,4083-4086; docs/journal/2026-04-19.md:113-115; docs/state.json:11076-11084
**Evidence**: `write-entry` reads standing blockers from `/eva_escalations/issues` and returns an empty list when that path is absent. The real `docs/state.json` in this repository has no `eva_escalations` object; it still tracks unresolved Eva questions in `open_questions_for_eva`, which currently lists eight issues. Because of that schema mismatch, the cycle-516 journal renders `### Standing Eva blockers` followed by `- None recorded.` even though the repository state still records active Eva-blocked questions.
**Recommendation**: Read blockers from the actual state schema (or support both schemas during migration), and add regression coverage using the repository’s real `docs/state.json` shape so the journal cannot silently print `None` when blockers exist.

## Complacency score

**3/5** — The cycle documented several of its own problems candidly, but it still cleared a blocking C5.5 failure by broadening a detector exemption, repeated the same review-events provenance drift in `state.json`, and shipped a newly-added journal section that silently reports no Eva blockers because it reads the wrong state path. Because a blocking pipeline gate was effectively waived in-cycle, the score is capped at 3/5.
