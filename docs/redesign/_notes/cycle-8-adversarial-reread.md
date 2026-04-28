# Cycle 8: Adversarial re-read of cycle-7 integrations

Cycle 7 (a20e6a8c) integrated Copilot feedback PR #2749 — 27 accept / 10 qualify / 0 dismiss across 37 findings. The retrospective grew 1034 → 1330 lines (+296). Cycle 7's evaluation document (`cycle-7-copilot-feedback-evaluation.md` lines 302-310) explicitly queued five adversarial checks for cycle 8. This document executes those checks and a sixth check (length / refactor-for-length) that cycle 7 separately flagged as "now overdue."

The cycle-3 worry-and-verification pattern: a 73% acceptance rate is high enough that cycle-by-cycle adversarial scrutiny matters more than acceptance-rate sentiment alone. Cycle 4 found 1 framing correction in 9 cycle-3 integrations (~11% issue-find rate). Cycle 8 finds two substantive issues plus two minor ones in five checks (~40% issue-find rate); the higher rate is consistent with cycle 7's larger and less-mature integration scope.

## Check 1 — Does the failure-family refactor improve clarity, or introduce indirection?

**Finding: real, substantive.** The family framing now appears in three places:
1. **Family table** at lines 120-125 (4 rows × 3 columns; gives core-mechanism for each family).
2. **Per-section family annotations** on every F-section header (e.g., "Constraint accretion as first-line response (F1) — *Defense accretion family*").
3. **"Reconciliation asymmetry and defense accretion: the families in detail"** section at lines 887-997 (110 lines).

Items 1 and 2 are the cycle-7 integration plan's design (per evaluation doc 1.A/1.B/1.C/1.E). Item 3 is the integration as-implemented; it duplicates what's already in F-sections rather than synthesizing higher-level claims.

Specific overlap. The detailed family section's F2/F3/F4/F11 manifestation summaries (lines 910-926) are 3-4 line restatements of evidence that's already in the F-sections themselves. Example:
- Detailed-section line 910-912: "**F2** — Eva-response detection: outbound (filing question-for-eva) is well-toolified; inbound (Eva replies) has no tool path. Eva's 6-day batch response went unread."
- F2 section line 240-272 contains the same finding with full evidence.

What is genuinely new in the detailed section, not present in the table or F-sections:
- F5 intersection note (lines 961-965): F5 is dual-membered between defense-accretion (storage) and reconciliation-asymmetry (write-mostly).
- F8 family-placement note (lines 967-975): F8's "fewer tools doing each job" prescription stands independently of reconciliation-asymmetry; F8 is in tooling-fragility family, not adjacent to reconciliation-asymmetry.
- Two design-implication blockquotes (lines 980-991): every state field needs a write+reconciliation pair; cycle boundaries should be checkpoint markers not state hard-boundaries.

The new content is ~26 lines; the recap is ~85 lines.

**Cycle 8 edit.** Condense the families-in-detail section to ~30 lines: keep the new content (F5 intersection, F8 placement, design implications), remove the manifestation summaries (already covered by family annotations on F-section headers).

## Check 2 — Does the freeze-vs-refresh formulation cover F1 cleanly, or strain to fit?

**Finding: real.** Cycle 7's integration plan (evaluation doc 2.D) committed to:
> "Replace the 'different observables of the same architectural bug' formulation with Copilot's freeze-vs-refresh formulation as the primary statement. Add a follow-on paragraph noting that this core bug has additional manifestations at the prompt-layer (F1) and as a cross-substrate pattern (F12), without forcing those into the freeze-vs-refresh frame."

The "without forcing those into the freeze-vs-refresh frame" qualification did not survive into the F11 architectural-implication paragraph as committed. The closing sentence at lines 715-718 reads:

> "F1's constraint-accretion (the prompt-layer face) and F12's cross-substrate defense catalog (the meta face) sit upstream of this temporal manifestation: F1 produces defenses; F12 catalogs them; F5 stores them as state; F11 is when the freeze-vs-refresh timing collision occurs."

This implies F1 → F12 → F5 → F11 is a single causal chain whose endpoint is the freeze-vs-refresh timing collision. F1 is structurally a *response-shape* problem (failures get encoded as constraints rather than tools). The freeze-vs-refresh mechanism is specifically about timing (refreshers fire after artifacts freeze). F1 contributes some of the defense surface that gets caught by the F11 timing collision, but F1's own root failure is not a timing problem.

The chain is plausible as a "stages of defense-accretion broadly" framing. It is misleading as a "stages of the freeze-vs-refresh mechanism" framing. As written, it conflates the two.

**Cycle 8 edit.** Soften the F11 architectural-implication paragraph. Keep the freeze-vs-refresh formulation as F11's local mechanism. Distinguish F1/F12/F5 as parallel manifestations of defense-accretion (not as upstream stages of the freeze-vs-refresh mechanism). The "follow-on paragraph" the integration plan promised needs to actually exist as a follow-on, not get folded into the F11 mechanism statement.

## Check 3 — Are the success-criteria demotions losing important Phase-2 anchors?

**Finding: not substantive.** The smell-test framing for the 4× state-surface and ≥1/5 schema-PR thresholds preserves their failure-direction ("if v2 still requires ~40 top-level bookkeeping structures, the redesign has likely failed") without overclaiming the success direction. The 9 candidate v2 measure-shapes from Copilot lens 3.B/E are correctly positioned as Phase-2 inputs (named candidates, threshold design deferred).

The success-criteria section is acceptable as-is for the post-retrospective checkpoint. Phase 2 candidate-design authors will need to pick from the 9 measure-shapes and propose thresholds; that is on Phase 2 work, not on the retrospective.

**No cycle 8 edit on this check.**

