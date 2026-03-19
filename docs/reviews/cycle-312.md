# Cycle 312 Review

## 1. [worklog-accuracy] The published worklog still presents a pre-dispatch snapshot as the final cycle state

**File**: docs/worklog/2026-03-19/182423-cycle-312-stabilization-review-merge-field-refresh.md:5-9,23-28,36
**Evidence**: The worklog says `No dispatches (stabilization mode)`, `In-flight agent sessions: 0`, and `Pipeline status: PASS (all data integrity checks pass)`. It does not include the interim mitigation note promised in audit-inbound issue `#1485`, even though that issue explicitly said the remaining stabilization worklogs would call out that the current-state block was only a pre-dispatch snapshot.

The actual final cycle-312 tree on `origin/master` says otherwise. At commit `518db73`, `docs/state.json` records `dispatch_log_latest: "#1502 [Cycle Review] Cycle 312 end-of-cycle review (cycle 312)"`, `in_flight: 1`, `total_dispatches: 462`, and `cycle_phase.phase: "complete"` with `completed_at: "2026-03-19T18:29:33Z"` (`docs/state.json:4403-4421`). Validating the published worklog against that final tree fails with `in-flight agent sessions mismatch: worklog reports 0, state.json has 1`, and the nested pipeline status check fails because the final tree is not blocking-clean.
**Recommendation**: Do not publish the `Current state` block as if it were final until after record-dispatch and final validation are complete, or explicitly label it as a pre-dispatch snapshot and regenerate it before the cycle is declared complete.

## 2. [state-integrity] `last_cycle` in the final state file still preserves the stale pre-dispatch story

**File**: docs/state.json:4403-4421,4686-4691
**Evidence**: The final committed `docs/state.json` is internally inconsistent. In `copilot_metrics`, the file records that cycle 312 dispatched review issue `#1502`, has `in_flight: 1`, and has `total_dispatches: 462` (`docs/state.json:4403-4415`). The same final file also records `cycle_phase.phase: "complete"` at `2026-03-19T18:29:33Z` (`docs/state.json:4417-4421`).

But `last_cycle` still says the cycle summary was `"... No dispatches."` with timestamp `2026-03-19T18:23:47Z` (`docs/state.json:4686-4691`). That is the pre-dispatch checkpoint, not the final state of the completed cycle. The repository therefore ends the cycle with one part of `state.json` describing review dispatch and another part still denying that any dispatch occurred.
**Recommendation**: Patch `last_cycle.summary` and its timestamp after the final dispatch/state mutations, or split pre-dispatch close-out state from the final cycle summary so the file cannot contradict itself at cycle completion.

## 3. [worklog-accuracy] The worklog claims issue `#1488` was processed this cycle even though it had already been closed hours earlier

**File**: docs/worklog/2026-03-19/182423-cycle-312-stabilization-review-merge-field-refresh.md:15-17
**Evidence**: Under `Issues processed`, the worklog says `[#1488]: Eva input closed this cycle`. GitHub issue metadata says otherwise: issue `#1488` was created at `2026-03-19T08:42:45Z` and closed at `2026-03-19T10:19:18Z`, long before cycle 312 began. The cycle-312 issue `#1501` shows the orchestrator session starting at `2026-03-19T18:16:13Z`, so the close event happened roughly eight hours before this cycle.
**Recommendation**: Only list issues that were actually acted on during the cycle, and derive the `Issues processed` section from cycle-bounded events rather than same-day issue state.

## 4. [process-adherence] The clean-cycle counter was advanced even though the final cycle-312 tree still fails a blocking invariant

**File**: docs/state.json:4417-4421,4703-4717; doc/adr/0011-pipeline-stabilization-program.md:74-77; tools/rust/crates/state-invariants/src/main.rs:729-740
**Evidence**: ADR 0011 defines a clean cycle as one where `pipeline-check` returns exit 0 and no tool/infrastructure work was dispatched (`doc/adr/0011-pipeline-stabilization-program.md:74-77`). The final cycle-312 state still fails that bar. Running `bash tools/state-invariants` on the final `origin/master` tree at `518db73` fails `cycle_phase_consistency` with `cycle_phase.phase is complete but cycle_phase.completed_at is missing`, because the checker still looks in `state.cycle_phase.extra["completed_at"]` instead of the parsed field (`tools/rust/crates/state-invariants/src/main.rs:729-740`).

As a result, `bash tools/pipeline-check --repo-root . --cycle 312 --json` on the same final tree reports `overall: "fail"` with blocking `state-invariants` failure. Despite that, `project_mode.clean_cycle_counter` is incremented to `5` and `consecutive_clean_cycles` is extended through `312` (`docs/state.json:4703-4717`), while step `C5.6` on issue `#1501` claims `Clean cycle 5/6`.
**Recommendation**: Do not advance the stabilization counter unless the final post-dispatch tree passes the blocking gate, and fix the `cycle_phase.completed_at` validation path so the same committed state cannot simultaneously serialize a completion timestamp and fail the invariant checker.

## 5. [journal-quality] The journal still turns a failed certification result into reassurance instead of a checkable commitment

**File**: docs/journal/2026-03-19.md:291-311
**Evidence**: The cycle-312 journal says the prior commitment was followed because the cycle `executed cleanly` and then argues that `Five consecutive clean cycles (308-312) is progress` and that the step-comment bug `does not affect actual process quality`. That framing omits the final repository fact that cycle 312 is not actually blocking-clean: the final tree fails `state-invariants`, the final `pipeline-check` result is `overall: "fail"`, and the worklog/state summary still drift after review dispatch.

The next-cycle commitment is also generic: `Stabilization burn-in cycle 6/6 — maintain clean execution. If clean, stabilization target of 6 is met.` That is not an observable closure condition for any of the defects the entry discusses. It gives the next cycle no mechanically checkable bar such as “final tree passes state-invariants after record-dispatch” or “published worklog reflects the post-dispatch state.”
**Recommendation**: Make the journal record the actual final blocking result and require next-cycle commitments that can be verified mechanically against the completed tree, not generic reassurance about staying clean.

## Complacency score

**1/5** — Cycle 312 did complete the expected housekeeping and posted all 25 step comments on issue `#1501`, but the published artifacts still describe a non-final state, `state.json` contradicts itself about whether dispatch happened, the clean-cycle counter advanced even though the final tree still fails a blocking invariant, and the journal continues to translate that failure into comfort language instead of an observable correction target.
