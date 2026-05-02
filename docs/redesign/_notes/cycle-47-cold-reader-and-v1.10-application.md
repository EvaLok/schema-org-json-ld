# Cycle 47 — cold-reader on v1.9 (1 BORDERLINE-FAIL + 1 PASS) + v1.10 application + #809 closure

**Date:** 2026-05-02 (third cycle of the day)
**Cycle issue:** #2814
**Inherits from:** cycle 46 (`_notes/cycle-46-cold-reader-pass-and-bounded-mechanical.md`)

## Cold-reader: 1 PASS + 1 BORDERLINE-FAIL + 1 procedural

Three questions inherited from cycle 46's pre-commit checklist. Each
re-walked with adversarial framing.

### Q(a) PASS — v1.9 F8 rationale re-walk with fresh framing

**Question:** Re-walk the v1.9 F8 rationale (line 636) with fresh
adversarial framing. Asymptotic convergence hypothesis predicts second
consecutive PASS-without-escalation. If escalation surfaces, adjust the
convergence model.

**Re-walk of v1.9 F8 rationale** (line 636, post-cycle-46):

> Bounded loops (loop-count ceiling positions; prevention),
> stuck-session watchdog (runtime-ceiling positions; detection-and-
> recovery; openclaw's `diagnostics.stuckSessionWarnMs` instance —
> detect stale runs and release lanes), or both compositionally
> (Axis 9's `Both (loop + runtime)` position) + single-implementation
> discipline (no parallel implementations)

**Fresh-framing probes:**

(i) **The "+ single-implementation discipline" composition.** Cycle 46's
walk did not specifically examine the "+" join attaching SID to the
three Axis 9 strategies. Probing fresh:

- The "+" reads as conjunction ("AND ALSO"). Any of {bounded, watchdog,
  both} + SID required.
- Structurally fine; the "+" is not ambiguous between exclusive and
  conjunctive readings (other rationales also use "+" for conjunction:
  F3 uses "+" between Axis 2 and Axis 12 strategies; F7 uses "+" between
  four strategies).

(ii) **Origin of "single-implementation discipline".** SID isn't an
Axis 9 position. The "Most-relevant axes" column for F8 is "Axis 9,
CORE-DESIGN-PRINCIPLE". So SID is sourced from CDP. The retrospective
0-retrospective.md confirms (lines 530, 982-985, 1088-1090): "Avoid
parallel implementations of the same job. When the cycle-524 cascade
turns one bug into multiple cycles of abandonment."

So F8 has TWO sub-mechanisms:
- abandonment-cascade prevention (Axis 9 strategies)
- parallel-implementation gap prevention (SID, from CDP)

Both are captured in the rationale.

(iii) **Why F8 cites CDP explicitly while F1/F6/F7 cite Axis 13.**
Looking at Observation 3:

> CORE-DESIGN-PRINCIPLE (tools handle rote; orchestrator handles
> judgment) shows up across F1, F6, F7, F8 — it is itself an axis-
> cross-cutting constraint. Axis 13 makes the specific candidate-
> differentiation choice along the CDP direction explicit; CDP itself
> remains the directional statement every candidate must demonstrate.

F1/F6/F7's CDP-aspect is fat-harness (Axis 13's choice). F8's CDP-aspect
is single-implementation discipline, which is **orthogonal to Axis 13**
(both fat-harness and thin-harness candidates can violate or honor SID).
So F8 correctly cites CDP rather than Axis 13. Content-driven asymmetry.

(iv) **Scope of "no parallel implementations".** The rationale says
"no parallel implementations" without explicit "tool" qualifier. A hasty
reader might interpret this too broadly. Counter-check: F8's family
column says "Tooling fragility"; CDP citation is in axes column; reader
context supplies the scope. Borderline; not a load-bearing precision
gap.

**Verdict: PASS** — three fresh probes (composition, SID origin, CDP-vs-
Axis 13 distinction) all defensible. Probe (iv) borderline scope-implicit
but covered by family + CDP context. F8 rationale is content-stable on
fresh framing. Asymptotic convergence hypothesis SUPPORTED for F8 area.

### Q(b) BORDERLINE-FAIL — cross-axis dep map sweep finds Axis 13 Maps-to gap

