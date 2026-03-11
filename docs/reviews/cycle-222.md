# Cycle 222 Review

## Findings

## 1. [code-quality] `state_snapshot_freshness` ignores dispatch-count drift because it reads a nonexistent state field

**File**: tools/rust/crates/check-doc-pr/src/main.rs:18-23
**Evidence**: `STATE_SNAPSHOT_FIELDS` compares `/copilot_metrics/dispatched`, but the live schema stores dispatch count as `total_dispatches` (`docs/state.json:2827-2839`). The new tests reinforce the wrong contract by building fixtures with `copilot_metrics.dispatched` (`tools/rust/crates/check-doc-pr/src/main.rs:1151-1207`). Comparing PR #1005's docs commit (`390962a`) to the master state at doc dispatch (`7fe470d`) shows real dispatch drift (`total_dispatches` 292 -> 293), while the currently monitored `/copilot_metrics/dispatched` path is missing on both sides, so the new check cannot report that divergence.
**Recommendation**: Change the monitored field to `copilot_metrics.total_dispatches`, update the unit tests to use the real schema field names, and add a regression test that compares snapshots shaped like the actual `docs/state.json`.

## 2. [code-quality] The phased-cycle step-comments fix never covered the real `#996` resumption pattern

**File**: tools/rust/crates/pipeline-check/src/main.rs:657-723
**Evidence**: `verify_step_comments` only downgrades to `WARN` when phased markers are present and `has_startup_step_comment(found)` is false. That does not match the real failing issue: `#996` contains `Step 0` plus `Opening`, `10.B`, `10.C`, and `Close`, so `has_startup_step_comment(found)` returns true and `bash tools/pipeline-check --cycle 222` still fails with “found 1 unique step comments on issue #996; missing steps: 0.5, 0.6, 1, 1.1, 2, 3, 4, 5, 6, 7, 8, 9, 10”. The added tests only cover phased markers without `Step 0` and a synthetic `previous_cycle_work_issue` fallback (`tools/rust/crates/pipeline-check/src/main.rs:2094-2208`), but cycle 222 state does not record any `previous_cycle_work_issue` field at all (`docs/state.json:3107` and no matching field elsewhere in the file).
**Recommendation**: Add a regression test that reproduces the actual `#996` comment set, teach the detector that the resumption `Step 0` comment is not proof of startup coverage, and either populate `previous_cycle_work_issue` consistently or remove the dead fallback path from the design.

## 3. [worklog-accuracy] The cycle 222 worklog froze stale state and a stale receipt instead of reflecting the final committed cycle state

**File**: docs/worklog/2026-03-10/220600-cycle-222-summary.md:33-48
**Evidence**: The merged worklog says the cycle had `0` in-flight sessions and records `a7e280a` as the cycle-complete receipt. That was already behind the repository state. At doc dispatch, master had advanced to `phase=doc_dispatched`, `in_flight=1`, and `total_dispatches=293` (`7fe470d`), while the doc PR was still based on pre-dispatch state (`390962a`: `phase=work`, `in_flight=0`, `total_dispatches=292`). Later, `state(cycle-complete): update last_cycle fields [cycle 222]` became the effective receipt (`6ad4217`), and `bash tools/cycle-receipts --cycle 222` now returns `6ad4217`, not the `a7e280a` hash printed in the worklog. This directly contradicts the cycle-doc prompt instruction to derive current state from committed `docs/state.json` and to use `cycle-receipts` output verbatim.
**Recommendation**: Regenerate or refresh worklog artifacts after master state advances, or block doc PR merge when the current-state section or receipt table lags the committed state. Do not merge a stale worklog and then rationalize the mismatch afterward.

## 4. [process-adherence] Phase B/C knowingly overrode failing doc validation instead of using the revision loop

**File**: COMPLETION_CHECKLIST.md:251-273
**Evidence**: The checklist says a documentation PR should merge only when `check-doc-pr` passes; otherwise, if `review_iteration < review_max`, the orchestrator must request revisions and return to `doc_dispatched`. Cycle 222 did the opposite. The Phase B/C issue records “Doc PR #1005 merged ... check-doc-pr: 7/9 pass, 2 expected divergences” even though `cycle_phase.review_iteration` remained `0` and `review_max` was `3` (`docs/state.json:2841-2848`). The journal then claims PR #1001 “closes that exact gap” and promises the *next* documentation PR will be the first real verification pass (`docs/journal/2026-03-10.md:94-109`), which means the new stale-doc guardrail was waived on the very first PR that exercised it.
**Recommendation**: Stop treating failing `check-doc-pr` results as acceptable background noise. Either follow the documented revision loop until the checks pass, or explicitly encode any allowed exception in the checklist/tooling so reviewers are not silently overriding a gate they just added.

## Complacency score

**5/5** — cycle 222 did substantive work, but the dominant pattern was still “note the problem, add a guardrail, then override it on first use.” The strongest evidence is the documentation path: the cycle review correctly identified stale docs as a real failure, PR #1001 claimed to fix it, the checklist still required revisions on failed doc checks, and Phase B/C nevertheless merged PR #1005 with 7/9 checks passing while calling the failures “expected divergences.” That is not simple incompleteness; it is an active normalization of failing verification.
