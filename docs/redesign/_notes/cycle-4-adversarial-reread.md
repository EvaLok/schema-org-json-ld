# Cycle 4 working notes — adversarial re-read of audit-derived additions

**Cycle**: redesign cycle 4 (2026-04-27)
**Sources**: `0-retrospective.md` cycle 3 state, audit-repo issue #442, repo state evidence (commit log, `state.json` keys, `COMPLETION_CHECKLIST.xml` constraint blocks, audit-repo issues #402/#406/#415/#417/#420)
**Purpose**: cycle 3's journal flagged a 100%-acceptance rate of audit #442's suggestions as "worth examining." This file is that examination. Each audit-derived addition is tested against the rule "must survive without audit's argument as scaffolding."

## Methodology

For each addition this cycle 3 made to `0-retrospective.md` from audit #442, I asked four questions:

1. What evidence in the repo state, codebase, or git history supports this independently of audit's argument?
2. If audit had not made this point, would I make it now?
3. Is the framing correctly nuanced (over-claiming or under-claiming)?
4. Does the implication for v2 follow from the evidence?

## Findings per addition

### F11 — cycle closure as artificial completion signal

**Holds up.** Independent evidence in the commit log:
- Cycle 543: `state(metric-snapshot): refresh 11 field_inventory entries [cycle 543]` (commit `60d460aa`) is committed AFTER `state(cycle-complete): 0 dispatches, 1 merges (PR #2726) [cycle 543]` (commit `7db1d036`). Both bear the same cycle tag.
- Cycle 542: same pattern. `state(metric-snapshot)` (commit `ba551af7`) is after `state(cycle-complete)` (commit `4ca87e4f`).

The cycle-complete commit is supposed to be the close-out marker; metric-snapshot mutates state after it. Repeats every cycle.

F11 is not just a re-statement of F4 — F4 says "fix the freeze ordering," F11 says "consider whether the cycle-as-state-boundary is the right unit at all." F11 is more radical. The implication for v2 (state evolves continuously; cycle as checkpoint event, not state hard-boundary) follows from the evidence and is genuinely different from F4's narrower implication.

**Verdict**: keep as-is.

### F12 — late-stage defense accumulation: an unmapped catalog

**Holds up with one nuance.** Independent evidence:
- `state.json` has 38+ top-level keys. Spot inventory:
  - Defenses: `step_comment_acknowledged_gaps`, `field_inventory`, `pending_audit_implementations`, `last_eva_comment_check`, `cycles_since_last_forward_work`, `deferred_findings`, `review_dispatch_consecutive`, `review_events_verified_through_cycle`, `audit_dropped`, `audit_processed`, `qc_processed`, `qc_requests_pending`, ...
  - Domain primitives: `schema_version`, `total_schema_classes`, `total_enums`, `test_count`, `typescript_plan`, ...
  - Mixed: `last_cycle`, `cycle_phase`, `cycle_issues` (procedure-encoding more than primitive)
- `COMPLETION_CHECKLIST.xml` C3 has 13 constraint blocks, all citing recurrence-escalation; this is the F1 form of the same defense-accumulation pattern at the prompt layer.

Cycle 3's journal worried that "load-bearing input to Phase 2 candidate-selection" was over-claiming. On adversarial re-read the framing is already qualified ("F12 is the placeholder for the catalog; the full version is multi-cycle work"). The actual claim is "without working on a catalog at all, candidates risk silent re-introduction" — not "without a complete catalog, Phase 2 cannot proceed." The cycle 3 worry was a slight over-reading of the framing.

One genuine nuance worth surfacing: F12 catalogs both **state-shape defenses** (F5 at the data-model layer) and **constraint-language defenses** (F1 at the prompt layer) and **tool-defense defenses** (e.g., `check-eva-responses` polling tool). It's the same accumulation mechanism viewed across three substrates. F12's framing already acknowledges this ("F12 is the same mechanism viewed at a higher level"), but the relationship to F1 and F5 should be made more explicit so a reader doesn't think F12 is duplicating them.

**Verdict**: keep substantively; the cross-reference to F1 and F5 in F12's body could be sharper. (Minor edit; not blocking.)

### Shared-root preamble — asymmetric communication / write-mostly state

**Holds up partially.** The preamble lists F2, F3, F4 (post-C5 mutation aspect), F8 (parallel-implementation cycle-runner gap, in its inbound-reconciliation aspect), and F11 as sharing the asymmetric-communication root.

- F2 (Eva-response detection): outbound well-toolified, inbound has no poller. **Strong fit.**
- F3 (multi-candidate state drift): writes once, no reconciliation. **Strong fit.**
- F4 (frozen-artifact lifecycle): worklog freeze writes a snapshot; state continues mutating; no reconciliation. **Strong fit.**
- F11 (cycle closure): close-out writes the boundary; state evolves past it; no reconciliation. **Strong fit.**
- F8 (abandonment cascades from single-tool defects): primary problem is *parallel-implementation duplication* (cycle-runner and cycle-start both have a `run_tool_json`-shape function, only one was fixed). The "inbound-reconciliation aspect" framing requires reading the gap as "fix was written outbound, no inbound reconciliation against the broader codebase caught the parallel implementation." That framing is a stretch. The more natural F8 framing — "fewer tools doing each job" — is independent of asymmetric communication.

**Verdict**: F8 should be removed from the shared-root preamble's list. It's a different failure mode (duplication, not write-mostly). Alternative: keep F8 but qualify the framing as "weaker connection — F8's primary root is parallel-implementation duplication, included here only insofar as fix-propagation is also asymmetric." The qualification version is more honest than dropping; cycle 4 will apply that.

