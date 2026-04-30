# openclaw (openclaw/openclaw, 365k★ / 75k forks / TypeScript)

[← back to Phase 1 index](../../1-research.md)

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
