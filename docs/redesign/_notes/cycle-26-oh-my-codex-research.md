# Cycle 26 — oh-my-codex repository survey (Phase 1 cross-system reading)

**Source**: https://github.com/Yeachan-Heo/oh-my-codex  
**Read date**: 2026-04-29  
**Reader note**: This is a cold read. The findings below are drawn from the repository's source code, documentation, and configuration files at commit `f0d9b3d`. Where claims are inference-from-source rather than documentation-backed, that is marked explicitly.

---

## 1. Overall architecture and named primitives

**What oh-my-codex does**: OMX is a multi-agent orchestration workflow layer that sits on top of OpenAI's Codex CLI. It does not replace Codex; Codex remains the execution engine. OMX installs a curated set of prompt files, skill definitions, configuration templates, native lifecycle hooks, and runtime scripts that transform a bare Codex CLI session into a structured workflow environment with named roles, reusable skill workflows, and durable state.

**Entrypoint shape**: CLI daemon. The binary is `omx` (`dist/cli/omx.js`). Users run `omx --madmax --high` to start a Codex session pre-loaded with OMX configuration, or they use sub-commands like `omx setup`, `omx doctor`, `omx team`, `omx explore`, `omx wiki`, `omx exec`. The interactive path is a human at a terminal who types `$skill-name "..."` inside a running Codex session. There is also an autoresearch path (`omx autoresearch <mission>`) that is closer to a bounded autonomous loop; it runs supervisor/candidate cycles and records logs under `.omx/logs/autoresearch/<run-id>/`.

