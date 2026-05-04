# Cycle 65 — Cross-implications synthesis (cycle-62 AutoGen + cycle-64 LangGraph)

**Date:** 2026-05-04
**Substantive focal activity:** option 2 from input-from-eva [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) (cross-system synthesis — compare two or more systems already read at depth and write up the contrasts/patterns), applied to the implications-mining layer rather than the patterns layer.
**Trigger condition:** cycle-64's provisional read named cross-system implications synthesis as the cycle-65 substantive focal *if cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) had not returned*. Confirmed at session-start: dispatch is OPEN, 0 comments. Synthesis is the cycle-65 substantive focal.

## What this document is, and is not

This is a synthesis pass over `_notes/cycle-62-autogen-implications.md`
and `_notes/cycle-64-langgraph-implications.md` — the two
implications-mining documents produced under [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829)'s
polarity inversion. Cycle-64's "Cross-system convergence" section
already produced a first-pass mapping (3 convergent + 5
LangGraph-singular + 4 AutoGen-singular). This document goes further:

1. **Coverage check** — verify the first-pass categorization is
   complete; surface any implications that fell out of categorization.
2. **Higher-order clusters** — group all 16 implications by
   architectural theme to surface clusters that cycle-by-cycle
   authoring missed.
3. **Within-system cross-reference matrices** — write the matrices
   both cycle-62 ("the eight implications above intersect each other
   in ways not yet mapped... a future implications-mining cycle could
   write the cross-reference matrix") and cycle-64 (inherited the gap
   in its remaining-open list) flagged as deferred.
4. **Re-examination of partial convergences** — cycle-64's "singular"
   labels are first-pass; some may understate cross-system signals
   that emerge under closer reading.
