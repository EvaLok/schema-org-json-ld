# Cycle 211 Review

## Findings

1. **[pr-923-robustness] PR #923 left the new journal scan fragile and under-tested**

   **File**: tools/rust/crates/pipeline-check/src/main.rs:534-540,612-648,1572-1687
   **Evidence**: The new code has an explicit WARN path for `docs/journal/` with no valid dated files, but the added tests only cover today, stale, missing-directory, and newest-file selection. There is no test for an existing directory with only invalid filenames, and the implementation calls `parse_iso_date(candidate)?` for any `YYYY-MM-DD.md`-shaped name. A local repro with a temp repo containing `docs/journal/2026-13-40.md` makes Phase 7 return `artifact-verify: ERROR (invalid date '2026-13-40': input is out of range)` instead of ignoring the bad file and continuing.
   **Recommendation**: Treat malformed date-shaped filenames as invalid journal artifacts to skip, and add tests for empty directories, non-dated filenames, and malformed `YYYY-MM-DD.md` names.

2. **[receipt-integrity] The cycle 211 worklog records three receipt hashes that do not exist**

   **File**: docs/worklog/2026-03-09/221546-cycle-211-summary.md:42-52
   **Evidence**: The receipt table lists `9a310d2`, `41d1a73`, and `16978a8`, but `git rev-parse --verify` cannot resolve any of them even after `git fetch --unshallow origin`, and GitHub commit lookup also reports no commit found for `41d1a73` and `16978a8`. The actual cycle 211 commits in `git log --oneline` are `bf3de9a` for `state(cycle-start)`, `e9e36a6` for `state(process-merge): PR #923 merged`, and `9c05218` for `state(process-merge): PR #925 merged`.
   **Recommendation**: Generate worklog receipts from the tool outputs or `HEAD`-adjacent commits directly, and add a closeout validation that every listed hash resolves before the worklog is committed.

3. **[worklog-accuracy] The worklog still contradicts itself about whether any issues were processed**

   **File**: docs/worklog/2026-03-09/221546-cycle-211-summary.md:5-10,22-24
   **Evidence**: In `What was done`, the worklog says `Processed cycle 210 review: 4 findings (1 actioned, 3 deferred)`, but the dedicated `### Issues processed` section still says `- None.`. This is the exact drift class the prior review warned about: cycle 210 recommended making these sections derived or required with fail-closed checks, not optional afterthoughts (`docs/reviews/cycle-210.md:27-30`).
   **Recommendation**: Make worklog generation fail closed when summary lines mention processed reviews/issues but the bookkeeping section still renders `None.`, and do not rely on optional manual flags to catch contradictions later.

4. **[journal-quality] The cycle 211 journal repeats the duplicate-follow-through bug and contradicts itself**

   **File**: docs/journal/2026-03-09.md:581-589
   **Evidence**: The entry contains two `### Previous commitment follow-through` blocks. The first quotes `> Previous commitment: 1. None.` and then says `Both commitments actioned`, while the second block enumerates two separate commitments. That is not credible reflection; it is templating spillover that survived one full cycle after cycle 210 already flagged duplicate journal sections as a problem.
   **Recommendation**: Reject duplicate section headings in a single journal entry and refuse to generate “followed” narratives from a `None.` commitment block.

5. **[state-ledger] The manual #924 repair created a one-off `agent_sessions` shape that drops dispatch timing**

   **File**: docs/state.json:2399-2406
   **Evidence**: The manually added issue `924` entry is the only `agent_sessions` row using `dispatched_cycle` instead of `dispatched_at`. Tool-generated sessions write `dispatched_at` consistently (`tools/rust/crates/record-dispatch/src/lib.rs:45-50`), and even the backfill tool reconstructs missing sessions with `dispatched_at` from the issue creation timestamp (`tools/rust/crates/backfill-sessions/src/main.rs:203-210`). This repair also shortens the title to `Cycle 210 review`, making the ledger less faithful than neighboring entries.
   **Recommendation**: Backfill issue `924` with the same schema the tools use elsewhere—include `dispatched_at` from the issue creation time and avoid introducing bespoke fields into `agent_sessions`.

