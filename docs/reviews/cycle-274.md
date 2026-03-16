# Cycle 274 Review

## 1. [tool-correctness] PR #1325 merged an auto-summary path that still cannot run on the live repository

**File**: tools/rust/crates/cycle-complete/src/main.rs:308-348
**Evidence**: `derive_cycle_summary` now aborts on any `agent_sessions` entry with `status == "merged"` and no `merged_at` before it can decide whether that session is even relevant to the current cycle. The current `docs/state.json` still contains 16 such legacy merged sessions, and the cycle-274 journal explicitly admits the new path hit that wall and required a manual `--summary` override instead (`docs/journal/2026-03-16.md:74-75`). So the merged PR did not satisfy its own core acceptance path on the repository it was merged into: omitting `--summary` still does not work in practice.
**Recommendation**: Do not treat this as done until the no-`--summary` path actually works on current state. Either backfill the legacy `merged_at` debt before merging the feature, or narrow the fail-closed check so it only blocks sessions that could plausibly belong to the current cycle window.

## 2. [journal-accuracy] The journal falsely exonerates finding #4 instead of correcting its scope

**File**: docs/journal/2026-03-16.md:68-70
**Evidence**: The journal says cycle-273 finding #4 was “factually incorrect” because PR #1327 “replaces old heuristic-based derivation with a cleaner opt-in approach.” But the pre-#1327 code already auto-derived `Issues processed` inside `apply_worklog_auto_derivations` from done items, git history, and `derive_issue_processed_from_state(...)` over `agent_sessions` (`git show 4c5b5567b17f949895c02a9c290160ff54a42c44:tools/rust/crates/write-entry/src/main.rs`). PR #1327 changed the interface and removed heuristics; it did not prove the original review wrong. The review’s point was that `#1326` was framed as “add support” even though support already existed in another form.
**Recommendation**: Retract the “factually incorrect” claim and record the narrower truth: `#1326` was a redesign of existing behavior, not evidence that the review fabricated a feature that never existed.

## 3. [review-ledger] Cycle 273 dispositions were recorded in state without the two dispatch-created outcomes

**File**: docs/state.json:6292-6305
**Evidence**: The cycle-274 worklog says the cycle processed the review and dispatched `#1331` and `#1332` as direct follow-ups (`docs/worklog/2026-03-16/032027-cycle-274-2-merges-2-dispatches-review-processed-verification-advancement.md:12-13`). `process-review` has an explicit `dispatch_created` field for this exact disposition type (`tools/rust/crates/process-review/src/main.rs:62-78,568-581`). But the stored history entry for cycle 273 records only `actioned: 2` and `deferred: 2` with no `dispatch_created`, which makes two actually-routed findings look merely deferred and strips the ledger of the concrete remediation path it is supposed to preserve.
**Recommendation**: Repair the cycle-273 history entry so it includes `dispatch_created: 2` and preserve a note mapping each finding number to its disposition. Aggregated counts without the dispatch bucket are not auditable enough for review consumption.

## 4. [receipt-integrity] `write-entry` still cannot safely apply manual receipt corrections when a cycle repeats the same tool name

**File**: tools/rust/crates/write-entry/src/main.rs:1102-1126
**Evidence**: `merge_receipts` reduces manual overrides into a `HashMap` keyed only by lowercase tool name, so multiple `--receipt` overrides for the same tool collapse to one surviving SHA. Cycle 274 is exactly the shape that breaks this model: its canonical receipt table repeats `cycle-tagged`, `process-merge`, and `record-dispatch` entries (`docs/worklog/2026-03-16/032027-cycle-274-2-merges-2-dispatches-review-processed-verification-advancement.md:57-64`). The only regression test covers a single `process-merge` override and never exercises duplicate tool names (`tools/rust/crates/write-entry/src/main.rs:3368-3386`). That leaves the hand-correction scenario from this cycle structurally unfixed.
**Recommendation**: Change receipt overrides to preserve occurrence order instead of keying only by tool, then add a regression test with duplicate tool names in the auto-derived table. Until that exists, the worklog generator is still one repeated-tool cycle away from silently reintroducing bad receipts.

## Complacency score

**2/5** — Cycle 274 did perform one real corrective action by reverting the verification marker and rerunning `verify-review-events`. But it also merged an auto-summary feature that the live state still cannot use without manual override, rewrote a prior review finding as “factually incorrect” on overstated grounds, recorded review dispositions without the actual dispatch-created outcomes, and left the multi-receipt hand-fix bug structurally intact. That is not a clean improvement cycle; it is a cycle that kept the favorable story moving while several core audit paths remained unreliable.