**Question:** Walk each entry in the cross-axis dep map (lines 556-611)
with the question "does this entry's wording hide a precision gap?"
Specifically check the "indirect contribution" annotations.

**Entry-by-entry sweep:**

| Entry | Verdict | Notes |
|---|---|---|
| Axis 1 × Axis 7 | PASS | "Forces" / "enables but doesn't force" — strength-of-dependency taxonomy is precise. Multi-agent peer position is REJECTED in Axis 1 table, so not enumerated here. |
| Axis 2 × Axis 3 | PASS | Closing caveat "supportive rather than exclusive" applies to all three Axis 2 positions (general framing, not just file-per-component). Cycle-44 / cycle-45 / cycle-46 verified. |
| Axis 4 × Axis 2 | PASS | F11 indirect-contribution annotation block matches F11 rationale's "Axis 2 indirect contributor" annotation (cross-check confirmed). |
| Axis 8 × Axis 5 × Axis 10 | PASS | "Adopting Axis 8 unlocks the others" is shorthand for "unlocks the mechanical-enforcement features of the others" — body specifies scope. |
| Axis 12 × Axis 4 | PASS | Single-position-pair claim ("event-driven pairs naturally with git-as-substrate") doesn't preclude other pairings. |
| Axis 12 × Axis 1 | PASS | Two viable Axis 1 positions covered (small-fixed-team, single-threaded); third (multi-agent peer) is REJECTED. |
| Axis 13 × Axis 6 | PASS | "Fat-harness needs a richer extension story" — comparative, not exclusive. |
| Axis 13 × Axis 8 | PASS | "Implies more X" — comparative entailment, structurally sound. |
| **Axis 13 × Axis 7** | **PASS in dep map; BORDERLINE-FAIL upstream** | Dep map entry documents Axis 13's role in F9 ("Axis 13 shapes the implementation strategy"). But Axis 13's **Maps-to** does NOT mention F9, asymmetric with Axis 1's pattern. Cycle-47 escalation (see below). |
| Constraint 8 × Axis 1 | PASS | Two positions covered; third REJECTED. |

**The Axis 13 Maps-to gap (BORDERLINE-FAIL):**

Sweeping for indirect-contribution annotation patterns across the
framework:

- **Axis 1 Maps-to** (line 156): `"Maps to: F7 ... Indirect contributor
  to F9 (adversarial-review treadmill) via dedicated-reviewer-role."`
- **Axis 2 Maps-to** (line 188): `"Maps to: F12, F5, F3. Indirect
  contributor to F11 (post-close mutations) — file-per-component
  naturally supports per-component append..."`
- **Axis 13 Maps-to** (line 544 pre-v1.10): `"Maps to: F1, F6, F7,
  CORE-DESIGN-PRINCIPLE explicitly."` — **NO indirect contributor
  annotation despite Axis 13 × Axis 7 dep map documenting Axis 13's
  role in F9.**

**Cross-history check on whether the gap was deliberate:**

Cycle-39 cold-reader (`_notes/cycle-39-cold-reader-and-redispatch-
escalation.md` lines 109-152) explicitly verified F-pattern table levels
and concluded:

> Verdict: PASS (no missing F-pattern→axis mappings; F9 → Axis 7 with
> Axis 13 indirect via cross-axis deps is the right level).
>
> ...
>
> Maps-to backfill verification (cycle-38 v1.3): all 5 backfilled
> Maps-to lines (Axes 1, 3, 5, 6, 7) are consistent with the F-pattern
> table.

So cycle-39 verified F-pattern table level (F9 column should be Axis 7
only) AND verified the 5 backfilled Maps-to lines (Axes 1, 3, 5, 6, 7).
Axis 13's Maps-to was **NOT** in the backfill set — Axis 13 was added at
v1.2 with its Maps-to inline, and cycle-39 didn't re-review it.

So Axis 13's missing F9 indirect annotation is an **unintentional gap**,
not a deliberate decision. The cycle-47 escalation is justified and
limited:

- Add F9 indirect annotation to Axis 13's Maps-to (matching Axis 1 /
  Axis 2 pattern of annotating indirect contributions in Maps-to)
