# Cycle 221 Review

## Findings

1. **The merged cycle-221 documentation artifacts are already stale relative to the committed state they are supposed to summarize**
   Category: stale-documentation-snapshot

   **Files**: `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-10/183000-cycle-221-summary.md:36-47`; `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-10.md:371-375`; `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:2791-2812`; `/home/runner/work/schema-org-json-ld/schema-org-json-ld/AGENTS.md:402-412`  
   **Evidence**: The worklog and journal were authored from the `doc_dispatched` snapshot: they say there is still 1 in-flight session and that the next job is to review PR `#994`. But the current committed `docs/state.json` already shows `in_flight = 0`, `doc_pr = 995`, and `cycle_phase.phase = "close_out"`. That means both artifacts were merged after the state advanced, without regenerating the “Current state,” “Next steps,” or “Concrete commitments” sections the documentation agent is supposed to derive from committed state.  
   **Why it matters**: This is not just normal historical drift inside an old worklog. These files are the first proof point for the new phased documentation architecture, and they landed on `master` already out of sync with the ledger they are meant to summarize.

2. **Cycle 221 marked `worklog-accuracy` as structurally verified before the new architecture had actually passed its first review**
   Category: premature-verification

   **Files**: `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:3141-3145`; `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-10.md:357-365`; `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:4312-4315`  
   **Evidence**: `docs/state.json` sets `review_agent.chronic_category_responses.entries[*].verification_cycle = 221` for `worklog-accuracy`. The same journal entry explicitly admits the prior commitment was “not followed as written” and that cycle 221 set the verification cycle “before this worklog existed.” Cycle 220’s review had already warned against declaring the structural fix complete before the full path was covered, yet cycle 221 still recorded verification before the first documentation-agent artifact had been reviewed and merged cleanly.  
   **Why it matters**: A structural fix is not verified because it sounds stronger on paper. It is verified only after the first real artifact survives the exact path that used to fail. Cycle 221 claimed that proof too early.

3. **`docs/state.json` still carries a stale `test_count`, and the field inventory now gives it a false freshness signal**
   Category: state-metric-drift

   **Files**: `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:3001-3003`; `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:4791-4796`  
   **Evidence**: `field_inventory.fields.test_count.last_refreshed` says `cycle 221`, but `test_count` still reports `php = 425`, `ts = 419`, `total = 844` with `last_verified = cycle 139`. Comparing against the repository’s actual test-file counts via `ls /home/runner/work/schema-org-json-ld/schema-org-json-ld/php/test/unit | wc -l` and `ls /home/runner/work/schema-org-json-ld/schema-org-json-ld/ts/test/schema | wc -l` yields `90` and `90`. So either the state entry is stale if it is meant to track file counts, or the field name/cadence now obscures that it is actually tracking something else, such as individual test cases.  
   **Why it matters**: Either interpretation is a state-quality problem. The metric is currently not self-describing enough to be checked against repository reality, yet cycle 221 still refreshed the inventory marker as if the value had been meaningfully re-verified.

4. **The journal notices the missed verification step, but then narrates around it instead of extracting a guardrail**
   Category: reflection-gap

   **Files**: `/home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-10.md:357-365`  
   **Evidence**: The journal correctly says the previous commitment was not followed as written, but the very next section reframes that miss as a “Decision” to treat the phased documentation architecture as the structural fix. The entry does not ask what concrete proof was still missing, what should block `verification_cycle` from advancing, or how the new architecture will avoid being declared “verified” before review evidence exists.  
   **Why it matters**: That is a complacency pattern, not just a style preference. The journal is adding less value when it turns a failed verification into a narrative of progress without also adding a new mechanical safeguard.

## Recommendations

1. Reopen `worklog-accuracy` verification: reset or qualify the chronic-response entry until a later cycle reviews a documentation-agent artifact that is still aligned with the final committed state.
2. Make the documentation path fail closed when state advances after the docs PR is generated: either regenerate the worklog/journal from the latest tip before merge, or block merge when `check-doc-pr` detects that `docs/state.json` has moved from the snapshot the docs were derived from.
3. Recompute `test_count` from a documented source of truth, or rename/document the field so it no longer looks like a file count. Do not mark `field_inventory.fields.test_count` refreshed unless the underlying metric has actually been re-measured.

## Complacency score

4/5 (where 5 = “going through motions”) — cycle 221 did real work: it merged substantive tool changes, consumed the missing cycle-220 review, and actually used the new phased documentation flow. But it also repeated a core bad habit from prior reviews: treating architecture-level intent as proof, merging documentation that was already stale relative to the final state, and refreshing state metadata without re-establishing that the underlying numbers are trustworthy.

## Priority items

1. Stop treating `worklog-accuracy` as verified until the documentation-agent path has passed at least one full end-to-end review without stale output.
2. Add a merge-time freshness check for cycle docs so worklog/journal “current state” and “next steps” cannot land behind `docs/state.json`.
3. Repair or redefine `test_count` and its field-inventory contract so state metrics match observable repository reality.
