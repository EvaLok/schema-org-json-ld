# Cognition Devin (writeups + Devin Wiki)

[← back to Phase 1 index](../../1-research.md)

**Status: stub.** A first-pass Copilot research dispatch landed in
cycle 26 as PR
[#2780](https://github.com/EvaLok/schema-org-json-ld/pull/2780).
Direct content access was constrained (`cognition.ai` was blocked
from the dispatch environment; substance came via secondary
sources). A deeper orchestrator-direct read remains pending under
issue [#2779](https://github.com/EvaLok/schema-org-json-ld/issues/2779).
Per-pattern citations live in the cross-system observations section
of [`1-research.md`](../../1-research.md); this stub assembles those
citations into a per-system view so the asymmetry with the
deeper-read systems (openclaw, PAI, AutoGen, Voyager, LangGraph) is
visible rather than hidden.

## Sources read so far

- "Don't Build Multi-Agents" — Walden Yan, June 2025 (Cognition blog
  post; primary content for the multi-agent stance)
- Devin and Devin Wiki documentation references (Devin 2.0 cross-
  session knowledge analog)
- Secondary sources (cycle-26 dispatch context;
  `cognition.ai` blocked from dispatch environment)

Caveat carried into all observations below: *documented-claim per
cycle-26 source-access note*. Statements about Devin internals are
read off Cognition's published writing, not off code or runtime
behavior.

## Patterns observed (citations resolve to cross-system observations)

- **Anti-pattern as deliverable artifact.** "Don't Build Multi-Agents"
  is a published anti-pattern argument framed as a prohibition with
  named-target rejections (OpenAI Swarm, Microsoft AutoGen). The
  framing is the strongest named-rejection in the surveyed systems.
  Argument: context fragmentation makes multi-agent designs fragile
  (Flappy Bird example: independent agents make incompatible implicit
  decisions).
- **Multi-agent decomposition is not a default.** Single-threaded
  linear agent is the documented default; task-decomposition into
  role-separated sub-agents is rejected. This contradicts the
  small-fixed-team pattern observed in Voyager / Magentic-One /
  oh-my-codex.
- **Agent trace as the unit of context.** "The context isn't just the
  user's message but includes everything the agent has done — code
  files examined, questions asked, and answers received." Devin Wiki
  (Devin 2.0) is the closest documented cross-session persistent
  knowledge analog.
- **Bounded session runtime.** A 45-minute session time limit is
  documented (bounds total runtime rather than iteration count;
  adjacent to oh-my-codex / Voyager iteration-ceiling pattern but on
  a different axis).

## Anchoring caveats

- **Source-access asymmetry.** Most of what's recorded above is
  read off blog/marketing content, not code. Compare the
  AutoGen/LangGraph deep-dives which cite source files at SHAs.
- **Production deployment vs research artifact.** Cognition Devin is
  a deployed commercial product; its documented patterns are at the
  level of "how the product is shaped" rather than "code-level
  primitives." The contrast with Voyager (research code) and
  AutoGen/LangGraph (open-source frameworks) is structural — patterns
  observed here are at coarser granularity.
- **Author-as-promoter.** "Don't Build Multi-Agents" is published by
  the team building Devin; it argues for the architectural choice the
  authors made. Treat as a position-paper-with-evidence, not a
  neutral survey.

## To-be-completed

A deeper read targeting Cognition's primary materials (blog posts,
published architecture pages, Devin 2.0 documentation) is queued via
issue [#2779](https://github.com/EvaLok/schema-org-json-ld/issues/2779).
When that read lands, this file should grow to the structural depth
of the openclaw / PAI / AutoGen / Voyager / LangGraph system files.
