# Cycle 75 — oh-my-claudecode first-pass survey (Yeachan-Heo/oh-my-claudecode)

Date: 2026-05-08
Scope: first-pass survey only (not deeper-read)
Primary source repo: https://github.com/Yeachan-Heo/oh-my-claudecode
Companion baseline: cycle-26 oh-my-codex 22-pattern list (issue #2847 comment 4383627221)

Method note:
- I treated docs as untrusted claims unless corroborated by implementation files.
- Every claim below is marked as one of: **Implementation-verified**, **Documentation-only**, or **Needs deeper read**.

---

## 1) Repo at a glance

### 1.1 Purpose, scope, and relationship framing

- README positions the project as “multi-agent orchestration for Claude Code” and explicitly points Codex users to `oh-my-codex` as the sibling experience. **Documentation-only** (`README.md:L12-L16`).
- Quick-start centers Claude Code plugin install and slash-command usage (not a standalone replacement shell). **Documentation-only** (`README.md:L54-L65`, `README.md:L83-L110`).
- The project advertises both in-session skill surface and terminal CLI surface, explicitly saying they are different runtimes. **Documentation-only** (`README.md:L105-L118`).

### 1.2 Top-level shape

Top-level root includes all of the following at once:
- `agents/`, `skills/`, `hooks/`, `src/`, `bridge/`, `docs/`, `missions/`, `.claude-plugin/`, `.mcp.json`. **Implementation-verified** (repo root listing: `/tmp/omc-root-paths.txt:L1-L50`).

Notable absences at root:
- No `crates/` directory (contrast: oh-my-codex root includes `crates/`). **Implementation-verified** (`/tmp/omc-root-paths.txt:L1-L50` vs `/tmp/omx-root-paths.txt:L1-L26`).

### 1.3 Repo-level metadata snapshot (asymmetric scale check)

From GitHub repository metadata/API snapshot collected during this survey:
- `oh-my-claudecode`: 3076 commits, 109 contributors, 224 releases, latest `v4.13.6`.
- `oh-my-codex`: 2441 commits, 61 contributors, 98 releases, latest `v0.16.1`.
- Root item count: 50 for oh-my-claudecode vs 26 for oh-my-codex.

Evidence source: GitHub API metadata and release endpoints captured in-session (2026-05-08).
Classification: **Implementation-verified (repository metadata)**.

### 1.4 Immediate conclusion for “config skin vs harness”

At first-pass depth, this is **not** a pure config skin:
- substantial TypeScript source (`src/`),
- hook registration (`hooks/hooks.json`),
- dedicated MCP server bridge (`.mcp.json` + `bridge/mcp-server.cjs`),
- runtime orchestration files (team runtime, autoresearch runtime).

Evidence: `hooks/hooks.json:L1-L212`, `.mcp.json:L1-L8`, largest source files inventory from tree, and `src/team/runtime-v2.ts:L1-L17`, `src/autoresearch/runtime.ts:L945-L1024`.
Classification: **Implementation-verified**.

---

## 2) Architecture at survey depth

### 2.1 Declared architecture

Architecture doc claims four interlocking systems: Hooks, Skills, Agents, State. **Documentation-only** (`docs/ARCHITECTURE.md:L7-L8`, `docs/ARCHITECTURE.md:L42-L44`).

### 2.2 Load-bearing files (operationally large)

Largest non-test TypeScript files observed (first-pass sizing pass):
- `src/hooks/bridge.ts` (~105KB)
- `src/team/runtime-v2.ts` (~83KB)
- `src/installer/index.ts` (~71KB)
- `src/hooks/persistent-mode/index.ts` (~67KB)
- `src/autoresearch/runtime.ts` (~55KB)

Classification: **Implementation-verified (tree metadata)**.

### 2.3 Do analogues exist for oh-my-codex’s three operationally-large surfaces?

Question: keyword detector / generator / autoresearch runtime analogues.

- **Keyword detector analogue: present.**
  - Hook docs describe `keyword-detector` on `UserPromptSubmit` and conflict rules. **Documentation-only** (`docs/HOOKS.md:L215-L223`, `docs/HOOKS.md:L346-L352`, `docs/HOOKS.md:L411-L430`).
  - Hook registration file wires `keyword-detector.mjs` under `UserPromptSubmit`. **Implementation-verified** (`hooks/hooks.json:L4-L20`, `hooks/hooks.json:L10-L17`).
- **Generator/assembler analogue: present (adapted form).**
  - `src/installer/index.ts` is a large assembly point that writes Claude config surfaces (`agents`, `skills`, `hooks`, settings file paths). **Implementation-verified** (`src/installer/index.ts:L31-L39`, `src/installer/index.ts:L43-L47`).
  - This is not a single `generator.ts` by name; it is installer/runtime composition logic.
- **Autoresearch runtime analogue: present.**
  - `src/autoresearch/runtime.ts` exists at ~54KB and contains run manifest/state/deadline handling. **Implementation-verified** (`src/autoresearch/runtime.ts:L945-L973`, `src/autoresearch/runtime.ts:L995-L1024`, `src/autoresearch/runtime.ts:L1041-L1073`).

Answer: **Yes, all three analogues are present at survey depth (keyword detector + composition/installer + autoresearch runtime).**

### 2.4 Lifecycle/supervisor shape signals

- Team runtime v2 explicitly claims event-driven replacement of polling watchdog loop and done.json loop removal. **Implementation-verified** (`src/team/runtime-v2.ts:L2-L17`).
- Team phase names appear explicitly (`team-plan`, `team-prd`, `team-exec`, `team-verify`, `team-fix`) in hook bridge constants and docs pipeline strings. **Implementation-verified + Documentation-only** (`src/hooks/bridge.ts` grep evidence around constants; `README.md:L140-L143`; `docs/ARCHITECTURE.md:L250-L253`).

Assessment: explicit phased lifecycle is visible at first-pass depth.

---

## 3) Skills, prompts, and extension mechanisms

### 3.1 Skill packaging format

- Skills are shipped as `skills/<name>/SKILL.md` markdown files with YAML frontmatter template documented in `skills/AGENTS.md`. **Implementation-verified** (`skills/AGENTS.md:L79-L111`, `skills/AGENTS.md:L128-L135`).
- Invocation surface documented as `/oh-my-claudecode:<skill-name>`. **Documentation-only** (`skills/AGENTS.md:L10-L13`, `skills/AGENTS.md:L115-L124`).

### 3.2 Skill discovery/registration pattern

- `skills/AGENTS.md` says registry is auto-detected from frontmatter. **Documentation-only** (`skills/AGENTS.md:L128-L131`).
- Build script mentions `build-skill-bridge.mjs`, implying generated bridge wiring. **Implementation-verified** (`package.json:L41-L47`).

### 3.3 Slash-command surface

- README repeatedly documents slash usage (`/setup`, `/team`, `/ask`, `/autopilot`, `/oh-my-claudecode:autoresearch`). **Documentation-only** (`README.md:L83-L89`, `README.md:L312-L318`, `README.md:L356-L361`).
- Command expansion code reads markdown command templates from Claude config `commands/` directory. **Implementation-verified** (`src/commands/index.ts:L28-L30`, `src/commands/index.ts:L55-L72`, `src/commands/index.ts:L82-L99`).

### 3.4 Agents / role prompts

- Root has `agents/*.md` (19 files in current snapshot). **Implementation-verified** (agents listing from repo API; `README.md:L259-L260`; `docs/ARCHITECTURE.md:L52-L53`).
- This is structurally analogous to oh-my-codex `prompts/` role prompt surface, but directory naming differs.

### 3.5 MCP servers/tools extension shape

- Project-level `.mcp.json` registers an MCP server (`t`) via `${CLAUDE_PLUGIN_ROOT}/bridge/mcp-server.cjs`. **Implementation-verified** (`.mcp.json:L2-L6`).
- MCP module exports both external server configs (Exa, Context7, Playwright, filesystem, memory) and in-process custom tools server. **Implementation-verified** (`src/mcp/servers.ts:L4-L10`, `src/mcp/servers.ts:L84-L110`, `src/mcp/index.ts:L5-L23`).

Conclusion: extension shape is hybrid (skills + hooks + agent prompts + MCP + CLI).

---

## 4) Hooks and lifecycle integration with Claude Code

### 4.1 Hook events used

- Hooks docs enumerate 11 Claude Code lifecycle events and map scripts per event. **Documentation-only** (`docs/HOOKS.md:L90-L207`).
- `hooks/hooks.json` concretely registers: `UserPromptSubmit`, `SessionStart`, `PreToolUse`, `PermissionRequest`, `PostToolUse`, `PostToolUseFailure`, `SubagentStart`, `SubagentStop`, `PreCompact`, `Stop`, `SessionEnd`. **Implementation-verified** (`hooks/hooks.json:L4-L210`).

### 4.2 Canonical location question (`.claude/hooks/`?)

Observed shape is **repo `hooks/hooks.json` + installer deployment** rather than committed `.claude/hooks/` source tree:
- Installer defines `HOOKS_DIR = <CLAUDE_CONFIG_DIR>/hooks` and settings file under `<CLAUDE_CONFIG_DIR>/settings.json`. **Implementation-verified** (`src/installer/index.ts:L31-L39`).
- README uses `~/.claude/settings.json` for enabling native teams. **Documentation-only** (`README.md:L144-L151`).

### 4.3 Permission model handling

- Hook docs explicitly call out `PreToolUse` enforcement and `PermissionRequest` bash handling. **Documentation-only** (`docs/HOOKS.md:L118-L137`).
- Hook registration confirms `pre-tool-enforcer.mjs` + `permission-handler.mjs`. **Implementation-verified** (`hooks/hooks.json:L63-L85`).

### 4.4 Lifecycle augmentation beyond baseline Claude Code

Visible augmentations include:
- persistent continuation gate on `Stop` (`persistent-mode`),
- pre-compact state preservation,
- subagent tracking start/stop,
- session start/end persistence hooks.

Evidence: `docs/HOOKS.md:L187-L207`, `docs/HOOKS.md:L228-L237`, `hooks/hooks.json:L150-L209`.
Classification: **Implementation-verified for registration; behavior details mostly Documentation-only**.

---

## 5) State, memory, persistence

### 5.1 On-disk layout and scopes

Architecture doc claims `.omc/` with mode state, sessions, notepad, project-memory, prompts, logs. **Documentation-only** (`docs/ARCHITECTURE.md:L432-L459`).

It explicitly distinguishes:
- control plane under `.omc/state/**`,
- data plane artifacts under `.omc/plans`, `.omc/notepads`, `.omc/prompts`, etc. **Documentation-only** (`docs/ARCHITECTURE.md:L461-L470`).

### 5.2 Session and global scope

- Session-scoped path documented: `.omc/state/sessions/{sessionId}/`. **Documentation-only** (`docs/ARCHITECTURE.md:L545-L548`).
- Global state path documented: `~/.omc/state/{name}.json`. **Documentation-only** (`docs/ARCHITECTURE.md:L471-L474`).

### 5.3 State-machine/reconciliation analogues

- Team runtime v2 advertises event-driven operations and lifecycle transitions via API operations rather than polling done-file loops. **Implementation-verified** (`src/team/runtime-v2.ts:L2-L17`, plus team API command contract evidence in runtime-v2 excerpts).
- `state-tools` module includes explicit session-owned/legacy state clear/read logic and mode-scoped state operations. **Implementation-verified** (`src/tools/state-tools.ts` grep evidence lines on session-owned state clearing and mode operations).

### 5.4 Memory mechanism across sessions

- Notepad and project-memory are documented with lifecycle hook integration (`SessionStart`, `PostToolUse`, `PreCompact`). **Documentation-only** (`docs/ARCHITECTURE.md:L499-L542`; `docs/HOOKS.md:L297-L307`).

### 5.5 Autoresearch persistence/supervisor loop

- Runtime writes run manifest, ledger, evaluator artifacts, candidate files, mode-state, and optional runtime deadline (`max_runtime_ms`, `deadline_at`). **Implementation-verified** (`src/autoresearch/runtime.ts:L961-L973`, `src/autoresearch/runtime.ts:L995-L1034`, `src/autoresearch/runtime.ts:L1066-L1073`).

---

## 6) Documentation patterns (anti-patterns, deprecations, honesty gaps)

### 6.1 `<Bad>` style anti-pattern catalogs

- I did **not** find a prominent `<Bad>`/`<Good>` catalog in README/CONTRIBUTING equivalent to cycle-26 oh-my-codex patterning.
- `CONTRIBUTING.md` did not show obvious anti-pattern blocks in quick scan.

Classification: **Needs deeper read** (possible elsewhere in docs/templates, not surfaced in first-pass focal files).

### 6.2 Hard deprecations / compatibility markers

- README explicitly calls `omc autoresearch` a “hard-deprecated shim” and gives replacement workflow. **Documentation-only** (`README.md:L356-L367`).
- README marks `swarm` alias removed and asks migration to `/team`. **Documentation-only** (`README.md:L132-L133`, `README.md:L326-L327`).

### 6.3 Documentation-to-enforcement linkage

- Hooks docs claim permission enforcement; hooks registration confirms actual hook wiring.
- Settings schema doc explicitly states company-context behavior is prompt-level contract, not runtime enforcement.

Evidence:
- `docs/HOOKS.md:L118-L137` + `hooks/hooks.json:L63-L85` (linkage present).
- `docs/settings-schema.md:L40-L42` (explicit non-enforcement caveat).

Classification: mixed; some doc statements backed by registration, some intentionally advisory-only.

### 6.4 Prompt-guidance-contract analogue?

- No direct `prompt-guidance-contract.md` analogue was identified in first-pass focal set.
- Could exist under docs not sampled deeply (e.g., REFERENCE/FEATURES internals).

Classification: **Needs deeper read**.

---

## 7) Cross-reference to oh-my-codex (cycle-26 patterns 1–22)

Source for pattern names/baseline descriptions:
- Issue #2847 supplement comment `4383627221` (cycle-76 robustness fix for lens 7).

Legend:
- **PARALLEL** = same shape visible.
- **ADAPTED** = analogous shape but substrate-adjusted.
- **ABSENT** = not visible at first-pass.
- **NOT-COMPARABLE** = substrate mismatch made direct mapping invalid.
- **NEW** = only for patterns in section 8 (not used in this table).

| # | Cycle-26 pattern (short name) | Classification | Confidence | Evidence / note |
|---|---|---|---|---|
| 1 | Workflow as named keywords + transition policy | ADAPTED | Medium | Keyword priorities and explicit Team staging exist (`docs/HOOKS.md:L411-L430`; `README.md:L140-L143`) but full transition table parity not verified. |
| 2 | Context snapshot grounding pre-execution | ADAPTED | Low | `.omc` plans/notepads/session artifacts documented (`docs/ARCHITECTURE.md:L447-L459`, `L549-L561`). Need code-level proof of pre-exec snapshot contract. |
| 3 | Explicit stop conditions + escalation | PARALLEL | Medium | Persistent-mode stop handling, stale checks, cancel path (`docs/HOOKS.md:L228-L237`). |
| 4 | Evidence-backed completion | ADAPTED | Low | Verification protocol documented (`docs/ARCHITECTURE.md:L596-L610`), but enforcement path not fully traced in code at survey depth. |
| 5 | Pre-execution gate for underspecified requests | ABSENT | Low | Deep-interview exists, but no explicit underspec gate like cycle-26 word-count policy found in sampled files. |
| 6 | Iteration limits / explicit ceilings | ADAPTED | High | Autoresearch supports bounded runtime and explicit deadline metadata (`src/autoresearch/runtime.ts:L949-L973`, `L1066-L1073`). |
| 7 | Behavioral prompt contract + tests | ABSENT | Low | No direct contract file discovered in first-pass set; deeper read may find one elsewhere. |
| 8 | Parallel delegation preference | PARALLEL | High | Team + ultrawork + multi-agent framing throughout docs (`README.md:L230-L233`, `L248-L253`; `docs/ARCHITECTURE.md:L244-L253`). |
| 9 | Session/root state with reconciliation | PARALLEL | Medium | Session and global scopes documented; state-tools include session-owned cleanup logic (`docs/ARCHITECTURE.md:L471-L548`; `src/tools/state-tools.ts` survey lines). |
| 10 | File-backed migration compatibility windows | ADAPTED | Low | Dedicated migration doc exists (`docs/MIGRATION.md` present), but migration mechanics not inspected deeply. |
| 11 | Doc-refresh warning in commit path | ABSENT | Low | Not surfaced in focal files. |
| 12 | Autoresearch bounded supervisor loop | PARALLEL | High | Runtime manifests/ledger/deadline + candidate/evaluator artifacts (`src/autoresearch/runtime.ts:L961-L1034`, `L1041-L1084`). |
| 13 | Mandatory deslop post-completion | ADAPTED | Low | AI-slop-cleaner exists; mandatory post-completion enforcement not verified (`skills/AGENTS.md:L53-L56`; `README.md:L297-L298`). |
| 14 | Persistent local wiki knowledge base | ADAPTED | Medium | Wiki hooks/tools surfaced in registration and MCP exports; full wiki contract not deeply traced (`hooks/hooks.json:L37-L39`, `L165-L167`, `L205-L207`). |
| 15 | MCP transport failure fallback | NEEDS-DEEPER-READ (treated as ADAPTED-uncertain) | Low | MCP surfaces are substantial, but explicit fallback chain not confirmed at first-pass. |
| 16 | Install false-green detection | NEEDS-DEEPER-READ (treated as ADAPTED-uncertain) | Low | `/omc-doctor` exists in docs; explicit smoke-check semantics not verified in sampled code. |
| 17 | Autonomy directive first-line contract | ABSENT | Medium | README tone is strong, but no equivalent first-line all-caps autonomy directive found in sampled core docs. |
| 18 | Advisory triage without direct mode activation | ADAPTED | Medium | Keyword detector + explicit Team non-auto-detect policy suggests guarded routing (`docs/HOOKS.md:L456-L459`). Full triage module parity needs deeper read. |
| 19 | Critical-module coverage gates | ABSENT | Low | Not found in sampled docs/files. |
| 20 | Compatibility layer separate from authoritative state | ADAPTED | Medium | Control-plane/data-plane split and explicit mode state tooling suggest separation (`docs/ARCHITECTURE.md:L461-L477`; `src/tools/state-tools.ts` survey lines). |
| 21 | Deterministic keyword detection first, heuristics second | PARALLEL | Medium | Hooks doc describes deterministic keyword matching and conflict order (`docs/HOOKS.md:L346-L352`, `L411-L430`). |
| 22 | Commit-signing / lore-format guardrails | ABSENT | Low | No equivalent surfaced in first-pass focal read. |

Important caveat: row #15 and #16 are explicitly weak-confidence because full implementation tracing was out of first-pass scope.

### 7.1 Hypothesis checks (H1/H2/H3)

#### H1 parallel-architecture hypothesis

Result: **CONFIRMED at first-pass depth** (>=4/5 components).

Observed mapping:
1. keyword detector analogue: present (`hooks/hooks.json:L4-L20`; `docs/HOOKS.md:L215-L223`)
2. generator/composition analogue: installer/config assembly present (`src/installer/index.ts:L31-L47`)
3. autoresearch runtime analogue: present (`src/autoresearch/runtime.ts:L945-L1024`)
4. MCP layer: present (`.mcp.json:L2-L6`; `src/mcp/index.ts:L5-L23`)
5. Rust substrate-edge component: **not found** (no `crates/` at root: `/tmp/omc-root-paths.txt:L1-L50`)

Score: 4/5 present.

#### H2 substrate-specific patterns hypothesis

Result: **CONFIRMED** (>=3 substrate-specific patterns visible).

Visible substrate-specific patterns:
- Claude lifecycle event hooks with event names (`docs/HOOKS.md:L90-L207`; `hooks/hooks.json:L4-L210`)
- Slash-command first-class invocation (`README.md:L55-L65`, `L312-L318`)
- Claude config deployment surfaces (`src/installer/index.ts:L31-L39`) and settings example (`README.md:L144-L151`)
- MCP registration through `.mcp.json` and plugin root bridge (`.mcp.json:L2-L6`)

#### H3 substrate-investment asymmetry hypothesis

Result: **REFUTED on current-repo metrics** (opposite direction observed).

Current metadata snapshot indicates oh-my-claudecode is larger on multiple axes:
- more commits (3076 vs 2441)
- more contributors (109 vs 61)
- more releases (224 vs 98)
- larger root footprint (50 items vs 26)

Therefore first-pass evidence does not support “less mature/smaller than oh-my-codex.”

---

## 8) NEW patterns specific to oh-my-claudecode (relative to cycle-26 list)

These are surfaced as observation-only patterns, not design recommendations.

### NEW-1: Explicit dual-runtime split (in-session native Team vs terminal tmux Team)

- README distinguishes `/team` in-session runtime and `omc team` terminal runtime as intentionally different execution paths. **Documentation-only** (`README.md:L116-L118`, `README.md:L138-L143`, `README.md:L248-L250`).

### NEW-2: Event-driven Team runtime v2 replacing done-file polling

- Team runtime v2 header explicitly states replacement of polling watchdog and done.json loop with event-driven lifecycle operations. **Implementation-verified** (`src/team/runtime-v2.ts:L2-L17`).

### NEW-3: Plugin-root hook command indirection for portability

- Hook commands consistently execute through `node "$CLAUDE_PLUGIN_ROOT"/scripts/run.cjs ...`, which is a portable indirection pattern across events. **Implementation-verified** (`hooks/hooks.json:L10-L17`, repeated through file).

### NEW-4: In-process MCP “tools server” plus optional external MCP registry

- Internal tools server (`omcToolsServer`) and external server builders coexist (`createExaServer`, `createContext7Server`, etc.). **Implementation-verified** (`src/mcp/index.ts:L17-L23`, `src/mcp/servers.ts:L19-L110`).

### NEW-5: State-plane/data-plane split documented as architectural primitive

- Docs explicitly separate orchestration metadata from durable artifacts with examples, not just ad-hoc file naming. **Documentation-only** (`docs/ARCHITECTURE.md:L461-L470`).

### NEW-6: Explicit non-auto-detection of `team` keyword to avoid recursive spawn

- Hook docs state `team` must be explicit slash invocation (not magic keyword auto-detect), explicitly to avoid infinite spawning. **Documentation-only** (`docs/HOOKS.md:L456-L462`).

### NEW-7: Settings-schema doc that marks prompt-level contract vs runtime enforcement

- `settings-schema.md` explicitly states a config behavior remains prompt-level workflow contract, not runtime enforcement. **Documentation-only** (`docs/settings-schema.md:L40-L42`).

### NEW-8: Hybrid provider topology after Codex/Gemini MCP removal

- README says Codex/Gemini MCP servers were removed and CLI worker mode is preferred for those providers. **Documentation-only** (`README.md:L156-L166`).

---

## 8.5) Cluster framework anchoring (observation-only; no transferability recommendation)

Potential cluster touches from first-pass evidence:

- **Cluster A (cycle-internal phasing primitives):**
  - explicit Team stage pipeline (`team-plan → team-prd → team-exec → team-verify → team-fix`) and event-driven runtime v2 signal strong A-shape evidence at survey depth.
  - Evidence: `README.md:L140-L143`; `src/team/runtime-v2.ts:L2-L17`.

- **Cluster B (storage architecture / persistence stratification):**
  - explicit `.omc` multi-surface persistence (state, prompts, logs, notepads, project-memory) and control-plane/data-plane split.
  - Evidence: `docs/ARCHITECTURE.md:L432-L470`.

- **Cluster D (documentation honesty / aspirational vs implemented):**
  - positive: settings-schema explicitly marks non-enforcement boundary (`docs/settings-schema.md:L40-L42`).
  - unresolved: `<Bad>`-style anti-pattern catalogs are less obvious in surveyed focal docs; deeper read needed.

- **Cluster F (stratification axes):**
  - clear stratification by runtime surface (slash vs CLI), model tiers, and role lanes.
  - Evidence: `README.md:L105-L118`; `docs/ARCHITECTURE.md:L52-L55`, `L103-L115`.

- **Cluster I (harness-enforced policy boundaries):**
  - hooks expose permission handling and pre-tool enforcement, but first-pass does not yet establish how deep fail-closed behavior runs beyond documented flow.
  - Evidence: `docs/HOOKS.md:L126-L137`; `hooks/hooks.json:L63-L85`; `src/mcp/mcp-config.ts:L34-L38`, `L75-L78`.

Interpretation status: preliminary anchor candidates only; deeper-read required for sub-shape-level confirmation.

---

## 9) Anchoring caveats (non-transfer arguments + what still transfers)

### Caveat 1: Invocation model mismatch (interactive user-driven vs autonomous cron)

Difference:
- oh-my-claudecode is built around user-invoked slash/CLI flows (`README.md:L105-L118`), while redesign target runs autonomously.

Discounted patterns:
- Direct UX ergonomics around slash-command invocation and interactive setup wizarding.

Still transferable:
- Event-hook lifecycle segmentation and explicit mode-state persistence contracts.

### Caveat 2: Multi-provider optionality in OMC vs narrower orchestrator scope

Difference:
- OMC supports optional Codex/Gemini orchestration (`README.md:L512-L519`) and provider-specific paths.

Discounted patterns:
- Provider-specific advisor orchestration details (`/ccg`, codex/gemini worker topologies).

Still transferable:
- Separation of deterministic orchestration machinery from provider-specific adapters.

### Caveat 3: Team runtime sophistication may exceed single-stakeholder needs

Difference:
- OMC includes large Team runtime with worker orchestration semantics (`src/team/runtime-v2.ts:L2-L17`).

Discounted patterns:
- Full multi-worker tmux orchestration complexity.

Still transferable:
- Explicit phase boundaries, event-driven state transitions, and bounded supervision loops.

### Caveat 4: Documentation volume and release velocity differences

Difference:
- OMC has high release churn and large docs surface (224 releases in metadata snapshot; many docs files at root/docs).

Discounted patterns:
- Process overhead tied to broad public package maintenance and marketplace/plugin distribution.

Still transferable:
- Honest doc-to-enforcement boundary marking (`settings-schema.md:L40-L42`) and explicit deprecation signaling (`README.md:L356-L367`).

### Caveat 5: Claude substrate alignment is high (this increases comparability)

Difference:
- Unlike codex-oriented systems, this project is natively Claude Code-targeted.

Discounted patterns:
- Very little discount on hook/event semantics; these are directly substrate-relevant.

Still transferable:
- Hooks/events, session-state handling, permission-hook placement, and command-surface composition are directly comparable at substrate level.

---

## Open verification gaps queued for future deeper-read

1. Confirm whether doc-claimed enforcement paths (`post-tool-verifier`, `permission-handler`, project-memory lifecycle) are fully implemented fail-closed or partially advisory.
2. Trace keyword detector + skill injector implementation details in `src/hooks/bridge.ts` and underlying modules for conflict/precedence semantics.
3. Verify migration mechanics beyond docs (`docs/MIGRATION.md`) in code paths.
4. Verify whether any anti-pattern catalogs equivalent to cycle-26 `<Bad>` patterns exist outside first-pass sampled files.
5. Validate MCP failure-mode behavior and fallback semantics in real runtime paths (not just available modules).

