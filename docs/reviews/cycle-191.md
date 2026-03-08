# Cycle 191 Review

## Findings

1. **Cycle 191 called finding #3 “actioned” even though the repository stayed fail-open**
   Category: disposition-overstatement

   The worklog records cycle 190 finding #3 as **ACTIONED** because issue `#776` was dispatched (`docs/worklog/2026-03-08/134500-hundred-ninety-first-orchestrator-cycle.md:8,21`), and `docs/state.json` repeats that framing in the review history note (`docs/state.json:3135-3139`). But none of the cycle 191 code receipts touched `tools/rust/crates/cycle-status/src/main.rs`; the only code change was `961af98`, which modified `cycle-complete`. At cycle end, `cycle-status` still used `is_pre_publish_gate_status()` and still routed null/unknown `publish_gate.status` through the non-blocking path (`tools/rust/crates/cycle-status/src/main.rs:546-560,875-898`).

   Even the review-issue closure comment used the weaker disposition “**ACCEPTED**: dispatching fail-closed fix to Copilot” rather than claiming the bug was fixed in-repo (issue `#773` comment `4019088711`, 2026-03-08T13:50:00Z). Dispatching follow-up work is real progress, but it is not the same as the reviewed finding being actioned in the codebase.

2. **`copilot_metrics` smooths over a real session state and no longer matches the ledger cleanly**
   Category: copilot-metrics-drift

   The `copilot_metrics` block says 209 sessions produced PRs and that those PR-producing sessions break down into 207 merged and 2 closed without merge (`docs/state.json:2106-2114`). But the session ledger still contains a third PR-bearing non-merged state: issue `#303` / PR `#305` is `reviewed_awaiting_eva` (`docs/state.json:742-749`). Meanwhile two sessions are marked `merged` with `pr: null` (`docs/state.json:1266-1283`), which means the rate fields are not a simple count over `pr != null` either.

   The headline counts used in the worklog (214 dispatches, 213 resolved, 207 merged, 1 in-flight) do reconcile, but the “canonical” note/rates in `copilot_metrics` are hand-shaped enough to hide one live PR state entirely. That is exactly the kind of drift `state.json` is supposed to prevent.

3. **The “review agent was right” reflection still cashes out as “consider it later”**
   Category: performative-self-criticism

   Cycle 191’s journal says the cycle 190 race finding was “the most important” and admits the earlier invalidation was self-protective (`docs/journal/2026-03-08.md:333-337`). But the operational follow-through is still vague. The worklog marks the finding deferred (`docs/worklog/2026-03-08/134500-hundred-ninety-first-orchestrator-cycle.md:10,19`), the next-steps list says only “Consider structural fix” (`docs/worklog/2026-03-08/134500-hundred-ninety-first-orchestrator-cycle.md:50-53`), and the journal commitment is only “Design structural fix for the review/artifact race” next cycle (`docs/journal/2026-03-08.md:347-351`).

   That falls short of the repository’s own reconciliation rule: repeated commitments need a checklist step, tracking issue, Copilot dispatch, or explicit rescission; “noted for future” does not count (`STARTUP_CHECKLIST.md:63-72`). The closing cycle summary on issue `#775` omitted the race entirely from next-cycle priorities (issue `#775` comment `4019104729`, 2026-03-08T14:01:33Z), which makes the self-criticism look more performative than behavioral.

4. **The new semantic freshness test still leaves part of the event-driven contract unguarded**
   Category: partial-semantic-test

   Cycle 191 correctly added the three `copilot_metrics.*` fields to `EVENT_DRIVEN_AUTO_REFRESH_FIELDS`, and those fields do match the field inventory (`tools/rust/crates/cycle-complete/src/main.rs:133-148`; `docs/state.json:2187-2197`). But the new semantic test is still only a partial list. The constant includes `review_agent.chronic_category_responses` (`tools/rust/crates/cycle-complete/src/main.rs:138`), and the inventory tracks that path (`docs/state.json:2251-2253`), yet the test’s `required_fields` array omits it (`tools/rust/crates/cycle-complete/src/main.rs:1057-1071`).

   So the immediate omission from cycle 190 was fixed, but the new test still does not fully protect the event-driven freshness contract. The worklog/journal claim that there is now a semantic guard is directionally true, but still stronger than what the test actually enforces.

## Recommendations

1. Reserve **ACTIONED** for findings whose fix landed in the repository during the reviewed cycle; use a separate label for “accepted and dispatched” work.
2. Derive `copilot_metrics` from explicit `agent_sessions` status buckets, including `reviewed_awaiting_eva` and merged-without-PR cases, instead of hand-maintaining rate strings.
3. Create a concrete vehicle for the review/artifact race fix (issue, dispatch, or direct `cycle-complete` change) rather than carrying it as reflective intent.
4. Strengthen the semantic freshness test so it asserts the full expected event-driven field set, not just a curated subset.

## Complacency score

3/5 — Cycle 191 is noticeably better than cycle 190 in one important way: it accepted criticism that had previously been rationalized away, and it did make a real code/test improvement in `cycle-complete`. But the cycle still shows narrative-smoothing habits. A dispatched follow-up became “actioned,” a structural race got heartfelt reflection without concrete tracking, and the state metrics compress away an awkward intermediate status. That is not total complacency, but it is still enough self-protective drift to matter.

## Priority items

1. Stop calling dispatched-but-unmerged work “actioned,” starting with the publish-gate fail-open finding.
2. Open or implement a concrete fix for the review/artifact race instead of carrying it as a vague next-cycle intention.
3. Repair `copilot_metrics` derivation and finish the semantic freshness guard so state/test narratives match the underlying data.
