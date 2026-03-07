# Cycle 168 Review

## Findings

1. **Cycle 168 closes with a false “all green” narrative: the repository currently fails 2 of the 9 state invariants.**  
   The worklog says `5/5 phases pass, 9/9 invariants` and the state summary repeats `Pipeline 5/5, 9/9 invariants` (`docs/worklog/2026-03-07/021500-hundred-sixty-eighth-orchestrator-cycle.md:31-34`, `docs/state.json:842-846`). But a fresh `cargo run -q -p state-invariants --manifest-path tools/rust/Cargo.toml -- --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld` fails on two checks: `review history accounting` and `copilot_metrics rates`. `cargo run -q -p pipeline-check --manifest-path tools/rust/Cargo.toml -- --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld --cycle 168` therefore also reports `Overall: FAIL`.

2. **The `dispatch_to_pr_rate` fix is still wrong in `state.json`, so the worklog overstates what was repaired.**  
   The worklog says the denominator was corrected and the current-state note says there are `94 dispatches ... 91 resolved` (`docs/worklog/2026-03-07/021500-hundred-sixty-eighth-orchestrator-cycle.md:21-23,31-34`). In `docs/state.json`, however, `dispatch_to_pr_rate` is still `89/94` while the adjacent note says `94 dispatches, 93 resolved, 1 in-flight` and `produced_pr` is `89` (`docs/state.json:631-640`). The invariant checker correctly flags this because the rate should be based on produced PRs over resolved sessions, i.e. `89/93`, not total dispatches.

3. **Cycle 167 review accounting was consumed into `state.json` incorrectly, which breaks the trend data the orchestrator is now relying on.**  
   The merged review file contains 5 findings (`docs/reviews/cycle-167.md:3-28`), but the latest `review_agent.history` entry records `finding_count: 17` with `actioned: 0`, `deferred: 0`, and `ignored: 0` (`docs/state.json:1299-1312`). That is exactly the `review history accounting` failure reported by `state-invariants`, and it means the cycle-168 worklog is claiming successful review consumption while the stored history is internally invalid.

4. **The shared I/O dedup story is ahead of the code: `process-eva` still carries private state read/write/commit helpers.**  
   Cycle 168 says the shared I/O refactor means “all five process-* tools” now use `state-schema::read_state_value`, `write_state_value`, and `commit_state_json` (`docs/journal/2026-03-07.md:25-27`) and the worklog says the only remaining shared-I/O task is `cycle-start` via `#626` (`docs/worklog/2026-03-07/021500-hundred-sixty-eighth-orchestrator-cycle.md:48-54`). But `process-eva` imports only `state_schema::set_value_at_pointer` and still defines local `read_state_value`, `write_state_value`, and `commit_state_json` functions (`tools/rust/crates/process-eva/src/main.rs:1-8,100-112,232-294`), even though the shared implementations already exist in `state-schema` (`tools/rust/crates/state-schema/src/lib.rs:93-109`). This is documentation drift and a real infrastructure gap, not just wording.

5. **`publish_gate` divergence tracking is stale even though the worklog says cycle 168 revalidated it.**  
   The current-state section says `No source divergence` and presents the publish gate as `FULLY CLEARED` for cycle 168 (`docs/worklog/2026-03-07/021500-hundred-sixty-eighth-orchestrator-cycle.md:31-34`). In `docs/state.json`, though, `publish_gate.last_divergence_check` is still `cycle 166` (`docs/state.json:853-860`). Raw inventory metrics are fine — `metric-snapshot` passes and the schema/test counts match the repository — so this is not a file-count problem. It is a traceability problem: the state record does not show the cycle-168 divergence check the narrative claims happened.

6. **The operator checklist is still materially out of sync with the tool-driven workflow that cycle 168 says is now the norm.**  
   `COMPLETION_CHECKLIST.md` still instructs the operator to manually edit `docs/state.json`, use `tools/commit-state-change`, and make manual commits for worklog/review dispatch steps (`COMPLETION_CHECKLIST.md:17-26,80-113,126-136`). That is inconsistent with the cycle-168 writeup, which says the write-side pipeline is functionally ready and explicitly lists `process-merge`, `process-review`, `process-audit`, `process-eva`, `record-dispatch`, and `cycle-complete` as the active path (`docs/worklog/2026-03-07/021500-hundred-sixty-eighth-orchestrator-cycle.md:36-54`). The next-step list acknowledges this drift, but the checklist is still the standing source of truth and remains misleading.

## Recommendations

1. Repair `docs/state.json` before the next cycle starts: fix `dispatch_to_pr_rate`, correct the cycle-167 `review_agent.history` entry to reflect the 5 actual findings, and re-run `state-invariants`/`pipeline-check` before claiming `9/9 invariants`.
2. Finish the shared-I/O migration for `process-eva` (and any other remaining holdouts), then tighten worklog/journal language so “all tools use shared helpers” is only said when the code actually matches it.
3. Update `COMPLETION_CHECKLIST.md` to the current tool-first workflow and add a last-step verification rule: after writing worklog/journal/state summaries, re-run the checks that support any “all green” claim.
4. Treat `publish_gate.last_divergence_check` as evidence, not prose: if the worklog says divergence was checked this cycle, the field should move this cycle or the worklog should explicitly say the prior check was reused.

## Complacency score

3/5 — there is real improvement here, especially in tooling coverage and inventory hygiene, but cycle 168 still shows “going through the motions” behavior in the final mile: the narrative claims closure before the state and validations actually support it.

## Priority items

1. Make `state-invariants` pass again by fixing the broken review-history entry and the still-wrong `dispatch_to_pr_rate`.
2. Reconcile the shared-I/O story with reality by migrating `process-eva` and updating the documentation/checklist to match the actual workflow.
3. Refresh publish-gate/divergence evidence in `docs/state.json` whenever the worklog claims a fresh no-divergence check.
