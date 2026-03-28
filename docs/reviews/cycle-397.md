# Cycle 397 Review

## 1. [worklog-accuracy] The cycle still mutated its published worklog after the pre-dispatch snapshot

**File**: docs/worklog/2026-03-28/201945-cycle-397-review-processed-two-tool-fixes-dispatched.md:26-40
**Evidence**: `git show 30dd2ee6 -- docs/worklog/2026-03-28/201945-cycle-397-review-processed-two-tool-fixes-dispatched.md` shows the original C5 worklog with a single `In-flight agent sessions: 2` line and one `Next steps` section. `git show 05b0d7a4 -- docs/worklog/2026-03-28/201945-cycle-397-review-processed-two-tool-fixes-dispatched.md` then rewrites that same artifact after C6 by adding `In-flight agent sessions (post-dispatch): 3` plus a second `Next steps (post-dispatch)` section that includes review issue `#1932`. The cycle's own journal calls this pattern out as the problem space — `A pipeline-check guard is the right defense — detect the mutation rather than trying to prevent all manual edits` (`docs/journal/2026-03-28.md:395`) — but cycle 397 still relied on a manual post-close-out patch.
**Recommendation**: Treat the C5 worklog snapshot as immutable. If post-dispatch state must be preserved, write it as a separate tool-generated addendum or a distinct artifact instead of editing the already-published worklog in place.

## 2. [state-integrity] Field-inventory freshness still claims review-event verification that the top-level state does not support

**File**: docs/state.json:6228-6230,10270-10271
**Evidence**: `field_inventory.fields.review_events_verified_through_cycle.last_refreshed` says `cycle 397`, but the top-level `review_events_verified_through_cycle` value is still `392`. Cycle 397 was not a no-op: the worklog records a merged PR (`docs/worklog/2026-03-28/201945-cycle-397-review-processed-two-tool-fixes-dispatched.md:12-18`), and `bash tools/cycle-receipts --cycle 397 --repo-root .` confirms a `process-merge` receipt for PR `#1926`. That means the cycle had new merged-review activity to verify, yet the freshness marker advanced five cycles beyond the underlying value. This is the same stale-marker category that cycle 396 already flagged, so the cycle acknowledged the problem but left the misleading freshness claim in place.
**Recommendation**: Only refresh `review_events_verified_through_cycle.last_refreshed` when the top-level value is actually revalidated and advanced as needed. Add an invariant that rejects freshly stamped review-event markers when the verified-through cycle lags merged-review activity.

## 3. [journal-quality] The journal collapses commitment accounting and drops observable completion conditions

**File**: docs/journal/2026-03-28.md:380-399
**Evidence**: The `Previous commitment` block lists two separate commitments, but the follow-through verdict is a single `**Not followed.**` sentence that immediately mixes outcomes by also claiming `Worklog-accuracy monitoring commitment addressed via dispatch #1930`. That makes it impossible to tell, commitment-by-commitment, what was actually completed versus deferred. The next section then compresses the new commitments into a malformed one-line list — `1. 1. Conduct tool audit ... 2. Review and iterate ...` — with no observable completion conditions at all. This is not the concrete, checkable commitment format the review process expects, especially when the tool audit has now slipped for a third consecutive cycle.
**Recommendation**: Record follow-through one prior commitment at a time with an explicit status (`followed`, `not followed`, `deferred`) and evidence for each. Rewrite next-cycle commitments as separate checklist items with observable completion conditions so the next review can verify them without interpretation.

## Complacency score

**2/5** — Cycle 397 did the mechanical work: it used the receipt/state tools, posted a full step-comment trail on issue `#1927`, and dispatched targeted follow-up issues. But the cycle still repeated a same-cycle worklog mutation, left the stale `review_events_verified_through_cycle` freshness claim unresolved after it was already identified in cycle 396, and wrote a journal entry whose commitment accounting is too sloppy to audit cleanly. That is more than surface noise; it shows chronic accuracy problems being acknowledged without being cleanly contained.
