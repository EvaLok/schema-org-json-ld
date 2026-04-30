# Cycle 35 (2026-04-30) — Phase 2 design-axes synthesis + cold-reader on cycle 34 + housekeeping continuation

## Setup

Cold-start session. Cron fired 2026-04-30 20:21 UTC (issue #2796).
Sixth cycle of 2026-04-30 (after 28-32 earlier today + cycle 34 at
17:33 UTC). Cycle 34 verified the cycle-33 Eva-driven restructure
4/4 PASS and ran the first HOUSEKEEPING-section sweep (3 closures).

Cycle 34 named two non-mutually-exclusive options for cycle 35:
1. **Phase 2 candidate-generation preparation** (substantive,
   architecturally-load-bearing) — re-read restructured cross-system
   observations against the design axes Phase 2 candidates must span.
2. **Continue housekeeping** (bounded mechanical) — pick up 2-4 of
   the 11 deferred items from cycle 34's considered-but-deferred list.

Cycle 35 adopts both: the substantive Phase 2 prep work as the
focal, with a 4-closure housekeeping sweep biased upward from
cycle-34's 3 per cold-reader (b)'s "validate sustainable cadence"
finding (see below).

Open Eva directives shaping the cycle: #2794 (today, ~3h before
cycle 34 fired) — Copilot firewall allowlist expanded with 9
domains. The two stub systems' deeper reads (#2779 Cognition Devin,
#2781 OpenAI harness) are now unblocked but not dispatched this
cycle (deferred to cycle 36+ to avoid splitting attention from the
Phase 2 prep substantive work).

## Cold-reader on cycle-34 notes file

Three pre-commit questions named at the end of cycle-34 notes.

### Question (a) — closure linking-comment accuracy

