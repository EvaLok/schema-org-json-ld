# Cycle 171 Review

I rechecked the concrete items called out in the issue. The directive #642 cleanup is real: `docs/journal/2026-03-07.md` no longer contains bare issue references, and `JOURNAL.md` now correctly lists 2026-03-06 as cycles 151–166 and 2026-03-07 as the active 167+ file (`JOURNAL.md:5-16`). The core cycle-171 state updates are also correct: `copilot_metrics` rates match the underlying counts, the latest `review_agent.history` entry records 3 findings with the expected categories, and `publish_gate.last_divergence_check` is now `cycle 171` (`docs/state.json:705-747`, `docs/state.json:1346-1360`).

## Findings

1. **The "cycle 172" sequencing drift is already persisted in state, so it is not just a cosmetic commit-label problem.**  
   Category: cycle-label-state-drift  
   The worklog describes the `process-eva` cycle label as a known sequencing quirk in the commit message (`docs/worklog/2026-03-07/060900-hundred-seventy-first-orchestrator-cycle.md:52-53`), but `docs/state.json` already contains two freshness markers stamped `cycle 172` even though `last_cycle` is still `cycle 171`: `eva_input_issues.closed_this_cycle` and `eva_input_issues.remaining_open` are both ahead of the active cycle (`docs/state.json:717-727`). That means the off-by-one logic has escaped presentation and is now polluting persisted bookkeeping. The current cycle is internally "green" only because no invariant appears to reject future-cycle freshness markers.

2. **The manual `process-review` repair restored the recorded history, but this is the same broad parser-fragility class as cycle 164 and should be treated as a recurring trust failure.**  
   Category: review-parser-regression  
   The end-state correction itself is adequate: `review_agent.history` now correctly records cycle 170 as 3 findings with the intended categories (`docs/state.json:1346-1360`). But the failure pattern is not new. In cycle 164, the first `process-review` bug came from naive markdown parsing that matched evidence text and read `162` instead of the actual finding count `5` (`docs/worklog/2026-03-06/194300-hundred-sixty-fourth-orchestrator-cycle.md:16`, `:64`). Cycle 165 shipped a targeted fix for that exact heuristic (`docs/worklog/2026-03-06/210800-hundred-sixty-fifth-orchestrator-cycle.md:17-28`), yet cycle 171 still reports `169` findings and junk categories like `crates`, `main-rs`, and `src` from file-path evidence (`docs/worklog/2026-03-07/060900-hundred-seventy-first-orchestrator-cycle.md:49-50`, `docs/journal/2026-03-07.md:85-87`). So this is the same overall error class—over-trusting unstructured review markdown with brittle heuristics—even if the exact trigger changed. The cycle is right to dispatch #645, but the process should treat future parser output as untrusted until that fix proves itself on real review files, ideally by replaying it against a small corpus of historical reviews before re-enabling automatic state writes.

## Recommendations

1. Fix every cycle-derivation path to use `current_cycle_from_state()` semantics and add an invariant that rejects any `field_inventory.last_refreshed` value greater than `last_cycle.number`.
2. After #645 lands, validate `process-review` against a corpus of real review files that include evidence blocks, file paths, numbered lists, and explicit `Category:` lines before trusting it to write state without manual inspection.
3. Keep the current discipline on journal links and index maintenance, but make the tool path the default: use `write-entry`/`cycle-start` instead of manual edits so "tool exists but was bypassed" stops recurring.

## Complacency score

3/5 — the cycle is mostly honest about what went well and what failed, and the journal observations are genuinely reflective rather than filler. But it still normalized two recurring process smells: trusting brittle parser output long enough to need another manual repair, and treating an off-by-one cycle write as cosmetic when it already leaked into `state.json`.

## Priority items

1. Correct the `process-eva` cycle derivation bug and clean any future-cycle freshness markers back to the real current cycle.
2. Prove the `process-review` fix on actual historical review files before using it as an authoritative writer again.
3. Close the adoption gap for existing tools (`write-entry`, `cycle-start`) so manual edits stop bypassing the safeguards that already exist.
