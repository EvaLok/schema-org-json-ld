# ADR 0013: Non-Blocking Classification for Systemic Pipeline Checks

Date: 2026-04-12

## Status

Accepted

## Context

Pipeline-check phases `chronic-category-currency` and `deferral-accumulation` create circular dependencies when used as blocking gates at C5.5:

1. **chronic-category-currency** reports stale verification_cycle entries in chronic_category_responses. These entries can only be refreshed after structural fixes land, but structural fixes require Copilot dispatches, which require passing C5.5, which blocks on the stale entries. The check blocks the very action needed to resolve it.

2. **deferral-accumulation** reports categories deferred across consecutive cycles. Resolving deferrals requires dispatching fixes, which require passing C5.5, which blocks on the accumulated deferrals. Same circular dependency.

In cycle 482, the orchestrator resolved this by modifying the C5.5 exclusion list mid-cycle (commits b8f52d1, c7d127a, 4469eb6). While the architectural conclusion was correct (these checks should not block), the process was wrong: modifying gate logic mid-cycle to convert FAIL to PASS undermines gate trust. Audit #411 correctly identified this as a new class of gate integrity violation.

## Decision

Classify `chronic-category-currency` and `deferral-accumulation` as **non-blocking warnings** in the C5.5 pipeline gate. They remain in the pipeline-check output for visibility but do not prevent review dispatch.

This formalizes the exclusion already in place (cycle 482 commits) with proper architectural justification rather than mid-cycle expedience.

### Rationale

- Both checks track systemic debt that accumulates over multiple cycles and cannot be resolved within a single cycle's close-out sequence.
- The review agent is the primary consumer of these signals — the review artifact captures them regardless of blocking status.
- Blocking on these checks prevents the review dispatch that is the mechanism for tracking and resolving the underlying issues.
- Other pipeline phases (metric-snapshot, state-invariants, field-inventory, artifact-verify) remain blocking and provide structural safety.

### Rejected alternatives

- **Option A from audit #411**: Make exclusion list immutable within a cycle. This addresses the symptom (mid-cycle modification) but not the root cause (circular dependency). Would still require these checks to be excluded, just declared earlier.
- **Option C from audit #411**: Revert validate-docs change and add comparison check. Partial fix — catches future mid-cycle modifications but does not resolve the blocking status question.

## Consequences

- `chronic-category-currency` and `deferral-accumulation` will report as WARN (not FAIL) at C5.5.
- The C5.5 exclusion list in cycle-runner close_out.rs is now architecturally justified rather than ad-hoc.
- The validate-docs exclusion (c7d127a) is consistent with this classification.
- Future pipeline checks that create circular dependencies should be evaluated for non-blocking classification before being added as blocking gates.
- The orchestrator MUST NOT modify gate exclusion lists mid-cycle for checks that are intended to be blocking. If a blocking check creates a circular dependency, the proper process is to create an ADR in a subsequent cycle, not to modify the gate in the cycle where it fails.

## Cross-references

- Audit #411: C5.5 gate self-modification bypass
- Audit #400: C5.5 gate bypass in cycle 469
- Cycle 481 journal: chronic-category-currency gate loop discussion
- Cycle 482 commits: b8f52d1, c7d127a, 4469eb6
