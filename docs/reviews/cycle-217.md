# Cycle 217 Review

## Findings

1. **The worklog's current-state metrics were rendered from stale numbers**
   Category: worklog-accuracy
   The cycle 217 worklog says the current state was `280 dispatched`, `274 produced PRs`, `272 merged`, and `98.2% dispatch-to-PR rate` (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-10/102710-cycle-217-summary.md:35-40`). The committed state for the same cycle says `total_dispatches: 281`, `produced_pr: 274`, `merged: 272`, `in_flight: 2`, and `dispatch_to_pr_rate: 97.5%` (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:2713-2725`). That means the worklog still is not a faithful snapshot of the state file it is supposed to summarize.

2. **The worklog says there were no self-modifications even though the cycle merged orchestrator-tool changes**
   Category: self-modification-coverage
   The worklog explicitly says `Self-modifications: None.` even while listing PRs `#963` and `#965` as merged and reviewed (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-10/102710-cycle-217-summary.md:14-33`). Those PRs were not documentation-only housekeeping: cycle 217 merged commit `2d6fe42` touching `tools/rust/crates/state-invariants/src/main.rs` and commit `035042d` touching `tools/rust/crates/write-entry/src/main.rs`. This underreports the amount of orchestrator/tooling change the cycle actually performed and makes the worklog a weaker audit artifact.

3. **The journal notices the recurring metrics problem but skips the lesson from the earlier partial fix**
   Category: journal-reflection-gap
   Cycle 215's journal entry said dispatch `#957` would make derived rates "always consistent after any dispatch" (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-10.md:139-141`). Two cycles later, cycle 217 says derive-metrics drift is "the most recurring pipeline issue" and dispatches `#969` and `#971` to make dispatch and merge writes atomic (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-10.md:209-211`). The cycle 217 entry never explicitly acknowledges that the earlier "structural fix" was only a partial fix or explains why that scope error survived review, so the reflection records the symptom but not the governing lesson.

4. **The chronic-response ledger is still documenting worklog drift instead of proving the fix worked**
   Category: chronic-response-stall
   `docs/state.json` still records `worklog-accuracy` as a chronic category on the "structural-fix" path with `verification_cycle: null` (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:3043-3049`). The same state file shows `worklog-accuracy` recurring in cycle reviews 211, 212, 213, 214, and 216 (`/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:4081-4158`), and cycle 217 immediately produced fresh evidence of the same class of drift in its current-state block and self-modification section. The bookkeeping is honest that a chronic category exists, but it still is not being used to force verification or recalibration when the promised fix remains unverified.

## Recommendations

1. Make `write-entry` fail if the rendered "Current state" block disagrees with the committed `copilot_metrics` snapshot that will be written in the same cycle.
2. Stop allowing `Self-modifications: None.` when the cycle merged or dispatched changes under `tools/`, `AGENTS.md`, skills, or other orchestrator-control surfaces.
3. Require journal entries for recurring categories to state what prior fix proved insufficient, not just what new follow-up issue was dispatched.
4. Add an explicit verification or recalibration rule for `review_agent.chronic_category_responses` so `verification_cycle` cannot remain null across repeated recurrences of the same category.

## Complacency score

4/5 — cycle 217 did real work: it cleaned a real duplicate session, merged two tooling fixes, and dispatched follow-ups for a recurring metrics problem. But the artifacts still show "going through motions" behavior in the places that were supposed to be structurally fixed: the worklog published stale state again, self-modification accounting still collapsed to `None.`, and the journal described another recurrence without squarely naming why the previous fix was under-scoped.

## Priority items

1. Make worklog generation fail closed on stale current-state metrics and missing self-modification coverage.
2. Turn chronic-category tracking into an enforced verification loop instead of a ledger with open-ended `verification_cycle: null`.
3. Tighten journal expectations for recurring problems so each recurrence records the failed assumption from the previous cycle.
