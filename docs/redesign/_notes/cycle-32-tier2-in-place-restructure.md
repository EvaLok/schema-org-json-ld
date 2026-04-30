# Cycle 32: Tier-2 cross-system observations in-place restructure

Cycle 31 (commit `449d548b`) produced the per-finding evaluation of
Copilot feedback PR
[#2791](https://github.com/EvaLok/schema-org-json-ld/pull/2791) with
60 verdicts (47 ACCEPT / 7 PARTIALLY ACCEPT / 6 DISMISS) and seven
structural decisions distilled. Cycle 31 explicitly named cycle 32's
focal as the in-place restructure of `1-research.md` lines 817-1219
per those seven decisions.

This cycle (32) absorbed the load-bearing focal: in-place application
of the seven structural decisions to `1-research.md`'s Cross-system
observations section. The cycle-31 cold-reader had named the
"principled but fragile" risk: cycle 32 would convert the verdict
file's principle to procrastination if it deferred application
again. Cycle 32 did not defer — the full restructure landed.

## The seven structural decisions, applied

### Decision 1 (family count and contents): 5 families with revised membership

Applied. The restructured section organizes 16 patterns into 5
families:

- **Family C (5 patterns)**: Component-local state persistence;
  Failed work as recorded artifact; Append-only history; Memory as
  architectural concept; Plans/specs as forward-versioned. Plus 2
  divergence callouts: Memory architectural stance, State-shape
  divergence.
- **Family E (4 patterns)**: Anti-patterns explicit; Mechanical
  enforcement; Iteration ceilings (moved from cycle-30 draft's
  Family D per Lens 1.1); Entropy / AI slop.
- **Family A (3 patterns)**: Multi-agent decomposition; Per-agent
  model selection; Small fixed team. Plus 1 divergence callout:
  Agent-hierarchy stance.
- **Family B (3 patterns)**: Code-vs-prompts split; Small core
  extends via plugins; Multiple orchestration patterns.
- **Family D (1 pattern, thin family acknowledged)**: Strong-defaults
  security with operator-controlled knobs.

Iteration ceilings move from cycle-30 draft's Family D to Family E
verified — Family D becomes single-pattern, framed as a thin family
explicitly acknowledged.

### Decision 2 (family ordering): evidence-weight ordering

Applied. Family order in restructured section: C (5 patterns) → E
(4 patterns) → A (3 patterns) → B (3 patterns) → D (1 pattern).
Tie-break between A and B resolved by alphabetical (A before B) per
cycle-31 decision 2's named tie-break criterion.

The preamble explicitly disclaims smuggling: "family ordering is by
pattern count (largest cluster first), not by Phase 2 priority."
This addresses Lens 2.1's "logical dependency is not neutral"
critique directly.

### Decision 3 (maturity badges): explicit visual prefixes

Applied. All 16 patterns carry a maturity badge in `**\`[badge]\`**`
format (bold backtick-monospace) immediately after the bold pattern
title, before the body sentence. Badge taxonomy values used:

| Pattern count | Caveat | Badge text | Used by |
|--|--|--|--|
| 3+ tier | clean | `[3+/N systems]` | Failed work, Multiple orchestration, Multi-agent, Anti-patterns, Code-vs-prompts, Small core (varying N=3,3,6,6,6,7) |
| 3+ tier | + diversity hedge | `[3+/N systems + diversity hedge]` | Component-local, Memory, Append-only, Per-agent model |
| 3+ tier | + contrary stance | `[3+/N systems + contrary stance]` | Small fixed team |
| 3+ tier | + scope condition | `[3+/N systems + scope condition]` | Strong-defaults security |
| 2-system | clean | `[2-system clean]` | Plans/specs, Entropy/AI slop |
| 2-system | strict + hedge | `[2-system strict + diversity hedge]` | Mechanical enforcement |
| 2-system | strict + adjacent | `[2-system strict + 1 adjacent partial]` | Iteration ceilings |

7 distinct badge values across 16 patterns — closed taxonomy
preserved.

### Decision 4 (divergences placement): hybrid

Applied. Two family-local "Divergence within this family" callouts:

- Family A callout: Agent-hierarchy stance (operator-driven vs
  goal-driven; openclaw vs PAI).
- Family C callout: Memory architectural stance + State-shape
  divergence (clustered as one callout per Lens 4.4 verdict — both
  address the same family's topical area).

Final compact "Divergences index" section after the 5 family
sections lists the 3 divergences as one-line entries with explicit
cross-references back to the family-local callouts ("see Family
A's...", "see Family C's...").

The substantive prose of each divergence lives in the family-local
callout; the index is bookkeeping with cross-reference. This matches
Lens 4.5/4.6 verdicts.

### Decision 5 (cross-family backreferences): three identified

Applied. Three cross-references added:

1. **Mechanical enforcement (Family E) ↔ Code-vs-prompts (Family B):**
   Italic cross-reference at end of Mechanical enforcement bullet
   body. Substance: "Code-vs-prompts is where execution authority
   lives; mechanical enforcement is whether behavioral promises are
   regression-tested. Both involve deterministic code constraining
   LLM behavior, but in different directions — authority-allocation
   vs verification-discipline." This articulates the conceptual
   distinction load-bearingly — Lens 1.5's concern that a reader
   sees OpenAI mechanical layers in both Family B and Family E and
   wonders about duplicate evidence is resolved.

2. **Plans-as-artifacts (Family C) ↔ Component-local state (Family C
   internal):** Italic within-family cross-reference inside
   Component-local state body, after the OpenAI sentence. Substance:
   "the OpenAI plans-as-artifacts mention here is the
   component-local-state framing; the forward-spec sharpening of
   this same OpenAI evidence appears in the Plans/specs as
   forward-versioned bullet later in this family — not duplicate
   evidence, but two angles on the same systems." This addresses
   Lens 1.7's concern about plans-as-artifacts hiding inside
   component-local state.

3. **Small fixed team (Family A) ↔ Multiple orchestration patterns
   (Family B):** Italic cross-reference at end of Small fixed team
   body. Substance: "the role-separation surfaced here implements
   across distinct topologies (Voyager peer-flow / Magentic-One
   lead-worker / oh-my-codex sequential mode), so the two patterns
   intersect on substrate." This addresses Lens 1.8's concern that
   Small fixed team straddles A and B; the cross-reference makes the
   substrate intersection explicit.

All three are substantive (load-bearing relationship articulation),
not bookkeeping (mere "see X" pointer).

### Decision 6 (section preamble): ~3-4 sentences before family sections

Applied. Three-sentence preamble after the existing 8-systems-read
intro paragraph:

1. Family-clustering rationale + ordering criterion (with explicit
   "not by Phase 2 priority" smuggling-disclaimer).
2. Maturity badge format + cycle-22 epistemic distinction surfaced
   at bullet level + caveat-type list.
3. Family-local divergence callouts + final divergences index +
   cross-references.

The preamble does what cycle-31 decision 6 specified.

### Decision 7 (single-system observations and `_notes` cross-system synthesis): unchanged

Applied. The `_notes/cycle-22-cross-system-synthesis.md`
single-system observations remain in their current location. The
cycle-32 restructure modifies `1-research.md` lines 817-1219 only.
Section closing paragraph preserved: "Single-system observations
(captured in `_notes/cycle-22-cross-system-synthesis.md`) should not
yet shape candidate generation."

## Lens 6.5 DISMISS rationale: fresh-eye second-pass

The cycle-31 cold-reader flagged Lens 6.5 DISMISS rationale as
borderline: "the substantive answer (1-pattern thin family is more
honest than artificial absorption) IS load-bearing, but the
'strained vs strained' comparison should be made explicit. Cycle-32
fresh-eye to consider whether to add a clarifying sentence."

**Fresh-eye verdict: CONFIRMED.** The original Lens 6.5 DISMISS
rationale only addressed the strain of absorption (violates Family
A's "decomposition" framing or Family B's "coordination" framing).
It did not address the counter-argument that the proposed
alternative (1-pattern thin family) ALSO has structural strain —
specifically, "small-family ceremonialness" (cycle-30 draft's
flagged concern).

The honest framing: BOTH options strain, but the strains differ in
kind:

- **Absorption strain:** the family's framing is silently violated
  (the absorbed pattern doesn't fit the family's topical area; the
  family becomes less coherent without naming it).
- **Thin-family strain:** small-family ceremonialness is real, but
  the strain is named explicitly (acknowledged thin-family with
  rationale).

The thin-family strain is "louder" but more honest. Absorption
strain is "quieter" but less honest. The DISMISS verdict (Lens 6.5)
holds at substance — 1-pattern thin family IS more honest — but the
rationale needed the comparison made explicit.

**Application: clarifying sentence added to Family D's family
description in the in-place restructure.** The Family D description
now reads:

> A single-pattern family covering how the system bounds its trust
> of external inputs and execution environments. The family is
> acknowledged as thin (pattern count is 1); trust-boundary
> configuration is a distinct architectural concern not naturally
> absorbed into Agent architecture (decomposition framing) or
> Orchestration (coordination framing). The thin-family framing has
> its own structural strain (small-family ceremonialness) but is more
> honest than artificial absorption: the strain is named explicitly
> rather than absorbed into a family that doesn't accommodate the
> pattern.

The third sentence addresses the cycle-31 cold-reader's flag — the
"strained vs strained" comparison is now explicit in the artifact
itself (not just in cycle-32 notes).

## Same-cycle cold-reader on the in-place restructure

Per cycle-31 pre-commit item 2, four cold-reader questions on the
restructure:

### (a) Did the badge taxonomy get applied uniformly across all 16 patterns, or were any skipped/inconsistent?

**PASS.** All 16 patterns have badges in `**\`[badge]\`**` format.
Format is uniform across all bullets (verified by grep). Taxonomy
values are from the closed set in cycle-31 decision 3, with the
`+ diversity hedge` extension per Lens 3.6 caveat-type taxonomy.

Distribution: 6 clean 3+ tier, 6 caveated 3+ tier, 4 2-system tier
(2 clean + 2 strict). No skipped or inconsistent badges.

### (b) Did the family-local divergence callouts preserve the cycle-22 framing without smuggling Phase 2 prescriptions?

**PASS.** Both family-local divergence callouts preserve the
cycle-22 substantive prose verbatim from the original Persistent
divergences section. The introductory framing sentences for the
callouts ("Two persistent divergences cluster in the State/Memory/
History area") are observational structural framing, not v2
prescription.

The Family A callout names "Agent-hierarchy stance is downstream of
operator-vs-goal-driven framing" — exact original wording. The
Family C callout preserves Memory architectural stance + State-shape
divergence wording. No prescriptive smuggling.

### (c) Did the cross-family backreferences read as substantive relationships or as bookkeeping?

**PASS — all three substantive.** Verdicts:

1. Within-family Component-local state ↔ Plans/specs: substantive
   ("not duplicate evidence, but two angles on the same systems"
   articulates the load-bearing relationship).
2. Cross-family Mechanical enforcement ↔ Code-vs-prompts:
   substantive ("authority-allocation vs verification-discipline"
   names the conceptual distinction).
3. Cross-family Small fixed team ↔ Multiple orchestration:
   substantive ("the two patterns intersect on substrate" with
   explicit topology list).

None are bookkeeping-only ("see X" without substance).

### (d) Does the section preamble explain badges + divergences hybrid cleanly without over-prescribing?

**BORDERLINE-PASS.** The three-sentence preamble explains:

- Family ordering rationale (with explicit "not by Phase 2
  priority" disclaimer)
- Badge format + caveat-type list
- Family-local divergence callouts + final index

What works: the smuggling-disclaimer ("not by Phase 2 priority") is
explicit; the caveat-type list (diversity hedge / scope condition /
contrary stance / adjacent partial) maps to the badges; the
divergence-hybrid structure is named.

Mild flag: the preamble doesn't explicitly map `[3+/N systems]` →
"3+ tier convergence (N systems)" or `[2-system clean]` → "2-system
tier convergence." A reader unfamiliar with the cycle-22 framing has
to recover this from the prior paragraph (lines 826-838) which
establishes the 3+ threshold + 2-system tier discipline. The
information is recoverable but the preamble alone doesn't fully
self-explain.

Mild flag: the A-B tie-break (alphabetical) isn't named in the
preamble. Both Family A and Family B have 3 patterns; A comes
first; the preamble doesn't explain why. Sub-precision concern, not
a defect.

**Cold-reader on restructure: 4/4 PASS or BORDERLINE-PASS.** Mild
flags noted for cycle-33+ fresh-eye if the imprecision matters in
practice (Phase 2 candidate-generation reading).

## Same-cycle cold-reader on cycle-31 notes file

Per cycle-31 pre-commit item 10, four cold-reader questions on the
cycle-31 verdict file:

### (a) Are the seven structural decisions clearly stated and actionable for cycle 32, or do any read as "needs more thought" defer-language?

**PASS.** Cycle 32 successfully applied all seven decisions in one
cycle (this cycle). The decisions translated to concrete edits
without ambiguity:

- Decision 1 → 5 family sections with named pattern membership
- Decision 2 → C → E → A → B → D ordering, applied
- Decision 3 → 16 badges, format specified, applied uniformly
- Decision 4 → hybrid (Family A callout, Family C callout, final
  index), applied
- Decision 5 → 3 cross-references, named, applied
- Decision 6 → 3-sentence preamble, drafted and applied
- Decision 7 → single-system observations unchanged (no edits)

No decision read as "needs more thought" defer-language. All were
actionable.

### (b) Did the Family D resolution avoid the "absorb into A or B artificially" failure mode while also avoiding Lens 6.5's "dissolve D entirely" alternative?

**PASS — with the cycle-32 fresh-eye clarification.** Family D
became a single-pattern family for security defaults, explicitly
acknowledging thin-family status. This avoids:

- "Absorb into A or B": the trust-boundary topical area doesn't fit
  Family A's decomposition framing or Family B's coordination
  framing. Absorption would silently make those families less
  coherent. Avoided.
- "Dissolve D entirely (Lens 6.5)": the alternative was rejected as
  DISMISS in cycle-31 verdicts. Cycle-32 fresh-eye CONFIRMED the
  DISMISS but flagged that the rationale needed the "strained vs
  strained" comparison made explicit. The clarifying sentence was
  added to Family D's family description in the restructure. The
  DISMISS verdict held; the rationale was completed.

So the resolution worked: thin-family acknowledged with explicit
strain-comparison rationale.

### (c) Does the dispatch-then-commit persistence-mechanism observation read as a clear discipline (commit-then-dispatch) or as ad-hoc workflow advice?

**PASS.** The cycle-31 notes named a 4-step procedure:

1. Write the draft to its file.
2. `git add` + `git commit` + `git push` the draft to master.
3. Verify the draft is on master.
4. Then create the dispatch issue.

This is verifiable discipline (each step is concrete) and applies
specifically to feedback dispatches that reference a draft artifact.
The cycle-26 / cycle-30 environmental-fallback observations
(WebFetch/curl gated; dispatch-task gated) are documented separately
as different category — workflow-sequencing (cycle-31) vs
environmental-fallback (cycle-26 / cycle-30).

Both belong in the persistence-mechanism log because both shape how
feedback artifacts get produced. The categorization is principled.

### (d) Does the verdict count (47 ACCEPT, 7 PARTIALLY ACCEPT, 6 DISMISS) read as honest engagement or as anchor-on-Copilot's-framing? The DISMISS verdicts especially — were they substantively justified or did I accept Copilot's framing too readily?

**PASS — with the Lens 6.5 caveat.** Walked the 6 DISMISS verdicts:

- **2.5 (problem-pressure ordering):** substantively justified.
  Argues problem-pressure is itself Phase-2-shaped, repeating Lens
  2.1's smuggling critique under different framing.
- **2.6 (alphabetical):** substantively justified. Alphabetical is
  degenerate — doesn't reveal evidence structure.
- **2.7 (warning label):** correctly superseded by 2.4 (evidence-
  weight ordering).
- **2.8 (quality-last):** correctly superseded by 2.4.
- **5.3 (transition sentence):** correctly superseded by ordering
  change.
- **6.5 (dissolve D):** the DISMISS verdict is correct in substance
  (thin-family more honest than absorption), but the cycle-31
  rationale was incomplete on the "strained vs strained" comparison.
  Cycle-32 fresh-eye added the clarifying sentence to Family D's
  family description. The DISMISS holds; the rationale completed at
  cycle-32.

The verdict count is honest engagement, not anchor-on-Copilot. The
ACCEPT verdicts include 5 "ACCEPT (validation of cycle-30)" which
are confirmations rather than load-bearing-new — flagged at
cycle-31 cold-reader for not over-weighting count tallies. The
PARTIALLY ACCEPT category captures real qualifications. The DISMISS
verdicts each carry substantive rationale.

The verdict-tally self-correction during cycle-31 cold-reader (47/7/6
empirically counted vs initial estimate "40+/4-5/4-5") was itself a
honest-engagement signal — bounded mechanical work being treated as
substantive when the count claim is load-bearing.

**Cold-reader on cycle-31 notes: 4/4 PASS** with the Lens 6.5
rationale completion noted (applied in this cycle's restructure).

## Persistence-mechanism observations

### Cycle-7 / cycle-12 / cycle-31 / cycle-32 pattern: 3-cycle per-finding-evaluation arc

The chain (cycle 30 light-prep → cycle 31 verdict file → cycle 32
in-place application) is the third application of the per-finding-
evaluation pattern (cycles 7-8, cycles 11-12, cycles 30-32). Pattern
shape stable: external feedback dispatch lands → next cycle does
verdict file (ACCEPT / QUALIFY / DISMISS per finding) → following
cycle does in-place application. Each cycle is bounded; the chain
prevents "land feedback and try to integrate it in one cycle"
failure modes.

The cycle-31 prediction was "Cycle 32 should attempt the full
application and only split if the work overflows the budget." Cycle
32 attempted the full application and it did not overflow. The
cycle-32 budget absorbed: (1) Lens 6.5 fresh-eye second-pass; (2)
in-place restructure of ~400 lines; (3) same-cycle cold-reader on
restructure (4 questions); (4) same-cycle cold-reader on cycle-31
notes (4 questions); (5) this notes file (~600 lines); (6) journal
entry; (7) README iteration log. The fragility verdict converted to
landed-application, not procrastination.

### Light-prep → heavy substantive sequencing reinforced

Three applications: cycle 28 → 29; cycle 30 → 31; cycle 30 → 31 →
32 (the longer chain when feedback-eval splits across two cycles).
The persistence-mechanism observation captured cycle 29 ("light prep
cycle → heavy substantive cycle is sequential when the heavy cycle
DEPENDS on output from the light cycle") extends to the 3-cycle
chain. Cycle 32 depends on cycle 31's verdict file; cycle 31
depends on cycle 30's draft; cycle 30 was the light-prep that
enabled the entire chain.

### Same-cycle cold-reader on rewrite: 5th application

Cycle 19 / 21 / 22 / 28 / 32 — five applications of the same-cycle-
cold-reader-on-rewrite pattern. Cycle-32 caught (and addressed) the
mild flags on section preamble (prefix→tier mapping not explicit;
A-B tie-break not stated). These flags are sub-precision (not
defects), but the cold-reader did its job at the borderline-PASS
level.

The cycle-29 finding (no self-introduced errors when convention
density is high) didn't apply to cycle 32 — the restructure was a
structural change to navigation, not an extension of existing
convention. The mild flags caught are about omissions (preamble
doesn't fully self-explain badges), not errors of commission.

### Honest-hedge tally: 11/11 stable

The cycle-30 honest-hedge applications (entropy/AI-slop "v1's
accretion-as-defense pattern (F12)"; throughput "conditions when the
security-stance pattern applies") propagated through cycle-31
verdicts and now into cycle-32 restructure. Both still honest at
cycle-32 fresh-eye reading. Tally: 11/11.

### Cycle-32 fresh-eye on cycle-31 borderline finding completed

The cycle-31 cold-reader flagged Lens 6.5 DISMISS rationale as
borderline-clarification-warranted. Cycle-32 fresh-eye CONFIRMED
the DISMISS and added the clarifying sentence to Family D's family
description. The pattern (cycle-N same-cycle cold-reader flags
borderline → cycle-N+1 fresh-eye converts borderline to action)
applies for the third time (cycle 28 → 29; cycle 29 → 30; cycle 31
→ 32). Stable.

## Same-cycle cold-reader on this notes file

Per cycle-19 same-cycle-cold-reader-on-rewrite pattern, ran the
cold-reader on the cycle-32 notes file before commit.

### Anti-smuggling discipline

Walked the prose for v2-prescription smuggling:

- **The seven decisions section:** describes how the cycle-32 in-
  place edits applied each decision. The decisions are about
  STRUCTURE OF THE OBSERVATIONS SECTION (family clustering, badges,
  callouts, cross-references), not v2 design direction. PASS.
- **The Lens 6.5 fresh-eye second-pass section:** the "strained vs
  strained" comparison is about WHICH STRUCTURE to use for Family D
  in the observations section, not about v2 architectural choices.
  PASS.
- **The persistence-mechanism observations section:** describes
  cycle-by-cycle workflow patterns, not v2 prescriptions. PASS.

### Cycle-18 transferability symmetry

Each cold-reader verdict carries explicit reasoning. PASS verdicts
have substance ("All 16 patterns have badges...", "verbatim from
the cycle-22 wording..."); BORDERLINE-PASS verdict carries flag
naming the sub-precision concern. Symmetry maintained.

### Self-introduced errors check

Walked the notes for self-introduced errors:

- **Decision-3 distribution table:** counts add to 16. Verified by
  re-counting the badges in the restructure. The "varying N=3,3,6,6,6,7"
  in the clean 3+ row maps to 6 patterns: Failed work (3), Multiple
  orchestration (3), Multi-agent (6), Anti-patterns (6), Code-vs-
  prompts (6), Small core (7). 6 patterns in clean 3+ ✓.
- **Verdict count walk for question (d):** 6 DISMISS verdicts named
  (2.5, 2.6, 2.7, 2.8, 5.3, 6.5). Cycle-31 notes had 6 DISMISS at
  the verdict tally. Verified ✓.
- **Honest-hedge tally:** 11/11. The increment from 10 to 11 needs
  justification. Cycle-30 was 10/10. Cycle-31's contribution: the
  family-D-as-thin-family verdict (the structural analogue of the
  diversity-hedge clause for individual patterns) — counts as one
  honest-hedge instance. So cycle-32 inherits 10/10 from cycle-30 +
  1 from cycle-31 = 11/11. ✓

No self-introduced errors caught.

### Section-transition smuggling check

- "Setup" → "The seven structural decisions, applied" → "Lens 6.5
  fresh-eye": clean. Each section flows from the prior context.
- "Lens 6.5" → "Same-cycle cold-reader on the in-place restructure":
  clean — fresh-eye precedes cold-reader on restructure (which can
  use the fresh-eye result).
- "Same-cycle cold-reader on the in-place restructure" → "Same-cycle
  cold-reader on cycle-31 notes file": clean — two cold-readers on
  different artifacts, named distinctly.
- "Same-cycle cold-reader on cycle-31 notes file" → "Persistence-
  mechanism observations" → "Same-cycle cold-reader on this notes
  file": clean — observations close the cold-reader sections, then
  meta-cold-reader on this file.

### Cold-reader verdict

PASS with one flag:
- The Family-D framing in the restructure now reads as PASSING the
  "absorb into A or B artificially" check AND the "dissolve D
  entirely" check. But the explicit thin-family framing in the
  family description is NEW prose this cycle (the strained-vs-
  strained sentence). Future cycles should watch whether the
  thin-family framing reads as principled epistemic transparency or
  as defensive justification — flagged for cycle-33+ fresh-eye if
  Phase 2 candidate-generation reading surfaces objections to the
  thin-family acknowledgment.

The cycle-32 notes file is ready for commit.

## Cycle-33+ pre-commits

Carry-forward from cycle 31, updated with cycle 32 progress:

1. **Update Persistent Divergences section** (carry-forward from
   cycle 27-31). Cycle-32 PARTIALLY RESOLVED via decision-4 hybrid
   (family-local callouts + final index). Remaining substantive
   prose work: Cognition Devin's anti-stance content; throughput-
   regime scope-condition content. The contrary-stance prose for
   small-fixed-team Cognition rejection is in the bullet body now;
   the throughput-vs-security scope condition is in the security
   bullet's inline note. Probably resolved enough to drop from
   pre-commits unless cycle 33+ fresh-eye finds additional gaps.

2. **Cross-validate against audit's A-pattern mapping** (carry-
   forward from cycle 25-31). Bounded mechanical.

3. **Read remaining audit retrospective sections** (carry-forward
   from cycle 25-31). "What v2 must demonstrably do better" section
   is the most relevant for Phase 2.

4. **Copilot research-only dispatch: oh-my-claudecode** (Eva
   directive #2774). Deferred from cycles 26-32. Cycle-33+ if
   budget permits. With commit-then-dispatch sequencing now in the
   persistence-mechanism log, the dispatch must follow that
   discipline.

5. **Copilot research-only dispatch: openai/symphony** (Eva
   directive #2775). Same gating as item 4.

6. **Codify SUPPORT/CONTRADICT gradient definition** (carry-forward
   from cycle 28-31). Bounded mechanical; defer until matrix shape
   is re-used.

7. **Codify third-category refinement to discipline-lightening rule**
   (carry-forward from cycle 28-31). Defer until trigger event
   occurs.

8. **Same-cycle cold-reader on this notes file (cycle-32).** Done
   above. PASS with one flag (thin-family framing future-watch).

9. **Cycle-33+ fresh-eye on cycle-32 borderline finding.** The
   section preamble's two mild flags (prefix→tier mapping not
   explicit; A-B tie-break not stated) and the thin-family framing
   future-watch are flagged for fresh-eye. If they don't materialize
   as actual reading-friction at cycle-33+, drop them.

10. **Long-deferred items roll-call** (carry-forward, 9 items
    unchanged cycles 26-31; carry into cycle 33+).

### Suggested cycle 33 plan (provisional)

- **Focal candidate (substantive):** item 2 (audit A-pattern
  cross-validation) OR item 3 (audit retrospective remaining
  sections). Both are bounded substantive work suitable as a focal.
- **Focal candidate (load-bearing):** Phase 2 candidate generation
  preparation — re-read the restructured cross-system observations
  section + audit's A-pattern mapping + single-system observations
  in `_notes/cycle-22-cross-system-synthesis.md` to set up Phase 2
  candidate generation. This would be architecturally-load-bearing
  for the next phase transition.
- **Bounded mechanical:** item 9 (fresh-eye on cycle-32 borderline
  flags) + cold-reader on cycle-33 notes file.
- **Possible Copilot dispatch:** items 4/5 (oh-my-claudecode and/or
  openai/symphony) IF cycle-33's focal completes early. Otherwise
  cycle 34+.

The redesign's Phase 1 deliverable shape is now stable post-
restructure. Phase 2 candidate generation is the next architectural
transition (per Phase 2 in the system prompt). Cycle 33+ should
consider whether to begin Phase 2 prep work or continue the Phase 1
audit-cross-validation backlog first.
