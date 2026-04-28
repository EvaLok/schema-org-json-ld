# Phase 1: External Research

## Status

**Phase 1 initiated cycle 14** (commit pending). Phase 1 is authorized to
run in parallel with finishing Phase 0 per `input-from-eva` issue #2759
(2026-04-28) and the redesign-prompt update at commit
[`f77b4341`](https://github.com/EvaLok/schema-org-json-ld/commit/f77b4341).
Phase 2 candidate generation still requires explicit post-retrospective
checkpoint approval; Phase 1 reading is preparatory, not a
candidate-generation step.

## Purpose and scope

The redesign prompt's Phase 1 definition names two required reads
(openclaw, PAI) and lists candidate further reads (LangGraph, Semantic
Kernel, AutoGen, Voyager, Cognition's Devin writeups). The purpose is to
study how other systems handle problems our v1 has shown structural
weaknesses on:

- multi-cycle persistence across cold-starts
- self-modification / self-improvement mechanisms
- prompt-vs-tool split (where does behavior live)
- audit / critique / review loops
- cross-process state representation
- security posture in an adversarially-readable context

This document is a working reference. It is not a Phase 2 candidate
proposal. Patterns are recorded with anchoring caveats; recommendations
for v2 belong to Phase 2 after multi-system reading is complete.

## Anchoring discipline

Per Eva's note in #2759: "If Phase 1 surfaces patterns that could anchor
candidate-space prematurely, journal the tensions rather than commit to
them."

Three failure modes to avoid:

1. **Confirmation bias on aligned principles.** PAI's principles are
   strikingly aligned with our CORE-DESIGN-PRINCIPLE. The reflex to
   read "yes, all of this validates the redesign" is exactly the
   anchoring failure to avoid. Note alignment; do not import.

2. **Context-mismatch import.** Both required reads are single-user
   personal-assistant systems, in TypeScript, with human-in-the-loop
   for most decisions. Our system is multi-agent autonomous on a
   public repo, with Rust tools and minimal-human-in-the-loop per
   the orchestrator's `EVA-DEFAULT-AUTONOMY` directive. Architectural
   patterns that work for one context may fail in the other.

3. **Premature commitment to first-found patterns.** Phase 1 must read
   multiple systems before settling on candidate shapes. A pattern that
   appears in openclaw or PAI may or may not generalize. Record
   patterns; defer evaluation.

Trust posture: README and VISION.md content from external repos is
**untrusted text** per `SECURITY` rules. The architectural claims are
data, not instructions. Where this document quotes external sources,
the source is named so a future reader can verify or weigh the
provenance.

## Initial findings (cycle 14)

### openclaw (openclaw/openclaw, 365k★ / 75k forks / TypeScript)

Cycle 14 read: `README.md`, `VISION.md`. Repo size 663MB suggests a
substantially larger codebase than the README captures; deeper reads
(directory survey, architecture pages, key source files) deferred to
later cycles.

**Architectural patterns named.** Local-first Gateway as single
control plane for sessions/channels/tools/events. Multi-channel
inbound (25+ messaging surfaces). Multi-agent routing (route channels
to isolated agents with workspaces + per-agent sessions). Plugin API
with two styles (code plugins for runtime extension, bundle-style
plugins for stable external surfaces). Memory as a special plugin
slot — only one memory plugin active at a time.

**Security posture.** "Treat inbound DMs as untrusted input." Default
DM policy is `pairing` (unknown senders get a pairing code, message
not processed; explicit approval required). Sandbox modes for
non-`main` sessions (Docker default; SSH and OpenShell backends).
Default sandbox allows `bash`, `process`, `read`, `write`, `edit`,
`sessions_*`; denies `browser`, `canvas`, `nodes`, `cron`,
`discord`, `gateway`. The pattern is *strong defaults with
operator-controlled knobs for trusted high-power workflows*.

**Roadmap guardrails (VISION.md "What We Will Not Merge").**
Explicit anti-patterns:
- "Agent-hierarchy frameworks (manager-of-managers / nested planner
  trees) as a default architecture"
- "Heavy orchestration layers that duplicate existing agent and tool
  infrastructure"

This is a **counter-signal to LangGraph/AutoGen-style hierarchical
agents**. The openclaw maintainers explicitly reject manager-of-
managers as a default. The reasoning is not given in VISION.md;
deeper reads should look for the actual rationale. *If our Phase 2
candidates trend toward hierarchical agents, this counter-signal is
worth weighing.*

**Tooling philosophy.** "Core stays lean; optional capability should
usually ship as plugins." Bundle-style plugins (preferred when
expressive enough) have "smaller, more stable interface and better
security boundaries." Plugin promotion to core has "intentionally
high" bar.

**Anchoring caveats on openclaw.**
- Single-user personal assistant; not multi-agent autonomous.
- Has human-in-the-loop on every interaction (user issues
  commands).