**Relationship to Codex CLI**: OMX is a **configuration layer and hook harness** around OpenAI Codex CLI, not a fork. It does not own the model call, the tool call loop, or the execution sandbox — Codex owns all of that. OMX adds:
- Prompt installation: copies `prompts/*.md` to `~/.codex/prompts/`
- Skill installation: copies `skills/*/SKILL.md` to `~/.codex/skills/`
- Config generation: writes `~/.codex/config.toml` via `src/config/generator.ts`
- Hook registration: writes `.codex/hooks.json` to register OMX-managed lifecycle scripts at `SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, and `Stop`
- State store: maintains `.omx/state/` files for workflow mode tracking
- Runtime services: a MCP state server (`src/mcp/state-server.ts`), a wiki MCP server, a tmux team runtime, and a HUD monitor

**Named primitives**:

| Primitive | Location | Description |
|---|---|---|
| Role prompts | `prompts/*.md` | System prompts for named agent roles (executor, planner, architect, critic, analyst, verifier, researcher, etc.). 30 files. Installed to `~/.codex/prompts/`. Invoked as `/prompts:name` inside Codex. |
| Skills | `skills/*/SKILL.md` | Workflow procedure files for reusable task patterns. 39 skill directories. Installed to `~/.codex/skills/`. Invoked as `$skill-name "..."` inside Codex. |
| Modes | `src/modes/` | Named workflow states (ralph, ralplan, deep-interview, team, autopilot, ultrawork, ultraqa). State is tracked in `.omx/state/<mode>-state.json`. |
| Session | `.omx/state/sessions/<session_id>/` | Per-session state scope. Session state wins over root state when both exist. |
| Team | `src/team/` | Parallel execution via tmux: N Codex or Claude CLI sessions in coordinated split panes, sharing state through `.omx/state/team/` files and `omx team api ...` commands. |
| Hook | `src/hooks/`, `.codex/hooks.json`, `.omx/hooks/*.mjs` | Lifecycle interceptors. Native Codex hooks (SessionStart, UserPromptSubmit, PreToolUse, PostToolUse, Stop) plus OMX plugin hooks in `.omx/hooks/*.mjs` plus tmux/runtime fallback paths. |
| MCP server | `src/mcp/` | First-party Model Context Protocol servers: state server (state_write/state_read/state_clear), wiki server, trace server. |
| `.omx/` | runtime | Local working directory for plans, logs, state, wiki, context snapshots, and progress ledgers. gitignored by default. |
| Templates | `templates/AGENTS.md` | A 23KB `AGENTS.md` template installed at project setup. This is the primary runtime guidance document; it carries the autonomous-agent directive and orchestration sharpness rules. |

The `$deep-interview`, `$ralplan`, `$ralph`, and `$team` keywords are the canonical four-step workflow:
1. `$deep-interview` — Socratic clarification before execution
2. `$ralplan` — consensus planning via Planner → Architect → Critic loop
3. `$ralph` — persistent completion loop with tiered architect verification
4. `$team` — coordinated parallel execution with tmux workers

**Plugin layout**: OMX also ships a Codex plugin layout at `plugins/oh-my-codex/` with marketplace metadata in `.agents/plugins/marketplace.json`. This is a second installation path (plugin discovery) alongside the primary `npm install -g oh-my-codex` + `omx setup` path. The plugin bundles the skill surface and plugin-scoped companion metadata for MCP servers. The README is explicit that the plugin install does NOT replace the native setup path; legacy setup installs native agents and prompts, while plugin setup relies on plugin discovery and archives legacy OMX-managed prompt/TOML files to prevent shadowing.

**Sibling project**: `Yeachan-Heo/oh-my-claudecode` is referenced in the community Discord. It is a related project for the Claude Code CLI, applying OMX-style patterns to Anthropic's Claude Code tool rather than OpenAI Codex CLI. The two repositories are separate; this survey did not read oh-my-claudecode in depth but its existence implies the OMX workflow layer pattern is being ported across coding agent CLIs. The README links to it as a companion project for users on the Anthropic stack.

**External links in README**: The README links to `yeachan-heo.github.io/oh-my-codex` as a project homepage. No external papers or research citations were found. The README's depth is entirely operational: installation, configuration, workflow, and troubleshooting. There is no design rationale section explaining why specific architectural choices were made — design rationale, when it exists, is in CONTRIBUTING.md, commit messages, and issue comments rather than a dedicated ADR directory.

---

## 2. Configuration model

**How OMX is configured**: Configuration is programmatic (TypeScript) generating declarative output. The core generator is `src/config/generator.ts` (43KB), which produces a TOML config file for Codex at `~/.codex/config.toml`. The generated config includes model selection, token window settings, `developer_instructions` (a large text block injected as context), and feature flags.

**Where prompts live**: In `prompts/*.md` files at the repository root. These are markdown files that get copied by `omx setup` to `~/.codex/prompts/`. They are not programmatically composed at the TypeScript level — they are verbatim markdown files that Codex reads from disk when `/prompts:name` is invoked. The CONTRIBUTING.md note ("read `docs/prompt-guidance-contract.md` before changing prompts") treats these as a contract surface requiring coordinated update.

**Where tool definitions live**: OMX does not define tools directly. It relies on Codex's built-in tool set (bash shell execution, file editing, etc.) plus MCP servers. MCP server configuration (which first-party MCP servers to register) lives in `src/config/omx-first-party-mcp.ts` and `src/config/mcp-registry.ts`. The MCP servers expose tools like `state_write`, `state_read`, `state_clear`, `wiki_query`, `wiki_add` as callable tools from within Codex sessions.

**Declarative vs. programmatic**: Mixed. Prompt files and skill files are declarative markdown. The configuration generator is programmatic TypeScript. Hook logic is programmatic TypeScript scripts. There is no single declarative config file that covers the full OMX setup — you need to run `omx setup` which executes the generator and installer.

**Model support matrix**: `src/config/models.ts` (9KB) defines the supported model list. OMX explicitly supports multiple models including GPT-5.4, GPT-5.4-mini, GPT-5.5, GPT-5.3-codex, and others. The generator in `src/config/generator.ts` seeds `gpt-5.5` config with `model_context_window = 250000` and `model_auto_compact_token_limit = 200000` when those keys are missing from an existing config.

**Multiple use cases / projects**: OMX uses a project-local `AGENTS.md` file (installed from `templates/AGENTS.md` via `omx setup`) as the project-scoped guidance document. The `--merge-agents` flag on setup merges into an existing `AGENTS.md` using marker comments (`<!-- OMX:AGENTS:START -->` / `<!-- OMX:AGENTS:END -->`). The `.omx/` directory is per-project. Multiple projects are supported by running OMX inside different working directories.

**Code-generated developer_instructions**: `src/config/generator.ts` generates a large `developer_instructions` blob that is injected into the Codex config. This is distinct from `templates/AGENTS.md`: the generator-produced text covers prompt guidance contract patterns, routing metadata, and model-specific adaptations. The template AGENTS.md covers broader orchestration guidance. Both are consulted by contributors updating prompts (per `docs/prompt-guidance-contract.md`). The generator is large (43KB) and is the single most important source file for understanding what the runtime looks like to the model — it is the programmatic assembly point for the Codex session context.

**Launch policy and overrides**: OMX has a documented launch policy hierarchy: CLI flag wins over environment, last CLI flag before `--` wins. Supported policies: `direct`, `tmux`, `detached-tmux`, `auto`. Environment variable `OMX_LAUNCH_POLICY` sets a persistent preference. A config-file setting is explicitly not provided ("intentionally does not add a config-file setting" — README). This policy hierarchy is deterministic and documented.

---

## 3. Prompt structure and discipline

**Structure**: Prompts are monolithic XML-tagged markdown files, not composed from pieces at runtime. Each `prompts/*.md` file is a self-contained role prompt with a consistent internal XML structure:
- `<identity>` — who the agent is and what they are responsible for
- `<constraints>` — what is blocked, what the default behavior is
- `<explore>` — the agent's analysis/investigation steps
- `<execution_loop>` — success criteria, verification loop, tool persistence
- `<delegation>` — escalation and routing rules
- `<style>` — output contract, anti-patterns, scenario handling, final checklist

Skills (`skills/*/SKILL.md`) use a YAML front-matter header (`name`, `description`, `argument-hint`) followed by free-form markdown that describes the workflow in prose, with XML sections like `<Purpose>`, `<Use_When>`, `<Do_Not_Use_When>`, `<Steps>`, `<Examples>`, `<Tool_Usage>`, and `<Final_Checklist>`. The skill files can be quite long: `skills/ralph/SKILL.md` is a 7KB file with detailed step-by-step procedure, state management instructions, and scenario examples. `skills/deep-interview/SKILL.md` is 26KB.

The `templates/AGENTS.md` (23KB) is the runtime orchestration brain — its opening lines are a directive block in all-caps: "YOU ARE AN AUTONOMOUS CODING AGENT. EXECUTE TASKS TO COMPLETION WITHOUT ASKING FOR PERMISSION. DO NOT STOP TO ASK 'SHOULD I PROCEED?' — PROCEED." This is installed at project setup and becomes the root AGENTS.md for the project.

**Prompt-engineering patterns visible**:
- Explicit role framing with persona names (Metis for analyst, Ralph for persistent executor)
- Negative constraints ("Read-only: Write and Edit tools are blocked" in `prompts/analyst.md`)
- Escalation gates — agents cannot complete work, they escalate findings to the leader
- Scenario-based Good/Bad examples embedded in the prompt files
- Final checklist at the end of every skill and some prompts
- Tiered effort levels (`--quick`, `--standard`, `--deep` in `$deep-interview`)
- "Ask only when truly ambiguous or destructive" as the stated question-asking policy
- Output contract sections that specify the exact shape of final output per role

**Prompt versioning and updating**: There is no version header in the prompt files themselves. Instead, OMX uses a formal `docs/prompt-guidance-contract.md` that specifies four behavioral patterns (quality-first output, automatic follow-through, localized task-update overrides, persistent tool use) and maps each to specific line numbers in specific prompt files. Regression tests in `src/hooks/__tests__/prompt-guidance-*.test.ts` validate that the behavioral wording contract is preserved across edits. CONTRIBUTING.md requires contributors to read the contract before changing any prompt surface. This is the closest thing to prompt versioning: a behavioral contract with test coverage, not a version number.

**Gap between README claims and code**: The README's "Prompt guidance contract" section points to `docs/prompt-guidance-contract.md` and says it defines the GPT-5.4 behavior contract. The code does enforce this via test files like `src/hooks/__tests__/prompt-guidance-contract.test.ts`. This appears backed by code. One area to verify more carefully would be whether the `docs/guidance-schema.md` section layout contract (also referenced in the prompt-guidance-contract.md) is enforced by automated test or only by convention — that file was not read in depth for this survey.

---

## 4. Tool definition and integration

**Tools provided or integrated**: OMX does not define its own tool surface for the agent loop — it inherits Codex's built-in tools (bash, file read/write/edit, etc.). What OMX adds is:

1. **MCP tool servers**: Three first-party MCP servers:
   - State server (`src/mcp/state-server.ts`): exposes `state_write`, `state_read`, `state_clear` for workflow mode persistence
   - Wiki server (`src/wiki/`): exposes `wiki_query`, `wiki_add`, `wiki_list`, `wiki_lint`, `wiki_refresh` over `.omx/wiki/`
   - Trace server (`src/mcp/trace-server.ts`): tracing/observability surface

2. **Shell scripts**: `src/scripts/ask-claude.sh`, `src/scripts/ask-gemini.sh` — scripts to invoke other LLM providers from within a Codex session. These are callable skills (`$ask-claude`, `$ask-gemini`) that shell out to the respective CLI tools.

3. **Spark shell** (Rust): `crates/omx-explore-harness` is a Rust binary for low-cost repository exploration. `omx sparkshell <command>` wraps shell execution through this harness. The explore routing (`src/hooks/explore-routing.ts`) decides whether to use the low-cost sparkshell-backend or the full Codex session for `omx explore` calls. `build:explore:release` in `package.json` compiles this Rust component for distribution.

4. **Keyword detector** (`src/hooks/keyword-detector.ts`, 44KB): A substantial TypeScript module that parses UserPromptSubmit events, detects skill invocation keywords (`$ralph`, `$ralplan`, etc.), and injects additional context into the prompt. This is how OMX routes skill invocations without patching Codex — it intercepts prompts at the native hook boundary and adds context.

**How tools are declared**: MCP tools are declared in TypeScript using the `@modelcontextprotocol/sdk` package. There is no standalone YAML/JSON tool manifest — tool declarations live in the MCP server implementation files (`src/mcp/`).

**Where tool execution happens**: In-process for MCP server tools. Shelled out via Codex's own bash execution for most agent work. The spark shell is a separate Rust process started by the CLI.

**Tool call loop / ReAct pattern**: OMX relies on Codex's native ReAct-style tool call loop (plan → act → observe). OMX does not implement its own tool loop; it adds lifecycle hooks around the Codex loop's entry/exit points (SessionStart, UserPromptSubmit, Stop). The skill files specify patterns for the agent ("Fire independent agent calls simultaneously — never wait sequentially for independent work"), but the actual parallelism happens inside Codex's model call, not in OMX code.

**Tool failure handling**: At the prompt level, skill files instruct agents on failure recovery (ralph's step 9: "fix the issues raised, then re-verify at the same tier"). The hook layer (`src/hooks/`) has post-tool-use processing that provides guidance for `command-not-found`, `permission-denied`, and `missing-path` errors from Bash results (per `docs/codex-native-hooks.md`). MCP transport failures have a fallback path: if `omx_state` MCP call fails, retry via CLI: `omx state write --input '<json>' --json` (`skills/ralph/SKILL.md`, Tool_Usage section).

---

## 5. Failure handling and reliability

**Persistence across runs — Ralph's loop**: The `$ralph` skill is explicitly designed for persistence across iterations. It tracks state with `state_write({mode: "ralph", iteration: N, current_phase: "executing"})` and on completion writes `state_write({mode: "ralph", active: false, ...})`. The Stop hook (`docs/codex-native-hooks.md`) re-blocks active Ralph sessions on Codex's native Stop event, keeping the loop running. The architectural pattern: the agent's own Stop signal triggers a hook that reads current state and, if ralph is active, injects a re-start signal into Codex's next response, implementing a durable loop via Codex's native event system rather than via an external process supervisor.

**File-backed state**: `src/ralph/persistence.ts` writes two artifacts:
- `.omx/plans/prd-<slug>.md` — canonical PRD markdown (migrated from legacy `.omx/prd.json`)
- `.omx/state/<session_or_root>/ralph-progress.json` — a JSON ledger with `schema_version`, timestamped entries, and visual feedback history (last 30 entries)

Legacy migration is explicitly handled: if `.omx/prd.json` exists and no canonical PRD exists, a one-way migration runs on next startup. The ledger is written with a `stableJsonPretty()` function that sorts keys for deterministic diffs.

**Context snapshots**: Before starting ralph execution work, `skills/ralph/SKILL.md` requires assembling a context snapshot at `.omx/context/{task-slug}-{timestamp}.md`. Minimum fields: task statement, desired outcome, known facts/evidence, constraints, unknowns/open questions, likely codebase touchpoints. This is explicitly grounding work before execution begins.

**Iteration limits and stop conditions**: Ralph has a `max_iterations: 10` default. The `$ralplan` skill has a re-review loop with `max 5 iterations`. The Escalation_And_Stop_Conditions section of ralph specifies exact stop conditions: fundamental blocker requiring user input, user says "stop"/"cancel"/"abort", or the same issue recurs across 3+ iterations (which triggers a "potential fundamental problem" report, not a silent stop).

**Team runtime and state reconciliation**: `src/state/workflow-transition.ts` and `src/state/workflow-transition-reconcile.ts` implement a state machine with documented transition rules (see `docs/STATE_MODEL.md`). State has both per-mode files and a compatibility `skill-active-state.json` layer. Stale state is terminalized during reconciliation. The reconciliation sequence is ordered (complete source mode → sync compatibility → activate destination) to prevent resurrection of completed modes. Allowed transitions are documented in a table in `docs/STATE_MODEL.md` with explicit columns for source, destination, and result. Transitions not in the allowlist are denied with an error that tells the user to clear state first.

**Session history**: `src/session-history/` module tracks session activity. The HUD (`src/hud/`, `omx hud --watch`) provides real-time monitoring.

**Autoresearch bounded loop**: `src/autoresearch/runtime.ts` (45KB) implements a supervisor/candidate loop. The `missions/` directory contains mission bundles with `mission.md` (objective/scope/deliverable) and `sandbox.md` (evaluator contract and safety/operating rules). Each run records logs at `.omx/logs/autoresearch/<run-id>/manifest.json`, `candidate.json`, and `iteration-ledger.json`. The supervisor makes keep/discard/stop decisions per iteration. The `missions/README.md` notes that evaluators can be run directly via `node scripts/eval-<mission>.js`, decoupling the autoresearch runtime from evaluator scripts.

**What happens when things go wrong**: Documented partial coverage. The native hook mapping doc (`docs/codex-native-hooks.md`) is explicit about unresolved gaps: `ask-user-question`, `PostToolUseFailure`, non-Bash tool interception, and `SubagentStop` are "runtime-fallback" or "not-supported-yet" — meaning failures in those paths degrade to best-effort tmux/notify fallbacks. This is honest documentation rather than silent omission.

Notably, the native hook mapping doc uses a formal table with columns `OMC / OMX surface`, `Native Codex source`, `OMX runtime target`, `Status`, and `Notes`. The `Status` values form a vocabulary: `native`, `native-partial`, `runtime-fallback`, `not-supported-yet`. This explicit vocabulary for implementation completeness status is a pattern in itself — it makes gap tracking a first-class documentation concern rather than something that gets buried in TODO comments.

---

## 6. Anti-patterns and explicit non-goals

**Explicit non-goals from README**:
- "OMX does NOT replace Codex." Stated prominently in both the quick mental model section and the FAQ equivalent. This is backed by code structure: OMX has no model call code, no tool execution engine.
- `$team` is not the default onboarding path. README says: "Use the team runtime when you specifically need durable tmux/worktree coordination, not as the default way to begin using OMX."
- Native Windows is explicitly a "secondary path" and "less supported." README has a caution table at the top.
- `omx doctor` is NOT an end-to-end readiness check; it only checks install shape, not auth or model call readiness. This scope limitation is documented explicitly, not implied.

**Deprecated patterns with explicit markers**:
- `$web-clone` is "hard-deprecated" (`skills/ralph/SKILL.md` step 5: "`$web-clone` is hard-deprecated; Visual Ralph owns the migrated live-URL visual implementation use case").
- Legacy `.omx/prd.json` and `.omx/progress.txt` are compatibility surfaces in a one-release-cycle compatibility window, with explicit migration code in `src/ralph/persistence.ts`.
- The `OMX_LAUNCH_POLICY` environment variable is a runtime preference; config-file setting is intentionally not added ("This iteration only adds CLI and environment controls; it intentionally does not add a config-file setting" — README).

**Patterns warned against in CONTRIBUTING.md**:
- "Do NOT stop to ASK 'SHOULD I PROCEED?' — PROCEED." (`templates/AGENTS.md`, first lines)
- Making changes to prompts without reading `docs/prompt-guidance-contract.md` first.
- Using a PR targeting `main` rather than `dev` without maintainer direction.
- Omitting "Document-refresh: not-needed | <reason>" from commits that change code without updating mapped docs.
- Deleting tests to make them pass — this is called out as an explicit anti-pattern in the ralph skill's execution policy.

**Design choices explicitly argued against**:
- Claiming completion without fresh verification evidence. The ralph skill has explicit `<Bad>` examples: "Claiming completion without verification: 'All the changes look good, the implementation should work correctly. Task complete.'" and marks "uses 'should' and 'look good'" as the failure signal.
- Sequential execution of independent tasks. Ralph's `<Bad>` examples show this anti-pattern and explain why.
- Generic pre-commit blocking hooks for document refresh. CONTRIBUTING.md says: "It does not add a generic CI failure, does not install a pre-commit framework, and must not hard-block `git commit` for document-refresh reasons."

**Potential gap between claim and code**: The README claims `omx doctor` verifies the install shape and then separately warns: "A green `omx doctor` means the install and local runtime wiring look sane. It does not prove that the active Codex profile can make an authenticated model call." This split is explicit and honest — the smoke test distinction is real and documented.

The CONTRIBUTING.md also describes a "document-refresh warning MVP" for agents. This is described as warning-only and advisory. The native hook mapping doc confirms the split: warnings fire at PreToolUse for staged commits and at Stop for final handoffs, but neither blocks committing. This is consistent with the code.

---

## 7. Anchoring caveats

The following caveats argue non-transfer. For each, specific patterns that are discounted and patterns that transfer despite the difference are noted.

### 7.1 oh-my-codex is a wrapper layer; the redesign target IS the agent

oh-my-codex does not own model inference, tool execution, the ReAct loop, or any execution sandbox. It owns the configuration, guidance, lifecycle hooks, and state that shape how Codex behaves. When oh-my-codex prompts say "Fire independent agent calls simultaneously", it is instructing the Codex model to do parallelism, not implementing parallelism itself.

**Discounts**: Patterns around MCP tool server design, skill/prompt installation mechanics, tmux team runtime implementation. These are plumbing for wrapping an external system; the redesign doesn't have an external system to wrap.

**Transfers despite the difference**: The conceptual separation of persistent procedure (skill/prompt files) from live session logic. The concept of "context snapshots" grounding execution before work starts. The state model design (per-mode files, session vs. root scope, transition rules). These are durable ideas independent of whether you are wrapping Codex or are the agent yourself.

### 7.2 oh-my-codex targets live interactive use; the redesign runs on a cron

oh-my-codex's main user interface is a human at a terminal typing `$ralph "..."` inside a Codex session. The four-step workflow (deep-interview → ralplan → ralph → team) assumes a human making sequential decisions at each stage.

**Discounts**: The `--interactive` flag on ralplan (human approval at plan review and execution choice stages). The `$deep-interview` Socratic interview pattern. The HUD monitoring surface. The tmux-based team runtime (tmux is not available in headless cron).

**Transfers despite the difference**: The pre-execution gate in ralplan — detecting underspecified requests and redirecting to planning before spinning up heavy multi-agent execution. The concept of requiring a grounded context snapshot before execution begins. The iteration-limited loop with explicit stop conditions. Evidence-backed completion (fresh test output, not "looks good"). These behavioral disciplines are valuable for autonomous cron agents as much as for interactive use.

### 7.3 oh-my-codex targets multiple LLM providers; the redesign is Anthropic Claude only

oh-my-codex explicitly supports GPT-5.4, GPT-5.4-mini, and has skills `$ask-claude` and `$ask-gemini` for cross-provider invocation. The prompt guidance contract is titled "GPT-5.4 Prompt Guidance Contract" and includes a "mini composition seam" for exact-model gating.

**Discounts**: The `gpt-5.4-mini` exact-model adaptation seam. Provider fallback logic. The ask-claude/ask-gemini skills as external consultation.

**Transfers despite the difference**: The prompt behavioral contract concept itself — four named behavioral patterns with regression tests. The idea of gating prompt changes behind a contract document with test coverage is provider-agnostic. The model-specific instruction adaptation seam, while currently GPT-focused, demonstrates the architectural idea of parameterized instruction composition by final resolved model.

### 7.4 oh-my-codex targets broad coding tasks; the redesign targets schema.org work specifically

oh-my-codex provides 39 skills and 30 role prompts covering frontend UI, ML Kaggle optimization, Bayesian optimization, security review, TDD, git operations, code review, etc. The task surface is as broad as software development generally.

**Discounts**: The need for broad role catalog (analyst, researcher, designer, etc.). The specialist agent tier system. The visual verdict scoring and web-clone use cases. The ML mission bundles.

**Transfers despite the difference**: The tiered agent model concept (LOW for simple lookups, STANDARD for normal work, THOROUGH for complex analysis). Even in a narrow domain, having tier-based effort allocation is useful. The skill composition pattern — having specialized workflows (deep-interview, ralplan, ralph, team) that can be combined — is applicable to narrow domains too. The autoresearch loop with mission bundles and evaluator contracts could apply to schema.org research dispatches.

### 7.5 oh-my-codex is a small/medium community project; the redesign has one human stakeholder

oh-my-codex has 2 core maintainers plus several contributors, a Discord, a changelog, multilingual README. It manages contributor onboarding concerns.

**Discounts**: The CONTRIBUTING.md PR template and onboarding setup. The multilingual documentation. The ambassador program. The plugin marketplace.

**Transfers despite the difference**: The document-refresh warning mechanism is motivated by a problem that applies equally to single-stakeholder projects: spec-driven changes where code evolves but docs don't. The pattern of having code changes warn when mapped documentation surfaces haven't been updated is valuable regardless of team size.

### 7.6 oh-my-codex is an interactive tool with user escape hatches; the redesign is fully autonomous

oh-my-codex provides explicit user override surfaces: `force:` and `!` prefixes to bypass the pre-execution gate, `--no-deslop` to skip the mandatory deslop pass, `--interactive` for human approval checkpoints in ralplan, the ability to `omx team shutdown --force --confirm-issues` to recover from stale state. There is always a human who can intervene.

**Discounts**: Any pattern that relies on the human as a safety valve. The `force:` bypass. The confirmation-dialog patterns. The "only ask when truly ambiguous or destructive" rules that assume a human will respond.

**Transfers despite the difference**: The named escape hatch pattern itself is interesting in the redesign context: having explicit named surfaces for human override (vs. silent bypass) means the system can be audited for when overrides were used. For an autonomous orchestrator, equivalent surfaces might be emergency-stop flags in state.json or issue-level override comments that are explicitly scoped and recorded.

### Summary of what transfers vs. what discounts

The strongest transfer candidates are conceptual: evidence-backed completion discipline, named workflow transitions with an allowlist, pre-execution gating against underspecified requests, context snapshots before work starts, iteration limits with named stop conditions, and the behavioral prompt contract pattern with test coverage. These all address problems that arise when an agent runs without a live human supervisor — exactly the redesign's context.

The weakest transfer candidates are implementation-specific: the tmux team runtime, the plugin installation mechanics, the interactive confirmation flows, and the GPT-5.4-specific model adaptation seam. These are either unavailable in the redesign's environment (no tmux on CI) or unnecessary given the redesign's narrower scope.

---

## Patterns observed in oh-my-codex

Listed without v2-relevance framing:

1. **Workflow as named keyword keywords with transition policy.** Modes (`deep-interview`, `ralplan`, `ralph`, `team`, `autopilot`, `ultrawork`, `ultraqa`) are named states with tracked transitions. Transitions are allowlisted or denied; rollback to planning from execution is forbidden. The transition table is maintained in code and documented in `docs/STATE_MODEL.md`.

2. **Context snapshot grounding before execution.** Before starting execution work (not just planning), OMX requires a context snapshot written to `.omx/context/{task-slug}-{timestamp}.md`. Minimum fields: task statement, desired outcome, known facts/evidence, constraints, unknowns, codebase touchpoints.

3. **Explicit stop conditions with named escalation paths.** Ralph specifies exact stop conditions (fundamental blocker, user cancel, 3+ recurrences of same issue). The autoresearch loop records keep/discard/stop decisions per iteration in an `iteration-ledger.json`.

4. **Evidence-backed completion, not assertion-backed.** The skill files use `<Bad>` examples explicitly marking "should work correctly" language as a failure. Fresh verification output (test run, build, lsp_diagnostics) is required before completion, plus an explicit architect sign-off at a minimum tier.

5. **Pre-execution gate for underspecified requests.** `$ralplan` has a word-count + signal-detection gate: prompts with ≤15 effective words and no concrete anchors (file paths, issue numbers, symbol names, etc.) are redirected to planning before execution starts. Escape prefix (`force:` or `!`) bypasses the gate.

6. **Iteration limits with explicit ceiling.** Ralph defaults to `max_iterations: 10`. Ralplan's review loop has `max 5 iterations`. These are not configurable via prompt text; they are structural stops.

7. **Behavioral prompt contract with test coverage.** `docs/prompt-guidance-contract.md` defines four named behavioral patterns, maps each to specific line numbers in specific prompt files, and enforces them via regression tests in `src/hooks/__tests__/prompt-guidance-*.test.ts`. Changes to prompt behavior must preserve these patterns or update the tests and contract document.

8. **Parallel delegation over sequential.** Skill files explicitly label sequential execution of independent tasks as `<Bad>` and simultaneous firing as `<Good>`. This is a stated design preference enforced by prompt instruction.

9. **Session scope and root scope for state, with explicit reconciliation.** Per-mode state files in session scope take precedence over root scope. A shared reconciliation helper (`src/state/workflow-transition-reconcile.ts`) ensures that syncing compatibility state doesn't resurrect completed source modes.

10. **File-backed migration with one-way compatibility windows.** When the state schema changes (e.g., `.omx/prd.json` → `.omx/plans/prd-<slug>.md`), migration code converts the legacy format on first access, writes a migration marker, and treats the legacy file as read-only for one release cycle. This is explicit rather than silent.

11. **Document-refresh warning integrated into commit path.** The native PreToolUse hook evaluates `git diff --cached` on inspectable `git commit` commands and emits a warning (not a block) when mapped product/test-contract code changes without a corresponding docs/spec refresh. Warning suppression requires explicit commit message acknowledgment.

12. **Autoresearch as a bounded supervisor/candidate loop.** `omx autoresearch <mission>` runs a structured loop: mission.md defines the objective and scope; sandbox.md defines the evaluator contract; each iteration generates a candidate, the supervisor evaluates it, and makes keep/discard/stop decisions. Logs are persisted under `.omx/logs/autoresearch/<run-id>/`.

13. **Deslop pass as a mandatory post-completion step.** After architect verification passes, Ralph runs `oh-my-codex:ai-slop-cleaner` on all files changed during the session, then re-runs regression tests. This is skippable with `--no-deslop` but is on by default.

14. **Wiki as a persistent, searchable local knowledge base.** `.omx/wiki/` stores markdown pages. SessionStart can inject bounded wiki context. `omx explore` has a wiki-first fallback path. Wiki data is markdown-first and search-first (not vector-based).

15. **MCP transport failure has a defined fallback.** If `omx_state` MCP tool calls fail (stdio transport closed), skill instructions specify: retry once via the CLI parity surface (`omx state write --input '<json>' --json`). If CLI also fails, fall back to `.omx/context`/`.omx/plans` file-backed artifacts. This degrades gracefully rather than halting.

16. **Explicit false-green detection for install readiness.** `omx doctor` verifies install shape but explicitly does not verify auth. The smoke test (`omx exec --skip-git-repo-check -C . "Reply with exactly OMX-EXEC-OK"`) is a separate required step. This distinction is maintained both in documentation and in the doctor tool's own output.

17. **Autonomy directive as the first line of the orchestration document.** `templates/AGENTS.md` opens with an all-caps block: "YOU ARE AN AUTONOMOUS CODING AGENT. EXECUTE TASKS TO COMPLETION WITHOUT ASKING FOR PERMISSION." This is not hedged or softened. The opening page of the orchestration guidance is a declaration of execution posture, not a capability inventory.

18. **Triage advisory routing without keyword activation.** When no `$skill` keyword matches a UserPromptSubmit event, the triage layer (`src/hooks/triage-heuristic.ts`, 13KB) classifies the prompt and may inject an advisory routing hint. Advisory destinations include `explore`, `executor`, `designer`, `researcher`. This classification does NOT activate a workflow mode — it adds a prompt-routing context hint only. Keywords remain the deterministic control surface; triage is advisory and suppressible with "no workflow", "just chat", or "plain answer".

19. **Coverage gate for critical modules.** CONTRIBUTING.md describes a team/state coverage gate enforced in CI: `npm run coverage:team-critical` checks that `dist/team/**` and `dist/state/**` meet minimum coverage thresholds (78% lines, 90% functions, 70% branches). Coverage is on the critical state-bearing modules specifically, not a blanket full-project coverage requirement.

20. **Compatibility layer kept separate from authoritative state.** `skill-active-state.json` is an explicitly labeled compatibility/visibility layer, not the authoritative state. The authoritative state is in per-mode files. The separation is named in `docs/STATE_MODEL.md` and enforced in the reconciliation code to prevent compatibility-layer writes from resurrecting completed source modes.

21. **Deterministic keyword detection over heuristic prompt parsing.** The keyword detector (`src/hooks/keyword-detector.ts`, 44KB) uses deterministic pattern matching for `$skill-name` tokens, not semantic classification. A `$ralph` in a prompt fires a specific state transition. The triage heuristic (advisory only) is a second-pass layer that only applies when deterministic matching finds no keyword. This hierarchy keeps the primary control surface predictable.

22. **Lore-format commit signing for agent-authored commits.** The native PreToolUse hook blocks inline `git commit` commands unless they include a required `Co-authored-by: OmX <omx@oh-my-codex.dev>` trailer and conform to the Lore commit format. Agent-authored commits are traceable and distinguishable from human commits. Team runtime worker commits are also expected to follow this format, with the leader responsible for rewriting worker auto-checkpoint commits into Lore-format final history.

---

## Size note

oh-my-codex is not a small wrapper. It is a medium-to-large system: 30 role prompts, 39 skills, TypeScript source across 30+ distinct modules (`src/adapt/`, `src/agents/`, `src/autoresearch/`, `src/catalog/`, `src/cli/`, `src/config/`, `src/document-refresh/`, `src/exec/`, `src/hooks/`, `src/hud/`, `src/mcp/`, `src/modes/`, `src/notifications/`, `src/openclaw/`, `src/pipeline/`, `src/planning/`, `src/question/`, `src/ralph/`, `src/ralplan/`, `src/runtime/`, `src/scripts/`, `src/session-history/`, `src/sidecar/`, `src/state/`, `src/subagents/`, `src/team/`, `src/types/`, `src/utils/`, `src/verification/`, `src/visual/`, `src/wiki/`), a Rust component (`crates/omx-explore-harness`), a plugin layout, mission bundles, and multi-language documentation. The surface area is commensurate with a production tool that has iterated toward increasing reliability and coverage for edge cases over multiple releases (currently v0.15.1). Many of the patterns described above emerged from real operational problems in earlier versions rather than being designed upfront.

The most operationally interesting single file in the repository is probably `src/hooks/keyword-detector.ts` (44KB), which contains the central integration point between user input and workflow mode activation. Second is `src/config/generator.ts` (43KB), which is the single assembly point for what the runtime looks like to the model. Third is `src/autoresearch/runtime.ts` (45KB), which implements the closest thing OMX has to a fully autonomous loop. None of these were read in full for this survey; they are flagged as areas requiring deeper reading if specific patterns from this report are being evaluated for transfer.

The project is at version 0.15.1 with 36 workflow skill sub-commands, active CI, and test coverage on critical state-bearing modules. It is clearly past the prototype stage and carries accumulated operational lessons in its design.
