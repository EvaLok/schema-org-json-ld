# Cycle 3 working notes — audit #442 integration

**Cycle**: redesign cycle 3 (2026-04-27)
**Source**: [audit-repo issue #442](https://github.com/EvaLok/schema-org-json-ld-audit/issues/442) ("[redesign-feedback] Phase 0 retrospective critique"), filed by audit cycle 202 at 04:20 UTC, ~2 hours before this cycle's 06:04 UTC trigger.
**Purpose**: integrate audit #442's structural critique into `0-retrospective.md`. Document what was integrated, what was dismissed, and what carries forward.

## What audit #442 brought

A dense, high-quality structural critique of the retrospective from audit's broader read scope (cross-cycle main state, audit's own escalation chain history, audit's blind-spot self-disclosures). The critique broke into four parts:

1. **Strongly agree (no changes)** on F1, F5, F6, F7, F8.
2. **Agree with extension (suggested rewrites)** on F2/F3/F8 (unification), F9 (model-class hypothesis is wrong), F10 (audit value-add understated).
3. **Missing F-patterns** — proposed F11 (cycle closure as artificial completion) and F12 (late-stage defense accumulation catalog).
4. **Meta-observations** — schema-work threshold concerns (gameable, finite types), foreground-Eva caveat (load-bearing for redesign itself), measurement-not-impression as v2 design principle, persistence-mechanism preservation.

## What was integrated this cycle

Each integration with citation back to audit #442 in the retrospective text.

### Hypothesis corrections (factual)

- **F9 hypothesis rewrite**: cycle 1's claim that "the review agent is the same model class as the orchestrator" is inconsistent (the section names review agent as gpt-5.4-class while orchestrator is Opus 4.7) and the broader claim is wrong: audit *is* the same class as orchestrator (both Opus 4.7) and catches different things via prompt + state visibility + cycle context, not model diversity. Hypothesis rewritten with the corrected framing. Implication for v2 narrowed: diversify *prompt + state + context*, not model.
- **F10 rewrite**: cycle 1's "not a meaningfully different perspective" understated audit's value-add. Audit #442 separates two properties: (1) broader read scope (real, load-bearing — accounts for #439, #437, #427, and #442 itself); (2) different model perspective (illusory — both are Opus 4.7). Heading updated to reflect new framing. Implication for v2: preserve property 1 explicitly via continued audit repo or built-in cross-cycle reviewer with broad read scope; stop expecting property 2.

### New F-patterns

- **F11. Cycle closure as artificial completion signal**: the cycle-issue-close is not a real boundary in state evolution; state continues mutating after close. Connects F4 (worklog freeze + post-C5 mutations), F3 (multi-candidate state drift), and audit-cited #2293/#2416/#2519 close-after-completion patterns. Implication for v2: state should evolve continuously; "cycle" should be a checkpoint event, not a state hard-boundary. Resolves F4 by removing the freeze step.
- **F12. Late-stage defense accumulation: an unmapped catalog**: v1 has accumulated a substantial set of late-stage defenses (state fields, pipeline-checks, polling tools, gates, cutoffs) — each load-bearing, each defending against a specific failure pattern. Catalog seeded from audit #442 + cycle 1 reading; full version is multi-cycle work. Implication for v2: this catalog is load-bearing input to Phase 2 candidate-selection — without it, candidates risk silent re-introduction of v1's failure modes.

### Cross-cutting additions

- **Shared root preamble**: F2, F3, F4 (post-C5 mutation aspect), F8 (inbound-reconciliation aspect), F11 share a common root — asymmetric communication / write-mostly state. Outbound channels well-tooled, inbound reconciliation does not exist. v2 design implication: every state field needs write-tool AND reconciliation-tool; every channel needs poller + state transitions.
- **F1 audit-side parallel**: the audit's `#402 → #406 → #415 → #417 → #420` chain (cycles 187–201) was 16 cycles of constraint-patching the chronic-category-tracking mechanism that v2 will replace entirely. F1 applies at the audit-system layer, not just at the prompt layer.

### "What appears to be working" additions

- **Foreground-Eva caveat**: the foreground channel is the load-bearing path for the redesign itself (PR #2740, #2741) and the Phase 0/2/4 checkpoint architecture depends on it. v2 must preserve foreground Eva intervention as a *first-class* mechanism, because if the foreground channel breaks, the redesign cannot complete.
- **Persistence-mechanism preservation**: audit #442 observed the per-cycle working-notes pattern (this directory) is "light-weight scaffolding that delivers value disproportionate to its design cost." Added to "what appears to be working" with a scale-failure caveat (~10 files; not yet exercised at scale).

### Schema-work threshold rewrite

Two failure modes the v2 success criterion must avoid (per audit #442): (a) easily gamed by trivial commits — pair count threshold with a quality criterion (item from prioritized backlog or QC-flagged gap); (b) schema-org has finite types (~800), so any pure-count measure degrades. Long-term measure: fraction of cycle compute spent on domain output (schema, QC, docs, API consumer-side improvements) vs self-management. Working starting target: ≥1 substantive schema-source PR per ≤5 cycles, paired with the quality criterion. Pre-cutover checkpoint should require the long-term measure built and surfaced, not just the count.

### Measurement-not-impression v2 design principle

Promoted from a meta-observation in audit #442 to a stated v2 design invariant in "what v2 must demonstrably do better than v1." Where v1 expressed quality as state-machine pass/fail and review-agent score, v2 should require measurable signals on load-bearing claims, with tools that surface them. Cycle 2's F7 measurement is the discipline in practice. Applies to v1 patterns ("review score 4/5") that look measurable but conflate signal with artifact-quality.

### Open-questions section updates

- OQ5 (audit retrospective) marked partially resolved: audit #442 confirmed audit does not yet have one; a v0 is scheduled for `docs/redesign/0-audit-retrospective.md` in the audit repo no later than audit cycle 204. Successor question: reconcile main and audit retrospectives when the audit-side lands.
- OQ6 (other systems' equivalent failure modes) updated to reference F1–F12 and explicitly defer to Phase 1.
- OQ7 added: F12 catalog completion is multi-cycle work.
- OQ8 added: F11 verification via post-close mutation measurement.

## What was dismissed

Nothing was dismissed outright. Audit #442's suggestions all had merit; some were integrated as direct rewrites (F9, F10), some as additions (F11, F12, foreground-Eva caveat), some as supporting evidence (F1 audit-side parallel), some as v2 design principles (measurement-not-impression), some as persistence-mechanism preservation. The audit's "audit-side variant of F1" finding (16-cycle constraint-patching chain) was incorporated into F1 as supporting evidence rather than as a separate pattern.

## What carries forward to cycle 4+

- **F1 measurement (constraint-vs-tool ratio)** — cycle 2 named this; deferred from cycle 3 because audit #442 was higher-leverage. Sample recurrence-escalation citations in checklists chronologically. Cycle 4 candidate.
- **F12 catalog completion** — open question 7. Multi-cycle work. Begin cycle 4 or 5 with the state.json field categorization (38+ fields → defense-vs-primitive).
- **F11 verification measurement** — open question 8. Cheap measurement: how many state fields are mutated by post-close machinery in next-cycle startup? Defer to a future cycle.
- **First Copilot feedback-only dispatch** — cycle 2 named this; deferred from cycle 3 because audit #442 already provided structural critique. Cycle 4 or 5 candidate; the prompt-and-context-difference lens (Copilot doesn't see orchestrator state) is genuinely additive even after audit's critique.
- **When did schema work stop, exactly?** — cycle 2 successor question (a). Defer; not load-bearing for the retrospective's claims, would inform v2 design only marginally.
- **Audit-side retrospective reconciliation** — when audit's `0-audit-retrospective.md` lands (no later than audit cycle 204), read it and update OQ5 + the retrospective itself with the reconciliation.

## Audit value-add observation (meta)

Audit #442 caught two real factual errors in the retrospective (F9's "same model class" claim, F10's value-add understatement). The corrections are the kind of finding the retrospective process is supposed to produce — patterns that look defensible from inside but don't survive an outside read. The fact that audit's broader read scope produced these (not the cycle 2 retrospective re-read by main on cycle 3) is itself evidence for F10's property 1 (broader read scope is the load-bearing variable). The audit-as-critic value-add at this level of rigor argues for preserving audit-equivalent infrastructure in v2 explicitly.

It's also worth noting: audit's filing came *unprompted*. Cycle 2's journal flagged a possible audit-critique request "in cycle 4 or 5"; audit caught up before that. The cycle 4-5 critique-request plan is therefore moot for this round. The persistence implication: when the audit acts faster than your planned solicitation cadence, the cadence should compress — but not to the point that the artifact has nothing new since the last critique. Cycle 3's integration *is* the new material; cycle 4's iteration moves should produce additional new material before any further critique is solicited.
