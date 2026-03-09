# Cycle 197 Review

Cycle 197 did land real work: PR #813 tightened `pipeline-check`, PR #815 committed the cycle 196 review artifact, and the repository snapshot is internally self-consistent at the moment it was written. The deeper problems are that the cycle overstated review-finding resolution, left at least one state field visibly stale after claiming to refresh it, and then kept doing work after freezing the worklog/journal/state snapshot. The result is a cycle record that looks cleaner than the actual end-of-cycle activity.

## Findings

1. **[pipeline-integrity] PR #813 makes `pipeline-check` stricter than `derive-metrics` itself, so the new blocking gate can fail while the authoritative tool still says the state is valid**

   Category: pipeline-integrity

   `pipeline-check` now treats rate fields as exact string matches only: for `dispatch_to_pr_rate` and `pr_merge_rate`, it accepts only `Value::as_str()` equality and otherwise records a mismatch (`tools/rust/crates/pipeline-check/src/main.rs:428-476`). But `derive-metrics` — the tool this phase is supposed to mirror — still explicitly accepts both the percentage display and the legacy `n/d` ratio format during migration (`tools/rust/crates/derive-metrics/src/main.rs:166-215,238-275`).

   The new tests lock that semantic drift in rather than exposing it. `derive_metrics_is_fail_when_rate_fields_diverge` and `derive_metrics_is_fail_when_pr_merge_rate_diverges` use `2/4` and `1/2` in state and assert that `pipeline-check` must now fail (`tools/rust/crates/pipeline-check/src/main.rs:757-822,824-860`). That means the cycle promoted derive-metrics drift to a blocking failure before the authoritative derive-metrics checker finished its own migration away from ratio strings.

   **Recommendation:** Make `pipeline-check` reuse the same rate-comparison semantics as `derive-metrics --check`, or remove the migration path from `derive-metrics` in the same change and backfill every remaining ratio-format state before promoting the pipeline gate to blocking.

2. **[disposition-accuracy] The worklog marks two cycle 196 findings as “ACTIONED” even though the repository snapshot still shows both fixes as merely dispatched or deprecated**

   Category: disposition-accuracy

   The disposition table says the orphaned `note` field was `**ACTIONED**` and the parser-contract mismatch was `**ACTIONED**` (`docs/worklog/2026-03-08/225043-hundred-ninety-seventh-orchestrator-cycle.md:16-23`). The repository snapshot does not support that label. `copilot_metrics.note` is still present in state, only replaced with a deprecation string (`docs/state.json:2250-2258`), and the cleanup work is still only an in-flight dispatch in `agent_sessions` (`docs/state.json:2106-2111`). Likewise, the parser fix was not present in the committed code: `process-review` still only looked for an indented `Category:` line after a numbered finding heading (`tools/rust/crates/process-review/src/main.rs:323-378`), while the supposed fix existed only as another in-flight dispatch (`docs/state.json:2100-2105`).

   The journal repeats the same inflation. It says PR #813 “correctly adds rate field string comparison,” says dispatching #817 counts as following through on the parser commitment, and frames the infrastructure as “genuinely solid now” with the main remaining friction point already “being fixed” (`docs/journal/2026-03-08.md:540-542,548-549,560-563`). That is not actioned work; it is queued follow-up work.

   **Recommendation:** Stop using `ACTIONED` for findings that were only deprecated, manually patched, or dispatched. Use `DISPATCHED` / `PARTIALLY ACTIONED` until the fix is merged, or land the actual fix before closing the cycle.

3. **[state-integrity] Cycle 197 claims it closed question-for-Eva issue #771, but `docs/state.json` still records that issue as open and falsely certifies the field as refreshed this cycle**

   Category: state-integrity

   The worklog says cycle 197 closed `#771` after Eva replied “continue as-is” (`docs/worklog/2026-03-08/225043-hundred-ninety-seventh-orchestrator-cycle.md:8-10`). But the committed state still lists `771` under `open_questions_for_eva` (`docs/state.json:2483-2485`). Worse, the field inventory says `open_questions_for_eva` was refreshed in cycle 197 (`docs/state.json:2365-2368`), which turns a stale value into a falsely certified one.

   This is not a hypothetical mismatch: GitHub issue #771 is in fact closed, with Eva’s reply recorded on 2026-03-08T20:04:34Z and the issue closed at 2026-03-08T22:43:42Z. The cycle therefore updated the narrative but not the state ledger that is supposed to back it.

   **Recommendation:** Reconcile `open_questions_for_eva` whenever a `question-for-eva` issue is closed, and add an invariant or pipeline check that rejects closed issue numbers lingering in that list.

4. **[cycle-close-drift] The committed worklog, journal, and state snapshot were frozen before the cycle actually stopped changing, so the final cycle record misses later merges and dispatches**

   Category: cycle-close-drift

   The committed artifacts all describe the cycle as if it ended with only PRs `#813` and `#815` merged and issues `#817` / `#819` still in flight (`docs/worklog/2026-03-08/225043-hundred-ninety-seventh-orchestrator-cycle.md:5-13,35-45`; `docs/state.json:2100-2112,2250-2264,2475-2480`; `docs/journal/2026-03-08.md:566-568`). But `last_cycle.timestamp` in state is `2026-03-08T22:51:56Z` (`docs/state.json:2475-2480`), and the actual cycle-complete comment on issue #816 was posted later, at `2026-03-08T22:56:10Z`, claiming that PR `#818` had also been merged and that review issue `#821` had been dispatched (comment: `https://github.com/EvaLok/schema-org-json-ld/issues/816#issuecomment-4020197361`). GitHub confirms both events: PR `#818` merged at `2026-03-08T22:55:37Z`, and issue `#821` was created at `2026-03-08T22:55:13Z`.

   That means the final cycle issue comment describes a later state than the committed repo artifacts. Once those extra events happened, the `agent_sessions` ledger and `copilot_metrics` block in `docs/state.json` were already stale, and the journal’s “next cycle” commitments were stale too because one of the supposedly pending PRs had already merged. This is also direct evidence that `derive-metrics` was not re-applied after the last merge/dispatch.

   **Recommendation:** Treat worklog/journal/state generation as the last mutable step of the cycle. If any PR merge or dispatch happens afterward, rerun the close-out sequence (including `derive-metrics --apply`) before posting the final completion comment.

## Complacency score

**5/5.** The cycle was not a rubber stamp — it made real changes — but it repeatedly chose the tidier story over the fully accurate one. PR #813 installed a blocking gate that is stricter than the tool it is supposed to mirror, two review findings were marked `ACTIONED` before their fixes existed in the repo snapshot, `open_questions_for_eva` was certified fresh while still containing a closed issue, and the cycle then kept changing after freezing the artifacts that future cycles will trust. That is exactly the kind of selective accounting an adversarial review is supposed to catch.
