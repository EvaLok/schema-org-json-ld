# oh-my-codex deeper read — cycle 63

**Date:** 2026-05-08
**Dispatch:** [EvaLok/schema-org-json-ld#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
**Supersedes:** cycle-26 high-level survey (PR #2784, closed-without-merge, branch `copilot/redesign-research-phase-1-survey`, commit `f291ec05`)
**Source repository:** https://github.com/Yeachan-Heo/oh-my-codex (commit `d1863f72d303857e331865a863ed7f057dbd2cf6`)

Code-level deep read to ground any redesign work that adapts OMX patterns. All claims below are cited to specific file paths and, where load-bearing, include verbatim quotes. This document supersedes cycle-26 notes where the two conflict; see §7 for explicit corrections.

---

## Table of Contents

1. [Three-File Deep Dive](#1-three-file-deep-dive)
2. [State Model and Transitions](#2-state-model-and-transitions)
3. [Hooks Lifecycle — End-to-End Flow](#3-hooks-lifecycle--end-to-end-flow)
4. [MCP Servers — Tool Surface](#4-mcp-servers--tool-surface)
5. [Rust Crate Ecosystem — omx-sparkshell](#5-rust-crate-ecosystem--omx-sparkshell)
6. [Prompt-Guidance Contract System](#6-prompt-guidance-contract-system)
7. [Cycle-26 Pattern Confirmation and Corrections](#7-cycle-26-pattern-confirmation-and-corrections)
8. [New Code-Level Patterns Not in Cycle 26](#8-new-code-level-patterns-not-in-cycle-26)
9. [Anchoring Caveats and Known Gaps](#9-anchoring-caveats-and-known-gaps)

---

## 1. Three-File Deep Dive

### 1.1 `src/hooks/keyword-detector.ts` (45.5 KB)

#### Purpose

Converts raw user prompt text into a typed list of activated workflow skills, seeds per-mode state files, and emits routing context for the native hook output. It is the first and most deterministic routing surface in the system.

#### Stage 1: Korean IME Normalization

Before any pattern matching, the detector normalizes Korean IME typos into their ASCII equivalents:

```typescript
// src/hooks/keyword-detector.ts ~line 48
const KOREAN_IME_MAP: Record<string, string> = {
  ㅕㅣㅈ: 'ulw',
  // ...
};
```

This means a user who types `ㅕㅣㅈ` (finger position for `ulw` on a Korean keyboard) correctly activates the `ultrawork` skill. The normalization runs on the raw input string before any other matching.

#### Stage 2: Explicit `$skill` Parsing

After normalization, the detector looks for explicit `$`-prefixed tokens:

```typescript
// ~line 160
const EXPLICIT_SKILL_RE = /(?:^|[^\w])\$(?:(?:oh-my-codex:)?([a-z][a-z0-9-]*))/g;
```

Parsing is left-to-right. Consecutive `$skill` tokens are collected as an ordered list. Parsing stops when a non-whitespace gap is encountered between tokens — so `$ralplan $team ship this` yields `['ralplan', 'team']` but `$ralplan hello $team` yields only `['ralplan']`.

**Anti-confusion guard**: If any `$`-prefixed token is present but none matches a registry entry, the detector returns an empty list rather than falling through to implicit matching. This prevents `$mytypo` from accidentally triggering implicit keyword logic.

#### Stage 3: Implicit Matching (KEYWORD_MAP)

If no explicit `$` tokens were found, the detector falls back to implicit matching against a precompiled `KEYWORD_MAP`:

```typescript
// ~line 260
const KEYWORD_MAP: Map<string, RegExp> = new Map(
  KEYWORD_TRIGGER_DEFINITIONS.map(def => [def.skill, keywordToPattern(def.keywords)])
);
```

`keywordToPattern()` wraps each keyword with word-boundary anchors. All skills with matching patterns are returned — not just the first match. This means a single prompt like "don't stop and also cancel" could theoretically match both `ralph` and `cancel` simultaneously.

#### Stage 4: Intent Gate for 7 Special Keywords

Seven skills require a secondary intent pattern check:

```
ralph, team, stop, abort, parallel, autoresearch, ultragoal
```

These are called `KEYWORDS_REQUIRING_INTENT`. After the keyword matches, the detector runs each match against `KEYWORD_INTENT_PATTERNS[skill]` — a set of positive-intent patterns. If intent cannot be confirmed, the skill match is dropped.

This is the mechanism that prevents false positives on conversational uses of, e.g., "stop" (which would otherwise always trigger `$cancel`).

#### Multi-Keyword Arbitration

`detectKeywords()` returns ALL matches. The caller, `resolveRequestedWorkflowSkills()`, then splits them:

- **Planning-like skills**: `deep-interview`, `ralplan`, `autoresearch`
- **Execution-like skills**: `team`, `ralph`, `autopilot`, `ultrawork`, `ultraqa`

When both types appear in the same prompt, only the first planning-like skill is returned as `requestedSkills`; execution-like skills are placed in `deferredSkills`. This enforces planning-before-execution at the detection layer.

#### Ralplan Gate

`applyRalplanGate()` and `isUnderspecifiedForExecution()` redirect execution keywords to `$ralplan` when the prompt is vague (≤ 15 words, no well-specified signals). Both functions are **advisory only** — they can be bypassed with a `force:` prefix, a `!` prefix, `planningComplete` status, or approved followup shortcuts.

#### State Seeding

`recordSkillActivation()` writes the initial per-mode state file at `.omx/state/<mode>-state.json` (or session-scoped). The written fields include `phase`, `activated_at`, `session_id`, `thread_id`, `turn_id`, and skill-specific fields like `iteration` count for ralph/autopilot or `handoff_artifacts` for autopilot.

`persistStatefulSkillSeedState()` handles the actual file creation via the shared `writeAtomicFile` primitive.

#### Continuation Detection

`shouldReusePreviousSkillForContinuation()` matches phrases like "keep going", "continue", "resume" and re-activates the previous skill without re-running the full detection pipeline. This is checked before the main 4-stage pipeline runs.

#### Code Block Protection Gap

There is no explicit code-block exclusion. The detector operates on raw text. A `$ralph` inside a fenced code block **would** trigger skill activation. This is an architectural gap worth noting for any port.

---

### 1.2 `src/config/generator.ts` (~50 KB)

#### Purpose

Generates and idempotently upserts the Codex `config.toml` file. It is a pure TOML merge/upsert engine — no side effects beyond writing config.toml.

#### Developer Instructions

There is one monolithic constant:

```typescript
// src/config/generator.ts ~line 60
const OMX_DEVELOPER_INSTRUCTIONS = `...`;
const OMX_PLUGIN_DEVELOPER_INSTRUCTIONS = `...`;
```

Neither is model-specific; the same string is used regardless of which model is selected. Plugin mode uses the second constant. Neither is composed from smaller pieces at runtime.

#### Idempotency: Strip-and-Rebuild

Before writing, the generator strips existing OMX blocks:

```typescript
// ~line 580
function stripExistingOmxBlocks(toml: string): string {
  // finds: # oh-my-codex (OMX) Configuration ... # End oh-my-codex
  // preserves customized [tui] sections via extractCustomizedTuiSectionsFromOmxBlocks()
}
```

This guarantees that running setup a second time does not accumulate duplicate blocks. Customized `[tui]` sections are extracted before stripping and reinjected after rebuild.

#### Output Structure Order

The generated config.toml always writes blocks in this deterministic order:

1. Top-level keys: `notify`, `model_reasoning_effort`, `developer_instructions`, optionally `model` (if new or override)
2. Seeded behavioral defaults (`model_context_window` = 250000, `model_auto_compact_token_limit` = 200000) — **only** when the selected model equals `DEFAULT_SETUP_MODEL`
3. `[features]`: `multi_agent`, `child_agents_md`, `codex_hooks`, `goals`
4. `[shell_environment_policy.set]`: `USE_OMX_EXPLORE_CMD=1`
5. `[agents]`: `max_threads=6`, `max_depth=2`
6. User's pre-existing sections (preserved, unchanged)
7. OMX tables block: MCP server registrations
8. `[tui]` with `status_line`

#### Model Context Window Seeding

```typescript
// ~line 420
if (model === DEFAULT_SETUP_MODEL) {
  lines.push(`model_context_window = 250000`);
  lines.push(`model_auto_compact_token_limit = 200000`);
}
```

`getModelContextRecommendation()` returns `null` for non-default models — meaning non-default model users do not get seeded window values. Model upgrade = change `DEFAULT_FRONTIER_MODEL` in `./models.js`; the generator adapts automatically.

#### AGENTS.md Relationship

The generator does **not** write `AGENTS.md`. That file is managed by `src/hooks/agents-overlay.ts`. The generator only owns `config.toml`. This is a clean separation of concerns.

#### `buildMergedConfig()` and `mergeConfig()`

`buildMergedConfig()` assembles the full config string from scratch. `mergeConfig()` calls `stripExistingOmxBlocks()`, then splices the new OMX block into the existing user config, then calls `buildMergedConfig()`. The function is deterministic given the same inputs.

---

### 1.3 `src/autoresearch/runtime.ts` (45.6 KB)

#### Purpose

Provides orchestration primitives for the autoresearch loop. The runtime is **not** a self-contained supervisor loop — the actual iteration loop is driven externally (by the calling MCP tool or CLI command). The model is the "supervisor" that calls `processAutoresearchCandidate()` after each candidate session completes.

#### `decideAutoresearchOutcome()` — Fully Deterministic

This function is the decision engine. It is pure logic — no LLM calls, no I/O:

```typescript
// ~line 280
function decideAutoresearchOutcome(candidate, evaluatorResult, keepPolicy): AutoresearchDecision {
  if (candidate.status === 'abort') return { decision: 'abort', ... };
  if (candidate.status === 'noop') return { decision: 'noop', ... };
  if (candidate.status === 'interrupted') return { decision: 'interrupted', ... };
  if (evaluatorResult?.error) return { decision: 'discard', ... };
  if (evaluatorResult?.pass === false) return { decision: 'discard', ... };
  if (keepPolicy === 'pass_only' && evaluatorResult?.pass === true) return { decision: 'keep', ... };
  if (keepPolicy === 'score_improvement' && noComparableScore) return { decision: 'ambiguous', ... };
  if (scoreImproved) return { decision: 'keep', ... };
  return { decision: 'discard', ... };
}
```

The decision tree has 8 terminal branches — all deterministic given the same inputs. There is no prompt-based judgment here.

#### Iteration Ledger Schema

`.omx/autoresearch/<run_id>/iteration-ledger.json`:

```typescript
{
  schema_version: 1,
  run_id: string,
  created_at: string,
  updated_at: string,
  entries: AutoresearchLedgerEntry[]
}
```

Each entry:

```typescript
{
  iteration: number,
  kind: 'baseline' | 'iteration',
  decision: 'keep' | 'discard' | 'abort' | 'noop' | 'interrupted' | 'ambiguous',
  decision_reason: string,
  candidate_status: string,
  base_commit: string,
  candidate_commit: string | null,
  kept_commit: string | null,
  keep_policy: 'pass_only' | 'score_improvement',
  evaluator: {
    command: string,
    ran_at: string,
    status: 'passed' | 'failed' | 'error',
    pass?: boolean,
    score?: number,
    exit_code?: number,
    stdout?: string,
    stderr?: string
  } | null,
  created_at: string,
  notes: string[],
  description: string
}
```

The ledger is **append-only** (entries array grows). Manifest is overwritten on each update. Results TSV is append-only.

#### Write Semantics: Atomicity Gap

The ledger uses plain `writeFile` (no atomic rename). This contrasts with `src/state/operations.ts` which uses `writeAtomicFile()` (write to `.tmp.<pid>.<ts>.<random>`, then `rename()`). The ledger is therefore susceptible to partial write corruption on crash — an architectural gap absent from the state layer.

#### Candidate Artifact Contract

The worker session (autoresearch worker agent) must write a JSON file to `candidate.json` with this schema:

```typescript
{
  status: 'candidate' | 'noop' | 'abort' | 'interrupted',
  candidate_commit: string | null,  // required when status='candidate'
  base_commit: string,
  description: string,
  notes: string[],
  created_at: string
}
```

The supervisor (`processAutoresearchCandidate()`) validates:
- `base_commit` must resolve in git and match `last_kept_commit`
- For `status='candidate'`: `candidate_commit` must resolve AND match the worktree HEAD

This is the contract boundary between the supervisor and the worker agent.

#### `buildAutoresearchInstructions()` — Prompt Construction

This function constructs the task prompt for the worker session. It includes:
- Mission objective from `AutoresearchMissionContract`
- Baseline commit information
- Evaluator command spec
- Iteration-specific context
- Output format requirements

The instructions are constructed entirely from code — no external template files.

#### Termination Responsibility

`countTrailingAutoresearchNoops()` counts trailing noop entries in the ledger. The runtime exports this utility but does not call it itself. Callers are expected to use this to decide when to stop. Bounded termination is **caller responsibility**, not enforced within runtime.ts.

#### Worktree Management

Uses `git worktree` at a separate path. Key utilities:
- `assertResetSafeWorktree()` — validates cleanliness
- `resetToLastKeptCommit()` — `git reset --hard` to `last_kept_commit`
- `AUTORESEARCH_WORKTREE_EXCLUDES = ['results.tsv', 'run.log', 'node_modules', '.omx/']` — allowed-dirty list

---

## 2. State Model and Transitions

### 2.1 Authoritative State Files

Per-mode state files are the single source of truth:

```
.omx/state/<mode>-state.json              (root scope)
.omx/state/sessions/<session_id>/<mode>-state.json  (session scope)
```

Examples: `ralph-state.json`, `ralplan-state.json`, `team-state.json`.

`skill-active-state.json` is a **compatibility/visibility layer**, not the authority. From `docs/STATE_MODEL.md`:

> `skill-active-state.json` is still used as a compatibility surface for hooks/HUD/native messaging, but transition reconciliation should be driven from the shared transition/reconciliation helpers rather than re-deriving semantics ad hoc.

#### Read Precedence (3-tier)

1. Explicit session scope (if provided)
2. Current session scope
3. Root scope fallback

If root and session disagree, **session wins** for the active execution context.

### 2.2 Terminal Lifecycle Vocabulary

Canonical user-facing lifecycle outcomes (`docs/STATE_MODEL.md`):
- `finished` / `blocked` / `failed` / `userinterlude` / `askuserQuestion`

These are **separate** from mode `current_phase` values. The `state_write` MCP tool exposes both `lifecycle_outcome` (canonical) and `terminal_outcome` (legacy alias) plus the older `run_outcome: 'continue' | 'finish' | 'blocked_on_user' | 'failed' | 'cancelled'`.

Read precedence for lifecycle interpretation:
1. `lifecycle_outcome` (canonical metadata field)
2. `run_outcome` (legacy)
3. Inference from `current_phase` + question metadata

### 2.3 Transition State Machine

`src/state/workflow-transition.ts` defines:

**`AUTO_COMPLETE_TRANSITIONS`** (Set of `from|to` strings):
```
deep-interview → ralplan
ralplan → team, ralph, autopilot, autoresearch
autopilot → ralplan  (review-loopback only)
```

**`ALLOWED_OVERLAP_PAIRS`** (Set of `mode1|mode2` strings):
```
ralph | team   (bidirectional)
ultrawork | *  (ultrawork overlaps with everything)
```

**Transition Decision Matrix**:

| From | To | Result |
|---|---|---|
| `deep-interview` | `ralplan` | auto-complete source, activate destination |
| `ralplan` | `team` / `ralph` / `autopilot` | auto-complete source, activate destination |
| `autopilot` | `ralplan` | auto-complete source (review-driven loopback) |
| `team` | `ralph` | overlap (both stay active) |
| `ralph` | `team` | overlap (both stay active) |
| any tracked mode | `ultrawork` | overlap |
| any execution-like | any planning-like | **denied** (rollback blocked) |
| non-allowlisted | conflicting mode | **denied** |

`evaluateWorkflowTransition()` returns a `WorkflowTransitionDecision`:
```typescript
{
  allowed: boolean,
  kind: 'allow' | 'overlap' | 'auto-complete' | 'deny',
  resultingModes: string[],
  autoCompleteModes: string[]
}
```

### 2.4 Reconciliation Sequence

From `docs/STATE_MODEL.md` (load-bearing ordering):

> 1. decide outcome
> 2. complete source mode(s) with audit metadata
> 3. sync compatibility `skill-active` state
> 4. activate destination mode(s)
> 5. return transition message for rendering

> This ordering matters because syncing too early can resurrect a mode that was just auto-completed.

### 2.5 Write Atomicity

`src/state/operations.ts`: `writeAtomicFile()` writes to a temp path `${path}.tmp.${pid}.${timestamp}.${random}` then calls POSIX `rename()`. On rename failure, the temp file is unlinked and the error rethrown.

In-process serialization: `withStateWriteLock()` maintains a per-path Promise-chain queue — serial within-process writes, not OS-level file locks.

### 2.6 Audit Fields on Auto-Complete

When a source mode is auto-completed, its state file must record:
- `active: false`
- `current_phase: 'completed'`
- `completed_at` timestamp
- `auto_completed_reason` or equivalent
- `completion_note` or equivalent
- `transition_target_mode` (when applicable)

---

## 3. Hooks Lifecycle — End-to-End Flow

### 3.1 UserPromptSubmit Path

From `docs/STATE_MODEL.md` (Prompt-submit flow section):

```
UserPromptSubmit
  → detectKeywords()
  → ordered explicit skill list
  → recordSkillActivation()
  → shared reconciliation helper
  → final active skills
  → buildAdditionalContextMessage()
  → native hook output
```

The native hook (`src/scripts/codex-native-hook.ts`) is the routing entry point. It calls `detectKeywords()` and then routes to `recordSkillActivation()`.

### 3.2 Session Lifecycle (`src/hooks/session.ts`)

Session identity is PID-based on Linux (`/proc/<pid>/...`). Key operations:
- `startSession()` — creates session directory, writes session metadata
- `endSession()` — writes `ended_at` to session metadata
- `isSessionStale()` — checks process liveness via `/proc` filesystem  
- `reconcileSession()` — terminates stale sessions detected at startup

The staleness check is Linux-specific: it reads `/proc/<pid>/status` to verify the PID is still live. On non-Linux systems, the check falls back to a weaker heuristic.

### 3.3 Triage Heuristic (`src/hooks/triage-heuristic.ts`)

An 11-rule pure classifier — synchronous, no I/O, no state reads. It runs only when no keyword matches. Output is `TriageLane`:

| Lane | Role | Trigger |
|---|---|---|
| `PASS` | — | Empty input |
| `PASS` | — | Trivial acknowledgment |
| `PASS` | — | Opt-out phrase present |
| `LIGHT` | `explore` | Question pattern |
| `LIGHT` | `executor` | Anchored edit ≤ 15 words |
| `LIGHT` | `explore` | Local lookup |
| `HEAVY` | — | Implementation + research signals |
| `LIGHT` | `researcher` | External docs lookup |
| `HEAVY` | — | Structural redesign |
| `LIGHT` | `designer` | Visual styling |
| `HEAVY` | — | Long imperative > 5 words (fallback) |

The output is **advisory only**. It never activates a workflow skill, never writes state, and never blocks execution.

### 3.4 Continuation Shortcut

Before entering the main 4-stage detection pipeline, `keyword-detector.ts` checks `shouldReusePreviousSkillForContinuation()`. Patterns like "keep going", "continue", "resume" re-activate the last active skill without re-parsing. This is a fast path that bypasses stages 1-4 entirely.

### 3.5 Deep-Interview Lock

During an active `deep-interview` session, `DEEP_INTERVIEW_BLOCKED_APPROVAL_INPUTS` blocks standard approval shortcuts. Separately, `hadDeepInterviewLock` check redirects cancel intent when `deep-interview input_lock` is active — preventing accidental cancellation via "stop" during interview.

### 3.6 AGENTS.md Overlay Markers

`templates/AGENTS.md` contains marker-bounded overlay zones that runtime hooks populate:

```html
<!-- OMX:RUNTIME:START --> ... <!-- OMX:RUNTIME:END -->
<!-- OMX:TEAM:WORKER:START --> ... <!-- OMX:TEAM:WORKER:END -->
<!-- OMX:GUIDANCE:OPERATING:START --> ... <!-- OMX:GUIDANCE:OPERATING:END -->
<!-- OMX:GUIDANCE:SPECIALIST-ROUTING:START --> ... <!-- OMX:GUIDANCE:SPECIALIST-ROUTING:END -->
<!-- OMX:GUIDANCE:VERIFYSEQ:START --> ... <!-- OMX:GUIDANCE:VERIFYSEQ:END -->
<!-- OMX:MODELS:START --> ... <!-- OMX:MODELS:END -->
```

These are managed by `src/hooks/agents-overlay.ts`. The generator (`config/generator.ts`) never touches `AGENTS.md`.

---

## 4. MCP Servers — Tool Surface

### 4.1 Server Inventory

Five MCP servers under `src/mcp/`:

| Server | File | Size | Tool Count |
|---|---|---|---|
| `omx-state` | `state-server.ts` | 5.4 KB | 5 |
| `omx-wiki` | `wiki-server.ts` | 8.8 KB | 8 |
| `omx-trace` | `trace-server.ts` | 10.5 KB | 2 |
| `omx-memory` | `memory-server.ts` | 15.8 KB | (not read) |
| `omx-code-intel` | `code-intel-server.ts` | (not read) | (not read) |

All servers use `autoStartStdioMcpServer()` from `./bootstrap.js` for uniform stdio transport lifecycle.

### 4.2 `omx-state` — State Management

Tools: `state_read`, `state_write`, `state_clear`, `state_list_active`, `state_get_status`.

Supported modes enum:
```typescript
["autopilot", "autoresearch", "team", "ralph", "ultrawork", "ultraqa", 
 "ralplan", "deep-interview", "skill-active"]
```

All tools delegate to `executeStateOperation()` from `src/state/operations.ts`. The server is a thin MCP wrapper with no additional logic.

**Legacy team tools are hard-deprecated** in this server:
```typescript
// state-server.ts
if (TEAM_COMM_TOOL_NAMES.has(name)) {
  return { content: [...], isError: true };
  // error: "MCP tool ${name} is hard-deprecated. Team mutations now require CLI interop."
}
```

The `state_write` tool exposes `lifecycle_outcome` (canonical) and `terminal_outcome` (legacy alias) alongside the older `run_outcome` enum — reflecting the in-flight migration described in `docs/STATE_MODEL.md`.

### 4.3 `omx-wiki` — Knowledge Management

Tools: `wiki_ingest`, `wiki_query`, `wiki_lint`, `wiki_add`, `wiki_list`, `wiki_read`, `wiki_delete`, `wiki_refresh`.

Categories: `architecture`, `decision`, `pattern`, `debugging`, `environment`, `session-log`, `reference`, `convention`.

Key design choices:
- `wiki_add` **rejects overwrites** — use `wiki_ingest` to merge into an existing page
- `wiki_refresh` detects legacy fallback mode (`isLegacyWikiFallbackActive()`) and returns a read-only warning:
  > "Legacy .omx/wiki fallback is read-only; copy selected pages into omx_wiki/ before refreshing canonical metadata."
- All mutations go through `withWikiLock()` for serialization
- `appendLog()` records every mutation to `.omx/wiki/log.jsonl` with operation type and pages affected

### 4.4 `omx-trace` — Debugging Timeline

Tools: `trace_timeline`, `trace_summary`.

Data source: `.omx/logs/turns-*.jsonl` files written by the notify hook. Entries are `TraceEntry` objects with `timestamp`, `type`, `thread_id`, `turn_id`, `input_preview`, `output_preview`.

`trace_timeline` merges two event streams:
1. Agent turn entries from JSONL files
2. Mode start/end events reconstructed from reading all `<mode>-state.json` files via `listModeStateFilesWithScopePreference()`

The merged timeline is sorted by timestamp. The `filter` parameter allows `'all'`, `'turns'`, or `'modes'`.

`trace_summary` reports:
```typescript
{
  turns: { total, byType, firstAt, lastAt, durationMs, durationFormatted },
  modes: { [mode]: { starts, ends } },
  metrics: { ... }  // from .omx/metrics.json
}
```

`keepLastEntries()` implements an in-place insertion sort to maintain the top-N most recent entries without allocating a full array — a deliberate memory optimization for large log files.

---

## 5. Rust Crate Ecosystem — omx-sparkshell

### 5.1 Crate Inventory (Not 1 — 5 Crates)

The `crates/` directory contains **5 crates**, not 1 as cycle-26 noted:

| Crate | Purpose |
|---|---|
| `omx-sparkshell` | Output summarization via codex exec |
| `omx-explore` | Repo exploration routing |
| `omx-mux` | tmux pane capture helpers |
| `omx-runtime-core` | Shared runtime types |
| `omx-runtime` | Runtime orchestration |

`omx-sparkshell` depends only on `omx-mux`. Its `Cargo.toml`:
```toml
[dependencies]
omx-mux = { path = "../omx-mux" }
```

No external crate dependencies — omx-mux is the only non-stdlib dependency.

### 5.2 `omx-sparkshell` Architecture

`src/main.rs` defines two input modes:

```rust
enum SparkShellInput {
    Command(Vec<String>),
    TmuxPane { pane_id: String, tail_lines: usize },
}
```

**Command mode**: Executes the provided argv directly via `execute_command()`, then checks output line count against `read_line_threshold()`. If `line_count <= threshold`, writes raw output to stdout/stderr. If above threshold, calls `summarize_output()`.

**TmuxPane mode**: Builds `tmux capture-pane` argv via `omx_mux::build_capture_pane_args()`, then applies the same raw-vs-summarize logic.

Tail lines: default 200, min 100, max 1000.

### 5.3 Codex Bridge (`src/codex_bridge.rs`)

This is the load-bearing file — where omx-sparkshell calls the Codex API.

**Model resolution** (3-tier with env var priority):
```rust
// codex_bridge.rs
pub const DEFAULT_SPARK_MODEL: &str = "gpt-5.3-codex-spark";
pub const DEFAULT_STANDARD_MODEL: &str = "gpt-5.4-mini";

pub fn resolve_model() -> String {
    env::var("OMX_SPARKSHELL_MODEL")           // highest priority
    .or(env::var("OMX_DEFAULT_SPARK_MODEL"))
    .or(env::var("OMX_SPARK_MODEL"))
    .unwrap_or(DEFAULT_SPARK_MODEL)
}
```

**Fallback model resolution**: `OMX_SPARKSHELL_FALLBACK_MODEL` → `OMX_DEFAULT_STANDARD_MODEL` → `gpt-5.4-mini`.

**Invocation**: `codex exec` is invoked as a subprocess via `Command::new("codex")`:

```rust
Command::new("codex")
    .arg("exec")
    .arg("--model").arg(model)
    .arg("--sandbox").arg("read-only")
    .arg("-c").arg("model_reasoning_effort=\"low\"")
    .arg("--skip-git-repo-check")
    .arg("--color").arg("never")
    .arg("-")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
```

The prompt is written to stdin; stdout is collected as the summary.

**Timeout**: Default 60 seconds (`DEFAULT_SUMMARY_TIMEOUT_MS = 60_000`). Configurable via `OMX_SPARKSHELL_SUMMARY_TIMEOUT_MS`. The timeout loop polls `child.try_wait()` every 25ms, then calls `child.kill()` on expiry.

**Retry on capacity errors**: If the primary model fails, `should_retry_with_fallback()` checks for signals in stderr:

```rust
["quota", "rate limit", "429", "unavailable", "not available", 
 "unknown model", "model not found", "no access", "capacity"]
```

If any match, the same prompt is retried with the fallback model. If fallback also fails, both error messages are included in the returned error.

**Optional instructions file**: `OMX_SPARKSHELL_MODEL_INSTRUCTIONS_FILE` injects `-c model_instructions_file="..."` into the codex exec call, allowing custom model instructions per environment.

### 5.4 `src/prompt.rs` — Summary Prompt Construction

`build_summary_prompt()` builds the exact prompt string sent to the model:

```
You summarize shell command output.
Return markdown bullets only. Allowed top-level sections: summary:, failures:, warnings:.
Do not suggest fixes, next steps, commands, or recommendations.
Keep the summary descriptive and grounded in the provided output.

Command: {command_line}
Command family: {family_key}
...
Exit code: {exit_code}

STDOUT total lines: {stdout_lines}
STDOUT: <<<STDOUT
{stdout}
>>>STDOUT

STDERR: <<<STDERR
{stderr}
>>>STDERR
```

Command family classification (`select_command_family()`) maps the executable basename to one of 11 families: `git`, `node-js`, `python`, `rust`, `go`, `ruby`, `java-kotlin`, `c-cpp`, `csharp`, `swift`, `generic-shell`.

Truncation: stdout and stderr are each independently truncated to `OMX_SPARKSHELL_SUMMARY_MAX_LINES` (default 400) and `OMX_SPARKSHELL_SUMMARY_MAX_BYTES` (default 24000). Truncation preserves head + tail with an elision marker.

### 5.5 Output Normalization (`normalize_summary()`)

The model's output is filtered through `normalize_summary()`, which **only allows three section headers**: `summary:`, `failures:`, `warnings:`. Any other section header (e.g., `next steps:`, `recommendations:`) is silently dropped. This is the contract between the model's output and what reaches the user's terminal.

```rust
fn normalize_summary(raw: &str) -> Option<String> {
    // collects only summary:, failures:, warnings: sections
    // returns None if no valid sections found
}
```

If `normalize_summary` returns `None` (no valid sections), the raw output is written instead.

---

## 6. Prompt-Guidance Contract System

### 6.1 Contract Definition

`src/hooks/prompt-guidance-contract.ts` defines a `GuidanceSurfaceContract`:

```typescript
interface GuidanceSurfaceContract {
  id: string;
  path: string;                  // relative to repo root
  requiredPatterns: RegExp[];    // all must match
}
```

Each contract specifies a file path and a list of regular expressions that MUST be present in that file. Tests in `src/hooks/__tests__/prompt-guidance-contract.test.ts` verify compliance.

### 6.2 Contract Arrays (9 Groups)

| Array | Count | Covers |
|---|---|---|
| `ROOT_TEMPLATE_CONTRACTS` | ~1 | `templates/AGENTS.md` |
| `CORE_ROLE_CONTRACTS` | ~3 | `executor`, `planner`, `verifier` prompts |
| `SCENARIO_ROLE_CONTRACTS` | ~4 | Scenario-specific role prompts |
| `WAVE_TWO_CONTRACTS` | 8 | Wave-two roles |
| `CATALOG_CONTRACTS` | 15 | Full agent catalog roles |
| `LEGACY_PROMPT_CONTRACTS` | ~3 | Legacy prompt files |
| `SPECIALIZED_PROMPT_CONTRACTS` | ~4 | Specialized prompts |
| `SKILL_CONTRACTS` | 8+1 | ralph, team, autopilot, ultrawork, ultraqa, ralplan, deep-interview, analyze + ultrawork extension |
| `PROMPT_REFACTOR_MARKER_CONTRACTS` | 4 pairs | Marker pair existence: `OMX:GUIDANCE:*:START/END` etc. |
| `PROMPT_REFACTOR_INVARIANT_CONTRACTS` | 9 | Cross-cutting invariants |

### 6.3 Marker Contract Pattern

`PROMPT_REFACTOR_MARKER_CONTRACTS` verify that each `OMX:GUIDANCE:*:START` marker has a corresponding `OMX:GUIDANCE:*:END` marker in the expected files. This mechanically prevents half-deleted or malformed overlay zones from surviving undetected.

### 6.4 Invariant Contracts

`PROMPT_REFACTOR_INVARIANT_CONTRACTS` (9 contracts) enforce cross-cutting structural requirements — for example, that the executor prompt still contains delegation rules, that the verification block exists in templates/AGENTS.md, etc. These are the regression shield for structural prompt drift.

### 6.5 Practical Impact

Adding a new `<!-- OMX:GUIDANCE:MYNEW:START -->` marker to AGENTS.md requires:
1. Adding the corresponding `END` marker
2. Adding both to `PROMPT_REFACTOR_MARKER_CONTRACTS`
3. Running the contract tests

Failing to do so causes `prompt-guidance-contract.test.ts` to fail in CI.

---

## 7. Cycle-26 Pattern Confirmation and Corrections

### 7.1 CORRECTIONS (cycle-26 was wrong)

**C1 — Rust crate count**  
Cycle 26 stated: "1 Rust crate (omx-sparkshell)".  
Actual: **5 crates** — `omx-explore`, `omx-mux`, `omx-runtime-core`, `omx-runtime`, `omx-sparkshell`.  
`omx-sparkshell` was correctly identified as the output summarizer; the others were not examined in cycle 26.

**C2 — Sparkshell model name**  
If cycle 26 assumed any specific model name, the current default primary model is `gpt-5.3-codex-spark` and fallback is `gpt-5.4-mini` (hardcoded in `codex_bridge.rs`). These are overridable via env vars.

**C3 — State file authority**  
Any prior assumption that `skill-active-state.json` is the authoritative state file is incorrect. It is a **compatibility/visibility layer**. Per-mode files (`<mode>-state.json`) are the authority.

### 7.2 CONFIRMED patterns

**C4 — keyword-detector.ts 4-stage pipeline**  
Confirmed exactly as described in cycle-26 notes: IME normalization → explicit `$skill` → implicit KEYWORD_MAP → intent gate.

**C5 — Planning-before-execution arbitration**  
`resolveRequestedWorkflowSkills()` correctly splits detection results into `requestedSkills` (planning) and `deferredSkills` (execution) when both types appear.

**C6 — Ralplan gate is advisory**  
The gate at `applyRalplanGate()` / `isUnderspecifiedForExecution()` is bypass-able. It does not enforce at the hook level.

**C7 — autoresearch runtime is orchestration primitives, not a supervisor loop**  
Confirmed: `runtime.ts` provides `prepareAutoresearchRuntime()`, `processAutoresearchCandidate()`, `finalizeRun()` etc. as primitives. The calling agent drives the loop.

**C8 — `decideAutoresearchOutcome()` is fully deterministic**  
Confirmed: no LLM calls, pure decision tree.

**C9 — generator.ts writes only config.toml**  
Confirmed: no AGENTS.md writes. AGENTS.md managed separately.

**C10 — `writeAtomicFile()` in state/operations.ts**  
Confirmed: POSIX atomic rename pattern with PID+timestamp+random temp suffix.

---

## 8. New Code-Level Patterns Not in Cycle 26

### 8.1 Codex Bridge Retry Pattern

`codex_bridge.rs` implements a retry pattern not previously documented:

1. Primary model attempt (`gpt-5.3-codex-spark` or env override)
2. If failure AND stderr contains a quota/rate/availability signal → retry with fallback model
3. If fallback also fails → return compound error with both failure messages
4. If primary succeeds but `normalize_summary` returns None → return error (not raw output)

This is load-bearing for reliability: sparkshell degrades gracefully to raw output passthrough rather than failing silently.

### 8.2 normalize_summary Allowlist

The model is instructed to produce sections `summary:`, `failures:`, `warnings:` — but `normalize_summary` enforces this at parse time regardless of what the model emits. Any section header not in the allowlist causes that content to be silently dropped. This is a model-output contract boundary.

### 8.3 Wiki Legacy Fallback Detection

`wiki-server.ts` checks `isLegacyWikiFallbackActive()` before `wiki_refresh`. When the old `.omx/wiki/` path exists instead of the canonical `omx_wiki/`, the server is read-only and emits a migration instruction. This is a forward-migration guard pattern.

### 8.4 Trace Server's Dual-Stream Timeline Merge

`trace-server.ts` merges two independently ordered streams (agent turn JSONL + mode state events) into a single sorted timeline. The `keepLastEntries()` function uses an in-place insertion sort (O(k) where k = window size) to maintain the top-N most-recent entries without allocating a full list. This is intentional for large log files.

### 8.5 docs/STATE_MODEL.md as Normative Reference

`docs/STATE_MODEL.md` was not read in cycle 26. It is a normative specification document, not just documentation. It defines:
- Which files are authoritative vs compatibility
- Terminal lifecycle vocabulary (5 canonical outcomes + legacy map)
- Reconciliation sequence (5-step ordering with rationale)
- Invariant list (must remain true unless intentionally changed)
- Practical guidance for stale-state debugging

Any work touching state transitions should read this document first.

### 8.6 Lore Commit Protocol in templates/AGENTS.md

`templates/AGENTS.md` includes a full `<lore_commit_protocol>` block defining a commit message trailer format:

```
<intent line: why the change was made, not what changed>

Constraint: <external constraint>
Rejected: <alternative> | <reason>
Confidence: <low|medium|high>
Scope-risk: <narrow|moderate|broad>
Directive: <forward-looking warning>
Tested: <what was verified>
Not-tested: <known gaps>
```

This is enforced by convention (not CI). `Rejected:` is specifically for alternatives future agents should not re-explore.

### 8.7 Child Agent Model Routing Guidance

`templates/AGENTS.md` `<child_agent_protocol>` contains a currently-live directive:

> Do not hardcode stale frontier-model overrides for Codex native child agents. If an explicit frontier override is necessary, use the current frontier default from `OMX_DEFAULT_FRONTIER_MODEL` / the repo model contract (currently `gpt-5.5`), not older values such as `gpt-5.2`.

The reference model is `gpt-5.5` at this commit. This is distinct from the sparkshell spark model (`gpt-5.3-codex-spark`).

### 8.8 `$skill` Explicit Chain Parsing: Consecutive-Only Rule

The explicit-skill chain parsing stops at the first non-whitespace gap. This is a stricter rule than cycle-26 may have implied:

```
$ralplan $team ship this   → ['ralplan', 'team']   (chain valid: $team immediately follows)
$ralplan hello $team       → ['ralplan']            (chain broken: 'hello' is a non-whitespace gap)
```

This is enforced by the regex `EXPLICIT_SKILL_RE` combined with left-to-right scan logic that breaks on non-whitespace between `$` tokens.

### 8.9 Autoresearch Ledger vs State Atomicity Gap

The autoresearch ledger (`iteration-ledger.json`) uses plain `writeFile`. The state layer uses `writeAtomicFile()`. This inconsistency means autoresearch run metadata is more vulnerable to partial writes on crash than workflow state. Any adaptation of the autoresearch pattern should decide whether to align atomicity.

---

## 9. Anchoring Caveats and Known Gaps

### 9.1 Commit Scope

All findings are grounded in commit `d1863f72d303857e331865a863ed7f057dbd2cf6`. The repository's default branch may have diverged. Nothing in this document should be treated as reflecting HEAD unless independently verified.

### 9.2 Files Not Read

The following files were listed but not read in this cycle:

| File / Directory | Size | Why Skipped |
|---|---|---|
| `src/mcp/memory-server.ts` | 15.8 KB | Not fetched — memory model not required for this cycle |
| `src/mcp/code-intel-server.ts` | (unknown) | Not fetched |
| `src/hooks/__tests__/` | 45 KB dir | Test directory listed but individual test files not read |
| `skills/` | 20.3 KB dir | Directory listing too large; individual skill SKILL.md files not read |
| `prompts/` | (unknown) | Not listed |
| `src/state/workflow-transition-reconcile.ts` | (unknown) | Referenced in docs but not read |
| `src/modes/base.ts` | (unknown) | Referenced in docs but not read |
| `crates/omx-explore/src/` | (unknown) | Directory listed, source not read |
| `crates/omx-mux/` | (unknown) | Not read (used by sparkshell but not examined) |
| `crates/omx-runtime/` | (unknown) | Not read |
| `crates/omx-runtime-core/` | (unknown) | Not read |
| `crates/omx-sparkshell/src/threshold.rs` | 2.2 KB | Not read (provides `read_line_threshold()`) |
| `crates/omx-sparkshell/src/exec.rs` | 2.8 KB | Not read (provides `execute_command()`) |
| `crates/omx-sparkshell/src/registry/` | (dir) | Not read |

### 9.3 Known Uncertainty: `src/hooks/__tests__/`

The `prompt-guidance-contract.test.ts` test file pattern and specific RegExp patterns for each contract array were inferred from `prompt-guidance-contract.ts` contract definitions but not verified by reading the test file itself. The test logic may differ from the inferred structure.

### 9.4 Known Uncertainty: `workflow-transition-reconcile.ts`

`docs/STATE_MODEL.md` references `src/state/workflow-transition-reconcile.ts` as a "shared transition reconciliation helper" that implements the 5-step reconciliation sequence. This file was not read. The reconciliation logic may be more complex than implied by the docs alone.

### 9.5 Known Uncertainty: `threshold.rs`

`crates/omx-sparkshell/src/threshold.rs` (2.2 KB) provides `read_line_threshold()` and `combined_visible_lines()`. The default threshold value was not confirmed from source — only that it controls the raw-vs-summarize branch in `main.rs`.

### 9.6 skills/ Directory Contents

The skills directory listing (20.3 KB, first 500 chars visible) shows at least `ai-slop-cleaner` as a first entry. The full list of ~18+ skills implied by `keyword-registry.ts` (39 entries across ~18 skills) was not independently verified against the `skills/` filesystem tree.

### 9.7 templates/AGENTS.md Was Truncated

The `templates/AGENTS.md` fetch was truncated at ~20 KB (the file is 24.6 KB). The final ~4.6 KB — which likely contains the `<execution_protocols>` tail, further command routing, and any auto-generated `<!-- OMX:MODELS:START --> ... <!-- OMX:MODELS:END -->` block — was not read.

---

*End of cycle-63 deep read. Document grounded at commit `d1863f72`. All code citations reference that commit. Corrections to cycle-26 are enumerated in §7.1.*
