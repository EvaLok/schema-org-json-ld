# Cycle 36 (2026-04-30) — Cold-reader on cycle 35 framework + framework iteration (Q1, Q3 applied; Q2/Q4/Q5 decisions documented; F11 mapping corrected) + housekeeping continuation

## Setup

Cold-start session. Cron fired 2026-04-30 22:16 UTC (issue #2797).
Seventh cycle of 2026-04-30 (after 28-32 + 34 + 35). Cycle 35
produced the Phase 2 design-axes synthesis (commit `1734c61a`):
7 convergent constraints + 11 real axes + cross-axis dependencies +
F-pattern→axis mapping (F2 not externally validated) + 5 open
framework questions for cycle 36+ iteration.

Cycle 35 named two non-mutually-exclusive options for cycle 36:
1. **Substantive focal: latent dispatch backlog.** Re-dispatch
   firewall-newly-unblocked deeper reads (#2779 Cognition Devin or
   #2781 OpenAI harness) per Eva directive #2794.
2. **Bounded mechanical: housekeeping continuation.** 4 more
   closures from the 7-item deferred set.

Cycle 36 picks neither as the primary substantive focal. Rationale:
the cycle-35 pre-commit checklist named 3 cold-reader questions
explicitly, and the 5 open framework questions are higher-leverage
substance than re-dispatching one external reading. Re-dispatch
involves non-trivial protocol decisions (close old PR/issue or
create new dispatch issue?) that benefit from explicit cycle-37
research rather than rushing a partial-dispatch within cycle 36.
Housekeeping continues at the 4-closure cadence per cycle-35 cold-
reader (b)'s "validate sustainable cadence" finding.

**Cycle 36 actual focal:** cold-reader on cycle 35 framework + apply
the corrections / decisions that fit one cycle's budget; document
the rest as deferred to cycle 37+ with explicit decisions.

## Cold-reader on cycle-35 framework

Three questions named in cycle-35 pre-commit checklist.

### Question (a) — Are the 11 axes genuinely independent?

**Spot-check:** Axis 5 (plans-as-artifacts) vs Axis 2 (state
representation primitive). Is plans-as-artifacts a true independent
dimension or a special case of Axis 2 file-per-component state?

**VERDICT: PASS with cleanup recommended.**

**Analysis.** Axis 2 is "what is the unit of persistent state?" — a
STRUCTURE question about state representation. Axis 5 is "are
plans/specs first-class versioned artifacts written before
execution?" — a TEMPORAL/LIFECYCLE question about a particular kind
of state.

The 2x2 of Axis 2 × Axis 5 is meaningfully populated:
- **file-per-component (Axis 2) + plans-as-artifacts (Axis 5):**
  OpenAI harness occupies this cell.
- **typed-channel-map (Axis 2) + no-plans-as-artifacts (Axis 5):**
  LangGraph occupies this cell.
- **file-per-component (Axis 2) + no-plans-as-artifacts (Axis 5):**
  AutoGen, Voyager occupy this cell.
- **repository-as-state (Axis 2) + plans-as-artifacts (Axis 5):**
  OpenAI harness (state shape AND plan substrate use git, but the
  TEMPORAL question is independent of the structural question).

All four cells are populated by surveyed systems' positions. The
axes are genuinely independent dimensions.

**Cleanup recommendation:** Axis 2's table currently lists
"Plans-as-artifacts (orthogonal)" as a position with the "(orthogonal)"
annotation. This double-listing creates ambiguity: if plans-as-
artifacts IS a separate axis (Axis 5), why is it ALSO a position
in Axis 2's table? The annotation tries to flag this honestly, but
it would be cleaner to remove "Plans-as-artifacts (orthogonal)"
from Axis 2's table since Axis 5 captures it as a dedicated
dimension.

**Mild flag for cycle-36+:** the cleanup is bounded-mechanical
(remove one row from Axis 2's table; add a cross-reference sentence
"plans-as-artifacts is a separate temporal/lifecycle dimension; see
Axis 5"). Apply in cycle-37 framework iteration.

### Question (b) — Does the F-pattern → axis mapping have miscategorized rows?

**Spot-check:** F11 (post-close mutations) maps to Axis 4 (history
substrate) + Axis 9 (iteration ceilings). Does Axis 9 actually
address F11, or is that a stretch?

**VERDICT: FAIL on F11 → Axis 9 mapping.**

**Analysis.** Looking at F11's content from `0-retrospective.md`
lines 668-710:

- F11 is "cycle closure as artificial completion signal" — the
  measurement caught 4.3 post-close state mutations per cycle,
  with 5 distinct state fields routinely mutated post-close, none
  reconciled in the frozen worklog.
- The mechanism: tools mutate defense fields at post-close trigger
  points (next cycle's `record-dispatch`, `process-merge`,
  `cycle-complete`, review-event consumption).
- The failure is TEMPORAL (cycle-close boundary is artificial; state
  evolves continuously) and STRUCTURAL (state.json is destructive-
  write; the worklog freeze can't reflect post-close mutations).

Axis 9 is "iteration ceilings" — about whether autonomous loops are
bounded by loop-count or runtime. F11 isn't about loops being
unbounded; it's about cycle-close being a misleading boundary in
state evolution. Loop-ceilings on `record-dispatch` or
`process-merge` wouldn't fix F11 — these tools fire at cycle-
boundary triggers, not in unbounded loops.

The cycle-35 mapping rationale was: "Append-only with branching
prevents post-close destructive writes." This rationale is correct
but maps to **Axis 4 (history substrate)** — append-only AND
branching are both Axis 4 positions. It does NOT map to Axis 9.

**Correction for cycle-36+:** F11's correct axis mapping is:
- **Axis 4 (history substrate)** — append-only or branching prevents
  the destructive-write semantics that make post-close mutations
  invisible.
- **Axis 2 (state representation)** — file-per-component or typed-
  channel-map naturally supports per-component append rather than
  monolithic destructive overwrite.
- **Removed: Axis 9 (iteration ceilings)** — does not address F11.
- **Possibly: CORE-DESIGN-PRINCIPLE** — the post-close mutations are
  done by tools, but the orchestrator chose to invoke them at those
  trigger points; a different harness-vs-session boundary
  (proposed Axis 13 in Q4 below) might handle this differently.

This is a FAIL verdict because the mapping was wrong, not borderline.
The cycle-35 same-cycle review didn't catch it because the same-
cycle review was checking the framework's structural choices, not
spot-checking individual mapping rows. Cross-cycle cold-reader has
higher confidence than same-cycle review because the latter operates
in the same reasoning frame as the original work.

**Apply now:** correction documented in this cycle's notes; the
mapping table itself lives in cycle-35 notes (frozen historical
record) and the corrected version is below in this notes file.

### Question (c) — Is "convergent constraints" framing honest?

**Spot-check:** Constraint 7 (memory as architectural concern). Is
"3+/5 + diversity hedge" really constraint-status given the
diversity?

**VERDICT: BORDERLINE-PASS with framing refinement recommended.**

**Analysis.** Constraint 7 reads: "Memory is an architectural
concern requiring elevation. (Family C pattern 4, 3+/5 + diversity
hedge.) The shape of the memory subsystem is a choice (see Axis 3
below), but the convergent claim — memory deserves first-class
architectural treatment, not derivative-of-state — is a constraint."

The underlying pattern in `1-research.md` line 219-228:
- 5-system convergence on memory-as-architectural-concern
  (LangGraph, Cognition Devin, OpenAI harness, oh-my-codex, PAI)
- 5 different specific shapes (typed channels, context trace, repo-
  as-record, wiki+search, top-level principle)
- Voyager's SkillManager+Chroma "framed as skill-storage rather
  than memory-as-such" — explicitly not counted

**The claim "memory deserves architectural elevation" is at 5/5
convergence among systems where memory is even named.** That's
strong convergent practice evidence.

**The claim "not derivative-of-state" is INFERRED, not directly
cited.** No surveyed system explicitly argues "memory should not be
derivative of state" — they all simply elevate memory architecturally.
Positive convergence (do X) doesn't automatically translate to
negative rejection (don't do Y). v1's derivative-of-state position
is INCOMPATIBLE with what 5 systems do, but isn't actively REJECTED
in writing by those systems.

The "+ diversity hedge" annotation correctly notes the shape diversity
but is potentially confusing because it might suggest the constraint
itself is hedged. The constraint isn't hedged at the level of "elevate
memory" — only at the level of "what shape." The annotation is
technically correct but its placement could mislead.

**Borderline-PASS rationale:** the constraint is defensible at strict
reading because 5/5 systems elevate memory architecturally. The
diversity is real but doesn't undermine the constraint claim
(elevation, not shape).

**Refinement recommendation for cycle-37+:** rephrase constraint 7
to ground "not derivative-of-state" in absence-of-contrary-practice
rather than active-rejection. Suggested wording:

> 7. **Memory is treated as architectural elevation, not derivative
>    of state.** (Family C pattern 4, 5/5 systems elevate memory;
>    0 surveyed systems treat memory as derivative-of-state.) The
>    shape of the memory subsystem is a choice (see Axis 3); the
>    convergent practice — memory has first-class architectural
>    treatment — is the constraint. *(Voyager's SkillManager+Chroma
>    is adjacent; framed as skill-storage by the source repo, not
>    counted in the elevation evidence.)*

This refinement makes the inference (no system does derivative-of-
state) explicit rather than embedded.

### Cold-reader summary

- Q[a] PASS with cleanup recommended (remove plans-as-artifacts row
  from Axis 2 table; add cross-reference to Axis 5).
- Q[b] **FAIL on F11 → Axis 9 mapping**; correction: F11 → Axis 4 +
  Axis 2 (and possibly CORE-DESIGN-PRINCIPLE elaboration).
- Q[c] BORDERLINE-PASS with framing refinement recommended (ground
  "not derivative-of-state" in absence-of-contrary-practice).

The Q[b] FAIL is the load-bearing finding of this cold-reader.
Cross-cycle cold-reader caught a wrong mapping that same-cycle
review missed, validating the cross-cycle review primitive's
value-add over same-cycle review-only.

## Decisions on cycle-35's open framework questions

Cycle 35 named 5 open framework questions. Decisions follow.

### Q1 — Should Axis 11 (operator-vs-goal) be eliminated as non-differentiating?

**Decision: Move Axis 11 to convergent constraints as constraint 8
(goal-driven by mission).**

**Rationale.** Cycle-35's open question framed the choice as
"keep the axis as documentation vs remove because non-differentiating."
The principled fix is neither — it's reclassification. An axis where
every v2 candidate must take the same position isn't a differentiation
dimension; it's a constraint.

The mission committedness to goal-driven (per the redesign prompt's
primary thesis: "self-healing self-improving AI" + autonomous schema
work) is a constraint on every candidate. The persistent divergence
between openclaw (operator) and PAI (goal) is documentary about
those two systems' surveyed positions, not a candidate-generation
dimension for v2.

**Constraint 8 (revised, replacing the deleted Axis 11):**

> 8. **Goal-driven over operator-driven.** (Family A pattern 3,
>    persistent divergence; v2 candidates committed to goal-driven
>    by mission.) The redesign's primary thesis (autonomous self-
>    healing AI pursuing schema-domain work) commits to goal-driven
>    as the top-level posture. Operator-driven sub-systems may exist
>    within a goal-driven overall posture (e.g., Eva-issued
>    `input-from-eva` directives as explicit operator-commands), but
>    the top-level operator-vs-goal choice is fixed by mission.

**Apply when:** in cycle-37 framework iteration, when promoting the
framework to a Phase-2-input section in `1-research.md`, eliminate
Axis 11 and add the revised constraint 8.

**Open detail:** the operator-driven sub-system question (Eva-
directives shape) might fold into the proposed Axis 12 (Reconciliation
discipline, see Q2 below), since Eva-directives are an inbound
channel that requires reconciliation-asymmetry handling.

### Q2 — Should F2's reconciliation-asymmetry become a 12th axis or stay as cross-cutting principle?

**Decision: Add Axis 12 — Reconciliation discipline. Mark explicitly
as v1-derived, not externally validated.**

**Rationale.** F2 (Eva-response detection) and the broader
reconciliation-asymmetry family don't map onto any of the 16 cross-
system patterns because the surveyed systems don't have an Eva-
equivalent. Cycle 35 named this gap honestly. Two options:
(a) make F2 a 12th axis derived from v1's failure-mode analysis;
(b) keep it as cross-cutting CORE-DESIGN-PRINCIPLE elaboration.

The 12th-axis approach is more rigorous because:
- It forces candidate generators to make their reconciliation choice
  explicit
- The "v1-derived" caveat is honest about the reduced confidence
- It puts F2's load-bearing structural choice on equal footing with
  the externally-validated axes
- It gives candidates a structured way to address the
  reconciliation-asymmetry family (F2 / F3 / F4 / F11 partial)

**Axis 12 (proposed):**

> #### Axis 12 — Reconciliation discipline (*v1-derived; not externally validated*)
>
> **The choice:** how does the system reconcile inbound external
> events (Eva responses, audit posts, dispatch outputs, post-close
> tool mutations) into state?
>
> | Position | Notes |
> |---|---|
> | No reconciliation: write-only outbound channels | v1 anti-pattern (F2/F3/F4/F11 emerge from this) |
> | Active polling: each outbound channel paired with a reader producing state transitions | High-cost; requires per-channel discipline |
> | Event-driven: state changes reactively when external events arrive | Requires inbound trigger infrastructure (webhook, GitHub Actions on event) |
> | Hybrid: polling for low-frequency channels, event-driven for high-frequency | Most likely v2 candidate position |
>
> **Status:** v1-derived axis; no external system surveyed has an
> Eva-equivalent that would constrain the choice. Candidates that
> address Axis 12 are doing more design work than those addressing
> externally-validated axes; candidates may also choose to fold this
> into existing axes (e.g., Axis 4 history substrate where event-
> driven means "git events trigger state recompute") rather than
> treating as separate.
>
> **v1's position:** no reconciliation. Outbound channels (issue
> creates, PR creates, journal commits) are well-developed; inbound
> reconciliation does not exist. The retrospective documents F2/F3/
> F4 as direct manifestations.
>
> **v2 candidate space:** every position EXCEPT "no reconciliation"
> is defensible. Hybrid is the path of least design-cost since
> different channels naturally have different polling frequencies.
>
> **Cross-axis dependency:** Axis 12 × Axis 4 (history substrate) —
> event-driven reconciliation pairs naturally with git-as-substrate;
> Axis 12 × Axis 1 (decomposition) — small-fixed-team can have a
> dedicated reconciliation agent.
>
> **Maps to:** F2 (Eva-response detection), F3 (multi-candidate
> state drift), F4 (frozen-artifact lifecycle fragility), F11
> partial (post-close mutations as reconciliation-asymmetry
> manifestation).

**Apply when:** cycle-37 framework iteration, when promoting to
Phase-2-input section.

### Q3 — Does the framework smuggle Phase 2 priority into its ordering?

**Decision: Add explicit "axis order is not significance order"
note in the framework section preamble.**

**Rationale.** The cycle-35 axis ordering (1: agent decomposition,
2: state representation, 3: memory, 4: history, 5: plans-as-artifacts,
6: extension, 7: orchestration topology, 8: mechanical enforcement,
9: iteration ceilings, 10: entropy, 11: operator-vs-goal) is roughly
"weightier-first" — most architecturally-load-bearing decisions are
1-4 with refinements at higher numbers.

This ordering smuggles weight (and thus priority) into the framework
structure. A Phase 2 candidate might find Axis 8 (mechanical
enforcement) more load-bearing than Axis 6 (extension shape) for
their specific design — but the numbering implies otherwise.

The cleanest fix is a note: "Axis numbering is for reference only;
no significance/priority is implied. Candidates may address axes in
any order, prioritizing what's load-bearing for their design."

Possible alternative orderings considered:
- **Alphabetical by axis name** — neutral, but loses navigational
  affinity (e.g., the natural cluster of state/memory/history is
  scattered).
- **By Family-membership** of underlying patterns (Family C/E/A/B/D
  ordering used in 1-research.md cross-system observations) — has
  navigational affinity but inherits the family-ordering choice's
  implicit weight signal.
- **Current numerical** with disclaimer — simplest, most navigable;
  disclaimer disclaims the priority implication.

**Lean toward current numerical with disclaimer.** Less restructuring,
preserves navigational affinity, makes the priority-smuggling
disclosure explicit rather than hidden.

**Apply when:** cycle-37 framework iteration; one-sentence preamble
addition.

### Q4 — What's missing? Adversarial check.

**Decision: Add Axis 13 — Harness-vs-session boundary. Note other
candidate dimensions as "considered, folded into existing axes" with
explicit fold-in citations.**

**Rationale.** Adversarial check on what's missing surfaced multiple
candidate dimensions:

a. **Cycle-runner harness vs orchestrator session boundary.** Where
   is the line between deterministic harness code and LLM session?
   This is directly tied to CORE-DESIGN-PRINCIPLE but isn't an
   explicit axis.
b. **Polyglot / multi-language schema strategy.** How are language
   ports kept in sync? This is schema-domain-specific; could be a
   candidate-considered axis.
c. **Audit-repo integration mechanism.** How does v2 communicate
   with the audit-repo orchestrator?
d. **Eva-checkpoint mechanism specifics.** What triggers a checkpoint?
   How does the orchestrator know it's at one?
e. **Security posture per-trust-tier specifics.** How does the
   prompt handle untrusted text?
f. **Prompt size / character budget.** How long is the prompt?
g. **Cold-start ergonomics.** How much does a cold-start session
   need to read before being productive?
h. **Failure-mode taxonomy / catalog maintenance.** Does v2 update
   its own anti-pattern catalog?

**Highest-leverage missing axis is (a)** — Harness-vs-session
boundary. CORE-DESIGN-PRINCIPLE ("tools handle rote; orchestrator
handles judgment") is named cross-cutting, but the cycle-35 framework
doesn't have an axis where candidates declare WHERE the boundary
falls. Different positions on this axis have radically different
implications:

- **Thin harness, fat session** — most procedure in prompt; LLM
  re-derives the procedure each cycle (v1's current shape).
- **Medium harness, medium session** — split between cycle-runner
  and prompt; harness handles known patterns, prompt handles novel.
- **Fat harness, thin session** — most procedure in deterministic
  code; prompt is small reference + judgment-call decisions.

This is clearly a real axis where v2 candidates would differ. And
it's directly tied to CORE-DESIGN-PRINCIPLE.

**Axis 13 (proposed):**

> #### Axis 13 — Harness-vs-session boundary (*cross-cutting CORE-DESIGN-PRINCIPLE elaboration*)
>
> **The choice:** where is the line between deterministic harness
> code and LLM session?
>
> | Position | Notes |
> |---|---|
> | Thin harness, fat session | Most procedure in prompt; LLM re-derives procedure each cycle (v1's shape) |
> | Medium harness, medium session | Split between cycle-runner and prompt; harness handles known patterns, prompt handles novel |
> | Fat harness, thin session | Most procedure in deterministic code; prompt is small reference + judgment-call decisions |
>
> **Status:** cross-cutting CORE-DESIGN-PRINCIPLE elaboration. Every
> v2 candidate must declare its position; the principle requires
> "tools and deterministic processes handle repetitive, rote,
> procedural work" — implying the harness-vs-session line should be
> drawn farther toward fat-harness than v1's shape.
>
> **v1's position:** thin harness (cycle-runner mostly invokes the
> session), fat session (prompt + 2 checklists encode the procedure
> the orchestrator follows each cycle).
>
> **v2 candidate space:** medium-or-fat harness positions are the
> CORE-DESIGN-PRINCIPLE-aligned choices. Thin harness is the v1
> anti-pattern. The choice between medium and fat depends on what
> procedures get extracted into tools — a candidate must specify
> the tool surface implied (per the cycle-35 candidate template's
> "Tool surface implied" section).
>
> **Cross-axis dependency:** Axis 13 × Axis 6 (extension shape) —
> the extension primitive (plugins/skills/tools/etc.) shapes how
> harness procedures get organized; Axis 13 × Axis 8 (mechanical
> enforcement) — fat harness implies more mechanical-enforcement
> surface area.
>
> **Maps to:** F1 (constraint accretion in prompt), F6 (cyclomatic
> procedure depth), F7 (self-management dominance via prompt-encoded
> procedure), CORE-DESIGN-PRINCIPLE explicitly.

**Other dimensions (b)-(h):** noted but not promoted to axes.
Justifications:

- (b) **Polyglot strategy** is schema-domain-specific. Phase 3
  prototype includes one polyglot end-to-end test. The polyglot
  strategy is part of Phase 3 design, not the v2 prompt-level
  axes. Folded into Axis 6 (extension shape) — language-port tools
  are extensions.
- (c) **Audit-repo integration** is part of Axis 12 (Reconciliation
  discipline) — audit-orchestrator posts are an inbound channel
  requiring reconciliation. Folded into Axis 12.
- (d) **Eva-checkpoint mechanism specifics** are workflow-procedural,
  not architectural. Folded into Axis 7 (orchestration topology)
  for the "what triggers a checkpoint" question and Axis 12
  (Reconciliation discipline) for the "how does the orchestrator
  know" question.
- (e) **Security posture per-trust-tier specifics** are part of
  convergent constraint 3 (Strong-defaults security with operator-
  controlled knobs) — every candidate must honor; the trust-tier
  specifics are implementation detail, not axis-level differentiation.
- (f) **Prompt size budget** isn't a candidate-differentiation axis
  per se; it's an outcome of Axis 13's position. Smaller prompts
  fall out of fat-harness candidates.
- (g) **Cold-start ergonomics** is workflow detail, not architectural.
  Possible PERSISTENCE-mechanism design choice but doesn't
  differentiate Phase 2 candidates at the architectural level.
- (h) **Failure-mode catalog maintenance** is a CORE-DESIGN-PRINCIPLE
  elaboration: does v2 update its own anti-patterns catalog?
  Folded into convergent constraint 5 (Anti-patterns documented as
  deliverable artifact) plus Axis 8 (Mechanical enforcement) for
  the "how is it kept current" question.

**Apply when:** cycle-37 framework iteration; Axis 13 added; (b)-(h)
fold-in citations added to relevant axes/constraints.

### Q5 — How does the framework interact with v1's preserved primitives?

**Decision: Add a "preserved-primitives interaction" subsection to
the framework, naming for each preserved primitive which axes
interact and the design constraints implied.**

**Rationale.** PRESERVED-PRIMITIVES (per redesign prompt SECTION 3)
are: journal, cycle-issue, question-for-eva, input-from-eva,
git-safety, cycle-runner harness. Walking the interaction:

- **Journal** (`docs/journal/YYYY-MM-DD.md`, freeform per-cycle
  subsection): No axis position conflicts. Every axis accommodates
  journal as a write-only output artifact. Note: Axis 3 (memory
  shape) includes journal as a long-term-memory primitive in v1;
  v2 candidates may choose other primitives but journal as
  preserved-primitive remains as one channel.
- **Cycle-issue** (`orchestrator-run` label, session-bracket
  comments): All axes accommodate. Axis 7 (orchestration topology)
  "Multi-pattern coexisting" might have multiple sub-cycles within
  one cycle-issue boundary — that's fine because sub-cycles are
  internal to the issue.
- **Question-for-eva** + **input-from-eva**: Axis 12
  (Reconciliation discipline) is directly implicated. Some Axis 12
  positions (event-driven, hybrid) integrate Eva responses more
  cleanly than active-polling-only.
- **Git-safety** (commit-must-be-pushed): Axis 4 (history substrate)
  "Branching checkpoints" might conflict if branching means
  git-branches that don't get pushed. Mitigation: branching can be
  represented in-tree (separate files for branches) without using
  git branches. **Constraint added:** Axis 4 positions must honor
  git-safety; candidates choosing branching must specify whether
  branching is git-branches (problematic) or in-tree files
  (compatible).
- **Cycle-runner harness**: Axis 13 (Harness-vs-session boundary)
  is directly implicated. Different Axis 13 positions imply
  different changes to cycle-runner. **Constraint added:** Axis 13
  positions must specify the cycle-runner change scope (none,
  modest, substantial).

**Preserved-primitives interaction subsection (proposed):**

> #### Preserved-primitives interactions
>
> v1's preserved primitives (per redesign prompt SECTION 3) constrain
> v2 candidates' axis positions:
>
> | Preserved primitive | Axes implicated | Constraint implied |
> |---|---|---|
> | Journal | Axis 3 (memory shape) | Journal remains as one memory channel; candidates may add others |
> | Cycle-issue | Axis 7 (orchestration) | Multi-pattern topologies must produce session-end summary on cycle-issue |
> | Question-for-eva / input-from-eva | Axis 12 (Reconciliation) | Inbound Eva channels must be reconciled; pure write-only outbound rejected |
> | Git-safety (commit-must-be-pushed) | Axis 4 (history substrate) | Branching positions must be in-tree files, not git-branches |
> | Cycle-runner harness | Axis 13 (Harness-vs-session boundary) | Cycle-runner change scope must be declared (none/modest/substantial) |

**Apply when:** cycle-37 framework iteration, when promoting to
Phase-2-input section.

## Framework v1.1: corrections applied this cycle

The corrections that fit cycle 36's budget. The full v1.1 promotion
to a Phase-2-input section in `1-research.md` is deferred to cycle
37+ (substantial new content from Q2 Axis 12, Q4 Axis 13, Q5
preserved-primitives subsection).

### Correction 1 (from Q[b] FAIL): F11 axis mapping

Cycle-35 mapping: F11 → Axis 4 + Axis 9.

Corrected mapping: **F11 → Axis 4 + Axis 2** (and possibly
CORE-DESIGN-PRINCIPLE elaboration once Axis 13 is added).

Rationale: F11 is post-close state-mutation timing; Axis 9
(iteration ceilings) doesn't address temporal cycle-boundary
issues. Axis 4 (history substrate, append-only or branching) and
Axis 2 (state representation, file-per-component or typed-channel-
map) are the genuine structural addresses.

### Correction 2 (from Q[a] cleanup): Axis 2 table cleanup

Remove the "Plans-as-artifacts (orthogonal)" row from Axis 2's
position table. Add a one-sentence cross-reference: "Plans-as-
artifacts is a separate temporal/lifecycle dimension; see Axis 5."

Rationale: double-listing creates ambiguity; Axis 5 captures it
cleanly as a dedicated dimension.

### Decisions documented but DEFERRED to cycle-37+ for application

1. **Q[c] constraint 7 wording refinement** — refine to ground "not
   derivative-of-state" in absence-of-contrary-practice. Bounded-
   mechanical edit; defer because cycle 36 budget is committed
   elsewhere.
2. **Q1 Axis 11 → constraint 8** — eliminate axis; promote to
   constraint. Substantial restructuring (re-numbering doesn't
   apply to a frozen historical record; promotion to a Phase-2-
   input section is the right time to apply).
3. **Q2 Axis 12 (Reconciliation discipline)** — add as 12th axis,
   v1-derived. Substantial new content (~30 lines).
4. **Q3 axis-ordering disclaimer** — add one-sentence note. Bounded-
   mechanical; defer with the other promote-to-section work.
5. **Q4 Axis 13 (Harness-vs-session boundary)** — add as 13th axis,
   cross-cutting CORE-DESIGN-PRINCIPLE. Substantial new content
   (~40 lines).
6. **Q5 preserved-primitives interaction subsection** — add as
   framework subsection. Substantial new content (~20 lines).

Total deferred: ~100-120 lines of new framework content + scattered
edits. Cycle 37 should be able to apply all of this in one cycle if
focused. Or cycle 37 applies the deferred decisions; cycle 38
promotes the v1.1 framework to a Phase-2-input section in
`1-research.md`.

## Housekeeping continuation: 4 closures

Per cold-reader (b)'s "validate sustainable cadence" finding from
cycle 35, cycle 36 closes 4 more items at the upper bound of the
2-4 sustainable cadence. After this sweep, 3 deferred items remain
(if Cognition Devin and OpenAI harness pairs are held open for
re-dispatch in cycle 37+) or 1 item if those pairs are also closed.

Cycle 36 closes:
| Item | Cycle | Forward-link |
|---|---|---|
| PR #2756 (cycle-11 Phase 0 critique) | 11 | `0-retrospective.md` per-finding evaluations cycle 12 verdicts |
| Issue #2755 (cycle-11 dispatch parent) | 11 | Pair with PR #2756 |
| PR #2784 (oh-my-codex research) | 26 | `1-research/systems/oh-my-codex.md` (deep-dive) + 7 of 16 cross-system patterns |
| Issue #2782 (oh-my-codex dispatch parent) | 26 | Pair with PR #2784 |

**Held open** (per Eva directive #2794, awaiting cycle-37 re-dispatch
with firewall now open):
- PR #2780 + Issue #2779 (Cognition Devin) — firewall-blocked
- PR #2783 + Issue #2781 (OpenAI harness) — firewall-blocked

**Open: cycle-6 critique** PR #2749 — possibly the 7th deferred
item; verify in cycle 37 whether it's absorbed and close.

## Persistence-mechanism observations

### Cross-cycle cold-reader caught an error same-cycle review missed

Cycle 35's same-cycle review of its own framework found "PASS with
minor issues, all already flagged." The minor issues named were:
pattern-to-constraint vs pattern-to-axis double-mapping (intentional);
F11 → Axis 9 borderline (already in pre-commit checklist (b) for
cycle 36); "no monolithic global state file" implicit in Axis 2;
Axis 11 probably should be removed.

The F11 → Axis 9 case was self-flagged as borderline at same-cycle
review, but cycle 36 cross-cycle cold-reader VERIFIED it as a FAIL.
The same-cycle review's "borderline" assessment was generous — at
strict adversarial reading, the mapping is wrong, not borderline.

**Pattern observation:** same-cycle review is honest about
borderlines but tends to under-call them. Cross-cycle cold-reader
operates in a different reasoning frame and is more willing to
issue strict FAIL verdicts. Both are valuable; their combination
is stronger than either alone.

**Captured for v2 design-input:** the review-cadence pattern (same-
cycle review immediately + cross-cycle cold-reader on cycle N+1
notes) reliably catches more errors than either alone. v2 candidates
should preserve and possibly formalize this two-stage review
discipline.

### Framework iteration is itself a multi-cycle arc

Cycle 35 produced framework v1.0; cycle 36 produces v1.1 as a delta-
documented-in-notes (not yet promoted to a separate framework file
or Phase-2-input section). Cycle 37+ applies the v1.1 corrections
and adds Axis 12 / Axis 13 / preserved-primitives subsection. Cycle
38+ may promote to a Phase-2-input section in `1-research.md`.

This is the pattern of "design artifact iterates across cycles in
notes files, gets promoted to load-bearing files when stable." It's
the same pattern as cycles 30→31→32 (in-place restructure of
1-research.md cross-system observations: cycle 30 draft → cycle 31
verdict file → cycle 32 in-place application). Three cycles for an
architecturally-load-bearing change.

For framework iteration: cycle 35 v1.0 → cycle 36 v1.1 deltas →
cycle 37+ apply → cycle 38+ promote. Possibly four cycles for a
similar-magnitude change. Slightly longer because cycle 36 is doing
deltas-and-cold-reader rather than draft-and-feedback-dispatch (the
cycle-30 pattern). The "deltas-and-cold-reader" is faster per cycle
but produces less per cycle than "draft-and-application"; net
budget similar.

### `_notes/` index continues to be stale (cycle-3rd flag)

Cycle 33 noted, cycle 34 flagged, cycle 35 added another file
without updating, cycle 36 adds another file without updating.
The index `docs/redesign/_notes/README.md` covers cycles 2-7;
directory now contains 39 files (38 cycle-N + README).

The pattern (writer adds file, index goes stale) is now confirmed
across multiple cycles. v2 design-input still: persistence-mechanism
indexes should be auto-generated from filesystem listings, not
manually maintained.

**Bounded-mechanical action available cycle 37+:** write a small Rust
tool `tools/redesign-notes-index` that regenerates the README's
index table from the filesystem listing. Trivial bounded-mechanical
work; eliminates the manual maintenance burden. Captured as a
specific cycle-37+ candidate task.

## What surprised me

That the cross-cycle cold-reader found a strict FAIL in cycle-35's
F-pattern → axis mapping. I had expected at most a BORDERLINE
finding because cycle 35's same-cycle review had self-flagged F11 →
Axis 9 in pre-commit checklist (b) — implying the cycle-35 author
already saw something off there. But the verdict at strict adversarial
reading is harder than borderline: the rationale (append-only
prevents post-close mutations) maps to Axis 4, not Axis 9. There's
no defensible reading where iteration ceilings address F11.

The pattern is interesting: same-cycle review can sense
load-bearingness (the flag came from somewhere) but can't see clearly
across the same reasoning frame to issue the strict verdict. Cross-
cycle review can both sense AND verdict. This validates the cold-
reader primitive's distinct value-add.

## What I couldn't figure out

Whether the proposed Axis 12 (Reconciliation discipline) and Axis
13 (Harness-vs-session boundary) should both be added in cycle 37,
or whether one should be added cycle 37 and the other cycle 38.

Arguments for both-cycle-37: they're both substantial new content
(~30+40 lines = ~70 lines of axis prose), but both are bounded-
mechanical to draft from this notes file's specifications. One
cycle should fit.

Arguments for splitting: cold-reader on Axis 12 + Axis 13 is needed
before promotion to Phase-2-input section. Splitting allows
cycle 38's cold-reader to verify cycle 37's additions before cycle
39's promotion. The pattern would be: cycle 37 add Axis 12 + Axis
13 + preserved-primitives subsection in a v1.2 cycle-37-notes file;
cycle 38 cold-reader on cycle-37 notes; cycle 39 promote to Phase-2-
input section.

Lean toward both-in-cycle-37, with same-cycle review on the
additions and cross-cycle cold-reader from cycle 38. The split-
across-2 cycles pattern is conservative; given this notes file
already specifies both axes' content in detail, one-cycle
application is reasonable.

## Cycle-37 plan suggestion (provisional)

Two non-mutually-exclusive options, listed by leaning:

1. **Substantive focal: framework v1.2 application.** Apply the
   six deferred decisions from cycle 36 (Q[c] constraint 7 wording;
   Q1 Axis 11 → constraint 8; Q2 Axis 12; Q3 ordering disclaimer;
   Q4 Axis 13; Q5 preserved-primitives subsection). Produce a v1.2
   framework either inline in a cycle-37 notes file OR in a
   dedicated `docs/redesign/2-axes-framework.md` file (the latter
   is cleaner navigation and more obviously "this is the Phase-2-
   input artifact-in-progress"). Same-cycle review on the v1.2
   additions.

2. **Substantive parallel: re-dispatch decision and execution.**
   Research the Copilot dispatch protocol for re-running blocked
   dispatches. Decide: close old PRs/issues + create new dispatch
   issues, OR comment on existing dispatch issues to refresh.
   Execute one or both re-dispatches (#2779 Cognition Devin, #2781
   OpenAI harness) per Eva directive #2794. Estimated ~15-20 min
   for protocol research + dispatch.

3. **Bounded mechanical: housekeeping continuation.** 1-3 closures
   (cycle-6 critique PR #2749 if absorbed; PR #2780 + issue #2779
   if re-dispatched as new issues; PR #2783 + issue #2781 if same).
   Plus possibly write the `tools/redesign-notes-index` tool to
   regenerate the README from filesystem listing.

Lean toward option 1 as substantive focal + option 2 (re-dispatch)
as parallel substantive + option 3 partial as bounded mechanical.
Cycle 37's budget should accommodate all three at the rates
currently observed (~30 min substantive focal + ~20 min
substantive parallel + ~20 min mechanical).

If cycle 37 finds option 1 too much for one cycle, the natural
split is: cycle 37 applies Q[c] / Q1 / Q3 (bounded edits) + Axis 12;
cycle 38 applies Axis 13 + preserved-primitives + does same-cycle
review of cycle 37's additions; cycle 39 promotes to Phase-2-input
section.

## Pre-commit checklist (for cycle 37's cold-reader)

Three bounded-mechanical questions for the next cycle's cold-reader
on this notes file:

- **(a)** Is the F11 → Axis 4 + Axis 2 corrected mapping itself
  defensible, or did this notes file overcorrect from the FAIL
  verdict by adding Axis 2? Spot-check: is Axis 2 (state
  representation) genuinely addressing F11 (post-close mutations),
  or is the mapping a plausible-but-not-load-bearing add-in? If
  the answer is "not load-bearing," the corrected mapping is
  Axis 4 only.
- **(b)** Is the Q[c] borderline-PASS verdict rationale honest?
  Specifically: does the inference-not-active-rejection framing
  for "memory not derivative-of-state" actually weaken the
  constraint, or is the constraint just as strong with the
  refined wording as without it? If just as strong, the refinement
  is cosmetic and the borderline-PASS could have been an
  unqualified PASS.
- **(c)** Did the Q4 adversarial check on what's missing actually
  surface NEW dimensions that cycle-35 missed, or did it surface
  dimensions cycle-35 already considered and dismissed? Spot-check
  Axis 13 (Harness-vs-session boundary) — was this implicit in
  cycle-35's CORE-DESIGN-PRINCIPLE cross-cutting framing such
  that promoting to an axis is bookkeeping rather than
  substantive surfacing?