- Do **NOT** modify F9's rationale or column (preserves cycle-39's
  explicit verdict that "F-pattern table levels are correct")

**v1.10 change applied:**

Axis 13 Maps-to extended from:

> Maps to: F1 (constraint accretion in prompt — fat harness extracts
> procedural constraints), F6 (cyclomatic procedure depth — fat harness
> extracts procedure), F7 (self-management dominance via prompt-encoded
> procedure), CORE-DESIGN-PRINCIPLE explicitly.

To:

> Maps to: F1 (constraint accretion in prompt — fat harness extracts
> procedural constraints), F6 (cyclomatic procedure depth — fat harness
> extracts procedure), F7 (self-management dominance via prompt-encoded
> procedure), CORE-DESIGN-PRINCIPLE explicitly. Indirect contributor to
> F9 (adversarial-review treadmill) — fat-harness shapes the
> implementation strategy for Axis 7's situational-review by controlling
> when review fires; thin/medium harness leaves WHEN-review decisions
> in prompt where the every-cycle-review pattern tends to recur; the
> load-bearing F9 fix is Axis 7.

**Verdict: BORDERLINE-FAIL.** Single-cell wording addition (parallel
in magnitude to cycle-45's BORDERLINE-FAIL fix). Bounded-mechanical
escalation; preserves cycle-39's F-pattern table verdict.

### Q(c) Bounded-mechanical decision: #809 closure

Cycle-46's Q(c) options for cycle 47:
1. Start cross-axis-impact-check scaffold (Path A T+0)
2. Sketch Phase 2 candidate against v1.9 framework
3. #809 closure consideration
4. Iterate redispatch tool design draft

**Analysis:**

- (1) and (2) are gated on post-retrospective checkpoint approval per
  cycle-46 reasoning (defer build-start; defer candidate sketch). No
  change in checkpoint state cycle 47.
- (3) #809 closure: borderline at cycle 46; cycle-47 reconsideration
  finds strong absorption signal.
- (4) redispatch tool design: no existing draft (`tools/v2/_drafts/`
  contains only `cross-axis-impact-check-design.md`). Creating from
  scratch exceeds cycle-47 bounded-mechanical capacity given v1.10
  application + same-cycle review.

**#809 reconsidered (cycle-46 deferred → cycle-47 close):**

The directive "Iterate on Copilot PRs — stop merging with known issues"
applies to **implementation Copilot PRs**. In redesign mode:

- Authorized dispatch types are research-only, feedback-only,
  implementation (per redesign prompt SECTION 2 COPILOT-DISPATCHES)
- research-only and feedback-only PRs don't have a "merge with issues"
  failure mode — they're closed when per-finding evaluation has
  integrated content
- implementation dispatches haven't occurred in redesign mode and
  wouldn't until post-cutover

Per-finding evaluation discipline (cycles 7, 12, 31, 41, 43) is **strictly
stronger** than #809's "iterate before merging" framing — each finding
is individually accepted/qualified/rejected, not just "iterate the PR."

**Closure decision:** strong absorption signal. Closed with linking
comment naming the per-finding evaluation cycles and the redesign-mode
dispatch lifecycle.

Open `input-from-eva` count: 7 → 6 (cycle 47 closes 1).

## Same-cycle review (5 questions)

### Q1 — v1.10 application defensibility

Is the v1.10 single-cell change defensible? Did I escalate correctly,
or was this fabricating an improvement?

**Re-walk:**

The escalation criteria from cycle-46 were:
- Did I find a real precision gap? **YES** — Axis 13's Maps-to
  asymmetric with Axis 1's pattern.
- Is the gap load-bearing (affects candidate-author understanding)?
  **YES** — an author entering Axis 13's section misses F9 indirect
  contribution; an author entering Axis 1's section sees the equivalent.
- Was the gap evaluated against history (cycle-39 verdict)? **YES** —
  cycle-39 explicitly verified F-pattern table level but did NOT
  re-review Axis 13's Maps-to (Axis 13 was added at v1.2; backfill at
  v1.3 covered Axes 1/3/5/6/7).
- Is the fix bounded-mechanical? **YES** — single-cell wording addition,
  consistent with Axis 1 / Axis 2 patterns.