6. **[metrics-reconciliation] The claimed metrics reconciliation is still wrong, and the pipeline catches it**

   **File**: docs/state.json:2583-2595
   **Evidence**: `docs/state.json` records `dispatch_to_pr_rate` as `97.4%`, but `bash tools/derive-metrics --repo-root /home/runner/work/schema-org-json-ld/schema-org-json-ld` computes `97.0%` from the live ledger. `bash tools/pipeline-check --cycle 211` currently fails step 6 on that exact mismatch. This also leaves the cycle 211 worklog’s `Pipeline status: PASS` line stale relative to the final state that was manually edited after `cycle-complete`.
   **Recommendation**: After any manual `state.json` repair, rerun the metrics derivation step with apply/update semantics and regenerate the cycle-close artifacts so worklog, journal, and state stay in sync.

7. **[issue-spec-quality] Dispatch #927 encodes another manual workaround instead of the structural fix the prior review asked for**

   **File**: docs/worklog/2026-03-09/221546-cycle-211-summary.md:7-10
   **Evidence**: Cycle 210’s review recommended extending `write-entry` so those bookkeeping sections are derived or required, with fail-closed tests when the same worklog already proves contrary activity (`docs/reviews/cycle-210.md:27-30`). Cycle 211 instead dispatched `#927` as optional `--pr-reviewed`, `--issue-processed`, and `--self-modification` flags. That still allows the exact same failure mode: the operator can forget the flags and ship another `None.` section even when the worklog body says otherwise.
   **Recommendation**: Re-scope `#927` so `write-entry` derives these sections from cycle data when possible and errors when explicit activity is present elsewhere in the same worklog but the bookkeeping fields are empty.

8. **[contract-softening] Dispatch #929 relaxes the parser contract instead of fixing the agent that keeps violating it**

   **File**: COMPLETION_CHECKLIST.md:171
   **Evidence**: The checklist explicitly says review findings must use inline `[category-name]` headings because that is what `process-review` parses. Cycle 211’s journal and worklog then justify `#929` as teaching strict mode to accept `Category:` fallback because the review agent has been wrong four cycles in a row (`docs/journal/2026-03-09.md:591-593`, `docs/worklog/2026-03-09/221546-cycle-211-summary.md:9-10`). That is pragmatic, but it also means “strict mode” is being weakened to tolerate a repeated contract violation instead of reconciling the instructions and the agent behavior.
   **Recommendation**: Either update the review-writing instructions and checklist together with the parser, or keep strict mode truly strict and fix the generating prompt/skill so the contract stops drifting.

## Complacency score

4/5 — Cycle 211 did merge a real code fix and did wait for CI on the two merged PRs, so this was not a fake-activity cycle. But the closeout artifacts still show strong “going through motions” behavior: bogus receipt hashes were recorded without verification, the worklog repeated an already-known bookkeeping contradiction, the journal repeated the duplicate-section bug one cycle after it was flagged, the state repair introduced a one-off ledger shape, and the response to repeated review-format drift was to soften the parser contract instead of fixing the source.

## Recommendations

1. Add a cycle-close validation pass that resolves every receipt hash, reruns `derive-metrics`, and rejects artifact generation if worklog/journal/state disagree.
2. Tighten the journal and worklog writers so duplicate sections and `None.` bookkeeping contradictions are generation errors, not review-time discoveries.
3. Revisit the follow-up issue specs so they address the structural causes of drift (`write-entry` derivation and review-format contract alignment) rather than optional/manual mitigations.

## Priority items

1. Fix the `#924` session backfill + `copilot_metrics.dispatch_to_pr_rate`, then regenerate cycle 211 closeout artifacts from the corrected state.
2. Harden PR `#923`’s journal scanner against malformed filenames and add the missing tests for invalid/empty `docs/journal/` contents.
3. Replace the optional/manual follow-up specs for `#927` and `#929` with fail-closed fixes that eliminate the recurring drift instead of tolerating it.
