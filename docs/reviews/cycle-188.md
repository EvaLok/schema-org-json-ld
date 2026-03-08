# Cycle 188 Review

## Findings

1. **Cycle 188’s closing narrative was not updated after PR #760 merged**
   Category: cycle-close-artifact-drift

   The final canonical state says cycle 188 merged PRs `#752`, `#754`, `#756`, and `#760`, with `resolved: 208`, `merged: 202`, and only one in-flight session (`docs/state.json:1912-1925`, `docs/state.json:2059-2071`, `docs/state.json:2283-2288`). But the cycle 188 worklog still presents `#759` as merely dispatched, omits PR `#760` from the merged-PR list, and freezes the metrics at `resolved: 207`, `merged: 201`, `in-flight: 2` (`docs/worklog/2026-03-08/090800-hundred-eighty-eighth-orchestrator-cycle.md:13-15`, `docs/worklog/2026-03-08/090800-hundred-eighty-eighth-orchestrator-cycle.md:40-49`, `docs/worklog/2026-03-08/090800-hundred-eighty-eighth-orchestrator-cycle.md:57-58`).

   The journal entry has the same problem: it says `#759` was only dispatched and makes “review and merge PRs from #759 and #761” a next-cycle commitment even though `#760` had already merged before the cycle closed (`docs/journal/2026-03-08.md:237-245`, `docs/state.json:1912-1917`). This is the same state/story drift the previous review warned about, just shifted from `state.json` into the closing artifacts.

2. **Field-inventory freshness still lags behind fields cycle 188 changed or relied on**
   Category: freshness-cadence

   Cycle 188 changed Copilot dispatch accounting and explicitly discussed remaining Eva directives, yet several cadence-governed freshness markers were not refreshed. `copilot_metrics.dispatch_to_pr_rate`, `copilot_metrics.in_flight`, and `copilot_metrics.pr_merge_rate` are still marked `cycle 187`, `eva_input_issues.closed_this_cycle` is still `cycle 187`, and `eva_input_issues.remaining_open` is still `cycle 184` (`docs/state.json:2141-2159`).

   That undercuts the worklog’s claim that cycle 187’s freshness-cadence finding was actioned (`docs/worklog/2026-03-08/090800-hundred-eighty-eighth-orchestrator-cycle.md:23-29`). The values themselves may currently be correct, but the evidence trail that they were checked this cycle is still stale.

3. **PR #760 made the clean-cycle gate fail on fresh shallow clones**
   Category: shallow-clone-commit-freeze

   The merged pipeline change now treats `cycle-status` as a blocking step (`tools/rust/crates/pipeline-check/src/main.rs:318-335`). In a fresh clone, running `bash tools/pipeline-check --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld` now returns `Overall: FAIL (1 warning)` because `cycle-status` fails the commit-freeze check even though `state-invariants` passes. The corresponding `bash tools/cycle-status --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld --json` output reports `check_failed: true` / `diverged: true` and the specific error `validated commit ea8ffff is not reachable in this repository`.

   The underlying code explicitly exits `1` whenever `commit_freeze.check_failed || commit_freeze.diverged` (`tools/rust/crates/cycle-status/src/main.rs:540-549`) and marks unreachable historical commits as `check_failed: true` (`tools/rust/crates/cycle-status/src/main.rs:552-620`). Cycle 188’s worklog still records the pipeline as `PASS` (`docs/worklog/2026-03-08/090800-hundred-eighty-eighth-orchestrator-cycle.md:57-60`), so the merged end-to-end behavior was not re-verified in the fresh-clone environment that Copilot agents actually use.

