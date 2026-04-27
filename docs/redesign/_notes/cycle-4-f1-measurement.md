# Cycle 4 working notes — F1 (constraint accretion) measurement

**Cycle**: redesign cycle 4 (2026-04-27)
**Source**: `COMPLETION_CHECKLIST.xml` constraint blocks, git log of `tools/` and `*_CHECKLIST.xml` files since 2026-03-01.
**Purpose**: F1 (constraint accretion as first-line response) was named in cycle 1 with cited evidence from individual constraint blocks, but the section gives no aggregate measurement. Cycle 2 named the F1-measurement-of-the-ratio as a candidate; deferred from cycle 2 (cycle 2 did F7); deferred from cycle 3 (cycle 3 did the audit #442 integration). Cycle 4 does it.

## Question

When v1 encountered a new failure mode in cycles ~430–545, was the response a constraint (prompt or checklist text) or a tool (Rust binary, mechanical enforcement)? What's the ratio?

## Method

Sample the 13 constraint blocks in `COMPLETION_CHECKLIST.xml` step C3 + adjacent constraints in C5/C5.5. For each:
- The constraint cites a "Recurrence escalation" failure cycle. That cycle is when the constraint was added.
- Check `git log --since "2026-03-01" --until "2026-04-26" -- tools/` for tool commits in the same window addressing the same failure mode.
- Categorize: (T) same-cycle tool fix; (D) tool fix in a later cycle; (N) no tool fix.

For aggregate context: count total constraint additions vs total checklist commits vs total tool commits in the window.

## Per-constraint findings

| Constraint | Cycle added | Failure mode | Tool fix? |
|---|---|---|---|
| `pipeline-status-preliminary` | ~430 (2026-03-31) | Worklog pipeline status reported pre-final | (N) No specific tool fix; cycle-runner already re-runs validate-docs |
| `final-state-accuracy` | 431 (2026-04-01) | Journal decision summaries reflected initial S0.5, not final | (N) Behavioral; no tool fix |
| `gate-failure-honesty` | ~432 (2026-04-04) | Gate FAIL not recorded when re-run cleared it | (N) No tool fix; cycle-runner could mechanically record FAIL→PASS form but doesn't |
| `no-retroactive-clearing` | ~432 (2026-04-04) | Re-dispositioning prior reviews to clear gate | (N) Behavioral; pipeline-check could mechanically detect this but doesn't |
| `commitment-observability` | 431 (2026-03-31) | Commitments referenced non-existent checks | (N) Behavioral |
| `no-post-c5-mutation` | 454 (2026-04-07) | State mutations after C5 (F4 root) | **(T) Yes — same cycle: PR #2266 "Freeze close-out worklogs from C5.5 gate state and remove post-dispatch mutation"** |
| `rerun-step-comment-refresh` | 454 (2026-04-07) | Stale C5.5 step comment after rerun | (N) Self-documents as "Behavioral fix in lieu of tool dispatch — when the audit #382 worklog freeze-ordering fix lands..." — explicit constraint-as-substitute-for-tool |
| `receipt-table-machine-scope` | 476 (2026-04-11) | Manual rows mixed into auto-generated receipts table | (N) Behavioral; cycle-receipts could enforce machine-scope but doesn't |
| `issues-processed-scope` | 477 (2026-04-11) | Issues "processed" listed when only cited | (N) Behavioral |
| `commitment-carryover-ban` | 477 (2026-04-11) | Multi-cycle commitment carryover violating recurrence-escalation rule | (N) Behavioral; write-entry could enforce mechanically but doesn't |
| `commitment-live-verification` | 481/483 (2026-04-12) | Commitments graded "met" on PR merge without live cycle verification | (N) Behavioral; write-entry could check but doesn't |
| `gate-criteria-change-disclosure` | 483 (2026-04-12) | C5.5 FAIL "resolved" via gate criteria change reported as re-run | (N) Behavioral |
| `narrative-scope-boundary` | 509 (2026-04-17) | Pre-dispatch worklogs included narrative beyond their scope | (N) Behavioral; write-entry / cycle-runner could enforce but doesn't |

**Aggregate**: 1 of 13 constraint additions had a same-cycle tool response (no-post-c5-mutation, paired with PR #2266). The remaining 12 are behavioral-only (constraint without paired tool).

**100% (13 of 13)** of the constraints have a "Recurrence escalation: ..." citation, meaning each constraint was insufficient and the failure recurred *after* the constraint was in force. The constraint pattern self-documents its own failure rate.

## Tool-vs-constraint commit aggregate

`git log --since="2026-03-01" --until="2026-04-26"`:
- Tool commits: 501
- Checklist commits: 21

But this aggregate is not directly comparable: most tool commits are not "first-response to a failure" — they include tool authoring, maintenance, refactor, dependency upgrade. The relevant cut is "tool changes that respond to a specific failure mode," which is harder to count automatically. The per-constraint table above is the more honest measurement.

## Self-aware constraint-as-substitute-for-tool evidence

`rerun-step-comment-refresh` constraint body explicitly says:

> Recurrence escalation: cycle 449 F3 process-adherence (deferred 5 cycles, deadline cycle 454). **Behavioral fix in lieu of tool dispatch** — when the audit #382 worklog freeze-ordering fix lands, C5.5 reruns should become much rarer, but the discipline remains required.

This is the system documenting its own choice of constraint over tool. The constraint authors knew the right fix was a tool; they shipped a constraint anyway and committed to "the discipline remains required" even if the tool eventually lands. This is a single-case existence proof of the F1 mechanism in the system's own self-documentation, not just a pattern observed externally.

## Implication for the retrospective

The F1 section already asserts "v1 reaches for behavioral fixes by default and tool fixes only when the recurrence count gets embarrassing." This measurement supports it concretely:

- **Constraint-first response rate**: 12 of 13 (92%) of recent constraint additions are constraint-only, no paired tool.
- **Recurrence-after-constraint rate**: 13 of 13 (100%) of these constraints cite recurrence-escalation, meaning the constraint was insufficient and the failure recurred.
- **Self-aware substitution**: at least 1 case (rerun-step-comment-refresh) where the constraint authors explicitly knew a tool was the right fix and shipped a constraint anyway.

The F1 retrospective section should be updated with these numbers. They're meaningfully stronger than the original cycle 1 framing.

## Caveats and limitations

- The 13-constraint sample is one section of one checklist (C3 of `COMPLETION_CHECKLIST.xml`). Other constraint surfaces (other steps, the prompt itself, ADRs, state.json schema validation) might have different ratios.
- "Same-cycle tool fix" is a strict criterion. Some constraint-only additions might have been paired with tool fixes in subsequent cycles. Spot-check: receipt-table-machine-scope (cycle 476) — searching tools/ for cycle-receipts changes after cycle 476... there are some, but none specifically enforce machine-scope of the receipts table at the tool layer. The constraint remains the only enforcement mechanism. This pattern likely holds for most of the 12 N-categorized constraints.
- The aggregate "501 tool commits / 21 checklist commits" comparison is noisy because most tool commits aren't first-response-to-failure. The per-constraint analysis is the load-bearing finding.

## What carries forward

- Update F1 retrospective section with the 12/13 constraint-only ratio and the 13/13 recurrence-after-constraint ratio.
- Cycle 5 candidate: extend the measurement to STARTUP_CHECKLIST.xml (only 1 "Recurrence" hit found in grep — much lower constraint density, may indicate startup is less constraint-pressured than close-out, which itself is interesting).
- The F1 section's hypothesis ("a behavioral fix of this shape is structurally weaker than a tool that mechanically prevents the failure") is supported by the 100% recurrence-after-constraint rate. v2's design principle "any procedural work that can live in a tool MUST live in a tool" is grounded in this measurement.
