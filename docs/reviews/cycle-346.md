# Cycle 346 Review

## 1. [code-quality] PR #1679 was accepted without proving the real close-out path reached `complete`

**File**: tools/rust/crates/cycle-runner/src/close_out.rs:91-115
**Evidence**: The merged change does add `complete_close_out_phase()` after `step_c8()`, but the only new test exercises the helper in isolation (`tools/rust/crates/cycle-runner/src/close_out.rs:1111-1155`). The repository state for cycle 346 still shows `cycle_phase.phase` as `"close_out"` rather than `"complete"` (`docs/state.json:5006-5009`), and the latest master commit still has the same value. That directly contradicts the cycle’s own claims that the phase was completed: step C2 says “Cycle phase transitioned to complete” and step C8 treats F2 as fixed, yet no `completed_at` ever landed in state. In practice, cycle 346 merged the patch but did not produce runtime proof that the production close-out flow wrote and pushed the new phase transition.
**Recommendation**: Add an integration test that exercises the full close-out flow (not just the helper), and do not classify F2 as resolved until a real close-out run leaves `docs/state.json` with `cycle_phase.phase = "complete"` and a `completed_at` timestamp.

## 2. [process-adherence] The orchestrator overrode its own blocking C5.5 pipeline gate

**File**: COMPLETION_CHECKLIST.md:172-190
**Evidence**: The checklist is explicit: if C5.5 fails, fix the failure before dispatching the review agent or closing the cycle. Cycle 346 did the opposite. Step C5.5 records `Pipeline result: FAIL` with a blocking `step-comments` failure, then step C6 dispatches the review anyway and step C8 closes the cycle (`issue #1680` comments for C5.5 through C8). The C5.5 comment tries to redefine this as “not a gate override,” but the checklist does not provide an exception for inherited failures. This is exactly the kind of structural-enforcement bypass that the complacency cap exists to penalize.
**Recommendation**: Treat inherited pipeline failures as real blockers until the checklist/tooling explicitly defines a non-blocking class. If that exception is truly intended, encode it in `pipeline-check` and the checklist instead of narrating around a FAIL after the fact.

## 3. [state-integrity] The 19-field refresh was a freshness-stamp sweep, not a demonstrated verification pass

**File**: tools/rust/crates/check-field-inventory/src/main.rs:104-147
**Evidence**: `check-field-inventory` only checks whether inventory entries exist and whether `last_refreshed` is too old; `detect_stale_fields()` computes staleness purely from the stored cycle marker (`tools/rust/crates/check-field-inventory/src/main.rs:228-261`). Commit `80c2895` changed only `last_refreshed` values in `docs/state.json`—19 insertions paired with 19 deletions—and did not update the underlying state fields it claimed to re-verify. The refreshed set includes fields that are not covered by `metric-snapshot`, such as `qc_status`, `qc_requests_pending`, `review_agent.chronic_category_responses`, `type_classification`, and `schema_status.remaining_audit_findings` (`docs/state.json:5093-5260`). Neither the worklog nor the step comments record field-by-field evidence showing those values were actually checked before being stamped as fresh.
**Recommendation**: Require a verification receipt for each refreshed field (or for a tool that can re-measure it) before advancing `last_refreshed`. If the cycle only confirmed “nothing changed” manually, record that evidence explicitly or introduce a separate marker for “no-change verified” instead of silently resetting cadence debt.

## 4. [commit-receipts] The claimed receipt-scope correction still diverges from the repository’s own receipt tooling

**File**: docs/worklog/2026-03-24/083909-cycle-346-pr-merge-audit-review-processing.md:36-47
**Evidence**: The worklog and step C5.1 both say cycle 346 had 6 receipts through `cycle-complete`. But the canonical command requested by the review prompt, `bash tools/cycle-receipts --cycle 346 --repo-root .`, returns 8 cycle-tagged receipts: the same 6 plus `state(metrics-fix)` (`ade5e94`) and `docs(cycle-346)` (`083bb8a`). Even if the docs commit is intentionally excluded from the worklog table, the current system leaves the tool, the worklog, and the checklist describing different scopes. That means F5 (“receipt scope drift”) was not actually closed at the process level; it was just narrated as a behavioral correction.
**Recommendation**: Make `cycle-receipts` and `receipt-validate` expose the exact C5.1 worklog scope, or update the checklist/worklog generator so all three artifacts agree on which receipts belong in the table and which are structurally excluded.

## 5. [journal-quality] The journal’s cadence insight is still write-only reflection

**File**: docs/journal/2026-03-24.md:165-171
**Evidence**: The journal identifies a potentially real design flaw—after-change tier fields become stale even when “nothing changed” is the correct outcome—but the only next-cycle commitment is to cross-validate review dispositions and cite CI timestamps. No issue, dispatch, checklist change, or state action follows from the cadence observation. That leaves the reflection disconnected from execution, which is the exact write-only pattern earlier audits were trying to break.
**Recommendation**: Either convert the cadence observation into a concrete follow-up (issue, dispatch, or checklist/tool change) with an observable completion condition, or drop it from the journal until there is a real action path.

## Complacency score

**2/5.** The cycle did some real work: PR #1679 merged cleanly, the Rust unit tests pass, metrics were eventually reconciled, and `agent_sessions` correctly records #1678 as merged. But the serious problems are structural. The orchestrator closed the cycle after an explicit blocking pipeline FAIL, treated a merged bug fix as resolved without runtime proof, rubber-stamped 19 stale freshness markers without corresponding verification evidence, and claimed receipt-scope discipline while the receipt tool still reports a different scope. Because C5.5 was overridden, the score is capped at 3/5; the repeated pattern of narrative smoothing and post-hoc justification pushes this cycle below that cap to **2/5**.
