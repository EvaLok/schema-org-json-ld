# Cycle 281 Review

## 1. [worklog-accuracy] The published current-state block was stale again before the cycle actually closed

**File**: docs/worklog/2026-03-16/163945-cycle-281-schema-dispatch-to-break-stabilization-deadlock.md:23
**Evidence**: The worklog publishes `In-flight agent sessions: 0` and `425 dispatches, 420 PRs` in its `Current state` block. But the same cycle's own close-out comments show two later dispatches at steps `C6`/`C7`, and the follow-up receipts `e462abd` and `67f7a26` update `docs/state.json` to `in_flight: 2`, `total_dispatches: 427`, and `dispatch_log_latest: "#1373 Review.contentReferenceTime (PHP + TS) (cycle 281)"`. The issue prompt explicitly exempted the receipt table from this post-worklog timing problem; it did not exempt the `Current state` section, and the worklog has no caveat that it is only a pre-dispatch snapshot.
**Recommendation**: Either publish the worklog after record-dispatch, or label the `Current state` block as a pre-dispatch snapshot so readers do not mistake it for the final cycle state.

## 2. [worklog-accuracy] The worklog reports the wrong state-invariants count for the final pipeline failure

**File**: docs/worklog/2026-03-16/163945-cycle-281-schema-dispatch-to-break-stabilization-deadlock.md:26
**Evidence**: The worklog says `Pipeline status: FAIL (state-invariants 13/15 — 3 chronic categories tool_hardened)`, and issue `#1372` step `2.5` repeats `13/15`. But the same issue's final gate comment at step `C5.5` says `state-invariants 14/15`, and a direct `bash tools/state-invariants` run on the cycle 281 state reports 14 passes with only `chronic intermediate state` failing. That means the published worklog understates the pass count (and overstates the failure count) while presenting the line as the cycle's settled state.
**Recommendation**: Derive the invariant count from the final `state-invariants` output used at `C5.5`, or mark earlier counts as provisional instead of publishing them as the final pipeline status.

## 3. [state-integrity] review_events_verified_through_cycle was advanced to 281 without a verify-review-events receipt

**File**: docs/state.json:4237
**Evidence**: Receipt `da99e93` (`state(field-refresh): review_events_verified 281, tool_pipeline refreshed [cycle 281]`) changes both `$.field_inventory.fields.review_events_verified_through_cycle.last_refreshed` and the top-level `$.review_events_verified_through_cycle` marker from `279` to `281`. The canonical cycle 281 receipt set contains `process-merge`, `cycle-start`, `process-review`, `field-refresh`, `cycle-complete`, and the excluded docs commit — but no `state(verify-review-events)` receipt. That is a process regression because this repository already has a dedicated `verify-review-events` tool and prior cycles recorded separate receipts when the marker was genuinely advanced (for example `98e5d65`, `357bd0c`, `428ab65`).
**Recommendation**: Advance `review_events_verified_through_cycle` only through the dedicated `verify-review-events` tool with its own auditable receipt, or emit an explicit no-op verification receipt if there were no review events to check.

## 4. [process-adherence] The cycle used a pipeline-gate override for dispatches, but the published artifacts do not say so

**File**: docs/worklog/2026-03-16/163945-cycle-281-schema-dispatch-to-break-stabilization-deadlock.md:28
**Evidence**: Issue `#1372` step `C5.5` explicitly says `Proceeding with --skip-pipeline-gate per existing deadlock`, and step `C7` records that two dispatches were then pushed (`#1373`, `#1375`). The worklog only says `Pipeline status: FAIL` and `Publish gate: published`; it never tells the reader that publication and dispatch required bypassing a blocking gate. Because the review prompt caps complacency scores when a blocking gate is overridden, omitting that detail weakens the forensic record.
**Recommendation**: Whenever `--skip-pipeline-gate` is used, state it plainly in the worklog and journal so the publication record distinguishes a genuine pass from an override.

## 5. [dispatch-quality] The deadlock-breaking dispatch points at a real schema gap, but the issue body is mis-specified for this repository

**File**: docs/journal/2026-03-16.md:316
**Evidence**: The journal frames issue `#1373` as the code-PR path that should exercise the chronic verification logic. That direction is reasonable, but the issue body itself tells the assignee to add a PHP test in `php/test/v1/Schema/ReviewTest.php` and to call `toArray()`. In this repository the Review tests actually live at `php/test/unit/ReviewTest.php`, and they serialize via `JsonLdGenerator::SchemaToJson()` rather than any `toArray()` method (`php/test/unit/ReviewTest.php:24-48`, `php/src/v1/Schema/Review.php:9-21`). Asking for `toArray()` is not just a path typo; it conflicts with the library's core architecture, where schema classes intentionally do not implement serialization methods and `JsonLdGenerator` handles reflection-based output. The issue body also says `Google uses this for VacationRental listings`, which overstates the public evidence: schema.org does list `contentReferenceTime` on `Review`, but Google's published Vacation Rental structured-data documentation does not list it in its documented property set. So the dispatch may generate a code PR, but the instructions are not repo-accurate and could easily send the coding agent down the wrong implementation path.
**Recommendation**: Rewrite the dispatch template to use the repository's real test paths and serialization pattern, and cite the exact schema.org/Google source with the correct scope instead of a broad unsupported `Google uses this` claim.

## Complacency score

**2/5** — Cycle 281 did some meaningful things right: the receipt table is accurate within its declared pre-doc scope, the journal finally marks the previous measurable condition as `UNMET`, and PR `#1371` really was merged with an `APPROVED` GitHub review artifact. But the cycle still repeated the stale-current-state reporting defect, published the wrong invariant count, advanced `review_events_verified_through_cycle` without a dedicated verification receipt, and hid the fact that both dispatches required `--skip-pipeline-gate`. Because a blocking gate was overridden, the score cannot exceed 3/5; given the repeated narrative drift and tool-bypass smell, 2/5 is the better fit.