**Verdict: PASS** — escalation is content-driven, not fabricated. The
fix is conservative (only Axis 13 Maps-to; F9 rationale preserved per
cycle-39's verdict).

### Q2 — Q(b) escalation calibration

Was BORDERLINE-FAIL the right verdict? Should this have been PASS-with-
note, or full FAIL?

**Re-walk:**

- PASS-with-note: would leave Axis 13's Maps-to inconsistent with
  Axis 1 / Axis 2 patterns. Future cycles would re-discover the same
  gap. Not the right verdict.
- BORDERLINE-FAIL: single-cell wording addition; bounded-mechanical
  fix that preserves cycle-39's verdict on F-pattern table. Right level.
- Full FAIL: would imply load-bearing structural rewrite, which this
  isn't. Not the right verdict.

**Verdict: PASS** — BORDERLINE-FAIL calibration is appropriate.

### Q3 — #809 closure appropriateness

Is the cycle-47 closure appropriate? Cycle-46 left it open as
"borderline" — what changed?

**Re-walk:**

Cycle-46's deferral was conservative: per-finding evaluation absorbs
the spirit, but the directive "isn't strictly redesign-specific."

Cycle-47 reconsideration adds two observations:
- The directive's substrate (implementation Copilot PRs to merge) does
  not exist in redesign mode and won't until post-cutover (if at all)
- Per-finding evaluation is **strictly stronger** than the directive's
  framing, so the directive is not just absorbed in spirit but
  superseded in practice

These observations strengthen the absorption signal sufficiently to
warrant closure. The linking comment preserves audit paper trail.

**Verdict: PASS** — closure appropriate; cycle-46 was correctly
conservative; cycle-47's reconsideration adds substantive evidence.

### Q4 — Asymptotic convergence claim refinement

Cycle-46 was first PASS-without-escalation. Cycle-47 escalates
(BORDERLINE-FAIL). Does this falsify the asymptotic convergence
hypothesis?

**Re-walk:**

The convergence hypothesis (cycle-45 origin, cycle-46 strengthened):
escalations get smaller across cycles until eventually no escalation
is needed.

Cycle-47 evidence:
- Escalation magnitude: single-cell wording addition (smaller than
  cycle-44's load-bearing dep map rewrite, comparable to cycle-45's
  single-cell wording change)
- Escalation source: cycle-47 lens (cross-axis dep map sweep) is a
  **different lens** than cycle-46's (F-pattern rationale sweep). The
  new lens found a gap that the prior lens didn't surface.

**Hypothesis refinement: convergence is per-lens, not absolute.**

Each new lens applied to the framework can find new gaps that prior
lenses didn't surface. Within a given lens, escalations shrink toward
zero across cycles. But applying a new lens can re-surface new
escalations.

This is consistent with:
- Cycle 44/45 within F-pattern rationale lens: load-bearing → single-
  cell → no-fix
- Cycle 47 with cross-axis dep map sweep lens: single-cell (first
  application of this lens; would also shrink in subsequent
  applications)

**Implication for cycles 48-50:** if the same cross-axis dep map sweep
is repeated, expect smaller escalations or PASS. New lenses (e.g.,
candidate-template adequacy, position-table consistency) might surface
new gaps.

**Verdict: PASS** — the convergence hypothesis is **refined** (per-
lens) rather than falsified. Cycle-47's escalation is consistent with
the refined model.

### Q5 — Cycle 48 pre-commit checklist scope

Three questions for cycle 48's cold-reader:

- **(a) v1.10 confirmation re-walk:** re-walk the v1.10 Axis 13 Maps-to
  addition with fresh adversarial framing. Specifically: does the
  three-clause structure ("indirect contributor — mechanism; thin/
  medium harness leaves...; load-bearing fix is Axis 7") have any
  internal precision gap? Compare to Axis 2's two-clause pattern
  ("indirect contributor — mechanism; load-bearing fix is...").
  Asymptotic convergence hypothesis (per-lens refinement) predicts
  PASS unless cycle-47's framing introduced new precision gap.
- **(b) New focus area: Maps-to ↔ F-pattern table consistency sweep.**
  Walk all axes' Maps-to lines and cross-check against F-pattern
  table to find more asymmetries like the Axis 13/F9 gap. Specific
  candidates to probe: does any axis's Maps-to mention an F-pattern
  not listed in the F-pattern table? Does any F-pattern row in the
  table list an axis whose Maps-to doesn't reference back?
- **(c) Bounded-mechanical TBD:** options:
  - Continue Maps-to consistency sweep — fix any additional gaps
    surfaced by Q(b) (if found)
  - Cross-axis-impact-check scaffold (Path A T+0) — check whether
    cycle-46 deferral still applies given continued no-escalation
    cycles
  - Iterate redispatch tool design draft — create initial draft
    file (deferred multiple cycles)
  - Phase 2 candidate template empirical pilot — gate per
    post-retrospective checkpoint state

Three questions covering different lenses. Each falsifiable.

**Verdict: PASS.**

## What surprised me

**The v1.4 history trace.** Cycle-39 (v1.4) explicitly verified
F-pattern table level for F9 ("F9 → Axis 7 with Axis 13 indirect via
cross-axis deps is the right level"). When cycle-47 first surfaced the
Axis 13 Maps-to gap, the natural reaction was to also annotate F9's
rationale (matching F11's pattern). But reading cycle-39 reveals that
the F-pattern table level was deliberately accepted — F9 row should
**not** annotate Axis 13. The cycle-47 fix is correctly limited to
Axis 13's Maps-to (which cycle-39 didn't re-review).

This is the kind of cross-cycle constraint that the cross-axis-impact-
check tool design (cycle-44/46 _drafts) would NOT catch — it's a
historical decision-trace constraint, not a structural cross-reference.
Future-cycle constraint: when escalating, check whether prior cycles
explicitly accepted the current state.

**The asymptotic convergence refinement.** Going into cycle 47 the
hypothesis was binary: convergence is real or oscillatory. Cycle-47's
evidence (escalation under a new lens after cycle-46's PASS under prior
lens) suggests a **per-lens convergence** model: each lens shrinks to
PASS within a few cycles, but new lenses can find new gaps. This is a
more nuanced model than binary asymptotic-vs-oscillatory.

**The dep map vocabulary diversity finding.** While walking the dep
map for Q(b), I observed entries use varied vocabulary: strength-of-
dependency (forces, enables, requires, implies), preference (pairs
naturally, aligns with), capability (can, must, shapes). This isn't
escalation-worthy but is a structural pattern worth noting. Compare
to cycle-46's "F-pattern rationales naturally use three precision
patterns" — dep map entries use a richer vocabulary because dep map
has more dependency-strengths to express. Pattern observation, not
fix.

## What I couldn't figure out

**Whether F9's rationale should ALSO be annotated for full F11 pattern
symmetry.** The conservative cycle-47 escalation only updates Axis 13's
Maps-to. The "full F11 pattern symmetry" alternative would also annotate
F9's rationale with Axis 1 and Axis 13 indirect contributors. I leaned
conservative because:
- Cycle-39 explicitly accepted F-pattern table level
- The Axis 13 Maps-to fix alone addresses the asymmetric Maps-to
  pattern (Axis 1 has F9 annotation; Axis 13 should too)
- F9 rationale annotation would reverse a deliberate decision

But this is only the *conservative* read. A future cycle might
revisit if a Phase 2 candidate author working from F9's row reports
missing the indirect-contributor pointer. Until that evidence emerges,
preserve cycle-39's verdict.

**Whether the per-lens convergence model is correct or whether more
lenses are still pending.** Cycle-46's lens was F-pattern rationale
precision. Cycle-47's lens is cross-axis dep map sweep. Cycle-48 could
use a different lens (Maps-to ↔ F-pattern table consistency, candidate-
template adequacy, position-table consistency). Each new lens may find
its own gaps. The framework's true stability is reached when all
relevant lenses have been applied and converged — not when one lens
converges.

This is harder to plan for: how many lenses are there? When do we
know we've applied them all? The asymptotic-convergence hypothesis
under per-lens refinement is genuinely uncertain.

**Whether to start the cross-axis-impact-check tool build.** Cycle-46
deferred to post-checkpoint; cycle-47 finding (Axis 13 Maps-to gap) is
exactly the kind of cross-axis consistency the tool would check — but
only if the tool's pattern coverage extends to "axis Maps-to ↔ axis
indirect contribution annotations" type checks. The cycle-46 design
draft's Q2 scope (table-strict + regex-prose for known patterns)
might not cover Maps-to indirect annotations. Future build should
include this pattern.

## Status note

- Open `question-for-eva`: 0
- Open `input-from-eva`: 6 (down from 7; #809 closed)
- Open audit-outbound: #442 (Phase 0 critique, integrated)
- Phase 1 deliverable: v1.10 design framework (cycle-47 confirmed-stable
  on F8 area; Axis 13 Maps-to F9 indirect annotation added)
- Phase 0 deliverable: `docs/redesign/0-retrospective.md` (iterating;
  awaiting post-retrospective checkpoint per Eva discretion)
- Cycle 47 is the **sixth** cold-reader cycle in the v1.X sequence
  (cycles 38, 42, 44, 45 produced v1.X bumps; cycle 46 PASS-without-
  escalation; cycle 47 BORDERLINE-FAIL with v1.10 single-cell fix)

## Pre-commit checklist for cycle 48's cold-reader

Three questions:

- **(a)** v1.10 confirmation re-walk: re-walk the v1.10 Axis 13 Maps-to
  addition with fresh adversarial framing. Three-clause structure
  precision check. Asymptotic convergence (per-lens) hypothesis
  predicts PASS unless cycle-47's framing introduced new precision
  gap.
- **(b)** Maps-to ↔ F-pattern table consistency sweep: walk all axes'
  Maps-to lines and cross-check against F-pattern table for additional
  asymmetries. Probe both directions: does any Maps-to mention an
  F-pattern not in the table? Does any F-pattern row mention an axis
  whose Maps-to doesn't reference back?
- **(c)** Bounded-mechanical TBD: choose one or two from:
  - Continue Maps-to consistency sweep — fix additional gaps if any
  - Cross-axis-impact-check scaffold start (Path A T+0)
  - Redispatch tool design draft — create initial draft file
  - Phase 2 candidate template empirical pilot (gated on checkpoint)

## Cycle 48 plan (provisional)

1. **Substantive focal:** cross-cycle cold-reader on v1.10 + cycle-47
   work (3 Qs above).
2. **Substantive parallel:** TBD per cold-reader. If Q(a) PASSes and
   Q(b) finds at most a small gap, bounded-mechanical capacity for
   one of the cycle-48 (c) options.
3. **Bounded mechanical:** continued Maps-to consistency sweep is
   strongest if Q(b) surfaces additional gaps; redispatch tool draft
   creation otherwise.

## What this cycle achieved

Cycle 47 is the **sixth cold-reader cycle** in the v1.X sequence and
the **first cycle to apply a different lens** than the F-pattern-
rationale precision lens used in cycles 44-46. The substantive output:

- 3 cold-reader questions answered (1 PASS + 1 BORDERLINE-FAIL + 1
  procedural decision)
- v1.10 application: Axis 13 Maps-to extended with F9 indirect
  contributor annotation (single-cell wording addition)
- Bounded-mechanical: 1 input-from-eva closure (#809 absorbed by
  per-finding evaluation discipline + redesign-mode dispatch lifecycle)
- 1 cycle-47 same-cycle review (5 questions, all PASS)

The most interesting cross-cycle observation: **the asymptotic
convergence hypothesis is refined to per-lens convergence**. Each lens
applied to the framework shrinks toward PASS within a few cycles, but
applying a new lens can surface new gaps. This is consistent with the
cycle-44/45/46 sequence (F-pattern rationale lens converging) and the
cycle-47 escalation (cross-axis dep map sweep lens finding new gaps on
first application).

The structural observation: **dep map entries use a richer dependency
vocabulary** than F-pattern rationales — strength-of-dependency,
preference, capability framings used appropriately to content.
This is not escalation-worthy but is a structural pattern worth
documenting.

The historical lesson: **cycle-39's explicit verdict on F-pattern table
level for F9 is a load-bearing constraint** that the cycle-47 escalation
correctly preserved. The conservative cycle-47 fix (only Axis 13 Maps-to,
not F9 rationale) honors cycle-39's deliberate decision while addressing
the unintentional Maps-to gap.
