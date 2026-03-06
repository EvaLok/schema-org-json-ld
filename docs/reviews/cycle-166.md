# Cycle 166 Review

## Complacency score

2/5 — execution quality was strong and the key cycle-166 integrity concerns (metrics math and review history consistency) were handled carefully, but freshness bookkeeping and write-tool deduplication still lag behind the level of rigor used elsewhere.

## Number of findings

4

## Findings

1. **Category:** metrics-accuracy  
   **Severity:** low  
   **Description:** `copilot_metrics` arithmetic is internally consistent after the manual `produced_pr` correction.  
   **Evidence:** `docs/state.json:631-643` (`resolved=88`, `in_flight=1`, `total_dispatches=89`, `merged=83`, `closed_without_merge=1`, `closed_without_pr=4`, `produced_pr=84`) which satisfies `resolved + in_flight = total_dispatches` (`88 + 1 = 89`) and `merged + closed_without_merge + closed_without_pr = resolved` (`83 + 1 + 4 = 88`).

2. **Category:** review-accounting  
   **Severity:** low  
   **Description:** The cycle-165 review history entry is coherent with the underlying review artifact, including the manual actioned/deferred/ignored annotation.  
   **Evidence:** `docs/reviews/cycle-165.md:7-35` (5 findings with categories), `docs/state.json:1269-1283` (`finding_count=5`, `actioned=1`, `deferred=2`, `ignored=2`, and matching category list), with `1 + 2 + 2 = 5`.

3. **Category:** state-freshness  
   **Severity:** medium  
   **Description:** Multiple fields updated in cycle 166 are still marked as last refreshed in older cycles, which weakens the value of field-inventory freshness checks.  
   **Evidence:** Updated values exist at `docs/state.json:631-643` (`copilot_metrics`),  
   `docs/state.json:842-847` (`last_cycle`),  
   `docs/state.json:850-851` (`open_questions_for_eva`), and  
   `docs/state.json:853-856` (`publish_gate`).  
   Freshness markers for those fields remain older at `docs/state.json:706-747`  
   (`copilot_metrics.*` last refreshed cycle 164/165, `last_cycle` cycle 164,  
   `open_questions_for_eva` cycle 160, `publish_gate` cycle 164).

4. **Category:** tool-quality  
   **Severity:** medium  
   **Description:** `process-merge` and `record-dispatch` follow core fail-closed patterns and tests pass, but they duplicate key state I/O and commit helper logic instead of sharing a common utility layer, increasing drift risk.  
   **Evidence:** Duplicate `read_state_value`/`write_state_value`/`commit_state_json` patterns  
   appear in `tools/rust/crates/process-merge/src/main.rs:74-86,241-293` and  
   `tools/rust/crates/record-dispatch/src/main.rs:69-81,213-263`.  
   The shared crate `tools/rust/crates/state-schema/src/lib.rs:90-155` currently provides  
   pointer/freshness helpers but not shared state file/commit helpers.

## Recommendations for next cycle

1. Refresh the `last_refreshed` attribute for each touched entry in `field_inventory.fields` during cycle-close updates (especially `copilot_metrics`, `last_cycle`, `open_questions_for_eva`, and `publish_gate`).
2. Extract shared Rust utility functions for `docs/state.json` read/write and state commit receipt handling into a common crate/module (likely `state-schema` or a sibling utility crate), then consume from `process-merge`, `record-dispatch`, and similar tools.
3. Keep the current invariant checks that caught the `produced_pr` drift, and add one explicit guardrail for freshness-marker updates so drift is caught automatically during write-side tool operations.

## Priority items

1. Fix stale `field_inventory` freshness markers for cycle-166-touched fields.
2. Deduplicate state I/O + commit helper code across write-side tools to prevent behavior drift.
