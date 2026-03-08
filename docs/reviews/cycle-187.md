# Cycle 187 Review

## Findings

1. **Cycle 187 never produced a reconciled closing snapshot**
   Category: cycle-close-drift

   The supposed end-of-cycle state is internally contradictory before considering any later activity. `docs/state.json` says `last_cycle.number` is `187`, but its `last_cycle.summary` still describes the previous cycle’s merges and dispatches (`docs/state.json:2255-2260`). The same file also leaves issues `#746` and `#748` marked `in_flight` in `agent_sessions` (`docs/state.json:1871-1884`) even though cycle 187 separately recorded merges for PRs `#747` and `#749` and the worklog lists both as merged (`docs/worklog/2026-03-08/074200-hundred-eighty-seventh-orchestrator-cycle.md:31-34`).

   This is not just stale prose. `cargo run -q -p state-invariants --manifest-path tools/rust/Cargo.toml -- --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld` currently fails the `agent_sessions reconciliation` invariant because `agent_sessions` still imply 4 in-flight sessions / 196 merged PRs while `copilot_metrics` claims 2 in-flight / 198 merged. The cycle closed without a canonical state write that matched the recorded events.

2. **The worklog’s “canonical from state.json” metrics do not match state.json**
   Category: metric-label-drift

   The worklog explicitly labels its dispatch counts as “canonical from state.json” and records `Resolved: 202` / `In-flight: 2` (`docs/worklog/2026-03-08/074200-hundred-eighty-seventh-orchestrator-cycle.md:24-29`). But `docs/state.json` currently says `resolved: 204` and `in_flight: 2` in `copilot_metrics` (`docs/state.json:2031-2043`), while the underlying `agent_sessions` entries still show four `in_flight` issues (`docs/state.json:1871-1898`).

   So the cycle repeated the exact class of “state/story drift” that cycle 186’s review warned about, only in a worse form: the narrative is not merely stale after a later dispatch, it is already citing a canonical source that disagrees with itself.

3. **Field-inventory cadence is still lagging behind fields cycle 187 clearly relied on**
   Category: freshness-cadence

   Several cadence-governed fields that cycle 187 discussed or updated were not refreshed in `field_inventory`. `eva_input_issues.remaining_open` is still marked `cycle 184`, `last_cycle.duration_minutes` is still marked `cycle 186`, and `pre_python_clean_cycles` is still marked `cycle 185` (`docs/state.json:2129-2155`) even though the worklog discusses remaining directives, cycle duration, and the clean-cycle gate as current-cycle facts (`docs/worklog/2026-03-08/074200-hundred-eighty-seventh-orchestrator-cycle.md:39-42`).

   That undermines the whole “check unchanged values and record freshness” discipline. The orchestrator is still narrating these fields as if they were verified this cycle, but the metadata that is supposed to prove verification did not move.

4. **The journal and worklog take a premature victory lap on clean cycles**
   Category: premature-clean-cycle-claim

   The cycle 187 journal says the severity-tier work “works exactly as designed” and that the pipeline now reports `Overall: PASS (1 warning)` so clean-cycle counting can begin (`docs/journal/2026-03-08.md:193-201`). The worklog repeats that the pipeline is `PASS` and that “next cycle can count as first clean cycle” (`docs/worklog/2026-03-08/074200-hundred-eighty-seventh-orchestrator-cycle.md:38-42`).

   But the canonical state does not show any actual clean-cycle advancement: `pre_python_clean_cycles.count` is still `0` and its freshness marker is still `cycle 185` (`docs/state.json:2153-2155,2264-2268`). Worse, the current repo reproduction of `bash tools/pipeline-check --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld` fails because `state-invariants` is red. The reflection is not empty filler, but it overstates success before the state and gate bookkeeping are actually consistent.