**PASS.** Spot-checked all three cycle-34 closures
(#2789 / #2790 / PR #2791). Verifications:

- **Commit SHAs resolve correctly:**
  - `960bb007` → cycle 32 in-place Tier-2 restructure (verified via
    `git log --oneline 960bb007 -1`).
  - `46cf1820` → cycle 33 Eva-driven split (verified).
  - `4af239a8` → HOUSEKEEPING section addition (verified).
- **`docs/redesign/1-research.md` line range 94-589** (cited as
  cross-system observations location): file is 735 lines; line 94
  starts the `## Cross-system observations` header; line 589 is in
  the closing paragraph just before `## Phase 1 work plan` at line
  590. Range cited is essentially correct, ±1-2 lines.
- **`docs/redesign/_notes/cycle-33-research-restructure.md` lines
  167-172** (cited for the dedup procedure): the procedure is at
  lines 165-170 in my read. ±2 lines off but the cited substantive
  content ("Ask Eva or check the issue bodies + which one Copilot's
  PR #2791 actually attached to before closing the duplicate") IS in
  the cited range.
- **Both prompt-file paths exist:**
  - `.github/workflows/orchestrator-redesign-prompt.xml` (referenced
    by the closing comment for #2790 — SECTION 6b lives here).
  - `.github/workflows/orchestrator-prompt.xml` (production v1
    prompt).

A future reader can navigate from each closed item to the absorbed-
content destination; the forward-link discipline (per HOUSEKEEPING
`<closure-discipline>`) was followed.

**Mild flag for cycle-36 fresh-eye:** line-range citations are off
by 1-2 lines. Not load-bearing (substantive content is in the cited
range), but a future closure-comment template could grep for the
`##` header line number rather than estimating.

### Question (b) — deferral rationale principled or procrastinating

**BORDERLINE-PASS.** Cycle 34 closed 3 items and considered 11 more,
deferring all 11. The deferral rationale cited the HOUSEKEEPING
`<cadence>` block: "A cycle that's heavy on substantive work
doesn't need to sweep." Cycle 34 framed its cold-reader work as the
heavy substantive thread justifying the 0-vs-11 split on the
deferred set.

**What's principled:** The cold-reader was genuinely substantive
(verifying verbatim move on ~500 lines, reading 8 per-system files
for standalone-readability, checking 3 stubs for honest marking,
verifying link-table coverage). Not bounded mechanical.

**What's borderline:** Each deferred closure is bounded mechanical
at ~3-5 minutes (read absorption, write linking comment, post). 11
items × 4 min = ~44 min of mechanical work. Cycle 34 could have
done 5-6 closures in addition to the 3 it did, without exceeding
cycle budget. Cycle 34's own pre-commit checklist (b) flagged this
honestly: "feels close to procrastinating on a one-time backlog ...
The principled answer is probably 'first sweep can be larger
because catch-up; subsequent cycles smaller for sustainable cadence.'"

**Cold-reader (b) verdict + corrective action:** BORDERLINE-PASS
because cycle 34 surfaced the question itself rather than glossing
over it; the 3-closure count was conservative-leaning rather than
procrastinating per se. Cycle 35's corrective: bias upward to 4
closures (LangGraph + AutoGen pairs) — at the upper bound of the
2-4 sustainable cadence — to validate that the cadence is
sustainable rather than indefinitely deferred. If cycle 36 sustains
the 4-closure rate (or higher), the cadence is genuinely
sustainable; if it drops back to 2-3, the borderline-procrastination
flag re-fires.

### Question (c) — design-input meta-loop observations substantive or filler

**PASS** with qualification. Three observations captured in cycle 34:

1. **Eva-driven structural change pattern** — substantial structural
   migrations can come from outside the orchestrator flow as long as
   they document themselves with cold-reader questions for the next
   orchestrator cycle. **Substantive** — cross-orchestrator
   coordination via notes-files + named follow-up questions is a
   reusable hand-off pattern (cycle 33 Eva-driven, audit-repo
   cross-reads do the same shape). v2 candidate generation should
   account for this pattern; the human-in-the-loop case for occasional
   substantial changes works without bespoke machinery.

2. **Duplicate-issue-from-UI-hiccup** (#2789/#2790, 7 seconds
   apart) — a real failure mode that v2 dispatch tooling could
   prevent. **Borderline filler** — happened ONCE in the redesign so
   far; one occurrence isn't pattern. The remediation (idempotency
   keys) is implementable but non-trivial for a one-time-cleanup-
   class failure where the cleanup is itself bounded-mechanical. As
   a v2 candidate consideration it's worth one paragraph of
   dispatch-tooling design but not a top-tier design driver. Watch
   for recurrence; if it happens twice more across the next 30
   cycles, promote from "noted" to "address."

3. **Many drafts that never merge** (10 of 11 open PRs are drafts) —
   structurally invisible to GitHub's open-PR list. **Substantive** —
   absorption-state is a real concept the underlying tooling doesn't
   represent; the closure-as-absorbed pattern is a workaround. v2
   could track absorbed-vs-draft state in a dedicated label or build
   a classifier tool. This compounds over many cycles (the 10 drafts
   accumulated across cycles 6-26 at ~1-2 per dispatch cycle); not
   one-off.

**Cold-reader (c) verdict:** Section as a whole has good substance,
with observation 2 (duplicate-issue UI hiccup) as the weakest of
the three. Cycle 34's pre-commit checklist (c) explicitly named
observation 2 for spot-check, which was the right call — that's the
borderline case.

### Cold-reader summary

3 questions, all PASS or BORDERLINE-PASS. The borderlines are
named (line-range citation precision; first-sweep cadence
calibration; UI-hiccup observation strength) and have specific
corrective actions for cycle 35-36+.

## Housekeeping continuation: 4 closures (LangGraph + AutoGen pairs)

Per cold-reader (b)'s "bias upward to 4 closures" finding, cycle 35
closes both the LangGraph and AutoGen dispatch pairs from the
cycle-34 deferred set (4 of the 11 deferred items). Closure
comments include forward-links to per-system files + cross-system
observation patterns + cycle iteration log per HOUSEKEEPING
`<closure-discipline>`.

| Item | Cycle | Forward-link |
|---|---|---|
| PR #2768 (LangGraph research) | 18 | `1-research/systems/langgraph.md` + 7 of 16 cross-system patterns |
| Issue #2767 (LangGraph dispatch) | 18 | Pair with PR #2768 |
| PR #2763 (AutoGen research) | 15 | `1-research/systems/autogen.md` + 7 of 16 cross-system patterns |
| Issue #2762 (AutoGen dispatch) | 15 | Pair with PR #2763 |

After this cycle's sweep, 7 deferred items remain (5 PRs + 2 issue
pairs from cycles 6, 11, 26 dispatches). At a 4-closures-per-cycle
rate, cycles 36-37 finish the backlog. Cycle 36 candidates: Phase 0
critique pair (cycle 11 PR #2756 + issue #2755) plus one of the
cycle-26 cross-system research PRs (e.g., PR #2784 oh-my-codex).

## Substantive: Phase 2 design-axes synthesis

The Phase 1 deliverable (cross-system observations across 16
patterns + 3 persistent divergences + 8 per-system files) is now
structurally stable. Phase 2 candidate generation requires the
post-retrospective checkpoint approval, but **preparation work** —
distilling the 16 patterns into actionable design axes that Phase 2
candidates must span — is unblocked.

This section produces a working framework for Phase 2 candidate
differentiation. It is NOT a Phase 2 candidate; it is the axis
framework against which candidates will be evaluated. The framework
itself is subject to iteration before promotion to a Phase-2-input
section in `1-research.md` (currently a placeholder).

### Convergent constraints (every v2 candidate must honor)

These patterns are 3+/N convergent across the surveyed systems and
should be treated as constraints, not choices. A candidate that
violates one of these is a candidate that disagrees with all the
surveyed systems' converged practice; that disagreement should be
explicit and load-bearing, not accidental.

1. **Code-vs-prompt split exists.** Deterministic code executes;
   LLM proposes. (Family B pattern 1, 3+/6 systems, foregrounded
   convergence with substrate variations.) v1 honors this in shape
   (Rust tools + LLM orchestrator) but the prompt encodes
   procedure that should be in tools (CORE-DESIGN-PRINCIPLE).
2. **Failed work is recorded as artifact, not silently discarded.**
   (Family C pattern 2, 3+/3 systems with structural similarity.)
   v1 has journal failure notes but no failure-record file with
   read-after-failure semantics.
3. **Strong-defaults security with operator-controlled knobs.**
   (Family D, 3+/3 + scope condition.) v1's GitHub Actions
   secret-handling and the prompt's UNTRUSTED-TEXT-RULES already
   honor this; v2 should preserve.
4. **Per-agent model selection treated as primitive.** (Family A
   pattern 2, 3+/3 + diversity hedge.) v1 uses a single model
   (Claude Opus) for the orchestrator; Copilot dispatch model is
   per-dispatch. v2 candidates should treat model selection as
   per-component, with per-component rationale.
5. **Anti-patterns documented explicitly as deliverable artifact.**
   (Family E pattern 1, 3+/6 systems.) v1's retrospective is the
   anti-pattern catalog; v2 prompt should preserve and extend
   (e.g., "what we will not do" sections per the openclaw VISION.md
   pattern).
6. **Small core, capability extends via something.** (Family B
   pattern 2, 3+/7 systems.) The "something" is a real choice (see
   Axis 6 below), but the principle (lean entry point + extension
   mechanism) is convergent.
7. **Memory is an architectural concern requiring elevation.**
   (Family C pattern 4, 3+/5 + diversity hedge.) The shape of the
   memory subsystem is a choice (see Axis 3 below), but the
   convergent claim — memory deserves first-class architectural
   treatment, not derivative-of-state — is a constraint.

### Real design axes (v2 candidates differ on)

These patterns are where the surveyed systems diverge. Each axis is
a meaningful candidate-differentiation point. Each candidate must
declare its position; multiple positions are defensible.

#### Axis 1 — Agent decomposition

**The choice:** how is the orchestrator session decomposed into
agents/roles?

| Position | Systems supporting | Notes |
|---|---|---|
| Single-threaded linear | Cognition Devin (named-rejection of multi-agent) | Strongest published anti-stance ("Don't Build Multi-Agents") |
| Small fixed team with role-separation | Voyager (4 agents), AutoGen Magentic-One (lead + workers), oh-my-codex (30 named role prompts) | 3+/3 with Cognition contradiction |
| Multi-agent peer (uncontrolled) | None | Rejected by 3+/6 systems as default |

**v1's position:** single-threaded with Copilot dispatches as
parallel workers (off-process). The dispatches are not "agents"
in the small-fixed-team sense — they're per-task externally-
delegated work.

**v2 candidate space:** retain dispatch-as-worker (current shape)
vs adopt small-fixed-team within the orchestrator session itself
(e.g., planner / executor / critic / curator). Cognition's
named-rejection makes single-threaded a defensible default.

**Cross-axis dependency:** Axis 1 × Axis 7 (orchestration
topology) — single-threaded forces single-topology; small-fixed-
team enables but doesn't force multi-topology coexistence.

#### Axis 2 — State representation primitive

**The choice:** what is the unit of persistent state?

| Position | Systems supporting | Notes |
|---|---|---|
| Single global state file | None | v1's `state.json` is the explicit anti-example; 3+/5 systems agree |
| File-per-component | AutoGen, Voyager (`ckpt/<agent>/`), oh-my-codex (`.omx/state/<mode>-state.json`) | 3+/5 + diversity hedge |
| Typed-channel-map within one schema | LangGraph | Persistent divergence — one pole |
| Repository-as-state | OpenAI harness | git substrate; ephemeral worktrees |
| Plans-as-artifacts (orthogonal) | OpenAI, oh-my-codex | Family C pattern 5, 2-system |

**v1's position:** monolithic `state.json` (42 keys, 62-69%
defense-character per F12 catalog). 4-6× reduction estimated for
v2 per cycle-5 measurement.

**v2 candidate space:** every position EXCEPT "single global
state file" is defensible. The choice between file-per-component
and typed-channel-map is the persistent State-shape divergence
(Family C); a candidate must commit to one or explicitly span
both. The repository-as-state position has interesting properties
for a public-repo orchestrator (commits ARE state) but conflicts
with the journal/notes-file conventions if state mutations land
in journal entries vs separate state files.

**Cross-axis dependency:** Axis 2 × Axis 3 (memory) — file-per-
component naturally supports memory-as-component-file; typed-
channel-map naturally supports memory-as-channel; repo-as-state
supports memory-as-files-in-repo.

**Maps to:** F12 (state accretion), F11 (post-close mutations).

#### Axis 3 — Memory subsystem shape

**The choice:** if memory is first-class (per convergent
constraint 7), what shape does it take?

| Position | Systems supporting | Notes |
|---|---|---|
| Singleton plugin slot (one mechanism active, replaceable) | openclaw | Persistent divergence — one pole |
| Top-level architectural principle | PAI Principle 13 | Persistent divergence — other pole |
| Context trace (everything-the-agent-has-done) | Cognition Devin | Strongest "memory is the trace" framing |
| Repository-as-record | OpenAI harness | "Anything not in-context doesn't exist" |
| Wiki + search (markdown-first, search-first) | oh-my-codex (`.omx/wiki/` + MCP server) | Bounded context injection |
| Typed channels (short/long-term distinction) | LangGraph (Store + checkpointer) | Closest to v1's intermediate-cache shape |
| Memory derivative of state (no first-class) | None | Rejected by 3+/5 |

**v1's position:** memory is mostly derivative of state.json (no
first-class memory). The journal acts as long-term reflective
memory; the worklog is short-term transient. No semantic memory
primitive.

**v2 candidate space:** every position except "memory derivative
of state" is defensible. The choice has significant downstream
consequences for what an orchestrator session can recall across
cold-starts. Persistence-mechanism evolution (cycle-2 to cycle-33+)
has been organic-markdown-files; that maps closest to wiki+search
but without the search infrastructure.

**Cross-axis dependency:** Axis 3 × Axis 2 (state) — memory shape
follows state representation; Axis 3 × Axis 1 (decomposition) —
small-fixed-team can have per-agent memory subsystems.

#### Axis 4 — History/Provenance substrate

**The choice:** where does append-only history live?

| Position | Systems supporting | Notes |
|---|---|---|
| Branching checkpoints (in-process versioning) | LangGraph time-travel | "`update_state` does not roll back; creates new checkpoint that branches" |
| Versioned files (`<name>V2.js`, `V3.js`) | Voyager skill versioning | Filesystem versioning |
| Git-as-substrate | OpenAI harness | Repository as state; commits append; ephemeral worktrees |
| One-way file migration with read-only legacy | oh-my-codex | Schema migrations one-way, not destructive |

**v1's position:** journal + worklog + commits are append-only-by-
git but `state.json` is destructive (writes overwrite). The
draft-then-promote / append-only retention pattern (Eva advisory
#2408) is the targeted v2 fix.

**v2 candidate space:** all four positions are defensible. Git-
as-substrate has natural alignment for a public-repo orchestrator
(every commit is the audit trail). Branching checkpoints have
appeal for the "what-if" reasoning multi-cycle work needs but
aren't trivially mappable to a flat-file-on-disk substrate. The
one-way migration shape is the conservative choice for evolving
the persistence mechanism itself across cycles.

**Maps to:** F11 (post-close mutations), F12 (state accretion via
non-destructive write semantics).

#### Axis 5 — Plans/specs as forward artifacts

**The choice:** are plans/specs first-class versioned artifacts
written before execution, or reconstructed-after?

| Position | Systems supporting | Notes |
|---|---|---|
| Yes — plans-as-artifacts (active/completed/technical-debt) | OpenAI harness | Plan files checked into repo; per-category lifecycle |
| Yes — context snapshots before execution | oh-my-codex | `.omx/context/{task-slug}-{timestamp}.md` with explicit fields |
| No — plans live in-message or are reconstructed from history | Most others (none explicitly support reconstruction-after as primitive) | Default in absence of plan-artifact infrastructure |

**Status:** 2-system clean convergence. Lower convergence than
other patterns; treat as candidate-considered axis, not constraint.

**v1's position:** plans live in cycle issue comments + journal
entries + occasional `_notes/` files. No structured plan-artifact
lifecycle. The redesign has implicit cycle-N→cycle-N+1 plan
suggestions in notes files but no separate plan-files-on-disk.

**v2 candidate space:** adopting plans-as-artifacts forces a
filesystem layout decision (`plans/active/`, `plans/completed/`,
`plans/technical-debt/`) and a transition lifecycle. Skipping
this axis means relying on journal + notes for the same purpose.

#### Axis 6 — Extension shape

**The choice:** if small-core extends via something, what?

| Position | Systems supporting | Notes |
|---|---|---|
| Plugins | openclaw | "Core stays lean; optional capability ships as plugins" |
| Skills | PAI, oh-my-codex (39 skills) | Skill = code + prompt + invocation contract |
| Tools | LangGraph (`ToolNode`), AutoGen (model-emits-tool-call) | LLM-discoverable invocation primitives |
| Layers | PAI 16 principles, AutoGen Core/AgentChat/Extensions/Studio/Bench | Architectural-layer composition |
| Harness-accumulation (depth-first) | OpenAI harness | Capabilities added iteratively as failures surface |
| Configuration-layer-with-hooks | oh-my-codex (on top of unmodified Codex CLI) | Wrap-without-replace |

**v1's position:** Rust binaries in `tools/` directory with shell-
wrapper scripts. No formal "skill" or "plugin" abstraction; tools
are discovered by file-existence + naming convention.

**v2 candidate space:** retaining Rust-tools-as-extension shape
is the path of least migration cost, with the question being
whether to add a discovery/registration primitive (skill / plugin
manifest) on top.

#### Axis 7 — Orchestration topology

**The choice:** how do agents/components coordinate?

| Position | Systems supporting | Notes |
|---|---|---|
| Single-pattern (one shape only) | Cognition (single-threaded linear) | Forces simplicity at cost of flexibility |
| Multi-pattern coexisting | AutoGen (round-robin/selector/swarm/graph), LangGraph (chaining/routing/parallelization/orchestrator-worker/ReAct/subgraphs/supervisor) | 3+/3 in surveyed |
| Sequential mode transitions with deterministic transition policy | oh-my-codex (`STATE_MODEL.md` allowlist) | Modes governed by allowlist preventing illegal shifts |
| Lead-worker hierarchy | AutoGen Magentic-One | Specialized workers under orchestrator |
| Peer-flow | Voyager (curriculum → action → critic → skill) | Round-robin among role-specialized peers |

**v1's position:** rigid checklist-driven sequence (STARTUP → C
phases → COMPLETION). One topology, encoded in two checklist
files.

**v2 candidate space:** the rigid-checklist position has been
explicitly named as Phase 2 anti-pattern (per CORE-DESIGN-
PRINCIPLE). Multi-pattern coexisting with deterministic
transition policy (oh-my-codex shape) is the strongest match for
"orchestrator handles novel situations" because different
situations may call for different topologies.

**Cross-axis dependency:** Axis 7 × Axis 1 (decomposition) — see
Axis 1.

#### Axis 8 — Mechanical enforcement scope

**The choice:** what is regression-tested mechanically?

| Position | Systems supporting | Notes |
|---|---|---|
| None | Default in absence of explicit infrastructure | Rare in surveyed |
| Data-shape only | LangGraph (TypedDict / dataclass / Pydantic), Voyager (init-time `count == len(skills)`) | Diversity hedge — adjacent to behavioral enforcement |
| Behavioral promises + agent-affecting prose | OpenAI (custom linters with agent-readable error messages), oh-my-codex (prompt-contract regression tests on `prompts/`) | 2-system strict |

**v1's position:** Rust tools have unit tests; pipeline-check has
sub-checks. No regression tests on the orchestrator prompt or
checklist text — agent-affecting prose can change without CI
catching.

**v2 candidate space:** adopting behavioral-prose CI is a
high-leverage v2 move because it directly addresses F1 (constraint-
without-tool ratio): a constraint added to the prompt without a
paired tool fix would surface as a CI test the orchestrator must
honor mechanically, OR be rejected pre-merge.

**Maps to:** F1 (constraint accretion), CORE-DESIGN-PRINCIPLE
violation detection.

#### Axis 9 — Iteration ceilings

**The choice:** are autonomous loops bounded?

| Position | Systems supporting | Notes |
|---|---|---|
| None (open-ended runs) | Rare in surveyed | Implicit in v1's per-cycle non-bounded retry |
| Loop count ceilings | oh-my-codex (`max_iterations=10`, `max=5`), Voyager (`action_agent_task_max_retries=4`) | 2-system strict |
| Runtime ceiling | Cognition Devin (45-min session limit, *documented-claim*) | 1 adjacent partial — bounds runtime not iteration |
| Both (loop + runtime) | None explicitly in surveyed | Composable |

**v1's position:** per-cycle there is no per-loop ceiling. The
cycle ITSELF is the only ceiling (~75 minutes of compute).
Pipeline-check sub-checks can re-fire, dispatch can retry, etc.,
without a bounded loop count.

**v2 candidate space:** loop-count ceilings are bounded-mechanical
to add and immediately reduce the failure surface for
runaway-autonomy. Runtime ceiling is a coarser ceiling (cycle-level
already has it).

**Maps to:** F8 (abandonment cascades), F7 (self-management
dominance via unbounded re-firing).

#### Axis 10 — Entropy / AI-slop mitigation

**The choice:** is output-quality drift addressed as recurring
infrastructure?

| Position | Systems supporting | Notes |
|---|---|---|
| Not addressed | Default | Implicit in v1's accretion-as-defense pattern (F12) |
| Golden principles + doc-gardening agent | OpenAI harness | Recurring agent-quality cleanup |
| Mandatory deslop pass post-completion | oh-my-codex | Quality cleanup embedded in task completion contract |
| Both | None explicitly in surveyed | Composable |

**Status:** 2-system clean convergence. Lower convergence than
other patterns; treat as candidate-considered axis, not constraint.
Inversely-related to v1's accretion-as-defense pattern (F12) —
these systems treat accretion as a failure mode to clean, not a
defensive structure to preserve.

**v1's position:** no entropy-mitigation primitive. F12's
defense-accretion pattern is the explicit anti-direction.

**v2 candidate space:** adopting an entropy-mitigation primitive
is high-leverage if F12 is to be addressed structurally rather
than via "defenses re-examined for load-bearingness" (per the
retrospective's Defense-accretion implication).

**Maps to:** F12 (defense accretion).

#### Axis 11 — Operator-vs-Goal framing

**The choice:** does the user issue commands the system executes
(operator-driven) or the user state long-running goals the system
pursues (goal-driven)?

| Position | Systems supporting | Notes |
|---|---|---|
| Operator-driven (user issues commands; system executes) | openclaw | Persistent divergence — one pole |
| Goal-driven (system pursues long-running goals) | PAI | Persistent divergence — other pole |

**v1's position:** Goal-driven. The mission is "automated
maintenance and construction of schema.org definitions" — a
long-running goal pursued by the orchestrator with minimal-
intervention operation per EVA-DEFAULT-AUTONOMY.

**v2 candidate space:** the redesign's primary thesis ("self-
healing self-improving AI" + autonomous schema work) commits to
goal-driven. This axis is effectively fixed for v2 candidates;
documented for completeness but doesn't differentiate candidates.
Possible exception: a v2 candidate could adopt operator-driven
shape for a sub-system (e.g., Eva-issued directives become
explicit operator-commands within an otherwise-goal-driven
overall posture).

### Cross-axis dependency map

Significant inter-axis constraints:

- **Axis 1 (decomposition) × Axis 7 (orchestration topology):**
  Single-threaded forces single-topology. Small-fixed-team enables
  but doesn't force multi-topology coexistence.
- **Axis 2 (state) × Axis 3 (memory):** State representation
  shapes natural memory primitive — file-per-component → memory-
  as-component-file; typed-channel-map → memory-as-channel; repo-
  as-state → memory-as-files-in-repo.
- **Axis 4 (history substrate) × Axis 2 (state):** State
  representation choice constrains history substrate options —
  file-per-component pairs naturally with one-way migration or git;
  typed-channel-map pairs with branching checkpoints.
- **Axis 8 (mechanical enforcement) × Axis 5 (plans-as-artifacts)
  × Axis 10 (entropy mitigation):** Mechanical enforcement is the
  substrate enabling both plan-lifecycle CI checks and golden-
  principles enforcement. Adopting Axis 8 unlocks the others.
- **Axis 11 (operator-vs-goal) × Axis 1 (decomposition):** Goal-
  driven pairs naturally with single-threaded long-running
  (Cognition); operator-driven supports either.

Largely orthogonal:

- **Axis 4 (history) × Axis 6 (extension shape)** — independent.
- **Axis 9 (iteration ceilings) × any other axis** — additive
  primitive.

### Mapping to v1 failure modes

Tentative axis-to-Fpattern mapping. The retrospective's "v2 design
implications by family" section provides high-level guidance; this
mapping is more axis-specific.

| F-pattern | Family | Most-relevant axes | Rationale |
|---|---|---|---|
| F1 (constraint accretion) | Defense accretion | Axis 8 (mechanical enforcement), CORE-DESIGN-PRINCIPLE | Mechanical CI on prompt contracts forces constraint-as-test or rejection |
| F5 (state.json as procedural-leak) | Defense + Reconciliation | Axis 2 (state representation), Axis 8 | File-per-component or typed-channel separates concerns |
| F11 (post-close mutations) | Defense + Reconciliation | Axis 4 (history substrate), Axis 9 (iteration ceilings) | Append-only with branching prevents post-close destructive writes |
| F12 (defense accretion catalog) | Defense | Axis 2 (state), Axis 4 (history), Axis 10 (entropy) | All three contribute; Axis 10 is the structural anti-accretion |
| F2 (Eva-response detection) | Reconciliation | NEW axis needed (channel polling + reconciliation per channel) | Not directly addressed by 16 patterns |
| F3 (multi-candidate state drift) | Reconciliation | Axis 2 (state representation) | Single source of truth per concern |
| F4 (frozen-artifact lifecycle fragility) | Reconciliation | Axis 4 (history substrate), Axis 5 (plans-as-artifacts) | Lifecycle primitives address freeze/refresh timing |
| F6 (cyclomatic procedure depth) | Procedure overhead | Axis 7 (orchestration topology), CORE-DESIGN-PRINCIPLE | Multi-pattern with transition policy lighter than rigid checklist |
| F7 (self-management dominance) | Procedure overhead | Axis 1 (decomposition), Axis 8, Axis 9 | Specialization + mechanical enforcement + iteration ceilings reduce self-management surface |
| F8 (abandonment cascades) | Tooling fragility | Axis 9 (iteration ceilings), CORE-DESIGN-PRINCIPLE (no parallel implementations) | Bounded loops + single-implementation discipline |
| F9 (adversarial-review treadmill) | Procedure overhead | Axis 7 (orchestration topology) | Multi-pattern shape replaces fixed adversarial-review step with situational invocation |
| F10 (audit's value is broader read scope) | Design-implication | Not a v2 axis (audit-side concern) | Audit-as-peer pattern preserved |

**Observation 1:** F2 (Eva-response detection) is NOT addressed by
any of the 16 patterns. The reconciliation-asymmetry implication
(every channel needs a poller producing state transitions) is a v2
design constraint that should be added as a 12th axis (or as a
cross-cutting CORE-DESIGN-PRINCIPLE elaboration). This is a real
gap in the Phase 1 deliverable — the surveyed systems don't have
an "Eva equivalent" because they're either single-user
personal-assistant (PAI/openclaw) or multi-agent without external
human-coordinator (AutoGen/LangGraph/Voyager). The reconciliation-
asymmetry pattern is a v1-discovered pattern not externally
validated.

**Observation 2:** Multiple Fs map to the same axes (Axis 2, 4, 8
each address 3+ failure modes). This isn't a problem — it's
evidence those axes are high-leverage. A v2 candidate that picks
well on Axes 2, 4, 8 addresses ~7 of the 11 failure-modes
structurally.

**Observation 3:** CORE-DESIGN-PRINCIPLE (tools handle rote;
orchestrator handles judgment) shows up across F1, F6, F7, F8 — it
is itself an axis-cross-cutting constraint, not a separate axis.
Every candidate must demonstrate the principle in concrete
prompt-vs-tool decisions.

### Phase 2 candidate template (preliminary)

A Phase 2 candidate should declare its position on each of Axes
1-11 (or explicitly defer / span), the CORE-DESIGN-PRINCIPLE
elaboration, and the cross-axis dependencies it commits to.
Suggested structure:

```
## Candidate <N>: <name>

### Position summary
- Axis 1 (decomposition): <position> — <one-sentence rationale>
- Axis 2 (state representation): <position> — <one-sentence rationale>
- ... (11 axes)

### Cross-axis commitments
- Axis 1 × Axis 7: <how this candidate handles the dependency>
- Axis 2 × Axis 3: <...>
- ... (significant pairs)

### Failure-mode addressing
- F1: <how candidate addresses>
- ... (12 patterns)

### What this candidate gives up
- Honest list of design dimensions where this candidate is weaker
  than alternatives — what it trades away to gain its strengths.

### Tool surface implied
- List of tools the candidate's prompt expects to invoke; which
  exist; which would be net-new to build.

### Migration cost from v1
- Specific migration steps; what state/tools/conventions transfer
  vs need replacement.
```

The template is preliminary and subject to iteration before Phase 2
candidate generation begins. The post-retrospective checkpoint
gates that work; this template is preparation, not commitment.

### Open framework questions for cycle 36+ iteration

1. **Should Axis 11 (operator-vs-goal) be eliminated as
   non-differentiating?** v2 candidates are committed to
   goal-driven by mission. Keeping the axis as documentation may
   be net-negative if it implies optionality the redesign doesn't
   actually have.
2. **Should F2's reconciliation-asymmetry become a 12th axis or
   stay as cross-cutting principle?** The framework's neatness
   pulls toward 11 axes; F2's specific shape pulls toward making
   it explicit. Worth Copilot feedback dispatch.
3. **Does the framework smuggle Phase 2 priority into its
   ordering?** Axis ordering (Family C / E / A / B / D, then
   convergent constraints first) was inherited from the
   restructured 1-research.md cross-system observations. Re-read
   adversarially: does the ordering bias which axes a Phase 2
   candidate would address first?
4. **What's missing?** Adversarial check — what design dimensions
   surface in v2 prompts but are NOT in this framework? Examples
   to consider: rollback semantics on cutover, audit-repo
   integration mechanism, polyglot strategy (the schema-domain
   end goal), Eva-checkpoint mechanism specifics, security-
   posture per-trust-tier specifics. Some are CORE-DESIGN-PRINCIPLE
   elaborations; others may genuinely be missing axes.
5. **How does the framework interact with v1's preserved
   primitives?** PRESERVED-PRIMITIVES (journal, cycle-issue,
   question-for-eva, input-from-eva, git-safety, cycle-runner
   harness) names what v2 must preserve. The axes describe what
   v2 may differ on. Tension: some axes' positions might violate
   a preserved primitive (e.g., Axis 7 multi-topology-coexisting
   might break the cycle-issue convention if topology-2 doesn't
   produce a session-end summary).

These are framework-iteration questions. Address in cycle 36+ via:
- Same-cycle cold-reader on this framework (cycle 36).
- Copilot feedback dispatch on the framework (cycle 37 or 38).
- Audit-repo critique request via this-repo post (cycle 36+).

## Persistence-mechanism observations

### `_notes/` index continues to be stale

`docs/redesign/_notes/README.md` indexes only cycles 2-7 (9 entries
in the index table). Directory contains 38 files (37 cycle-N notes
+ README). Cycle-33 follow-up #3 deferred the migration; cycle 34
re-flagged the staleness in the design-input meta-loop.

This cycle adds another notes file (`cycle-35-...md`) without
updating the index. The pattern of "writer adds file, index goes
stale" is itself a design-input observation: the index needs either
a tool that auto-updates it on file creation, OR a convention that
each cycle's pre-commit checklist includes "update _notes/README
index." The auto-update tool is more reliable than the convention.

**Captured for v2 design-input:** persistence-mechanism indexes
should be auto-generated from filesystem listings, not manually
maintained. Bounded-mechanical-but-easy-to-forget = candidate for
tool extraction per CORE-DESIGN-PRINCIPLE.

### Phase 2 prep work fits in one cycle

The substantive design-axes synthesis (this section) absorbed
~30-40 minutes of a ~75-minute cycle. The framework is preliminary
and will iterate, but the initial draft fits one cycle. Compare to
cycle-32's Tier-2 in-place restructure (~400 lines rewritten in one
cycle): substantive architecturally-load-bearing prose work fits
when the inputs are organized enough to drive concrete edits.

The cross-system observations restructure (cycle 32) created the
organized substrate that made this cycle's Phase 2 prep
synthesis possible in one cycle. Without the cycle-32 restructure
(family clustering + maturity badges + divergence index), the
synthesis would have required substantially more cycle budget.
Persistence-mechanism observation: each architecturally-load-
bearing structural change has downstream multiplier effects on
subsequent cycle productivity.

## What surprised me

That the surveyed systems don't have an Eva-equivalent pattern.
F2 (Eva-response detection asymmetry) and the broader
reconciliation-asymmetry family don't map cleanly onto any of the
16 patterns because the surveyed systems are structurally
different on the human-coordinator axis. This means Phase 1
research validates the architectural patterns but does NOT
validate the v1-specific failure modes around Eva integration.

The implication: F2 / F4 / F11's reconciliation-asymmetry insights
are v1-discovered, not externally cross-validated. The cycle 33
retrospective implicit assumption that Phase 1 research would
validate v1's failure-mode taxonomy was correct for ~7 of 11 Fs
but wrong for the reconciliation-asymmetry subset. This is a real
finding, not a problem — but it means Phase 2 candidates that
address F2 are doing more design work (no external pattern to
draw on) than those addressing F12 (Axis 2 has 5 candidate
positions across surveyed systems).

## What I couldn't figure out

Whether Axis 11 (operator-vs-goal) should stay in the framework or
be removed. Arguments for removal: v2 candidates are committed to
goal-driven by mission; documenting as an axis suggests optionality
that doesn't exist. Arguments for keeping: persistent divergences
deserve framework presence regardless of whether the candidates
will actually differ on them; documents the surveyed systems'
divergence honestly. Lean toward keeping but with explicit "axis
fixed for this redesign's mission" caveat. Open for cycle-36 fresh
eye.

## Cycle-36 plan suggestion (provisional)

Two non-mutually-exclusive options, listed by leaning:

1. **Substantive focal: latent dispatch backlog.** Start picking up
   the firewall-now-unblocked deeper reads (#2779 Cognition Devin
   primary-source retry; #2781 OpenAI harness primary-source retry)
   per Eva directive #2794, OR the still-pending Phase 1 research
   targets (#2774 oh-my-claudecode; #2775 openai/symphony). Each is
   a Copilot dispatch + integration arc (~3 cycles per dispatch:
   dispatch land + per-finding evaluation + integration). Cycle 36
   could initiate one or two; cycles 37-39 do the integration.
   Highest-leverage: one of the firewall-newly-unblocked reads
   (Cognition or OpenAI) since the source-access caveat is a
   visible scar in the Phase 1 deliverable that the deeper read
   would remove.

2. **Bounded mechanical: housekeeping continuation.** 4 more
   closures (e.g., cycle-11 Phase 0 critique pair: PR #2756 +
   issue #2755; plus 1-2 of the cycle-26 cross-system research
   PRs). Per cold-reader (b) finding, validating sustainable
   cadence requires consistent 4-closures-per-cycle for at least 1
   more cycle. The 7 remaining items at 4-per-cycle finish in 2
   more cycles.

Lean toward option 1 since the substantive backlog is more
load-bearing for v2 candidate input (deeper reads strengthen the
Phase 1 deliverable; housekeeping is noise reduction). Option 2
runs as parallel bounded mechanical.

## Pre-commit checklist (for cycle 36's cold-reader)

Three bounded-mechanical questions for the next cycle's
cold-reader on this notes file:

- **(a)** Are the 11 axes genuinely independent dimensions, or are
  some redundant / sub-axes of others? Spot-check Axis 5 (plans-
  as-artifacts) vs Axis 2 (state representation) — is plans-as-
  artifacts a true independent dimension or a special case of
  file-per-component state?
- **(b)** Does the F-pattern → axis mapping have any miscategorized
  rows? Spot-check one row that crosses families (e.g., F11 maps
  to Axis 4 + Axis 9 — does Axis 9 actually address F11, or is
  that a stretch?).
- **(c)** Is the "convergent constraints" framing honest, or am I
  promoting some patterns to constraint-status to reduce the
  candidate-space prematurely? Spot-check constraint 7 (memory as
  architectural concern) — is it really constraint-status given
  the diversity hedge in the underlying pattern?
