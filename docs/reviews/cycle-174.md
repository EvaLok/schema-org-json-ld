# Cycle 174 Review

I rechecked the concrete areas called out in the issue. The receipt trail is solid: all listed SHAs exist and match the claimed actions (`7207dad`, `8a8bef5`, `308fc9b`, `27a12a1`, `6776a96`, `6566253`, `c9d8afd`, `0374812`, `b4bed9d`).

The current `docs/state.json` is mechanically consistent on the requested math checks. `copilot_metrics` now shows `107` dispatches, `107` resolved, `0` in flight, `104` produced PRs, `103` merged, and `1` closed without merge (`docs/state.json:632-644`).

The issue-tracking fields also line up: `eva_input_issues` correctly records `586` and `591` as closed this cycle with only `247` and `436` remaining open (`docs/state.json:646-695`), the touched freshness markers for `copilot_metrics.*`, `eva_input_issues.*`, `last_cycle`, `last_eva_comment_check`, `publish_gate`, and `review_agent` are all at cycle 174 (`docs/state.json:708-766,857-865`), and the cycle-173 review history entry now correctly shows `actioned=1` and `deferred=1` (`docs/state.json:1386-1399`).

The main remaining concerns are not arithmetic or receipts; they are claim precision and whether the closure language got slightly ahead of the implementation.

## Findings

1. **The checklist consolidation is materially better, but the cycle-start / write-side closure claims still overstate one remaining state-ownership gap.**  
   Category: cycle-start-scope-drift  
   The new step 0 is much clearer than cycle 173’s duplicated startup flow: it now makes `bash tools/cycle-start --issue {NUMBER}` the single entry point and explicitly forbids a separate manual opening comment (`STARTUP_CHECKLIST.md:9-25`). That genuinely addresses the reviewed duplicate-comment behavior from cycle 173. But the same checklist step also says `cycle-start` “refreshes `open_questions_for_eva` and sets freshness markers” (`STARTUP_CHECKLIST.md:17-23`), and the implementation does not actually do that. `cycle-start` gathers open questions for the startup brief (`tools/rust/crates/cycle-start/src/main.rs:140-149`), but its state patch only updates `last_cycle`, `last_eva_comment_check`, and those two freshness markers (`tools/rust/crates/cycle-start/src/main.rs:204-230`). That leaves a loose end under both closed directives: #591’s “startup sequence handled by cycle-start” claim is accurate for the comment + brief path, but not for every state side effect the checklist now attributes to it; and #586’s stronger “the tools now handle every state.json mutation” framing is still a little too absolute while this field lacks an evident write-side owner in the reviewed tool flow.

2. **Directive #591 is functionally satisfied, but the closing comment rounds “derive from state.json” up to “all tools use the shared helper,” which the codebase has not fully standardized yet.**  
   Category: cycle-derivation-standardization  
   The functional outcome is good: the relevant tools now derive their cycle from `docs/state.json`, the workspace tests pass, and the current cycle-174 state is consistent. But the closing comment on #591 says all tools now do this “via `current_cycle_from_state()`,” while several implementations still read `/last_cycle/number` themselves instead of going through that shared helper—for example `process-review` (`tools/rust/crates/process-review/src/main.rs:405-410`), `record-dispatch` (`tools/rust/crates/record-dispatch/src/main.rs:70-77`), `process-merge` (`tools/rust/crates/process-merge/src/main.rs:89-104`), and `cycle-start`’s own cycle derivation (`tools/rust/crates/cycle-start/src/main.rs:190-201`). This is not causing a live bug right now, so I would not reopen the directive on functionality alone. But it is exactly the sort of “process language got a little cleaner than the code” drift that the cycle-174 journal itself warns about when it names procedural inertia and layered process habits as recurring risks (`docs/journal/2026-03-07.md:135-140`).

## Recommendations

1. Either teach `cycle-start` (or another explicit tool) to write `open_questions_for_eva` and its freshness marker, or narrow the checklist / directive-closing language so it only claims the side effects the tool actually performs.
2. Normalize the remaining bespoke cycle readers onto `current_cycle_from_state()` if that helper-level standardization is important, or else rewrite the closure note to the more accurate “all tools derive from state.json.”
3. Keep the current receipt discipline and invariant/testing habit; it made this cycle unusually easy to verify and is the strongest evidence that the infrastructure improvements are real.

## Complacency score

3/5 — better than the long-running baseline, but not quite as clean as the worklog’s “complete” tone suggests. The journal is genuinely reflective and names the real pattern risk (`docs/journal/2026-03-07.md:131-143`), the receipt trail is concrete, and the state math is clean. The complacency signal is narrower: the cycle rounded two “mostly there” infrastructure claims into “done” language before the last bits of checklist/tool alignment were fully nailed down.

## Priority items

1. Close the `open_questions_for_eva` ownership gap so startup/state bookkeeping is fully tool-driven rather than only mostly tool-driven.
2. Standardize the remaining cycle readers on the shared helper, or tighten the wording of the closed-issue claims to match the actual implementation.
3. Recheck one more cycle of #586 / #591 behavior before treating these closures as settled precedent for future infrastructure reviews.
