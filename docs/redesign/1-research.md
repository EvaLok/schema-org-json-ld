# Phase 1: External-system research

**Status**: initial survey — systems read at architecture level, not yet at
implementation detail. Authorized to proceed in parallel with Phase 0 per
[#2759](https://github.com/EvaLok/schema-org-json-ld/issues/2759)
(input-from-eva, 2026-04-28).

**Reading guide**: this document catalogs architectural patterns from external
AI agent/orchestrator systems that are relevant to the v2 redesign. It is
organized by cross-cutting concern (not by system) because the v2 design will
draw from patterns across systems, not adopt any single system wholesale.
Per-system summaries appear in the appendix for reference.

**Anchoring discipline**: per the Phase 1 authorization, the purpose of this
research is to identify patterns that could inform Phase 2 candidates — not to
converge on a single design direction. Where a pattern could anchor
candidate-space prematurely, this document names the tension rather than
committing to the pattern.

---

## Systems surveyed

| System | Source | Coverage |
|---|---|---|
| PAI (Personal AI Infrastructure) | danielmiessler/Personal_AI_Infrastructure v4.0.3 | Detailed — repo accessible, rich architectural patterns |
| LangGraph | LangChain team | Architecture-level — graph-based state machine, checkpointing, tool orchestration |
| AutoGen | Microsoft | Architecture-level — multi-agent conversation patterns, error recovery |
| Semantic Kernel | Microsoft | Architecture-level — plugin/function catalog, planner patterns, stateless kernel |
| Voyager | NVIDIA (2023 paper) | Architecture-level — skill library, self-improvement loop, verification gates |
| Devin | Cognition | Architecture-level — long-running tasks, scratchpad planning, failure backtracking |
| CrewAI | — | Brief — role-based agent specialization |
| openclaw | openclaw/openclaw | **Not accessible** — repo appears private or non-existent. Retry needed with network access or alternative source. |

---

## Cross-cutting patterns

### 1. Cold-start persistence

The problem: each session starts with no working memory. How do external
systems solve this?

**PAI's PRD-as-session-contract.** Every session writes a PRD.md into
`MEMORY/WORK/{slug}/` with YAML frontmatter encoding phase, progress, and
status. On cold start, a hook reads the most recent PRD by mtime and injects a
summary as a system reminder. Recovery from any cold start is "read the most
recent PRD." The frontmatter carries all state needed to resume.

**LangGraph's per-node checkpointing.** State is serialized after every node
execution. A `thread_id` + `checkpoint_id` resumes from any prior state.
Cold-start resumption is first-class.

**Voyager's disk-backed skill library.** Skills are written to a vector store
on disk after verification. On restart, the vector store reloads and the agent
resumes. Cold-start cost is embedding lookup, not re-learning.

**Devin's scratchpad.** A rolling plan document tracks completed steps, pending
steps, and key facts. Context is managed by summarizing what was done rather
than keeping raw artifacts resident.

**Pattern synthesis.** Two approaches: (a) structured document with typed
frontmatter (PAI) — human-readable, easy to inspect, but requires discipline
to keep current; (b) machine-serialized state (LangGraph) — automatic, but
opaque to human review. The v1 orchestrator's `state.json` is closer to (b)
but suffers from defense-accretion because fields accumulate without pruning.
PAI's approach of separating durable state (PRD) from ephemeral cache (STATE/)
from historical signals (LEARNING/) is a three-tier model worth considering.

> **Anchoring tension**: PAI's PRD model is appealing but could anchor v2
> toward a "single structured document" pattern that replicates v1's
> state.json problems at a different layer. The pattern to extract is the
> *tiered separation* (durable/ephemeral/historical), not necessarily the
> specific file format.

### 2. Defense-accretion prevention

The problem: v1 accumulates constraints, pipeline-checks, and state fields
without removal-tests. How do external systems prevent unbounded growth?

**Voyager's strict admission gate.** Only verified, novel skills enter the
library. Code that fails the automated verifier is discarded. Deduplication by
semantic similarity at insertion prevents near-duplicates. The library stays
compact because the admission bar is high.

**PAI's "surgical fixes only" rule.** AISTEERINGRULES explicitly state: "never
add components as a fix." First principles over bolt-ons. Minimal scope.

**LangGraph's compaction node pattern.** No built-in pruning, but the
recommended architecture is a `compaction_node` that runs before the judgment
node on every cycle — reads accumulated state, applies
summarize/deduplicate/prune, writes compacted version back. The checkpointer
then stores the compacted state.

**Pattern synthesis.** Two complementary mechanisms: (a) strict admission
(Voyager) — a new constraint/defense earns its place only when observed to
prevent a real failure, not pre-emptively; (b) periodic compaction
(LangGraph) — even with strict admission, accumulated items need periodic
review-and-prune. v1 has neither: constraints are added pre-emptively (F1),
and no mechanism removes them even when they demonstrably fail (recurrence-
escalation citations prove this).

> **Anchoring tension**: Voyager's gate is clean but Voyager operates in a
> domain (Minecraft) where skill verification is objective (did the agent
> accomplish the task?). Orchestrator constraint verification is harder —
> "did this constraint prevent the failure it was added for?" requires
> counterfactual reasoning. The v2 admission gate may need to be
> evidence-based (show N cycles where the constraint fired and helped)
> rather than verify-on-insert.

### 3. Separating tool-work from judgment-work

The problem: v1's prompt embeds procedural steps the orchestrator follows
every cycle, consuming compute that should go to judgment and improvement.

**LangGraph's ToolNode pattern.** Tools run only when the LLM explicitly
requests them via tool calls. The LLM decides *whether* to invoke tools;
the runtime handles *how*. Tools do not self-schedule.

**Semantic Kernel's catalog model.** Tools (KernelFunctions) are registered
in a catalog with descriptions and schemas. The planner discovers and
selects tools by consulting the catalog, not by following a hardcoded
sequence. The kernel is stateless — it composes tool calls per session
without ambient procedural state.

**AutoGen's agent-type separation.** Tool agents vs. LLM agents are
structurally distinct. Tool agents execute; LLM agents reason. Error
text from tool failures becomes a message the LLM agent can reason about
and re-plan from — tool failure does not crash the conversation.

**PAI's capability-selection-with-binding-commitment.** During the OBSERVE
phase, the AI selects capabilities from a skill library. Selection creates
an "invocation obligation" — selected capabilities must be called. But
the AI chooses *which* capabilities, not a fixed procedure.

**Devin's thought-act-observe loop.** The model narrates its reasoning
before each action. Judgment (what to do next) is separated from execution
(issuing the tool call) at the interface level.

**Pattern synthesis.** Universal pattern: the AI decides *what* to do; tools
handle *how*. No surveyed system embeds a fixed procedural sequence in the
prompt. The closest is LangGraph's graph definition (which fixes the node
topology) but even there, conditional edges let the AI choose which path
to take. v1's prompt prescribes a specific checklist sequence — this is the
anti-pattern every surveyed system avoids.

