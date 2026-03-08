# Cycle 186 Review

## Findings

1. **Cycle 186 closed with a stale narrative because the review dispatch happened after the “final” worklog/state summary**
   Category: cycle-close-drift

   The committed worklog says cycle 186 ended with `203` total dispatches and `1` in-flight session (`docs/worklog/2026-03-08/062400-hundred-eighty-sixth-orchestrator-cycle.md:22-27,35-41`), and `docs/state.json` records the same snapshot in both `copilot_metrics` and `last_cycle.summary` (`docs/state.json:2010-2022,2237-2244`). But the orchestrator’s own closing comment on issue `#743` immediately reports `204 dispatches` and `2 in-flight` because `#748` (this review task) had already been dispatched, and master has the matching follow-up commit `7992fc14504579b940f3ad6cba6e09f52a0e42ef` (`https://github.com/EvaLok/schema-org-json-ld/issues/743#issuecomment-4018468340`).

   This is the same class of “state/story drift” the review process is supposed to catch. The metric labels are better than cycle 185, but the supposedly canonical end-of-cycle artifact was still obsolete before the cycle actually closed.

2. **Field-inventory freshness is still lagging behind what cycle 186 claims it checked**
   Category: freshness-cadence

   `docs/state.json` still marks `eva_input_issues.closed_this_cycle` and `eva_input_issues.remaining_open` as last refreshed in `cycle 184` (`docs/state.json:2107-2113`) even though cycle 186 explicitly reports the current remaining Eva directives in the worklog (`docs/worklog/2026-03-08/062400-hundred-eighty-sixth-orchestrator-cycle.md:35-41`) and the journal discusses the clean-cycle gate and directive follow-through (`docs/journal/2026-03-08.md:160-189`). The values themselves are plausible, but the freshness metadata says those fields have not been checked for two cycles.

   That makes the field inventory less trustworthy as a cadence control. The orchestrator is still doing the verification work informally, but not updating the mechanism that is supposed to prove it happened.

3. **PR #740 shipped the exact weak stale-issue heuristic that cycle 185’s review warned about**
   Category: stale-dispatch-heuristic

   The merged `housekeeping-scan` logic now suppresses any stale Copilot issue whenever *any* newer draft PR exists, with no issue↔PR linkage at all (`tools/rust/crates/housekeeping-scan/src/main.rs:136-140`). The tests explicitly lock that behavior in: `stale_agent_issue_excluded_when_any_newer_draft_pr_exists` asserts that one later draft PR is enough to hide the stale issue (`tools/rust/crates/housekeeping-scan/src/main.rs:488-499`).

   That is not just an imperfect heuristic; it is a broad false-negative rule. In a repository that regularly has multiple concurrent Copilot tasks, one unrelated draft PR can now make an older stalled issue disappear from housekeeping output. Cycle 186’s journal defends this as “good enough for our 2-concurrency model” (`docs/journal/2026-03-08.md:168-170`), but the code merged to master still weakens the detector instead of establishing a real association.

4. **Issue #746’s severity-tier spec downgrades all `cycle-status` findings even though that step can surface commit-freeze failures**
   Category: severity-tier-gap

   Audit `#144` was accepted for a good reason — `pipeline-check` really does fail today on warning-grade housekeeping noise (`bash tools/pipeline-check --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld` reproduced `Overall: FAIL` with only `housekeeping-scan` failing). But dispatched issue `#746` hard-codes the entire `cycle-status` step as `Warning`, described as purely advisory. That is too coarse.

   `cycle-status` does not only report concurrency and Eva-input counts. It also emits action items such as `Commit freeze check failed — could not verify QC-validated commit integrity` and `Source files changed since QC-validated commit — re-validation required` (`tools/rust/crates/cycle-status/src/main.rs:849-859`). A blanket “cycle-status = warning” rule would let those conditions pass through the clean-cycle gate as non-blocking noise. The spec should classify findings, or at minimum carve commit-freeze failures out as blocking.

5. **Audit handling was directionally correct, but the handoff/closure lifecycle is still sloppy enough to recreate housekeeping noise**
   Category: audit-handoff

   Audit `#144` was reasonably accepted, and audit `#145` is genuinely QC-directed based on its own text (`https://github.com/EvaLok/schema-org-json-ld-audit/issues/144`, `https://github.com/EvaLok/schema-org-json-ld-audit/issues/145`). The acknowledgment issues `#744` and `#745` are also formatted coherently. The problem is lifecycle discipline after creation: `housekeeping-scan --json` now reports both as stale audit-inbound issues, and those two findings are the entire reason the pipeline is still red.

   `#744` stayed open even after cycle 186 dispatched `#746`, which largely satisfied the “will dispatch this cycle” promise in the issue body. `#745` says the QC orchestrator “will need to process this recommendation via its own audit-inbound channel,” but I could not find a corresponding QC-repo handoff issue by searching for audit `#145` or the recommendation text in `EvaLok/schema-org-json-ld-qc`. So the classification was acceptable, but the transfer/closure mechanics were not fail-closed: the main repo kept two open audit-inbound issues and one of them does not appear to have an actual downstream handoff.

## Recommendations

1. Add a final cycle-close reconciliation step: dispatch the review task before writing the worklog/`last_cycle.summary`, or regenerate both artifacts after the review dispatch so the “final” narrative cannot lag behind the closing state.
2. Refresh field-inventory markers whenever a cycle explicitly checks unchanged values, especially `eva_input_issues.*`, `pre_python_clean_cycles`, and other cadence-governed state that is discussed in the worklog/journal.
3. Rewrite the stale-agent logic to require explicit issue↔PR linkage, not “any newer draft PR,” and update the tests to prove unrelated draft PRs do **not** suppress stale issues.
4. Amend issue `#746` so severity is per finding or otherwise preserves blocking treatment for `cycle-status` commit-freeze failures.
5. Close or transfer audit-ack issues as part of processing: close `#744` once the dispatch exists, and create a traceable QC-side inbound issue for audit `#145` before closing `#745`.

## Complacency score

4/5 — Cycle 186 did make a real process improvement by accepting audit `#144`, and the journal is not empty ritual. But it still normalized “close enough” state reporting, shipped a knowingly weak detector heuristic to master, and left audit handoffs open long enough to become the very housekeeping noise it was trying to demote.

## Priority items

1. Fix cycle-close synchronization so worklog/state summaries cannot be stale the moment the cycle closes.
2. Rewrite the `housekeeping-scan` stale-issue heuristic to use explicit linkage instead of “any newer draft PR.”
3. Tighten the severity-tier design and audit handoff lifecycle (`#746`, `#744`, `#745`) so warnings do not mask commit-freeze failures and processed audit acknowledgments do not linger open.