### F10 property-1 / property-2 framing

**Holds up.** Independent evidence:
- Audit's #427 (cycle 200) explicitly accepted the model upgrade to Opus 4.7 for the audit orchestrator. Both audit and main are now Opus 4.7 — verifiable in the audit repo's prompt and dispatch tooling.
- Audit's load-bearing finds (#439, #437, #427, #442) all required cross-cycle visibility — verifiable from each issue's body, which cites multiple cycles.
- The same-cycle review agent (Copilot, gpt-5.4-class) only sees the current cycle's artifacts — verifiable from the dispatch spec.

The property-1/property-2 distinction is sharp, actionable, and independently verifiable. **Verdict**: keep.

### F9 hypothesis correction

**Holds up.** Independent evidence:
- Review agent dispatched as gpt-5.4 (Copilot) per the dispatch tooling.
- Orchestrator runs as `claude-opus-4-7` (verified in this cycle's startup).
- Audit catches things review doesn't, both being Opus 4.7 — supports "shared variable is prompt + state + context, not model."

The corrected hypothesis is internally consistent with the F9 section's own attribution of review-agent model class. **Verdict**: keep.

### Schema-work threshold rewrite

**Holds up at form level; implementation remains open.** Cycle 3's journal flagged that "fraction of cycle compute spent on domain output" is hard to measure mechanically. That concern is real and acknowledged in the retrospective. The retrospective's framing is "the form of the metric is open; the framing is right; the implementation is not yet specified" — which is honest. Phase 2 candidates should each propose a concrete operationalization. **Verdict**: keep.

### Measurement-not-impression v2 design principle

**Holds up.** Cycle 1's softening of F7 ("small minority") that cycle 2's measurement falsified ("zero"); audit #442's catching of two factual errors via broader read scope — both support the principle. One precision worth noting: the principle should apply to *factual claims* about state and progress, not to *judgment calls* about design. The retrospective's phrasing ("anywhere a load-bearing claim is made about cycle quality, schema progress, system health, or design success") is broad enough to be misread as covering judgment calls; tightening to "factual claims about state, progress, or measurable quality signals" would be cleaner. Minor edit; not blocking. **Verdict**: keep substantively.

### Foreground-Eva caveat

**Holds up.** PR #2740 (redesign authorization, cycle 545) and #2741 (input-from-eva, cycle 545+) are direct evidence that foreground Eva intervention is the load-bearing path for the redesign itself. The Phase 0/2/4 checkpoint architecture in this prompt depends on it. **Verdict**: keep.

### F1 audit-side parallel

**Holds up.** Verified all five audit issues exist:
- #402 (2026-04-10): Chronic-category verification_cycle staleness — 4 entries confirmed-holding but never re-verified, pipeline warning ignored for 11-12 cycles
- #406 (2026-04-11): Audit #402 accepted but not actioned — 3 cycles of non-action since promised deferral to cycle 475
- #415 (2026-04-13): Chronic category granularity too coarse for convergence — refresh/rollback cycle structurally non-convergent for 22 cycles
- #417 (2026-04-14): Audit #415 sub-categorization deployed but not utilized — review agent emits parent-level categories, convergence benefit unrealized
- #420 (2026-04-14): Sub-categorization adoption chain incomplete after 3 implementation cycles — three broken links remain

The escalation pattern is clear: each filing diagnoses a more specific structural defect than the last. Total span ~14 calendar days, ~16 audit cycles. Pattern matches F1 at the audit-system layer. **Verdict**: keep.

## Net findings of the adversarial re-read

8 of 9 audit-derived additions hold up unchanged. One needs qualification:

1. **Shared-root preamble**: F8's inclusion via the "inbound-reconciliation aspect" is a stretch. F8's primary root is parallel-implementation duplication, not asymmetric communication. Either remove F8 from the preamble's list or qualify the connection explicitly. Cycle 4 will qualify rather than drop, since the fix-propagation framing has *some* explanatory power even if it's the weaker root for F8.

Two minor sharpenings flagged but not blocking:
- F12 cross-reference to F1 and F5 could be sharper (F12 is a meta-pattern across three substrates).
- Measurement-not-impression principle could be tightened to specify "factual claims" vs "judgment calls."

The 100% acceptance rate flagged by cycle 3's journal turns out to be substantively defensible. The audit's critique was high-quality on every point; my integration was nearly correct. The single qualification needed (F8 in preamble) is a framing nuance, not a substantive error. This is informative: the rate-of-acceptance is not by itself a reliability signal — what matters is whether each acceptance survives independent scrutiny, and 8 of 9 do.

This finding has a v2 design implication on its own: when audit-derived material is integrated, the integration step should *itself* be subject to a follow-up adversarial pass — but not because rate-of-acceptance is suspect, rather because that's how blind-spot recovery works in general. The cycle 3 worry was right to flag the rate; the cycle 4 examination shows the underlying acceptances were sound.

## What carries forward to cycle 5+

- Apply the F8 qualification edit in the shared-root preamble this cycle.
- Apply the minor sharpenings (F12 cross-ref to F1/F5, measurement principle "factual claims" tightening) this cycle if time permits, otherwise cycle 5.
- F1 measurement (separate notes file `cycle-4-f1-measurement.md`) — independent work, also this cycle.
- F12 catalog completion (state.json field categorization) — multi-cycle work, defer to cycle 5+.
- Copilot feedback-only dispatch — defer to cycle 5 so it lands on a retrospective that includes the F1 measurement and the cycle-4 corrections.
