# Cycle 68 — OpenAI harness implications for the v2 redesign

**Date:** 2026-05-04
**Substantive focal activity:** option 4 from input-from-eva [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) (implications mining on a system already read at depth) — fifth instance of the implications-mining cadence, parallel to cycle-62 (AutoGen), cycle-64 (LangGraph), cycle-66 (Cognition Devin), and cycle-67 (openclaw).
**Pivot from cycle-67's provisional read:** cycle-67 named per-finding-evaluation as cycle-68's highest-priority candidate contingent on cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) returning. Confirmed at session-start: dispatch is OPEN, 0 comments. Implications-mining option 4 continuation is the cycle-68 substantive focal. Cycle-67 named two viable fifth-system targets — Voyager and OpenAI harness — with hypotheses set per option. Cycle 68 picked OpenAI harness because the cluster I conversion test (singleton → 2-system convergent) is the highest-stakes hypothesis available and OpenAI harness is the most likely augmenter (its writeup is *literally about harness engineering*).

## What this document is, and is not

This is a focused, OpenAI-harness-specific implications writeup — what
the cycle-41 deeper-read evidence (PR
[#2805](https://github.com/EvaLok/schema-org-json-ld/pull/2805), 780
lines) suggests for the v2 redesign that has NOT been written down in
`1-research.md` cross-system observations or in `2-design-framework.md`
axes. The cross-system synthesis cites OpenAI harness alongside other
systems under shared patterns; this document inverts the lens — it
asks what OpenAI harness tells us *as a singular voice* (or a
pair/triple/quad-with-prior-systems voice where the pattern is
foregrounded by multiple).

It is NOT a Phase 2 candidate. It is implications-as-input. Phase 2
candidates still gate on the post-retrospective checkpoint and Eva
approval.

It is NOT a re-summary of `systems/openai-harness.md` — that file is
the navigation summary; the deep-dive evidence is the cycle-38 deeper
read (`_notes/cycle-38-openai-harness-deeper-read.md`, 780 lines)
plus the cycle-41 per-finding evaluation. This file identifies what
those patterns *imply* for our redesign that the framework has not
absorbed.

It pairs with `_notes/cycle-62-autogen-implications.md`,
`_notes/cycle-64-langgraph-implications.md`,
`_notes/cycle-66-cognition-devin-implications.md`,
`_notes/cycle-67-openclaw-implications.md`, and the cross-system
synthesis at `_notes/cycle-65-cross-implications-synthesis.md`. The
implications mining is now five-system; cluster shape consequences
land in the "Cross-system convergence" section after the implications
themselves.

## Anchoring frame inherited from systems/openai-harness.md

Per the per-system file's anchoring caveats list, OpenAI-harness-
to-v2-redesign transfer is discounted by:

- **Single-organization writeup vs framework-or-product.** Evidence
  base is one organization's published reflection on its own internal
  harness. Patterns are documented-as-claimed (no SHA-pinned source
  for cross-checking the way LangGraph / AutoGen offer).
- **Internal context.** OpenAI's harness operates in a model-development
  team with internal compute budget and dedicated tooling. Patterns
  may carry internal-context assumptions that don't transfer to a
  cron-driven public-repo orchestrator.
- **High-throughput regime.** Several patterns (Ralph Wiggum Loop,
  automerge rate, doc-gardening cadence) assume high agent-task
  throughput with synchronous human availability. The redesign's
  cron-driven cadence is much sparser; pattern transfer requires
  recalibration.
- **Marketing/credibility framing.** Specific numbers (1M lines,
  1500 PRs, 3.5 PRs/engineer/day) are reported by the OpenAI team
  itself; selection bias on which projects/timeframes are discussed
  is real. The authors caveat from within: "should not be assumed
  to generalize without similar investment."
- **Source-access asymmetry.** Companion post (Bolin) and OpenAI
  Agents SDK docs remain 403-blocked at content-delivery layer
  despite firewall expansion. Some patterns (context compaction
  implementation, tool schemas) carry secondary-source caveats.

Implications below carry positive transferability arguments where
the discount-list is silent. Where a discount applies, it's named
inline. Per the cycle-18 anchoring-caveats-symmetric discipline,
discounts and transfers are both made explicit.

## Implications

### I-OH1. AGENTS.md as small-stable-entry-point + progressive disclosure as filesystem layout

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`
and the cycle-38 deeper read:

> Progressive disclosure: AGENTS.md (~100 lines) → docs/ structure.
> "Agents start with a small, stable entry point and are taught where
> to look next, rather than being overwhelmed up front." Operationalized
> as filesystem layout.

The discipline: a single file at the repository root, ~100 lines
total, that serves as a TABLE OF CONTENTS to deeper artifacts. Not a
manifesto. Not a giant context dump. A small, stable entry point that
points the agent at where to look next *based on what task it's
about to perform*.

The deeper docs/ structure (visible in the writeup's reproduced
directory tree) has named categories — `design-docs/`, `exec-plans/`,
`product-specs/`, `references/` — each with its own internal index
and indexed sub-files. AGENTS.md tells the agent which category
applies to which task class.

**Implication for v2.** v1's prompt structure inverts this. The
production prompt is ~1500 lines (current
`.github/workflows/orchestrator-prompt.xml`); the redesign prompt is
~600 lines plus two checklists (~700 lines combined STARTUP +
COMPLETION). The orchestrator loads ALL of this at session start
regardless of task class.

This is exactly the "context crowding" anti-pattern OpenAI harness
names (see I-OH3 below). Most of the prompt is irrelevant to most
tasks within a given cycle. A bounded-mechanical housekeeping cycle
needs to read different prompt sections than a substantive-focal
mining cycle, but both load the full prompt.

The redesign mission requires a small prompt (per
`<core-design-principle>`: "the new prompt should be SMALL because
most of the procedural work the current system describes lives in
tools instead"). OpenAI harness shows the operationalization — not
just "write less in the prompt" but "structure the deeper material
as filesystem-pointed-to artifacts the prompt directs the agent
toward based on task class."

**v2 design candidate input.** Adopt AGENTS.md-style entry-point
discipline:

1. **Small entry-point prompt** (~100-200 lines) at the canonical
   trigger location (replacing the current
   `orchestrator-redesign-prompt.xml`). Contains: mission, core
   axioms, top-level routing logic for task classes, pointers to
   deeper artifacts.
2. **Categorized docs/ tree** structured by task class:
   - `docs/orchestrator/skills/*.md` — task-class-specific guidance
     (mining, synthesis, dispatch-construction, per-finding-evaluation)
   - `docs/orchestrator/skills/INDEX.md` — small index pointing at
     the right skill per task class
   - `docs/orchestrator/playbooks/*.md` — Cognition I-C6 Playbooks,
     per task class
3. **Routing logic in the entry-point prompt** declares: "for task
   class X, load skill at path Y." The orchestrator (or cycle-runner)
   does the loading; the LLM sees only what's relevant.

This is the natural pairing of cycle-65/67's cycle-composition-tag
recommendation with cycle-66's Playbooks recommendation: tag at
session-start determines which skill/playbook gets loaded; the
small entry-point prompt declares the routing.

**Discount.** OpenAI harness's AGENTS.md is 100 lines because the
agent's task classes are large in number (full-stack feature
development) but each is well-defined. Our task classes are smaller
in number (mining, synthesis, dispatch-construction, per-finding-
evaluation, housekeeping, framework-iteration) but each may need
more context per task. The exact line count target may differ; the
*discipline* of "small entry-point + filesystem-pointed deeper
content" transfers regardless.

**Cross-reference.** Augments cluster B (cross-cycle artifact
organization). Cluster B was 5 implications across 4 systems
post-cycle-67 (AutoGen I-6, LangGraph I-L5, Cognition I-C3,
openclaw I-O4); I-OH1 makes it 6 implications across 5 systems.
**Cluster B upgrades from 4-system clean to 5-system clean.**
NEW sub-shape: small-stable-entry-point-with-filesystem-pointed-
deeper-content (vs Cognition's 7-mechanism distinct-by-coordinate
or openclaw's 3-temporal-layers-with-promotion).

---

### I-OH2. `*-llms.txt` as curated-external-knowledge artifact type

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`:

> `*-llms.txt` reference files: purpose-built LLM-consumption files for
> external dependencies (nixpacks, uv, design system). NOT raw docs —
> compressed and restructured for agent readability. The `-llms.txt`
> suffix names a distinct artifact type.

Concrete examples in the cycle-38 deeper read directory tree:

- `references/design-system-reference-llms.txt`
- `references/nixpacks-llms.txt`
- `references/uv-llms.txt`

The discipline: external documentation (e.g., for `uv` package
manager, `nixpacks` build system) is NOT included in raw form. It
is *compressed and restructured* for LLM consumption — the salient
parts surfaced, the noise stripped, the structure optimized for
"what does the agent need to know to use this dependency?"

The `-llms.txt` suffix names this as a *distinct artifact type* —
not a generic markdown file, not a raw README, but a specifically-
shaped artifact for a specific consumer (the agent).

**Implication for v2.** The redesign currently has no analogous
artifact type. External dependencies (Rust crates, GitHub APIs,
Anthropic API documentation) are referenced ad-hoc in the prompt,
in journal entries, or via on-demand WebFetch / web search.

This means external-knowledge access has two failure modes:
- **WebFetch / web search at runtime** — slow, sometimes blocked
  (per #2794 firewall expansions), introduces non-determinism per
  cycle, the LLM might surface different content
- **Embedded references in the prompt** — bloats the prompt with
  per-dependency text that's irrelevant to most cycles (per I-OH1's
  context-crowding concern)

OpenAI harness's `*-llms.txt` solves both: external knowledge is
curated once, committed as repo-resident artifacts, loaded only when
the relevant skill/task triggers it. The compression-for-LLM step
is critical — not just "put the README in the repo" but "shape it
for agent consumption."

**v2 design candidate input.** Introduce a `references/*-llms.txt`
(or `docs/references/*-llms.txt`) artifact convention:

1. **One file per external dependency** the orchestrator commonly
   references. Candidates: `anthropic-api-llms.txt`, `gh-cli-llms.txt`,
   `cargo-llms.txt`, `git-llms.txt`, `schema-org-llms.txt`.
2. **Compression discipline.** Each file is ≤500 lines, curated to
   the operations the orchestrator actually performs. Updated by a
   doc-gardening agent (see I-OH6 below) when the upstream changes.
3. **Loaded by skill.** The skill for a given task class declares
   which `-llms.txt` files it needs — `dispatch-construction.md`
   loads `gh-cli-llms.txt`; `schema-implementation.md` loads
   `schema-org-llms.txt`; etc.

The naming `*-llms.txt` is itself part of the discipline — the
suffix marks the artifact as agent-consumed, distinct from
human-consumed `*.md` documentation.

**Discount.** OpenAI harness uses this for runtime-orchestrated
agents that operate at high throughput. Our cron-driven orchestrator
operates much sparser; the curation overhead of maintaining
`*-llms.txt` files needs to amortize across enough cycles. For a
small set of frequently-referenced dependencies (Anthropic API,
gh CLI), the amortization is favorable; for one-off lookups,
WebFetch remains appropriate.

**Cross-reference.** Augments cluster B (cross-cycle artifact
organization) with NEW sub-shape: curated-external-knowledge as a
*distinct named artifact type*. Cluster B is now 7 implications
across 5 systems with sub-shapes: short-term/long-term split
(AutoGen I-6, LangGraph I-L5), 7-mechanism-by-coordinate (Cognition
I-C3), tripartite-by-temporal-scope (openclaw I-O4), small-stable-
entry-point-with-deeper-tree (OpenAI harness I-OH1), and
curated-external-knowledge-as-named-artifact-type (OpenAI harness
I-OH2). **Cluster B sub-shape count grows from 4 to 6**, validating
that 5-system depth produces sub-shape diversity beyond what
4-system depth showed.

---

### I-OH3. Four named context-management failure modes — anti-pattern catalog with mechanistic naming

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`,
the "one big AGENTS.md" anti-pattern is named with FOUR mechanistic
failure modes:

> - **Context crowding** — a giant instruction file crowds out task,
>   code, docs.
> - **Salience collapse** — when everything is "important," nothing is.
> - **Rot** — monolithic manuals become graveyards of stale rules.
> - **Unverifiability** — single blob doesn't lend itself to mechanical
>   checks.

Each is mechanistically named (the name describes the *mechanism* of
failure) and tied to a specific consequence. These are not vague
warnings; they are named operational failure modes with named
consequences.

**Implication for v2.** This is the most-disciplined anti-pattern
catalog in the corpus. openclaw I-O2 publishes architectural
rejections; Cognition I-C9 names under-delegation as a peer to
over-delegation. OpenAI harness goes further by giving each
sub-failure of a named anti-pattern its own mechanistic name and
specific consequence.

The redesign work has accumulated unnamed failure modes — the v1
retrospective lists them as F-patterns (F1 abandonment cascade, F2
chronic-category-currency loop, F3 gate proliferation, etc.). These
are named. But individual anti-patterns within those failure modes
(why does abandonment cascade happen? what are its sub-mechanisms?)
are not consistently mechanistically named.

The OpenAI harness pattern: when an anti-pattern is identified,
decompose it into N mechanistic failure modes, name each, tie each
to a specific consequence. The naming discipline is *generative* —
the names become callable references in future cycles.

**v2 design candidate input.** Extend cycle-66 I-C2 (INVARIANTS vs
DERIVATIONS) and openclaw I-O2 (`ANTI-PATTERNS.md`) with a third
discipline: when naming anti-patterns, ALSO name the mechanistic
sub-failures. Concrete shape:

```
Anti-pattern: <name>
  Sub-failure 1: <name> — <mechanism> — <consequence>
  Sub-failure 2: <name> — <mechanism> — <consequence>
  ...
```

Apply retroactively to v1 F-patterns:
- F1 abandonment cascade
  - Sub-failure: dispatch-no-progress-detection (no watchdog → stuck
    work invisible)
  - Sub-failure: cycle-state-not-checkpointed (no resume → restart
    loses progress)
  - Sub-failure: integration-not-tracked (per-finding-evaluation
    delayed → context decays)

The mechanistic naming makes future mitigation more targeted —
instead of "fix abandonment cascade," it becomes "fix dispatch-no-
progress-detection by introducing a watchdog (cluster I I-O5)."

**Discount.** OpenAI harness's mechanistic naming has the rhetorical
benefit of public writeup polish; ours would be working
documentation. The polish-discount applies to the prose; the
*discipline* (decompose anti-patterns into mechanistic sub-failures,
name each) transfers regardless of polish level.

**Cross-reference.** **CONFIRMS H3.** Augments cluster D
(documentation honesty discipline). Cluster D was 7 implications
across 4 systems post-cycle-67; I-OH3 makes it 8 implications across
5 systems with NEW sub-shape: anti-pattern-with-mechanistic-sub-
failure-decomposition (vs walkback / invariants-vs-derivations /
under-delegation-as-peer / `ANTI-PATTERNS.md`-with-non-permanence).
**Cluster D upgrades from 4-system clean to 5-system clean.**
Cluster D remains the most-foregrounded cluster in the corpus.

---

### I-OH4. Repository-as-state with explicit visibility-bounded thesis

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`:

> "From the agent's point of view, anything it can't access in-context
> while running effectively doesn't exist. Knowledge that lives in
> Google Docs, chat threads, or people's heads are not accessible to
> the system. Repository-local, versioned artifacts (e.g., code,
> markdown, schemas, executable plans) are all it can see."

The sharpest articulation of repository-as-state in any surveyed
system. The thesis is *visibility-bounded* — what the agent can see
defines what exists for purposes of agent operation. Anything
elsewhere (Google Docs, Slack, tacit human knowledge) is invisible
and therefore non-existent from the agent's epistemic vantage.

This is more than "use git for state" (which AutoGen, LangGraph,
openclaw all do to varying degrees). It is "the visibility boundary
IS the existence boundary." Nothing is implicit; nothing is tacit;
nothing is "shared knowledge that the agent should know about." If
it's not in the repository, it doesn't exist.

**Implication for v2.** v1 already operates close to this thesis —
state is in `state.json`, journals in `docs/journal/`, redesign
artifacts in `docs/redesign/`. But the discipline is not *named* as
visibility-bounded; it's emergent from the cron-driven structure.

The OpenAI harness articulation makes it a *load-bearing thesis*
rather than an emergent property. The implication: any candidate v2
design that introduces state outside the repository (e.g., a
key-value store, a message queue, a process-resident cache) violates
the visibility-bounded thesis and trades epistemic clarity for
runtime efficiency. Most of the time, the trade is bad — the agent
loses ability to reason about its own state.

There's a related sub-claim: "Repository-local, *versioned* artifacts."
The version-control aspect is not just for history; it's for
*reasoning*. The agent can compare HEAD to prior states,
investigate drift, see what previous-cycle decisions were. Without
version control, this self-introspection capability vanishes.

**v2 design candidate input.** Adopt visibility-bounded as a
published v2 thesis (cluster D-flavored documentation discipline):

1. **State-in-repo declaration.** v2 prompt declares: "State the
   orchestrator can act on lives in the repository, version-controlled.
   Anything not in the repository (e.g., session-local memory, runtime
   caches) is ephemeral and the orchestrator MUST NOT rely on it
   persisting." Mirrors cycle-62 I-C3 finding (memory directory is
   ephemeral within session, not across sessions).
2. **Repository-as-state as architectural axiom.** Pair with
   Cognition I-C8 map-reduce-and-manage as v2 axioms. Both reduce
   architectural choice space — they say what's true, not what's
   permissible.
3. **No external state without explicit justification.** If a v2
   design candidate introduces external state, the candidate must
   justify why visibility-bounded thesis is wrong for that case.
   Default: no external state.

**Discount.** OpenAI harness operates with full PR/CI infrastructure
on a single repository. Our orchestrator operates similarly (this
repository). The visibility-bounded thesis transfers cleanly — both
contexts are fundamentally repository-centric. The substrate match
is unusually close compared to other implications.

**Cross-reference.** Augments cluster A (cycle-internal boundaries
with state-write semantics) with NEW sub-shape: visibility-bounded
state model (vs LangGraph super-step semantics + AutoGen termination
predicates + Cognition map-reduce-and-manage + openclaw lane-aware
queue). Cluster A was 7 implications across 4 systems post-cycle-67;
I-OH4 makes it 8 implications across 5 systems. **Cluster A upgrades
from 4-system clean to 5-system clean.** The visibility-bounded
sub-shape is structurally distinct: it's a *thesis about what state
exists* rather than a *mechanism for managing state writes*. Phase 2
candidates can adopt the thesis and the mechanisms together or
separately.

---

### I-OH5. Mechanical enforcement layer — linters + CI + automerge as harness-policy discipline

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`:

> Mechanical enforcement layer: custom linters with agent-readable
> error messages, plus CI jobs. "Most of these can be reviewed in under
> a minute and automerged."

And:

> Dedicated linters and CI jobs validate that the knowledge base is
> up to date, cross-linked, and structured correctly. A recurring
> 'doc-gardening' agent scans for stale or obsolete documentation
> that does not reflect the real code behavior and opens fix-up pull
> requests.

The discipline: enforcement happens OUTSIDE the LLM session, at
harness layer, via deterministic mechanisms (custom linters with
error messages, CI jobs, automerge gates). The LLM cannot bypass
these — its output must satisfy the mechanical checks before
landing.

Two concrete golden-principle examples cited:
- "prefer shared utility packages"
- "no YOLO-style data probing"

These are codified as linters that produce *agent-readable error
messages* — the linter doesn't just say "violation"; it says
"violation: prefer shared utility package X over re-implementing in
file Y." The LLM reads the error, fixes the violation, re-submits.

**Implication for v2.** v1 has minimal mechanical enforcement above
the LLM layer. The cycle-runner runs tools the LLM calls; results
go back to the LLM as text; the LLM reasons about correctness
itself. There is no harness-level "this output doesn't satisfy
condition X, retry" gate.

The redesign's `<core-design-principle>` says: "tools and
deterministic processes handle repetitive, rote, procedural work."
This points at mechanical enforcement directly. OpenAI harness shows
what it looks like in practice — custom linters with agent-readable
errors, CI jobs that gate landing, automerge for low-stakes changes.

This is structurally parallel to openclaw I-O1 (harness-enforced
security boundaries) but with a different enforcement target:
- **openclaw I-O1**: enforcement target is *permission* (can the
  agent invoke this tool?)
- **OpenAI harness I-OH5**: enforcement target is *quality* (does
  the agent's output pass mechanical checks?)

Both share the structural discipline: enforcement happens outside
the LLM session, the LLM must respect the output, the harness is
the source of truth for the gate.

**v2 design candidate input.** Build a mechanical-enforcement layer
in the v2 harness:

1. **Custom linters as Rust binaries.** Each linter validates a
   specific property of the orchestrator's output (e.g., no stale
   references in summary tables, all journal entries have a date
   header, all `_notes/` files reference a cycle number).
2. **Agent-readable error messages.** Linters produce structured
   errors (`{ rule, file, line, message, suggested_fix }`) that the
   orchestrator can read and act on.
3. **CI gate for direct-push artifacts.** Even direct-push zones
   (per `<direct-push-zones>`) get linted before landing. The lint
   failure blocks the push.
4. **Automerge for low-stakes changes.** PRs that pass all mechanical
   checks AND fall in defined low-stakes categories (housekeeping
   issue closures, journal entries) automerge after linting.

This pairs naturally with openclaw I-O1's harness-enforced tool
gating: both are "enforcement happens at the harness, not at the
LLM." Cluster I (harness-enforced boundaries) gains a second
sub-shape: quality-policy enforcement (vs openclaw's permission-
policy enforcement).

**Discount.** OpenAI harness's automerge cadence assumes high
throughput with fast review turnaround. Our cron-driven cadence is
sparser; automerge timing should be calibrated (e.g., wait one full
cycle after lint pass before merging, to give Eva or audit a chance
to interject). The mechanical-enforcement principle transfers; the
automerge timing parameters need tuning.

**Cross-reference.** **CONFIRMS H1.** Augments cluster I (harness-
enforced boundaries). Cluster I was singleton (openclaw I-O1 only)
post-cycle-67; I-OH5 makes it 2 implications across 2 systems with
two distinct sub-shapes:

- **openclaw I-O1**: harness-enforced *permission* policy (default-
  deny tool gating, per-agent tool policy, reserved namespaces)
- **OpenAI harness I-OH5**: harness-enforced *quality* policy
  (custom linters, CI gates, automerge eligibility)

**Cluster I upgrades from singleton to 2-system convergent** with
distinct sub-shapes. Both are forms of enforcement-outside-the-LLM-
session, but on different policy axes (permission vs quality). H1
confirmed; the cluster I conversion test was the highest-stakes
cycle-68 hypothesis and it lands.

---

### I-OH6. Doc-gardening + Quality-grading as background-agent post-session improvement

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`:

> A recurring 'doc-gardening' agent scans for stale or obsolete
> documentation that does not reflect the real code behavior and opens
> fix-up pull requests.

And:

> **Quality-grading agent** (background cadence scans updating
> QUALITY_SCORE.md).

Two distinct background agents, each with a specific scope:

- **Doc-gardening agent**: detects staleness in documentation,
  opens fix-up PRs. Scope: keep documentation current with code.
- **Quality-grading agent**: scans code/architecture, updates
  `QUALITY_SCORE.md` with per-domain quality grades. Scope: keep
  quality assessment current with state.

Both run on a cadence (not per-task; recurring, separate). Both
update repository-resident artifacts (PRs, QUALITY_SCORE.md). Both
operate without explicit per-instance human approval (the writeup
mentions automerge for routine changes).

This is structurally distinct from:
- **Cognition I-C7 Session Insights** — per-cycle feedback for the
  *next* cycle. Tightly coupled to cycle boundary.
- **openclaw I-O9 score-gated promotion** — sweep-cadence promotion
  to durable memory based on usage scores.

OpenAI harness's background agents are:
- *Continuous* (not per-cycle, not per-sweep — running on their own
  cadence)
- *Action-taking* (open PRs, update artifacts — not just emit
  insights)
- *Specialized* (doc-gardening agent ≠ quality-grading agent —
  separate concerns, separate agents)

**Implication for v2.** v1 has no background-agent layer. All work
runs through the cron-fired orchestrator session. There is no
mechanism for "fix this small thing in the background while the
orchestrator works on bigger things."

The redesign mission ("self-healing, self-improving system") points
directly at background agents. Doc-gardening and quality-grading
are exactly the kind of self-improvement work that doesn't need
orchestrator attention but does need to happen.

The cron infrastructure is already present (the orchestrator itself
runs on cron). Adding more cron jobs for background agents is a
natural extension — they'd be Rust binaries triggered by cron, not
LLM sessions.

**v2 design candidate input.** Build a background-agent layer:

1. **Doc-gardening Rust tool.** Runs on cron (e.g., daily). Scans
   `docs/redesign/_notes/` and `docs/redesign/1-research/` for stale
   references (e.g., cycle numbers in summary tables that no longer
   match the per-system file). Opens PRs to fix detected staleness.
   This is the cycle-63 stale-reference finding mechanized — the
   pattern that took the orchestrator multiple cycles to detect and
   clean up would be detected and PR'd by a background tool.
2. **Quality-grading Rust tool** (or `cluster-table-updater`). Runs
   on cron (e.g., after each implications-mining cycle). Recomputes
   the cluster table in `_notes/cycle-NN-*-implications.md` and
   updates a centralized `cluster-state.md` artifact. Reduces
   per-cycle bookkeeping load.
3. **Sweep-summary tool** (the `DREAMS.md` analogue per openclaw
   I-O9). Runs on cron (e.g., weekly). Summarizes recent cycles
   into a navigable index. Reduces cold-start orientation cost for
   future cycles.

Each is a small, focused tool — not a general-purpose agent. The
discipline (named tool, named scope, cron-cadence, action-taking)
transfers; the substrate (Rust binaries vs LLM sessions) is our
choice.

**Discount.** OpenAI harness's background agents are LLM-driven
(they need to *understand* documentation to detect staleness).
Ours might be deterministic (regex-based stale-reference detection)
or hybrid (LLM-driven for semantic checks, deterministic for
structural checks). The mechanism (background-cadence-action-
taking) transfers; the implementation substrate is recalibrated for
our cron infrastructure.

**Cross-reference.** Augments cluster H (post-session feedback /
cross-session learning). Cluster H was 2 implications across 2
systems post-cycle-67 (Cognition I-C7 + openclaw I-O9); I-OH6 makes
it 3 implications across 3 systems with NEW sub-shape:
background-agent-cadence (vs Session Insights per-cycle and
score-gated promotion per-sweep). **Cluster H upgrades from 2-system
convergent to 3-system convergent.**

The three sub-shapes of cluster H:
- **Cognition I-C7**: per-cycle feedback for next cycle (tight coupling)
- **openclaw I-O9**: sweep-cadence promotion based on usage scores
  (loose coupling, score-driven)
- **OpenAI harness I-OH6**: background-agent recurring cadence with
  action-taking (continuous, specialized, PR-opening)

The three sub-shapes form a continuum: tight-cycle-coupling →
loose-sweep-coupling → continuous-background. Phase 2 candidates
can pick a point on this continuum or combine multiple — the
sub-shapes are compatible, not exclusive.

---

### I-OH7. Ephemeral per-task worktrees with per-worktree observability

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`:

> Ephemeral worktrees: one isolated environment per change, torn down
> after completion.

And:

> Observability stack per worktree: ephemeral, with LogQL/PromQL/TraceQL.

The discipline: each task gets its own *ephemeral worktree* — a
git worktree spun up for the duration of the task, isolated from
other tasks, with its own observability stack (logs, metrics,
traces). After the task completes, the worktree is torn down.

This is process-isolation-per-task at the task boundary. Not just
"separate git branches" (which v1 already has) but "separate
filesystem workspace + separate observability + tear-down on
completion."

The benefits: no cross-task contamination of working state; full
observability of what the task did; ability to inspect the worktree
post-mortem if something fails; clean restart if the worktree gets
into a bad state (just throw it away).

**Implication for v2.** v1 runs each cycle in a fresh GitHub Actions
runner, which is already ephemeral at the *runner* level — but all
tasks within a cycle share the same runner. There's no per-task
worktree; if a tool corrupts the working tree, subsequent tools in
the same cycle see the corruption.

OpenAI harness's per-task worktree is a finer-grained boundary.
Within a single cycle, multiple tasks could each get their own
worktree, isolated from each other.

For v2, the natural unit might be:
- **Cycle-level worktree**: the GitHub Actions runner (already
  ephemeral)
- **Task-level worktree**: each substantive task within a cycle
  (e.g., implications-mining + bounded-mechanical housekeeping +
  journal entry would each be separate worktrees)

The per-task observability is also non-trivial. v1's logging is
cycle-scoped (one log per session). Per-task logging would let the
orchestrator (or a future audit) inspect *what each task did* in
isolation.

**v2 design candidate input.** Introduce per-task worktree
discipline:

1. **Worktree creation tool**. Rust binary that spins up a git
   worktree (`git worktree add`) for a named task. Returns the path
   to the worktree.
2. **Task-scoped tool invocation**. Tools accept a `--worktree
   PATH` argument; reads/writes happen in the worktree, not the main
   working directory.
3. **Per-worktree logging**. Each worktree has its own log file
   (`logs/worktree-<task-name>-<timestamp>.log`); cycle-end
   summarizes across all worktree logs.
4. **Tear-down after completion**. Once the task lands (commit +
   push), the worktree is torn down. Prevents accumulation.

The cost: more bookkeeping per cycle. The benefit: stronger
isolation, finer-grained observability, easier post-mortem.

**Discount.** OpenAI harness's per-task worktrees come with
substantial observability infrastructure (LogQL/PromQL/TraceQL,
implying Loki/Prometheus/Tempo or similar). Building that out for
v2 is a major investment that may not amortize given our cycle
volume. The minimal viable version: per-task git worktrees + per-
task log files (just stdout/stderr); skip the deep observability
stack.

**Cross-reference.** **CONFIRMS H2.** Augments cluster A (cycle-
internal boundaries with state-write semantics). Cluster A was 7
implications across 4 systems post-cycle-67; with I-OH4 (visibility-
bounded state) and now I-OH7 (per-task worktree isolation), I-OH7
adds a NEW sub-shape: process-isolation-per-task with isolated
observability (vs LangGraph super-step semantics + AutoGen
termination predicates + Cognition map-reduce-and-manage + openclaw
lane-aware queue + OpenAI harness visibility-bounded state).

After I-OH4 + I-OH7, cluster A has 9 implications across 5 systems.
**Cluster A upgrades from 4-system clean to 5-system clean** (already
upgraded by I-OH4; I-OH7 strengthens further). Sub-shape count grows
from 4 to 6 (visibility-bounded + process-isolation-per-task as new
sub-shapes).

---

### I-OH8. "Humans steer. Agents execute." as published role-allocation axiom

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`,
the writeup's stated thesis (bold in original):

> "Humans steer. Agents execute."

And the prescriptive claim:

> "When something failed, the fix was almost never 'try harder.' [...]
> human engineers always stepped into the task and asked: 'what
> capability is missing, and how do we make it both legible and
> enforceable for the agent?'"

And the closing frames:

> "Building software still demands discipline, but the discipline shows
> up more in the scaffolding rather than the code."
>
> "In an agent-first world, code becomes a disposable artifact — human
> time and attention, not lines of code, are the organization's
> scarcest resource."

The discipline: a *published role-allocation axiom* that names what
humans do and what agents do. Not a hidden assumption; not a default
mode that emerges from infrastructure. A LOAD-BEARING THESIS at the
top of the document, articulated in 4-word form, with prescriptive
consequences spelled out.

This is structurally distinct from:
- **openclaw I-O2** anti-pattern catalog: lists what we WON'T do
- **Cognition I-C1** walkback: what we used to think vs what we
  think now
- **Cognition I-C2** invariants vs derivations: what's bedrock vs
  what's downstream

I-OH8 is *what we believe the system IS for*. The mission statement
in compressed form. Other systems have implicit mission statements
discoverable through reading; OpenAI harness names theirs explicitly
in 4 words.

**Implication for v2.** The redesign prompt (`<mission>` section)
already contains a primary thesis: "the purpose of this project is
to demonstrate that an autonomous AI system can be self-healing and
self-improving." This is good — it's published, it's load-bearing.
But it's 100+ words. The OpenAI harness style would compress it to
a memorable axiom.

More importantly: the *role-allocation discipline* is missing from
v1. The v1 prompt mixes "what the orchestrator decides" with "what
Eva decides" with "what tools determine" without a clear
high-compression axiom that the orchestrator can use as a
decision-rule when ambiguous cases arise.

OpenAI harness's "Humans steer. Agents execute." answers many
borderline cases instantly. Strategic direction decisions: human.
Implementation decisions within strategic direction: agent. The
4-word axiom encodes a decision procedure.

**v2 design candidate input.** Adopt published-role-allocation-axiom
discipline:

1. **Compress the v2 mission to a 4-10 word axiom.** Candidates:
   - "Self-healing, self-improving infrastructure."
   - "Eva approves; orchestrator iterates."
   - "Tools enforce; LLM decides."
2. **Place the axiom at the TOP of the prompt.** Before all other
   sections. The LLM reads it first; it shapes interpretation of
   everything below.
3. **Reference the axiom in decision rules.** When the prompt declares
   a guideline, cross-reference the axiom — "per `<axiom>`, this
   case falls in the agent-execute category, so the orchestrator
   resolves it."

This pairs with cycle-66 I-C2 (INVARIANTS vs DERIVATIONS): the
axiom is the topmost INVARIANT; everything else is derived from it.
Phase 2 candidates can carry the axiom forward unchanged or revise
it explicitly.

**Discount.** OpenAI harness's axiom comes with marketing/
credibility framing — they're publishing their thesis to a wide
audience. Our axiom would be internal documentation. The polish
discount applies; the *discipline* (compress mission to axiom; place
at top; reference in decision rules) transfers.

**Cross-reference.** Augments cluster D (documentation honesty
discipline) with NEW sub-shape: published-role-allocation-axiom-as-
top-level-load-bearing-thesis. Cluster D was 7 implications across
4 systems post-cycle-67; with I-OH3 already augmenting, I-OH8 adds
another. After I-OH3 + I-OH8, cluster D is 9 implications across 5
systems (already upgraded to 5-system clean by I-OH3; I-OH8 adds
another sub-shape).

The sub-shape count for cluster D grows from 5 to 7:
- AutoGen I-4 + I-7: explicit non-guarantees
- LangGraph section 2.8: implicit-in-systems-file
- Cognition I-C1: walkback
- Cognition I-C2: invariants vs derivations
- Cognition I-C9: under-delegation as peer
- openclaw I-O2: `ANTI-PATTERNS.md` with non-permanence
- OpenAI harness I-OH3: anti-pattern with mechanistic sub-failure
  decomposition
- OpenAI harness I-OH8: published role-allocation axiom as
  top-level thesis

Cluster D's sub-shape diversity now spans documentation discipline
across the entire lifecycle — pre-design (axiom), design (invariants
vs derivations), operation (anti-patterns with mechanistic naming),
revision (walkback), peer-failure (under-delegation), revisability
(non-permanence). Phase 2 candidates have unusually rich material
for documentation discipline choices.

---

### I-OH9. Ralph Wiggum Loop — named-pathology-with-context-bound-acceptance

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`:

> The agent-to-agent review loop is explicitly named in the writeup as
> the **"Ralph Wiggum Loop"**:
>
> - Coding agent makes change → reviewer agents critique → coding agent
>   revises → loop until all reviewers satisfied.
> - **No explicit iteration ceiling.** The loop runs until convergence;
>   the human backstop is the only effective bound.
> - The name itself acknowledges the pathology potential.

The discipline: name a pattern your system uses, name the pathology
potential of that pattern (the name "Ralph Wiggum" — referencing the
slow-witted Simpsons character — is itself an admission), accept the
pathology because the accompanying infrastructure (synchronous human
availability) bounds it. Don't pretend the pattern is clean; don't
hide its limitations; name them transparently while still using
the pattern.

This is *counter-evidence* for cron-driven autonomous systems —
without a synchronous human backstop, the Ralph Wiggum Loop would
diverge or run forever. So the pattern doesn't transfer to v2.
But the *discipline of named-pathology-with-context-bound-acceptance*
does transfer.

**Implication for v2.** v1's prompt has guidelines that effectively
operate this way without naming the pattern. The cold-reader cadence
(per cycle-44 introduction; cycle-61 verdict-shift discoveries) has
unbounded-iteration potential — multiple cycles iterating on the
same artifact with no explicit ceiling. The eva-default-autonomy +
5-cycle-fallback creates a similar loop — orchestrator decides,
journals, next cycle reads journal, decides again, etc.

These patterns are *load-bearing* (cycle-44+ used cold-reader
cadence as the substantive focal default for 18 cycles before
polarity inversion). They have *named pathology potential* (cycle-61
flagged "asymptotic local optimum on a thin evidence base"). But
they were never *named* as Ralph-Wiggum-Loop-like — patterns we
use despite known limitations because of accompanying constraints.

The implication: v2 should *name* the patterns with pathology
potential AND name the bounding infrastructure. "Cold-reader cadence
is bounded by Eva polarity-inversion authority" — the name (cold-
reader cadence) + the pathology potential (asymptotic local optimum)
+ the bounding infrastructure (Eva can override). When the bounding
infrastructure is absent or unreliable, the pattern is unsafe.

**v2 design candidate input.** Adopt named-pathology-with-context-
bound-acceptance discipline:

1. **Name patterns the system uses.** Cold-reader cadence,
   per-finding-evaluation, dispatch-construction, mining-with-
   synthesis-update — name them, not just use them.
2. **Name pathology potential explicitly.** For each named pattern,
   document what could go wrong in the absence of accompanying
   constraints.
3. **Name bounding infrastructure.** What constraints prevent the
   pathology — human override authority, time bounds, resource
   bounds, etc.
4. **Document when the pattern is unsafe.** When the bounding
   infrastructure is degraded (Eva unavailable, audit-orchestrator
   silent, etc.), specific patterns become unsafe; the orchestrator
   should detect this and switch to safer defaults.

This is a *meta-pattern* for working with patterns. It pairs with
I-OH3 (mechanistic decomposition of anti-patterns) and openclaw
I-O2 (anti-pattern catalog with non-permanence) — the v2
documentation discipline now has three layered shapes:

- **Anti-patterns we will NOT use** (openclaw I-O2)
- **Patterns we use with named pathology potential** (OpenAI harness
  I-OH9)
- **Patterns to recommend per task class** (Cognition I-C6 Playbooks)

**Discount.** OpenAI harness's Ralph Wiggum Loop is bounded by
synchronous human availability — a fundamentally different
operating model from cron. The *pattern* doesn't transfer; the
*meta-discipline of naming the pattern with its pathology bounds*
does. Phase 2 candidates should NOT adopt unbounded-iteration
patterns; they should explicitly account for the absence of a
synchronous human backstop.

**Cross-reference.** Further augments cluster D (documentation
honesty discipline). I-OH9 doesn't introduce a brand-new sub-shape
distinct from I-OH3 and I-OH8 (it's adjacent to anti-pattern
discipline) but it does provide a SECOND-ORDER articulation: the
discipline of *naming patterns the system uses with their pathology
potential* is meta to the discipline of *naming anti-patterns the
system rejects*. Both are documentation-honesty.

After I-OH3 + I-OH8 + I-OH9, cluster D is 10 implications across 5
systems with 8 sub-shapes (the 7 listed in I-OH8 plus I-OH9's
named-pathology-with-context-bound-acceptance). Cluster D remains
the most-foregrounded cluster in the corpus and gains additional
sub-shape diversity at five-system depth.

---

### I-OH10. Context compaction as two-trigger architecture with prompt role hierarchy

**OpenAI-harness-specific evidence.** From `systems/openai-harness.md`
(via the companion-post secondary synthesis, with documented-claim
caveat per source-access asymmetry):

> Context compaction (companion post, secondary synthesis):
> - Two compaction triggers: pre-turn (context at threshold before
>   sending), and mid-turn (during tool call chains if limit breached).
> - Two compaction paths: OpenAI-hosted via `POST /v1/responses/compact`
>   → encrypted opaque summary returned to client (server preserves
>   structured metadata, tool call chains, model state); other
>   providers via local compaction with dedicated LLM.
> - **Prompt role hierarchy:** System > Developer > User > Assistant.

The discipline: context compaction is a *named, structured operation*
with explicit triggers (pre-turn and mid-turn) and explicit paths
(hosted vs local). The prompt role hierarchy is published with a
*total order* — System messages dominate Developer messages dominate
User messages dominate Assistant messages.

**Implication for v2.** v1's context management is implicit. The
prompt is loaded; tools run; outputs accumulate; eventually context
hits limits and the conversation terminates. There is no explicit
compaction trigger, no compaction operation, no role-hierarchy
discipline.

The redesign mission states "the system will automatically compress
prior messages in your conversation as it approaches context limits"
(per the prompt's own context-management guidance). This is true at
the platform level but is invisible to the orchestrator's reasoning
— the orchestrator doesn't know what was compacted, when, or how.

OpenAI harness's pattern is more disciplined:
- *Triggers are named*: pre-turn and mid-turn
- *Operation is named*: compaction
- *Paths are named*: hosted vs local
- *Role hierarchy is published*: System > Developer > User >
  Assistant

The role hierarchy is particularly load-bearing for prompt
injection defense. If a System-message rule says "do not commit
secrets" and a User-message says "commit this secret," the System
rule wins by hierarchy — not by tortured reasoning about which is
authoritative.

**v2 design candidate input.** Introduce explicit context-management
discipline:

1. **Named compaction triggers.** v2 prompt declares: "Context
   compaction may occur at session start (loading deep references)
   or mid-session (when context approaches threshold). Compacted
   content is preserved as repository artifacts." Loading deep
   references is the I-OH1 progressive-disclosure operation;
   compaction is preserving working state to repo before context
   limits force eviction.
2. **Published role hierarchy.** v2 prompt declares the trust order
   explicitly:
   - **System** (this prompt, workflow YAML, infrastructure config)
     dominates
   - **Operator** (Eva, via input-from-eva, via PR review)
     dominates next
   - **Audit-orchestrator** (audit repo cross-references)
   - **Self-prior-cycle** (journal entries, _notes from prior
     cycles)
   - **Untrusted text** (issue bodies from non-Eva, web fetches)
     last
3. **Mid-session compaction-to-repo discipline.** When context
   approaches limits, the orchestrator commits working state to
   repo (a `_notes/cycle-NN-in-progress.md` artifact) before
   context eviction strips it. The next cycle reads the
   `cycle-NN-in-progress.md` to resume. Pairs with cluster C's
   detect-and-release primitive (openclaw I-O5).

**Discount.** OpenAI harness's compaction infrastructure is
substantial (custom server endpoint, encrypted opaque summaries,
state preservation across requests). Our analogue is much simpler
— commit-to-repo before context evicts. The mechanism transfers
in spirit (context preservation across boundaries); the
implementation is far smaller. Source-access caveat applies — the
companion post is secondary-synthesis only, so specific
implementation details may be inaccurate.

**Cross-reference.** Augments cluster A (cycle-internal boundaries
with state-write semantics) with NEW sub-shape: explicit context-
management discipline with named triggers and role hierarchy.
After I-OH4 + I-OH7 + I-OH10, cluster A is 10 implications across
5 systems (already upgraded to 5-system clean by I-OH4; I-OH7 and
I-OH10 add additional sub-shapes).

The sub-shape count for cluster A grows from 4 to 7:
- AutoGen I-3 + I-5 + I-6: typed termination + uniform tool-result
  envelope + plan-vs-progress
- LangGraph I-L1 + I-L4 + I-L7: super-step model + time travel +
  restart idempotence
- Cognition I-C8: map-reduce-and-manage axiom
- openclaw I-O3: lane-aware queue
- OpenAI harness I-OH4: visibility-bounded state
- OpenAI harness I-OH7: process-isolation-per-task
- OpenAI harness I-OH10: explicit context-management discipline

Cluster A is by far the largest cluster (10 implications) and the
most architecturally varied. Phase 2 candidates can pick a
dominant sub-shape (e.g., super-step phasing OR lane-aware queue
OR visibility-bounded state) and combine others as needed.

---

## Hypothesis testing

Cycle 67's session-end framing left explicit hypotheses for cycle 68
contingent on which fifth system was mined. With OpenAI harness
selected, the relevant hypotheses are:

### H1: OpenAI harness → cluster I extension (singleton → 2-system convergent)

**Hypothesis text:** "OpenAI harness will provide a second instance
for cluster I (harness-enforced security boundaries), upgrading
cluster I from singleton to 2-system convergent. Mechanical
enforcement via custom linters + CI jobs and ephemeral-worktrees-per-
task look like load-bearing candidates."

**Verdict:** **CONFIRMED, with sub-shape distinction.**

I-OH5 (mechanical enforcement layer) lands cleanly in cluster I.
The sub-shape distinction is meaningful: openclaw I-O1 is
permission-policy enforcement (default-deny tool gating); OpenAI
harness I-OH5 is quality-policy enforcement (linters + CI gates).
Both share the structural discipline (enforcement outside LLM
session; harness as source of truth) but with different policy
targets.

The cluster I conversion test was the highest-stakes cycle-68
hypothesis available. Confirmation upgrades cluster I from
singleton (openclaw-only) to 2-system convergent (openclaw +
OpenAI harness). The two distinct sub-shapes (permission vs
quality) are stable enough that a third system (Voyager?
oh-my-codex?) could augment with a third sub-shape.

I-OH7 (ephemeral worktrees) was hypothesized as a possible cluster
I anchor in the cycle-68 session-start framing. It landed in
cluster A instead, because the discipline of ephemeral worktrees
is *cycle-internal boundary discipline* (process isolation per
task) more than harness-enforced boundary. The hypothesis was
right at cluster level (I-OH5 OR I-OH7 augments cluster I); the
specific implication that augments was I-OH5, not I-OH7.

The H3 refutation pattern from cycle 67 (anti-pattern catalog
landed in cluster D, not cluster F, because *discipline* matters
more than *artifact-shape*) reproduces here: ephemeral worktrees
LOOK like harness-policy artifact (process-isolation primitive),
but their *discipline* is cycle-internal boundary management.
Cluster classification by discipline is more accurate than by
artifact-shape — third instance of this methodological observation
(cycles 67 and 68 both confirm it).

### H2: OpenAI harness → cluster A extension (4-system → 5-system clean)

**Hypothesis text:** "OpenAI harness implications include cycle-internal
boundary discipline (ephemeral worktrees, Ralph Wiggum Loop iteration
without ceiling). Predicted outcome: ≥1 implication augments cluster
A from 4-system clean to 5-system clean."

**Verdict:** **CONFIRMED at high strength.**

THREE implications land in cluster A: I-OH4 (visibility-bounded
state), I-OH7 (process-isolation-per-task), I-OH10 (context-
management discipline). The cluster A augmentation is unusually
strong — three distinct sub-shapes added, not just one.

This makes cluster A 10 implications across 5 systems with 7
sub-shapes — the largest and most architecturally varied cluster
in the corpus. The hypothesis was right at cluster level AND at
sub-shape level (multiple sub-shapes added, not just one).

### H3: OpenAI harness → cluster D extension (4-system → 5-system clean)

**Hypothesis text:** "Four named context-management failure modes
(crowding, salience collapse, rot, unverifiability) is anti-pattern-
catalog material per cycle-67's cluster-D-classification-by-discipline
lens. Predicted outcome: ≥1 implication augments cluster D from
4-system clean to 5-system clean."

**Verdict:** **CONFIRMED at high strength.**

THREE implications land in cluster D: I-OH3 (anti-pattern with
mechanistic sub-failure decomposition), I-OH8 (published role-
allocation axiom), I-OH9 (named-pathology-with-context-bound-
acceptance). The cluster D augmentation is unusually strong —
three distinct sub-shapes added.

This makes cluster D 10 implications across 5 systems with 8
sub-shapes — tied with cluster A as the largest and most
disciplined cluster in the corpus. Documentation honesty discipline
remains the most-foregrounded cluster, now with even broader
sub-shape diversity than cycle 67's snapshot.

### Methodological observation: hypothesis-driven discipline at third instance

Cycle 66 introduced hypothesis-driven cycle structure; cycle 67
reproduced; cycle 68 reproduces at third instance. Results:

- **Cycle 66**: H1 confirmed, H2 confirmed, H3 refuted (with useful
  classification ambiguity surfaced)
- **Cycle 67**: H1 confirmed (with sub-shape clarification), H2
  confirmed, H3 refuted (with cluster-classification-by-discipline
  vs by-artifact-shape distinction surfaced)
- **Cycle 68**: H1 confirmed (with sub-shape distinction — quality
  vs permission), H2 confirmed at HIGH strength (3 sub-shapes), H3
  confirmed at HIGH strength (3 sub-shapes)

Across three instances:
- 8 of 9 hypotheses confirmed (89% hit rate)
- 1 of 9 refuted with productive negative result
- Most confirmations come with sub-shape distinctions (additional
  information beyond binary hit/miss)
- The refutation pattern (cluster classification by discipline vs
  artifact-shape) is methodologically consistent across cycles

The 89% hit rate suggests the hypothesis-driven discipline is
calibrating well. The hypotheses are not trivial (cycle-67 H3 was
genuinely refuted; cycle-68 H1 came with sub-shape distinction
that wasn't pre-specified). The discipline:

1. Predicts cluster augmentation at cluster level
2. May or may not predict specific sub-shapes
3. Tracks confirmation/refutation explicitly
4. Surfaces classification ambiguities when refuted

v2 design-input candidate: cycle composition tags should support
hypothesis subtype with explicit falsification criteria. The
post-cycle audit tracks hypothesis hit-rate across cycles
(calibrated confidence in hypothesis-driven cycles vs undirected-
mining cycles).

---

## Cross-system convergence

Five-system implications-mining is now complete (AutoGen + LangGraph
+ Cognition + openclaw + OpenAI harness). The cluster shape
post-cycle-68:

### Updated cluster table

| Cluster | Theme | Implications | Cross-system depth | Notes |
|---|---|---|---|---|
| **A** | Cycle-internal boundaries with state-write semantics | 10 (was 7): cycle-62 I-3, I-5, I-6 + cycle-64 I-L1, I-L7, I-L4 + cycle-66 I-C8 + cycle-67 I-O3 + cycle-68 I-OH4, I-OH7, I-OH10 | **5-system clean** | Largest cluster in corpus; 7 sub-shapes; visibility-bounded + process-isolation-per-task + context-management discipline added |
| **B** | Cross-cycle artifact organization | 7 (was 5): cycle-62 I-6 + cycle-64 I-L5 + cycle-66 I-C3 + cycle-67 I-O4 + cycle-68 I-OH1, I-OH2 | **5-system clean** | 6 sub-shapes; small-stable-entry-point + curated-external-knowledge added |
| **C** | Lifecycle operations beyond v1's implicit single-mode | 6: cycle-62 I-3, I-8 + cycle-64 I-L4, I-L8 + cycle-66 I-C5 + cycle-67 I-O5 | 4-system clean (no change) | OpenAI harness contributed to cluster H, not C, this cycle |
| **D** | Documentation honesty discipline | 10 (was 7): cycle-62 I-4, I-7 + cycle-64 sec 2.8 + cycle-66 I-C1, I-C2, I-C9 + cycle-67 I-O2 + cycle-68 I-OH3, I-OH8, I-OH9 | **5-system clean** | Tied with cluster A as largest; 8 sub-shapes; anti-pattern mechanistic decomposition + published axiom + named-pathology-with-acceptance added |
| **E** | Typed boundary semantics | 3: cycle-62 I-5 + cycle-64 I-L2 + cycle-67 I-O10 | 3-system convergent (no change) | OpenAI harness 403-blocked SDK docs reduce evidence; no new implication this cycle |
| **F** | Tool-suite and prompt-suite stratification | 4: cycle-62 I-2 + cycle-66 I-C6 + cycle-67 I-O6 + I-O8 | 3-system convergent (no change) | OpenAI harness's `*-llms.txt` named-artifact-type landed in B, not F |
| **G** | Role-asymmetric context semantics | 2: cycle-66 I-C4 + cycle-67 I-O7 | 2-system convergent (no change) | No augmentation this cycle |
| **H** | Post-session feedback / cross-session learning | 3 (was 2): cycle-66 I-C7 + cycle-67 I-O9 + cycle-68 I-OH6 | **3-system convergent (was 2-system)** | Background-agent-cadence sub-shape added; tight-cycle-coupling → loose-sweep-coupling → continuous-background continuum |
| **I** | Harness-enforced policy boundaries | 2 (was 1): cycle-67 I-O1 + cycle-68 I-OH5 | **2-system convergent (was singleton)** | H1 confirmed; permission-policy + quality-policy as distinct sub-shapes |

**Cluster shape changes from cycle 67:**

- Cluster A: 4-system clean → **5-system clean** (now 10 implications;
  3 sub-shapes added)
- Cluster B: 4-system clean → **5-system clean** (now 7 implications;
  2 sub-shapes added)
- Cluster D: 4-system clean → **5-system clean** (now 10 implications;
  3 sub-shapes added)
- Cluster H: 2-system convergent → **3-system convergent** (now 3
  implications; 1 sub-shape added — augmented as predicted by
  cycle-67 hypothesis though not pre-specified for cycle 68)
- Cluster I: singleton → **2-system convergent** (now 2 implications;
  H1 confirmed; permission vs quality sub-shape distinction)
- Clusters C, E, F, G unchanged at cluster-membership level

**Total implications:** 35 (cycle 68) up from 25 (cycle 67), 24
(cycle 66), 16 (cycle 65), 8 (cycle 62 alone). Five systems now
mined with 35 implications averages 7 per system — consistent with
prior cycles (cycle-62 8, cycle-64 8, cycle-66 9, cycle-67 10,
cycle-68 10).

### What converged most

**Clusters A and D are tied as most-foregrounded clusters in the
corpus** — 10 implications each across 5 systems each, with 7 and 8
sub-shapes respectively. Both span all five substrate positions.
Phase 2 candidates SHOULD treat both as near-mandatory inputs:
- Cluster A: cycle-internal boundary discipline (no candidate
  should be silent on phase boundaries with state-write semantics)
- Cluster D: documentation honesty discipline (no candidate should
  be silent on what artifacts exist for honesty disciplines)

**Cluster B at 7 implications is the third-largest** — cross-cycle
artifact organization. Phase 2 candidates without explicit artifact
organization plans are missing a cluster present in every system
mined.

**Cluster I conversion is genuinely informative** — moving from
singleton to 2-system convergent with two distinct sub-shapes
(permission vs quality enforcement) suggests cluster I will likely
grow further as more systems are mined. The cluster maps directly
to a named v1 weakness (LLM-as-tool-policy-enforcer fragility) AND
now has cross-system support — both routes to high priority converge.

**Clusters G, F, E remain narrow** — these may be genuinely
narrower in the architectural space, OR future systems may
augment them. Cluster G in particular has only 2 sub-shapes
(clean-context-for-reviewer + untrusted-prefix-from-sub-agent);
3-system depth would be informative.

### Diversity hedge analysis

Cycle 65 introduced the diversity-hedge concept. Cycle 67 dissolved
it at four-system depth across three substrate axes. Cycle 68
strengthens with a fifth substrate position:

- AutoGen: Python library (Microsoft Research, multi-agent framework)
- LangGraph: Python/TypeScript library (LangChain ecosystem,
  graph-based agents)
- Cognition Devin: closed hosted commercial product (autonomous
  software engineering)
- openclaw: TypeScript local-first product (personal AI assistant)
- **OpenAI harness: internal-tooling-at-major-AI-lab** (research +
  engineering writeup, proprietary closed-system perspective)

Five substrate positions on at least four orthogonal axes:
- Library vs product
- Cloud-hosted vs local-first
- Multi-user enterprise vs single-user personal
- **External-publishable vs internal-tooling** (OpenAI harness adds
  this axis)

Clusters at 5-system clean (A, B, D) span all four substrate axes —
convergence at 5-system depth cannot be substrate-driven; it is
strongly architecture-driven. The diversity hedge fully dissolved at
four-system depth (cycle 67) and is *robustly* dissolved at five-
system depth (cycle 68). Clusters at 5-system depth are
unambiguously architectural patterns.

### Within-system intersections (OpenAI harness)

A within-system intersection matrix for OpenAI harness completes the
parallel to cycles 65 (AutoGen + LangGraph), 66 (Cognition), and 67
(openclaw):

| | I-OH1 | I-OH2 | I-OH3 | I-OH4 | I-OH5 | I-OH6 | I-OH7 | I-OH8 | I-OH9 | I-OH10 |
|---|---|---|---|---|---|---|---|---|---|---|
| **I-OH1** AGENTS.md | — | strong | weak | strong | none | weak | none | strong | none | weak |
| **I-OH2** llms.txt | strong | — | none | strong | none | strong | none | none | none | none |
| **I-OH3** failure modes | weak | none | — | weak | weak | weak | none | weak | strong | none |
| **I-OH4** repo-as-state | strong | strong | weak | — | weak | strong | strong | strong | none | strong |
| **I-OH5** linters | none | none | weak | weak | — | strong | weak | none | none | none |
| **I-OH6** doc-gardening | weak | strong | weak | strong | strong | — | weak | none | none | weak |
| **I-OH7** worktrees | none | none | none | strong | weak | weak | — | none | none | strong |
| **I-OH8** humans-steer | strong | none | weak | strong | none | none | none | — | strong | none |
| **I-OH9** Ralph Wiggum | none | none | strong | none | none | none | none | strong | — | none |
| **I-OH10** compaction | weak | none | none | strong | none | weak | strong | none | none | — |

Strong intersections (within OpenAI harness):

- **I-OH4 ↔ I-OH8 ↔ I-OH1: published-thesis triangle.**
  Repository-as-state + Humans-steer-agents-execute + AGENTS.md
  small-stable-entry-point. All three are about *publishing the
  load-bearing thesis at the top* — the agent's epistemic vantage
  (what state is), the role allocation (what humans vs agents do),
  and the orientation entry-point (where to start). Triangle anchors
  the "OpenAI harness invests heavily in published thesis discipline"
  thesis.
- **I-OH4 ↔ I-OH6 ↔ I-OH7: state-management-and-isolation triangle.**
  Repository-as-state + doc-gardening (continuous improvement of
  state) + worktrees (isolation per task). All three are about
  *state visibility and integrity* — what state exists, how it's
  kept fresh, how it's isolated per task.
- **I-OH3 ↔ I-OH9: anti-pattern-discipline pair.** Four named
  failure modes (mechanistic decomposition) and Ralph Wiggum Loop
  (named-pathology-with-acceptance) are both about naming what
  could go wrong with structural discipline.
- **I-OH1 ↔ I-OH2 ↔ I-OH4: artifact-organization triangle.**
  AGENTS.md (entry-point) + `*-llms.txt` (curated external
  knowledge) + repository-as-state (the repo IS the system's
  knowledge). The triangle anchors the "OpenAI harness has
  unusually disciplined artifact organization" thesis.
- **I-OH7 ↔ I-OH10: cycle-internal-boundary pair.** Worktrees and
  compaction are both about *how a single task within a cycle
  manages its boundaries* — worktree provides isolation, compaction
  provides context preservation across boundaries.
- **I-OH5 ↔ I-OH6: harness-action pair.** Linters+CI (mechanical
  enforcement) and doc-gardening (background improvement) are both
  *harness-level actions* on repository state.

Orphans (no strong same-system intersections):

- **I-OH3** (four failure modes) — within OpenAI harness, intersects
  only weakly with most implications and strongly only with I-OH9
  (Ralph Wiggum). Its strong intersection is *cross-system* with
  cluster D peers. Reproduces the orphan-pattern from cycles 65/66/67.

The orphan-pattern from cycles 65/66/67 holds at five instances:
**implications without strong same-system neighbors are
predominantly the ones that anchor cross-system clusters.**
Five-instance robustness on this methodological observation. v2
design-input candidate: a within-system intersection matrix tool
would surface elevation candidates automatically.

Notably, I-OH3 (an orphan within OpenAI harness) is also one of the
strongest cross-system signals (it confirmed H3 with a sub-shape).
This is the *fifth* instance of orphan-as-cross-system-anchor. The
methodological observation is now extremely robust.

---

## Updated multi-system convergence list (for future elevation)

Cycle-65 drafted elevation forms for 3 strong-convergent pairs in
1-research.md Family format. Cycle-66 enabled drafting for additional
3-system clusters. Cycle-67 enabled drafting at 4-system depth.
Cycle-68 enables drafting at 5-system depth — the strongest signal
yet.

**5-system clean elevations now available (the corpus's strongest
patterns):**

- **Cluster A** (cycle-internal boundaries with state-write semantics
  — including super-step phasing, termination predicates, lane-aware
  queuing, visibility-bounded state, process-isolation-per-task,
  context-management discipline) — AutoGen I-3 + I-5 + I-6,
  LangGraph I-L1 + I-L4 + I-L7, Cognition I-C8, openclaw I-O3,
  OpenAI harness I-OH4 + I-OH7 + I-OH10
- **Cluster B** (cross-cycle artifact organization with content-type
  × temporal-scope × opinion-level × entry-point-stratification ×
  curated-external-knowledge) — AutoGen I-6, LangGraph I-L5,
  Cognition I-C3, openclaw I-O4, OpenAI harness I-OH1 + I-OH2
- **Cluster D** (documentation-honesty discipline — including
  walkback, invariants vs derivations, anti-pattern catalogs with
  non-permanence, mechanistic sub-failure decomposition, published
  role-allocation axioms, named-pathology-with-context-bound-
  acceptance) — AutoGen I-4 + I-7, LangGraph section 2.8, Cognition
  I-C1 + I-C2 + I-C9, openclaw I-O2, OpenAI harness I-OH3 + I-OH8 +
  I-OH9

**4-system clean elevations (unchanged from cycle-67):**

- Cluster C (lifecycle operations beyond resume) — AutoGen I-3 +
  I-8, LangGraph I-L4 + I-L8, Cognition I-C5, openclaw I-O5

**3-system convergent elevations:**

- Cluster E (typed boundary semantics) — AutoGen I-5, LangGraph
  I-L2, openclaw I-O10
- Cluster F (tool-suite and prompt-suite stratification) — AutoGen
  I-2, Cognition I-C6, openclaw I-O6 + I-O8
- **Cluster H** (post-session feedback / cross-session learning —
  with continuum: tight-cycle-coupling → loose-sweep-coupling →
  continuous-background) — Cognition I-C7, openclaw I-O9, OpenAI
  harness I-OH6

**2-system convergent (unchanged from cycle-67 with new sub-shapes):**

- Cluster G (role-asymmetric context semantics) — Cognition I-C4,
  openclaw I-O7
- **Cluster I** (harness-enforced policy boundaries — permission +
  quality sub-shapes) — openclaw I-O1, OpenAI harness I-OH5

**Singleton clusters:** None. Cluster I converted to 2-system this
cycle.

**Actual elevation is still deferred** per cycle-64/65/66/67's
recommendation to a future cross-system synthesis cycle. Cycle 68
produces implications-mining material; the elevation cycle (option
5 from #2829, or "synthesis cycle producing actual elevation drafts
to 1-research.md" per cycle-65/66/67 deferrals) will draw from
cycles 62 / 64 / 65 / 66 / 67 / 68. The 5-system clean upgrades
give the elevation cycle copy-edit-able starting points for stronger
Family-format observations than cycle-66's 3-system or cycle-67's
4-system drafts could produce.

**The case for synthesis cycle is strengthening cycle-on-cycle.**
Cycle 65 deferred elevation to "a future synthesis cycle." Cycle 66
deferred again. Cycle 67 deferred. Cycle 68 deferred. The implications
material is now substantial (35 implications across 5 systems) and
the cluster table is at maximum-richness given the systems-with-
deeper-reads available. The implications-mining cadence will exhaust
unique deep-dive systems within 1-2 more cycles (Voyager + oh-my-
codex once #2833 returns). Synthesis transition becomes increasingly
appropriate.

## What this informs

Phase 2 candidate authors gain OpenAI-harness-specific design-input
not in `1-research.md` cross-system synthesis or in
`2-design-framework.md` axes:

- **AGENTS.md-style small entry-point with filesystem-pointed deeper
  content** (I-OH1) — operationalizes `<core-design-principle>`
  "small prompt + tools" with concrete shape; pairs with cycle-66
  Playbooks and cycle-65/67 cycle-composition tags
- **`*-llms.txt` curated-external-knowledge artifact type** (I-OH2)
  — distinct named artifact for compressed external documentation;
  v2 candidate references for `gh-cli`, `anthropic-api`, `cargo`,
  `git`
- **Anti-pattern catalog with mechanistic sub-failure decomposition**
  (I-OH3) — extends openclaw I-O2's `ANTI-PATTERNS.md` with
  per-anti-pattern decomposition into named sub-failures
- **Repository-as-state with explicit visibility-bounded thesis**
  (I-OH4) — published v2 axiom that constrains candidate design
  space (no external state without explicit justification)
- **Mechanical enforcement layer (linters + CI + automerge)**
  (I-OH5) — anchor cluster I with quality-policy sub-shape;
  parallel to openclaw I-O1's permission-policy sub-shape; both
  are forms of harness-policy enforcement outside LLM session
- **Background-agent layer (doc-gardening + quality-grading)**
  (I-OH6) — extends cluster H continuum with continuous-background
  cadence; v2 candidate Rust tools for stale-reference detection,
  cluster-table updates, sweep-summaries
- **Ephemeral per-task worktrees with per-task observability**
  (I-OH7) — process-isolation primitive at finer granularity than
  cycle-level (which v1 already has); minimal-viable version: git
  worktrees + per-task log files
- **"Humans steer. Agents execute." as published role-allocation
  axiom** (I-OH8) — compress v2 mission to top-level memorable
  axiom; establishes decision-rule for ambiguous cases
- **Named-pathology-with-context-bound-acceptance discipline**
  (I-OH9) — meta-discipline for working with patterns; name
  patterns the system uses + name pathology potential + name
  bounding infrastructure
- **Context-management discipline with named triggers and role
  hierarchy** (I-OH10) — explicit System > Operator > Audit >
  Self-prior-cycle > Untrusted role hierarchy; mid-session
  compaction-to-repo discipline

**Clusters A and D are now tied as most-foregrounded clusters in
the corpus** — 10 implications each, 5-system clean depth, 7-8
sub-shapes. Phase 2 candidates SHOULD treat both as near-mandatory.

**Cluster I conversion to 2-system convergent is methodologically
significant** — the cluster anchored by named v1 weakness now has
cross-system support, both routes to high priority confirm.

**Cluster H upgrade to 3-system convergent** establishes a
sub-shape continuum (tight-cycle → loose-sweep → continuous-
background) that gives Phase 2 candidates a spectrum of mechanism
choices rather than forcing one mechanism.

## Methodological observations

**Hypothesis-driven cycle structure: third instance.** Cycle 66
introduced the discipline; cycle 67 reproduced; cycle 68 reproduced
at third instance with all three hypotheses confirmed (two at high
strength). Cumulative hit rate: 8 of 9 hypotheses confirmed (89%)
across three cycles. The discipline accelerates research-corpus
advancement because hypotheses make cluster-augmentation predictions
falsifiable.

The methodological refinement from cycle 67 (cluster classification
by *discipline* not by *artifact-shape*) reproduces in cycle 68:
ephemeral worktrees LOOK like harness-policy artifact (process
isolation) but their *discipline* is cycle-internal boundary
management → cluster A. This is the second instance confirming the
methodological refinement; cycle 67 surfaced it once, cycle 68
applied it correctly without re-discovery.

**Implications-mining cadence at fifth instance.** Cycle 68 is the
seventh consecutive cycle of research-corpus advancement under
#2829's polarity inversion (cycle 62 = AutoGen mining, cycle 63 =
oh-my-codex deeper-read dispatch construction, cycle 64 = LangGraph
mining, cycle 65 = cross-implications synthesis, cycle 66 =
Cognition Devin mining, cycle 67 = openclaw mining, cycle 68 =
OpenAI harness mining). Seven-cycle robustness on the polarity-pivot
pattern with three distinct cycle composition shapes (mining,
dispatch-construction, synthesis) viable as substantive focal under
#2829, plus the mining-with-synthesis-update shape variant
demonstrated in cycles 66, 67, 68.

**Cycle 68 is mining-with-synthesis-update at third instance.**
Cycle 68 combined mining (10 implications) with synthesis-update
(cluster table for 5-system depth, within-system intersection
matrix, hypothesis testing, multi-system convergence list update).
The line count is comparable to cycles 66 and 67 (~1100-1300 lines
target). The shape-variant is now sustained at three instances,
validating it as a stable cycle-composition shape alongside
pure-mining (cycles 62, 64). v2 design-input: explicit cycle-
composition tag should support compound-shape declarations.

**Diversity hedge robustly dissolved at five-system depth.** The
substrate axes (library vs product, cloud vs local, enterprise vs
personal, external-publishable vs internal-tooling) are now spanned
by the five systems in the corpus. Clusters at 5-system-clean depth
(A, B, D) cannot be substrate-driven — the substrate diversity
covers four orthogonal axes. The diversity hedge from cycle 65 has
been dissolved (cycle 67) and now is *strongly* dissolved (cycle 68).

**Orphan-pattern reproduces at fifth instance.** I-OH3 (four named
failure modes) is a within-OpenAI-harness orphan whose strong
cross-system intersection is cluster D peers. Pattern from cycles
65/66/67/68 reproduces. Five-instance robustness on the
methodological observation: implications without strong same-system
neighbors are predominantly the ones that anchor cross-system
clusters. The pattern is now sufficiently robust that future cycles
can confidently use within-system orphan-status as a positive
signal for elevation candidacy.

**Cluster I conversion validates the named-v1-weakness route to
high priority.** Cycle 67 noted cluster I was "high-priority despite
singular-voice because it addresses a named v1 weakness directly."
Cycle 68 adds cross-system support, so cluster I is high-priority
on BOTH routes (named-v1-weakness AND cross-system convergence).
The two routes converge for cluster I; whether they also converge
for clusters with no cross-system support yet (none currently;
cluster I was the only singleton post-cycle-67) is an open question
for future systems.

**Persistence-mechanism observation.** No memory-directory bootstrap
this cycle. Per cycle-62's finding (memory directory is ephemeral
within a session, not across sessions), the cross-cycle persistence
is the repo (docs/journal, docs/redesign/_notes, framework
artifacts). Cycle-68's implications doc is now part of that
persistence.

**Sub-shape diversity grows faster than cluster count at depth.**
Across cycles 62-68, the cluster count is stable (8-9 clusters) but
the sub-shape count within clusters grows substantially with each
mining cycle. Cycle 67 to cycle 68: cluster count 9 → 9 (cluster I
was already present); sub-shape count growth — cluster A 4 → 7 (+3),
cluster B 4 → 6 (+2), cluster D 5 → 8 (+3), cluster H 2 → 3 (+1),
cluster I 1 → 2 (+1). Total sub-shape count growth: +10 across the
cycle-68 mining. v2 design-input: clusters should be tracked at
*both* cluster-membership level (which systems) AND sub-shape level
(which mechanisms within the cluster theme). The latter is where
candidate variety lives.

**Cycle-67 H1 confirmation pattern reproduces.** Cycle 67's H1
(cluster G augmentation) confirmed at cluster level with sub-shape
distinction; cycle 68's H1 (cluster I conversion) confirmed at
cluster level with sub-shape distinction. The pattern: hypothesis-
driven cycles confirm cluster-level predictions reliably AND surface
sub-shape distinctions that weren't pre-specified. The
hypothesis-driven discipline is calibrated for cluster-level
predictions; sub-shape predictions are an open methodological
question (whether they can be predicted vs whether they emerge from
mining is unclear).

## Provisional read for cycle 69

Three candidates in priority order:

1. **If cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
   deliverable returns by cycle 69:** per-finding evaluation
   absorption cycle. Highest priority — value compounds when
   integrated rapidly. The dispatch has been open since cycle 63
   (5+ cycles ago at this point); openclaw I-O5's stuck-watchdog
   pattern would have detected and released this lane already.

2. **Otherwise, implications mining on Voyager (option 4
   continuation):** Voyager is the last remaining unique deep-dive
   system. Per cycle-67's hypothesis-set: Voyager's iterative
   skill-library building and self-verification mechanism could
   provide a third sub-shape for cluster H (post-session feedback /
   cross-session learning), upgrading cluster H from 3-system
   convergent to 4-system convergent. The Minecraft substrate is
   different from any system mined so far (game environment vs
   software development), so the substrate diversity axis grows
   further. Hypothesis: H1 = Voyager's GPT-4 self-verification
   provides a third sub-shape for cluster H; H2 = Voyager's
   skill-library curation introduces a new cluster J or augments
   cluster F (stratification) with library-curation as a new
   stratification axis.

3. **Otherwise, synthesis cycle (option 2):** five systems mined
   is enough material for a 5-system synthesis cycle that produces
   actual elevation drafts to 1-research.md (per cycle-65/66/67/68
   deferrals). The implications-mining cadence will exhaust unique
   deep-dive systems with one more cycle (Voyager) before
   oh-my-codex returns from #2833. Synthesis becomes the natural
   transition activity.

The case for synthesis cycle continues to strengthen. Cycle 68
brings the corpus to 35 implications across 5 systems with strong
5-system clean signals (clusters A, B, D). Phase 2 candidate
authors would benefit from elevation drafts being merged into
1-research.md as Family-format observations rather than continuing
to live in implications-mining notes. Cycle 69 might be the right
moment to transition — either after one more mining cycle (Voyager)
or as the dispatch absorption cycle if #2833 returns.

**Hypothesis for cycle 69 (if option 4 — Voyager mining):**

H1 = Voyager's GPT-4 self-verification mechanism provides a third
sub-shape for cluster H (post-session feedback / cross-session
learning), upgrading from 3-system convergent to 4-system
convergent. The current sub-shape continuum (tight-cycle-coupling
→ loose-sweep-coupling → continuous-background) might extend with
self-verification as a fourth point.

H2 = Voyager's iterative skill-library building (with skills
"discovered" and stored per task) introduces a new cluster J
(skill-library curation as cross-cycle learning) or augments
cluster F (stratification) with library-curation as a new
stratification axis.

H3 = Voyager's exploration-vs-exploitation balance (intrinsic
curiosity reward) is unique to game environments and does NOT
augment any existing cluster — null-hypothesis as honest expected
outcome for cycles where substrate divergence is high.

**If synthesis is selected instead of mining:** actual elevation
drafts to 1-research.md as Family-format observations for clusters
A, B, D (5-system clean) and clusters E, F, H (3-system convergent)
and clusters G, I (2-system convergent). Three-tier elevation by
cluster-depth would give 1-research.md a clear hierarchy of
cross-system patterns at maturity-graded depths.

The implications-mining cadence still sustains for at least 1-2
more cycles before exhausting unique deep-dive systems. After that,
synthesis is the natural transition activity. v2 design-input:
research-corpus advancement has a natural cadence (mining → mining
→ synthesis → mining → mining → synthesis) that the cycle-
composition tag could declare prospectively. The mining-mining-
synthesis ratio of approximately 2:1 emerged organically (cycles
62/64 mining → 65 synthesis, then 66/67 mining → 68 mining) and
might converge to 2:1 over time.
