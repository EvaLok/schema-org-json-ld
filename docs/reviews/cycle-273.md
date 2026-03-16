# Cycle 273 Review

## 1. [cycle-boundary] Cycle 273 counted two PRs that were already merged before the cycle started

**File**: docs/worklog/2026-03-16/002528-cycle-273-2-merges-2-dispatches-review-processed-state-integrity-fix.md:5-15
**Evidence**: The worklog says cycle 273 merged PR `#1320` and PR `#1322`, and `docs/state.json:4152` repeats that in `last_cycle.summary`. But GitHub shows `merged_at` for those PRs at `2026-03-16T00:17:13Z` and `2026-03-16T00:17:19Z`, while the cycle-start receipt `7e2acf6` was committed at `2026-03-16T00:17:27Z`. The merge commits (`65d11c5` and `8ba2536`) therefore predate the cycle start. The repository then re-stamped those sessions with later `agent_sessions[].merged_at` values (`docs/state.json:3667-3682`) because `tools/rust/crates/process-merge/src/main.rs:63-76,267-292` records `current_utc_timestamp()` instead of the actual GitHub merge time. Cycle 273 therefore reported receipt-processing time as if it were the merge event itself.
**Recommendation**: Make `process-merge` record the authoritative GitHub `merged_at` timestamp (or reject merges whose real merge time predates the current cycle), and derive cycle membership from that value before rendering worklogs, summaries, and processed-issue sections.

## 2. [verification-drift] Review verification was advanced to cycle 273 without closing the chronic entries or showing a verify-review-events run

**File**: docs/state.json:4246-4277,6270
**Evidence**: `review_events_verified_through_cycle` was advanced to `273` (`docs/state.json:6270`) by the cycle-273 `process-review` path, but the two chronic categories that were supposedly waiting on runtime proof still retain the string-valued `verification_cycle` entries `270-tool-hardened, pending-code-PR-runtime-proof` (`docs/state.json:4252` and `4277`). The cycle-273 worklog and journal also carry the same step forward as unfinished (`docs/worklog/2026-03-16/002528-cycle-273-2-merges-2-dispatches-review-processed-state-integrity-fix.md:37-38`; `docs/journal/2026-03-16.md:17-20,36-41`), and the cycle-273 receipt table contains no `verify-review-events` receipt at all. The state therefore claims review-event verification is current through cycle 273 while every adjacent artifact still describes the same categories as unverified `tool_hardened`.
**Recommendation**: Gate `review_events_verified_through_cycle` advancement on an auditable `verify-review-events` run, and update the chronic-category entries to numeric runtime-verified cycles in the same state transition. If that did not happen in cycle 273, revert the marker to `272`.

## 3. [pipeline-reporting] The new warning-only invariant is hidden from the pipeline denominator

**File**: tools/rust/crates/pipeline-check/src/main.rs:372-376
**Evidence**: PR `#1320` added the `chronic_intermediate_state` invariant, and `tools/rust/crates/state-invariants/src/main.rs:1492-1493` now reports totals as `Passed: 14/15` with one WARN. Running `bash tools/state-invariants` on the current repository shows exactly that: 15 checks, 14 passes, 0 fails, and a WARN for `worklog-accuracy`, `state-integrity`, and `review-evidence`. But `pipeline-check` formats the detail as `passed / (passed + failed)`, which discards warned checks from the denominator, so `bash tools/pipeline-check` reports `state-invariants: PASS (14/14 invariants pass)`. The cycle-273 worklog then copied that misleading summary into `docs/worklog/2026-03-16/002528-cycle-273-2-merges-2-dispatches-review-processed-state-integrity-fix.md:31`.
**Recommendation**: Change `pipeline-check` to include warned checks in the total count and/or surface warning counts explicitly for `state-invariants`, then render worklog pipeline summaries from that corrected output instead of a denominator that makes unresolved warnings disappear.

## 4. [dispatch-quality] Dispatch #1326 proposes a feature that write-entry already implements

**File**: tools/rust/crates/write-entry/src/main.rs:624-668
**Evidence**: Issue `#1326` asks for a new `--auto-issues` flag so `write-entry worklog` can derive the “Issues processed” section from `agent_sessions`. But `apply_worklog_auto_derivations` already auto-derives processed issues from git history and `docs/state.json`, merges them into the final section, and `derive_issue_processed_from_state` already walks `agent_sessions` using status-change timestamps (`tools/rust/crates/write-entry/src/main.rs:788-832`). The repository also already has a regression test proving this behavior (`tools/rust/crates/write-entry/src/main.rs:3427-3490`). The cycle-273 defect was therefore not “missing auto-issues support”; it was that the underlying session timestamps were shifted and the wrong cycle data was being trusted.
**Recommendation**: Retarget `#1326` from “add a new flag” to “find why the existing auto-derivation path produced wrong cycle-272 output,” focusing on timestamp provenance and code paths that bypass or misfeed the current derivation logic.

## Complacency score

**2/5** — Cycle 273 did repair one genuine defect from the prior review (the cycle-271 disposition counts) and the receipt table itself now matches `cycle-receipts`. But the cycle still misclassified pre-cycle merges as in-cycle activity, advanced the review-verification marker without corresponding evidence or state closure, published a pipeline summary that hid the new warning-only invariant, and dispatched at least one follow-up issue (`#1326`) that duplicates behavior already present in the codebase. That is not active improvement; it is a pattern of acting on favorable narratives without checking whether the recorded mechanism matches reality.
