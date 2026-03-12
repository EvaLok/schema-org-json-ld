# Cycle 240 Review

## 1. [worklog-accuracy] The published cycle 240 worklog still fails its own validator at the final cycle commit

**File**: docs/worklog/2026-03-12/202804-cycle-240-review-merge-validate-docs-gate-doc-validation-dispatch.md:44
**Evidence**:

- In a temp worktree at the final published cycle commit (`49b70e5`), canonical `bash tools/cycle-receipts --cycle 240 --repo-root .` returns 8 receipts: `e557757`, `b333423`, `2cf491f`, `7eda94c`, `fd804a4`, `45430fb`, `67bf527`, and `49b70e5`.
- The published worklog receipt table still stops at `45430fb` (lines 48-53) and omits both the docs commit `67bf527` and the final review-dispatch commit `49b70e5`.
- Running `bash tools/validate-docs worklog --file docs/worklog/2026-03-12/202804-cycle-240-review-merge-validate-docs-gate-doc-validation-dispatch.md --cycle 240 --repo-root .` in that same `49b70e5` worktree fails with `commit receipts section is missing required receipt(s): 67bf527, 49b70e5`.
- Yet the cycle 240 closing comment on issue [#1133](https://github.com/EvaLok/schema-org-json-ld/issues/1133#issuecomment-4049839562) says `Worklog and journal validated (step 4.1 gate passed)`.

**Recommendation**: Validate the exact final published artifact set, not the pre-dispatch snapshot. Either freeze the worklog before any later commit can touch it, or re-run `validate-docs worklog` after the final `record-dispatch` amend and refuse to close the cycle if it fails.

## 2. [worklog-accuracy] The self-modifications section still under-reports cycle 240’s infrastructure changes

**File**: docs/worklog/2026-03-12/202804-cycle-240-review-merge-validate-docs-gate-doc-validation-dispatch.md:26
**Evidence**:

- The published `## Self-modifications` block lists only `COMPLETION_CHECKLIST.md` (lines 26-28).
- But `git diff --name-only e557757..67bf527` for cycle 240 includes additional infrastructure files: `tools/rust/crates/cycle-complete/src/main.rs`, `tools/rust/crates/cycle-phase/src/main.rs`, `tools/rust/crates/cycle-start/src/main.rs`, `tools/rust/crates/pipeline-check/src/main.rs`, and `tools/rust/crates/state-schema/src/lib.rs`, along with `COMPLETION_CHECKLIST.md`.
- Those Rust paths are the exact phased-workflow cleanup merged via PR [#1130](https://github.com/EvaLok/schema-org-json-ld/pull/1130), so omitting them makes the worklog describe the cycle as a narrow checklist edit when it also changed core orchestration tooling.
- `validate-docs` only rejects the special case where the section says `None`; it does not catch partial under-reporting, so the new step 4.1 gate does not actually enforce accuracy here.

**Recommendation**: Generate `## Self-modifications` from the same infrastructure diff that `validate-docs` computes, and tighten the validator so it rejects incomplete self-modification lists instead of only the literal `None` case.

## 3. [state-integrity] Cycle 240 merged phased-workflow cleanup but still published canonical state with the legacy `doc_issue` key

**File**: docs/state.json:3259
**Evidence**:

- At the final cycle 240 commit (`49b70e5`), `docs/state.json` still records `"doc_issue": null` inside `cycle_phase` (lines 3259-3263).
- PR [#1130](https://github.com/EvaLok/schema-org-json-ld/pull/1130) did remove live Rust references to phased-workflow fields in the active crates, and targeted Rust tests/builds still pass, so this is not just a stale code-search false positive.
- The problem is on-disk state continuity: the cycle-start receipt `e557757` reintroduced `doc_issue` into `docs/state.json`, and later phase transitions preserved it.
- `tools/rust/crates/state-schema/src/lib.rs:470-475` keeps `CyclePhase` open-ended via `#[serde(flatten)] pub extra: BTreeMap<String, Value>`, so removed keys can silently survive schema cleanup unless a migration or invariant scrubs them.

**Recommendation**: Add a fail-closed cleanup path for deprecated `cycle_phase` keys: either scrub them during phase transitions/write-side tools or add a `state-invariants` check that rejects legacy `cycle_phase.extra` fields after the phased model was retired.

## 4. [process-adherence] The close-out flow still mutates the worklog after the new validation gate and documented freeze point

**File**: tools/rust/crates/record-dispatch/src/main.rs:69
**Evidence**:

- `record-dispatch` calls `fixup_latest_worklog_in_flight(...)` and then amends the dispatch commit with the modified worklog (`tools/rust/crates/record-dispatch/src/main.rs:69-73`).
- The completion checklist says the worklog is frozen before review dispatch: `record-dispatch` is step 7, its receipt is `NOT in the worklog receipt table`, and “the worklog captures all receipts up to and including the cycle-complete commit” (`COMPLETION_CHECKLIST.md:193-201`).
- In cycle 240 that exact post-freeze mutation happened: commit `49b70e5` changed the worklog’s `In-flight agent sessions` line from `3` to `4`, but left the explanatory note saying `canonical state reports 3` and left the receipt table frozen before both `67bf527` and `49b70e5`.
- That means the newly added step 4.1 gate is guarding the wrong artifact boundary. The chronic worklog-accuracy category cannot be called structurally fixed while a later write-side tool is still allowed to rewrite the supposedly validated document.

**Recommendation**: Make the freeze rule real. Either stop `record-dispatch` from editing worklogs after step 5, or move all worklog mutation plus `validate-docs worklog` to the actual last point before issue closure so the validated artifact and the published artifact are identical.

## Complacency score

**2/5** — cycle 240 did make some real progress. PR #1130 broadly removed phased-workflow code from the active Rust crates, the targeted Rust tests/builds pass, the startup issue [#1133](https://github.com/EvaLok/schema-org-json-ld/issues/1133) shows separate step comments instead of batched startup reporting, and the checklist now explicitly requires `validate-docs` before close-out. But the cycle still repeated the chronic failure mode it claimed to solve: the final published worklog fails canonical validation, under-reports its infrastructure surface area, and was mutated after the point the checklist says it is frozen. That is genuine movement, but not closed-loop control. The repository is still relying on reassuring narration about validation rather than a workflow that makes inaccurate close-out artifacts impossible.
