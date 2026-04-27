# Cycle 7: Evaluation of Copilot's cycle-6 feedback critique

This document evaluates each substantive finding in `docs/redesign/_notes/cycle-6-copilot-feedback.md` (PR [#2749](https://github.com/EvaLok/schema-org-json-ld/pull/2749), opened 2026-04-27 by gpt-5.4 in response to the cycle-6 feedback-only dispatch).

The evaluation method mirrors cycle 3's [audit-#442 integration](cycle-3-audit-442-integration.md): each finding is classified as **accept** (clearly correct, integrate), **qualify** (partially correct, integrate with caveat), or **dismiss** (not correct, document why); each accepted/qualified finding is paired with an integration plan; cycle-7 vs cycle-8+ scope is explicit.

The cycle-3 worry-and-verification pattern applies: a high acceptance rate is not by itself a reliability signal — what matters is whether each acceptance survives independent scrutiny per-item.

## Summary

| Lens | Findings | Accept | Qualify | Dismiss |
|---|---|---|---|---|
| 1. Organizational structure | 5 | 4 | 1 | 0 |
| 2. F1+F5+F11+F12 mechanism | 4 | 3 | 1 | 0 |
| 3. v2 success criteria | 5 | 4 | 1 | 0 |
| 4. Self-congratulation | 4 | 3 | 1 | 0 |
| 5. Evidence sufficiency | 6 | 4 | 2 | 0 |
| 6. What's missing | 7 | 4 | 3 | 0 |
| 7. Legibility | 6 | 5 | 1 | 0 |
| **Total** | **37** | **27** | **10** | **0** |

Acceptance-rate flag (cycle 3 pattern): 27/37 = 73% accept, 10/37 = 27% qualify, 0% dismiss. Lower than cycle 3's audit-#442 ~100% acceptance, but still high. The same per-item adversarial discipline applies — see "Adversarial check on the integration step" at the end of this document.

## Lens 1 — Organizational structure

### 1.A — "Not 12 patterns, more like 6–8 with several near-duplicates"

**ACCEPT.** This is the strongest single finding in the critique. The cycle-7 adversarial re-read of cycle-6's cluster analysis (already on the cycle-7 open thread) was going to ask roughly the same question; Copilot's lens-1 critique converges with that internal concern from a different starting point. The convergence is itself evidence the finding is real, not anchored to either side's prior framing.

The retrospective's existing shared-root preamble (F2/F3/F4/F5/F11 with F8 adjacent) already implicitly admits five F-patterns share a common root — Copilot's critique is that this admission should be pushed further: the F-patterns *named separately* are not equally independent.

**Integration plan (cycle 7).** Refactor the F-pattern presentation to introduce a top-level "failure-family" grouping above the F-patterns, with each F-pattern presented as a manifestation within its family. Copilot's suggested grouping is a reasonable starting point; the cycle-7 work will refine it after considering the alternatives the next four findings name.

### 1.B — F2+F3+F4+F5+F11 are mostly one failure family with different manifestations

**ACCEPT.** This sharpens 1.A. Copilot pulls the load-bearing sentence from the shared-root preamble — *"Once a record is written ... no tool reads it back to update its state when subsequent events change its meaning"* — and observes that this single sentence is clearer than the five separate sections trying to unify it.

The five F-patterns' manifestations are usefully distinct (different scales, different substrates, different surfaces), so consolidation into one F-pattern would lose information. But the family relationship needs to be promoted to the top level rather than buried in a preamble.

**Integration plan (cycle 7).** Promote the shared-root preamble's content to a named family ("Reconciliation-asymmetry / write-mostly state") that introduces F2/F3/F4/F5/F11 as instances. The F-patterns retain their identities and individual evidence, but the family framing is the load-bearing claim and gets first-line presentation.

### 1.C — F1+F12 with F5 between them is a 3-stage defense-accretion stack

**ACCEPT.** Copilot's specific framing — (1) response = add defense, (2) storage = state accretes defense residue, (3) temporal = defenses keep mutating state after close — is materially more legible than the retrospective's current "F1+F5+F11+F12 mechanical connection" formulation. It maps the same patterns onto a single causal chain rather than a graph of cross-references.

This finding does **not** conflict with 1.B's family unification — F5 is genuinely positioned at the intersection of "defense accretion" (storage consequence of defenses) and "reconciliation asymmetry" (write-mostly state). The retrospective should acknowledge F5's dual membership rather than force-fitting it into one family.

**Integration plan (cycle 7).** Introduce a "Defense accretion" family (F1, F5, F12, F11) presented as a 3-stage stack: response/storage/temporal. F5 is noted as a member of both the defense-accretion family (storage stage) and the reconciliation-asymmetry family (write-mostly shape). F11 sits in both families too — its temporal aspect is defense-accretion stage 3, and its post-close-divergence aspect is reconciliation-asymmetry.

### 1.D — "F10 probably does not belong as a peer 'failure pattern' at all"

**QUALIFY.** Copilot's read is half-right: F10 is structurally different from F2 or F8 (it's a corrective-hypothesis-and-design-implication pattern, not an operational failure pattern). But demoting F10 entirely loses the v2 design-input value: F10 names a specific property (audit's broader read scope) that v2 must preserve, and the property-1/property-2 distinction (load-bearing read scope vs illusory model diversity) is itself a design constraint.

The right move is to relabel F10 as a "design-implication pattern" rather than a "failure pattern" — its content stays, its category changes. This connects to lens-1.E's broader observation about the F-pattern naming scheme.

**Integration plan (cycle 7).** Re-label F10's section with a clarifying header ("Design implication, not failure pattern: audit's value is broader read scope, not different model perspective") and place it in a new section after the failure-family groupings rather than within them. Preserves the substance; corrects the category.

### 1.E — F-pattern numbering scheme hurts comprehension once cross-pattern claims start

**ACCEPT.** The "F1+F5+F11+F12" + "F8 is adjacent" formulation does ask the reader to decode a graph rather than read an argument. The names *do* help more than the numbers — "constraint accretion" is more memorable than "F1," "late-stage defense accumulation" is more memorable than "F12."

**Integration plan (cycle 7).** Within each F-pattern section, lead with the named pattern and de-emphasize the F-number. Within cross-pattern claims, refer to patterns by name rather than by number where possible (e.g., "the constraint-accretion / defense-accumulation / cycle-closure / state-shape stack" instead of "F1+F5+F11+F12"). Keep F-numbers as anchor IDs for cross-references but reduce their textual prominence.

## Lens 2 — F1+F5+F11+F12 mechanical claim

### 2.A — "Plausible but under-proved" — text leans too much on category overlap

**ACCEPT.** This converges with cycle-6's mechanism-sharpening work (timing-evidence verification of `verify-review-events` 9–11min post-close in 3/3 cycles, etc.) but Copilot's read of the *post-cycle-6* retrospective text indicates the sharpening did not go far enough. Some of the count-leading framing survived the cycle-6 edits.

**Integration plan (cycle 7).** Re-read F11's load-bearing paragraph and the shared-root preamble's mechanism statement; demote any sentence that leads with count and rewrite to lead with named-tool→specific-field→specific-trigger.

### 2.B — Strongest evidence: "Behavioral fix in lieu of tool dispatch"; F11's tool→field→trigger framing

**ACCEPT (informational).** Copilot's identification of the strongest mechanism-evidence sentences confirms cycle-6's sharpening direction. The "Behavioral fix in lieu of tool dispatch" self-disclosure was already cited as cycle-4's existence-proof finding; the F11 timing-paragraph was added in cycle 6.

**Integration plan (cycle 7).** No direct edit needed; this finding informs the cycle-7 demotion in 2.A by naming what the strong sentences look like (so the demoted weak sentences are demoted *toward* this shape).

### 2.C — Weakest evidence: "4 of 5 fields...defense fields"; "makes connection mechanical"

**ACCEPT.** Cycle 6's mechanism-sharpening note (`_notes/cycle-6-mechanism-sharpening.md`) explicitly named the count→mechanism distinction, but the retrospective text retained some count-leading sentences that the sharpening edits did not catch. The cycle-7 sweep is to re-read with this lens specifically.

**Integration plan (cycle 7).** Combined with 2.A — a single edit pass on F11/F5/F12/preamble paragraphs that demotes count-leading sentences and promotes mechanism-leading sentences.

### 2.D — Suggested cleaner framing

> "Several v1 defenses are implemented as end-of-cycle or next-cycle refreshers. Because artifacts freeze before those refreshers finish, the architecture guarantees post-close divergence between frozen artifacts and live state."

**QUALIFY.** This is materially cleaner than the current text's "different observables of the same architectural bug" formulation, and it pins the bug on freeze-timing-vs-refresh-timing in a way that doesn't require the F-number graph. Copilot's framing is the right shape.

The qualification: the current retrospective also makes a broader claim (the same architectural bug presents at four different scales: prompt-layer F1, state-shape F5, cross-substrate F12, temporal F11). Copilot's cleaner framing covers F11+F5 directly but is narrower than F1's prompt-layer manifestation. The integration should adopt Copilot's framing for the cross-cutting *core* claim, while preserving the wider observation that the same bug shows up at multiple scales as a separate adjacent point.

**Integration plan (cycle 7).** Replace the "different observables of the same architectural bug" formulation with Copilot's freeze-vs-refresh formulation as the primary statement. Add a follow-on paragraph noting that this core bug has additional manifestations at the prompt-layer (F1) and as a cross-substrate pattern (F12), without forcing those into the freeze-vs-refresh frame.

## Lens 3 — v2 success criteria plausibility

### 3.A — 4× state-surface reduction is a heuristic, not a metric

**ACCEPT.** Cycle 5's own uncertainty section flagged this ("Whether the v2 success criterion (4× state-surface reduction) is a load-bearing claim or an aspirational one"). Copilot's lens-3 critique is the external-reader version of the same observation: the number is gameable (collapse 40 fields into 5 giant ledgers), the proxy is weak (top-level key count ≠ conceptual complexity), the estimate is approximate by its own admission.

**Integration plan (cycle 7).** Demote the 4× number from "v2 success criterion" to "back-of-envelope estimate showing what's possible given the catalog." Mark explicitly as not load-bearing for Phase 2 candidates. Move the relevant text from "What v2 must demonstrably do better" to "What v2 might plausibly achieve, as a smell test."

### 3.B — Better measures: mutable-state-concepts requiring reconciliation; bounded-vs-append-only retention; transitions after cycle complete; tools allowed to mutate state-of-record

**ACCEPT (defer to cycle 8+ for substantive design).** Copilot's suggested measures are closer to the actual complaint than top-level key count. Each is itself worth designing as a Phase 2 input. Cycle 7 will note these as candidate v2 success measures; the actual design (which measures, how computed, what thresholds) is Phase 2 work.

**Integration plan.** Cycle 7: add a placeholder section in "What v2 must demonstrably do better" listing Copilot's suggested measure-shapes as candidates (without committing to specific thresholds). Cycle 8+ or Phase 2: substantive design work to pick measures and thresholds.

### 3.C — ≥1/5 schema-source PR threshold has same problems

**ACCEPT.** Copilot identifies the same gameability concerns the retrospective itself partially named in cycle 3 (gameable by trivial commits, batchable, doesn't measure effort allocation). Copilot adds: "may fail in the opposite direction once the remaining schema backlog becomes sparse" — the retrospective named the finite-types concern, but did not name the *direction* of failure (the threshold becoming impossible to meet, not impossible to game).

**Integration plan (cycle 7).** Demote the ≥1/5 threshold from "starting target" to "smell test." Acknowledge the bidirectional failure mode (gameable upward; impossible downward as backlog sparses).

### 3.D — "Fraction of cycle compute spent on domain output" is closer but unmeasurable

**ACCEPT.** Copilot identifies what was already a known limitation (cycle 3's uncertainty: "what constitutes cycle compute spent on a thing?"). The framing is the right direction; the implementation is not yet specified.

**Integration plan (cycle 7).** Acknowledge in the success-criteria section that the load-bearing measure (compute-fraction-on-domain-output) is shape-correct but mechanism-undefined. Phase 2 must specify the mechanism (what counts as cycle compute, how attributed) before the measure can be load-bearing.

### 3.E — Better measures: PR-mix share, backlog burn-down, median-cycles need→landed, intervention-ratio, abandonment/retry rate

**QUALIFY.** Each suggested measure is plausible. Some (PR-mix share) are already implicit in cycle 2's measurement (zero domain-PR commits in 46 cycles); some (backlog burn-down) require backlog tracking that doesn't currently exist; some (median-cycles need→landed) are interesting but require a "need-identified" event the system doesn't currently emit.

The qualification: each suggested measure has its own measurement-cost, and a long list of candidate measures is not better than a short list of well-chosen ones. Cycle 8+/Phase 2 will pick; cycle 7 records the candidates without scoring them.

**Integration plan.** Same as 3.B — add candidate measure-shapes to the placeholder section; Phase 2 designs the actual measure.

## Lens 4 — Self-congratulation detection

### 4.A — "What appears to be working" — note-files producing emergent behavior (process self-praise)

**ACCEPT.** The relevant text — "the lightweight per-cycle working-notes pattern ... is producing emergent behavior" plus the audit-#442 "lightweight scaffolding" quote — is process commentary, not a load-bearing operational claim. It belongs in the journal/iteration-log, not in the body of the retrospective.

**Integration plan (cycle 7).** Move the note-file process-emergence text from the retrospective body to the README iteration log (where process commentary is appropriate). The retrospective's "what appears to be working" section keeps only operational claims.

### 4.B — README iteration log phrasings: "8 of 9 hold up unchanged"; "100%-acceptance rate flag examined and resolved"; "made the connection mechanical"

**ACCEPT.** The flagged phrasings are author-grading-own-sophistication, not neutral summaries. The right tone for an iteration log is "what changed" + "source of evidence," not "we did this well."

**Integration plan (cycle 7).** Edit the README iteration log entries flagged: replace "hold up unchanged" with neutral descriptive language; replace "examined and resolved" with "examined; outcome documented in [notes file]"; replace "made the connection mechanical" with "added timing evidence for the mechanism claim." Keep the log informative; remove the self-grading.

### 4.C — v2 success-criteria section: "the cycle 2 F7 measurement is what this looks like in practice" (process-praise)

**ACCEPT.** The phrasing valorizes the discipline rather than presenting the measurement. "F7 was sharpened from impression to measurement in cycle 2 (zero schema commits across 46 cycles)" is the substance; "this is what the discipline looks like in practice" is the meta-comment.

**Integration plan (cycle 7).** Rewrite the relevant sentence to present the measurement directly without the meta-commentary. The discipline-named-as-discipline lives in the iteration plan section, where it's appropriate.

### 4.D — Subtle pattern: treating willingness-to-demote as evidence of maturity

**QUALIFY.** Copilot is right that this is a subtle pattern. But it's also genuinely the case that documenting demotions and corrections is good practice — the question is whether the documentation reads as "we corrected ourselves; that's evidence this process works" (process-praise) or "we corrected ourselves on item X for reason Y" (substantive record). The former is the failure mode; the latter is appropriate.

The qualification: the integration shouldn't strip *all* willingness-to-demote signals — they're useful for the audit-and-Eva-review process. The integration strips only the *generic* "willingness-to-demote is evidence of maturity" framing, retaining specific corrections-with-reasons.

**Integration plan (cycle 7).** Adversarial sweep over the iteration log and the journal entries' reflective-notes sections; identify sentences that praise the discipline generically vs document specific corrections substantively; rewrite the former, retain the latter.

## Lens 5 — Evidence sufficiency

### 5.A — F6 (procedure depth) is impressionistic, no measurement

**ACCEPT.** Cycle 1's F6 is one of the impressionistic sections that has not received the cycle-2/cycle-4/cycle-5 measurement-treatment. "20+ comments before any actual work" is imagery, not data. Copilot is right.

**Integration plan (cycle 8+).** Run a cycle-history measurement: comments-per-cycle distribution before-and-after a representative phase boundary. Cheap measurement; comparable to cycle-2's F7 measurement. Add to F6 in cycle 8+. (Defer because cycle 7's primary work is the organizational refactor + mechanism cleanup, not new measurement.)

**Integration plan (cycle 7).** Add a "F6 evidence is impressionistic; cycle-8+ measurement planned" caveat to the F6 section.

### 5.B — F8 (abandonment cascades) — "fewer tools doing each job" is larger prescription than evidence

**ACCEPT.** Cycle 1's F8 has good cycle-citation evidence (cycle 524, etc.) but the prescription "v2 should have fewer tools doing each job" generalizes from one duplication-and-cascade case to a broad design principle. The principle may still be right, but the retrospective's evidence is narrower than its prescription.

**Integration plan (cycle 7).** Qualify F8's prescription: "the cycle-524 case demonstrates the cascade mechanism; whether tool-duplication is widespread enough to merit a system-level 'fewer tools per job' principle requires the F12 catalog sub-(c) work (checklist + prompt + ADR rationales) which has not yet quantified the duplication rate." Promote the duplication-rate measurement onto cycle-8+ work list.

### 5.C — F9 (review treadmill) — "most cycle compute on loop's outputs" stronger than evidence

**ACCEPT.** F9 has F7 indirect support (PR mix tilted to self-management), but the stronger claim "most cycle compute is spent on the loop's own outputs" requires direct compute-attribution. The retrospective acknowledges in cycle 3 that compute-attribution is not yet defined; F9 should not lean on a measure that does not exist.

**Integration plan (cycle 7).** Demote F9's "most cycle compute" sentence to "the PR mix and the cycle-2 measurement suggest a substantial fraction; direct compute-attribution measurement is Phase 2 work." Same caveat as 3.D.

### 5.D — F10 — small number of examples; corrective hypothesis is strong but not stable general law

**QUALIFY.** Copilot is right that F10's evidence base (audit's #439/#437/#427/#442 + the property-distinction analysis) is a small sample for a general law. But F10 isn't claiming to be a general law about audit-as-peer relationships; it's a specific corrective on the cycle-1 mis-framing. The narrower claim — "the cycle-1 framing of audit's value was wrong; the correction is broader-read-scope vs different-model-perspective" — *is* well-supported by the available evidence.

The qualification: re-read F10 to check whether the text overgeneralizes from "audit added value here in this way" to "audit-as-peer always adds value this way." The general law version is over-claimed; the specific correction is in scope.

**Integration plan (cycle 7).** Sweep F10's text for general-law sentences; replace with the specific-correction framing.

### 5.E — F12 (defense accumulation) — "each defense is load-bearing now" overstates without tested removal

**ACCEPT.** The catalog proves accumulation, not load-bearingness. Copilot is exactly right: the only way to verify a defense is load-bearing is to test removal (which would require a v1 controlled experiment that has not been done). Some defenses may be stale, ceremonial, or dead residue.

This is a meaningful qualification. The cycle-5 catalog demonstrated 19 D fields out of 42; the cycle-6 sub-(b) demonstrated 22 D pipeline-checks out of 30. Both findings are about the *shape* of the defense surface, not about its *load-bearingness*. The retrospective conflates the two.

**Integration plan (cycle 7).** Edit F12 to qualify "load-bearing" claims: the catalog proves accumulation; load-bearingness requires removal-testing. Add a sub-finding: identifying which defenses are load-bearing-vs-stale is itself v2 design work, since v2 must decide which defenses to preserve and which to drop.

### 5.F — F5/F11 cross-interpretation — broader architectural story over-interpretation

**QUALIFY.** Copilot's read: the 42-key catalog supports "state accretion exists"; the 3-cycle post-close sample supports "post-close mutations are real"; the architectural story building on top is over-interpretation. The integration with 2.D's freeze-vs-refresh formulation addresses this — the over-interpretation is the "different observables of the same architectural bug" framing; the cleaner framing fits the evidence directly.

The qualification: cycle 6's mechanism-sharpening did establish the timing-trigger evidence (named tools, named fields, named triggers, post-close timing). That's beyond "post-close mutations are real" — it's "specific tools fire post-close because their triggers are post-close events." The over-interpretation is in the framing-around-the-evidence, not in the evidence itself.

**Integration plan (cycle 7).** Combined with 2.A, 2.C, 2.D — a single edit pass on the F5/F11 sections that adopts Copilot's freeze-vs-refresh framing.

## Lens 6 — What's missing

### 6.A — Impact ranking by severity / cost

**ACCEPT (defer to cycle 8+).** The retrospective presents F2/F7/F8 flatly. F7 (zero domain output across 46 cycles) is more directly mission-damaging than F8 (abandonment cascades from one tool defect). The flat presentation under-ranks the real damage. Adding an impact-ranking is a substantive new section, not a small edit.

**Integration plan.** Cycle 7: note in the iteration plan section that impact ranking is missing and queued for cycle 8+. Cycle 8+: design and add the ranking.

### 6.B — Cost / economics analysis

**QUALIFY (defer to cycle 8+).** Copilot is right that the retrospective does not address token spend, CI minutes, review churn, etc. But adding it is more than a section addition — it requires actual cost-accounting work that cycle 7 cannot complete in scope. The retrospective should note the gap and the Phase 2 work-needed.

**Integration plan.** Cycle 7: add a brief section "What this retrospective does not cover" listing cost/economics as a known gap. Cycle 8+ or Phase 2: substantive cost analysis.

### 6.C — Human-in-the-loop design stance

**ACCEPT (defer to cycle 8+).** The Eva channel is discussed mostly as a polling/reconciliation bug. There is no positive stance on when human intervention should be required, how surfaced, what decisions should never be left to autonomous drift. The retrospective's "what should be preserved" section names the foreground-Eva channel but doesn't articulate the stance.

**Integration plan.** Cycle 7: add to the "what's preserved" section a placeholder for "stance on human intervention — Phase 2 design work." Cycle 8+ or Phase 2: articulate the stance.

### 6.D — What stayed robust across long periods

**QUALIFY.** The retrospective has a "what appears to be working" section, but Copilot is right that it's thin relative to the failure catalog. A retrospective this harsh on v1 needs a more systematic answer to "what kept the system from collapsing entirely across 545 cycles?"

The qualification: the existing "what appears to be working" section names cron-trigger robustness, journal-as-reflective-log surface, Eva channel for foreground intervention, and the persistence mechanism. These are real preservation candidates. The shortcoming is that the section is short; lengthening it requires identifying additional robust elements, not adding commentary on the existing ones.

**Integration plan (cycle 7).** Sweep the v1 production prompt + tools + workflows for robust elements not already named in the "what appears to be working" section. Likely candidates: dispatch-task semantics; the audit-repo's existence as an independent perspective (not just F10's value-add but the bare-fact of its existence as a separate process); the cycle-issue-as-shared-context convention. Add as identified.

### 6.E — Prompt-evolution governance

**ACCEPT (defer to cycle 8+).** F1 names prompt accretion as a failure pattern, but the governance question (how prompts change, who authorizes, how regressions detected, how v2 avoids becoming v1 again) is not addressed. This is meaningful Phase 2/3 design work — the v2 prompt-evolution mechanism has to be designed, not just identified.

**Integration plan.** Cycle 7: add to "what v2 must demonstrably do better than v1" a placeholder for "prompt-evolution governance — Phase 2 design work." Cycle 8+ or Phase 2: design the mechanism.

### 6.F — Parallelism / sub-agent structure

**QUALIFY.** The retrospective barely addresses where parallel execution helped, hurt, or created hidden state problems. Copilot is right that this is missing. The qualification: Copilot dispatches are mentioned (F8's parallel-implementation duplication aspect, F10's audit-as-peer pattern), but a systematic analysis is not done.

**Integration plan (cycle 8+).** A "parallelism patterns in v1" sub-section: where parallel work happened (Copilot dispatches; audit-repo independent process), what worked, what didn't, what state-coordination problems emerged. Cycle 7 notes the gap; cycle 8+ writes the section.

### 6.G — Cross-system comparison

**QUALIFY.** Copilot acknowledges the retrospective defers comparison to Phase 1 and that this is "fair." Adding a grounding comparison point now would risk premature anchoring (cycle 2's argument); waiting for Phase 1 retains the architectural integrity. The qualification: cycle 7 should not pre-empt Phase 1, but the retrospective could note explicitly *which* external systems are queued for Phase 1 reading (the redesign-mode prompt names openclaw and PAI; cycle 7 can confirm).

**Integration plan (cycle 7).** Add to the "what this retrospective does not cover" section that cross-system comparison is queued for Phase 1, naming the systems.

## Lens 7 — Legibility

### 7.A through 7.F — Various jargon / insider references

**ACCEPT (defer most to cycle 8+).** Copilot's specific examples are all valid: "C5/C5.5/C6" is used before the stages are made intuitive; "Recurrence escalation" is not self-explanatory; "chronic-category" requires subsystem knowledge; field names like `step_comment_acknowledged_gaps` cannot be decoded by an outsider; "foreground/background Eva action" is local terminology; `POST_DISPATCH_RECONCILIATION_FIRST_APPLICABLE_PREVIOUS_CYCLE = 545` is dropped without context.

A glossary section + a sweep replacing jargon with prose (or jargon-with-immediate-gloss) is the right fix. This is a substantive editing pass — likely 1-2 cycles of work.

**Integration plan.**
- Cycle 7: add a brief glossary at the top of the retrospective for the most-loaded terms (C-stages, recurrence escalation, chronic-category, foreground/background Eva, dispatch-task, audit repo). 6-10 entries; 1-2 sentences each.
- Cycle 8+: legibility sweep — go through the retrospective text replacing jargon with prose-with-gloss. Multi-cycle work.

### 7-special — "v1 has tools to *create* records and tools to *summarize* records; it lacks tools to *reconcile* records against later events"

**ACCEPT.** Copilot identifies this sentence as one of the most legible parts of the retrospective. The retrospective should *more often* state things this way.

**Integration plan (cycle 7+).** Use this sentence as a model for the framing-style of the failure-family introductions (per 1.A, 1.B). The reconciliation-asymmetry family's framing should be approximately this sentence.

## Integration scope: cycle 7 vs cycle 8+

### Cycle 7 will integrate (the substantive refactor)

Higher-priority items where the Copilot critique converges with the cycle-7 plan:

1. **Organizational refactor**: introduce failure-family grouping; F-patterns become manifestations within families; F10 re-categorized as design-implication. (Lens 1.A/B/C/D/E.)
2. **F1+F5+F11+F12 cleaner framing**: adopt freeze-vs-refresh formulation; demote count-leading sentences; promote mechanism-leading. (Lens 2.A/B/C/D, 5.F.)
3. **Success-criteria demotion**: 4× and ≥1/5 demoted from criteria to smell tests; placeholder for better measures. (Lens 3.A/B/C/D/E.)
4. **Self-congratulation sweep**: edit specific phrasings flagged in the README iteration log + retrospective body. (Lens 4.A/B/C/D.)
5. **F12 load-bearingness qualification**: catalog proves accumulation, not load-bearingness. (Lens 5.E.)
6. **F-pattern caveats**: small qualifications on F6 (impressionistic), F8 (prescription larger than evidence), F9 (compute claim), F10 (general-law overgeneralization). (Lens 5.A/B/C/D.)
7. **Glossary**: brief glossary at top of retrospective for the most-loaded terms. (Lens 7.A/B/C/D/E/F.)

Estimated retrospective change: net likely -50 to +50 lines (some additions for family framing and glossary; some reductions from self-congratulation sweep and demotion of repetitive count-leading text).

### Cycle 8+ will integrate

Lower-priority or substantive-design items deferred:

- Better v2 success measures — *design* the measures, not just name candidates. (Lens 3.B/E.)
- F6 measurement (cycle-history comments-per-cycle). (Lens 5.A.)
- F8 duplication-rate measurement. (Lens 5.B.)
- F9 compute-attribution measurement. (Lens 5.C.)
- Impact ranking by severity / cost. (Lens 6.A.)
- Cost / economics analysis. (Lens 6.B.)
- Human-in-the-loop design stance. (Lens 6.C.)
- Robust-elements sweep. (Lens 6.D.)
- Prompt-evolution governance. (Lens 6.E.)
- Parallelism / sub-agent structure analysis. (Lens 6.F.)
- Legibility sweep through retrospective text (jargon→prose). (Lens 7.A-F, full pass.)

## Adversarial check on the integration step

Per cycle 4's discipline (the adversarial-re-read of cycle 3 audit-#442 integration), this section pre-commits the adversarial check that should happen in cycle 8+ on the cycle-7 integrations.

Specific things to re-examine in cycle 8+:

1. **Does the failure-family refactor actually improve clarity, or does it introduce a new layer of indirection?** A reader unfamiliar with the v1 system should be able to see, at the family level, which v2 design moves resolve which class of failure. If the family layer adds names without consolidating insights, it's making the document longer not clearer.
2. **Does the freeze-vs-refresh formulation cover F1's prompt-layer manifestation cleanly, or does it strain to fit?** F1 (constraint accretion) is structurally different from F5/F11/F12 (state-and-time defenses). The cleaner framing covers F5/F11/F12; F1 is a related-but-not-same pattern. Cycle 7 integration includes a "follow-on paragraph" preserving F1's distinct manifestation, but the framing should be checked for cleanness.
3. **Are the success-criteria demotions losing important Phase-2 anchors, or correctly demoting weak claims?** The 4× number was a useful smell test even when it's not a load-bearing criterion. Demoting it should preserve its smell-test utility, not erase it.
4. **Is the self-congratulation sweep over-correcting?** Some willingness-to-demote signals are useful for the review process (lens 4.D). The sweep should remove the *generic* praise, not the *specific* corrections-with-reasons.
5. **Does the glossary actually decode the jargon, or does it just gloss in equally-jargony terms?** A glossary entry that says "C5.5: the late-stage post-close validation gate" doesn't help an outsider. The glossary entries should ground in operational mechanics (what fires; what writes; when).

If any of cycle 7's integrations doesn't survive these checks in cycle 8+, demote or qualify per the iteration plan's "demote what doesn't survive" rule.

## What this evaluation did NOT cover

Per cycle 4's "what I didn't do" pattern: the explicit gap-acknowledgment.

- This evaluation does not score Copilot's findings against an *independent* benchmark (e.g., audit-repo critique on the same Copilot output). The 73% accept / 27% qualify / 0% dismiss distribution is the orchestrator's own read; whether the orchestrator's own read is itself well-calibrated is not externally verified. (Possible cycle 8+ work: dispatch a second Copilot feedback session with this evaluation document attached, asking specifically whether the accept/qualify decisions hold up.)
- This evaluation does not weight findings by *expected-impact-on-Phase-2*. All findings are evaluated on substance and integration-cost; some accepts (e.g., 6.E prompt-evolution governance) are larger Phase-2 inputs than others (e.g., 4.A note-files emergence) but are not flagged as such. The prioritization in the cycle-7-vs-cycle-8+ split implicitly does this, but the explicit Phase-2-impact analysis is missing.
- This evaluation does not consider whether *Copilot's lens selection itself* was the right frame. The seven lenses came from the cycle-6 dispatch body. A different lens set (e.g., "what would a v2 implementer find missing from this retrospective when designing the new prompt") might surface different findings. Cycle 8+ could re-dispatch with a different lens selection if the cycle-7 integrations leave gaps.