> **Design implication (strong signal, low anchoring risk)**: v2's prompt
> should not contain step-by-step procedures. Procedures belong in tools
> that the prompt teaches the orchestrator to invoke when appropriate.
> This is consistent with the redesign prompt's own CORE-DESIGN-PRINCIPLE
> and is independently validated by every surveyed system.

### 4. Self-improvement loops

The problem: how does a system learn from its own failures and improve
over time?

**PAI's closed feedback loop.** execution → LEARN phase (structured
self-critique appended to reflections JSONL) → rating capture (user
signals) → failure analysis (low-rating dumps) → pattern synthesis
(weekly aggregation) → algorithm revision. The loop is closed: outputs
feed back into the system that produced them.

**Voyager's curriculum-verify-store cycle.** propose task → attempt →
verify (automated, objective) → store if verified → propose next. The
verifier is the critical component — without objective verification,
the loop drifts.

**Andrew Ng's reflection pattern.** The agent critiques its own output
before returning it. Lightweight single-session version of the same idea.

**Pattern synthesis.** Two requirements for a self-improvement loop that
works: (a) objective verification — subjective LLM scoring drifts; use
test results, observable state, measurable outcomes; (b) structured
storage — reflections must be retrievable by future sessions, not just
appended to an ever-growing log.

> **Anchoring tension**: PAI's LEARN phase is rich but couples
> self-improvement to user feedback (ratings). v2 has no real-time user —
> Eva reviews asynchronously. The self-improvement loop must work without
> synchronous human signal. Voyager's automated verification is a better
> fit for the cron model, but orchestrator "success" is harder to verify
> objectively than Minecraft task completion.

### 5. State representation

The problem: what shape should state take?

**LangGraph's flat typed dict.** State is a single typed dictionary.
Custom reducers control how updates merge (last-write-wins, append, etc.).
Flat by design — no nesting, no message buses.

