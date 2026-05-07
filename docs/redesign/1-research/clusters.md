# Implications-mining clusters (cycles 62-74)

This file holds the deeper-pass implications-mining catalogue and
cross-cluster intersection patterns. It is the load-bearing Phase 2
evidence-base alongside the Family-level cross-system observations in
[`../1-research.md`](../1-research.md). Migration recorded in
[`../_notes/cycle-73-cluster-restructure.md`](../_notes/cycle-73-cluster-restructure.md)
per the redesign-prompt's `<evolve-the-mechanism>` mandate.

The Family-level observations in [`../1-research.md`](../1-research.md)
record cross-system patterns from the first-pass per-system reads
(cycles 14-32). A deeper-pass over the same six deep-dive systems ran
across cycles 62-69 under the polarity inversion of
[#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829),
mining ~10 per-system implications per cycle and clustering them by
architectural concern. The mining produced 45 implications across
9 architectural clusters across 6 systems. The per-cycle docs live
under [`../_notes/cycle-62-autogen-implications.md`](../_notes/cycle-62-autogen-implications.md)
through [`../_notes/cycle-69-voyager-implications.md`](../_notes/cycle-69-voyager-implications.md);
a first synthesis at
[`../_notes/cycle-65-cross-implications-synthesis.md`](../_notes/cycle-65-cross-implications-synthesis.md)
covered the AutoGen + LangGraph layer; the cross-cluster intersections
synthesis lives at
[`../_notes/cycle-72-cross-cluster-intersections.md`](../_notes/cycle-72-cross-cluster-intersections.md).
This file is the elevation to a permanent reference that cycles
65/66/67/68/69 deferred to a future synthesis cycle (now cycle 70),
extended by cycle 72's cross-cluster intersections synthesis, and
migrated to its own file at cycle 73.

The clusters are a deeper layer than the Families: Families are
named pattern themes; clusters are named *architectural concerns*
each containing multiple sub-shapes (different mechanisms within the
same concern). The relationship between clusters and Families is
not 1:1 — many clusters intersect Family patterns; some clusters
(notably H, I) surface concerns that the Family layer didn't
foreground. The clusters add (a) sub-shape granularity the Families
don't surface, (b) the per-system implications evidence base used to
evaluate Phase 2 candidate variations, and (c) sub-axis catalogues
for stratification patterns Phase 2 candidates can compose against.

The six systems mined: AutoGen
([cycle 62](../_notes/cycle-62-autogen-implications.md))
+ LangGraph ([cycle 64](../_notes/cycle-64-langgraph-implications.md))
+ Cognition Devin ([cycle 66](../_notes/cycle-66-cognition-devin-implications.md))
+ openclaw ([cycle 67](../_notes/cycle-67-openclaw-implications.md))
+ OpenAI harness-engineering ([cycle 68](../_notes/cycle-68-openai-harness-implications.md))
+ Voyager ([cycle 69](../_notes/cycle-69-voyager-implications.md)).
Substrate diversity at maximum across five orthogonal axes:
library/product/research-artifact, cloud/local, enterprise/personal/research,
software-development/game-environment, external-publishable/internal-tooling.
Patterns surviving 6-system cross-substrate convergence are robustly
transferable.

## Cluster table (post cycle 69)

| Cluster | Theme | Depth | Implications | Sub-shapes/sub-axes |
|---|---|---|---|---|
| A | Cycle-internal boundaries with state-write semantics | 6-system clean | 11 | 9 sub-shapes |
| B | Cross-cycle artifact organization | 6-system clean | 10 | 9 sub-shapes |
| D | Documentation honesty | 5-system clean (+ Voyager partial) | 11 | 9 sub-shapes |
| F | Tool-suite stratification (multi-axis) | 5-system convergent | 8 | 8 sub-axes |
| H | Post-session feedback / cross-session learning | 4-system convergent | 4 | 4 sub-shapes |
| C | Lifecycle operations beyond resume | 4-system clean | 6 | 5 sub-shapes |
| E | Typed boundary semantics | 3-system convergent | 3 | 2 sub-shapes |
| G | Role-asymmetric context | 2-system convergent | 2 | 2 sub-shapes |
| I | Harness-enforced security/policy boundaries | 2-system convergent (substrate-correlated) | 3 | 2 sub-shapes |

Clusters A, B, D are tied as the most-foregrounded clusters in the
corpus (5+ system clean depth). Cluster F is the multi-axis-king
(8 sub-axes — largest single-cycle sub-shape growth occurred when
Voyager was mined cycle 69, +4 sub-axes). Cluster I is at lower
2-system convergent depth but is *substrate-correlated* to v1's
own substrate (cloud-anchored multi-actor with audit) — Phase 2
candidates SHOULD weight cluster I patterns highly despite the low
depth count.

**v1-substrate instantiation annotation work begun cycle 86
(audit#454 M1 absorption); M2 self-management cost annotations
begun cycle 87; M2 retro for cluster A and M3 v1 strengths layer
begun cycle 88.** Cluster A received full substrate-fit
annotations cycle 86 as proof-of-format using cycle 85's D3
sub-cluster grouping scaffold; cluster A M2 retro-annotations
landed cycle 88 (4 LOW + 5 MODERATE + 0 HIGH). Cluster B received
substrate-fit + self-management cost annotations cycle 87 (M1 + M2
layers interleaved per sub-shape). Cluster D received M1 + M2
annotations cycle 88 (2 STRONG + 6 PARTIAL + 1 ABSENT; 4 LOW +
5 MODERATE + 0 HIGH — most substrate-aligned cluster annotated
to date). Cluster F received M1 + M2 annotations cycle 88 (0
STRONG + 3 PARTIAL + 5 ABSENT; 2 LOW + 4 MODERATE + 2 HIGH —
least substrate-aligned cluster annotated to date). M3 v1
strengths layer added cycle 88 with framing + 3 distinct v1
strengths surfaced from STRONG sub-shapes (process-isolation
A↔B dual-cast; anti-patterns published; walkback as artifact).
Cycle 89 extends the annotation to clusters C / E / G / H / I per
cycle 85's M1-M5 multi-cycle plan, plus M2 retro for clusters
B/D/F if not already present. Annotation format (substrate-fit
STRONG / PARTIAL / ABSENT + self-management cost LOW / MODERATE /
HIGH + current v1 instantiation + design work needed) and Phase 2
evaluation use are documented in the per-cluster subsections
below; M3 layer documented as standalone cluster-spanning
subsection between clusters B and D.

## Cluster A: cycle-internal boundaries with state-write semantics

`[6-system clean]` AutoGen + LangGraph + Cognition + openclaw +
OpenAI harness + Voyager.

The orchestrator's cycle has internal boundaries across which
state-writes must respect explicit semantics. The principle —
cycle-internal phasing has explicit semantics rather than implicit
sequencing — is shared across all six systems; the mechanisms
diverge widely. Sub-shapes (9) include super-step semantics with
per-channel reducer rules (LangGraph I-L1), phase-boundary state
semantics (AutoGen I-3 + LangGraph I-L1 cross-system convergent,
elevated cycle 65), termination predicates as cycle-internal phase
delimiters (AutoGen I-3), per-key reducers with explicit merge
rules (LangGraph I-L2), lane-aware FIFO queue with per-lane
concurrency caps and per-session serialization (openclaw I-O3),
stuck-session watchdog as recovery-without-abort lifecycle
operation (openclaw I-O5), sync invariants asserted at session
init for dual-storage components with fail-fast remediation hint
(Voyager I-V4), bounded retries with critic-feedback fed forward
into next-attempt prompt (Voyager I-V7), and (cycle 68) process-
isolation discipline at session level via ephemeral worktrees
(OpenAI harness).

**Sub-cluster groupings (cycle 85 audit#454 D3 absorption).** The 9
sub-shapes partition into four conceptually distinct sub-cluster
groupings:

- **Phase-boundary semantics** (4 sub-shapes): super-step semantics,
  phase-boundary state semantics, termination predicates, per-key
  reducers
- **Recovery operations** (3 sub-shapes): stuck-session watchdog,
  sync invariants at init, bounded retries with feedback
- **Concurrency / queuing** (1 sub-shape): lane-aware FIFO with
  per-session serialization
- **Process isolation** (1 sub-shape): ephemeral worktrees

Distribution (4/3/1/1) signals: phase-boundary is structurally
well-attested, recovery is well-attested, but concurrency and
process-isolation are 1-sub-shape thin within cluster A. Future
systems may elevate these to their own sub-cluster groupings or
distinct clusters as corpus depth supports it. Phase 2 candidate
evaluation should check coverage at the sub-cluster grouping level,
not just at the cluster A level — a candidate covering only
phase-boundary semantics is not equivalent to a candidate covering
all four sub-cluster groupings.

**v1-substrate instantiation (M1, cycle 86 audit#454 absorption) +
self-management cost (M2, cycle 88 audit#454 absorption — retro
annotation alongside M1 cluster D + F work).** Each of the 9
sub-shapes is annotated with how its mechanism maps to v1's
substrate (GitHub Actions runner / Rust tools / Claude Code session
/ cron trigger / repository-as-state / GitHub issue tracker) +
self-management cost (LOW / MODERATE / HIGH). Annotation format:
substrate-fit (STRONG / PARTIAL / ABSENT) + current v1
instantiation if any + design work needed for Phase 2 candidates
that adopt the sub-shape + self-management cost. Phase 2 candidate
evaluation uses these annotations to weight implementation effort
by ABSENT count and to identify mechanisms that v1's substrate
provides "for free" (STRONG) versus those requiring substantial
substrate-design work; M2 layer additionally weights per-cycle
decision overhead. Cluster A M2 grades use the same LOW / MODERATE
/ HIGH scale as cluster B (see cluster B M2 framing for definition).

*Phase-boundary semantics sub-shapes:*

1. **Super-step semantics with per-channel reducer rules
   (LangGraph I-L1)** — substrate-fit PARTIAL. v1's cron-triggered
   cycle is implicit super-step (cycle issue lifecycle as the
   boundary), but per-channel reducer rules are absent (multiple
   file writes in the same cycle silently overwrite, no per-state-
   surface declared reducer). Design work: explicit phase boundaries
   within a cycle (Phase 0 read state / Phase 1 substantive work /
   Phase 2 write artifacts / Phase 3 post comments) + per-state-
   surface reducer rules declared in a registry. Substrate via Rust
   tool `enforce-phase-boundary` invoked at session start and at
   phase transitions. **Self-management cost: MODERATE.** Reducer
   registry needs maintenance as state surfaces are added or
   retired; per-channel routing adds minor decision cost per cycle;
   phase-boundary discipline becomes self-enforcing once tool is in
   place.

2. **Phase-boundary state semantics (AutoGen I-3 + LangGraph
   I-L1)** — substrate-fit ABSENT. v1's cycle has implicit phase
   boundaries but no declarative state-write semantics at each
   boundary (no "at this phase boundary, these state writes have
   happened or must happen"). Design work: typed phase-state
   machine (`cycle-state.json` with phase tag + transition guards).
   Substrate via Rust tool `cycle-state-machine` embedded in
   cycle-runner harness; transitions emit state-write semantics
   declared per phase. **Self-management cost: MODERATE.** State
   machine schema evolves as new phases are added; transition guards
   need maintenance; once schema is stable, per-cycle cost is low
   (substrate enforces).

3. **Termination predicates as cycle-internal phase delimiters
   (AutoGen I-3)** — substrate-fit ABSENT. v1 terminates on
   wall-clock (75-min session timeout) or orchestrator-judgment
   (close cycle issue), not predicate-driven per-phase termination.
   Design work: declarative termination predicates per phase (e.g.,
   "Phase 1 complete when journal-entry-written AND
   cycle-end-comment-posted"). Substrate via Rust tool
   `phase-termination-check` that runs at phase boundaries; exits
   non-zero if predicates fail (escalates via question-for-eva).
   **Self-management cost: LOW.** Predicates declared per phase;
   substrate enforces; predicate set evolves slowly. No per-cycle
   decision overhead beyond the standard phase-completion check.

4. **Per-key reducers with explicit merge rules (LangGraph
   I-L2)** — substrate-fit PARTIAL. v1 has implicit reducers
   (journal entries append; clusters.md uses last-write-wins;
   issue comments append) but no declared per-state-surface
   reducer registry — multiple writes silently overwrite when
   intent was merge-or-append. Design work: state surface registry
   declaring reducer per surface (append / replace / merge-keys).
   Substrate via Rust tool `apply-state-update` that takes
   (surface, update) and applies the declared reducer; files
   declare reducer via header comment or central registry file.
   **Self-management cost: MODERATE.** State surface registry needs
   maintenance as surfaces are added; per-surface reducer choice is
   one-time per surface but write-routing per cycle adds minor
   decision cost. Boundary with cluster B sub-shape 2 short/long
   split (state-surface-registry) — same substrate; same cost level.

*Recovery operations sub-shapes:*

5. **Stuck-session watchdog (openclaw I-O5)** — substrate-fit
   ABSENT. v1's cycle-runner has wall-clock session timeout but
   no detection of "previous cycle abandoned mid-flight" (no
   journal entry written, no end-of-cycle comment posted,
   dispatched issue without acknowledgment). The cycle 71
   stuck-dispatch incident is the canonical example: dispatch was
   filed but the orchestrator-bot couldn't self-fix the malformed
   assignment, and v1 had no detection mechanism so the dispatch
   sat 8+ cycles before diagnosis. Design work: Rust tool
   `detect-abandoned-cycles` runs at session start, checks recent
   cycle issues for incomplete artifacts and outputs structured
   remediation hints. Optional integration with cycle-runner
   harness for automatic recovery or escalation to question-for-
   eva. **Self-management cost: LOW.** Watchdog runs deterministically
   at session start; orchestrator only acts on remediation hints.
   The watchdog itself is one-time tool design; no per-cycle
   maintenance overhead.

6. **Sync invariants at init (Voyager I-V4)** — substrate-fit
   ABSENT. v1 session init reads recent journal entries and the
   prompt but doesn't validate cross-storage consistency. The
   stale-reference accumulation in 1-research.md (cycles 30-50,
   cleanup work in cycles 60-61) is the canonical example. Design
   work: Rust tool `state-sync-check` (already named in v1-failure-
   mode mapping) validates clusters.md cross-references; 1-research.md
   system list matches systems/ subdirectory; cycle issue label
   state coherent. Outputs structured remediation text on
   divergence; exits non-zero on critical divergence to halt cycle
   entry. **Self-management cost: LOW.** Sync-check runs
   automatically at session start; orchestrator only acts on
   divergence reports; invariant set evolves slowly.

7. **Bounded retries with critic-feedback fed forward (Voyager
   I-V7)** — substrate-fit ABSENT. v1's dispatch failures (Copilot
   assignment delays; malformed dispatches) require orchestrator-
   decides-per-cycle judgment. The 4 dispatches awaiting Copilot
   assignment for 22+ cycles (#2833, #2842, #2847, #2851) are the
   canonical example. Design work: Rust tool `dispatch-with-retry`
   (already named in v1-failure-mode mapping) wrapping `gh issue
   create` with max-retries semantic; prior-attempt context
   preserved in retry payload; structured failure diagnostic on
   max-retries-exceeded. Each retry includes critique context from
   prior attempt fed forward into the next-attempt issue body.
   **Self-management cost: MODERATE.** Critic-feedback prompts need
   maintenance per failure class as new failure classes emerge;
   retry semantics need calibration (max-retries, back-off, what
   counts as failure for retry purposes). Per-cycle cost is low
   (tool runs retry); design-time and evolution cost is moderate.

*Concurrency / queuing sub-shape:*

8. **Lane-aware FIFO with per-lane concurrency caps and
   per-session serialization (openclaw I-O3)** — substrate-fit
   ABSENT. v1's cycle is serial: one cron tick per ~6 hours, one
   orchestrator session per tick. There's no notion of multiple
   parallel lanes — substantive / absorption / housekeeping work
   shapes are sequential within a single cycle. Design work:
   substrate change is non-trivial. Three options for Phase 2
   candidates: (a) multiple parallel cron schedules with
   lane-tagged outputs; (b) single cycle spawns parallel tool
   invocations as lanes (Claude Code session invokes multiple
   tools each acting as a parallel lane); (c) issue-tracker-as-
   queue with explicit lane labels and per-lane stuck-watchdog.
   Most v1-substrate-aligned: option (c) with Rust tool
   `lane-queue-status` reading issue labels — preserves
   repository-as-state without architectural shift.
   **Self-management cost: MODERATE.** Lane assignment requires
   orchestrator decision per work-unit (which lane does this work
   belong to?); lane-queue maintenance involves issue-label
   discipline; per-lane stuck-watchdog adds substrate complexity.
   Most cost is concentrated at work-unit creation (one-time per
   work-unit), not per-cycle baseline.

*Process isolation sub-shape:*

9. **Process-isolation discipline at session level via ephemeral
   worktrees (OpenAI harness, cycle 68)** — substrate-fit STRONG.
   GitHub Actions runners ARE ephemeral worktrees: each cron-
   triggered cycle runs in a fresh runner with the repo cloned
   fresh; no state carries across runners except through the
   repository (commits) and external systems (issues, labels,
   comments). v1 inherits this property "for free" from the
   substrate. Design implication: Phase 2 candidates that preserve
   "cycle = isolated process" inherit this without effort;
   candidates that deviate (e.g., persistent worker process) lose
   the property and must re-derive it via different mechanism
   (Docker container per cycle; in-memory sandbox reset).
   **Self-management cost: LOW.** Substrate handles process
   isolation; orchestrator pays no cost. Cross-references the
   cluster B sub-shape 5 repository-as-state STRONG annotation —
   same v1 substrate property dual-cast across cluster lenses (per
   audit#454 D2 dual-cast classification).

**Substrate-fit summary across cluster A's 9 sub-shapes:**
- **STRONG (1):** process-isolation via ephemeral worktrees
- **PARTIAL (2):** super-step semantics, per-key reducers
- **ABSENT (6):** phase-boundary state semantics, termination
  predicates, stuck-session watchdog, sync invariants at init,
  bounded retries with feedback, lane-aware FIFO

The 6 ABSENT sub-shapes constitute the bulk of cluster A's
substrate-design work for Phase 2 candidates. The 2 PARTIAL
sub-shapes need design work but build on existing v1 patterns
(implicit super-step boundary, implicit per-file reducer
behavior). The 1 STRONG sub-shape (process-isolation) is
inherited from the substrate; Phase 2 candidates should not
assume it requires implementation.

**Self-management cost summary across cluster A's 9 sub-shapes
(M2 retro, cycle 88):**
- **LOW (4):** termination predicates (3), stuck-session watchdog
  (5), sync invariants at init (6), process-isolation (9)
- **MODERATE (5):** super-step semantics (1), phase-boundary state
  semantics (2), per-key reducers (4), bounded retries with feedback
  (7), lane-aware FIFO (8)
- **HIGH (0):** none

Cluster A self-management cost distribution (4 LOW + 5 MODERATE
+ 0 HIGH) is bimodal LOW/MODERATE: substrate-handled mechanisms
(watchdog, sync-check, predicate evaluation) cluster at LOW;
declarative-registry mechanisms (reducer registry, state machine,
lane queue) cluster at MODERATE. No HIGH-cost sub-shapes in cluster
A — design substrate offloads continuous-evaluation work to tools
once the substrate is in place. Phase 2 candidates adopting most
of cluster A inherit moderate per-cycle decision cost concentrated
on declarative-registry maintenance (reducer choice per surface,
state-machine schema, lane assignment per work-unit).

**Phase 2 evaluation use of these annotations:**

- *Implementation effort* — weight by ABSENT count; cluster A's
  6 ABSENT sub-shapes signal substantial Rust tool design work
  for any candidate adopting most of cluster A
- *Substrate-design effort by sub-cluster grouping* — recovery
  operations have 3 ABSENT sub-shapes (largest substrate-design
  burden); phase-boundary semantics have 2 ABSENT + 2 PARTIAL
  (next largest); concurrency-queuing has 1 ABSENT (single
  decision-point with 3 named options); process-isolation has
  0 ABSENT (free from substrate)
- *"Free from substrate" inheritance* — candidates should
  explicitly acknowledge which mechanisms they inherit (don't
  need to design) versus which they implement; failure to do so
  is itself a Phase 2 evaluation flag for under-specification
- *Self-management cost* (M2 retro added cycle 88) — Phase 2
  candidates adopting cluster A inherit 4 LOW + 5 MODERATE per-cycle
  cost; no HIGH-cost sub-shapes. Candidates favoring LOW-cost
  sub-shapes (termination predicates + watchdog + sync-check +
  process-isolation) minimize per-cycle overhead. The
  declarative-registry-heavy alternative (reducer registry + state
  machine + lane queue) inherits the MODERATE sub-shapes and is
  the higher-decision-cost path.

**Phase 2 implication**: cluster A is near-mandatory. v1's failure
modes (stale-reference accumulation, abandonment cascade,
chronic-category currency loop) all map to gaps in cycle-internal
phasing — see sub-cluster grouping annotations in the v1-failure-mode
mapping below. Sub-shape variety (9 across 4 sub-cluster groupings)
gives Phase 2 candidates significant combinatoric room for
differentiation, but candidates should be evaluated for coverage
distribution across sub-cluster groupings, not just total sub-shape
count.

## Cluster B: cross-cycle artifact organization

`[6-system clean]` AutoGen + LangGraph + Cognition + openclaw +
OpenAI harness + Voyager. Newly upgraded to 6-system clean cycle 69
via Voyager I-V3/I-V5/I-V6/I-V8.

State organized by component-of-origin, multi-surface, with
forward-spec + backward-history + active-retrieval-surface
distinction. The principle — state organized by component-of-origin
rather than central state file, multi-surface with separated
semantics — is shared across all six systems; the substrate
diverges across in-process dictionaries, file-per-component, typed
channels, repository-as-state, wiki servers, and skill-library +
vectordb hybrids. Sub-shapes (9) include component-local
persistence with per-component resume opt-in (Voyager I-V3 +
AutoGen I-6 cross-system parallel), short-term/long-term split via
typed-channel-map (LangGraph I-L5), multi-mechanism-per-coordinate
memory architecture with 7 distinct mechanisms (Cognition I-C3),
tripartite memory by content × temporal scope of durable / daily /
sweep-summary (openclaw I-O4), repository-as-state with ephemeral-
worktree task isolation (OpenAI harness I-OH4 / I-OH7), active-
surface-vs-monotonic-history storage discipline where active
retrieval surface is single-version while disk is monotonic-append
(Voyager I-V5), top-k semantic-retrieval over LLM-generated
descriptions rather than raw artifacts (Voyager I-V6), failure-as-
first-class-artifact (Voyager I-V8 + openclaw + OpenAI harness
sub-failure decomposition), and plans-as-forward-versioned-artifacts
(OpenAI harness; oh-my-codex stub-mention pending #2833 deeper
read).

**v1-substrate instantiation (M1, cycle 87 audit#454 absorption —
second cluster after cycle 86 cluster A) + self-management cost
(M2, cycle 87 audit#454 absorption — beginning of M2 annotation
work).** Each of the 9 sub-shapes is annotated with substrate-fit
(STRONG / PARTIAL / ABSENT) + self-management cost (LOW / MODERATE
/ HIGH) + current v1 instantiation if any + design work needed for
Phase 2 candidates that adopt the sub-shape. Cluster B sub-shapes
do NOT divide cleanly into sub-cluster groupings (unlike cluster
A's 4-3-1-1 phase-boundary / recovery / concurrency / process-
isolation distribution); cluster B's 9 sub-shapes are all storage-
architecture mechanisms with overlapping scope (component-of-origin
× temporal-scope × retrieval-mechanism intersect across sub-shapes).
Annotations proceed linearly without sub-grouping. Methodological
observation: sub-cluster grouping helped cluster A because of
natural conceptual boundaries (cycle 85 D3); not all clusters
benefit from sub-grouping.

Self-management cost grading: LOW = substrate handles the work,
orchestrator just invokes; MODERATE = orchestrator must explicitly
maintain state in declarative or automated form; HIGH = orchestrator
must continuously evaluate and update state as ongoing per-cycle
work. Self-management cost is a v1 failure-mode lens — v1's
chronic-category currency loop, gate proliferation, and abandonment
cascade represent self-management cost paid by orchestrator cycles.
Phase 2 candidates that minimize HIGH-cost sub-shape adoption
inherit lower self-management overhead. Cluster B M2 annotations
begin the M2 layer; cluster A M2 retro-annotations deferred to
cycle 88+.

1. **Component-local persistence with per-component resume opt-in
   (Voyager I-V3 + AutoGen I-6)** — substrate-fit PARTIAL. v1's
   filesystem organization already provides per-component
   persistence: `docs/journal/`, `docs/redesign/_notes/`,
   `docs/redesign/1-research/systems/`, `clusters.md`,
   `prompts/v2/` — each artifact is a separate file by
   component-of-origin; resume happens implicitly via orchestrator
   reading what's there. Per-component resume *opt-in* (declarative
   checkpoint per component) is absent; resume is uniformly
   read-everything. Design work: optional component-registry
   declaring "this component participates in resume" with explicit
   checkpoint semantics. Substrate via Rust tool
   `component-registry-check` validates declared components match
   filesystem state at session start. **Self-management cost: LOW.**
   Filesystem handles persistence; opt-in registry is one-time
   declaration per component, low ongoing maintenance.

2. **Short-term/long-term split via typed-channel-map (LangGraph
   I-L5)** — substrate-fit PARTIAL. v1 has implicit short/long
   split: cycle issue comments are short-term ephemeral chatter,
   journal entries are daily-scope durable, clusters.md is fully
   durable artifact, prompt is design-contract durable. The split
   is convention; no typed-channel-map declares scope per surface.
   Design work: state-surface registry with per-surface scope tag
   (short / long / durable) + reducer rule + retention policy.
   Substrate via Rust tool `state-surface-registry` consulted at
   session start to validate write targets match declared scope;
   warns on cross-scope writes. **Self-management cost: MODERATE.**
   Channel-map registry needs maintenance as surfaces are added or
   retired; write-routing checks per cycle add minor decision cost.

3. **Multi-mechanism-per-coordinate memory architecture with 7
   distinct mechanisms (Cognition I-C3)** — substrate-fit PARTIAL.
   v1 has multi-mechanism storage: journal entries, clusters.md,
   _notes/cycle-NN/, cycle issue comments, dispatch issues, audit
   cross-reads, PR descriptions, the prompt itself — that's 8
   mechanisms, comparable to Cognition's 7. Per-coordinate
   organization (mechanism × coordinate map: agent × topic × time)
   is absent; v1's mechanisms organize by semantic role not
   coordinate. Design work: explicit coordinate-map declaring
   mechanism-by-coordinate intersection (which mechanism handles
   which (agent, topic, time-scope) cell). Substrate via Rust tool
   `coordinate-map-check` ensures coverage and detects redundancy.
   **Self-management cost: HIGH.** Each mechanism has its own
   maintenance discipline; coordinate-system itself evolves over
   time as new mechanisms are added; retrieval-routing per
   coordinate adds per-cycle decision cost.

4. **Tripartite memory by content × temporal scope (openclaw
   I-O4: durable / daily / sweep-summary)** — substrate-fit
   PARTIAL. v1 has durable scope (clusters.md, prompt, retrospective)
   and daily scope (journal entries) instantiated; sweep-summary
   is absent — closest is periodic synthesis cycles like cycle 65
   + cycle 84 + cycle 73 cluster restructure, but those are
   themselves durable artifacts not sweep-summaries-rolling-up-
   daily-or-cycle-bounded-period. Design work: explicit `sweep/`
   directory or scheduled-cycle convention where each defined
   period (weekly, every-N-cycles) produces a summary rolling up
   intervening journals into key observations. Substrate via Rust
   tool `sweep-rollup` triggered manually or on schedule.
   **Self-management cost: MODERATE-HIGH.** Sweep-summary requires
   explicit periodic rollup work each cycle (or per defined cadence)
   — this is exactly the kind of work that consumes cycles per
   audit#454 self-management cost critique; tool can lower the
   per-cycle cost but the rollup discipline itself is overhead.

5. **Repository-as-state with ephemeral-worktree task isolation
   (OpenAI harness I-OH4 / I-OH7)** — substrate-fit STRONG. v1 IS
   this. Repository (Git repo) holds all state (issues, comments,
   files); GitHub Actions runner is ephemeral worktree (each
   cron-triggered cycle runs in fresh runner with repo cloned
   fresh; no state carries across runners except through the
   repository). Both halves of the substrate are inherent to v1.
   Phase 2 candidates that preserve "state in repo + isolated
   cycle process" inherit this without effort; candidates that
   deviate (persistent worker process, in-memory state across
   cycles) lose the property and must re-derive it via different
   mechanism. **Self-management cost: LOW.** Substrate handles it;
   orchestrator commits and pushes; the only explicit cost is
   per-commit message and structure discipline (already established
   convention). Cross-references the cluster A sub-shape 9
   process-isolation STRONG annotation — same substrate property
   surfaces in both clusters (this is sub-shape overlap across
   clusters, expected per the audit#454 D2 dual-cast classification).

6. **Active-surface-vs-monotonic-history storage discipline
   (Voyager I-V5)** — substrate-fit PARTIAL. v1 has the discipline
   structurally: clusters.md is single-version active surface
   (overwritten each cycle); _notes/cycle-NN/ files are monotonic
   per-cycle append (one file per cycle, never modified after);
   journal entries are append-only per day. Git history provides
   the underlying monotonic timeline for the active-surface
   artifacts. What's missing: orchestrator doesn't typically
   *retrieve* from monotonic history within a cycle — it reads
   only the active surface. Design work: optional `git-show` or
   `_notes/index.md` retrieval index providing structured history
   queries. Substrate via Rust tool `history-query` for retrieving
   active-surface history slices on-demand. **Self-management
   cost: LOW-MODERATE.** File-system convention handles most
   discipline (no maintenance); explicit retrieval-from-history
   adds cost (re-reading old _notes/ files, querying git history)
   but is opt-in per cycle, not per-cycle baseline.

7. **Top-k semantic-retrieval over LLM-generated descriptions
   rather than raw artifacts (Voyager I-V6)** — substrate-fit
   ABSENT. v1 has no semantic search, no LLM-generated descriptions
   of artifacts, no top-k retrieval mechanism. Orchestrator reads
   files via Read tool with explicit paths; cross-artifact
   connections are made by orchestrator memory (limited within a
   cycle, absent across cycles) not by retrieval system. Design
   work: substantial — index over _notes/ + journal/ + clusters.md
   with LLM-generated descriptions; semantic-search Rust tool
   (likely backed by embedding-DB integration: sqlite-vss or
   similar) supporting top-k retrieval. Substrate via Rust tool
   `semantic-retrieve` plus periodic re-indexing infrastructure on
   artifact changes. The substrate change is non-trivial because
   v1 has no embedding-DB infrastructure; this sub-shape is the
   most architecturally-distinct from v1 within cluster B.
   **Self-management cost: HIGH.** LLM-generated descriptions need
   maintenance as artifacts evolve; re-indexing on artifact changes;
   retrieval-failure modes require monitoring; top-k threshold
   tuning is ongoing.

8. **Failure-as-first-class-artifact (Voyager I-V8 + openclaw +
   OpenAI harness sub-failure decomposition)** — substrate-fit
   PARTIAL. v1 tracks failures unstructured: cycle 71 stuck-
   dispatch documented in journal + _notes; the 4 dispatches
   awaiting Copilot assignment for 22+ cycles tracked via issue
   tracker (open status as proxy for "still failing"); audit#454 +
   audit#455 acknowledged as comments. There's no structured
   `state/failures/` directory or failure-record format. Design
   work: structured failure artifact with declared schema (failure-
   mode tag, root-cause hypothesis, observed symptoms, attempted
   mitigations, current state, decision impact). Substrate via
   Rust tool `failure-record` for creating/updating; consulted at
   next-cycle-composition decision per Voyager I-V8 pattern.
   Cross-references v1-failure-mode mapping above (forgotten-failure
   mode is named cluster B failure-as-first-class-artifact + cluster
   D failure-as-recorded-artifact). **Self-management cost:
   MODERATE.** Each failure requires explicit structured-artifact
   authoring (more cost than journal-prose); structured schema
   evolves over time; consulting failure records at composition
   decision adds per-cycle decision surface.

9. **Plans-as-forward-versioned-artifacts (OpenAI harness;
   oh-my-codex stub-mention pending #2833 deeper read)** —
   substrate-fit ABSENT. v1 has no plans-as-distinct-artifacts.
   The redesign prompt is a versioned-by-commit forward-spec
   (design contract), but cycle-level plans are journal prose
   (e.g., cycle 86 → 87/88/89 hand-off plan named in the cycle 86
   journal entry). Multi-cycle plans aren't separately versioned
   artifacts. Design work: `plans/` directory with one file per
   plan; per-plan version-history (git-backed) + retired-on-
   supersede mechanism + active-vs-retired discipline. Substrate
   via Rust tool `plan-status` showing active vs retired plans
   and detecting plan-to-actual divergence. **Self-management
   cost: MODERATE.** Each plan must be authored, versioned,
   retired when superseded; if plans are not maintained, they
   become stale references (the v1 1-research.md drift pattern,
   cycles 30-50). Without active-vs-retired discipline, plan
   accumulation matches the issue-tracker-accumulation problem
   per cycle 33 housekeeping observation.

**Substrate-fit summary across cluster B's 9 sub-shapes:**
- **STRONG (1):** repository-as-state with ephemeral-worktree task
  isolation (sub-shape 5)
- **PARTIAL (6):** component-local persistence (1), short/long
  split (2), multi-mechanism-per-coordinate (3), tripartite (4),
  active-surface-vs-monotonic-history (6), failure-as-first-class-
  artifact (8)
- **ABSENT (2):** top-k semantic-retrieval (7), plans-as-forward-
  versioned-artifacts (9)

**Self-management cost summary across cluster B's 9 sub-shapes:**
- **LOW (3):** component-local persistence (1), repository-as-state
  (5), active-surface-vs-monotonic-history (6)
- **MODERATE (4):** short/long split (2), tripartite (4) — boundary
  MODERATE-HIGH, failure-as-first-class-artifact (8), plans-as-
  forward-versioned-artifacts (9)
- **HIGH (2):** multi-mechanism-per-coordinate (3), top-k semantic-
  retrieval (7)

**Comparison to cluster A** (cycle 86 annotations): cluster B is
substantively MORE substrate-aligned than cluster A (1 STRONG / 6
PARTIAL / 2 ABSENT vs cluster A's 1 STRONG / 2 PARTIAL / 6 ABSENT).
Cycle 86 hand-off prediction (cluster B is 6-system clean and v1's
repo-as-state pattern aligns with multiple cluster B sub-shapes —
likely to produce more STRONG/PARTIAL grades than cluster A) bears
out empirically. The PARTIAL count flip (2→6) reflects v1's
filesystem-organization pattern providing structural foundation
for most cluster B sub-shapes that requires only declarative
scaffolding to formalize, whereas cluster A's mostly-ABSENT pattern
requires substantial new substrate design (typed phase-state
machine, lane-aware queue, watchdog, retry semantics).

**Phase 2 evaluation use of these annotations:**

- *Implementation effort* — weight by ABSENT count; cluster B's
  2 ABSENT sub-shapes (top-k semantic-retrieval + plans-as-forward-
  versioned-artifacts) signal substantial Rust tool design but
  materially less than cluster A's 6 ABSENT sub-shapes. Phase 2
  candidates adopting most of cluster B inherit moderate
  implementation effort centered on declarative scaffolding for
  PARTIAL sub-shapes.
- *Self-management cost* — Phase 2 candidates that adopt cluster
  B's HIGH-cost sub-shapes (multi-mechanism-per-coordinate +
  top-k semantic-retrieval) inherit per-cycle decision overhead;
  candidates favoring LOW-cost sub-shapes (repository-as-state +
  component-local persistence + active-surface-vs-monotonic-
  history) minimize self-management cost. The "minimal-departure-
  from-v1" combination (sub-shapes 5 + 9) is also LOW + MODERATE
  self-management cost.
- *"Free from substrate" inheritance* — repository-as-state with
  ephemeral-worktree (sub-shape 5) is the strongest free-from-
  substrate inheritance across cluster B. Phase 2 candidates that
  preserve this inherit it without effort; candidates that deviate
  lose the property and must re-derive it (Docker container per
  cycle, in-memory sandbox reset).
- *"Minimal-departure-from-v1" combination* (already named in
  v1-failure-mode mapping above): repository-as-state (5) +
  plans-as-forward-versioned-artifacts (9). With M1 annotations
  visible: this combination is now 1 STRONG + 1 ABSENT — the
  plans-as-forward-versioned-artifacts sub-shape requires new
  design work even in the "minimal-departure" combination. This
  is empirical confirmation that even minimal departure has
  non-trivial substrate-design surface.
- *"Parsimonious 3-sub-shape" combination* (already named in
  v1-failure-mode mapping above): component-local (1) + active-
  surface-vs-monotonic-history (6) + failure-as-first-class-
  artifact (8). With M1 annotations visible: this combination is
  0 STRONG + 3 PARTIAL — all three build on existing v1 patterns
  but require explicit declarative scaffolding. This is the
  lowest-substrate-effort meaningful combination addressing the
  named v1 failure modes (stale-reference accumulation +
  forgotten-failure).
- *"Maximal 9-sub-shape" combination* (already named): all 9
  sub-shapes. With M1 annotations visible: this combination is
  1 STRONG + 6 PARTIAL + 2 ABSENT, with 2 HIGH self-management
  cost sub-shapes. The maximal combination has the highest
  implementation surface AND the highest per-cycle decision cost;
  not recommended unless the candidate can demonstrate the rich
  storage architecture is load-bearing for substantive work.

**Phase 2 implication**: cluster B is the strongest case for
storage-architecture as a v2 candidate generator — Phase 2
candidates differ substantially based on which sub-shape
combination they adopt (parsimonious 3-sub-shape candidate vs
all-9-sub-shape rich candidate; repository-anchored vs file-per-
component vs hybrid).

## Cluster-spanning v1 strengths layer (M3, cycle 88 audit#454 absorption)

Audit#454 named "what v1 already does well is unnamed" as M3
missing. M1 substrate-fit annotations identify STRONG sub-shapes
per cluster; M3 layer surfaces these as cluster-spanning v1
strengths to be preserved by Phase 2 candidates. Cycle 88
implements M3 framing + initial annotations for clusters annotated
to date (A, B, D, F). STRONG sub-shapes from cycles 86-88
annotations yield 3 distinct v1 strengths after de-duplication of
dual-cast substrate properties:

**v1 strength 1: Process-isolation via ephemeral worktrees**
(A↔B substrate-property dual-cast). v1 inherits this from GitHub
Actions runner: each cron-triggered cycle runs in a fresh runner
with the repo cloned fresh; no state carries across runners except
through the repository (commits) and external systems (issues,
labels, comments). Cross-references: cluster A sub-shape 9 STRONG,
cluster B sub-shape 5 STRONG (same v1 substrate property dual-cast
across cluster lenses per audit#454 D2 dual-cast classification).
*Phase 2 implication*: candidates that preserve "cycle = isolated
process" inherit this without effort; candidates that deviate
(persistent worker, in-memory state across cycles) lose the
property and must re-derive it via different mechanism (Docker
container per cycle, in-memory sandbox reset). This is the
strongest substrate-inherited v1 strength — single property,
single substrate, two cluster-lens annotations.

**v1 strength 2: Anti-patterns as published artifact alongside
recommended patterns**. v1 has 0-retrospective.md (Phase 0 honest
disclosure of v1 failure modes), journal entries (per-cycle prose
acknowledging failures and uncertainty), clusters.md "v1-failure-
mode mapping" subsection (5 named v1 failure modes mapped to which
cluster sub-shapes address them), and the cycle 33 housekeeping
observation (issue-tracker accumulation as v1 failure pattern
documented as design-input). Cross-references: cluster D sub-shape
1 STRONG. *Phase 2 implication*: candidates that preserve this
discipline inherit honest failure documentation as default;
candidates that suppress failure-disclosure (e.g., framing all v1
failures as "v1 wins" or hiding failure cases) lose this v1
strength. The substrate (git repository, public repo, journal
directory) supports this inherently — the discipline is the
strength, not the substrate.

**v1 strength 3: Walkback as first-class artifact preserving
prior framing alongside revised one**. v1 has framework iteration
history preserved in 2-design-framework.md (v1.0-v1.22 changes
documented across cycles 35-61), per-cycle _notes/ capturing
reasoning at the time of decision, git history preserving prior
commits. Cycle 73 cluster restructure preserved prior structure
note in _notes/cycle-73-restructure.md; cycle 85 D2 dual-cast vs
compositional revision preserved prior framing in clusters.md
text via "compositional vs dual-cast classification" preamble.
Cross-references: cluster D sub-shape 2 STRONG. *Phase 2
implication*: candidates that preserve git-backed history +
per-cycle _notes/ + framework iteration history inherit this
without extra effort; candidates that compress history (squash
all framework changes into "v2 framework v1.0") lose this v1
strength.

**M3 layer use across clusters:**

- *Phase 2 evaluation* — candidates should explicitly acknowledge
  which v1 strengths they preserve vs deviate from; failure to
  acknowledge is itself a Phase 2 evaluation flag for
  under-specification (parallel to "free from substrate"
  inheritance under-specification flag in cluster A and B M1
  layers)
- *Substrate inheritance vs convention inheritance* — strength 1
  (process-isolation) is substrate-inherited (GitHub Actions
  runner property); strengths 2 (anti-patterns published) and 3
  (walkback as artifact) are convention-inherited (depend on v1
  documentation discipline, not substrate property). The
  distinction matters for Phase 2: substrate-inherited strengths
  cost nothing to preserve; convention-inherited strengths cost
  ongoing discipline.
- *Across-cycle preservation* — strengths 2 and 3 require ongoing
  discipline; if v1 documentation discipline lapses across many
  cycles (e.g., cycles where journal entries become slim, _notes/
  drop in detail, framework iteration history becomes uneven),
  the strength erodes. M3 layer is a record-keeping discipline as
  much as a substrate property. Per cycle 87 reflection on
  iteration-until-approval, sustained discipline across cycles is
  itself the load-bearing factor.
- *Cluster F adds no v1 strengths to M3 layer* — cluster F
  (tool-suite stratification) has 0 STRONG sub-shapes per cycle
  88 M1 annotation. v1 has rudimentary stratification (Rust tools
  as primitives + orchestrator as LLM-composed) but no formal
  stratification across cluster F's 8 sub-axes. Phase 2 candidates
  have substantial design-work surface in cluster F regardless of
  which v1 strengths they preserve. This is the first cluster
  annotated to add nothing to M3 — empirical confirmation that
  M3 layer composition is per-cluster and not all clusters
  contribute equally.
- *M3 vs M2 distinction* — M3 names what v1 already does well
  (positive framing of substrate + convention); M2 names per-cycle
  decision overhead (negative framing of self-management cost).
  M3 strengths can correlate with M2 LOW grades (substrate or
  convention handles) but not always: cluster D sub-shape 2
  (walkback) is STRONG + LOW (substrate-handled); cluster D
  sub-shape 1 (anti-patterns) is STRONG + LOW (convention-handled).
  Both M2 LOW grades are M3-strength candidates because the work
  is already happening at low cost.

**M3 layer growth across cycles 89+**: cycles 89+ may add v1
strengths from clusters C / E / G / H / I as M1 annotations land;
cycle 89 plan covers those clusters. Expected: cluster G (role-
asymmetric context — clean-context-reviewer pattern) likely adds
1 STRONG strength via v1's audit-as-peer pattern; cluster I
(harness-enforced security/policy boundaries) likely adds 0-1
STRONG via v1's GitHub Actions secret injection + branch
protection. Other clusters less certain at this stage.

## Cluster D: documentation honesty

`[5-system clean + Voyager partial]` AutoGen + LangGraph + Cognition
+ openclaw + OpenAI harness primary; Voyager partial via I-V8
failure-as-recorded-artifact contribution.

Published artifacts honestly disclose what was tried and what
didn't work, what was decided and why, what the position currently
is — including walkbacks of prior positions. Documentation is
treated as first-class engineering artifact, not optional cruft.
Sub-shapes (9) include anti-patterns as published artifact
alongside recommended patterns (six-system Family-pattern at
[3+/6 systems] depth — see Family E in [`../1-research.md`](../1-research.md)
for the elevation), walkback as first-class artifact preserving
prior thesis alongside revised one (Cognition I-C1 — June 2025
single-threaded thesis preserved alongside April 2026 walkback),
invariants vs derivations as separate published layers (Cognition
I-C2 — durable invariants vs revisable derivations stratified),
migration guide as first-class artifact with explicit deprecation
semantics (AutoGen I-1), AGENTS.md as table-of-contents to deeper
docs/ rather than single monolithic prompt (OpenAI harness I-OH1),
structured failure-mechanism decomposition for documented
anti-patterns (OpenAI harness I-OH3 — four named failure mechanisms
for "one big AGENTS.md"), failure-as-first-class-recorded-artifact
(Voyager I-V8), under-delegation as documented failure mode peer
to over-delegation (Cognition I-C9), and anti-pattern catalog with
explicit non-permanence framing — versioned and revisable rather
than absolute (openclaw I-O2 "roadmap-guardrail-not-law-of-physics"
qualifier; structurally distinct from Cognition's June-2025-vs-
April-2026 walkback because openclaw's catalog is upfront with
non-permanence built in).

**v1-substrate instantiation (M1, cycle 88 audit#454 absorption —
third cluster after cycle 86 cluster A and cycle 87 cluster B) +
self-management cost (M2, cycle 88).** Each of the 9 sub-shapes is
annotated with substrate-fit (STRONG / PARTIAL / ABSENT) +
self-management cost (LOW / MODERATE / HIGH) + current v1
instantiation if any + design work needed for Phase 2 candidates
that adopt the sub-shape. Cluster D sub-shapes do NOT divide
cleanly into sub-cluster groupings (parallel to cluster B); the 9
sub-shapes mostly cover documentation form (anti-patterns,
walkback, invariants/derivations, migration, AGENTS.md,
structured-failure, failure-as-artifact, under-delegation,
anti-pattern non-permanence) — overlapping concerns rather than
orthogonal groupings. Annotations proceed linearly without
sub-grouping.

1. **Anti-patterns as published artifact alongside recommended
   patterns (six-system Family E [3+/6 systems])** — substrate-fit
   STRONG. v1 has 0-retrospective.md (Phase 0 retrospective with
   anti-patterns named), journal entries that honestly acknowledge
   failures (e.g., cycle 71 stuck-dispatch documented in journal +
   _notes), clusters.md "v1-failure-mode mapping" subsection naming
   5 v1 failure modes mapped to cluster sub-shapes, and the cycle
   33 housekeeping observation (issue-tracker accumulation as v1
   anti-pattern). v1 already does this work via the redesign
   discipline. Phase 2 candidates that preserve "publish anti-
   patterns as first-class material" inherit this without effort;
   candidates that suppress anti-pattern publication lose this v1
   strength. **Self-management cost: LOW.** Substrate (git repo,
   public repo, journal directory) supports this inherently; the
   discipline is the strength.

2. **Walkback as first-class artifact preserving prior thesis
   alongside revised one (Cognition I-C1)** — substrate-fit STRONG.
   v1 preserves prior framing alongside revisions: framework
   iteration history v1.0-v1.22 preserved in 2-design-framework.md
   across cycles 35-61; per-cycle _notes/ capture reasoning at the
   time of decision; git preserves prior commits. Cycle 73 cluster
   restructure walkback documented in _notes; cycle 85 D2 dual-cast
   vs compositional revision preserved prior framing via
   classification preamble in clusters.md. Phase 2 candidates that
   preserve git-backed history + per-cycle _notes/ + framework
   iteration history inherit this without effort; candidates that
   compress history lose this v1 strength. **Self-management cost:
   LOW.** Git-backed history is automatic; per-cycle _notes/ is
   already-baseline cycle work; iteration history preserved by
   minimal-change discipline established cycles 35-61.

3. **Invariants vs derivations as separate published layers
   (Cognition I-C2)** — substrate-fit PARTIAL. v1 has
   2-design-framework.md with framework axes and constraints, but
   invariants vs derivations are not stratified into separate
   layers. The Constraint 8 promotion (cycle 37 v1.2) is the
   closest formalized stratification (axis-promoted-to-constraint
   pattern). Design work: explicit stratification of v2 prompt
   into "invariants" (non-negotiable structural commitments, e.g.,
   process-isolation) and "derivations" (revisable on evidence,
   e.g., specific tool names). Substrate via prompt structure
   convention (separate subsections + version-comment header per
   layer); no Rust tool needed. **Self-management cost: MODERATE.**
   Stratification requires per-revision evaluation (is this an
   invariant or a derivation?); cross-references between layers
   add maintenance cost; promotion-from-derivation-to-invariant
   requires explicit ratification (cycle 37 Constraint 8 pattern).

4. **Migration guide as first-class artifact with explicit
   deprecation semantics (AutoGen I-1)** — substrate-fit ABSENT.
   v1 has no migration guide for v1→v2 yet; that work is Phase 4
   cutover deliverable. The pre-cutover checkpoint requires
   migration runbook + rollback runbook (per artifact-composition
   in this prompt), but those are Phase 4 outputs not Phase 1
   substrate. Design work: structured migration guide format with
   per-component deprecation-and-replacement semantics (table or
   per-component subsections). Substrate via Rust tool
   `migration-status` showing per-component migration progress; v1
   retention discipline (~30 cycles minimum after cutover) declared
   explicitly. **Self-management cost: MODERATE.** Migration guide
   requires per-deprecation maintenance during cutover phase;
   outside cutover, low ongoing cost. Cost is concentrated at
   version-boundary moments (Phase 4 cutover; future v2→v3
   transitions).

5. **AGENTS.md as table-of-contents to deeper docs/ rather than
   single monolithic prompt (OpenAI harness I-OH1)** — substrate-fit
   PARTIAL. v1 has the orchestrator prompt + STARTUP_CHECKLIST.xml
   + COMPLETION_CHECKLIST.xml + docs/redesign/ tree. The prompt is
   currently large (~600+ lines with extensive section structure),
   more monolithic than TOC-pointing-to-deeper-docs. STARTUP_CHECKLIST
   and COMPLETION_CHECKLIST act as deeper docs the prompt references,
   but the v2 prompt may be substantially smaller per the
   core-design-principle (prompt instructs orchestrator to invoke
   tools; tools handle procedure). Design work: v2 prompt
   structured as TOC pointing to per-task playbooks (cluster F
   sub-axis 2 dual-cast) + per-component capability declarations
   (cluster F sub-axis 3 dual-cast) + per-tool documentation.
   Substrate via filesystem layout convention; no Rust tool needed
   but cross-references must be maintained. **Self-management
   cost: MODERATE.** TOC needs maintenance as docs grow or change;
   cross-references break if files are renamed without updating;
   periodic TOC audit is cycle work.

6. **Structured failure-mechanism decomposition for documented
   anti-patterns (OpenAI harness I-OH3 — four named failure
   mechanisms for "one big AGENTS.md")** — substrate-fit PARTIAL.
   v1 has unstructured failure documentation in journals (cycle 71
   stuck-dispatch documented but not with structured failure-
   mechanism decomposition); cluster B sub-shape 8 (failure-as-
   first-class-artifact) overlaps with this — same need for
   structured schema. Design work: per-anti-pattern structured
   decomposition with named failure mechanisms (root cause,
   observed symptoms, mitigations attempted). Substrate via Rust
   tool `failure-record` (cross-cast with cluster B sub-shape 8).
   **Self-management cost: MODERATE.** Each anti-pattern requires
   structured analysis (more cost than journal prose);
   decomposition templates evolve over time. Same cost level as
   cluster B sub-shape 8 (failure-as-first-class-artifact) — they
   share substrate.

7. **Failure-as-first-class-recorded-artifact (Voyager I-V8 +
   openclaw + OpenAI harness sub-failure decomposition)** —
   substrate-fit PARTIAL. Same as cluster B sub-shape 8
   (substrate-fit PARTIAL, self-management cost MODERATE).
   Dual-cast across cluster B (storage architecture lens —
   failure-as-stored-artifact) and cluster D (documentation
   honesty lens — failure-as-published-artifact). The v1
   instantiation, design work, and self-management cost are the
   same. **Self-management cost: MODERATE.** Per audit#454 D2
   dual-cast classification, this annotation cross-references
   cluster B sub-shape 8 rather than counting as a distinct
   cluster D substrate-design item.

8. **Under-delegation as documented failure mode peer to
   over-delegation (Cognition I-C9)** — substrate-fit PARTIAL.
   v1 has discussion of under-delegation in journals (orchestrator
   doing routine work that should be in tools — the core-design-
   principle violation pattern is itself an under-delegation
   discussion), but no structured catalog of under-delegation
   cases peer to over-delegation. Design work: anti-pattern catalog
   with both over- and under-delegation entries; per-entry: what
   was over/under-delegated, what should have been the right level,
   what the observable failure mode is. Substrate via clusters.md
   or v2 anti-pattern catalog file (same substrate as sub-shape 9).
   **Self-management cost: LOW.** Once documented, rarely needs
   updating; mostly passive reference material.

9. **Anti-pattern catalog with explicit non-permanence framing
   (openclaw I-O2 "roadmap-guardrail-not-law-of-physics")** —
   substrate-fit PARTIAL. v1's clusters.md has implicit "this is
   current state, may evolve" framing through the cycle-N-by-cycle
   update pattern, but no explicit non-permanence catalog framing
   per anti-pattern. Design work: anti-pattern catalog with per-
   entry "current state — may revise on evidence" framing
   (versioned, revisable rather than absolute); cross-references
   to walkback artifacts when entries are revised. Substrate via
   clusters.md or v2 anti-pattern catalog file (same substrate as
   sub-shape 8). **Self-management cost: LOW.** Non-permanence is
   upfront framing not ongoing maintenance; revision-when-evidence-
   warrants is event-driven not cycle-baseline.

**Substrate-fit summary across cluster D's 9 sub-shapes:**
- **STRONG (2):** anti-patterns as published artifact (1), walkback
  as first-class artifact (2)
- **PARTIAL (6):** invariants/derivations stratification (3),
  AGENTS.md as TOC (5), structured failure-mechanism decomposition
  (6), failure-as-first-class-artifact (7), under-delegation (8),
  anti-pattern catalog with non-permanence (9)
- **ABSENT (1):** migration guide with explicit deprecation
  semantics (4)

**Self-management cost summary across cluster D's 9 sub-shapes:**
- **LOW (4):** anti-patterns published (1), walkback (2),
  under-delegation (8), anti-pattern non-permanence (9)
- **MODERATE (5):** invariants/derivations (3), migration guide
  (4), AGENTS.md TOC (5), structured failure-mechanism (6),
  failure-as-first-class-artifact (7)
- **HIGH (0):** none

**Comparison to clusters A and B:** cluster D is the MOST
substrate-aligned of the three clusters annotated to date (2 STRONG
/ 6 PARTIAL / 1 ABSENT vs cluster B's 1 STRONG / 6 PARTIAL / 2
ABSENT vs cluster A's 1 STRONG / 2 PARTIAL / 6 ABSENT). The
audit#454 statement that "v1 already does this work via the
retrospective cadence and the journal" is empirically validated —
cluster D has the highest STRONG count (2) and the lowest ABSENT
count (1) of the three most-foregrounded clusters. Self-management
cost distribution is the lowest-cost: 4 LOW + 5 MODERATE + 0 HIGH
(vs cluster B's 3 LOW + 4 MODERATE + 2 HIGH; vs cluster A's 4 LOW
+ 5 MODERATE + 0 HIGH per cycle 88 retro). Cluster D is a strong
candidate for Phase 2 candidate adoption with low effort.

**Phase 2 evaluation use of these annotations:**

- *Implementation effort* — weight by ABSENT count; cluster D's
  1 ABSENT sub-shape (migration guide) signals limited Rust tool
  design (migration guide is mostly format convention, not tool
  surface). Phase 2 candidates adopting most of cluster D inherit
  minimal implementation effort centered on declarative scaffolding
  for PARTIAL sub-shapes.
- *Self-management cost* — Phase 2 candidates that adopt cluster
  D inherit LOW-or-MODERATE per-cycle cost; no HIGH-cost sub-shapes
  to weight against. Cluster D is the lowest-cost cluster annotated
  to date.
- *"Free from substrate" inheritance* — anti-patterns as published
  artifact (1) and walkback as first-class artifact (2) are
  inherited from v1's documentation discipline rather than from
  substrate. Phase 2 candidates that preserve documentation
  discipline inherit these strengths; candidates that compress
  history or suppress failure disclosure lose them. (Cross-reference
  to M3 v1 strengths layer below.)
- *"Maximal 9-sub-shape" combination*: cluster D max is 2 STRONG
  + 6 PARTIAL + 1 ABSENT, with 4 LOW + 5 MODERATE + 0 HIGH
  self-management cost. The maximal combination has limited
  implementation surface and bounded per-cycle decision cost;
  recommended as default-inclusion in Phase 2 candidates.
- *"Minimal-departure-from-v1" combination*: cluster D minimum is
  sub-shapes 1 + 2 (the two STRONG sub-shapes). With M1 annotations
  visible: this is 2 STRONG + 0 PARTIAL + 0 ABSENT — pure
  inheritance, no design work required. Recommended as default
  for Phase 2 candidates that want documentation-honesty discipline
  without additional implementation.
- *Dual-cast with cluster B* — sub-shape 7 (failure-as-first-class-
  recorded-artifact) is the same v1 instantiation as cluster B
  sub-shape 8; design work and self-management cost shared.
  Phase 2 candidates adopting either cluster B sub-shape 8 or
  cluster D sub-shape 7 inherit this once, not twice.

**Phase 2 implication**: v1 already does this work via the
retrospective cadence and the journal. Cluster D additions for v2
are walkback as first-class protocol (formalizing the existing
discipline), invariants/derivations stratification within the v2
prompt, anti-pattern catalog ("What v2 Will Not Do" alongside
"What v2 Will Do"), and structured failure records. The
2-STRONG-sub-shape inheritance from v1 is the strongest argument
for cluster D as default-inclusion in Phase 2 candidates: even a
candidate that adopts only cluster D's STRONG sub-shapes inherits
real v1 strengths at zero cost.

## Cluster F: tool-suite stratification (multi-axis)

`[5-system convergent]` openclaw + OpenAI harness + Cognition +
AutoGen + Voyager. Cycle 69 added Voyager as the 5th system with
the largest single-cycle sub-axis growth in the corpus (4 sub-axes
→ 8 sub-axes).

Tools, capabilities, models, terms, roles, and modes are stratified
along multiple parallel discrimination axes. The meta-architectural-
pattern: many distinct stratification axes coexist within a single
architectural domain. The 8 sub-axes:

1. **Version stratification** — active-version vs prior versions
   (openclaw plugin versioning; Voyager skill versioning V2/V3 on
   disk while vectordb keeps active version)
2. **Task-class stratification** — Playbook templates per task-class
   with outcome + steps + postconditions + advice + forbidden
   actions (Cognition I-C6)
3. **Capability-tier stratification** — Tier 1 read-only / Tier 2
   send-on-behalf / Tier 3 autonomous-with-standing-orders
   (openclaw I-O6)
4. **Terminology stratification** — explicit different meanings per
   term (openclaw tool / skill / plugin distinction, I-O8 — tools=
   function calls, skills=Markdown-injected, plugins=npm packages)
5. **Role stratification** — distinct named conceptual roles per
   responsibility (Voyager 4-agent architecture I-V1 — ActionAgent
   / CurriculumAgent / CriticAgent / SkillManager)
6. **Cost-tier stratification** — model-per-task-class within an
   agent ecosystem (Voyager I-V2 — gpt-4 for novel reasoning,
   gpt-3.5-turbo for cached/derivative work; v2 candidate extends
   within Anthropic family Opus / Sonnet / Haiku per task-class)
7. **Autonomy-mode stratification** — auto vs manual per-component
   with explicit human-in-the-loop method paths (Voyager I-V9;
   openclaw operator-tier-level)
8. **Capability-layer stratification** — primitives vs LLM-composed
   compositions (Voyager I-V10 — control_primitives + skill-library
   compose primitives + earlier skills; openclaw tool/skill
   distinction is parallel)

**v1-substrate instantiation (M1, cycle 88 audit#454 absorption —
fourth cluster after clusters A / B / D) + self-management cost
(M2, cycle 88).** Each of the 8 sub-axes is annotated. Cluster F
sub-axes are largely orthogonal (per "Phase 2 implication:
stratification axes are mostly independent and combinable") so
annotations proceed linearly. Note: cluster F is a multi-axis
cluster — each sub-axis is its own stratification dimension, not a
sub-mechanism within a single architectural domain. Substrate-fit
grades reflect "how much stratification along this axis is already
in v1."

1. **Version stratification — active-version vs prior versions
   (openclaw plugin versioning; Voyager skill versioning V2/V3 on
   disk while vectordb keeps active version)** — substrate-fit
   PARTIAL. v1 has git-backed versioning + redesign-mode
   `prompts/v2/` directory alongside production prompt at
   `.github/workflows/orchestrator-prompt.xml`. Active-version-vs-
   prior is partial: redesign zones support versioning explicitly;
   production code is git-only. Design work: explicit active-version
   manifest per component (which version is loaded at session
   start); prior-version retention policy. Substrate via filesystem
   convention (existing) + Rust tool `version-status` listing
   active vs prior per component. **Self-management cost: LOW.**
   Git-backed; manifest is one-time declaration per component;
   active-version selection happens at session start (substrate-
   handled).

2. **Task-class stratification — Playbook templates per task-class
   with outcome + steps + postconditions + advice + forbidden
   actions (Cognition I-C6)** — substrate-fit ABSENT. v1's prompt
   has SECTION-by-SECTION structure (mission, authority, constraints,
   security, etc.) but these are not Playbook templates per task-
   class. STARTUP_CHECKLIST + COMPLETION_CHECKLIST have steps but
   lack the outcome + postcondition + forbidden-actions structure.
   Design work: substantial — per-task-class Playbook authoring
   (e.g., audit-absorption, dispatch-construction, M-item
   integration each have a Playbook). Substrate via `playbooks/`
   directory with one Markdown file per task-class; Rust tool
   `playbook-load` invoked at task-start. **Self-management cost:
   HIGH.** Each task-class needs Playbook authoring; templates
   evolve as new task-classes emerge; cross-references between
   Playbooks add maintenance cost; per-cycle Playbook-selection
   adds decision cost.

3. **Capability-tier stratification — Tier 1 read-only / Tier 2
   send-on-behalf / Tier 3 autonomous-with-standing-orders
   (openclaw I-O6)** — substrate-fit PARTIAL. v1 has direct-push
   zones (essentially Tier 3 within `prompts/v2/`, `tools/v2/`,
   `docs/redesign/`, `docs/journal/`), workflow-change PR zones
   (Tier 2 — orchestrator drafts, Eva merges), and forbidden zones
   (this prompt, `.github/workflows/`, production tools — Tier 0/1
   read-only). Design work: explicit Tier declaration per
   filesystem region; per-Tier capability table (which actions are
   authorized at which Tier). Substrate via filesystem layout +
   Tier-manifest file; Rust tool `tier-check` validates path-Tier
   intersection at write time. **Self-management cost: LOW.** Tier
   boundaries declared once; mostly passive enforcement; cross-Tier
   promotion (e.g., a tool moves from Tier 3 sandbox to Tier 0
   production at cutover) is event-driven not cycle-baseline.

4. **Terminology stratification — explicit different meanings per
   term (openclaw tool / skill / plugin distinction, I-O8 — tools
   = function calls, skills = Markdown-injected, plugins = npm
   packages)** — substrate-fit ABSENT. v1 has informal terms (tool
   / dispatch / question-for-eva / cycle issue / journal entry /
   cluster sub-shape) but they're not strictly stratified into
   formal types with declared meanings. The terms have meaning by
   convention rather than declaration. Design work: explicit
   terminology stratification (e.g., "Rust tool" = compiled binary,
   "skill" = Markdown-injected procedure, "playbook" = task-class
   template, "dispatch" = Copilot-assigned issue, etc.). Substrate
   via terminology table in v2 prompt + cross-references in tooling
   docs; no Rust tool needed but discipline of consistent naming
   is required. **Self-management cost: MODERATE.** Term-introduction
   needs ratification; cross-references between terms add cost;
   renaming requires sweep across artifacts.

5. **Role stratification — distinct named conceptual roles per
   responsibility (Voyager 4-agent architecture I-V1 — ActionAgent
   / CurriculumAgent / CriticAgent / SkillManager)** — substrate-fit
   ABSENT. v1 has named external roles (orchestrator, audit-
   orchestrator, Copilot, Eva) but no internal role stratification
   within the orchestrator session. The orchestrator does design +
   retrospective + dispatch + integration + housekeeping +
   research-absorption all in one role. Design work: substantial —
   internal role decomposition (e.g., a "designer" role for
   design-direction work, a "retrospective-evaluator" role for
   cold-reader work, an "absorber" role for audit critique
   integration). Substrate via prompt structure (per-role section)
   + per-role tool surface; potential Rust tool `role-load` shifting
   prompt section into focus per declared role. **Self-management
   cost: HIGH.** Internal roles require per-cycle role assignment +
   per-role capability declaration + role-conflict resolution +
   role-evolution discipline.

6. **Cost-tier stratification — model-per-task-class within an
   agent ecosystem (Voyager I-V2 — gpt-4 for novel reasoning,
   gpt-3.5-turbo for cached/derivative work; v2 candidate extends
   within Anthropic family Opus / Sonnet / Haiku per task-class)**
   — substrate-fit ABSENT. v1 uses Opus 4.7 for orchestrator
   across all task classes; no model-per-task-class assignment.
   Design work: per-task-class model declaration (e.g., Opus for
   design + retrospective + audit-absorption; Sonnet for
   dispatch-construction + housekeeping; Haiku for routine reads +
   integrity checks). Substrate via task-class declaration (cross-
   cast with sub-axis 2 Playbook templates) + workflow YAML model
   selection per task-class; potentially separate Rust tools per
   task-class invoking different models. **Self-management cost:
   MODERATE.** Per-task-class model assignment requires evaluation
   as new task-classes emerge; cost-quality trade-offs need
   calibration; model-version updates require sweep across
   task-class declarations.

7. **Autonomy-mode stratification — auto vs manual per-component
   with explicit human-in-the-loop method paths (Voyager I-V9;
   openclaw operator-tier-level)** — substrate-fit PARTIAL. v1
   has implicit autonomy modes: orchestrator cycles run
   automatically (auto); checkpoints require Eva approval (manual);
   Copilot dispatches require Eva manual assignment (manual); audit
   cron-triggered (auto). The pattern exists at multi-actor level
   but not formalized as per-component autonomy mode declarations.
   Design work: per-component autonomy mode declaration (auto /
   manual / hybrid) + explicit human-in-the-loop method paths
   (e.g., "checkpoint approval = Eva merges PR with
   `cutover-approved` label"). Substrate via component-registry
   (cross-cast with cluster B sub-shape 1) + per-component autonomy
   field. **Self-management cost: MODERATE.** Per-component
   declaration requires maintenance as components are added;
   mode transitions require explicit human-in-loop method
   definition; mode-conflict resolution.

8. **Capability-layer stratification — primitives vs LLM-composed
   compositions (Voyager I-V10 — control_primitives + skill-library
   compose primitives + earlier skills; openclaw tool/skill
   distinction is parallel)** — substrate-fit ABSENT. v1 has Rust
   tools (primitives) and orchestrator (LLM-composed via prompt
   invocation), but no formal layer stratification. The orchestrator
   invokes tools but doesn't compose them into a stratified skill
   library. There's no Markdown-injected "skill" tier between
   primitives and orchestrator-level composition. Design work:
   explicit layer stratification — primitives (Rust binaries,
   deterministic), skills (Markdown-injected procedures invoking
   primitives in known patterns), compositions (LLM-composed
   sequences invoking skills + primitives, novel-decision
   required). Substrate via filesystem convention + per-layer
   documentation; Rust tool `skill-load` for skill-tier injection.
   **Self-management cost: MODERATE.** Skill library evolves;
   primitives → skills → compositions promotion path needs
   per-skill maintenance; cross-layer references add cost.

**Substrate-fit summary across cluster F's 8 sub-axes:**
- **STRONG (0):** none
- **PARTIAL (3):** version stratification (1), capability-tier
  stratification (3), autonomy-mode stratification (7)
- **ABSENT (5):** task-class Playbook templates (2), terminology
  stratification (4), role stratification (5), cost-tier
  stratification (6), capability-layer stratification (8)

**Self-management cost summary across cluster F's 8 sub-axes:**
- **LOW (2):** version stratification (1), capability-tier
  stratification (3)
- **MODERATE (4):** terminology stratification (4), cost-tier
  stratification (6), autonomy-mode stratification (7),
  capability-layer stratification (8)
- **HIGH (2):** task-class Playbook templates (2), role
  stratification (5)

**Comparison to clusters A, B, D:** cluster F is the LEAST
substrate-aligned of the four clusters annotated to date (0 STRONG
/ 3 PARTIAL / 5 ABSENT vs cluster D's 2 STRONG / 6 PARTIAL / 1
ABSENT vs cluster B's 1 STRONG / 6 PARTIAL / 2 ABSENT vs cluster
A's 1 STRONG / 2 PARTIAL / 6 ABSENT). The 5 ABSENT count signals
substantial substrate-design work for any candidate adopting most
of cluster F. Self-management cost distribution: 2 LOW + 4 MODERATE
+ 2 HIGH — both HIGH-cost sub-axes (Playbook templates + role
stratification) are HIGH because they impose per-cycle decision
overhead (Playbook-selection per task; role-assignment per cycle).
Cluster F is the only cluster among the four to have both 0 STRONG
sub-shapes AND HIGH-cost sub-axes — the most expensive cluster
annotated to date on both axes.

**Phase 2 evaluation use of these annotations:**

- *Implementation effort* — weight by ABSENT count; cluster F's
  5 ABSENT sub-axes signal the largest substrate-design work of
  the four clusters annotated to date. Phase 2 candidates adopting
  most of cluster F inherit substantial implementation effort
  across multiple Rust tools + filesystem conventions.
- *Self-management cost* — Phase 2 candidates that adopt cluster
  F's HIGH-cost sub-axes (Playbook templates + role stratification)
  inherit per-cycle decision overhead. Candidates favoring LOW-cost
  sub-axes only (version stratification + capability-tier)
  minimize cost but also minimize stratification benefit.
- *"Free from substrate" inheritance* — version stratification
  (1) is the only cluster F sub-axis with substantial substrate
  inheritance (git-backed); capability-tier (3) and autonomy-mode
  (7) are convention-inherited (depend on declaration discipline).
- *Subset selection is critical* — per "Phase 2 implication:
  stratification axes are mostly independent and combinable",
  Phase 2 candidates can adopt any subset. With M1 annotations
  visible: the cheapest meaningful subset is sub-axes 1 + 3 + 7
  (3 PARTIAL, 0 ABSENT, 0 HIGH) — the candidate gets multi-axis
  stratification discipline at minimal cost. The maximally-
  stratified subset is all 8 (3 PARTIAL + 5 ABSENT, 4 MODERATE +
  2 HIGH) — substantial implementation and per-cycle cost for
  full stratification.
- *Unified-vs-decomposed question* (already named in cluster F
  principle paragraph) — with M1 annotations visible: the case
  for decomposing cluster F into narrower clusters is strengthened
  by the bimodal cost distribution (2 LOW + 2 HIGH). Decomposition
  would let Phase 2 candidates select clusters by cost rather than
  by sub-axis. One natural decomposition (informed by M1 + M2):
  "static stratification" (sub-axes 1 + 3 + 4 — version, tier,
  terminology) versus "dynamic stratification" (sub-axes 2 + 5 +
  6 — Playbook, role, cost-tier) versus "behavioral stratification"
  (sub-axes 7 + 8 — autonomy, capability-layer). Static = LOW-cost
  declaration; dynamic = HIGH-cost per-cycle assignment;
  behavioral = MODERATE-cost convention-driven.

**Phase 2 implication**: stratification axes are mostly independent
and combinable — Phase 2 candidates can adopt any subset.
Candidate-shape implication: a candidate adopting all 8 sub-axes
will have a substantially more layered architecture than a
candidate adopting 2-3. The unified-vs-decomposed question (one
cluster F vs four narrower clusters — see "Open structural
questions" below) is itself a Phase 2 candidate-shape discriminator.

## Cluster H: post-session feedback / cross-session learning

`[4-system convergent]` Cognition + openclaw + OpenAI harness +
Voyager. Cycle 69 confirmed cluster H upgrade from 3-system to
4-system via H1 hypothesis (capability-accumulation as 4th
sub-shape).

Artifacts produced at the end of a cycle / session serve as input
for the next cycle / session — the system improves across cycles
through structured feedback-loop artifacts. Four distinct sub-shapes
(rather than one canonical mechanism):

- **Tight-cycle meta-feedback** — Cognition Session Insights
  (I-C7): post-session reflective capsule for next session,
  machine-readable manifest plus human-readable prose; next
  cold-start reads first
- **Score-gated consolidation** — openclaw dreaming (I-O9):
  cron-driven sweep promotes short-term entries (`memory/.dreams/`)
  to long-term (`MEMORY.md`) gated on recall-frequency and
  query-diversity thresholds
- **Continuous-background gardening** — OpenAI harness doc-gardening
  + quality-grading (I-OH6): ongoing background process maintaining
  artifacts vs episodic sweep
- **Capability-accumulation** — Voyager skill-library
  (I-V5/I-V6/I-V10): successful cycles produce reusable
  *capabilities* (skills) for future cycles, not just meta-lessons —
  next cycle inherits *what to do* not just *what to think about*

**Phase 2 implication**: Phase 2 candidates have a *spectrum of
mechanism choices* rather than one canonical shape. The four
sub-shapes are not mutually exclusive and can compose. The minimal
v2 commitment is at least one cluster H mechanism (else no
cross-session learning); the maximal commitment is all four
(substantial implementation effort, rich learning surface).

## Other clusters (C, E, G, I) — brief

**Cluster C — lifecycle operations beyond resume `[4-system clean]`.**
AutoGen + LangGraph + Cognition + openclaw. Lifecycle vocabulary
extends beyond `resume`: terminate, reset, fork, replay, event-
trigger / reactive-bot-comment-pickup, stuck-watchdog with stale-
lane release. Sub-shapes (5). v1's only lifecycle operation is
`resume`; cluster C surfaces 4-7 additional operations Phase 2
candidates may adopt selectively.

**Cluster E — typed boundary semantics `[3-system convergent]`.**
LangGraph + AutoGen + openclaw via TypeBox. Typed contracts on
data-shape at boundaries. Two sub-shapes: schema-discipline at
process-boundaries (TypeBox single-source-of-truth produces
validators in TypeScript / Swift / JSON-Schema) vs typed-channel-
merger-rules at within-process boundaries (LangGraph per-key
reducers). Voyager doesn't add cleanly: structured CriticAgent
output `{success: bool, critique: str}` is parallel to cluster E
patterns but isn't foregrounded as architectural axiom — it's a
JSON contract between two agents, not boundary discipline.

**Cluster G — role-asymmetric context `[2-system convergent]`.**
Cognition + openclaw. Two sub-shapes: (1) clean-context-reviewer
(Cognition Devin Review, I-C4) — different roles get different
trust/context semantics, reviewer role inverts share-full-traces
default; (2) untrusted-prefix sub-agent injection (openclaw
active-memory I-O7) — sub-agent output enters the main context as
untrusted prefix, cannot instruct main agent. Voyager's CriticAgent
↔ ActionAgent has role asymmetry but at per-action retry granularity
(landed in cluster A as bounded-retry-with-feedback) rather than
per-session role-context-shape (cluster G). The boundary between
cluster A bounded-retry and cluster G role-asymmetric is thin in
some cases; see "Open structural questions" below.

**Cluster I — harness-enforced security/policy boundaries
`[2-system convergent, substrate-correlated]`.** openclaw + OpenAI
harness. Two sub-shapes: permission-policy enforcement at the
harness level decoupled from prompt-level rules (openclaw I-O1 —
default-deny on multiple capability surfaces, before_tool_call.
block-true terminal enforcement, plugin discovery/promotion gated
by ClawHub security review) + quality-policy enforcement via
mechanical linters with agent-readable error messages (OpenAI
harness — golden principles mechanically checked). Substrate
observation: cluster I is *absent in research-artifact substrates*
(Voyager runs locally with full environment access; no need for
harness-enforced policy). Cluster I is correlated with
cloud-anchored multi-actor environments. v1's substrate
(GitHub-Actions-anchored multi-actor with audit) places it CLOSE
to the cluster I correlation; Phase 2 candidates SHOULD weight
cluster I patterns highly even at 2-system convergence depth,
because the substrate alignment is strong.

## Phase 2 design-input from clusters

Phase 2 candidates compose against the cluster sub-shapes as
options. The clusters establish *architectural concerns* that need
addressing; the sub-shapes are *mechanism options* within each
concern. This catalogue surfaces the load-bearing combinations.

**v1-failure-mode mapping (inverted-from-retrospective lens).**
Specific cluster sub-shapes map to named v1 failure modes:

- **Stale-reference accumulation** (v1's `1-research.md` summary
  table drift across cycles 30-50; the cleanup work in cycles 60-61):
  cluster A sync-invariants-at-init (Voyager I-V4 — `state-sync-check`
  Rust tool runs at session start, validates cross-storage consistency,
  exits with structured remediation text on divergence)
- **Dispatch-fails-orchestrator-decides-per-cycle gap** (v1 has no
  formal retry mechanism for dispatch failures): cluster A bounded-
  retry-with-feedback (Voyager I-V7 — `dispatch-with-retry` Rust
  tool wrapping `gh issue create` with prior-attempt-context +
  failure-diagnostic + max-retries semantic)
- **Abandonment cascade** (v1 loses track of failed cycles): cluster
  A lane-aware FIFO queue with stuck-watchdog (openclaw I-O3 +
  I-O5)
- **Chronic-category currency loop** (v1 self-management consuming
  cycles): cluster F autonomy-mode stratification + cluster H
  capability-accumulation (skills accumulate across cycles, reducing
  the per-cycle decision surface)
- **Forgotten-failure** (v1 abandoned dispatches and dropped
  patterns disappear into journal entries): cluster B failure-as-
  first-class-artifact + cluster D failure-as-recorded-artifact
  (Voyager I-V8 + structured `state/failures/` consulted at next-
  cycle-composition decision)
- **Implicit cycle-phasing** (v1 cycle structure is procedural-
  prompt rather than typed-state): cluster A super-step semantics +
  termination predicates + phase-boundary state semantics
- **No-recovery-without-abort** (v1 cycle has terminate but not
  release-stuck-lane): cluster A stuck-session-watchdog (openclaw
  I-O5 — `diagnostics.stuckSessionWarnMs` detects stale lanes and
  releases them; NEW lifecycle operation distinct from
  terminate/reset/resume/fork/event-trigger)

This mapping is *direct evidence* that the clusters address v1's
named failure modes, not generic architectural improvements.

**Storage-architecture sub-shape combinations (cluster B, 9
sub-shapes).** Concrete Phase 2 candidate-shape combinations:
- *Parsimonious* (3 sub-shapes): component-local + active-surface-
  vs-monotonic-history + failure-as-artifact. The minimum that
  addresses v1's stale-reference and forgotten-failure failure modes.
- *Maximal* (all 9 sub-shapes): rich storage architecture but high
  implementation complexity; substantial Rust tool surface.
- *Minimal-departure-from-v1* (2 sub-shapes): repository-as-state +
  plans-as-forward-versioned-artifacts. v1 already uses repo-as-state
  implicitly; this combination formalizes the existing pattern with
  minimal architectural churn.

**Cycle-internal phasing sub-shape combinations (cluster A, 9
sub-shapes).** Strong Phase 2 candidate-discriminators:
- *Boot-discipline focus* (3 sub-shapes): sync-invariants-at-init +
  bounded-retry-with-feedback + stuck-session-watchdog. Addresses
  v1's three most-foregrounded failure modes (stale-reference,
  dispatch-recovery, abandonment) with minimum complexity.
- *Typed-state focus* (3 sub-shapes): super-step semantics +
  termination predicates + per-key reducers. v2 with typed cycle-
  internal-state, larger architectural shift.
- *Lane-discipline focus* (2 sub-shapes): lane-aware FIFO queue +
  stuck-session-watchdog. v2 with explicit cycle-internal work-
  type lanes (substantive vs absorption vs housekeeping) and
  per-lane discipline.

**Stratification axes (cluster F, 8 sub-axes).** Phase 2 candidates
can adopt any subset. The 8 axes are mostly independent and
combinable. Candidate-shape implication:
- *Heavy stratification* (all 8 axes): substantially layered
  architecture, larger implementation surface, but strong
  separation-of-concerns properties
- *Light stratification* (2-3 axes — typically role + cost-tier +
  capability-layer): smaller implementation, retains the most
  load-bearing axes for v1's substrate
- The unified-vs-decomposed question (one cluster F vs four
  narrower clusters) is itself a candidate-shape discriminator —
  see "Open structural questions"

**Post-session feedback mechanisms (cluster H, 4 sub-shapes).**
Phase 2 candidates can adopt any combination; the four sub-shapes
are not mutually exclusive. The minimal v2 commitment is at least
one cluster H mechanism (else no cross-session learning); the
maximal commitment is all four (substantial implementation effort
but rich learning surface). Candidate-shape implication:
- *Tight-feedback-only*: Cognition I-C7-style Session Insights
  capsule, smallest implementation, captures meta-lessons
- *Capability-only*: Voyager I-V5/I-V6/I-V10-style skill-library,
  captures *capabilities* not lessons — different surface
- *Tight + Capability*: hybrid — meta-lessons for "what worked"
  and capabilities for "what we can now do." Strong candidate
  default
- *All four*: rich cross-cycle learning with episodic-sweep,
  continuous-gardening, capability-accumulation, and tight-cycle
  meta-feedback. Substantial Rust tool surface

**Documentation honesty sub-shape combinations (cluster D, 9
sub-shapes).** v1 already does this work via retrospective +
journal. Cluster D additions for v2:
- **Walkback as first-class artifact protocol** (Cognition I-C1) —
  explicit protocol for revising prior positions: `POSITIONS.md`
  artifact with version semantics
- **Invariants vs derivations stratified within v2 prompt**
  (Cognition I-C2) — INVARIANTS section vs DERIVATIONS section so
  revising derivations doesn't destabilize axioms
- **Anti-pattern catalog with explicit non-permanence framing**
  (openclaw I-O2) — `ANTI-PATTERNS.md` alongside `POSITIONS.md`
  and `MIGRATIONS.md` (the documentation-discipline triad)
- **Failure-as-first-class-recorded-artifact** (Voyager I-V8) —
  structured failure records consulted at next-cycle-composition
  decision

**Cluster I substrate-fit observation (revised cycle 85 per audit#454
D1).** v1's substrate (GitHub-Actions-anchored multi-actor with audit)
is *close* to the cloud-anchored multi-actor substrate that
correlates with cluster I. Phase 2 candidates SHOULD evaluate cluster
I patterns for **substrate-fit**: the limited evidence we have is
about systems whose substrate looks like v1's substrate, so the
patterns transfer with substantively higher prior probability for
v1 than for substantively different substrates.

**Substrate-fit weighting is NOT corpus-depth weighting.** Cluster I
remains at 2-system convergence (openclaw + OpenAI harness). The
fact that cluster I now has 3 cross-cluster intersection disciplines
(D↔I, F↔I, E↔I) is **intersection coverage**, not evidence-depth
increase — the same 2 systems are analyzed from 3 different angles.
Phase 2 candidate evaluation should distinguish:

- **Substrate-fit weighting** (real, applies for v1's
  GitHub-Actions-anchored substrate): cluster I patterns have higher
  prior probability of transfer than corpus depth alone would suggest
- **Corpus-depth weighting** (unchanged): cluster I patterns are
  attested at 2-system convergence; cluster A and cluster B are at
  6-system convergence. Two candidates with similar cluster I
  coverage may differ on whether their architecture genuinely *needs*
  cluster I patterns (substrate-fit) vs adopting them aspirationally
  because the synthesis weights them highly

Specific mechanisms:
- **Harness-enforced tool-call policy decoupled from prompt-level
  rules** (openclaw I-O1) — Rust tools enforce what the LLM can
  invoke, not just prompt-level "don't do X" rules
- **Quality-policy enforcement via mechanical linters** (OpenAI
  harness) — agent-readable error messages on quality violations,
  CI-enforced

## Cross-cluster intersections (cycles 72 + 74 synthesis)

The within-cluster catalogues above (clusters A-I, ~9 sub-shapes
per foregrounded cluster) tell Phase 2 candidate authors *what
mechanisms exist for each architectural concern*. Cross-cluster
intersections tell Phase 2 candidate authors *how mechanisms
compose across architectural concerns to produce emergent
properties* — and where v1's failure modes are not just "missing
a sub-shape" but "missing the discipline of two clusters'
mechanisms composing at their boundary."

Cycle 70's hand-off named three priority intersections for cycle
72 synthesis: cluster A↔B (storage-discipline at cycle-boundary
moments), cluster F↔H (stratification of feedback mechanisms),
cluster D↔I (documentation-as-policy-enforcement). Cycle 72 flagged
four additional intersections (A↔C, B↔C, F↔I, E↔I) for future
synthesis; cycle 74 elevates them to full treatment. The seven
intersections together describe the cross-cluster architectural-
discipline surface Phase 2 candidates compose against. Each is
examined below with sub-patterns from the corpus and Phase 2
implications.

**Sub-pattern classification (cycle 85 audit#454 D2 absorption).**
Sub-patterns within each intersection are classified as one of:

- **Compositional**: two distinct mechanisms from the two clusters
  composing at their shared boundary. The intersection is the
  composition discipline. Compositional sub-patterns count toward
  intersection-coverage criteria for Phase 2 candidates.
- **Dual-cast**: one mechanism cast under both cluster labels —
  the same primitive serves dual roles in the two clusters. Dual-cast
  sub-patterns are evidence the cluster boundary is thin at that
  point and warrant revisitation; they should NOT count toward
  intersection-coverage criteria for Phase 2 candidates.

The compositional / dual-cast classification was added cycle 85 in
response to audit#454 D2. Three dual-cast cases identified
explicitly (A↔B sub-pattern 2; A↔C sub-pattern 2; E↔I sub-pattern
1); other sub-patterns default to compositional unless tagged.
Future synthesis cycles may surface additional dual-cast cases in
F↔I, B↔C, F↔H, D↔I that warrant similar tagging.

### A↔B: storage-discipline at cycle-boundary moments

Cluster A defines *when* in the cycle phase-boundary moments
occur; cluster B defines *what* gets persisted and *where*. Their
intersection is the discipline of cluster B storage writes
happening at named cluster A boundaries — which is what produces
consistent state across cycles.

Five sub-patterns from the corpus:

1. **Sync invariants asserted at session-init** (Voyager I-V4 +
   cluster B dual-storage discipline). Cluster A boundary (cycle
   init) + cluster B invariant (vectordb count vs JSON manifest
   count). The intersection IS a cluster A operation that enforces
   a cluster B consistency invariant. Without the intersection
   discipline, dual-storage drift accumulates silently across
   cycles.

2. **State-commit at end-of-super-step** (LangGraph I-L1 + I-L2).
   `[DUAL-CAST]` Cluster A super-step boundary + cluster B per-key
   reducer rules. Each super-step ends with a coordinated
   state-commit through reducers; cluster B reducer rules ARE
   cluster A boundary semantics — same LangGraph mechanism cast
   under both cluster labels. Without explicit super-step
   boundaries, reducer rules fire at fuzzy points and produce
   write-write conflicts. **Dual-cast classification (cycle 85
   audit#454 D2):** this sub-pattern does not count toward A↔B
   intersection-coverage criteria for Phase 2 candidates because
   the boundary between cluster A super-step and cluster B
   reducer-rule is thin at this point — they're the same
   primitive serving dual roles. Phase 2 candidates that adopt
   LangGraph-style super-step+reducer don't need separate A↔B
   intersection discipline at this sub-pattern's location.

3. **Failure-record-write at retry-exhaustion** (Voyager I-V7 +
   I-V8). Cluster A bounded-retry-with-feedback (retry exhaustion
   is a defined phase boundary) + cluster B failure-as-first-
   class-artifact (the write target). Retry exhaustion triggers a
   structured cluster B failure record. Without the intersection,
   exhausted retries produce ad-hoc journal mentions that get
   forgotten by the next cycle's composition decision.

4. **Watchdog-release-with-state-cleanup** (openclaw I-O5 +
   cluster B). Cluster A stuck-watchdog (lifecycle operation) +
   cluster B lane-state-and-failure-record (storage cleanup
   target). When the watchdog releases a stale lane, cluster B
   storage is coordinated cleanup (lane state cleared, optional
   failure record written). Without the intersection,
   watchdog-released lanes leave dangling state that the next
   cycle must reason about.

5. **Component-local persistence loaded at init** (AutoGen I-6 +
   Voyager I-V3 cross-system parallel). Cluster A init phase +
   cluster B component-local persistence per agent. Each component
   loads its own state at init; cluster A super-step boundary +
   cluster B distributed-storage. Without the intersection, init
   loads from inconsistent points and produces partial-state
   sessions where some components have current state and others
   have stale state.

**v1 failure modes addressed by A↔B**:

- **Stale-reference accumulation** (cleanup work cycles 60-61):
  cluster B writes happening at fuzzy cluster A boundaries
  produce stale references; sync-invariants-at-init (intersection
  sub-pattern 1) catches drift at the init boundary
- **Abandonment cascade**: dispatches dropped between sessions
  because cluster A has no explicit cycle-end boundary triggering
  cluster B failure-record write; intersection sub-pattern 3
  (failure-record-write-at-retry-exhaustion) provides the
  discipline
- **Forgotten-failure**: failed cycles disappear into journal
  entries because cluster B failure-as-artifact discipline isn't
  triggered at any cluster A boundary; intersection sub-patterns
  3 + 4 provide both the trigger and the write

**Phase 2 implication**: A↔B is the highest-priority intersection
for v1 failure-mode coverage. v2 candidates that adopt cluster A
boundaries AND cluster B mechanisms but **not** their intersection
will have super-step semantics + storage architecture but the
same stale-reference / abandonment / forgotten-failure failure
modes as v1. The intersection discipline (cluster B writes happen
at named cluster A boundaries) is what closes the gap. v2
candidate-shape implication: every cluster B write target should
name the cluster A boundary at which it fires; orphaned cluster
B writes (no associated boundary) are smell of within-cluster-
mechanism without intersection-discipline.

### F↔H: stratification of feedback mechanisms

Cluster F provides 8 stratification axes (version, task-class,
capability-tier, terminology, role, cost-tier, autonomy-mode,
capability-layer). Cluster H provides 4 post-session feedback
sub-shapes (tight-cycle meta-feedback, score-gated consolidation,
continuous-background gardening, capability-accumulation). Their
intersection: feedback mechanisms differentiated along F-axes —
different feedback frequency, authority, gating, scope, or
content for different tiers.

Five sub-patterns from the corpus:

1. **Capability-layer × capability-accumulation** (Voyager I-V10 +
   I-V5/I-V6). Cluster F capability-layer (primitives vs
   LLM-composed compositions) + cluster H capability-accumulation.
   The capability layer determines what gets accumulated as
   feedback. Primitives are hand-written and don't accumulate;
   compositions/skills accumulate. Stratification of WHICH layer
   produces capability-accumulation feedback.

2. **Autonomy-mode × tight-cycle-vs-continuous** (Voyager I-V9 +
   openclaw operator-tier + cluster H tight + continuous).
   Cluster F autonomy-mode (auto vs manual) determines feedback
   frequency. Manual mode → tight-cycle Session-Insights-style
   (operator reads between cycles); auto mode → continuous-
   background gardening or score-gated consolidation (no human
   reads each cycle). Different autonomy modes get different
   feedback shapes.

3. **Cost-tier × score-gated consolidation** (Voyager I-V2 +
   openclaw I-O9). Cluster F cost-tier (model-per-task-class) +
   cluster H score-gated consolidation. High-cost tier (gpt-4 /
   Opus) needs gating to amortize cost across cycles; low-cost
   tier (gpt-3.5-turbo / Haiku) might do continuous-without-gating
   because per-action cost is low. Stratification of feedback
   economics.

4. **Role × clean-context-reviewer feedback** (Cognition I-C4 +
   cluster F role-stratification + cluster G role-asymmetric).
   When role-stratified architecture has a reviewer role
   (Cognition Devin Review, I-C4), the reviewer's feedback IS the
   cluster H mechanism for that role. Reviewer-role feedback
   (clean-context, no shared traces with action role) is shaped
   differently from action-role self-reflection. Stratification
   of feedback authority and trust.

5. **Task-class × Playbook-derived feedback** (Cognition I-C6 +
   cluster H tight-cycle). Cluster F task-class stratification
   (Playbook templates per task-class with outcome + steps +
   advice + forbidden) + cluster H tight-cycle. Each task class
   gets its own Playbook-derived meta-feedback shape; different
   task classes get different feedback templates and different
   `forbidden` lists. Stratification of feedback content per
   task class.

**v1 failure mode addressed by F↔H**: chronic-category currency
loop. v1 has uniform feedback for all chronic categories
regardless of category urgency, autonomy mode, or capability-
layer relevance. F↔H stratification produces differentiated
cross-cycle learning — high-priority chronic categories with
elevated feedback frequency, low-priority categories with
score-gated consolidation, capability-accumulating categories
at the appropriate layer.

**Phase 2 implication**: v2 candidates with cluster F sub-axes
AND cluster H mechanisms but **not** their intersection produce
uniform-feedback-across-tiers (v1's pattern). The intersection
is where stratification produces differentiated cross-cycle
learning. v2 candidate-shape implication: light-stratification
candidates (2-3 F sub-axes) need 1-2 H sub-shapes; heavy-
stratification candidates (all 8 F sub-axes) need at least 3
H sub-shapes to match the discrimination granularity. Orphaned
F sub-axes (axis with no associated H mechanism differentiation)
are smell of stratification-without-feedback-discipline.

### D↔I: documentation-as-policy-enforcement

Cluster D produces honest documentation (anti-pattern catalogs,
walkbacks, invariants/derivations stratified, failure records,
MIGRATIONS.md). Cluster I produces harness-enforced security/
policy boundaries (default-deny, before_tool_call.block-true,
mechanical linters with agent-readable error messages). Their
intersection: documentation that is *machine-readable* and feeds
cluster I enforcement.

Five sub-patterns from the corpus:

1. **Anti-pattern catalog × mechanical-linter** (openclaw I-O2
   ANTI-PATTERNS.md + OpenAI harness mechanical linters). The
   anti-pattern catalog is documentation (cluster D) AND the
   source of linter rules (cluster I). The intersection IS the
   catalog being machine-readable enough to drive enforcement.
   Without the intersection, anti-patterns are advisory only —
   the catalog says "don't do X" but nothing prevents X.

2. **VISION.md "What We Will Not Merge" × ClawHub security
   review** (openclaw). Documentation declaration of forbidden
   patterns (cluster D) + harness-enforced policy at PR/dispatch
   time (cluster I). The "Will Not Merge" list IS the
   enforcement boundary; documentation and policy are the same
   artifact at different consumption layers.

3. **POSITIONS.md × tool-call validation** (Cognition I-C1
   walkback protocol + cluster I). Versioned positions
   documented (cluster D walkback as first-class artifact) AND
   enforced as constraints on tool calls (cluster I). The
   intersection: position changes propagate to enforcement
   without manual policy update.

4. **Invariants/Derivations stratified × axiom-enforcement**
   (Cognition I-C2 + cluster I). Invariants section is durable
   axiomatic documentation (cluster D invariants/derivations
   stratification) + enforcement that derivation-changes don't
   silently change axioms (cluster I machine check).
   Intersection: stratified documentation enables stratified
   enforcement (strict on invariants, advisory on derivations).

5. **Failure-record × failure-pattern-detection-watchdog**
   (Voyager I-V8 + cluster I). Structured failure records as
   documentation (cluster D failure-as-recorded-artifact)
   consumed by a watchdog/lint that detects re-occurring failure
   patterns (cluster I). Documentation-becomes-policy via
   repeat-detection: a documented failure that recurs N times
   becomes an enforced anti-pattern.

**v1 failure mode addressed by D↔I**: anti-patterns documented
in retrospective but not enforced. v1's retrospective cycle
catches anti-patterns retrospectively (lagging enforcement); the
system can re-introduce a documented anti-pattern and only catches
it on the next retrospective cycle. D↔I intersection produces
immediate enforcement at policy boundaries. v1's specific
instance: the chronic-category currency loop was a documented
anti-pattern (multiple retrospective cycles named it) that
re-occurred across many subsequent cycles because no I-level
enforcement prevented the orchestrator from re-entering it.

**Phase 2 implication**: v2 candidates with cluster D
documentation but no cluster I enforcement get
*lagging-corrective* behavior (retrospective-style catch). v2
candidates with both produce *immediate-prevention* behavior
(catch at point-of-violation). The intersection is what
determines whether documentation is advisory or load-bearing. v2
candidate-shape implication: candidates that adopt cluster D
anti-pattern catalogs MUST also adopt at least one cluster I
enforcement mechanism (mechanical linter, watchdog, harness-
policy) to convert documentation into prevention; otherwise the
catalog is decorative.

### A↔C: lifecycle operations at named phase boundaries

Cluster A defines *when* in the cycle phase-boundary moments occur
(termination predicates, super-step boundaries, watchdog detection).
Cluster C names *what kinds of lifecycle ops* exist beyond resume
(terminate, reset, fork, replay, reactive event-trigger, stuck-watchdog
with stale-lane release). Their intersection: lifecycle ops execute AT
named cluster A boundaries — typed cycle-internal vocabulary instead of
ad-hoc lifecycle execution.

Five sub-patterns from the corpus:

1. **Termination-predicate × terminate operation** (AutoGen I-3 —
   within-system pair). Cluster A termination predicate IS the trigger
   condition; cluster C terminate IS the lifecycle op the predicate
   triggers. AutoGen architecturally pairs them: predicate-fires →
   terminate-runs. Without the intersection, terminate fires on fuzzy
   criteria (e.g., turn-limit hit) and termination predicates evaluate
   without triggering anything; the cycle terminates via timeout or
   external interrupt rather than typed completion.

2. **Stuck-watchdog × lane-release** (openclaw I-O5 — within-system
   pair). `[DUAL-CAST]` Cluster A I-O5 is the watchdog as
   phase-boundary detection (recovery-without-abort lifecycle
   operation per cycle 70 phrasing); cluster C I-O5 is the same
   mechanism cast as lane-release lifecycle op. The same I-O5
   implication is BOTH a cluster A boundary-detection sub-shape AND
   a cluster C lifecycle-op sub-shape — same openclaw mechanism cast
   under both cluster labels. The intersection IS what makes I-O5
   actionable: watchdog without lane-release detects stuckness
   silently; lane-release without watchdog has no trigger.
   Recovery-without-abort emerges only at the intersection. **Dual-
   cast classification (cycle 85 audit#454 D2):** this sub-pattern
   does not count toward A↔C intersection-coverage criteria for
   Phase 2 candidates because the boundary between cluster A
   watchdog-detection and cluster C lane-release lifecycle-op is
   thin at this point — they're the same primitive serving dual
   roles. Phase 2 candidates that adopt openclaw-style I-O5 don't
   need separate A↔C intersection discipline at this sub-pattern's
   location.

3. **Super-step boundary × fork** (LangGraph I-L1 super-step + I-L4
   time travel — within-system pair). Cluster A super-step boundary
   creates the clean-state moment; cluster C fork operates AT that
   moment to produce a checkpoint or branch. Without the intersection,
   mid-super-step fork produces inconsistent state (some channels
   merged, others not) and downstream replay can't reliably reproduce.

4. **Super-step boundary × replay** (LangGraph I-L1 + I-L4). Cluster
   A super-step boundary IS the replay point; cluster C replay reads
   state at a super-step checkpoint and re-executes downstream.
   Without the intersection, replay reads fuzzy mid-super-step state
   and produces non-deterministic re-execution. The intersection makes
   time-travel architecturally trivial — every super-step boundary
   is automatically a valid replay point.

5. **Phase-boundary × reactive event-trigger** (openclaw reactive-bot-
   comment-pickup + cluster A boundary). Reactive event-triggers
   (cluster C) introduce an out-of-cycle boundary type; the
   orchestrator's normal cluster A super-step semantics extend to
   handle "event arrived between cycles." Without the intersection,
   event-triggers either pre-empt the current super-step (chaos) or
   queue silently until the next cycle (latency without typed
   handling). The intersection produces typed event-handling: events
   become first-class boundaries with their own state-write semantics.

**v1 failure modes addressed by A↔C**:

- **Implicit cycle-phasing** (v1's cycle structure is procedural-prompt
  rather than typed-state): A↔C sub-pattern 1 (termination-predicate
  × terminate) provides explicit phase-boundary semantics with named
  termination triggers
- **No-recovery-without-abort** (v1's stuck-dispatch handling at cycle
  71 — orchestrator filed a malformed dispatch and only diagnosed it
  8 cycles later because the watchdog was diagnostic-only, not
  action-taking): A↔C sub-pattern 2 (stuck-watchdog × lane-release) is
  the recovery mechanism v1 lacks — diagnostic comment alone is not
  recovery, lane-release IS recovery
- **Abandonment cascade** (v1 loses track of failed cycles between
  sessions): A↔C sub-pattern 5 (phase-boundary × reactive event-
  trigger) provides typed mechanism for cross-session event handling
  — failed-cycle events trigger structured next-cycle pickup rather
  than relying on next-cycle's cold-read of the journal

**Phase 2 implication**: A↔C is the *typed-lifecycle* enabler. v2
candidates with cluster A boundaries AND cluster C lifecycle ops but
**not** their intersection get ad-hoc execution semantics —
terminate/fork/replay/reset exist but fire at fragile timing
assumptions. The intersection discipline (each cluster C op declares
the cluster A boundary at which it fires) is what makes lifecycle
vocabulary architecturally trustworthy. v2 candidate-shape implication:
every cluster C lifecycle op should name its cluster A boundary (e.g.,
terminate fires at end-of-super-step on predicate match; fork fires at
named checkpoint boundary). Orphan cluster C ops (no associated cluster
A boundary) are smell of lifecycle-without-phase-discipline.

### B↔C: storage operations on lifecycle-event boundaries

Cluster B defines *what gets persisted and where* (component-local
persistence, active-vs-monotonic, failure-as-artifact, semantic-
retrieval, plans-as-versioned-artifacts). Cluster C defines *what
lifecycle ops exist*. Their intersection: cluster C ops produce
cluster B writes — fork branches storage, replay reads storage to
construct context, reset truncates storage at a checkpoint, watchdog
writes failure-record before lane-release, event-trigger loads relevant
component state.

Five sub-patterns from the corpus:

1. **Fork × component-local-persistence-branch** (LangGraph I-L4 fork
   + Voyager I-V3 component-local persistence). Cluster C fork
   operation needs cluster B component-local persistence to support
   efficient branching: each component branches independently rather
   than the monolithic state being copied. Without the intersection,
   fork copies all state (expensive) OR creates inconsistent branches
   where some components are forked and others aren't.

2. **Replay × failure-record-as-context-source** (Voyager I-V8
   failure-as-first-class-artifact + cluster C replay). Replay
   reconstructs cycle context partly from cluster B failure-records —
   "what was tried last time, what failed, what was recorded." Without
   the intersection, replay has no failure-context and re-tries
   already-failed approaches. Voyager's curriculum agent reads
   failed_tasks.json plus completed_tasks.json before next-task
   selection — this IS the B↔C intersection in operation.

3. **Reset × active-surface-vs-monotonic-history** (Voyager I-V5
   active-vs-monotonic + cluster C reset). Cluster C reset truncates
   the active retrieval surface but preserves monotonic-history
   (cluster B active-vs-monotonic discipline is what makes reset safe).
   Without the intersection, reset either destroys history (loss of
   failure-record evidence + retrospective material) OR doesn't
   actually reset (stale active surface persists alongside new state).

4. **Watchdog × failure-record-write** (openclaw I-O5 watchdog +
   cluster B I-V8 failure-as-artifact). Watchdog detection triggers
   cluster B failure-record write before lane-release. Without the
   intersection, watchdog releases lanes silently → loss of v2
   design-input evidence. The v1 cycle-71 stuck-dispatch incident
   is well-described by this intersection in retrospect (cycle 85
   audit#454 D4 corrective): the orchestrator self-diagnosed the
   malformed dispatch and produced a diagnosis comment on the
   dispatch issue, but no structured failure-record persisted to a
   known storage surface for next-cycle composition decisions; the
   diagnosis lives only in unstructured issue comments and the
   journal. Note: B↔C synthesis happened cycle 72, post-incident;
   the cycle-71 incident is post-hoc described by B↔C, not
   predicted by it. Cycle 85 J-Q(a) discipline applied: empirical
   observation (cycle 72 B↔C describes cycle 71 incident pattern)
   distinguished from claim about prediction (cycle 71 was
   understood in B↔C terms) — the former is licensed, the latter
   is not.

5. **Event-trigger × per-component resume opt-in** (openclaw reactive
   event-trigger + Voyager I-V3 per-component resume). Reactive
   event-triggers (cluster C) need cluster B per-component resume:
   the event handler loads only its own component state, not central
   state. Without the intersection, event handlers reload everything
   (slow + interferes with other components) OR run on stale state
   (handler doesn't know its component-state is out of date).

**v1 failure modes addressed by B↔C**:

- **Forgotten-failure** (v1's failed dispatches and abandoned cycles
  disappear into journal entries): B↔C sub-pattern 4 (watchdog ×
  failure-record-write) provides the typed mechanism — watchdog
  detection IS a failure-record-write trigger, producing structured
  artifact rather than journal prose
- **No-experiment-branching** (v1 can't fork its state to try
  alternatives without disrupting in-flight cycle work): B↔C
  sub-pattern 1 (fork × component-local-persistence-branch) enables it
- **Reset-destroys-history concern** (any naive v1 reset would lose
  retrospective evidence): B↔C sub-pattern 3 (reset × active-vs-
  monotonic) shows reset can preserve history by stratifying the
  storage surfaces

**Phase 2 implication**: B↔C is the *operational vocabulary on storage*
intersection. Cluster B mechanisms without cluster C lifecycle ops
produces "frozen" storage architecture — write semantics but no
operations on top. Cluster C ops without cluster B coordination produces
ad-hoc state mutations divorced from the architectural storage layer.
The intersection produces typed lifecycle operating on typed storage.
v2 candidate-shape implication: any cluster C lifecycle op must declare
its cluster B effect (which storage surfaces it touches, what it
writes, what it preserves vs truncates). Orphan cluster C ops without
cluster B effect specification are smell of lifecycle-divorced-from-
storage.

### F↔I: tier-stratification of harness enforcement

Cluster F provides 8 stratification axes (version, task-class,
capability-tier, terminology, role, cost-tier, autonomy-mode,
capability-layer). Cluster I provides 2 enforcement sub-shapes
(permission-policy default-deny harness + quality-policy mechanical
linters). Their intersection: enforcement strictness stratified by
F-axis tier — different tiers get different policy strictness,
different roles get different validation, different cost-tiers get
different quality-gates.

Five sub-patterns from the corpus:

1. **Capability-tier × default-deny-harness** (openclaw I-O6
   capability-tier + I-O1 default-deny — within-system pair). Tier 1
   read-only enforced strictly via default-deny on writes; Tier 3
   autonomous-with-standing-orders has substantially fewer harness
   restrictions. The intersection IS what makes graduated autonomy
   possible. Without the intersection, blanket policies — either
   everyone strict (Tier 3 can't operate) or everyone permissive
   (Tier 1 has no protection).

2. **Cost-tier × quality-policy enforcement** (Voyager I-V2 cost-tier
   + OpenAI harness mechanical linters). Different cost tiers warrant
   different quality-policy strictness — high-cost tier (Opus / gpt-4)
   gets full mechanical-linter validation since cost amortizes across
   rare-but-deep usage; low-cost tier (Haiku / gpt-3.5-turbo) gets
   advisory-only since per-action cost is low and running full
   validation on every action wastes capacity. Without the
   intersection, uniform quality enforcement → either wastes high-cost
   capacity on simple validations or over-validates simple low-cost
   output.

3. **Role × tool-call validation per role** (Cognition I-C4 reviewer
   role + cluster I tool-call validation, with cluster G role-
   asymmetric context as substrate). Reviewer role has clean-context
   (no shared traces with action role) AND different tool-call
   validation: review-mode tools (read repo, read traces, file
   critique) vs action-mode tools (write repo, dispatch issues, mutate
   state). Stratification of policy by role. Without the intersection,
   all roles share validation policy and reviewer can call action-tools
   that compromise review independence.

4. **Autonomy-mode × policy-strictness** (Voyager I-V9 autonomy-mode
   toggle + cluster I). Manual autonomy-mode → operator approves
   before each significant action (less harness-enforcement needed
   because operator-in-loop catches violations); auto autonomy-mode
   → strict harness-enforcement (operator absent, harness must
   substitute). Stratification of enforcement strictness by autonomy
   mode. Without the intersection, auto and manual share policy →
   either auto under-enforced (operator absent + lax harness) or
   manual over-blocked (operator present + strict harness creates
   friction).

5. **Capability-layer × promotion-gate** (Voyager I-V10 capability-
   layer composition + openclaw plugin-discovery/promotion gated by
   ClawHub security review). Primitives layer = trusted (hand-written
   by humans, code-reviewed); skills/compositions layer = needs
   promotion-gate (LLM-generated, must pass security review before
   promotion to active retrieval). Stratification of promotion-policy
   by capability layer. Without the intersection, all layers share
   semantics — primitives needlessly gated (slow evolution) OR
   LLM-compositions promoted unreviewed (security risk).

**v1 failure modes addressed by F↔I**:

- **Blanket-policy-with-no-tier-discrimination** (v1's tool permissions
  apply uniformly across the orchestrator regardless of task class,
  role, autonomy mode, or capability layer): F↔I sub-patterns 1 + 4
  are the direct fix — tier-stratified default-deny + autonomy-mode-
  stratified strictness
- **Reviewer-action-conflation** (v1 has no role-stratification, so
  any role can call any tool): F↔I sub-pattern 3 (role × tool-call
  validation) is the mechanism v1 lacks
- **Capability-promotion-without-review-gate** (any v2 with LLM-
  generated skills that doesn't have F↔I sub-pattern 5 will
  accumulate unreviewed compositions in production): F↔I makes the
  gate explicit

**Phase 2 implication**: F↔I is the *graduated-autonomy* enabler. v2
candidates with cluster F sub-axes AND cluster I enforcement but no
F↔I intersection get blanket-strict OR blanket-permissive policy →
either the orchestrator can't function or the system is unsafe. F↔I
intersection produces tiered autonomy: wide tier-1 protection + light
tier-3 enablement. v2 candidate-shape implication: every cluster I
enforcement mechanism should declare which cluster F sub-axis tiers
it applies to (e.g., default-deny strict on tier 1, advisory on tier 3;
mechanical linter strict on cost-tier-Opus, advisory on cost-tier-
Haiku). Blanket cluster I rules without F-axis stratification are
smell of enforcement-without-tier-discipline.

### E↔I: typed boundary discipline as enforcement substrate

Cluster E provides typed boundary semantics (2 sub-shapes: schema-
discipline at process-boundaries via TypeBox single-source-of-truth
+ typed-channel-merger-rules via LangGraph per-key reducers). Cluster
I provides harness enforcement (2 sub-shapes: default-deny permission-
policy + mechanical-linter quality-policy). Their intersection: typed
boundaries are *the substrate enforcement runs on* — schema validation
IS policy enforcement; per-key reducer rules ARE merge-time policy.
Typed boundaries make I-level enforcement mechanical rather than
hand-rule-driven.

Four sub-patterns from the corpus:

1. **TypeBox schemas × harness validation** (openclaw — within-system
   pair). `[DUAL-CAST]` TypeBox is single-source-of-truth that
   produces validators in TypeScript / Swift / JSON-Schema; cluster I
   enforces those validators at boundaries (default-deny on schema
   mismatch). The intersection IS the substrate: schema *is* the
   policy, validator *is* the harness check — same TypeBox primitive
   serving dual roles. Without the intersection, harness has
   hand-written validation rules that drift from TypeBox schemas →
   silent acceptance of invalid data or false rejection of valid
   data. **Dual-cast classification (cycle 85 audit#454 D2):** this
   sub-pattern does not count toward E↔I intersection-coverage
   criteria for Phase 2 candidates because the boundary between
   cluster E typed-contract and cluster I harness-enforcement is
   thin at this point — they're the same TypeBox primitive serving
   dual roles. Phase 2 candidates that adopt openclaw-style TypeBox
   single-source-of-truth don't need separate E↔I intersection
   discipline at this sub-pattern's location.

2. **Per-key reducer rules × policy-at-merge-time** (LangGraph I-L2
   per-key reducers + cluster I). Per-key reducers are typed merge-
   rules; cluster I treats them as merge-time policy boundaries —
   policy fires at the merge, not after. Without the intersection,
   merge happens then policy validates retrospectively → expensive
   rollback when invalid merges propagate.

3. **Schema discipline × default-deny on schema mismatch** (openclaw
   E + I-O1 default-deny). Default-deny semantics: data not matching
   schema is rejected by default rather than coerced to fit. Without
   the intersection, mismatches are silent (data drift accumulates)
   or coerced (data shape silently changes from declared shape).

4. **Structured tool output × downstream-typed consumption** (cross-
   system; AutoGen typed tool args + Cognition I-C4 review handoff +
   cluster I). Structured tool output IS contract consumed by next
   agent or harness check — not a JSON suggestion that downstream
   consumers parse defensively, but a typed message validated at
   handoff. Without the intersection, structured output is decorative;
   type drift accumulates as different agents make different parsing
   assumptions.

**v1 failure modes addressed by E↔I**:

- **Ad-hoc validation** (v1 has hand-written validation at various
  boundaries — state.json, dispatch payloads, journal entries — and
  these drift independently): E↔I sub-pattern 1 (schemas as harness
  source-of-truth) eliminates hand-written validation
- **Silent-data-coercion** (v1 has no default-deny on schema
  mismatches, so type errors are silently coerced rather than
  caught): E↔I sub-pattern 3 catches them
- **Stale-reference accumulation** (cycles 60-61 cleanup work — v1's
  summary table references drifted across cycles because no merge-
  time validation): E↔I sub-pattern 2 (merge-time policy) provides
  the mechanism

**Phase 2 implication**: E↔I is the *enforcement-on-typed-substrate*
intersection. v2 candidates with cluster E typed boundaries but no
cluster I enforcement get typed-boundaries-with-no-enforcement
(typing-as-decoration); v2 candidates with cluster I enforcement but
no cluster E typed boundaries get hand-written-validators-that-drift.
The intersection produces *typed boundaries that ARE enforcement
boundaries* — single source of truth for shape and policy. v2
candidate-shape implication: every cluster E typed boundary should
be enforceable by cluster I machinery; schema feeds harness validator
without manual translation. Cluster E patterns without I-level
enforcement integration are decorative typing.

Note: cluster E is 3-system convergent (LangGraph + AutoGen +
openclaw), substantially less foregrounded than the F↔H or D↔I
intersection clusters. Sub-pattern density (4 vs 5 elsewhere) reflects
the smaller corpus. PAI deeper-read [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842)
or oh-my-codex deeper-read [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
returns may surface additional sub-patterns within E↔I.

### Meta-observation: cross-cluster as next-layer v2 design-input

The cross-cluster layer expresses *architectural-discipline-
emergent-from-mechanism-composition* rather than mechanism-by-
mechanism choices. Phase 2 candidates that adopt within-cluster
sub-shapes WITHOUT thinking about cross-cluster intersections
will produce systems where the mechanisms exist but don't compose
well. Concrete failure modes such candidates produce:

- *cluster A super-step semantics + cluster B storage but no
  A↔B intersection* → super-step boundaries don't trigger
  storage commits → same stale-reference failure as v1
- *cluster F sub-axes + cluster H mechanisms but no F↔H
  intersection* → uniform feedback across tiers → same
  chronic-category currency loop as v1
- *cluster D anti-pattern catalog + cluster I harness but no
  D↔I intersection* → anti-patterns documented but not
  enforced → same lagging-retrospective enforcement as v1
- *cluster A boundaries + cluster C lifecycle ops but no A↔C
  intersection* → ad-hoc lifecycle execution → same
  no-recovery-without-abort failure as v1 cycle 71 stuck-dispatch
- *cluster B storage + cluster C lifecycle ops but no B↔C
  intersection* → lifecycle ops divorced from storage → same
  forgotten-failure pattern as v1
- *cluster F sub-axes + cluster I enforcement but no F↔I
  intersection* → blanket-policy without tier discrimination →
  either orchestrator under-empowered or system unsafe
- *cluster E typed boundaries + cluster I enforcement but no
  E↔I intersection* → either decorative typing or hand-written
  validators that drift → same ad-hoc validation as v1

The within-cluster sub-shape catalogues from cycle 70 provide
WHAT mechanisms exist; the cross-cluster intersections from
cycles 72 + 74 provide HOW mechanisms compose to produce emergent
architectural properties. v2 candidate evaluation can be sharpened
by intersection coverage: how many of the seven cross-cluster
intersection disciplines (A↔B, F↔H, D↔I, A↔C, B↔C, F↔I, E↔I) does
the candidate's architecture explicitly address?

**Cluster I substrate-correlation revisited.** Cycle 70's
observation that cluster I is substrate-correlated to v1's
substrate (GitHub-Actions-anchored multi-actor with audit) is
strengthened by D↔I, F↔I, and E↔I intersections — three of the
seven elevated cross-cluster intersections involve cluster I.
After cycle 74 elevated F↔I and E↔I from flagged-only to full
treatment, the substrate-correlation argument hardens: every
cluster I sub-shape now has at least one full intersection
discipline (D↔I for documentation-as-policy, F↔I for tier-
stratification, E↔I for typed-substrate enforcement). Phase 2
candidates SHOULD weight cluster I patterns highly even at
2-system convergence depth, AND they should weight all three
cluster I intersections (D↔I, F↔I, E↔I) as part of the
substrate-fit evaluation. v1 substrate's GitHub-Actions-multi-
actor-with-audit shape correlates strongly with cluster I's
canonical substrate; this is not a peripheral cluster for v2.

**Cluster C as lifecycle-vocabulary linchpin.** After cycle 74,
cluster C now participates in two full intersections (A↔C and
B↔C), elevating its load-bearing role for Phase 2 candidates
that adopt rich lifecycle vocabulary. The pair A↔C + B↔C
combined produces *typed lifecycle operating on typed storage* —
v2 candidates that adopt cluster C lifecycle ops without both
intersections will produce ad-hoc lifecycle that mutates storage
unpredictably. Cluster C's 4-system clean depth (no Voyager
contribution) places it at the same depth as cluster H.

**Symmetric-vs-asymmetric intersection observation.** Some
intersections are symmetric — both clusters contribute mechanisms
that compose at their boundary (A↔B is symmetric: cluster A
boundaries trigger cluster B writes AND cluster B writes inform
cluster A boundary semantics — sub-pattern 2 makes this explicit
with "cluster B reducer rules ARE cluster A boundary semantics").
A↔C is **mixed-symmetry**: sub-pattern 2 (stuck-watchdog ×
lane-release) is genuinely symmetric because the same I-O5
implication is cast as both a cluster A boundary-detection
sub-shape and a cluster C lifecycle-op sub-shape; sub-patterns 1
and 5 are weakly bidirectional (termination-predicate triggers
terminate, with the converse "terminate establishes a phase
boundary" being somewhat tautological by definition; phase-boundary
semantics extend to handle reactive event-triggers introduced by
cluster C); sub-patterns 3 and 4 (super-step boundary × fork or
replay) are clearly A→C asymmetric. A↔C is therefore less
symmetric than A↔B overall, but contributes one genuinely
symmetric data point (the I-O5 dual-cluster cast) that A↔B does
not have. Other intersections are asymmetric — one cluster's
artifacts feed the other's mechanisms (D↔I is asymmetric: cluster
D documentation feeds cluster I enforcement, not the reverse;
F↔I is asymmetric: F provides tier definitions, I consumes them;
E↔I is asymmetric: E schemas feed I validators; B↔C is mostly
asymmetric with a partial reverse channel — C ops trigger B
writes, except for failure-record-as-replay-context where B
reads inform C replay). Of the seven intersections, A↔B is
symmetric; A↔C is mixed-symmetry; F↔H is mostly symmetric
(feedback shapes respond to stratification AND vice versa); the
remaining four (D↔I, B↔C, F↔I, E↔I) are asymmetric. v2
candidate-shape implication: asymmetric intersections require
explicit pipe-direction (which cluster's artifact feeds which
cluster's mechanism); symmetric intersections require explicit
composition-rule (how the two clusters' mechanisms coordinate at
their shared boundary); mixed-symmetry intersections require
both — pipe-direction for the asymmetric sub-patterns, composition-
rule for the symmetric sub-patterns, and the candidate must name
which sub-patterns it adopts.

## Open structural questions

Three structural questions surfaced by the implications-mining
cycles, deferred this synthesis:

1. **Should cluster F split into 2-4 narrower clusters?** Cycle 69's
   8 sub-axes partition naturally into four conceptual themes:
   *nomenclature* (version, terminology, role) / *resource semantics*
   (cost-tier, capability-tier) / *capability semantics* (task-class,
   capability-layer) / *governance semantics* (autonomy-mode).
   Splitting into 4 narrower clusters yields cleaner conceptual
   boundaries. Keeping cluster F unified preserves the
   meta-architectural-pattern that *stratification axes proliferate
   naturally where multi-agent architecture surfaces them*. Phase 2
   candidates can adopt either view; the synthesis layer here keeps
   the unified view and flags the question for revisitation when
   PAI deeper-read or oh-my-codex deeper-read return adds another
   stratification-foregrounding system.

2. **Cluster A vs cluster G boundary for per-action retry vs
   role-asymmetric context.** Voyager's CriticAgent ↔ ActionAgent
   loop has role asymmetry (different agents) but per-action
   granularity (each action gets a critic call) — cycle 69
   classified as cluster A (within-task retry, bounded-retry-with-
   feedback sub-shape). The argument for cluster G is non-trivial:
   critic and action have different trust semantics. The
   classification line is thin; future synthesis cycles or Phase 2
   candidate work may elevate the distinction. v2 candidates
   implementing this could choose either pattern shape — cluster A
   bounded-retry mechanism (within a single role's task lifecycle)
   or cluster G role-asymmetric context (different roles consume
   each other's outputs with explicit trust semantics).

3. **Orphan-pattern observation depends on within-system coherence.**
   Cycles 65-68 surfaced an orphan-pattern: implications without
   strong same-system neighbors are predominantly the ones
   anchoring cross-system clusters. Cycle 69 broke the pattern —
   Voyager has 0 orphans, all 10 implications participate in
   within-system intersections. Refinement: orphan pattern holds
   for moderately-coherent systems; tightly-coherent systems are
   exceptions. Methodological observation for future mining: the
   orphan-pattern correlates with within-system architectural
   coherence rather than being a universal mining-result property.
   v2 candidates that define explicit interaction protocols
   between named roles (high-coherence shape) will have implications
   participating in within-system intersections; v2 candidates with
   loose component-coupling will produce orphans whose value is in
   cross-system convergence.

4. **File-size threshold crossed; cluster section split deferred
   to next synthesis cycle.** Cycle 70's hand-off observed
   `1-research.md` approaching the cycle-33 restructure trigger
   (~1422 lines / 78KB). Cycle 72's cross-cluster intersections
   subsection (~310 lines added) puts the file past the trigger.
   Cycle 72 deferred the structural restructure (combining content
   addition with structural restructure in the same cycle is risky
   — link breakage, inconsistent diffs, and the editorial decisions
   compound). **Resolved cycle 73:** the cluster section was split
   to this file (`1-research/clusters.md`) mirroring the cycle-33
   per-system-files split; the index summary points here. Migration
   note at
   [`../_notes/cycle-73-cluster-restructure.md`](../_notes/cycle-73-cluster-restructure.md).
   v2 design-input from this observation: the
   file-size-driven restructure pattern is itself a meta-
   architectural-pattern; v2 candidates with active-surface
   artifacts should plan for periodic restructure triggers and
   have explicit thresholds (cycle-33 demonstrated 1422 lines as
   the threshold; cycle 72 demonstrates the same threshold reached
   via deeper-pass content rather than per-system content).

## Implications-mining cadence summary (cycles 62-74)

- Cycle 62: AutoGen mining (8 implications)
- Cycle 63: oh-my-codex deeper-read dispatch construction
  ([#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833),
  in flight 9+ cycles as of cycle 72)
- Cycle 64: LangGraph mining (8 implications)
- Cycle 65: cross-implications synthesis (first synthesis cycle;
  produced 6 elevation drafts, deferred actual elevation to a
  future synthesis cycle)
- Cycle 66: Cognition Devin mining (9 implications)
- Cycle 67: openclaw mining (10 implications)
- Cycle 68: OpenAI harness mining (10 implications)
- Cycle 69: Voyager mining (10 implications, exhausts unique
  deep-dive system pool)
- Cycle 70: synthesis with elevation of within-cluster sub-shape
  catalogues to this catalogue (second synthesis cycle; completed
  the deferral cycle 65 set up — Family-format observations for
  clusters A/B/D/F/H + Phase 2 design-input + open structural
  questions, file 736→1241 lines)
- Cycle 71: PAI deeper-read dispatch construction
  ([#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842))
  + stuck-dispatch self-healing finding diagnosing #2833 8-cycle
  malformation (root cause: filed without `Copilot` as assignee;
  orchestrator-bot lacks GraphQL permission to self-fix; mitigation
  = diagnosis comments requesting Eva manually assign Copilot)
- Cycle 72: cross-cluster intersections synthesis (third synthesis
  cycle; A↔B, F↔H, D↔I primary deep-mine plus 4 additional
  intersections A↔C/B↔C/F↔I/E↔I flagged for future synthesis;
  ~310 lines added; file-size threshold crossed, restructure
  deferred to cycle 73)
- Cycle 73: cluster-section file restructure (this file extracted
  from the index per the cycle-33-style split; index retains a
  brief summary + quick-reference cluster table + link here)
- Cycle 74: cold-reader on cycle-73 restructure (3/3 PASS with
  3 minor findings — undocumented preamble paragraph + self-
  reference rewrite + section-header rename; clusters.md content
  verbatim modulo documented transformations) + deeper synthesis
  on the four flagged intersections from cycle 72 (A↔C, B↔C,
  F↔I, E↔I — all four elevated to full treatment matching cycle-
  72 format with 4-5 sub-patterns each, v1 failure-mode mapping,
  Phase 2 implication; ~340 lines added; cluster-I intersection
  coverage extended — every cluster I sub-shape now has a full
  intersection discipline (revised cycle 85 per audit#454 D1: this
  is intersection-coverage extension, NOT corpus-depth hardening;
  cluster I remains at 2-system convergence); cluster C
  re-foregrounded as lifecycle-vocabulary linchpin via A↔C + B↔C
  dual participation)

### Post-cycle-74 cold-reader rhythm and cycle 85 toggle

Cycles 75-84 ran a cold-reader rhythm with substantive focal
diversifying across stress-tests, corrective audits, explore-
alternatives, and synthesis:

- Cycles 75-77: cold-reader on prior cycle (3 instances)
- Cycles 78-80: cold-reader-then-stress-test (cluster I; cluster F;
  cluster A vs G — HARDENED at 3 instances)
- Cycle 81: cold-reader-then-self-congratulation-audit (NOVEL shape)
- Cycle 82: cold-reader-then-over/under-prescription-audit
- Cycle 83: cold-reader-then-explore-alternatives
- Cycle 84: cold-reader-then-synthesis (RE-INSTANCE; ~3400 lines of
  accumulated _notes distilled into 6 v2 design-input categories)

The post-cycle-74 cold-reader rhythm produced refinements integrated
into clusters.md cycles 79-80 (cluster A vs G boundary, cluster F
multi-lens framework) and methodology refinements documented in
_notes/ for cycles 81-84 (granular-vs-broad conflation, 3.5-axis
matrix for lexicon, anti-inheritance discipline). The artifact-
resident integration rate dropped post-cycle-80 — cycles 81-84
produced methodology refinements that live in _notes/ but did not
update clusters.md.

**Cycle 85 toggle (audit#454 D5/P5 absorption).** Audit cycle 212
identified the post-cycle-80 cold-reader rhythm as having reached
diminishing-returns boundary and recommended cycles 85-90 toggle
to artifact-resident integration of accumulated synthesis OR
Phase 2 candidate authoring. Cycle 85 implements the toggle:

- Cold-reader cadence as cycle-agnostic prefix is **suspended**
  for cycles 85-89 in favor of artifact-resident integration
- Cycle 85 substantive focal: per-question audit#454 absorption +
  targeted clusters.md revisions (D1, D2, D3, D4 ACCEPT verdicts
  applied; D5 acknowledged in this cadence summary)
- Cycles 86-89 plan: M1 v1-substrate instantiation layer, M2
  self-management cost annotations, M3 v1 strengths layer, M4
  cycle frequency Phase 2 variable, M5/P6 audit-as-peer
  preservation pattern, P1-P6 Phase 2 evaluation discipline
- Cycle 90 trigger: if cycles 86-89 complete cleanly, cycle 90
  begins Phase 2 candidate authoring against the augmented
  synthesis surface

Cold-reader prefix may return cycle 90+ if specific outputs warrant
fresh-eyes verification (e.g., major artifact restructure cycle),
but the cycle 78-84 default of "every cycle starts with cold-reader"
is dropped. Per-cycle process documents continue under `../_notes/`.

**Cycles 86-88 M-item integration arc** (first three of cycles
86-89). Cycle 86 implemented M1 v1-substrate instantiation for
cluster A using cycle 85's D3 sub-cluster grouping scaffold (9
substrate notes: 1 STRONG + 2 PARTIAL + 6 ABSENT). Cycle 87
implements M1 for cluster B (9 substrate notes: 1 STRONG + 6
PARTIAL + 2 ABSENT — substantively more substrate-aligned than
cluster A as predicted in cycle 86 hand-off) and begins M2
self-management cost annotation layer (cluster B sub-shapes
annotated LOW/MODERATE/HIGH; cluster A retro-annotation deferred
to cycle 88+). Cycle 87 also documents the methodological
observation that sub-cluster grouping helped cluster A (natural
conceptual boundaries) but not cluster B (overlapping-scope
storage-architecture sub-shapes don't divide cleanly). Cycle 88
implements M1 for clusters D and F (cluster D: 2 STRONG + 6
PARTIAL + 1 ABSENT, the most substrate-aligned cluster; cluster
F: 0 STRONG + 3 PARTIAL + 5 ABSENT, the least substrate-aligned)
and adds M2 retro for cluster A (4 LOW + 5 MODERATE + 0 HIGH) +
M2 for clusters D and F + M3 v1 strengths layer (3 distinct v1
strengths surfaced from STRONG sub-shapes: process-isolation A↔B
dual-cast; anti-patterns published; walkback as artifact). M3
layer begins as standalone cluster-spanning subsection between
clusters B and D. Cycle 88 prediction: cluster D would be most
substrate-aligned given audit#454 explicit statement "v1 already
does this work via the retrospective cadence and the journal" —
empirically validated (2 STRONG sub-shapes, the most of any
cluster annotated). Cycle 88 prediction also held for cluster F:
limited tool-suite stratification produces substantial ABSENT
count (5 of 8 sub-axes) — empirically validated. Cycle 89 plan:
clusters C / E / G / H / I M1 + M5/P6 audit-as-peer preservation
pattern + P1-P6 Phase 2 evaluation discipline integration into
2-design-framework.md (or new 2-evaluation-criteria.md if
factoring is cleaner).

Total (post cycle 88): 45 implications across 6 systems across 9
clusters across 8 mining cycles, plus 5 synthesis cycles (65, 70,
72, 74, 84) producing within-cluster sub-shape catalogues + 7
cross-cluster intersection disciplines (A↔B, F↔H, D↔I, A↔C, B↔C,
F↔I, E↔I) with compositional / dual-cast sub-pattern classification
+ 1 audit-engagement absorption cycle (85) integrating 21 audit#454
verdicts + 3 M-item integration cycles (86 cluster A + 87 cluster
B + 88 clusters D/F + cluster A M2 retro + M3 layer) producing 35
v1-substrate instantiation notes (M1 layer, 4 of 9 clusters
annotated: A, B, D, F) plus 35 self-management cost notes (M2
layer, 4 of 9 clusters annotated: A, B, D, F) plus 1 cluster-
spanning v1 strengths layer (M3, framing + 3 strengths). Implications-mining cadence on unique deep-dive systems
is exhausted post-cycle 69. Future mining requires either dispatch
deliveries (oh-my-codex via
[#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
still in flight, PAI via
[#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842)
dispatched cycle 71, oh-my-claudecode via
[#2847](https://github.com/EvaLok/schema-org-json-ld/issues/2847)
dispatched cycle 75, openai/symphony via
[#2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851)
dispatched cycle 77) OR re-mining existing systems at deeper
depth. The synthesis arc has been substantively additive across
cycles 65, 70, 72, 74, 84.
