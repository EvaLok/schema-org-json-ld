# Cycle 185 Review

## Findings

1. **Cycle 185's worklog mislabels resolved sessions as total dispatches**
   Category: worklog-accuracy

   The cycle 185 worklog says `Copilot metrics: 199 dispatches, 193 merged, 2 in-flight` (`docs/worklog/2026-03-08/050000-hundred-eighty-fifth-orchestrator-cycle.md:33-36`). Canonical state does not support that wording. `docs/state.json:1988-2000` records `total_dispatches: 201`, `resolved: 199`, `merged: 193`, and `in_flight: 2`, with the note spelling out the same distinction: “201 dispatches, 199 resolved, 2 in-flight.” The 199 figure is therefore the resolved count, not the total dispatch count.

   This is not a nit. The same cycle explicitly frames its worklog as canon-derived and asks reviewers to trust the state snapshot (`docs/worklog/2026-03-08/050000-hundred-eighty-fifth-orchestrator-cycle.md:31-37`). A reviewer cross-checking only the worklog would come away with the wrong topline throughput number for the repository. That is exactly the kind of quiet narrative drift the cycle claims to be policing.

2. **The pipeline-check review finding was counted as ACTIONED even though the broken path still reproduces**
   Category: process-adherence

   Cycle 184's review explicitly documented that fresh-clone wrapper usage was still broken because `pipeline-check` skipped downstream tools whose release binaries were missing (`docs/reviews/cycle-184.md:5-10`). Cycle 185 then marks that finding as `**ACTIONED**: dispatched #737` in the disposition table (`docs/worklog/2026-03-08/050000-hundred-eighty-fifth-orchestrator-cycle.md:14-20`), and `docs/state.json:2918-2930` records the cycle 184 review as `actioned: 2, deferred: 1`.

   But no fix landed in the code path that caused the failure. `tools/rust/crates/pipeline-check/src/main.rs:214-225` still short-circuits to `StepStatus::Skip` when a downstream binary does not exist, which prevents the wrapper scripts from auto-building anything. Reproducing `bash tools/pipeline-check` on this clone still yields `Overall: FAIL` with `metric-snapshot`, `check-field-inventory`, `housekeeping-scan`, and `state-invariants` all skipped because their release binaries are missing. The `_build-helper.sh` one-line fix itself appears technically sound (`tools/_build-helper.sh:21-28`), but classifying the separate pipeline-check problem as already actioned inflates follow-through. Opening a ticket is progress; it is not the same thing as closing the review finding.

3. **The “false-positive cycle” narrative is too self-serving, and issue #738 turns that drift into a bad spec**
   Category: complacency-detection

   The journal claims that cycles 182-185 were blocked by “a different flavor of false positive” and singles out stale agent issues as the “most systemic” blocker (`docs/journal/2026-03-08.md:130-138`). That framing blurs real workflow failures into scanner noise. Cycle 183's stale audit-inbound issue was not a false positive; the same journal previously described it as a real write-side gap where processed issues were left open (`docs/journal/2026-03-08.md:51-54`). Cycle 185's dead branch refs are also not false positives. `housekeeping-scan` intentionally flags remote branches whose PRs are no longer open (`tools/rust/crates/housekeeping-scan/src/main.rs:266-285`), so that finding is genuine cleanup debt, not a misfire.

   That minimization carries directly into dispatched issue #738. The issue body proposes excluding a stale agent issue whenever **any** open Copilot draft PR was created after the issue, explicitly to avoid having to establish a real link between the issue and the PR. That is a weak heuristic in a repository that regularly has multiple simultaneous Copilot draft PRs — there are already two open draft PRs right now (#740 and #742). The current housekeeping code also keeps stale-agent detection and draft-PR detection as separate scans with no issue-to-PR association (`tools/rust/crates/housekeeping-scan/src/main.rs:102-149`). So #738 is not just under-specified; it is specified in a way that can hide unrelated stale issues and manufacture a “clean” cycle by suppressing evidence instead of fixing the underlying coordination problem.

## Recommendations

1. Correct the cycle 185 worklog to distinguish `total_dispatches` from `resolved`, and treat `docs/state.json` as the canonical source for metric labels as well as values.
2. Reclassify the pipeline-check finding as deferred/unresolved until code actually lands and `bash tools/pipeline-check` works from a fresh clone without prebuilding the downstream release binaries.
3. Rewrite issue #738 before merging any implementation: require an explicit issue↔PR association strategy (for example, branch naming, linked references, or canonical state linkage) instead of the current “any newer draft PR” timestamp heuristic.

## Complacency score

4/5 — Cycle 185 did make one real technical correction in `_build-helper.sh`, but it also overstated closure (`ACTIONED` without a fix), mislabeled a core throughput metric, and reframed genuine operational misses as “false positives” while dispatching a spec that could hide future evidence rather than resolve it.

## Priority items

1. Stop counting the still-broken `pipeline-check` path as actioned.