**PAI's three tiers.** Durable (PRD.md with YAML frontmatter), ephemeral
(STATE/ directory with JSON), historical (LEARNING/ directory with JSONL).
Each tier has different lifecycle expectations and different
read/write patterns.

**Semantic Kernel's stateless kernel.** The kernel itself has no state.
Memory is a plugin. This means state accumulation is opt-in, not
ambient — you carry only what you explicitly load.

**Pattern synthesis.** Two design axes: (a) flat vs. tiered — flat is
simpler but everything shares a lifecycle; tiered separates concerns but
adds complexity; (b) ambient vs. opt-in — ambient state (v1's state.json,
LangGraph's dict) grows unless pruned; opt-in state (SK's memory-as-
plugin) requires explicit loading but prevents bloat by default.

v1's state.json is flat and ambient — the worst combination for
defense-accretion. The survey suggests either tiered-ambient (PAI) or
flat-opt-in (SK) as improvements.

### 6. Review and feedback mechanisms

The problem: how do systems ensure quality without consuming all compute
on review overhead?

**PAI's multi-layer review.** THINK phase (premortem before execution),
VERIFY phase (check against criteria after execution), LEARN phase
(self-critique for future sessions). Three distinct review points with
different purposes.

**PAI's ISC (Ideal State Criteria).** Atomic, binary-testable criteria
defined before work begins. The "Splitting Test" prevents compound
criteria: if a criterion contains "and" or crosses domain boundaries,
decompose it. Minimum count gates per effort tier.

**Voyager's automated verification.** The critic LLM confirms skill
correctness or patches and retries. Only verified skills enter the
library.

**CrewAI's role separation.** Reviewer agents have different system
prompts than executor agents. Writer/reviewer separation prevents
self-review bias.

**Pattern synthesis.** Two principles: (a) review criteria should be
defined before execution, not discovered after (PAI's ISC, Voyager's
verification); (b) review should be structurally separated from execution
(CrewAI's role separation, PAI's phase separation). v1's review mechanism
(the review agent) is structurally separated but its output (chronic-
category tracking, finding disposition) generates procedural overhead
that consumes subsequent cycles (F9). The issue is not review itself
but the overhead of *maintaining review state across cycles*.

> **Anchoring tension**: PAI's ISC is a strong pattern but assumes work
> has clear pre-definable success criteria. Orchestrator work includes
> novel judgment calls where criteria emerge during execution. The ISC
> pattern may apply to the *tool* layer (deterministic, criteria-clear)
> but not to the *judgment* layer (criteria-evolving).

### 7. Cycle boundary handling

The problem: v1's cycle boundary creates reconciliation asymmetry —
artifacts freeze before post-close defenses finish.

**LangGraph's interrupt-and-resume.** The graph halts at a specified
node, serializes state, waits for external input. State is consistent at
the interrupt point because checkpointing happened at every prior node.

**PAI's session hooks.** SessionStart loads context; SessionEnd runs
integrity checks. Hooks are asynchronous and fail gracefully (errors
logged, never blocking).

**Devin's rolling plan.** No hard cycle boundaries. The plan is
continuously updated. Completed steps are marked done; new steps are
added. "Session end" is just saving the current plan state.

**Pattern synthesis.** The surveyed systems mostly avoid hard cycle
boundaries. LangGraph's interrupt is the closest analog, but it creates
a clean serialization point rather than a freeze-then-reconcile pattern.
Devin's approach (no hard boundary, rolling plan) is the most radical
departure from v1.

> **Anchoring tension**: "eliminate hard cycle boundaries" is an
> appealing direction but may not be feasible in a cron-triggered system
> where each session is a fresh cold start. The actionable pattern may be:
> make the cycle boundary a clean serialization point (LangGraph-style)
> rather than an artifact-freeze point. Defenses run *before* the boundary,
> not after; or defenses are tools that run *at the start* of the next
> cycle rather than at the end of the current one.

---

## Patterns NOT observed (notable absences)

- **No surveyed system has a constraint-accumulation mechanism like v1's
  recurrence-escalation.** Every system either gates admission strictly
  (Voyager) or keeps rules minimal by design (PAI, SK). v1's pattern of
  adding constraints when failures recur and then adding *more* constraints
  when the first constraints fail is unique to v1 and appears pathological
  by comparison.

- **No surveyed system embeds a multi-step procedural checklist in the AI's
  system prompt.** LangGraph defines procedure as graph topology; SK
  defines it as planner discovery; AutoGen defines it as conversation
  routing. Procedure lives in the framework, not in the prompt.