5. **PR #747 still masks commit-freeze failures behind a warning-only step**
   Category: severity-tier-gap

   The merged severity-tier implementation classifies all `cycle-status` findings as `Severity::Warning` (`tools/rust/crates/pipeline-check/src/main.rs:318-324`), and `status_from_exit_code` converts exit code `1` from a warning step into `StepStatus::Warn`, not `Fail` (`tools/rust/crates/pipeline-check/src/main.rs:327-335`). But `cycle-status` is not purely advisory: it emits commit-freeze action items such as “Commit freeze check failed” and “Source files changed since QC-validated commit” (`tools/rust/crates/cycle-status/src/main.rs:849-858`), and its human report already distinguishes `CHECK FAILED` / `DIVERGED` states (`tools/rust/crates/cycle-status/src/main.rs:1032-1045`).

   That means the merged severity model is still too coarse for the actual risk model. The test coverage added in `pipeline-check` exercises only the successful `cycle-status` summary path (`tools/rust/crates/pipeline-check/src/main.rs:450-465`) plus generic exit-code mapping, so there is no end-to-end guard proving commit-freeze failures would block the clean-cycle gate.

6. **Issue #751’s spec does not require the consumer-side change needed to make commit-freeze truly blocking**
   Category: incomplete-issue-spec

   The issue correctly identifies that `cycle-status` should exit non-zero on commit-freeze failures, but its own “Why this matters” section explicitly frames that as a prerequisite for some later refinement rather than a complete fix. As long as `pipeline-check` still treats `cycle-status` as warning severity (`tools/rust/crates/pipeline-check/src/main.rs:318-335`), changing `cycle-status` to exit `1` will only turn those cases into `WARN`, not `FAIL`.

   The spec therefore leaves a design gap in the acceptance criteria: it asks for `cargo test -p cycle-status` and `cargo clippy -p cycle-status`, but nothing that proves the clean-cycle gate would actually fail on commit-freeze divergence after the change lands. The next cycle should either expand the issue or immediately pair it with a pipeline consumer change.

7. **Issue #753 identifies the right bug, but its linkage spec is still weaker than existing repository practice**
   Category: linkage-spec-gap

   The current `housekeeping-scan` bug is real: it suppresses stale issues whenever *any* newer draft PR exists (`tools/rust/crates/housekeeping-scan/src/main.rs:136-140`), and the tests explicitly lock that behavior in (`tools/rust/crates/housekeeping-scan/src/main.rs:488-499`). Issue `#753` correctly rejects that rule, but its preferred solution still centers branch-name heuristics (“contains the issue number as a token”) and only treats PR-body linkage as an alternative.

   That is weaker than the codebase already knows how to do. `cycle-status` already matches issue references with title/body parsing that avoids `#746` vs `#7460` false positives and recognizes `Fixes`/`Closes`/`Resolves` references (`tools/rust/crates/cycle-status/src/main.rs:474-499,517-526`). The spec should have required reuse or parity with that stronger linkage logic instead of inviting another custom heuristic.

## Recommendations

1. Run a real cycle-close reconciliation step before writing the review artifacts: update `last_cycle.summary`, `agent_sessions`, `copilot_metrics`, and the worklog from the same final state snapshot, then verify `state-invariants` passes.
2. Treat `field_inventory` refreshes as mandatory whenever a worklog/journal claim depends on unchanged state, especially for `eva_input_issues.*`, `last_cycle.duration_minutes`, and `pre_python_clean_cycles`.
3. Close the severity-tier gap end-to-end: either make commit-freeze failures blocking in `pipeline-check` now or pair `#751` with a follow-up issue that is explicitly required for completion.
4. Strengthen `#753` before implementation by requiring explicit issue↔PR linkage using the same whole-issue matching rules already proven in `cycle-status`, plus tests for unrelated draft PRs and body-only references.
5. Stop calling the clean-cycle counter “ready to start” until both the pipeline result and `pre_python_clean_cycles` state have been updated in the same cycle.

## Complacency score

4/5 — Cycle 187 did respond to review feedback and dispatched the right general areas of work, so this was not pure box-ticking. But it still declared success before the canonical state was reconciled, left invariant failures in the closing snapshot, and wrote follow-up specs that stop one step short of the real gate behavior.

## Priority items

1. Reconcile `docs/state.json` and cycle-close generation so `last_cycle`, `agent_sessions`, and `copilot_metrics` cannot disagree after merges and dispatches.
2. Fix the clean-cycle gate end-to-end so commit-freeze failures cannot be downgraded to warnings by step-level severity.
3. Rewrite stale issue↔PR linkage around explicit references instead of branch-age or branch-name heuristics, and refresh the corresponding tests.
