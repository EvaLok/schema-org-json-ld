# Cycle 112 — Selection central-bet stress-test (family-level vs pattern-level dominance)

**Cycle issue:** [#2901](https://github.com/EvaLok/schema-org-json-ld/issues/2901)
**Date:** 2026-05-10
**Mode:** redesign Phase 2 candidate iteration, **twenty-fourth cycle of Phase 2 candidate-set work** (cycles 90-112), under [`ITERATION-UNTIL-APPROVAL`](../../../.github/workflows/orchestrator-redesign-prompt.xml).

## Setup and the cycle 111 → 112 forward-going-commitment trajectory

Cycle 110 (2026-05-10) closed C Risk 5 and named cycle 111+ rotation commitment to A-side or B-side risk closure. Cycle 111 (2026-05-10) departed from that commitment to author [`docs/redesign/2-selection.md`](../2-selection.md) — the Phase 2 selection deliverable that cycles 92-110 had not produced. Cycle 111 named `discretionary-departure-from-forward-going-commitment` as NOVEL@1 candidate-pattern and named cycle 112+ work including rotation re-attempt.

**Cycle 112 also departed from rotation** — choosing central-bet stress-testing of the cycle 111 selection draft over rotation. This is the **second consecutive cycle of departure**. The forward-going-commitment-honored discipline (cycle 110 NOVEL@1 across 4-cycle chain 107→108, 108→109, 109→110, 110→111-forward-named) is now broken at the cycle 111 link AND the cycle 112 link. **The discipline is REFUTED at strict-discipline level**; it survives only as "honor when no higher-priority work surfaces" — closer to no-discipline than to a discipline.

The cycle 112 departure is reasoned: the cycle 111 selection draft contained a load-bearing flaw in its central-bet rationale that needed surfacing before the candidate-selection checkpoint can responsibly proceed. Risk closure on A-side or B-side would have been valuable but lower-priority than fixing the selection draft's mis-grounded central-bet claim.

## The central-bet stress-test finding

The cycle 111 selection draft asserts (Criterion 4 paragraph 2 + Criterion 5 paragraph A):

> "The dominant failure pattern is **F1 (constraint accretion) compounding F7 (self-management dominance)** — exactly what A's Axis 13 medium-harness extraction is designed to address most directly."

And uses this to ground A's central-bet defensibility as:

> "this bet is **directly supported** by the retrospective's identification of F1+F7 as the dominant failure pattern."

**The retrospective does not identify F1+F7 as the dominant failure pattern.** The retrospective at lines 161-162 explicitly names *reconciliation asymmetry* as the dominant family:

> "The cleanest single-sentence statement of the dominant family (reconciliation asymmetry) is: *v1 has tools to **create** records and tools to **summarize** records; it lacks tools to **reconcile** records against later events.*"

Line 959 repeats: "reconciliation asymmetry as dominant family."

**Reconciliation asymmetry is the F2/F3/F4/F5†/F11† family** (per the table at retrospective line 138-143). **F1 and F7 are in different families:**
- F1 = defense-accretion family (F1/F5†/F11†/F12)
- F7 = procedure/review-overhead family (F6/F7/F9)

The cycle 111 framing "F1 compounding F7" crosses two family boundaries (defense-accretion + procedure-overhead) to construct a cross-family "dominant pattern" that the retrospective does not name. The framing fits A's Axis 13 medium-harness extraction targeting (which addresses F1 + F7 directly), and was therefore selected for that reason — a textbook motivated-reasoning pattern that `ITERATION-UNTIL-APPROVAL`'s "examine for self-congratulation" / "stress-test claims" activities are designed to surface.

## Two valid readings of "dominance"

The retrospective supports two readings, at different abstraction levels:

| Reading | Source | Claim | Selection ordering implication on Criterion 4 |
|---|---|---|---|
| **Family-level** | Retrospective lines 161-162, 959 (explicit) | Reconciliation asymmetry is the dominant family | C > B > A (C addresses F2/F4/F11 with equivalent outcomes to A but better legibility; B addresses with most depth but inverts CORE-DESIGN-PRINCIPLE) |
| **Pattern-level prevalence** | Cycle 2 measurements (`docs/redesign/_notes/cycle-2-measurements.md`) | F1 (pervasive) and F7 (130+ consecutive cycles of zero schema work) are the most-pervasive and most-measurable individual patterns | B > C > A (B addresses F1+F7 most aggressively via role decomposition; A and C have similar F1+F7 addressing) |

The cycle 111 selection draft picked the pattern-level reading without acknowledging the family-level reading exists in the retrospective. The cycle 112 update surfaces this and revises Criterion 4 accordingly.

## Why this matters less than it first appears: equivalent reconciliation outcomes

Reading [`A-evolved-single-orchestrator.md`](../2-candidates/A-evolved-single-orchestrator.md) lines 36-37 + 45 and [`C-hybrid.md`](../2-candidates/C-hybrid.md) lines 22 + 44-46 confirms:

- Both use cursor-file polling (`state/eva-input-cursor.json`)
- Both have 1-cycle latency for Eva-response detection (~6h at 4 cycles/day cron)
- Both detect post-close mutations via cycle-history append-only files (Axis 4 git-as-substrate)
- Both treat audit-as-peer with cross-repo reading discipline

**The detection mechanism and detection latency are equivalent between A and C.** The structural difference is **legibility**:
- A's reconciliation is interleaved with boot-phase polling (one of several boot activities; not a named harness phase)
- C's reconciliation is `reconcile-mode` — a named harness phase between boot-mode and work-mode, with bounded runtime budget (≤10min), per-channel structured handlers, typed-deltas to per-component state files

A reader of an A-system inferring reconciliation state has to read boot-phase logs; a reader of a C-system reads the named reconcile-mode artifact directly. The detection works identically; the legibility differs.

This is the load-bearing observation the cycle 111 selection draft glossed over. **The A vs C question is not "outcomes vs outcomes"; it is "interleaved-with-boot reconciliation legibility vs first-class named-phase reconciliation legibility, both producing the same outcomes."**

## The reframed selection question

Cycle 112 reframes the A-vs-C question from cycle 111's:

> Is A's parsimony enough for v1's dominant failure pattern (F1+F7)?

To:

> Reconciliation asymmetry IS the family-level dominant failure pattern. A's interleaved reconciliation and C's first-class reconcile-mode produce equivalent outcomes (same latency, same mechanism). **Is named-phase reconciliation legibility worth +~1000-2600 LOC migration cost over A's interleaved approach?**

This reframing makes the trade-off honest:
- A's win on Criterion 3 (cost of being wrong) is real and quantifiable: A's net-add ~3600-6200 LOC; C's ~5200-8200 LOC; delta ~1000-2600 LOC
- C's win on family-level-dominance addressing is real but its load-bearingness depends on whether legibility-to-Eva-and-audit is judged structurally important

This is exactly the kind of value judgment Eva is positioned to evaluate at the candidate-selection checkpoint.

## Why the recommendation tentatively stays as A

The recommendation does not flip post-cycle-112 because:

1. **Criteria 1 (schema-work-enabling) and 3 (cost of being wrong) still favor A** — these are the highest-weighted criteria per the redesign's primary thesis, and the cycle 112 finding does not change them
2. **A and C have equivalent reconciliation OUTCOMES** — A is not actually worse on reconciliation outcomes; A is worse on reconciliation legibility
3. **A's central-bet rationale weakens but A's overall criteria-weighted ranking does not flip** — Criterion 5 ordering shifts from A > C to A ≈ C; Criteria 1+2+3 still favor A; Criterion 4 ordering becomes contested between two valid readings; Criterion 6 narrowly favors B at unchanged margin
4. **The cost-benefit framing is now Eva's call** — Eva resolves whether named-phase reconciliation legibility is worth +~1000-2600 LOC

## Edits made cycle 112

1. **[`docs/redesign/2-selection.md`](../2-selection.md)** — substantial update:
   - Status header: cycle 111 + cycle 112 attribution
   - Authoring cycle line: cycle 112 stress-test acknowledged
   - Criterion 4 paragraph: rewritten to acknowledge two valid dominance readings, cite retrospective lines 161-162 + 959 explicitly, note F1 and F7 are in different families, name the cycle 112 finding, ordering becomes contested ("C > B > A under family-level reading OR B > C > A under pattern-level reading")
   - Criterion 5 paragraph: rewritten — A's central bet downgraded from "directly supported" to "consistent with one valid reading"; C's central bet upgraded to "consistent with family-level dominance reading"; ordering downgraded from "A > B ≈ C" to "A ≈ C > B"; A vs C becomes "contested rather than A-dominant"
   - Side-by-side comparison table "Central bet defensibility" row: A "contested post-cycle-112" / C "upgraded post-cycle-112 to consistent-with-family-level-dominance reading" / B "no retrospective grounding at any reading"
   - "Why not C" section: added "Post-cycle-112 stress-test update" paragraph reframing A-vs-C trade-off as cost-benefit value judgment; Reason #3 weakened (F4/F11 addressing improvements over A no longer "unproven as load-bearing" under family-level reading)
   - "What A gives up" item #5: added post-cycle-112 update; legibility-on-F4/F11 is now legibility-on-dominant-family-addressing; mitigation expanded with Phase 3 measurement of reconciliation-related boot-phase work
   - **NEW SECTION** "Central uncertainty: family-level vs pattern-level dominance (cycle 112 stress-test)" between "Why not C" and "Outstanding work before lock-in" — ~115 lines covering the finding, the two readings table, why-this-matters-less-than-it-first-appears, the reframed selection question, why-recommendation-stays-as-A, what-this-means-for-cycle-112+-outstanding-work
   - Outstanding work item #3 ("Stress-test the central bet"): marked partially absorbed cycle 112; remaining cycle-113+ work named (other load-bearing-claim stress-tests; Phase-3-equivalent-outcomes claim verification)
   - Open questions for Eva: added question #6 (resolve family-level vs pattern-level dominance question; named the cost-benefit value judgment)
   - Iteration log: added cycle 112 row

2. **[`docs/redesign/2-candidates/README.md`](../2-candidates/README.md)** — status paragraph extended with cycle 112 stress-test summary

3. **[`docs/redesign/_notes/cycle-112-dispatch-1-body.md`](./cycle-112-dispatch-1-body.md)** — adversarial Copilot feedback dispatch body authored. **Dispatch attempted but blocked by runtime permission**; deferred to cycle 113+. Body covers two adversarial lenses in parallel: (Lens 1) argue strongly for flipping recommendation to C; (Lens 2) stress-test the cycle 112 reasoning itself for errors.

## Verification (compressed)

| Item | Pre-cycle-112 | Post-cycle-112 |
|---|---|---|
| `docs/redesign/2-selection.md` line count | 206 | ~340 (+134 / +65%) |
| Sections in 2-selection.md | 12 (top-level headings) | 13 (new "Central uncertainty: family-level vs pattern-level dominance") |
| Open questions for Eva | 5 | 6 (+ family-vs-pattern dominance) |
| Side-by-side comparison table "Central bet defensibility" row content | A "high" / C "medium" / B "medium" | A "contested post-cycle-112" / C "upgraded post-cycle-112" / B "no retrospective grounding at any reading" |
| Criterion 4 ordering verdict | "B > C > A with caveats" | "C > B > A under family-level reading OR B > C > A under pattern-level reading — central uncertainty" |
| Criterion 5 ordering verdict | "A > B ≈ C" | "A ≈ C > B — A vs C contested rather than A-dominant" |
| Recommendation | A (tentative) | A (tentative; central-bet rationale contested rather than confidently defended) |
| Adversarial dispatch on selection draft | not yet attempted | **attempted; blocked by runtime; body file in `_notes/cycle-112-dispatch-1-body.md`; deferred cycle 113+** |
| Forward-going-commitment-honored discipline | NOVEL@1 (tentatively-with-cycle-111-departure) | **REFUTED-at-strict-discipline-level** (cycle 111 AND cycle 112 both departed from rotation; discipline survives only as "honor when no higher-priority work surfaces") |
| `discretionary-departure-from-forward-going-commitment` | NOVEL@1 (cycle 111) | **TESTED@2** (cycle 111 + cycle 112 both depart — substrate-content-distinct: cycle 111 was selection-draft-authoring; cycle 112 is central-bet-stress-test) |
| Functional-class shape instances (formal tracking) | 24 / 54 | 24 / 54 (no formal tracking promotions cycle 112 — observance only; aligns with cycle 111's observation that shape-tracking is not load-bearing for non-closure work) |

## What surprised me / what I noticed

1. **The cycle 111 framing "F1+F7 dominant" was a textbook motivated-reasoning pattern, and it would have been easy to miss.** The cycle 111 selection draft's Criterion 4 paragraph reads naturally on first read — it cites the retrospective, names specific patterns, draws a defensible conclusion. The flaw is only visible on the second reading, where you ask "did the retrospective actually identify F1+F7 as dominant, or did the draft assert that?" The retrospective explicitly says "reconciliation asymmetry is the dominant family" at lines 161-162; the cycle 111 draft framing is at best a misreading of the retrospective and at worst motivated reasoning shaped by what supports A. **Implication for v2 design:** the design must include mechanisms for cross-cycle adversarial review of load-bearing rationales, not just for individual claims. The cycle 111 draft would have benefited from a mechanism that specifically asks "does the retrospective actually claim what this draft claims it claims?" — which is precisely what audit-as-peer is supposed to do but at slower cadence than within-session.

2. **The cycle 112 finding does not flip the recommendation, but it does change the rationale's character.** Pre-cycle-112: the recommendation was "A is best because it directly addresses the dominant pattern (F1+F7)." Post-cycle-112: the recommendation is "A wins on cost-of-being-wrong and equivalent-reconciliation-outcomes; whether named-phase legibility is worth the migration cost is a value judgment Eva resolves." The post-cycle-112 framing is more honest: it surfaces the trade-off Eva needs to make rather than asserting A is dominant on the central-bet criterion. **The cycle 111 draft's confident "directly supported" framing prevented Eva from making the trade-off explicitly; the cycle 112 reframing makes the trade-off the central question.**

3. **A and C having equivalent reconciliation outcomes is the load-bearing observation that prevents the recommendation flipping.** If A and C had different reconciliation latencies (e.g., A every cycle, C every 30 minutes via continuous polling), the family-level dominance finding would more clearly favor C. But because both A and C have 1-cycle latency with cursor-file polling, the family-level dominance finding doesn't actually translate into a meaningful outcome difference — only a legibility difference. **This is the kind of detail that's easy to miss in higher-level criterion comparison; it required reading both candidate documents at the F2/F11 addressing level to surface it.**

4. **The dispatch was blocked by runtime permissions despite being authorized by the redesign prompt.** Per `COPILOT-DISPATCHES`, feedback-only dispatches are explicitly authorized. Per `COPILOT-DISPATCH-METHOD`, the syntax is documented. The body file is written. The dispatch should have proceeded. The runtime nonetheless blocked the command requiring approval. **This is a friction point worth observing for v2:** the authorization-in-prompt does not match the runtime's permission model. Either the runtime needs to read the prompt's authorization, or the prompt needs to declare the runtime-permission delta as a known operational constraint. The cycle 112 work proceeded without the dispatch; cycle 113+ retries.

5. **Cycle 112 is the second consecutive cycle of forward-going-commitment departure.** Cycle 110 named cycle 111+ rotation commitment; cycle 111 departed (selection-draft authoring); cycle 112 departed again (central-bet stress-test). The `forward-going-commitment-honored` discipline (cycle 110 NOVEL@1 across 4-cycle chain) is now refuted at strict-discipline level. The `discretionary-departure-from-forward-going-commitment` candidate-pattern is at TESTED@2. **The departure pattern itself is becoming the rotation pattern.** Cycle 113+ may need to either honor rotation (refuting the departure pattern) or articulate why departures are continuing (formalizing the soft-discipline framing).

6. **The cycle 111 _notes flagged "the cycle 103-110 pattern as potentially self-amplifying meta-tracking" but did not flag the cycle 111 selection draft as potentially containing motivated reasoning.** The cycle 111 _notes' meta-observation was honest about the cycles 103-110 risk-closure pattern, but did not turn the same scrutiny on the cycle 111 selection draft itself. The motivated-reasoning pattern in the F1+F7 framing was therefore the cycle 112 stress-test's first-real surface. **Implication:** ITERATION-UNTIL-APPROVAL's "examine for self-congratulation" activity must be applied not just to meta-tracking but to the substantive draft's claims themselves. Cycle 113+ should continue this pattern — examining other load-bearing claims in the selection draft (B's cluster G aspirational extension, C's conditional-improvement risk, the equivalent-reconciliation-outcomes claim).

7. **The cycle 112 work is structurally distinct from cycles 78-111 bottleneck-asynchronous pattern.** Cycle 111 was bottleneck-asynchronous-but-structurally-distinct (selection-draft authoring vs per-candidate sharpening). Cycle 112 extends the structural distinction further: it surfaces a flaw in cycle 111's substantive output. **Cycle 112 is the second consecutive cycle whose substantive focal addresses cycle 111's substantive output rather than per-candidate sharpening.** The cycle 78-110 pattern (per-candidate sharpening with shape-tracking) is increasingly diverging from the current cycle pattern. Cycle 113+ may need to either (a) return to per-candidate sharpening if no further selection-draft flaws surface, or (b) continue selection-draft work if more flaws or audit critique surfaces.

## Sibling pattern tracking (informal — formal tracking remains paused per cycle 111)

| Pattern | Pre-cycle-112 status | Post-cycle-112 observation |
|---|---|---|
| `discretionary-departure-from-forward-going-commitment` (cycle 111 named) | NOVEL@1 (cycle 111 first instance: selection-draft authoring) | **TESTED@2** (cycle 112 second instance: central-bet stress-test; substrate-content-distinct) |
| `forward-going-commitment-honored` (cycle 110 named) | NOVEL@1 (4-cycle chain 107→108, 108→109, 109→110, 110→111-forward-named) | **REFUTED-at-strict-discipline-level** (cycle 111 + cycle 112 both broke the chain at the cycle-111-rotation link); discipline survives only as soft "honor when no higher-priority work surfaces" |
| Risk-closure-at-specification-level (shape #21) | HARDENED-at-7 (cycle 110) | unchanged (cycle 112 not a risk closure) |
| Three-layer-closure-verification (shape #25 candidate) | TESTED@2 (cycle 110) | unchanged |
| Risk-closure-with-warm-up-window (shape #24) | HARDENED@5 (cycle 110) | unchanged |
| Discipline-conditional risk-shape type | HARDENED@3 (cycle 110) | unchanged |
| `central-bet-stress-test-finds-mis-grounding` (cycle 112 NEW candidate-pattern) | not named | **NOVEL@1** (cycle 112 first instance: cycle 111 F1+F7 framing mis-grounded against retrospective family-level dominance reading); applicability test is whether cycle 113+ stress-tests of other load-bearing claims surface analogous mis-groundings |

## Lexicon entries

- **family-level vs pattern-level dominance:** the retrospective supports two readings of "what is dominant" — at the family level (reconciliation asymmetry per lines 161-162) or at the individual-pattern level (F1, F7 by prevalence). The cycle 111 selection draft picked the pattern-level reading; cycle 112 surfaces the family-level reading
- **equivalent reconciliation outcomes:** A and C have the same reconciliation latency (1 cycle) and the same detection mechanism (cursor-file polling). The structural difference between A and C on reconciliation is legibility (named-phase vs interleaved with boot), not outcomes
- **reconciliation legibility:** how easy it is for Eva or audit-as-peer to read the reconciliation state of an A-system vs a C-system. C's named `reconcile-mode` artifact is more legible; A's interleaved-with-boot reconciliation requires reading boot-phase logs to infer state
- **named-phase legibility worth +~1000-2600 LOC?:** the reframed A-vs-C question post-cycle-112; a value judgment Eva is positioned to make
- **central-bet stress-test:** ITERATION-UNTIL-APPROVAL activity; pick a load-bearing claim in the artifact and try to disprove it against the retrospective, the candidate documents, or external evidence
- **mis-grounded against retrospective:** a draft asserts the retrospective claims X but the retrospective actually claims Y, where X supports a particular candidate / framing and Y does not
- **motivated-reasoning surface:** the draft selects from multiple valid framings the one that supports the conclusion the author already favors; cycle 111 selected pattern-level dominance over family-level dominance because pattern-level supports A
- **the discipline is breaking, but the pattern is forming:** when a forward-going commitment is broken across multiple cycles for legitimate reasons, the discipline is REFUTED but a candidate-pattern of "discretionary departure" is forming. Cycle 112 instantiates this dynamic
- **runtime-permission delta vs prompt-authorization:** an action is authorized by the redesign prompt but blocked by the runtime's permission model. Friction point for v2 design (either the runtime needs to read prompt-authorization, or the prompt needs to declare the delta)

## Bottleneck-state honesty

Bottleneck remains external: no audit critique landings since #454; no Copilot dispatch returns; no Eva input arrivals since the standing directives (and now: cycle 112's adversarial dispatch attempt was blocked by runtime, so there will be no cycle-112-originated dispatch return either). Cycle 112 contribution is fully repo-internal but **structurally distinct from cycles 78-111 bottleneck-asynchronous pattern AND from cycle 111's selection-draft-authoring shape**. Cycle 112's substantive focal — central-bet stress-test of cycle 111's substantive output — represents the second consecutive cycle whose work addresses cycle 111's substantive output rather than per-candidate sharpening. The 8-cycle bottleneck-asynchronous-and-per-candidate-sharpening run (cycles 103-110) is broken by 2 cycles now (111, 112). **Cycle 112 is the twenty-fourth consecutive cycle (cycles 78-112) whose output is fully repo-internal; it is the second cycle since cycle 102 not focused on per-candidate sharpening (cycle 111 was first; cycle 112 second).**

Audit cycle 215 (2026-05-10 04:21-04:41 UTC) closed without substantive Phase 2 critique landing; cycle 215 named cycle 216 watch item (e) as substantive Phase 2 candidate critique on one risk closure. Audit cycle 216 expected at next audit cron (~04:00 UTC 2026-05-11, ~20h from cycle 112's start). Cycle 113+ is the most likely landing window for audit-side Phase 2 critique; cycle 112 cannot wait for it.

## Forward work for cycle 113+

1. **Audit cycle 216 substantive Phase 2 critique landing absorption** (~ next audit cron, ~20h from cycle 112)
2. **Adversarial Copilot feedback dispatch retry** — body file at [`cycle-112-dispatch-1-body.md`](./cycle-112-dispatch-1-body.md) is ready; cycle 113+ retries the dispatch when runtime permits
3. **Continue the central-bet stress-test pattern on other load-bearing claims:**
   - "B's central bet requires aspirational extension" — does B's cluster G framing have any retrospective grounding cycle 112 missed?
   - "C's conditional-improvement risk" — is C's "if additions don't carry their weight, C is just A with more migration surface" framing grounded or motivated?
   - "A and C have equivalent reconciliation outcomes" — does Phase 3 measurement under realistic Eva-response-burst load actually support equivalence, or expose differences?
   - "Migration cost is 3-5× A's" for B — is the LOC estimate methodology defensible against PR #2877's lens-5 critique?
4. **Cycle 110 _notes-named rotation commitment** — pending cycle 113+ honoring (now two cycles late). If cycle 113 also departs, the rotation discipline is conclusively REFUTED
5. **Cycle 103-110 risk-closure threshold table integration into 2-selection.md** (cycle 113+) — structured Phase 3 measurement plan
6. **Per-candidate Phase 3 prototype evidence deepening** (cycle 113+) — extending cycles 93-94 measurements to A's `boot-phase` + `wiki-search`; equivalent for C's `reconcile-mode` if Eva pre-approves
7. **Symphony deeper-read elevation** (cycle 113+ or based on cluster catalogue findings) — first-pass at cycle 98
8. **oh-my-claudecode deeper-read elevation** (cycle 113+) — first-pass at cycle 99
9. **Eva-authored dispatch-test issues #2879 + #2881** — left open per HOUSEKEEPING discipline