5. **Elevation candidates** — for convergent implications that should
   become 1-research.md cross-system observations, draft how the
   elevation would look in Family format. Actual elevation is deferred
   per cycle-64 to a future cross-system-synthesis cycle (option 5
   from #2829); this document gives Phase 2 candidate authors a head
   start by drafting the form forward.
6. **Phase 2 design-input clusters** — which combinations of
   implications naturally compose into a candidate; which create
   tension; which are independent and can be mixed-and-matched.

This is NOT a Phase 2 candidate. It is synthesis-as-input. Phase 2
candidates still gate on the post-retrospective checkpoint and Eva
approval.

This is NOT a re-summary of cycle-62 / cycle-64 — those documents
contain the per-implication evidence with discounts. This file
operates above them at the synthesis layer.

## Inputs and method

**Inputs:**
- `_notes/cycle-62-autogen-implications.md` — 8 AutoGen implications
  (I-1 through I-8)
- `_notes/cycle-64-langgraph-implications.md` — 8 LangGraph
  implications (I-L1 through I-L8) plus a first-pass cross-system
  convergence section
- `1-research.md` Family A-E format for elevation drafting reference

**Method:**
- Side-by-side read of all 16 implications
- Categorization check against cycle-64's mapping
- Theme-clustering using the architectural-axis vocabulary already
  present in `2-design-framework.md` (Axis 1 prompt-vs-tool, Axis 2
  cycle boundary, Axis 4 state representation, Axis 7 multi-agent
  decomposition, Axis 13 anti-pattern integration) plus theme
  vocabulary that emerged organically from the implications
  themselves (lifecycle operations, documentation honesty,
  composition with namespaces)
- Within-system intersection mapping by reading each cycle's
  cross-reference annotations and surfacing additional intersections
  not annotated
- Elevation drafting against the Family format already used in
  `1-research.md` ("`[N+/N systems]`" maturity badges, diversity
  hedges, source citations)

## Findings

### Finding 1: Coverage check — cycle-64 dropped cycle-62 I-5 from categorization

Cycle-64's convergent + LangGraph-singular + AutoGen-singular lists
account for 3 + 5 + 4 = 12 implications. AutoGen has 8, LangGraph has
8 = 16 total; convergents counted from each side yields 16 - 3 = 13
unique. Cycle-64's split accounts for 3 convergent + 5 + 4 = 12; one
implication is unaccounted-for.

**The unaccounted implication:** cycle-62 I-5 (uniform tool-result
envelope, `is_error: true`). Not in the convergent list. Not in
cycle-64's AutoGen-singular list (which contained I-1, I-2, I-4, I-7).
This is an inadvertent omission in cycle-64's categorization.

**Why it was missed:** cycle-64's I-L2 (per-key reducers) carries an
explicit cross-reference to cycle-62 I-5: "both push toward typed
state with explicit semantics rather than ad-hoc shapes." Cycle-64
likely viewed the cross-reference as already covering I-5; the
formal categorization step omitted it.

**Where I-5 actually belongs:** under stricter categorization, I-5
is a partial convergence with I-L2. Both push toward typed contracts
on data boundaries:
- I-5: uniform envelope on tool→orchestrator output (across-process
  boundary, JSON-stdout)
- I-L2: per-key reducers on shared state (within-process boundary,
  in-memory channels with merge rules)

The patterns are NOT identical (different boundaries, different
substrates); they are *adjacent* — same architectural family of
"explicit semantics on data crossing a boundary." Cycle-64's
implicit categorization (treating the I-L2 ↔ I-5 cross-reference as
sufficient acknowledgment) is defensible but produces a coverage
gap in the formal split. This synthesis cycle restores I-5 to the
categorization as a partial-convergent with I-L2 (see Finding 4
below for the broader partial-convergent treatment).

**Methodological lesson.** First-pass categorization at the end of a
single-cycle implications doc tends to drop borderline cases that
sit between the strong-convergent and clean-singular categories.
Synthesis cycles are the natural correction point. Future
implications-mining cycles should expect their categorization to be
revised during synthesis.

### Finding 2: Higher-order clusters across all 16 implications

The 3+5+4 partition (or 3+5+4+1 after Finding 1) is by *cross-system
appearance pattern*. A different partition by *architectural theme*
surfaces clusters that span the partition:

#### Cluster A: Cycle-internal boundaries with state-write semantics

Implications about how a cycle's interior is structured into phases
or super-steps with explicit state-write boundaries, and how
state writes/failures behave across those boundaries.

- **Convergent** cycle-62 I-3 (termination as composable callable,
  graceful vs immediate) ↔ I-L1 (super-step model as cycle
  composition)
- **LangGraph** I-L3 (pending writes for failed siblings —
  partial-failure resumption)
- **LangGraph** I-L6 (durability modes as explicit per-write tradeoff)
- **LangGraph** I-L7 (restart-from-beginning idempotence on
  interrupts)

Five implications cluster here; four of the five are LangGraph-sided.
This makes sense — LangGraph's Pregel/BSP substrate forces explicit
within-cycle phasing, while AutoGen's actor model is event-driven
without super-step semantics. The 5-of-5 cluster says: v2's
within-cycle phase boundary discipline is a pattern that LangGraph
foregrounds heavily and AutoGen partially confirms (via I-3
termination predicates being the AutoGen-sided framing of the same
concept — boundary semantics).

#### Cluster B: Cross-cycle artifact organization

Implications about how artifacts are organized for institutional
memory across cycles, separating different content kinds or temporal
scopes.

- **Convergent** cycle-62 I-6 (plan-vs-progress artifact split) ↔
  I-L5 (short-term vs long-term memory split)
- **AutoGen** I-2 (layered architecture with intentional opinion-
  gradient) — applied to artifacts, this is opinion-stratified
  documentation

Three implications cluster here; the convergent pair plus an AutoGen
side-implication. Cycle-64's cross-reference to I-6 in I-L5 already
flagged the convergent pair as combinable into a 2x2 (content type
× temporal scope). Adding I-2 to the cluster suggests a 3rd axis —
opinion level — could add a third dimension. The 2x2 becomes a
2x2x2 (cycle-local-progress, cycle-local-plan, institutional-progress,
institutional-plan, each at low/high opinion). This is rich design
space; whether the full cube is useful or whether a flatter
projection suffices is a Phase 2 question.

#### Cluster C: Lifecycle operations beyond v1's implicit single-mode

Implications about explicit lifecycle operations the orchestrator
needs beyond v1's "always resume from current state" implicit mode.

- **Convergent** cycle-62 I-8 (Reset vs Resume as distinct first-class
  operations) ↔ I-L4 (time travel as append-only fork)
- **AutoGen** I-1 (de-prescription between major versions as
  first-class workflow)
- **LangGraph** I-L8 (subgraphs as graphs-as-nodes with namespace
  tracking) — composition is a lifecycle operation in the sense of
  "spawn child execution context with explicit input/output mapping"

Four implications cluster here. Together they propose a v2 lifecycle
vocabulary covering: reset (clear state, start fresh), resume
(continue from current), fork (branch into alternative exploration),
de-prescribe (remove parts in a future version), subgraph-spawn
(spawn child execution with namespace). This is significantly richer
than v1's single implicit "resume" mode.

#### Cluster D: Documentation honesty discipline

Implications about what the system declares about itself, distinguishing
implemented from aspirational, and naming non-guarantees explicitly.

- **AutoGen** I-4 (publishing what the system does NOT centrally
  guarantee)
- **AutoGen** I-7 (aspirational vs implemented as explicit documented
  distinction)
- **LangGraph** *implicit* — section 2.8 of PR #2768 frames this
  pattern as "honest implementation-vs-marketing-claims discipline."
  Cycle-64 referenced it inside I-L7 but did NOT promote it to a
  separate implication.

Three implications cluster here once we elevate the LangGraph implicit
pattern to peer status. The cluster is what cycle-64's first-pass
categorization called "AutoGen-singular" but is actually a partial
two-system convergence with LangGraph supplying the same architectural
discipline under different framing (claim qualification rather than
non-guarantee enumeration). See Finding 4 for the partial-convergent
upgrade.

#### Cluster E: State semantics and tool-result conventions

Implications about typed semantics on data crossing boundaries
(within-process state-channel boundaries; across-process
tool→orchestrator boundaries).

- **AutoGen** I-5 (uniform error-shape on tool results — the
  implication restored by Finding 1)
- **LangGraph** I-L2 (per-key reducers for shared state)

Two implications cluster here, partial-convergent (different
boundaries, similar discipline). Both push toward typed contracts.
A v2 candidate adopting both would have: (a) all tool outputs use
one envelope shape with explicit success/error flag, (b) all shared
state fields have explicit reducer annotations declaring how merges
work. The two together form a "typed boundary" architectural theme.

#### Cluster F: Tool-suite and prompt-suite stratification

Implications about stratifying the tool and prompt artifacts by
structural role (which is distinct from cluster B's content-type
stratification).

- **AutoGen** I-2 (layered architecture with opinion-gradient) — the
  unique implication that strongly fits this cluster

One implication. This is the smallest cluster but the only one
focused specifically on internal *tool-suite* organization. v2's
substantive tool suite (per `<artifact-composition>` half of the
deliverable) needs a stratification discipline; AutoGen's
Core/AgentChat/Extensions layering is the only direct evidence here.
Whether the cluster grows with future implications-mining (e.g.,
oh-my-codex once #2833 returns might foreground its own tool-suite
stratification) is open.

#### Cluster summary table

| Cluster | Theme | Implications | Convergent? | Phase 2 priority |
|---------|-------|--------------|-------------|------------------|
| A | Cycle-internal boundaries | I-3↔I-L1, I-L3, I-L6, I-L7 | 1 strong + 3 LG-singular | High — within-cycle structure is core |
| B | Cross-cycle artifact organization | I-6↔I-L5, I-2 | 1 strong + 1 AG-singular | High — persistence shape decision |
| C | Lifecycle operations | I-8↔I-L4, I-1, I-L8 | 1 strong + 1 AG-singular + 1 LG-singular | Medium — addresses v1's gap |
| D | Documentation honesty | I-4, I-7, LG-section-2.8 | 2 AG + 1 LG-implicit | Medium — drift prevention |
| E | Typed boundary semantics | I-5, I-L2 | partial-convergent | Medium — interacts with Cluster A |
| F | Tool-suite stratification | I-2 | 1 AG-singular | Medium — load-bearing for tool suite |

(I-2 appears in both Cluster B and Cluster F — opinion-gradient is
applicable to both artifacts and tools. Not a categorization error;
the implication has multiple architectural surfaces.)

### Finding 3: Within-system cross-reference matrices

Both cycle-62 and cycle-64 explicitly flagged within-system
intersections as deferred. Drawing the matrices reveals the
implications are not 8 independent items per system but a network
of reinforcing patterns.

#### AutoGen within-system matrix (cycle-62)

| | I-1 | I-2 | I-3 | I-4 | I-5 | I-6 | I-7 | I-8 |
|---|---|---|---|---|---|---|---|---|
| **I-1** de-prescription | — | weak | none | strong | weak | none | strong | weak |
| **I-2** opinion-gradient | weak | — | none | weak | strong | weak | weak | none |
| **I-3** termination predicates | none | none | — | none | none | weak | none | strong |
| **I-4** non-guarantees | strong | weak | none | — | weak | weak | strong | weak |
| **I-5** tool-result envelope | weak | strong | none | weak | — | none | none | none |
| **I-6** plan/progress split | none | weak | weak | weak | none | — | none | weak |
| **I-7** aspirational vs implemented | strong | weak | none | strong | none | none | — | weak |
| **I-8** Reset vs Resume | weak | none | strong | weak | none | weak | weak | — |

Strong intersections (within AutoGen):
- I-1 ↔ I-4 ↔ I-7: documentation discipline triangle (de-prescription,
  non-guarantees, aspirational-vs-implemented). All three are
  documentation-honesty patterns at different time scopes
  (I-1 between versions, I-4 about current capabilities, I-7 about
  future work). This is the AutoGen-side of cluster D.
- I-2 ↔ I-5: tool-suite design pair (opinion-gradient layering,
  uniform tool-result envelope). Both shape how tools are organized
  and how their outputs cross boundaries. Cluster E + F intersection.
- I-3 ↔ I-8: lifecycle operation pair (termination predicates,
  reset/resume). Both about explicit lifecycle vs implicit-default.
  AutoGen-side of cluster C.

Orphans (no strong intersection within AutoGen):
- I-6 (plan/progress split) — within AutoGen, intersects only weakly.
  Its strong intersection is *cross-system* with I-L5 (the
  convergent pair).

#### LangGraph within-system matrix (cycle-64)

| | I-L1 | I-L2 | I-L3 | I-L4 | I-L5 | I-L6 | I-L7 | I-L8 |
|---|---|---|---|---|---|---|---|---|
| **I-L1** super-step phasing | — | strong | strong | weak | none | strong | strong | weak |
| **I-L2** per-key reducers | strong | — | weak | none | weak | weak | none | none |
| **I-L3** pending writes | strong | weak | — | none | none | strong | strong | none |
| **I-L4** time travel as fork | weak | none | none | — | weak | none | none | strong |
| **I-L5** short/long memory split | none | weak | none | weak | — | none | none | weak |
| **I-L6** durability modes | strong | weak | strong | none | none | — | weak | none |
| **I-L7** restart idempotence | strong | none | strong | none | none | weak | — | none |
| **I-L8** subgraphs with namespaces | weak | none | none | strong | weak | none | none | — |

Strong intersections (within LangGraph):
- I-L1 ↔ I-L2 ↔ I-L3 ↔ I-L6 ↔ I-L7: a five-implication mesh around
  cycle-internal state semantics. Super-step phasing (I-L1) is the
  central node — it provides the boundary that reducers (I-L2)
  merge at, that pending-writes (I-L3) preserve across, that
  durability modes (I-L6) commit at, and that restart-idempotence
  (I-L7) re-executes from. Cluster A is this mesh.
- I-L4 ↔ I-L8: composition pair (fork, subgraph). Both about
  explicit alternative execution paths.

Orphans (no strong intersection within LangGraph):
- I-L5 (short/long memory split) — within LangGraph, intersects only
  weakly. Its strong intersection is *cross-system* with cycle-62
  I-6 (the convergent pair).

#### Pattern from both matrices

Each system has one "orphan" implication whose strong intersection is
cross-system rather than within-system. AutoGen's I-6 and LangGraph's
I-L5 are the cycle-64-recognized convergent pair (cluster B). This
is structurally interesting: the strong cross-system convergences
are precisely the implications that don't have strong same-system
neighbors. This makes them more elevatable to 1-research.md
cross-system observations — they are not load-bearing within their
home system's network, so elevation doesn't disrupt within-system
coherence.

The 3 cycle-64-named convergents map to:
- I-3 ↔ I-L1: I-3 has within-AutoGen strong link to I-8; I-L1 has
  within-LangGraph strong links to four others. Both are NOT orphans
  within their home systems — elevation is more delicate.
- I-6 ↔ I-L5: both are orphans (described above). Cleanest
  elevation candidate.
- I-8 ↔ I-L4: I-8 has within-AutoGen strong link to I-3; I-L4 has
  within-LangGraph strong link to I-L8. Both are mid-degree —
  elevation should preserve within-system links.

### Finding 4: Re-examination of cycle-64's "singular" labels surfaces partial convergences

Cycle-64's first-pass categorization treated some implications as
clean-singular when they are actually partial-convergent. The
synthesis cycle is the natural place to upgrade these labels.

#### Partial convergence #1: documentation honesty (Cluster D)

- AutoGen I-4 (publishing non-guarantees): explicit-non-guarantees
  enumeration. Five named non-guarantees in PR #2763 deep-dive.
- AutoGen I-7 (aspirational vs implemented): explicit
  aspirational-feature marking. "Agent paging in/out" named as
  aspirational rather than shipped.
- LangGraph section 2.8 (PR #2768): "the code does NOT resume from
  the same line of code where execution stopped" — admitted in the
  durable-execution docs themselves. Cycle-64 referenced this
  inside I-L7 but did not elevate it to a peer implication.

The three patterns share a discipline: the system tells you what
it can't or doesn't do, with mechanical/procedural markers
distinguishing claim from reality. The framings differ:
- AutoGen I-4: forward-looking ("here's what we don't guarantee")
- AutoGen I-7: feature-status-marking ("this isn't built yet")
- LangGraph 2.8: backward-looking ("our previous claim was
  technically misleading; here's the corrected framing")

Cycle-64 treated AutoGen I-4 and I-7 as singular and LangGraph 2.8
as embedded-context inside I-L7. This synthesis upgrades the cluster:
all three are partial convergents on the same architectural
discipline (documentation honesty) at different time scopes.

**Categorization upgrade:** cluster D is a 2-system *partial*
convergence (with LangGraph evidence requiring extraction from
section 2.8 rather than being already-elevated as an implication).

**Implication for elevation.** The cluster D pattern is weaker than
the 3 strong convergents in cycle-64 — LangGraph's evidence is
implicit-from-systems-file rather than explicit-as-implication. But
the cluster is real. A future implications-mining cycle on LangGraph
could write a separate I-L9 (documentation honesty discipline) that
elevates section 2.8 to peer-implication status; that would convert
the partial convergence to a strong convergence.

#### Partial convergence #2: typed boundary semantics (Cluster E)

- AutoGen I-5 (uniform tool-result envelope): cross-process boundary
  with explicit error flag. Cycle-64 dropped this from
  categorization (Finding 1).
- LangGraph I-L2 (per-key reducers): within-process boundary with
  explicit merge rules.

Both push toward typed contracts on data boundaries; the boundaries
differ. This is partial-convergent (similar discipline, different
substrate). Restoring I-5 from Finding 1 makes this cluster
explicit.

**Categorization upgrade:** I-5 is reclassified from
"missed-from-categorization" to "partial-convergent with I-L2."

#### Partial convergence #3: explicit composition with structural identifiers (sub-pattern of Cluster C)

- AutoGen I-2 (layered architecture with opinion-gradient): named
  layers (Core / AgentChat / Extensions / Studio / Bench) with
  load-bearing opinion semantics.
- LangGraph I-L8 (subgraphs as graphs-as-nodes with namespace
  tracking): subgraph namespaces (`checkpoint_ns`: `""` for root,
  `"node_name:uuid"` for subgraphs, nested joined by `|`).

Both are about composition mechanisms with structural identifiers
that survive across the artifact boundary. AutoGen's layers have
named module identities; LangGraph's subgraphs have namespace
identifiers. The patterns are NOT identical — AutoGen is about
opinion-stratified module layers; LangGraph is about
parent/child execution-tree namespacing — but they share the
discipline of "compose with explicit named structure, not flat".

**Categorization upgrade:** I-2 and I-L8 are weakly partial-convergent.
This is a less-strong upgrade than the previous two; flagging here
for completeness.

#### Updated mapping after Findings 1 + 4

Of 16 formal implications across the two docs (AutoGen I-1..I-8;
LangGraph I-L1..I-L8):

| Status | Implications | AutoGen-side | LangGraph-side |
|---|---|---|---|
| Strong convergent (cycle-64-recognized) | 3 pairs | I-3, I-6, I-8 | I-L1, I-L5, I-L4 |
| Partial convergent (synthesis-recognized) | 3 pairs | I-2, I-4, I-5, I-7 | I-L2, I-L8 (+ LG-2.8 implicit, not a formal implication) |
| Pure AutoGen-singular | 1 | I-1 | — |
| Pure LangGraph-singular | 3 | — | I-L3, I-L6, I-L7 |

Counts check: 6 strong-pair implications + 6 partial-pair implications
(4 AutoGen + 2 LangGraph formal) + 1 AutoGen-pure + 3 LangGraph-pure
= 16 formal implications. The "pure-singular" set (4 implications) is
much smaller than cycle-64's first-pass categorization (4 + 5 = 9)
suggested — re-reading surfaces partial cross-system signals on five
of the nine first-pass-singular implications.

### Finding 5: Elevation candidates for 1-research.md

The 3 strong-convergent pairs are candidates for elevation to
1-research.md cross-system observations. Per cycle-64's
recommendation, actual elevation is deferred to a future
cross-system synthesis cycle (option 5 from #2829). This synthesis
drafts the elevation form forward so the future cycle can
copy-edit-and-promote rather than draft-from-scratch.

**Elevation discipline.** 1-research.md uses `[N+/N systems]`
maturity badges. The convergent implications are 2-system patterns,
which would be `[2-system clean]` or `[2-system strict + diversity
hedge]` per the format. The diversity-hedge applies here because
AutoGen and LangGraph are both Python/TypeScript libraries with
in-process state — limited substrate diversity. The hedge text
should name the substrate-diversity limit explicitly.

**Elevation target families.** The three convergents map to:
- I-3 ↔ I-L1: phase-boundary with state-write semantics → likely
  Family A (orchestration patterns) or Family C (state, memory,
  history) depending on framing
- I-6 ↔ I-L5: artifact stratification → Family C (state, memory,
  history)
- I-8 ↔ I-L4: lifecycle operations → currently no exact family
  match; might warrant Family F (lifecycle operations) addition or
  fit under Family A (orchestration) loosely

#### Draft elevation: I-3 ↔ I-L1 (phase-boundary state semantics)

**Phase boundaries with explicit state-write semantics within a
cycle.** **`[2-system strict + diversity hedge]`** AutoGen frames
termination as composable callable predicates (`MaxMessageTermination`,
`TextMentionTermination`, `TokenUsageTermination`, etc.) combinable
with AND/OR; graceful termination lets the current agent's turn
finish before team stop ("keeping the team's state consistent");
immediate is exception-based. LangGraph frames cycle composition as
Pregel/BSP super-steps where parallel nodes within a super-step do
not observe each other's writes until the super-step ends; reducers
merge at super-step boundary. Both push toward explicit cycle-
internal boundaries with state-write semantics distinguishing
graceful-and-state-consistent from immediate-and-state-suspect.
**Diversity hedge:** both systems are Python/.NET (AutoGen) and
Python/TypeScript (LangGraph) in-process libraries; the
within-process boundary mechanism may not map cleanly to cross-
process substrates. Repo-internal: cycle-65 cluster A maps four
LangGraph singular implications (I-L3 pending writes, I-L6
durability modes, I-L7 restart idempotence) and the AutoGen I-3
together; phase boundaries are the central architectural primitive
for those four.

#### Draft elevation: I-6 ↔ I-L5 (artifact stratification by content type and temporal scope)

**Persistence artifacts stratified by content type and temporal
scope.** **`[2-system clean]`** AutoGen's `MagenticOneGroupChat`
maintains distinct Task Ledger (planning artifact) and Progress
Ledger (tracking artifact) rather than collapsing both into one log.
LangGraph's persistence docs explicitly motivate the split between
short-term thread-scoped checkpoints and long-term cross-thread
`Store` interface: "with checkpointers alone, we cannot share
information across threads." Both push toward explicit artifact
stratification rather than a single collapsed log holding all
content kinds at all temporal scopes. The two splits are along
different axes (content type vs temporal scope) and combine into a
2x2 (cycle-local-plan, cycle-local-progress, institutional-plan,
institutional-progress). Repo-internal: v1's journal and `_notes/`
collapse multiple kinds; cycles 60-64 surfaced this as a structural
issue (cycle-59 row-ordering, cycle-61 hypothesis-counting,
cycle-63 stale-reference findings all involve implicit-merge silently
going wrong across collapsed artifacts).

#### Draft elevation: I-8 ↔ I-L4 (lifecycle operations beyond resume)

**Explicit lifecycle operations beyond implicit "always resume from
current state."** **`[2-system clean]`** AutoGen provides distinct
`reset` (clear state, start fresh) and `resume` (continue from
current) operations on team objects, with component-local-state
architecture making the distinction clean (resetting a team is a
composition of component resets). LangGraph provides `replay`
(retry from a prior checkpoint) and `fork` (branch from a prior
checkpoint with modified state) as time-travel operations: "the
original execution history remains intact." Both push toward
explicit lifecycle-operation vocabularies beyond v1's implicit
single-mode "always resume from current cycle state." Combined
vocabulary: reset, resume, replay, fork, plus AutoGen I-1's
de-prescribe (between major versions) and LangGraph I-L8's
subgraph-spawn (composed child execution with namespaces). The
v2 lifecycle vocabulary is significantly richer than v1's. Phase 2
candidates should declare which subset they adopt and the
corresponding mechanisms. Repo-internal: cycle 524's corruption
class (issue [#2638](https://github.com/EvaLok/schema-org-json-ld/issues/2638))
is precisely the failure mode where lifecycle vocabulary matters —
no clean reset path beyond Eva manually editing files.

### Finding 6: Phase 2 design-input clusters

The clusters from Finding 2 are not just descriptive groupings —
they suggest natural Phase 2 candidate compositions. Phase 2
candidates can adopt clusters wholesale, partially, or in
combinations.

#### Composable cluster combinations

Some clusters are independent and can be mixed-and-matched:
- Cluster D (documentation honesty) is mostly independent of A/B/C
  — a candidate can adopt the honesty discipline regardless of
  cycle-internal phasing or lifecycle vocabulary choices.
- Cluster E (typed boundary semantics) is mostly independent of A/B
  — typed boundaries help any architectural shape but don't dictate
  one.
- Cluster F (tool-suite stratification) is mostly independent of
  A/B/C — opinion-gradient layering applies to any tool suite.

Some clusters interact:
- Cluster A (cycle-internal phasing) ↔ Cluster B (artifact
  stratification): if a candidate adopts phase boundaries (A), the
  artifacts written at each phase boundary need clear semantics
  (B's content-type splits). The two together prescribe a richer
  shape than either alone.
- Cluster A (cycle-internal phasing) ↔ Cluster E (typed boundary
  semantics): phase boundaries are a kind of typed boundary; per-key
  reducers (I-L2) merge at phase boundary (I-L1); pending writes
  (I-L3) and durability modes (I-L6) are about phase-boundary
  semantics. Adopting cluster A almost forces adopting cluster E for
  the boundary semantics to be explicit.
- Cluster C (lifecycle operations) ↔ Cluster B (artifact
  stratification): fork (I-L4) operations need to fork artifact sets
  (B); reset (I-8) operations need to know which artifacts to clear
  (B). Adopting cluster C without B leaves the lifecycle operations
  underspecified.

#### Independent vs entangled implications

Implications independent across clusters (can be adopted singly):
- I-1 (de-prescription between versions) — purely an artifact-shape
  decision
- I-2 (opinion-gradient layering) — applies to any tool suite

Implications entangled with clusters (can't be cleanly adopted
without the rest of the cluster):
- I-L1 (super-step phasing) — entangles with I-L2/I-L3/I-L6/I-L7
- I-L7 (restart idempotence) — entangles with I-L1 and the runtime
  semantics
- I-3 (termination predicates) — entangles with I-L1 (the same
  pattern from a different framing)

#### Candidate composition recipes

Three candidate compositions worth considering for Phase 2:

**Recipe 1: Phase-boundary discipline + typed semantics + artifact
stratification.** Adopts clusters A + B + E. Heaviest. v2 cycles
have explicit phases with state-write semantics; artifacts are
stratified by content type and temporal scope; tool outputs and
state writes are typed. Lifecycle is still implicit (reset by
manual Eva intervention). This is "make the within-cycle structure
explicit" candidate.

**Recipe 2: Lifecycle vocabulary + documentation honesty + tool-
suite stratification.** Adopts clusters C + D + F. Heaviest on
cross-cycle structure; cycle-internal stays similar to v1. Reset
+ resume + replay + fork + de-prescribe + subgraph as explicit
operations; v2 documents non-guarantees and aspirational features
mechanically; tools are stratified by opinion level. This is "make
the cross-cycle structure explicit" candidate.

**Recipe 3: Minimal — typed boundaries + documentation honesty
only.** Adopts clusters D + E. Lightest. v2 prompt is small (per
`<core-design-principle>`); tool outputs are typed; non-guarantees
are documented; aspirational features are tagged. Within-cycle
phasing and lifecycle vocabulary stay v1-like. This is "minimal
delta from v1, maximum legibility gain" candidate.

These recipes are NOT Phase 2 candidates themselves — they are
*input* for Phase 2 candidate generation. Each recipe could be
expanded into a fuller candidate by adding a state-representation
choice, a multi-agent decomposition stance, etc. The recipes
illustrate that the implications-mining doesn't force a single
candidate shape — it exposes design choices.

## Methodological observations after two implications-mining cycles

Two instances now (AutoGen cycle 62, LangGraph cycle 64). Plus this
synthesis cycle. Some observations about the methodology itself:

**Implications-mining cadence is viable.** Each mining cycle fits in
~75 minutes of compute, produces a single doc of moderate density
(~8 implications, ~500 lines), and adds material that's additive to
the cross-system synthesis already in `1-research.md`. Cycle 62
established the template; cycle 64 confirmed the shape works at
second instance. The cadence sustains.

**First-pass categorization tends to drop borderline cases.** Cycle
64 dropped cycle-62 I-5 from its convergent + LangGraph-singular +
AutoGen-singular split (Finding 1). This is a methodology hazard —
the strong-convergent vs clean-singular framing has a borderline
where partial-convergents fall between the categories and get
dropped. Synthesis cycles are the natural correction point. Future
implications-mining cycles should expect their categorization to
be revised during synthesis.

**Singular-label is provisional.** Cycle-64's "singular" implications
became partial-convergents under stricter cross-reading (Finding 4).
This means the 3+5+4 first-pass split is an upper bound on the
"clean-singular" count and a lower bound on the "convergent" count.
A synthesis cycle's revised count (3 strong + 3 partial-convergent +
4 pure-singular = 10 partially-or-fully cross-system + 4 pure-
singular) is a more accurate cross-system signal.

**Within-system intersections matter.** Cycle 62 and cycle 64 each
had ~5 strong intra-system intersections per their 8 implications
(Finding 3). The implications are not independent items — they form
networks with central nodes (AutoGen I-3/I-4 high-degree;
LangGraph I-L1 highest-degree). Phase 2 candidates that adopt one
implication often inherit several through the network. Synthesis
cycles should make these networks explicit.

**Synthesis is faster than mining.** This document is ~600 lines
covering both AutoGen and LangGraph; the mining cycles each took a
full session for one system. Synthesis is a natural rhythm-break
between mining cycles — cheaper, structurally additive, and
self-correcting (catches first-pass errors).

**Synthesis as second-instance pattern.** Per cycle-64's
methodological observation about implications-mining as
second-instance pattern: cycle 65 is the first-instance synthesis
cycle. If a future cycle does another synthesis (e.g., after Cognition
Devin or openclaw mining adds a third system to the corpus), that
will confirm synthesis as a second-instance pattern. The expected
shape: each new mining cycle produces ~8 implications; synthesis
cycles every 2-3 mining cycles to recategorize, surface clusters,
and update elevation candidates.

**Recommendation for future cycles.** Maintain a 2-3:1 mining-to-
synthesis ratio. After mining a third system (Cognition Devin or
openclaw, both viable), do a second synthesis revising the cluster
table with the new evidence. After mining a fourth or fifth, the
cluster structure should stabilize and elevation cycles can begin
formalizing the strong-convergents into 1-research.md.

## What this informs

This synthesis adds to the v2 design-input pool at the synthesis layer
rather than at the per-system layer. Specifically:

- **Cluster table** (Finding 2) gives Phase 2 candidate authors a
  thematic groupings of implications, alternative to per-system
  reading.
- **Within-system matrices** (Finding 3) give Phase 2 candidate
  authors the network structure of implications — which combinations
  naturally fit, which are central, which are orphans.
- **Updated convergent count** (Finding 4) gives a more accurate
  cross-system signal than cycle-64's first-pass split (3 strong + 3
  partial vs 3 + 9 singular).
- **Elevation drafts** (Finding 5) give a future cross-system-
  synthesis cycle copy-edit-able starting points for elevating
  convergents into 1-research.md.
- **Composition recipes** (Finding 6) give Phase 2 candidate authors
  three concrete starting points illustrating cluster combinations.

None of these are *commitments* — Phase 2 candidates will choose
which to adopt, which to reject, which to combine.

## What remains open

- The synthesis covers two systems. Cognition Devin, OpenAI
  harness-engineering, openclaw, Voyager, oh-my-codex (post-#2833)
  are remaining deep-dive systems with implications not yet mined.
  Each will likely shift the cluster structure when added.
- Cluster F (tool-suite stratification) has only one implication
  (AutoGen I-2). Whether the cluster grows or remains a singleton is
  an open question pending future mining (oh-my-codex's
  deterministic-vs-LLM separation may foreground tool-suite
  stratification when its dispatch returns).
- Cluster D (documentation honesty) is partial-convergent on
  LangGraph evidence that's *implicit-in-systems-file* rather than
  explicit-as-implication. A future LangGraph implications-mining
  cycle could write an I-L9 (documentation honesty discipline)
  explicitly, converting the partial convergence to a strong
  convergence. This is methodologically interesting — the synthesis
  cycle surfaces what mining missed, then mining can address the gap.
- Elevation drafts (Finding 5) are first-pass against the
  1-research.md Family format. Whether the strong-convergents fit
  cleanly into Family A (orchestration), Family C (state/memory/
  history), or warrant a new Family F (lifecycle operations) is a
  future-elevation-cycle decision.
- The cluster table (Finding 2) treats clusters as orthogonal but
  Finding 6 shows they interact in non-trivial ways. A more rigorous
  treatment would map cluster interactions formally (a cluster-
  interaction matrix). Out of scope for this cycle.
- The 2-3:1 mining-to-synthesis ratio recommendation is a guess
  based on first-instance synthesis. May need revision after a
  second synthesis cycle.

## Cycle accounting note

This is the FIRST synthesis cycle in the v1.X+ sequence (cycles 35-64
were 27 cold-reader + 1 dispatch-construction + 2 implications-mining
+ 1 synthesis = cycle-65). It is the FOURTH consecutive cycle of
research-corpus advancement under [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829)'s
polarity inversion (cycle 62 = AutoGen implications; cycle 63 =
oh-my-codex deeper-read dispatch construction; cycle 64 = LangGraph
implications + bounded-mechanical housekeeping; cycle 65 =
cross-implications synthesis). Four-cycle robustness on the polarity-
pivot pattern is now established with three distinct cycle composition
shapes (mining, dispatch-construction, synthesis) all viable as
substantive focal under #2829.

The synthesis cycle composition shape adds to the cycle-composition
vocabulary alongside mining, dispatch-construction, per-finding-
evaluation, framework-iteration (cold-reader), and bounded-
mechanical. Cycle 62 surfaced the meta-observation that "v2 should
have explicit cycle composition guidance, not implicit-by-prompt-
default." Cycle 65 sustains that observation: synthesis is a distinct
cycle composition shape from mining and from per-finding-evaluation;
v2's prompt could declare cycle-composition tags explicitly at
session start.

**Provisional read for cycle 66.** Three candidates in priority
order:
1. **If cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
   deliverable returns by cycle 66:** per-finding evaluation
   absorption cycle (parallel to cycle-41 cognition / cycle-43
   openclaw absorption pattern). Highest priority because the
   dispatch was costly to construct and value compounds when
   integrated rapidly.
2. **Otherwise, implications mining on a third deep-dive system
   (option 4 continuation):** Cognition Devin (lowest cross-system
   citation in `1-research.md` so largest singular-voice surface) or
   openclaw (also significant singular-voice surface — anti-pattern
   catalog framing) — pick one. Adding a third system's implications
   to the corpus revises the cluster table and may convert the
   partial-convergent cluster D to a strong-convergent if Cognition
   or openclaw foregrounds documentation honesty.
3. **Otherwise (least likely given options 1-2 robust availability):**
   v1-system retrospective mining (option 3 from #2829) — read
   `0-retrospective.md` against the implications corpus and surface
   what implications address which v1 F-patterns. This produces an
   implications-to-F-pattern mapping useful for Phase 2 candidate
   evaluation against retrospective evidence.

Cold-reader remains fallback only and is not anticipated to be in
play given the substantive options' robust availability.
