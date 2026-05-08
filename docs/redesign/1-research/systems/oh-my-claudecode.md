# oh-my-claudecode (omc — Claude Code-native multi-agent orchestration plugin, sister to oh-my-codex)

[← back to Phase 1 index](../../1-research.md)

**Status: first-pass; surface architecture + hooks/lifecycle + .omc state model + cycle-26 22-pattern mapping; deeper-read queue documented.**
A Copilot research-only dispatch landed cycle 99 as
PR [#2876](https://github.com/EvaLok/schema-org-json-ld/pull/2876)
(originating issue [#2847](https://github.com/EvaLok/schema-org-json-ld/issues/2847),
cycle-75 dispatch under Eva directive
[#2774](https://github.com/EvaLok/schema-org-json-ld/issues/2774)
which named oh-my-claudecode as a Phase 1 research target alongside
oh-my-codex). Per the absorption convention (PR closed without merge;
deliverable preserved on never-merged branch), the deliverable lives
on branch `copilot/redesign-research-cycle-75-survey-oh-my-claudecode`
as `docs/redesign/_notes/cycle-75-oh-my-claudecode-survey.md`
(429 lines; 9-lens structure: repo glance, architecture surfaces,
skills/prompts/extension, hooks/lifecycle, state/memory/persistence,
docs honesty patterns, cycle-26 22-pattern cross-mapping, NEW patterns,
anchoring caveats). This per-system file summarizes the deliverable
in the per-system shape and cites it as the primary evidence base.

**Verification status (cycle 99):** load-bearing factual claims
spot-checked against the repository — the strongest verification result
in the absorption arc to date.

| Class | Subject | Verdict |
|---|---|---|
| Repo metadata (5) | commits 3076 / contributors 109 (anon=true) / releases 224 / latest tag v4.13.6 / language TypeScript | ✓ all EXACT |
| Repo structure (3) | root item count 50 / agents/*.md count 19 / no `crates/` at root | ✓ all EXACT |
| File sizes (5) | src/hooks/bridge.ts 105722 ≈ ~105KB / src/team/runtime-v2.ts 83319 ≈ ~83KB / src/installer/index.ts 71332 ≈ ~71KB / src/hooks/persistent-mode/index.ts 67124 ≈ ~67KB / src/autoresearch/runtime.ts 54592 ≈ ~55KB | ✓ all match at decimal-KB precision |
| oh-my-codex sister metadata (5) | commits 2441 vs 2442 / contributors 61 (anon=true) / releases 98 vs 99 / latest tag v0.16.1 vs v0.16.2 / root 26 vs 27 | ✓ all within natural drift consistent with ~14h elapsed time since PR creation 2026-05-08T08:30Z (v0.16.2 published 22 minutes after PR creation) |

**Net:** 18 quantitative claims verified — 13 EXACT + 5 within elapsed-time drift. Methodologically the strongest verification result in the absorption arc to date (cycle 96 PR #2878 fabrication-magnitude failure / cycle 97 PR #2877 0.05-0.1% precision verified / cycle 98 PR #2873 byte-level + EXACT metadata / cycle 99 PR #2876 EXACT across 13 metrics + 5 file sizes + sister-comparison drift consistent with elapsed time). PR #2876's author actually counted file sizes and queried GitHub API metadata before producing the survey — calibration-first methodology in the same shape as cycle-97 PR #2877 and cycle-98 PR #2873.

## Sources read so far

- Root README and language-localized variants (`README.md` plus de/es/fr/it/ja/ko/pt/ru/tr/vi/zh translations).
- Agents directory: 19 `agents/*.md` role prompts.
- Skills directory: skill packaging format and skill registry note (`skills/AGENTS.md`).
- Hooks: `hooks/hooks.json` registration file plus `docs/HOOKS.md` documentation (11 Claude Code lifecycle events).
- Architecture doc: `docs/ARCHITECTURE.md` (state model, control-plane/data-plane separation).
- Settings schema doc: `docs/settings-schema.md` (explicit non-enforcement boundary marker).
- MCP integration: `.mcp.json`, `bridge/mcp-server.cjs`, `src/mcp/servers.ts`, `src/mcp/index.ts`, `src/mcp/mcp-config.ts`.
- Operationally-large source files (verified file sizes): `src/hooks/bridge.ts` (105KB), `src/team/runtime-v2.ts` (83KB), `src/installer/index.ts` (71KB), `src/hooks/persistent-mode/index.ts` (67KB), `src/autoresearch/runtime.ts` (55KB).
- Commands and tools modules: `src/commands/index.ts`, `src/tools/state-tools.ts`.
- GitHub API directory listings + repo metadata + commits/contributors/releases counts for verification.

## Project framing

oh-my-claudecode (`omc`) is a **Claude Code-native multi-agent orchestration plugin**, the sister project to `Yeachan-Heo/oh-my-codex` (omx) by the same author. Where omx is a configuration-and-hooks layer over an unmodified OpenAI Codex CLI, omc is a plugin targeting the Claude Code substrate via its native plugin/hooks/skills/agents/MCP surfaces.

The README positions it as "multi-agent orchestration for Claude Code" and explicitly redirects Codex users to oh-my-codex (`README.md:L12-L16`). The quick-start centers on Claude Code plugin install and slash-command usage; the project advertises both an in-session skill surface and a terminal CLI surface, intentionally as two different runtimes (`README.md:L105-L118`).

This is structurally **not** a pure config skin: substantial TypeScript source (`src/`), explicit hook registration (`hooks/hooks.json`), a dedicated MCP server bridge (`.mcp.json` + `bridge/mcp-server.cjs`), and runtime orchestration files (team runtime v2, autoresearch runtime). At the same time it is structurally distinct from agent frameworks (AutoGen, LangGraph, framework defining own runtime) and from autonomous-loop research code (Voyager) — its substrate is "thick plugin over Claude Code" rather than "new runtime."

**Sister-project asymmetry note (H3 verdict).** PR #2876's H3 hypothesis ("oh-my-claudecode is less mature/smaller than oh-my-codex") was REFUTED at first-pass depth. Cycle-99 verification confirmed at EXACT precision: oh-my-claudecode is the **larger** project across multiple metrics (3076 commits vs 2442; 109 contributors vs 61; 224 releases vs 99; 50 root items vs 27; repo-size 45751 vs 16254 — 1.26× to 2.81× larger across 5 metrics). The asymmetry is in the opposite direction from the dispatch hypothesis.

## Patterns observed

### State, memory, history

- **Multi-surface persistence under `.omc/`.** Documentation describes
  `.omc/state/` (mode state, sessions), `.omc/plans`, `.omc/notepads`,
  `.omc/prompts`, `.omc/logs`, `.omc/wiki/` with lifecycle hook integration
  (`docs/ARCHITECTURE.md:L432-L470`). **Documentation-only** at first-pass
  depth — full implementation behavior not traced.
- **Control-plane / data-plane split as architectural primitive.** Docs
  explicitly separate orchestration metadata (`.omc/state/**`) from durable
  artifacts (`.omc/plans`, `.omc/notepads`, `.omc/prompts`) with example file
  layouts, not just ad-hoc file naming (`docs/ARCHITECTURE.md:L461-L470`).
- **Session-scoped vs global-scoped state with explicit paths.**
  Session-scoped: `.omc/state/sessions/{sessionId}/`; global: `~/.omc/state/{name}.json`
  (`docs/ARCHITECTURE.md:L471-L548`). State-tools module includes session-owned
  cleanup logic and mode-scoped operations (`src/tools/state-tools.ts`
  survey-depth grep evidence). Structurally analogous to oh-my-codex's
  `.omx/state/<mode>-state.json` per-mode state with session vs root
  scope reconciliation.
- **Project-memory + notepad as documented multi-session memory primitives.**
  Lifecycle integration via `SessionStart`, `PostToolUse`, `PreCompact`
  hooks (`docs/ARCHITECTURE.md:L499-L542`; `docs/HOOKS.md:L297-L307`).
  **Documentation-only** — implementation enforcement not traced.
- **Autoresearch persistence with run-manifest, ledger, evaluator artifacts,
  candidate files, mode-state, and optional runtime deadline.** Implementation
  evidence at `src/autoresearch/runtime.ts:L961-L1073` (54KB file).
  **Implementation-verified.** Direct analogue to oh-my-codex's autoresearch
  iteration ledger.

### Orchestration & system shape

- **Event-driven Team runtime v2 replacing polling watchdog/done.json loop.**
  Team runtime v2 file header explicitly states replacement of polling watchdog
  loop with event-driven lifecycle operations
  (`src/team/runtime-v2.ts:L2-L17` — 83KB file). **Implementation-verified.**
  Conceptually parallel to Symphony's reconciliation-before-dispatch tick
  (cycle-77 / 98) — both replaced polling with explicit event-driven
  lifecycle, though Symphony achieves this via OTP/BEAM primitives (GenServer +
  Task.Supervisor) and omc via TypeScript event-driven module composition.
- **Explicit phased Team lifecycle vocabulary.** Phase names appear in hook
  bridge constants and pipeline strings: `team-plan` → `team-prd` → `team-exec`
  → `team-verify` → `team-fix` (`README.md:L140-L143`;
  `docs/ARCHITECTURE.md:L250-L253`; `src/hooks/bridge.ts` constants).
  **Implementation-verified + Documentation-only** mixed.
- **Hooks as the primary lifecycle integration mechanism.** All 11 Claude Code
  lifecycle events used: `UserPromptSubmit`, `SessionStart`, `PreToolUse`,
  `PermissionRequest`, `PostToolUse`, `PostToolUseFailure`, `SubagentStart`,
  `SubagentStop`, `PreCompact`, `Stop`, `SessionEnd`
  (`hooks/hooks.json:L4-L210`). **Implementation-verified.**
- **Plugin-root command indirection for portability.** Hook commands consistently
  execute through `node "$CLAUDE_PLUGIN_ROOT"/scripts/run.cjs ...` — a portable
  indirection pattern repeated across all events (`hooks/hooks.json:L10-L17`,
  repeated). **Implementation-verified.** Substrate-specific to Claude Code's
  plugin model.
- **MCP layer as both external server registry + in-process tools server.**
  Project-level `.mcp.json` registers an MCP server via `${CLAUDE_PLUGIN_ROOT}/bridge/mcp-server.cjs`
  (`.mcp.json:L2-L6`); `src/mcp/servers.ts:L4-L110` exports both external server configs
  (Exa, Context7, Playwright, filesystem, memory) and an in-process custom tools server
  (`omcToolsServer`); `src/mcp/index.ts:L17-L23` composes them. **Implementation-verified.**

### Documentation honesty

- **Settings-schema with explicit non-enforcement boundary marker.**
  `docs/settings-schema.md:L40-L42` explicitly states a config behavior remains
  prompt-level workflow contract, not runtime enforcement. **Documentation-only.**
  The honesty marker is itself the load-bearing observation: omc's docs are
  willing to mark the doc-vs-enforcement gap rather than imply universal
  enforcement.
- **Hard-deprecation signaling.** README explicitly calls `omc autoresearch`
  a "hard-deprecated shim" with replacement workflow (`README.md:L356-L367`);
  README marks `swarm` alias removed and asks migration to `/team`
  (`README.md:L132-L133`, `L326-L327`). **Documentation-only.**
- **Doc-to-enforcement linkage where present.** Hooks docs claim permission
  enforcement (`docs/HOOKS.md:L118-L137`) and hook registration confirms wiring
  (`hooks/hooks.json:L63-L85` registers `pre-tool-enforcer.mjs` +
  `permission-handler.mjs`). **Implementation-verified for registration;
  behavior details mostly Documentation-only.**
- **No prominent `<Bad>`/`<Good>` style anti-pattern catalog at first-pass depth.**
  Unlike oh-my-codex (which has CONTRIBUTING.md `<Bad>` examples plus
  `templates/AGENTS.md` opening with negative directives — cycle-26 pattern 7),
  no equivalent surfaced in surveyed focal docs. Could exist in templates or
  REFERENCE/FEATURES docs not deeply sampled. **Needs deeper read.**

### Quality & discipline

- **Iteration ceilings with explicit deadlines (autoresearch).** Runtime
  supports bounded runtime via `max_runtime_ms` and `deadline_at` metadata
  (`src/autoresearch/runtime.ts:L949-L1073`). **Implementation-verified.**
  Direct analogue of cycle-26 oh-my-codex pattern 6 (max_iterations=10 for
  tool-loops; max=5 for review-loops; explicit keep/discard/stop).
- **AI-slop-cleaner skill present.** Acknowledged in skills surface
  (`skills/AGENTS.md:L53-L56`; `README.md:L297-L298`). **Documentation-only.**
  Mandatory post-completion enforcement (cycle-26 oh-my-codex pattern 13)
  not verified at first-pass depth.

### Trust posture & security defaults

- **Permission enforcement via PreToolUse + PermissionRequest hooks.**
  Hook docs call out enforcement responsibility (`docs/HOOKS.md:L118-L137`);
  hook registration confirms `pre-tool-enforcer.mjs` and `permission-handler.mjs`
  wiring (`hooks/hooks.json:L63-L85`). **Implementation-verified for registration.**
- **Persistent-mode continuation gate on `Stop`.** Lifecycle augmentations
  documented include persistent continuation gate, pre-compact state preservation,
  subagent tracking start/stop, session start/end persistence
  (`docs/HOOKS.md:L187-L207`, `L228-L237`; `hooks/hooks.json:L150-L209`).
  **Implementation-verified for registration; behavior details mostly Documentation-only.**

### Agent architecture

- **Hybrid extension shape: skills + agents + missions + slash commands +
  MCP + hooks.** Top-level root contains all of: `agents/`, `skills/`, `hooks/`,
  `bridge/`, `missions/`, `.claude-plugin/`, `.mcp.json`, `src/` (verified;
  50 root items total). This is structurally distinct from pure-skill or
  pure-prompt extension models — multiple complementary extension surfaces
  composed.
- **19 named agent role prompts.** Verified count at `agents/*.md = 19`.
  Structurally analogous to oh-my-codex's 30-role-prompt surface (cycle-26
  pattern: Metis as analyst, Ralph as persistent executor, planner / architect
  / critic / verifier / researcher / etc.) but lower count and different
  directory naming (`agents/` vs `prompts/`).
- **Skills as `skills/<name>/SKILL.md` markdown with YAML frontmatter,
  invoked via `/oh-my-claudecode:<skill-name>`.** Auto-detected registry per
  frontmatter; `build-skill-bridge.mjs` for generated bridge wiring
  (`skills/AGENTS.md:L79-L131`; `package.json:L41-L47`).
- **Multi-provider optionality (Codex/Gemini paths).** README documents
  provider-specific paths (`README.md:L156-L166`, `L512-L519`); recent change
  removed Codex/Gemini MCP servers in favor of CLI worker mode for those
  providers. Direct analogue of oh-my-codex's `$ask-claude` and `$ask-gemini`
  cross-provider invocation skills (cycle-26 pattern), inverted: omc shells out
  to non-Claude providers from a Claude Code session, mirroring omx's posture
  of shelling out to non-Codex providers from a Codex session.

## Hypothesis verdicts (per cycle-75 dispatch)

The deliverable's H1-H3 hypothesis verdicts at first-pass depth:

- **H1 parallel-architecture hypothesis (oh-my-claudecode mirrors
  oh-my-codex's keyword-detector + generator/composition + autoresearch
  runtime + MCP layer + Rust substrate-edge architecture):
  CONFIRMED at first-pass depth (4/5 components present).** Mapping:
  (1) keyword detector analogue present (`hooks/hooks.json:L4-L20`;
  `docs/HOOKS.md:L215-L223`); (2) generator/composition analogue present
  via installer/config assembly (`src/installer/index.ts:L31-L47`); (3)
  autoresearch runtime analogue present (`src/autoresearch/runtime.ts:L945-L1024`);
  (4) MCP layer present (`.mcp.json:L2-L6`; `src/mcp/index.ts:L5-L23`);
  (5) Rust substrate-edge component **NOT FOUND** — no `crates/` at root
  (verified). Score 4/5.
- **H2 substrate-specific patterns hypothesis (Claude lifecycle hooks,
  slash-command first-class invocation, Claude config deployment surfaces,
  MCP `.mcp.json` integration as Claude-specific): CONFIRMED (≥3 substrate-specific
  patterns visible).** All four named substrate-specific patterns observed
  across `docs/HOOKS.md`, `hooks/hooks.json`, `README.md`, `src/installer/index.ts`,
  and `.mcp.json`.
- **H3 substrate-investment asymmetry hypothesis (oh-my-claudecode is
  less mature / smaller than oh-my-codex): REFUTED on current-repo metrics
  (opposite direction observed).** Verified at cycle-99 absorption: oh-my-claudecode
  is **larger** across 5 metrics: 3076 vs 2442 commits (1.26×), 109 vs 61
  contributors (1.79×), 224 vs 99 releases (2.26×), 50 vs 27 root items (1.85×),
  45751 vs 16254 KB repo-size (2.81×). The H3 dispatch hypothesis was wrong
  about direction; first-pass evidence supports the **opposite** asymmetry.

## Cross-reference to cycle-26 22-pattern catalogue

Per the cycle-75 deliverable (section 7), oh-my-claudecode's mapping
against the cycle-26 oh-my-codex 22 named patterns
(per-pattern citations: `_notes/cycle-75-oh-my-claudecode-survey.md` table at section 7):

- **PARALLEL (5):** explicit stop conditions + escalation (#3); parallel
  delegation preference (#8); session/root state with reconciliation (#9);
  autoresearch bounded supervisor loop (#12); deterministic keyword detection
  first (#21).
- **ADAPTED (9):** workflow as named keywords + transition policy (#1);
  context snapshot grounding (#2); evidence-backed completion (#4); iteration
  limits / explicit ceilings (#6); file-backed migration compatibility windows (#10);
  mandatory deslop post-completion (#13); persistent local wiki knowledge base (#14);
  advisory triage without direct mode activation (#18); compatibility layer
  separate from authoritative state (#20).
- **ABSENT (6):** pre-execution gate for underspecified requests (#5);
  behavioral prompt contract + tests (#7); doc-refresh warning in commit path (#11);
  autonomy directive first-line contract (#17); critical-module coverage gates (#19);
  commit-signing / lore-format guardrails (#22).
- **NEEDS-DEEPER-READ (2):** MCP transport failure fallback (#15); install
  false-green detection (#16).

Interpretation: oh-my-claudecode shows **more parallel/adapted overlap** with
oh-my-codex's 22-pattern catalogue (14 of 22 = 64%) than Symphony does (13 of
22 = 59% PARALLEL+ADAPTED), consistent with sister-project structural alignment
on the Codex-CLI-adjacent thin-wrapper-over-existing-CLI substrate. ABSENT
clusters are different from Symphony's: Symphony's ABSENTs cluster in
keyword-mode UX (Symphony is tracker-state-driven, not keyword-triggered);
oh-my-claudecode's ABSENTs cluster in commit-discipline patterns (no behavioral
prompt-contract tests, no autonomy directive first-line contract, no commit-signing
guardrails surfaced at first-pass depth).

## NEW patterns surfaced by oh-my-claudecode (single-system, pending elevation)

Per the cycle-75 deliverable (section 8), patterns oh-my-claudecode adds
that are not captured by the cycle-26 22-pattern baseline:

1. **Explicit dual-runtime split (in-session native Team vs terminal tmux Team).**
   README distinguishes `/team` in-session runtime and `omc team` terminal
   runtime as intentionally different execution paths
   (`README.md:L116-L118`, `L138-L143`, `L248-L250`). **Documentation-only.**
2. **Event-driven Team runtime v2 replacing done-file polling.**
   Team runtime v2 header explicitly states replacement of polling watchdog
   and done.json loop with event-driven lifecycle operations
   (`src/team/runtime-v2.ts:L2-L17`). **Implementation-verified.**
   Potential 2-system convergence with Symphony's reconciliation-before-dispatch
   pattern at conceptual level — both replaced polling with event-driven
   lifecycle. Held as single-system pending Symphony deeper-read parity.
3. **Plugin-root hook command indirection for portability.** Hook commands
   consistently execute through `node "$CLAUDE_PLUGIN_ROOT"/scripts/run.cjs ...`
   across all events (`hooks/hooks.json:L10-L17`, repeated through file).
   **Implementation-verified.** Substrate-specific to Claude Code's plugin
   architecture; transferable as a *substrate-rooting indirection* pattern
   to other plugin-based substrates.
4. **In-process MCP "tools server" plus optional external MCP registry.**
   Internal tools server (`omcToolsServer`) and external server builders
   (`createExaServer`, `createContext7Server`, etc.) coexist
   (`src/mcp/index.ts:L17-L23`; `src/mcp/servers.ts:L19-L110`).
   **Implementation-verified.**
5. **State-plane / data-plane split documented as architectural primitive.**
   Docs explicitly separate orchestration metadata from durable artifacts
   with examples, not just ad-hoc file naming
   (`docs/ARCHITECTURE.md:L461-L470`). **Documentation-only.** Conceptual
   alignment with cycle-26 oh-my-codex per-mode-state-with-reconciliation
   could elevate this to 2-system if implementation parity is confirmed
   in deeper reads.
6. **Explicit non-auto-detection of `team` keyword to avoid recursive spawn.**
   Hook docs state `team` must be explicit slash invocation (not magic
   keyword auto-detect), explicitly to avoid infinite spawning
   (`docs/HOOKS.md:L456-L462`). **Documentation-only.** Related to
   cycle-26 pattern 21 (deterministic keyword detection first) but inverted:
   instead of "deterministic detection over heuristic," it's "explicit
   invocation over any detection" for safety-critical workflows.
7. **Settings-schema doc that marks prompt-level contract vs runtime
   enforcement.** `docs/settings-schema.md:L40-L42` explicitly states a config
   behavior remains prompt-level workflow contract, not runtime enforcement.
   **Documentation-only.** This is documentation-honesty pattern;
   single-system at first-pass.
8. **Hybrid provider topology after Codex/Gemini MCP removal.** README
   describes recent removal of Codex/Gemini MCP servers in favor of CLI
   worker mode for those providers (`README.md:L156-L166`).
   **Documentation-only.** Architectural posture: deterministic orchestration
   machinery separated from provider-specific adapters; provider adapters
   migrated from MCP-server topology to CLI-worker topology.

These are **single-system observations** at first-pass depth. Per the
cross-system convergence discipline, single-system patterns are held
pending elevation via deeper-read or deeper adversarial-on-adversarial
review. NEW-2 (event-driven runtime replacing polling) is the strongest
elevation candidate via potential Symphony convergence; NEW-5 (state/data-plane
split) is a candidate via potential oh-my-codex convergence; the remainder
are single-system at present.

When consolidated to genuinely-distinct architectural axes (per cycle-98
honest-reflection discipline), these 8 NEW patterns collapse to **~4
distinct shapes**: (a) **dual-runtime split** [NEW-1], (b) **event-driven
lifecycle replacing polling** [NEW-2 + the Team v2 vocabulary], (c) **substrate-rooted
plugin indirection** [NEW-3 + NEW-4 hybrid MCP topology + NEW-8 provider-CLI-worker
migration; all manifestations of plugin-substrate-specific composition],
and (d) **documentation-honesty as architectural primitive** [NEW-7 settings-schema
non-enforcement marker]. NEW-5 (control-plane/data-plane split) is implementation-detail
of broader state-architecture and NEW-6 (explicit non-auto-detection) is a
safety-driven inversion of cycle-26 #21. Genuinely-novel-vs-corpus contributions:
~3-4 architectural axes, with NEW-2 strongest as cross-system elevation candidate.

## Anchoring caveats

- **First-pass depth, not deep-dive parity.** The cycle-75 read covered
  README + agents-listing + skills surface + hooks registration + architecture
  doc + MCP integration + key file-size measurement, but **deferred** full
  enforcement-path tracing (post-tool-verifier, permission-handler,
  project-memory lifecycle), full keyword-detector implementation tracing in
  `src/hooks/bridge.ts`, full migration-mechanics audit beyond
  `docs/MIGRATION.md`, full anti-pattern-catalog search outside first-pass
  focal files, and full MCP failure-mode behavior validation. A deeper read
  is the prerequisite for treating oh-my-claudecode as a 9th deep-dive
  system in the cluster catalogue (Symphony reached first-pass at cycle 98;
  oh-my-claudecode reaches first-pass at cycle 99 — both pending deeper-read
  for cluster catalogue elevation).
- **Documentation-vs-implementation asymmetry.** Many architectural claims
  are Documentation-only (`.omc/` state model details, project-memory lifecycle,
  enforcement boundaries). **Implementation-verified** claims focus on
  hook registration, file presence/size, MCP composition, and a small set of
  source-file headers. Behavior validation is deferred.
- **Sister-project read positions oh-my-claudecode against oh-my-codex,
  not against the corpus broadly.** The 22-pattern cross-reference table is
  oh-my-codex-anchored; oh-my-claudecode's mapping reflects sister-project
  structural alignment more than independent corpus position. Deeper reads
  on oh-my-claudecode and oh-my-codex together would be needed to disentangle
  shared-author conventions from sister-substrate convergent design.
- **Invocation model mismatch (interactive user-driven vs autonomous cron).**
  oh-my-claudecode is built around user-invoked slash/CLI flows
  (`README.md:L105-L118`); the redesign target runs autonomously via cron.
  Direct UX ergonomics around slash-command invocation and interactive setup
  wizarding don't transfer; event-hook lifecycle segmentation, explicit
  mode-state persistence contracts, and hook-driven enforcement primitives do.
- **Multi-provider optionality vs narrower orchestrator scope.** omc supports
  optional Codex/Gemini orchestration (`README.md:L512-L519`) and provider-specific
  paths; redesign context is single-provider. Provider-specific advisor
  orchestration details don't transfer; separation of deterministic
  orchestration machinery from provider-specific adapters does.
- **Team runtime sophistication may exceed single-stakeholder needs.** omc
  includes large Team runtime with worker orchestration semantics
  (`src/team/runtime-v2.ts` 83KB). Full multi-worker tmux orchestration
  complexity is beyond redesign scope; explicit phase boundaries, event-driven
  state transitions, and bounded supervision loops are transferable shapes.
- **Documentation volume and release velocity differences.** omc has high
  release churn (224 releases) and large docs surface; this reflects broad
  public-package maintenance and marketplace/plugin distribution overhead
  that doesn't apply to redesign context. Honest doc-to-enforcement boundary
  marking and explicit deprecation signaling are transferable disciplines.
- **Claude substrate alignment is high** (this *increases* comparability for
  hook/event semantics, session-state handling, permission-hook placement,
  command-surface composition). Unlike omx (Codex-CLI-targeted), omc is
  natively Claude Code-targeted. For hook semantics specifically, omc is the
  closest direct comparable in the corpus.

## To-be-completed (deeper-read queue)

The cycle-75 deliverable explicitly defers five follow-up items to a
deeper-read stage:

1. Confirm whether doc-claimed enforcement paths (`post-tool-verifier`,
   `permission-handler`, project-memory lifecycle) are fully implemented
   fail-closed or partially advisory — important for cluster I substrate-correlation
   analysis on enforcement-boundary integrity.
2. Trace keyword detector + skill injector implementation details in
   `src/hooks/bridge.ts` (105KB) and underlying modules for conflict and
   precedence semantics. Direct analogue of cycle-63 oh-my-codex deeper-read
   on `keyword-detector.ts` (44KB).
3. Verify migration mechanics beyond docs (`docs/MIGRATION.md`) in code
   paths — relevant for cluster B / cycle-26 pattern 10 (file-backed migration
   compatibility windows) confirmation.
4. Verify whether any anti-pattern catalogs equivalent to cycle-26 `<Bad>`
   patterns exist outside first-pass sampled files — currently classified
   ABSENT but **Needs deeper read** before elevation.
5. Validate MCP failure-mode behavior and fallback semantics in real runtime
   paths (not just available modules) — currently classified
   NEEDS-DEEPER-READ; resolution would move pattern #15 to
   PARALLEL or ABSENT.

A code-level deeper-read dispatch could be filed under a new research issue
if oh-my-claudecode moves from first-pass to deep-dive status. At present,
oh-my-claudecode's first-pass depth is adequate for Phase 1 corpus expansion
(8th deep-dive system + Symphony first-pass + oh-my-claudecode first-pass)
and does not block Phase 2 candidate iteration; a deeper read would be
needed before treating oh-my-claudecode as full deep-dive parity in the
cluster catalogue.

A natural deeper-read pairing would dispatch oh-my-claudecode and oh-my-codex
deeper-reads together — both sister projects at first-pass + stub status,
both with similar deeper-read scope (operationally-largest files + enforcement-path
tracing + 22-pattern code-level confirm/refine). The cycle-63 oh-my-codex
deeper-read [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
PR [#2874](https://github.com/EvaLok/schema-org-json-ld/pull/2874) is queued
for cycle-100 absorption.
