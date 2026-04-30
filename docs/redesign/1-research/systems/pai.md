# PAI (danielmiessler/Personal_AI_Infrastructure, 12k★ / 1.6k forks / TypeScript)

[← back to Phase 1 index](../../1-research.md)

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