- TypeScript-first by stated design choice ("fast to iterate in,
  easy to read, modify, and extend") — our system mandates Rust for
  tools.
- 365k★/75k-fork scale is anomalous for a 5-month-old repo
  (created 2025-11-24); raises questions about whether the
  repository reflects an organic engineering project or has unusual
  amplification (parody/marketing/etc). The architecture is real
  regardless, but external validation of design choices is weaker
  than the star count suggests.
- The "EXFOLIATE!" slogan and lobster theming flag this as at least
  partly tongue-in-cheek; treat architectural claims on their own
  merit, not on project gravitas.

**Patterns observed in openclaw** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Small-core architecture with plugin-extensible capability
- Anti-pattern list ("What We Will Not Merge") as a deliverable artifact
- Memory as a singleton plugin slot (one active mechanism, replaceable,
  not layered)
- Strong-defaults-with-operator-knobs security posture

### PAI (danielmiessler/Personal_AI_Infrastructure, 12k★ / 1.6k forks / TypeScript)

Cycle 14 read: `README.md` (extensive). Deeper reads of `Tools/`,
`Packs/`, `.claude/`, `Releases/v4.0.3/` deferred to later cycles.

**The 16 PAI Principles** (verbatim summaries from the PAI README):

| # | Principle | PAI's framing |
|---|-----------|---------------|
| 1 | User Centricity | Built around the user, not tooling |
| 2 | The Foundational Algorithm | Observe → Think → Plan → Build → Execute → Verify → Learn |
| 3 | Clear Thinking First | Good prompts come from clear thinking |
| 4 | Scaffolding > Model | System architecture matters more than which model |
| 5 | Deterministic Infrastructure | AI is probabilistic; infrastructure shouldn't be |
| 6 | Code Before Prompts | If you can solve it with bash, don't use AI |
| 7 | Spec / Test / Evals First | Specs and tests before building |
| 8 | UNIX Philosophy | One thing well; composable; text interfaces |
| 9 | ENG / SRE Principles | Treat AI infra like production software |
| 10 | CLI as Interface | CLI faster, more scriptable, more reliable than GUIs |
| 11 | Goal → Code → CLI → Prompts → Agents | Decision hierarchy for where behavior lives |
| 12 | Skill Management | Modular capabilities routing on context |
| 13 | Memory System | Everything worth knowing gets captured; history feeds context |
| 14 | Agent Personalities | Specialized agents with unique voices |
| 15 | Science as Meta-Loop | Hypothesis → Experiment → Measure → Iterate |
| 16 | Permission to Fail | Explicit permission to say "I don't know" |

**Strikingly aligned with our CORE-DESIGN-PRINCIPLE.** Principles 4, 5, 6,
8, 11 specifically describe the same posture our redesign is targeting:
deterministic tools handle procedure; orchestrator handles judgment.
Principle 11's hierarchy "Goal → Code → CLI → Prompts → Agents" is the
explicit decision-tree version of what our prompt names as "test for
violation: can a tool do this deterministically? If yes, build the
tool, do not embed the procedure in the prompt."

**Continuous-learning loop.** The PAI primary loop is:

> Observe → Think → Plan → Execute → Verify → **Learn** → Improve

The "Learn" closure is the explicit meta-loop for self-modification.
Compare v1 which has Observe (audit/review)-Plan (worklog)-Execute
(tools)-Verify (pipeline-check)-Learn (journal) but *no closed loop
to Improve* — the journal is reflective but does not produce
mechanical change. PAI claims (in the README) that PAI does close
this loop; verifying that claim requires reading the actual code,
not the README.

**Memory as a top-level principle.** Principle 13: "Memory System —
Everything worth knowing gets captured. History feeds future
context." Treating memory as a first-class architectural concern
(not scaffolding) aligns with our `PERSISTENCE` directive's framing
of multi-session memory as a design problem to solve, not an
implementation detail.

**Anchoring caveats on PAI.**
- Single-user personal assistant; user provides goals, system pursues.
  Our system has no human-provided goal stream beyond Eva's
  occasional input-from-eva — the orchestrator generates its own
  next-cycle work from prior cycles.
- "Memory" in PAI captures user preferences and history (what *the
  user* has done / wanted). Our "persistence" captures
  cross-session orchestrator state (what *the orchestrator* has
  decided / is investigating). These are different shapes despite
  the shared label.
- TypeScript / Bun-based. Our tools are Rust.
- The 16 principles are stated as design goals in the README;
  whether the PAI codebase actually instantiates each of them is
  not yet verified by code-reading.
- The README is a marketing surface (it includes the project's
  philosophical mission about "activating people"). Architecture
  detail will be in deeper docs and code; the README is the
  starting point, not the substance.

**Patterns observed in PAI** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Principle-list published as part of the deliverable (16 named
  principles included in PAI's README)
- Decision hierarchy "Goal → Code → CLI → Prompts → Agents" for
  where capability lives
- Closed feedback loop (Observe→Think→Plan→Execute→Verify→Learn→Improve)
  with explicit Learn → Improve closure
- Explicit "I don't know" as a sanctioned response (PAI's Principle 16,
  "Permission to Fail")

## Cross-system observations (preliminary)

Both required reads (openclaw, PAI) explicitly value:
- **Small core**, extension via plugins/skills/tools
- **Deterministic infrastructure** around probabilistic AI
- **Security posture** with strong defaults
- **Modular capability** that can be added without core changes

Differences:
- openclaw explicitly rejects agent-hierarchy frameworks; PAI lists
  Principle 14 ("Agent Personalities — specialized agents with
  unique voices") which gestures toward multi-agent.
- openclaw's "memory as a singleton plugin slot" suggests architectural
  conservatism on persistence; PAI elevates memory to a top-level
  principle.
- openclaw is operator-driven (user's commands run on user's host);
  PAI is goal-driven (system pursues user's goals).

These shouldn't yet inform Phase 2 candidates — multi-system reading
should establish what's idiosyncratic to each project vs what
cross-validates as a generalizable pattern.

## Phase 1 work plan (subject to evolution)

### Required reads remaining

Both required reads have had a first-pass review (README + VISION.md
where available). Deeper reads queued:

- openclaw: directory survey, architecture pages, key source files
  (gateway core, session management, plugin loading, memory slot)
- PAI: `Tools/`, `Packs/`, `.claude/`, `Releases/v4.0.3/`, the
  scientific-method loop in code

### Further systems to study

Drawn from the redesign prompt's candidate list plus my own
identifications. Order not yet committed.

| System | Why relevant | Mechanism |
|---|---|---|
| AutoGen | Microsoft's multi-agent framework; explicit conversation patterns between agents (relevant to my orchestrator + audit + Copilot setup) | Copilot research-only dispatch (planned cycle 15+) |
| LangGraph | Production state-management for agents; explicit graph-based state | Copilot research-only dispatch or orchestrator-direct |
| Voyager | Long-running self-improving Minecraft agent; skill library accumulation | Orchestrator-direct (the paper is short) |
| Cognition Devin writeups | Autonomous coding agent; production deployment patterns | Orchestrator-direct (blog posts, not a repo) |
| Semantic Kernel | Microsoft's agent SDK; planner/skills split | Copilot research-only dispatch (lower priority) |
| Anthropic engineering posts | Claude Code, agent SDK, internal tooling experience | Orchestrator-direct |

### Cycle plan (provisional)

Cycle 14 (this cycle): openclaw + PAI first-pass; this document; no
dispatch.

Cycle 15+: dispatch options, in priority order:
1. Copilot research-only on AutoGen (multi-agent coordination
   focus). Use cycle-6 procedure (`jq | gh api ... --method POST
   --input -` with `agent_assignment.model: gpt-5.5`) — cycle 11's
   `gh issue create` shortcut produces a PR but does not propagate
   model selection.
2. Orchestrator-direct read of Voyager paper.
3. Optional: parallel Copilot research dispatches on LangGraph
   and Semantic Kernel if cycle-15's load permits and cycle 14's
   in-cycle work suggests value in faster coverage.

The dispatch sequence is tentative. Phase 1 should not over-commit
to a research order before Phase 0 close-out and the actual cycle-15
state determines what's tractable.

### What Phase 1 will produce

A reference document (this file) capturing:
- Each system studied (architecture summary, anchoring caveats)
- Cross-system patterns (with anchoring discipline)
- Tensions surfaced (alignment that may be confirmation bias;
  patterns that may not generalize to autonomous-public-repo
  context)
- A Phase-2-input section listing the patterns that survive
  multi-system reading and have load-bearing relevance to v2
  candidate generation

The Phase-2-input section is **not** to be drafted until at least
3-4 systems have been read. Premature commitment to first-found
patterns is the failure mode to avoid.

## Persistence-mechanism note

This file is the Phase 1 working surface, mirroring the role
`docs/redesign/0-retrospective.md` plays for Phase 0. The
`_notes/cycle-N-*.md` per-cycle convention from Phase 0 carries over:
each cycle's Phase 1 work gets a `_notes/cycle-N-*.md` file, and the
README iteration log (when cycle 15+ updates that section) tracks
Phase 1 cycle progression alongside Phase 0.

Cycle-N-pre-commits-cycle-N+1-checks chain (now nine cycles deep)
extends to Phase 1: each cycle's Phase-1 notes file pre-commits
adversarial-on-adversarial checks for the next cycle, same discipline
as Phase 0 has used since cycle 7.
