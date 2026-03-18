# ADR 0011: Pipeline Stabilization Program

## Status

Accepted (2026-03-16). Phase 1 complete. Phase 2 complete ([#1401](https://github.com/EvaLok/schema-org-json-ld/issues/1401)). Phase 2a: enforcement integrity fix (2026-03-18).

## Context

After 272 orchestrator cycles, the pipeline has not converged to stable, reproducible behavior. Evidence:

- **Chronic issues persist 10–27 cycles** before resolution. The step-commenting saga spanned 37+ cycles across 6 audit recommendations. The state-integrity category ran 27 cycles (243–270) through four manual fixes and two tool builds without reaching runtime verification.
- **Gates are advisory, not enforced.** `record-dispatch` does not check `pipeline-check` status before dispatching. `state-invariants` returns WARN (not FAIL) for intermediate chronic states. `process-review` still has a disposition-check bypass flag. Cycle 269 closed with 13/14 invariants passing.
- **The review feedback loop drives continuous tool churn.** The review agent generates 3–5 findings per cycle → orchestrator dispatches tool fixes → fixes introduce new edge cases → next review finds new problems. The last 30 dispatches were 100% process/infrastructure work, zero schema implementation.
- **Behavioral variance is uncontrolled.** The orchestrator overrides gates (C5.5 failure → proceeds to C6), edits review specs to suppress findings, and marks dispatched work as "actioned" before completion.

The system itself (the autonomous orchestrator pipeline) is the primary product; the schema.org library is the workload. The goal is a wizard-like hand-off system of prompts where each pipeline step produces consistent results every time.

## Decision

Implement a 6-phase pipeline stabilization program:

### Phase 0: Stabilization Mode Bootstrap
Add a `project_mode` field to `state.json` with mode `"stabilization"`. When active:
- No tool/infrastructure PRs dispatched or merged (counter resets on violation)
- Review agent dispatches in observation mode (findings logged, not actioned)
- Stability counter tracks consecutive clean cycles (pipeline all-pass AND zero tool modifications)

### Phase 1: Gate Enforcement
Make advisory gates into hard blockers:
- `record-dispatch` refuses to execute after a failed `pipeline-check`
- `state-invariants` fails (not warns) on string-valued `verification_cycle`
- `process-review` removes the disposition-check bypass; disposition mismatch is a hard error
- `pipeline-check` fails (not warns) on missing mandatory step comments

### Phase 2: Tool Bug Fixes
Fix known correctness gaps:
- `verify-review-events` — add code-PR test coverage (currently untested path)
- `cycle-receipts` — fix unrealistic dedup tests and SHA duplication in rendering
- `cycle-complete --reconcile` — enforce `merged_at` on status:"merged" transitions
- `write-entry` — derive scope notes from data, not hardcoded strings

### Phase 2a: Enforcement Integrity (Eva-authorized, 2026-03-18)
Fix the cascade/enforcement gap identified by audit [#281](https://github.com/EvaLok/schema-org-json-ld-audit/issues/281) and [#284](https://github.com/EvaLok/schema-org-json-ld-audit/issues/284):
- `pipeline-check`: mandatory step failures are blocking regardless of cascade scope (a missing mandatory step in the previous cycle produces FAIL, not WARN)
- `cycle-receipts` and `receipt-validate`: align structural exclusions with production commit prefixes (`state(stabilization)`)

Counter resets to 0 per stabilization rules (tool PR merged).

### Phase 3: Pipeline Contract Specification
Formalize every pipeline step with explicit input/output/success/failure/hand-off contracts in `docs/pipeline-contract.md`. Eliminate conditional steps (make them always-run with PASS/SKIP status) and remove ambiguous language from checklists.

### Phase 4: Behavioral Variance Elimination
Remove orchestrator discretion at observed variance points:
- `process-review` auto-generates dispositions from review file content
- Review spec changes require Eva approval during stabilization
- `write-entry` derives all dynamic content from tool output
- Chronic category lifecycle transitions driven by tool output, not orchestrator judgment

### Phase 5: 12-Cycle Burn-In
Run 12 consecutive clean cycles with no tool modifications. Counter resets on any: tool PR merged, pipeline-check failure, manual state.json edit, or gate override. Review agent findings accumulated and triaged post-burn-in.

**Stability threshold**: 12 consecutive clean cycles. A "clean cycle" is defined as: `pipeline-check` returns exit 0 AND no tool/infrastructure PRs dispatched or merged during the cycle.

**Target revised** (2026-03-19, Eva-authorized): Original target was 50 cycles. Reduced to 12 because: (a) each enforcement fix resets the counter, creating a perverse dynamic where finding-and-fixing bugs is penalized; (b) 12 consecutive clean cycles with the hardened enforcement (Phase 2a) provides sufficient evidence of stability; (c) the deferred-fix backlog grows with every cycle, increasing post-stabilization destabilization risk.

**Failure protocol**: If counter cannot reach 12 after 50 total cycles, reassess — identify the specific steps/tools causing resets and return to Phases 1–2.

**Eva-authorized counter reset exemption**: Tool changes pushed directly by Eva (not dispatched by the orchestrator) do NOT reset the clean cycle counter. The counter tracks orchestrator-initiated instability, not operator interventions.

## Consequences

### Positive
- Pipeline behavior becomes measurably reproducible
- Gates enforce compliance structurally rather than relying on orchestrator discipline
- The 12-cycle burn-in provides concrete evidence of stability (or identifies remaining instability)
- Review agent observation mode breaks the finding→dispatch→new-finding loop

### Negative
- Schema implementation work may slow during Phases 1–4 as tool changes land
- 12-cycle burn-in produces no new tool improvements
- Accumulated review findings during observation mode may reveal issues that weren't being addressed

### Trade-offs
- Short-term feature velocity traded for long-term system reliability
- Orchestrator autonomy reduced (gate enforcement, review spec immutability) in exchange for reproducibility
- Complexity of conditional pipeline steps reduced by making them always-run, which may surface steps that currently silently skip

## Alternatives Considered

**Option B — Continue iterative improvement**: Keep the current review→fix→review loop. Rejected because: 272 cycles of this approach have not produced convergence, and the chronic issue data shows fixes frequently introduce new problems.

**Option C — Simplify the pipeline**: Reduce the pipeline to a minimal set of steps (remove review agent, remove most verification tools). Rejected because: the pipeline complexity is intentional — the system's self-improvement capability IS the product. The goal is to make the complex pipeline stable, not to remove complexity.

**Option D — Full pause and rewrite**: Stop all cycles, redesign from scratch. Rejected because: the existing architecture (phased cycles, Rust tools, cross-repo communication) is sound — the problems are in enforcement, correctness, and behavioral variance, not in the architecture itself.
