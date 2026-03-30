# Cycle 415 Review

## 1. [worklog-accuracy] The cycle 415 write-entry fix (PR #2049) still preserves the stale primary in-flight counter

**File**: tools/rust/crates/write-entry/src/main.rs:444-470,7015-7083
**Evidence**: The new `patch-pipeline` logic does derive an `in_flight_value` from `docs/state.json`, but it only feeds that value into the post-dispatch patch/addendum path. The regression test added with the fix explicitly asserts that the original `- **In-flight agent sessions**: 0` line remains unchanged while only `- **In-flight agent sessions (post-dispatch)**: 1` is added. That is exactly what the published worklog shows (`docs/worklog/2026-03-30/204947-cycle-415-review-processed-2-tool-fixes-merged-field-inventory-refreshed.md:36-39`). `bash tools/pipeline-check` still fails cycle 415 `doc-validation` with `worklog validation failed: in-flight agent sessions mismatch: worklog reports 0, state.json has 1`. Cycle 414's review identified the stale primary line as the chronic root cause, so cycle 415's code change fixed the addendum path without fixing the defect under review.
**Recommendation**: Update `patch-pipeline` to re-derive and replace the primary in-flight counter from committed `docs/state.json` during the post-dispatch refresh, then add a regression test that expects the primary line itself to match state after the patch.

## 2. [journal-quality] The journal says the worklog-accuracy deadline was met even though canonical state still says it is unresolved

**File**: docs/journal/2026-03-30.md:291-296
**Evidence**: The cycle 415 journal states, `The worklog-accuracy deferral deadline (cycle 415) was met: the write-entry auto-derive fix directly addresses the root cause.` But `docs/state.json:6475-6478` still records the `worklog-accuracy` deferred finding with `deadline_cycle: 415` and `resolved: false`. The current `bash tools/pipeline-check` output reinforces that mismatch twice. It warns `category 'worklog-accuracy' is due this cycle`, and it separately fails `doc-validation` on the cycle 415 worklog/state mismatch. That is the same optimistic grading pattern cycle 414 had just criticized.
**Recommendation**: Do not mark a deferred-finding deadline as met until the state ledger marks it resolved and the validating gate is green. Journal claims should require both a merged fix and an observable verification artifact.

## 3. [process-adherence] Cycle 415 still advanced past a blocking pipeline failure, so the complacency score is capped at 3/5

**File**: docs/worklog/2026-03-30/204947-cycle-415-review-processed-2-tool-fixes-merged-field-inventory-refreshed.md:34-40
**Evidence**: The issue record for cycle 415 shows a blocking failure was overridden. Issue comment `#issuecomment-4157888926` posted `Step 4` as `Pipeline check failed (see warnings)`. The cycle nevertheless continued through dispatches, merges, and close-out before later posting a `Step C5.5` PASS and `Step C8` completion. The review instructions for this issue are explicit: if the cycle overrode any FAIL or blocking-level pipeline gate, the maximum complacency score is 3/5. The final worklog state section records only the later PASS statuses and omits that the cycle advanced after an earlier blocking failure. The step-comment discipline itself was good — issue #2045 has 29 total comments, 28 step-tagged, and `bash tools/pipeline-check` reports all mandatory current-cycle steps present — but that is only mitigation, not an exemption from the cap.
**Recommendation**: Apply the mandatory 3/5 cap in the review and explicitly call out the overridden gate in the cycle artifacts whenever a blocking pipeline step fails before later repair.

## Complacency score

**3/5** — capped by the issue mandate because cycle 415 advanced after a blocking pipeline-check failure at Step 4 (`https://github.com/EvaLok/schema-org-json-ld/issues/2045#issuecomment-4157888926`) before later repair at Step C5.5 (`https://github.com/EvaLok/schema-org-json-ld/issues/2045#issuecomment-4158015024`). I am not scoring it lower because the cycle did produce two targeted tool fixes, the published receipt hashes resolve and match `bash tools/cycle-receipts --cycle 415 --repo-root .`, state/metric invariants currently pass, and issue #2045's step-comment coverage was strong. But the chronic worklog-accuracy category was not genuinely retired: the write-entry fix still preserves the stale primary counter, the journal overclaims that the deadline was met, and the worklog still depends on a post-dispatch addendum rather than a truthful primary state line.