- **No surveyed system has chronic-category tracking as a first-class
  concern.** Review categories are either dispositioned (PAI's VERIFY
  pass/fail) or discarded (Voyager's verifier reject). Carrying review
  categories forward across sessions as a maintenance obligation is absent
  from all surveyed systems.

---

## Open questions for deeper reading

1. **openclaw** — not accessible in this survey. Needs network access or
   alternative source. Required per Phase 1 definition.

2. **Reflexion (Shinn et al., 2023)** — cited as relevant to
   defense-accretion: models how accumulated verbal feedback degrades
   performance over long horizons. Worth reading for empirical evidence
   on when self-critique helps vs. hurts.

3. **TaskWeaver** — cited for serialized plan/verify loops relevant to
   reconciliation asymmetry. Not yet surveyed.

4. **Temporal.io** — workflow engine with retry policies and durable
   execution. Relevant to failure recovery patterns. Not yet surveyed.

5. **PAI implementation detail** — the survey covered architecture; the
   hook implementations, JSONL schema, and ISC splitting mechanics
   deserve deeper reading for v2 tool-design inspiration.

6. **LangGraph implementation detail** — custom reducer patterns and the
   `compaction_node` idiom deserve implementation-level reading for v2
   state-management design.

---

## Appendix: per-system summaries

### PAI (Personal AI Infrastructure)

Daniel Miessler's system (v4.0.3, TypeScript/Bun). File-system-as-database
with three tiers: MEMORY/WORK/ (PRD files), MEMORY/STATE/ (ephemeral cache),
MEMORY/LEARNING/ (reflections, signals, synthesis). Seven-phase algorithm
(OBSERVE → THINK → PLAN → EXECUTE → VERIFY → LEARN → OUTPUT). ISC (Ideal
State Criteria) system with atomic binary-testable checkboxes and splitting
test. Capability selection with binding commitment. Closed feedback loop via
LEARN phase + rating capture + failure analysis + pattern synthesis + algorithm
revision. Effort tier calibration (Standard/Comprehensive). Hooks for
SessionStart/SessionEnd. Agent types: researcher, architect, engineer, QA.
Parallel work via competing-hypotheses debugging and worktree isolation.

### LangGraph

LangChain's graph-based agent framework. State as typed dict with custom
reducers. Directed graph with conditional edges. Per-node checkpointing to
SQLite/PostgreSQL. Cold-start resumption via thread_id + checkpoint_id.
ToolNode for structured tool invocation (LLM decides whether to call; runtime
handles execution). No built-in retry, pruning, or context-window management —
all are the designer's responsibility. Interrupt-and-resume for human-in-the-
loop. Cycles (loops) are first-class via back-edges; termination requires
explicit exit conditions.

### AutoGen

Microsoft's multi-agent framework. ConversableAgent nodes in conversation
graphs. GroupChatManager routes via round-robin, LLM-selected, or custom
policy. Stateless between sessions by default. Error recovery via conversation
loop (error text becomes a message the LLM reasons about). HumanProxyAgent
as structural oversight position. Functions registered as OpenAI-compatible
tool schemas. Agent coordination is message-passing, not shared state.

### Semantic Kernel

Microsoft's AI integration framework. Stateless kernel — memory is a plugin.
Plugin/function catalog with descriptions and schemas. Sequential and stepwise
planners. Termination strategies as evaluator functions for loop control.
State accumulation is opt-in, not ambient. Clean cold-start model.

### Voyager

NVIDIA's Minecraft agent (2023). Skill library as vector-indexed code snippets
on disk. Three-component self-improvement: curriculum agent (proposes tasks) +
skill manager (executes and verifies) + critic LLM (confirms or patches).
Strict admission: only verified, novel, deduplicated skills enter the library.
Compact by design. Cold-start cost is embedding lookup only.

### Devin

Cognition's AI software engineer. Rolling scratchpad plan (completed/pending
steps + key facts). Thought-act-observe loop separating judgment from
execution. Shell/browser/editor as distinct tool channels. Failure handling via
backtrack-to-last-good-checkpoint + re-plan (revise plan before retrying, not
blind retry). Context managed by summarization rather than raw artifact
retention.

### CrewAI

Role-based agent specialization. Agents have fixed roles with goal and
backstory in system prompt. Sequential or hierarchical process composition.
Inter-agent handoffs pass structured task outputs, not raw conversation
history. Role specificity replaces general-purpose instructions.
