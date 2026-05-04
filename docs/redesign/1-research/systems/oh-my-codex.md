# oh-my-codex (omx — configuration layer + hook harness over Codex CLI)

[← back to Phase 1 index](../../1-research.md)

**Status: stub; cycle-63 deeper read in flight.** A Copilot research dispatch
landed in cycle 26 as PR [#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784),
which is the evidence base for this stub (PR closed-without-merge per the
absorption convention; the 299-line deliverable lives on the never-merged
branch `copilot/redesign-research-phase-1-survey` at commit `f291ec05`,
with patterns absorbed into [`1-research.md`](../../1-research.md)
cross-system observations across cycles 27-32 and split into this
per-system file at cycle 33). A deeper code-level read of the repository
is dispatched at cycle 63 as
[#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833), targeting
the three operationally-largest files cycle-26 explicitly flagged as
not-read-in-full (`src/hooks/keyword-detector.ts` 44KB,
`src/config/generator.ts` 43KB, `src/autoresearch/runtime.ts` 45KB) plus
code-level confirm/refine of cycle-26's 22 named patterns; supersedes the
closed cycle-26 dispatch tracking issue
[#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782).
Per-pattern citations live in the cross-system observations section of
[`1-research.md`](../../1-research.md); this stub assembles those
citations into a per-system view so the asymmetry with the deeper-read
systems is visible rather than hidden. (Of the three cycle-26 dispatches,
oh-my-codex has the densest citation footprint across cross-system
observations.)

## Sources read so far

- PR [#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784)
  Copilot research deliverable (organized into named patterns 1-13+
  including pattern 21 in cycle-26 notes).
- Repository structure references to `.omx/state/`, `.omx/wiki/`,
  `.omx/context/`, `src/state/`, `src/hooks/`, `src/wiki/`,
  `prompts/`, `templates/`, `docs/STATE_MODEL.md`, plus the Rust
  sparkshell extension and three first-party MCP servers.
- `_notes/cycle-26-oh-my-codex-research.md` (cycle-26 working
  reference referenced by cross-system observations).

## Project framing

oh-my-codex (omx) is **a configuration layer + hook harness over an
unmodified Codex CLI**, not an agent framework itself. The README
states explicitly and repeatedly: "OMX does NOT replace Codex." The
project ships 39 skills, 30 role prompts, three first-party MCP
servers, and a Rust sparkshell extension all sitting on top of an
unmodified Codex CLI.

This makes oh-my-codex structurally distinct from the agent
frameworks (AutoGen, LangGraph) and the autonomous agent (Voyager) —
its substrate is "thin layer over an existing CLI" rather than "new
runtime."

## Patterns observed (citations resolve to cross-system observations)

### State, memory, history

- **Per-mode state files.** `.omx/state/<mode>-state.json` with
  explicit session vs root scope reconciliation rules
  (`src/state/workflow-transition-reconcile.ts`) preventing
  compatibility-layer writes from resurrecting completed source
  modes.
- **Failure as recorded artifact.** Ralph progress ledger
  (`.omx/state/<session_or_root>/ralph-progress.json`) records
  failure entries with timestamps. The autoresearch loop's
  `iteration-ledger.json` records keep/discard/stop decisions per
  iteration with reasons.
- **Append-only history with one-way migrations.** File-backed
  migration with one-way compatibility windows (legacy
  `.omx/prd.json` → `.omx/plans/prd-<slug>.md`): legacy files
  preserved as read-only, schema migrations one-way, not destructive.
- **Markdown-first wiki memory.** `.omx/wiki/` markdown wiki with
  MCP wiki server (`src/wiki/`); SessionStart hook can inject
  bounded wiki context; markdown-first, search-first (not
  vector-based).
- **Plans/specs as forward-versioned artifacts.** `.omx/context/
  {task-slug}-{timestamp}.md` written before execution begins, with
  explicit fields for task statement, desired outcome, known facts,
  constraints, unknowns, and codebase touchpoints. (Pattern 2 in
  PR #2784.)

### Quality & discipline

- **Anti-patterns at multiple layers.** CONTRIBUTING.md `<Bad>`
  examples (e.g., "Claiming completion without verification: 'should
  work correctly. Task complete.'"), explicit deprecations
  (`$web-clone` "hard-deprecated"), and `templates/AGENTS.md`
  opening with negative directives.
- **Behavioral prompt-contract regression tests.**
  `src/hooks/__tests__/prompt-guidance-*.test.ts` — agent-affecting
  prose has CI coverage on exact wording. (Pattern 7 in PR #2784.)
- **Iteration ceilings with explicit numerical limits.**
  `max_iterations=10` for tool-loops; `max=5` for review-loops; an
  autoresearch loop with explicit keep/discard/stop per-iteration
  decision. (Pattern 6 in PR #2784.)
- **Mandatory deslop pass.** Required as post-completion step
  (Pattern 13 in PR #2784); the workflow embeds quality cleanup into
  each task's completion contract.

### Agent architecture

- **Per-agent model selection across providers.**
  `src/config/models.ts` declares supported models GPT-5.4,
  GPT-5.4-mini, GPT-5.5, GPT-5.3-codex; the "mini composition seam"
  gates exact-model behavior; `$ask-claude` and `$ask-gemini` skills
  shell to non-OpenAI provider CLIs from within a Codex session.
- **30 named role prompts in `prompts/*.md`.** Metis as analyst,
  Ralph as persistent executor, plus planner / architect / critic /
  verifier / researcher / etc. Workflow stages
  (`$deep-interview` → `$ralplan` → `$ralph` → `$team`) hand off
  across role-named agents.

### Orchestration & system shape

- **Code-vs-prompts split at implementation level.** MCP servers, the
  44KB keyword detector (deterministic pattern matching, not
  semantic classification — pattern 21 in cycle-26 notes), and the
  Rust sparkshell harness all sit deterministically alongside the
  LLM-driven Codex tool loop.
- **Configuration-layer-with-hooks architecture.** Not an agent
  framework — a small entry-point pattern adding 39 skills, 30 role
  prompts, three MCP servers, and a Rust sparkshell extension on
  top of an unmodified Codex CLI.
- **Named workflow modes with deterministic transition policy.**
  `deep-interview`, `ralplan`, `ralph`, `team`, `autopilot`,
  `ultrawork`, `ultraqa` governed by an explicit transition
  allowlist in `docs/STATE_MODEL.md` — multiple orchestration
  patterns coexist with deterministic transition policy preventing
  illegal mode shifts.
- **`$team` runtime as opt-in, not default.** README explicitly
  states "$team is not the default onboarding path." Multi-agent is
  available but de-prescribed for new users.

## Anchoring caveats

- **Configuration layer, not agent framework.** Patterns reflect a
  configuration-and-hooks substrate that wraps an external CLI.
  Compare AutoGen/LangGraph (frameworks defining their own runtime)
  or Voyager (research code with its own loop). Some patterns
  (workflow modes; per-mode state) are tightly coupled to that
  substrate.
- **TypeScript with Rust sparkshell.** Most omx code is TypeScript;
  the Rust sparkshell extension is a small surface. Architectural
  ideas transfer; library affordances do not.
- **Deep deliverable from one Copilot dispatch.** PR #2784 is the
  primary evidence base; patterns 1-13+ are citations against the
  repository as observed by Copilot. A direct repository read may
  shift granularity or surface patterns not yet noted.

## To-be-completed

The dispatch deliverable (PR #2784) plus
`_notes/cycle-26-oh-my-codex-research.md` (on never-merged branch
`copilot/redesign-research-phase-1-survey` per absorption convention)
together cover roughly 22 named patterns. A code-level read with
file:line citations is dispatched at cycle 63 as
[#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833),
targeting the three operationally-largest files cycle-26 flagged as
not-read-in-full (`keyword-detector.ts`, `generator.ts`,
`autoresearch/runtime.ts`) plus the state/hooks/MCP-server modules and
the Rust sparkshell crate. The closed cycle-26 dispatch tracking issue
[#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782) is
superseded by [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833).
When the deliverable lands at
`docs/redesign/_notes/cycle-63-oh-my-codex-deeper-read.md`, this file
should grow to deep-dive depth (per-finding evaluation cycle following
the cycle-41 cognition / cycle-43 openclaw absorption pattern).