## Check 4 — Is the self-congratulation sweep over-correcting?

**Finding: minor over-correction (one location).** At lines 1059-1062, the cycle 7 sweep left a meta-residue note in the "What appears to be working" section's working-notes-pattern entry:

> "*Note*: observations about 'emergent behavior' or 'process maturity' are process commentary and belong in the README iteration log, not in this section (cycle 7 sweep per Copilot feedback PR #2749 lens 4.A)."

This is itself process commentary (a meta-comment about what should be where, attributing the decision to a specific cycle and Copilot lens). The relevant content was moved; the note explaining the move should not be permanent body text. The cleanest fix is to delete the note entirely.

The iteration plan section (1287-1330) has cycle-by-cycle progress accounting that is appropriate for an iteration log (cycle 5 named connection at count level; cycle 6 added timing evidence; cycle 7 reframed per Copilot lens 2). That is descriptive, not self-congratulatory; keep.

**Cycle 8 edit.** Delete the meta-residue note at lines 1059-1062.

## Check 5 — Does the glossary decode jargon, or gloss in equally-jargony terms?

**Finding: two small issues; full sweep remains queued.**

Issue 5.1 — **C5.5 forward-references F4.** Line 22-25:
> "C5 is 'freeze the worklog,' C5.5 is the late-stage post-close validation gate added after F4 surfaced (it validates the close-out happened correctly), C6 is 'dispatch the review,' C8 is 'write the journal entry.'"

A reader using the glossary to bootstrap into the document hits "F4" before they know what F-patterns are. Decode in operational terms: "C5.5 is a validation gate that runs at close-out and checks the C5 freeze produced a coherent worklog."

Issue 5.2 — **F-pattern entry is circular.** Line 48-50:
> "**F-pattern** — one of the 12 named failure patterns cataloged below (F1 through F12). They are not equally independent; cycle 7's failure-family preamble groups them."

The decoding ("named failure patterns F1-F12") restates the abbreviation. Either rewrite to give an operational hook (e.g., "Each F-pattern is a named failure mode in v1's behavior, with cycle citations and a hypothesis about its root cause; the families section above groups them.") or remove the entry since the F-pattern presence is self-evident from the next section.

Issue 5.3 — **Significant terms missing from glossary.** "Worklog," "review agent," "step comments," "pipeline-check," "C5.5 gate" are central to the retrospective but not in the glossary. The cycle 7 plan was 6-10 entries; full sweep deferred to cycle 8+. Adding 4-5 entries in cycle 8 would substantively shrink the legibility gap; the full prose-replacement sweep through the body remains larger work.

**Cycle 8 edit.** Fix C5.5 wording, rewrite F-pattern entry, add 4 entries (worklog, review agent, step comments, pipeline-check). Total glossary expansion ~12 lines.

## Check 6 (additional) — Length / refactor-for-length

Cycle 7 named the refactor-for-length sweep as "now overdue." The artifact is 1330 lines. Section breakdown:
- Header + Glossary: 50 lines
- "What v1 actually is": 48 lines
- Failure families preamble + F-pattern sections: 780 lines (59%)
- Families-in-detail section: 111 lines (8%)
- "What appears to be working" / "What might appear to": 93 lines
- Open questions: 59 lines
- "What should be preserved": 12 lines
- "What v2 must demonstrably do better": 110 lines (8%)
- Iteration plan: 44 lines

The families-in-detail section (Check 1) is the highest-value cut: ~80 lines of recap can be removed without losing claims, only restating-of-claims. The F-pattern sections themselves resist compression — each holds specific evidence with cycle citations.

Net cycle-8 expected change: -70 to -80 lines from the families-in-detail condensation, +12 lines from glossary expansion, -4 lines from meta-residue deletion, plus minor F11 paragraph rework that could be flat. Estimate: 1330 → 1255-1275 (~5% reduction). Substantive but not a full refactor. The full refactor-for-length sweep (compressing F-pattern sections themselves) remains queued; a single cycle should not attempt to compress sections that hold the artifact's primary evidence base.

## Adversarial check on these checks (cycle 9+ pre-commit)

Per cycle 4's discipline, the cycle-8 corrections should themselves be re-examined in cycle 9+:
- Does the families-in-detail condensation lose family-level synthesis the F-section annotations don't carry? Re-read the condensed section against a cold reader and check whether family relationships still come through.
- Does the F11 paragraph correction over-separate F1 from defense-accretion? F1 IS in the defense-accretion family; the correction is about not putting F1 in the freeze-vs-refresh causal chain specifically. Check that family membership remains clear.
- Did the glossary additions actually decode the terms operationally, or did they just gloss in different jargon? (The same trap Copilot warned about.)

If any cycle-8 edit doesn't survive cycle-9 re-read, demote or revert per the iteration plan's "demote what doesn't survive" rule.

## What this re-read did NOT cover

- The cycle-7 self-congratulation sweep through journal entries (cycle 7 evaluation doc 4.D: "Adversarial sweep over the iteration log and the journal entries' reflective-notes sections"). The journal entries themselves were not re-read for surviving self-praise. Deferred to cycle 9+.
- The full legibility sweep (Lens 7 full pass replacing jargon with prose). Deferred to cycle 9+ multi-cycle work.
- The F6/F8/F9 measurements (cycle 7 lens 5.A/B/C deferrals). Deferred independently; not in scope for an adversarial re-read of cycle-7 integrations.
- A second-Copilot-feedback dispatch on the post-cycle-7 retrospective. Cycle 7 evaluation doc closing section flagged this as cycle 8+ work. Currently deferred — the cycle-8 adversarial re-read produces in-house findings sufficient for this cycle; an independent second opinion remains valuable but is not blocking.