4. **PR #754’s branch-name linkage heuristic does not match this repo’s real Copilot branch naming**
   Category: branch-linkage-mismatch

   `housekeeping-scan` now suppresses stale issue findings only when an open Copilot draft PR branch contains the issue number as a numeric token, and the file comment states that “the repository uses” this as issue↔PR linkage (`tools/rust/crates/housekeeping-scan/src/main.rs:251-267`). The tests lock in the same assumption with examples like `copilot/add-severity-tiers-746` (`tools/rust/crates/housekeeping-scan/src/main.rs:513-566`).

   But the actual Copilot PRs merged this cycle do not follow that naming pattern. PR `#752` used head ref `copilot/fix-cycle-status-exit-code`, PR `#754` used `copilot/improve-housekeeping-scan-heuristic`, and PR `#760` used `copilot/fix-cycle-status-blocking` (GitHub PR metadata for EvaLok/schema-org-json-ld#752, #754, #760). The merged heuristic therefore encodes a repository convention that recent repository practice immediately disproves, so active work can still be reported stale simply because the branch name does not carry the issue number.

5. **The STARTUP_CHECKLIST response to audit #147 improves visibility, not enforcement**
   Category: checklist-enforcement-gap

   Step 0.6 says repeated journal commitments “must be either actioned this cycle or explicitly dropped” and concludes that the checklist change makes commitment tracking “structural rather than behavioral” (`STARTUP_CHECKLIST.md:63-74`). But the mechanism is still just a manual read-and-verify instruction; nothing in the checklist creates a mandatory artifact, blocking check, or invariant when the orchestrator skips follow-through.

   Cycle 188’s own journal entry shows the limit of the fix. Its new “Previous commitment follow-through” section explicitly records one commitment as “Not followed” (`docs/journal/2026-03-08.md:223-236`), which is better than silence but does not satisfy audit #147’s stronger goal of converting repeated observations into checklist steps, tracking issues, or dispatches. The change is worthwhile, but the cycle over-claimed it as an enforcement mechanism.

6. **Issue #761’s spec is internally contradictory about who determines merged PR linkage**
   Category: reconcile-spec-ambiguity

   The issue first says `cycle-complete` should determine whether an in-flight session has a merged PR “by checking if a PR exists with head branch containing the issue number, or by checking the issue state,” but then immediately says the tool “should NOT make API calls” and should instead accept reconciliation data through `--reconcile` arguments supplied by the orchestrator (EvaLok/schema-org-json-ld#761). Those are two different designs: one puts merge/linkage detection inside the tool, the other makes the tool a pure state applicator.

   The acceptance criteria only test the CLI-input path (“when `--reconcile` args are provided…”), so the ambiguous detection language is left unresolved. That matters because cycle 188 had already demonstrated that branch-name linkage is a weak proxy in this repository; carrying that ambiguity into the process-fix spec risks baking the same assumption into `cycle-complete`.

## Recommendations

1. Make cycle-close artifact generation atomic: either update the worklog/journal after the final merge/dispatch reconciliation or generate the closing summary directly from the same final state snapshot that writes `docs/state.json`.
2. Treat `field_inventory` refreshes as part of cycle-close, not optional cleanup. If a cycle changes or relies on `copilot_metrics.*` or `eva_input_issues.*`, their freshness markers should move in the same closing write.
3. Add a fresh-clone/shallow-history regression test for the `cycle-status` → `pipeline-check` path and decide the intended behavior when `publish_gate.validated_commit` is outside the shallow clone history.
4. Replace branch-token linkage in stale-issue suppression with explicit issue references (title/body links, state-backed reconciliation, or both) instead of assuming branch names contain issue numbers.
5. Tighten the audit #147 response so repeated journal commitments require a concrete artifact (dispatch, checklist step, tracking issue, or explicit rescission) rather than only a note that the commitment was missed.
6. Rewrite issue #761’s spec so it clearly chooses one design boundary: either the orchestrator supplies reconciliation facts, or `cycle-complete` derives them from local inputs — but not both.

## Complacency score

4/5 — Cycle 188 did some real follow-through: it reconciled stale state, merged four pending PRs, and responded to an audit finding with a checklist change. But the cycle still over-reported closure, let the final review artifacts drift behind the canonical state, and merged process/tooling changes whose real fresh-clone behavior was not re-checked before declaring success.

## Priority items

1. Fix the `cycle-status` / `pipeline-check` shallow-clone failure path so the clean-cycle gate behaves predictably in the actual Copilot environment.
2. Replace branch-name issue↔PR linkage with an explicit linkage method and update both `housekeeping-scan` and related specs accordingly.
3. Make cycle-close output generation atomic so worklog, journal, field-inventory freshness, and `docs/state.json` cannot disagree at the end of the same cycle.
