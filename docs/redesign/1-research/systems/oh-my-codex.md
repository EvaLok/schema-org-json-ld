# oh-my-codex (Yeachan-Heo/oh-my-codex — configuration layer + hook harness over Codex CLI; TypeScript + Rust)

[← back to Phase 1 index](../../1-research.md)

**Status: deep-dive (commit `d1863f72`).** Cycle-63 dispatch deeper-read
landed cycle 101 as PR
[#2874](https://github.com/EvaLok/schema-org-json-ld/pull/2874)
(originating issue
[#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833),
cycle-63 dispatch superseding closed cycle-26 dispatch
[#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782)).
Per the absorption convention (PR closed without merge; deliverable
preserved on never-merged branch), the deliverable lives on branch
`copilot/redesign-research-cycle-63-deeper-read` as
`docs/redesign/_notes/cycle-63-oh-my-codex-deeper-read.md` (913 lines;
9-lens structure: three-file deep dive [keyword-detector / generator /
autoresearch runtime], state model + transitions, hooks lifecycle, MCP
servers, Rust crate ecosystem, prompt-guidance contract system, cycle-26
confirmation/corrections, new code-level patterns not in cycle 26,
anchoring caveats and known gaps). This per-system file summarizes the
deliverable in the per-system shape and cites it as the primary evidence
base.

The cycle-26 high-level survey (cycle-33 stub layout, 168 lines) is
superseded by this deep-dive read. The earlier framing remains
historically accurate as a survey-surface read; cycle-63 explicitly
read the three operationally-largest files cycle-26 flagged as
not-read-in-full (`keyword-detector.ts`, `generator.ts`,
`autoresearch/runtime.ts`) plus state/hooks/MCP/Rust-crate code-level
coverage with file:line citations.

**Verification status (cycle 101):** load-bearing factual claims spot-checked
against the live `Yeachan-Heo/oh-my-codex` repo at commit
`d1863f72d303857e331865a863ed7f057dbd2cf6` via `gh api` before integration,
following the cycle-96/97/98/99/100 verification-discipline pattern.

| Class | Subject | Verdict |
|---|---|---|
| Commit ref | claim `d1863f72d303857e331865a863ed7f057dbd2cf6` | ✓ EXACT SHA (commit dated 2026-05-08T08:26:24Z, ~4 min before PR creation) |
| File size | `src/hooks/keyword-detector.ts` claim ~45.5 KB | ✓ EXACT (45590 bytes) |
| File size | `src/config/generator.ts` claim ~50 KB | ✓ EXACT (50358 bytes) |
| File size | `src/autoresearch/runtime.ts` claim ~45.6 KB | ✓ EXACT (45592 bytes) |
| File size | `src/mcp/state-server.ts` claim 5.4 KB | ✓ EXACT (5352 bytes) |
| File size | `src/mcp/wiki-server.ts` claim 8.8 KB | ✓ EXACT (8784 bytes) |
| File size | `src/mcp/trace-server.ts` claim 10.5 KB | ✓ EXACT (10505 bytes) |
| File size | `src/mcp/memory-server.ts` claim 15.8 KB | ✓ EXACT (15810 bytes) |
| File size | `templates/AGENTS.md` claim 24.6 KB | ✓ EXACT (24606 bytes) |
| File size | `crates/omx-sparkshell/src/threshold.rs` claim 2.2 KB | ✓ EXACT (2151 bytes) |
| File size | `crates/omx-sparkshell/src/exec.rs` claim 2.8 KB | ✓ EXACT (2819 bytes) |
| MCP server count | claim **5** (corrects cycle-26's 3) | ✓ EXACT (state, wiki, trace, memory, code-intel — 5 `*-server.ts` files) |
| Rust crate count | claim **5** (corrects cycle-26's 1) | ✓ EXACT (omx-explore, omx-mux, omx-runtime-core, omx-runtime, omx-sparkshell) |
| Cargo.toml dep | claim omx-sparkshell depends only on omx-mux | ✓ EXACT (`omx-mux = { path = "../omx-mux" }`, no other deps) |
| Constant | `DEFAULT_SPARK_MODEL = "gpt-5.3-codex-spark"` | ✓ EXACT |
| Constant | `DEFAULT_STANDARD_MODEL = "gpt-5.4-mini"` | ✓ EXACT |
| Constant | `DEFAULT_SUMMARY_TIMEOUT_MS = 60_000` | ✓ EXACT |
| Constant | `DEFAULT_FRONTIER_MODEL = "gpt-5.5"` (in `src/config/models.ts`) | ✓ EXACT |
| Constant content | `KEYWORDS_REQUIRING_INTENT` 7 entries: ralph/team/stop/abort/parallel/autoresearch/ultragoal | ✓ EXACT |
| Constant content | `should_retry_with_fallback` signals (9): quota/rate limit/429/unavailable/not available/unknown model/model not found/no access/capacity | ✓ EXACT |
| AGENTS.md text | line-132 directive "Do not hardcode stale frontier-model overrides … `gpt-5.5`" | ✓ EXACT TEXT |
| Code structure | claim `KOREAN_IME_MAP: Record<string, string>` constant at ~line 48 | ⚠ STRUCTURAL-FABRICATION (no such constant; actual is inline `text.replace(/ㅕㅣㅈ/g, 'ulw')` inside `normalizeWorkflowKeyboardTypos()` at line ~535) |
| Code structure | claim `EXPLICIT_SKILL_RE = /…/g` constant at ~line 160 | ⚠ STRUCTURAL-FABRICATION (no constant; regex inline inside `parseExplicitSkillInvocations()`; actual regex is `/(?:^\|[^\w])\$(?:(?:oh-my-codex:)?([a-z][a-z0-9-]*))\b/gi` — note `\b` boundary and `gi` flags PR's pseudo-code omits) |
| Code structure | `AUTO_COMPLETE_TRANSITIONS` claim shows 6 transitions | ⚠ MAGNITUDE-1-MISSING (actual set has 7 entries; PR missed `deep-interview->autoresearch`) |
| Code structure | `ALLOWED_OVERLAP_PAIRS` claim `ultrawork \| *` is in the set | ⚠ STRUCTURE-WRONG (actual set has only `'ralph\|team'`; the ultrawork-overlaps-everything semantic is real but lives in the runtime function `isAllowedOverlap()` short-circuit, not in the set) |

**Net:** 26 quantitative/structural claims verified — **21 EXACT/within-drift + 5 substantive inaccuracies** in the dispatch deliverable's
code-citation specifics. The 5 inaccuracies cluster as a **NEW failure
mode**: *structural-fabrication-with-correct-direction* — the architectural
patterns described in PR's prose ARE present (Korean IME normalization
runs, explicit-skill parsing happens, ultrawork overlaps with everything,
auto-complete transitions exist), but the cited code STRUCTURE (constant
names, regex literals, set membership locations, magnitude counts) is
fabricated, mis-located, or off-by-one. This is distinct from cycle-96
fabrication-magnitude (numbers without counting; same direction; large
magnitude error), cycle-99 direction-failure (H3 hypothesis directionally
wrong; opposite asymmetry from claimed), and cycle-100 magnitude-1-off-by-source-vs-runtime
(direction-accurate, count-correct against operational state, off-by-one
against source release).

Methodologically, **PR #2874's verification posture is the WEAKEST in the
absorption arc since cycle-96 PR #2878**: file sizes were measured (so the
quantitative-physical claims verify EXACT), but code-structure claims
appear to have been authored from memory or pattern-matching against
similar codebases rather than re-read against the source. The dispatch-
author verification-discipline meta-attribute now stands at **3-success
+ 2-mixed-success / 1-failure on a 6-instance evidence base** (cycle 96
PR #2878 fabrication-magnitude failure / cycle 97 PR #2877 calibration
verified / cycle 98 PR #2873 file-state verified / cycle 99 PR #2876
EXACT across 13 metrics / cycle 100 PR #2875 EXACT across 14 with
direction-accurate magnitude-1-off / cycle 101 PR #2874 EXACT across
21 quantitative + structural-fabrication-with-correct-direction across
5 code-citation specifics).

## Sources read so far

- Cycle 26 (high-level survey): cycle-33 stub assembled from PR #2784
  cycle-26 dispatch deliverable (closed-without-merge per absorption
  convention; `_notes/cycle-26-oh-my-codex-research.md` on never-merged
  branch `copilot/redesign-research-phase-1-survey` at commit `f291ec05`).
- Cycle 63 dispatch (code-level deeper-read), cycle 101 absorbed:
  - `src/hooks/keyword-detector.ts` (45590 bytes — 4-stage pipeline:
    Korean IME normalization → explicit `$skill` parsing → implicit
    KEYWORD_MAP → intent gate on 7 special skills)
  - `src/config/generator.ts` (50358 bytes — pure TOML upsert engine;
    monolithic developer instructions; strip-and-rebuild idempotency;
    never touches AGENTS.md)
  - `src/autoresearch/runtime.ts` (45592 bytes — orchestration primitives
    with calling-agent-driven loop; deterministic 8-branch
    `decideAutoresearchOutcome`; ledger atomicity gap vs state layer)
  - `src/state/workflow-transition.ts` (7983 bytes — `AUTO_COMPLETE_TRANSITIONS`
    7-entry set, `ALLOWED_OVERLAP_PAIRS` 1-entry set + ultrawork
    short-circuit, `evaluateWorkflowTransition` decision matrix)
  - `src/state/operations.ts` (14308 bytes — `writeAtomicFile()` POSIX
    rename semantics, in-process serialization via `withStateWriteLock()`)
  - `src/mcp/state-server.ts` (5352 bytes — 5 tools, mode enum 9 entries,
    legacy team tools hard-deprecated)
  - `src/mcp/wiki-server.ts` (8784 bytes — 8 tools, 8 categories,
    `wiki_add` rejects overwrites, legacy fallback detection)
  - `src/mcp/trace-server.ts` (10505 bytes — 2 tools, dual-stream timeline
    merge, `keepLastEntries()` in-place insertion sort)
  - `src/mcp/memory-server.ts` (15810 bytes — read-only metadata noted)
  - `src/mcp/code-intel-server.ts` (25128 bytes — read-only metadata noted)
  - `crates/omx-sparkshell/Cargo.toml` (271 bytes — single dep `omx-mux`)
  - `crates/omx-sparkshell/src/codex_bridge.rs` (14597 bytes —
    3-tier model resolution with env priority; primary/fallback retry
    logic; 9-signal capacity-error detection; 60-second timeout)
  - `crates/omx-sparkshell/src/main.rs` (10371 bytes — `SparkShellInput`
    enum: Command vs TmuxPane; tail lines default 200 / min 100 / max 1000)
  - `crates/omx-sparkshell/src/prompt.rs` (11877 bytes — summary prompt
    construction; 11 command families; truncation 400 lines / 24000 bytes)
  - `crates/omx-sparkshell/src/threshold.rs` (2151 bytes — read-line
    threshold logic)
  - `crates/omx-sparkshell/src/exec.rs` (2819 bytes — command execution)
  - `src/hooks/triage-heuristic.ts` (13629 bytes — 11-rule pure classifier;
    advisory only)
  - `src/hooks/session.ts` (13465 bytes — PID-based identity via /proc on
    Linux)
  - `src/hooks/prompt-guidance-contract.ts` (11887 bytes — 9 contract
    arrays + marker pairs + invariant contracts)
  - `templates/AGENTS.md` (24606 bytes — overlay marker zones; lore commit
    protocol; child agent model routing directive at line 132)
  - `docs/STATE_MODEL.md` (8951 bytes — normative state spec; 5-step
    reconciliation sequence; lifecycle vocabulary; legacy enum mapping)
- `crates/omx-explore`, `crates/omx-mux`, `crates/omx-runtime-core`,
  `crates/omx-runtime`: directory structure verified; Cargo.toml metadata
  noted; source files NOT read (deferred).

## Project framing

oh-my-codex (omx) is **a configuration layer + hook harness over an
unmodified Codex CLI**, not an agent framework itself. The README states
explicitly and repeatedly: "OMX does NOT replace Codex." The project
ships **39 skills, 30 role prompts, 5 first-party MCP servers, and a 5-crate
Rust workspace (`omx-explore`, `omx-mux`, `omx-runtime-core`, `omx-runtime`,
`omx-sparkshell`)** all sitting on top of an unmodified Codex CLI.

Cycle-26 misreported both MCP server count (3, actual 5) and Rust crate
count (1, actual 5). The cycle-63 deep read corrects these and adds
substantial code-level evidence.

This makes oh-my-codex structurally distinct from the agent frameworks
(AutoGen, LangGraph) and the autonomous agent (Voyager) — its substrate
is "thin layer over an existing CLI" rather than "new runtime." It shares
this substrate posture with **oh-my-claudecode** (omc, cycle 99 first-pass)
which uses Claude Code as its substrate; the two are sister projects per
[Eva directive #2774](https://github.com/EvaLok/schema-org-json-ld/issues/2774).

**Required-reads-completion + sister-project parity.** With omx now at
deep-dive parity (cycle 101), two architectural alignments become
visible:

- **omx + omc both at first-pass-or-deeper depth.** PAI's deep-dive
  (cycle 100) and openclaw's deep-dive (cycle 43) covered the
  redesign prompt's two named required reads. omx and omc are
  Eva-named (#2774, #2775) further reads — both now at first-pass or
  deeper, with omx at deep-dive (cycle 101) and omc at first-pass
  (cycle 99 pending elevation).
- **Sister-project structural-alignment hypothesis testable.** Cycle 99
  noted that omc shows 14 of 22 cycle-26 patterns as PARALLEL or
  ADAPTED (64% mapping density), reflecting sister-project structural
  alignment with the omx-derived 22-pattern catalogue. With omx now at
  deep-dive, the 22-pattern catalogue is *itself* refreshable from
  cycle-63 evidence — cycle-26 patterns 1-13 confirm at code-level depth;
  3 corrections enumerated; 8-9 NEW patterns added.

## Hypothesis verdicts (cycle 63 dispatch + cycle 101 verification)

PR #2874 was authored as a deep-read with explicit cycle-26 supersedence
intent rather than as a hypothesis-test dispatch. There is no H1/H2/H3
hypothesis structure analogous to PAI / Symphony / oh-my-claudecode.
Instead, cycle 63 dispatched 22-pattern code-level confirmation/correction
with explicit "what does the code say" framing.

Result: **3 corrections (Rust crate count, sparkshell model name, state
file authority) + 7 named confirmations (4-stage pipeline, planning-before-execution
arbitration, ralplan gate is advisory, autoresearch runtime is orchestration
primitives not a supervisor, deterministic decision tree, generator.ts
writes only config.toml, writeAtomicFile() POSIX atomicity).** Beyond
that, **9 new code-level patterns not in cycle 26** were extracted (codex
bridge retry, normalize_summary allowlist, wiki legacy fallback detection,
trace dual-stream timeline merge, STATE_MODEL.md normative reference,
lore commit protocol, child agent model routing, consecutive-only `$skill`
chain rule, autoresearch ledger vs state atomicity gap).

Cycle 101 verification adds: **the 3 corrections + 7 confirmations are
all CONFIRMED at the code-level the dispatch claimed.** The 9 new patterns
are CONFIRMED at architectural-pattern level but with the structural-
fabrication-with-correct-direction caveat noted: where PR cites specific
constant names or regex literals as evidence, the prose-level claims are
correct but the code-citation specifics may be off (see verification banner).

## Patterns observed (organized into family buckets)

### State, memory, history

- **Per-mode state files as authority.** `.omx/state/<mode>-state.json`
  (root scope) or `.omx/state/sessions/<session_id>/<mode>-state.json`
  (session scope) is the single source of truth for each workflow mode.
  `skill-active-state.json` is a *compatibility/visibility layer*, not
  the authority — explicitly stated in `docs/STATE_MODEL.md` (cycle-26
  ambiguity corrected at cycle 63).

- **Read precedence (3-tier).** Explicit session scope → current session
  scope → root scope fallback. If root and session disagree, **session
  wins** for the active execution context (`docs/STATE_MODEL.md`).

- **Failure as recorded artifact.** Ralph progress ledger
  (`.omx/state/<session_or_root>/ralph-progress.json`) records failure
  entries with timestamps. The autoresearch loop's `iteration-ledger.json`
  records keep/discard/abort/noop/interrupted/ambiguous decisions per
  iteration with explicit `decision_reason` strings.

- **Append-only history with one-way migrations.** File-backed migration
  with one-way compatibility windows (legacy `.omx/prd.json` →
  `.omx/plans/prd-<slug>.md`): legacy files preserved as read-only,
  schema migrations one-way, not destructive.

- **Markdown-first wiki memory.** `.omx/wiki/` markdown wiki with
  MCP wiki server (`src/mcp/wiki-server.ts` 8784 bytes); SessionStart hook
  can inject bounded wiki context; markdown-first, search-first (not
  vector-based). Categories: `architecture`, `decision`, `pattern`,
  `debugging`, `environment`, `session-log`, `reference`, `convention`.

- **Plans/specs as forward-versioned artifacts.** `.omx/context/
  {task-slug}-{timestamp}.md` written before execution begins, with
  explicit fields for task statement, desired outcome, known facts,
  constraints, unknowns, and codebase touchpoints (Pattern 2 in cycle-26
  dispatch).

- **POSIX atomic write semantics for state.** `writeAtomicFile()` writes
  to `${path}.tmp.${pid}.${timestamp}.${random}`, then calls POSIX
  `rename()`. On rename failure, temp file is unlinked and error rethrown.
  In-process serialization via `withStateWriteLock()` Promise-chain
  queue (per-path, not OS-level file locks).

- **Atomicity asymmetry: state vs autoresearch ledger.** `src/state/operations.ts`
  uses `writeAtomicFile()`; `src/autoresearch/runtime.ts` uses plain
  `writeFile()` for the iteration-ledger. The asymmetry is documented as
  a known architectural gap (PR #2874 §1.3 "Atomicity Gap"). Adapting
  the autoresearch pattern requires deciding whether to align atomicity.

### Quality & discipline

- **Anti-patterns at multiple layers.** CONTRIBUTING.md `<Bad>` examples
  (e.g., "Claiming completion without verification: 'should work
  correctly. Task complete.'"); explicit deprecations (`$web-clone`
  "hard-deprecated"); and `templates/AGENTS.md` opening with negative
  directives.

- **Behavioral prompt-contract regression tests.** `src/hooks/__tests__/prompt-guidance-*.test.ts`
  enforces 9 contract arrays (`ROOT_TEMPLATE`, `CORE_ROLE`, `SCENARIO_ROLE`,
  `WAVE_TWO`, `CATALOG`, `LEGACY_PROMPT`, `SPECIALIZED_PROMPT`, `SKILL`,
  `PROMPT_REFACTOR_MARKER`, `PROMPT_REFACTOR_INVARIANT`). Each contract
  specifies a file path and required regex patterns that MUST be present.
  Tests fail in CI if patterns drift.

- **Marker-pair contracts as mechanical guards.** `PROMPT_REFACTOR_MARKER_CONTRACTS`
  verify that each `<!-- OMX:GUIDANCE:*:START -->` marker has a
  corresponding `<!-- OMX:GUIDANCE:*:END -->` marker — mechanically
  preventing half-deleted overlay zones from surviving undetected.

- **Iteration ceilings with explicit numerical limits.** `max_iterations=10`
  for tool-loops; `max=5` for review-loops; autoresearch loop with explicit
  keep/discard/stop per-iteration decision via deterministic
  `decideAutoresearchOutcome()` 8-branch decision tree.

- **Mandatory deslop pass.** Required as post-completion step
  (Pattern 13 in cycle-26 dispatch); embeds quality cleanup into each
  task's completion contract.

- **Lore commit protocol.** `templates/AGENTS.md` `<lore_commit_protocol>`
  block defines a commit message trailer format with explicit fields:
  `Constraint:`, `Rejected:` (alternatives future agents should not
  re-explore), `Confidence:` (low/medium/high), `Scope-risk:`
  (narrow/moderate/broad), `Directive:` (forward-looking warning),
  `Tested:`, `Not-tested:`. Enforced by convention, not CI. The
  cycle-101-verified PR commit message itself uses this format.

### Algorithm & cycle-internal phasing

- **8-branch deterministic decision tree.** `decideAutoresearchOutcome()`
  in `runtime.ts` is **fully deterministic** — no LLM calls, pure
  decision logic. 8 terminal branches across `abort`/`noop`/`interrupted`
  status, evaluator error/pass-false, keep policy `pass_only` /
  `score_improvement`, score-improved logic. This is the cluster-A
  Algorithm-as-code-construct sub-shape: business logic as deterministic
  decision tree, not as prompt judgment.

- **Workflow transition decision matrix.** `evaluateWorkflowTransition()`
  in `workflow-transition.ts` returns a `WorkflowTransitionDecision` with
  `kind: 'allow' | 'overlap' | 'auto-complete' | 'deny'`. The 7-entry
  `AUTO_COMPLETE_TRANSITIONS` set governs source-completion semantics;
  the 1-entry `ALLOWED_OVERLAP_PAIRS` set + ultrawork short-circuit
  governs overlap; `isRollbackTransition()` blocks
  any-execution-like → any-planning-like transition. **Transitions are
  pure logic, not prompt judgment.**

- **Reconciliation sequence (5-step ordering).** From `docs/STATE_MODEL.md`:
  (1) decide outcome, (2) complete source modes with audit metadata,
  (3) sync compatibility `skill-active` state, (4) activate destination
  modes, (5) return transition message. The ordering is load-bearing —
  syncing too early can resurrect an auto-completed mode.

- **Audit fields on auto-complete.** Auto-completed mode files must record
  `active: false`, `current_phase: 'completed'`, `completed_at` timestamp,
  `auto_completed_reason`, `completion_note`, `transition_target_mode` —
  preserving the audit trail when a mode was completed by transition rather
  than by explicit `$cancel` or natural finish.

### Hooks & lifecycle

- **PID-based session identity via /proc.** `src/hooks/session.ts`
  detects stale sessions on Linux by reading `/proc/<pid>/status`. Stale
  sessions are reconciled at startup. Falls back to weaker heuristic on
  non-Linux. **Substrate-tied to Linux-with-/proc**, an architectural
  choice not portable.

- **UserPromptSubmit routing.** `src/scripts/codex-native-hook.ts` is
  the entry point. Flow: `detectKeywords()` → ordered explicit skill list
  → `recordSkillActivation()` → shared reconciliation helper → final
  active skills → `buildAdditionalContextMessage()` → native hook output.

- **Continuation shortcut bypass.** `shouldReusePreviousSkillForContinuation()`
  matches phrases ("keep going", "continue", "resume") and re-activates
  the previous skill without re-running the 4-stage detection pipeline.
  Fast path before the main pipeline.

- **Triage heuristic as advisory-only classifier.** 11-rule pure
  classifier in `triage-heuristic.ts` — synchronous, no I/O, no state
  reads. Output is `TriageLane` (PASS / LIGHT / HEAVY) with optional role
  hint (executor / explore / researcher / designer). Runs only when no
  keyword matches. **Never activates a workflow skill, never writes
  state, never blocks execution.**

- **Deep-interview lock blocks shortcuts.** `DEEP_INTERVIEW_BLOCKED_APPROVAL_INPUTS`
  blocks standard approval shortcuts during active deep-interview;
  `hadDeepInterviewLock` redirects cancel intent to prevent accidental
  cancellation via "stop" during interview.

- **AGENTS.md overlay marker zones.** `templates/AGENTS.md` contains
  marker-bounded overlay zones (`<!-- OMX:RUNTIME:START/END -->`,
  `<!-- OMX:TEAM:WORKER:START/END -->`, `<!-- OMX:GUIDANCE:OPERATING:START/END -->`,
  `<!-- OMX:GUIDANCE:SPECIALIST-ROUTING:START/END -->`,
  `<!-- OMX:GUIDANCE:VERIFYSEQ:START/END -->`,
  `<!-- OMX:MODELS:START/END -->`) populated by `src/hooks/agents-overlay.ts`.
  The generator never touches AGENTS.md (clean separation of concerns).

### Detection & routing

- **4-stage keyword detection pipeline.** Confirmed exactly as cycle-26
  described:
  - **Stage 1: Korean IME normalization.** `normalizeWorkflowKeyboardTypos()`
    runs `text.replace(/ㅕㅣㅈ/g, 'ulw')` before any pattern matching
    — single normalization (Korean 2-set keyboard typo for ultrawork
    activation), narrow scope by design.
  - **Stage 2: Explicit `$skill` parsing.** `parseExplicitSkillInvocations()`
    uses inline regex `/(?:^|[^\w])\$(?:(?:oh-my-codex:)?([a-z][a-z0-9-]*))\b/gi`.
    Left-to-right scan; consecutive-only chain (parsing stops at first
    non-whitespace gap between `$` tokens). Anti-confusion guard: if any
    `$`-prefixed token is present but none matches a registry entry, the
    detector returns empty list rather than falling through to implicit
    matching.
  - **Stage 3: Implicit matching.** `KEYWORD_MAP` array scan; word-boundary
    anchors via `keywordToPattern()`; ALL matching skills returned (not
    just first match).
  - **Stage 4: Intent gate.** `KEYWORDS_REQUIRING_INTENT` 7-entry set
    (`ralph`, `team`, `stop`, `abort`, `parallel`, `autoresearch`,
    `ultragoal`). Each match runs against `KEYWORD_INTENT_PATTERNS[skill]`;
    if positive intent cannot be confirmed, the skill match is dropped.
    Prevents false positives on conversational uses of e.g. "stop".

- **Multi-keyword arbitration enforces planning-before-execution.**
  `resolveRequestedWorkflowSkills()` splits results into planning-like
  (`deep-interview`, `ralplan`, `autoresearch`) and execution-like
  (`team`, `ralph`, `autopilot`, `ultrawork`, `ultraqa`). When both
  appear, only the first planning-like skill is returned as
  `requestedSkills`; execution-like skills go to `deferredSkills`.
  **Planning-before-execution is enforced at the detection layer**, not
  delegated to prompt-level guidance.

- **Ralplan gate is advisory-only.** `applyRalplanGate()` and
  `isUnderspecifiedForExecution()` redirect execution keywords to
  `$ralplan` when prompt is vague (≤ 15 words, no well-specified
  signals). Both are bypass-able via `force:` prefix, `!` prefix,
  `planningComplete` status, or approved followup shortcuts. **The gate
  does NOT enforce at the hook level** — it's a soft suggestion the
  user can override.

- **Code-block exclusion gap.** No fenced-code exclusion in keyword
  detection. A `$ralph` inside a code block **would** trigger skill
  activation. Architectural gap worth noting for any port.

### Configuration synthesis

- **`generator.ts` is a pure TOML upsert engine.** No side effects beyond
  writing config.toml. Strip-and-rebuild idempotency (`stripExistingOmxBlocks()`
  finds `# oh-my-codex (OMX) Configuration ... # End oh-my-codex` and
  removes it before rebuilding). Customized `[tui]` sections are extracted
  before stripping and reinjected after rebuild.

- **Deterministic output structure order.** Top-level keys → seeded
  behavioral defaults (model_context_window=250000, model_auto_compact=200000,
  only when `model == DEFAULT_SETUP_MODEL`) → `[features]` → shell env
  policy → `[agents]` → user's pre-existing sections (preserved) → OMX
  tables block → `[tui]` with status_line. **Order is fixed**, simplifying
  diffing across regenerations.

- **Generator owns config.toml only.** AGENTS.md managed separately by
  `src/hooks/agents-overlay.ts`. Clean separation of concerns: config
  generation is mechanical, AGENTS.md overlay is hook-driven.

- **Monolithic developer instructions.** `OMX_DEVELOPER_INSTRUCTIONS` and
  `OMX_PLUGIN_DEVELOPER_INSTRUCTIONS` are single string constants — not
  composed from smaller pieces at runtime. Plugin mode swaps the constant
  but doesn't compose. Same string regardless of which model is selected.
  This is **opposite to PAI's principle-driven Algorithm document approach**
  (PAI composes phase-driven prompts from the 46KB Algorithm v6.3.0
  doctrine; omx hard-codes the developer instructions in TypeScript
  source).

### Agent architecture

- **Per-agent model selection across providers.** `src/config/models.ts`
  declares supported models GPT-5.4, GPT-5.4-mini, GPT-5.5, GPT-5.3-codex.
  `DEFAULT_FRONTIER_MODEL = 'gpt-5.5'`, `DEFAULT_STANDARD_MODEL = 'gpt-5.4-mini'`,
  `DEFAULT_SPARK_MODEL = 'gpt-5.3-codex-spark'` — three-tier model hierarchy
  with env-var override priority `OMX_DEFAULT_FRONTIER_MODEL` >
  `OMX_DEFAULT_STANDARD_MODEL` > `OMX_DEFAULT_SPARK_MODEL` > config.toml
  model > defaults. The "mini composition seam" gates exact-model
  behavior; `$ask-claude` and `$ask-gemini` skills shell to non-OpenAI
  provider CLIs from within a Codex session.

- **30 named role prompts in `prompts/*.md`.** Metis as analyst, Ralph
  as persistent executor, plus planner / architect / critic / verifier /
  researcher / etc. Workflow stages
  (`$deep-interview` → `$ralplan` → `$ralph` → `$team`) hand off across
  role-named agents.

- **Child agent model routing directive.** `templates/AGENTS.md` line 132
  contains current-frontier directive: *"Do not hardcode stale frontier-model
  overrides for Codex native child agents. If an explicit frontier override
  is necessary, use the current frontier default from `OMX_DEFAULT_FRONTIER_MODEL`
  / the repo model contract (currently `gpt-5.5`), not older values such
  as `gpt-5.2`."* The directive is **enforcement by convention** in
  AGENTS.md — readable by child agents, not enforced by code.

### MCP tool surface

- **5 MCP servers (corrects cycle-26's 3).** `omx-state` (5.4 KB,
  5 tools), `omx-wiki` (8.8 KB, 8 tools), `omx-trace` (10.5 KB, 2 tools),
  `omx-memory` (15.8 KB), `omx-code-intel` (25.1 KB). All servers use
  `autoStartStdioMcpServer()` from `src/mcp/bootstrap.ts` for uniform
  stdio transport lifecycle.

- **Hard-deprecated team MCP tools.** `state-server.ts` rejects
  `TEAM_COMM_TOOL_NAMES` with explicit error: *"MCP tool ${name} is
  hard-deprecated. Team mutations now require CLI interop."* Migration
  signal: legacy tool removal preserved in code with deprecation message
  pointing at successor mechanism.

- **`wiki_add` rejects overwrites.** Use `wiki_ingest` to merge into
  existing pages. Distinguishes idempotent merge from accidental overwrite.

- **Legacy wiki fallback detection.** `wiki_refresh` calls
  `isLegacyWikiFallbackActive()`. When old `.omx/wiki/` path exists
  instead of canonical `omx_wiki/`, server is **read-only** and returns
  migration instruction: *"Legacy .omx/wiki fallback is read-only; copy
  selected pages into omx_wiki/ before refreshing canonical metadata."*
  Forward-migration guard pattern — old data can be read but not
  modified, forcing eventual migration.

- **Trace dual-stream timeline merge.** `trace_timeline` merges agent turn
  JSONL entries + mode start/end events reconstructed from per-mode
  state files into a single sorted timeline. `keepLastEntries()` uses
  in-place insertion sort (O(k) where k = window size) to maintain
  top-N most-recent without allocating full array — deliberate memory
  optimization for large log files.

### Rust substrate edge

- **5-crate Rust workspace (corrects cycle-26's 1).** `omx-explore` (repo
  exploration routing), `omx-mux` (tmux pane capture helpers),
  `omx-runtime-core` (shared runtime types), `omx-runtime` (runtime
  orchestration), `omx-sparkshell` (output summarization via codex exec).
  cycle-26's claim of "1 Rust crate" reflected only the most-prominent
  crate — the workspace has 5 with shared `Cargo.lock` and workspace-level
  dependency configuration.

- **`omx-sparkshell` minimal-dependency-graph.** Cargo.toml depends only
  on `omx-mux`. **Zero external (non-stdlib, non-workspace) crate
  dependencies.** Architectural choice: keep the spark path tight and
  auditable.

- **`SparkShellInput` enum: Command vs TmuxPane.** Two input modes —
  Command runs argv directly via `execute_command()`; TmuxPane builds
  `tmux capture-pane` argv via `omx_mux::build_capture_pane_args()`.
  Both apply same raw-vs-summarize threshold logic. Tail lines: default
  200, min 100, max 1000.

- **Codex bridge as subprocess invocation.** `codex_bridge.rs` invokes
  `codex exec --model X --sandbox read-only -c model_reasoning_effort=low
  --skip-git-repo-check --color never -` as subprocess. Prompt written
  to stdin; stdout collected. Sandbox: read-only. **Codex is the LLM
  primitive; sparkshell is the harness around it.**

- **3-tier model resolution with env priority.** Primary: `OMX_SPARKSHELL_MODEL`
  > `OMX_DEFAULT_SPARK_MODEL` > `OMX_SPARK_MODEL` > `DEFAULT_SPARK_MODEL`
  ("gpt-5.3-codex-spark"). Fallback: `OMX_SPARKSHELL_FALLBACK_MODEL` >
  `OMX_DEFAULT_STANDARD_MODEL` > `DEFAULT_STANDARD_MODEL` ("gpt-5.4-mini").
  Six independent env-var override paths.

- **Retry-on-capacity-error with 9-signal allowlist.** `should_retry_with_fallback()`
  lowercases stderr, checks for any of: "quota", "rate limit", "429",
  "unavailable", "not available", "unknown model", "model not found",
  "no access", "capacity". If matched, retries with fallback model. If
  fallback also fails, returns compound error with both messages.

- **60-second timeout with 25ms poll loop.** `DEFAULT_SUMMARY_TIMEOUT_MS = 60_000`,
  configurable via `OMX_SPARKSHELL_SUMMARY_TIMEOUT_MS`. Loop polls
  `child.try_wait()` every 25ms; calls `child.kill()` on expiry.

- **`normalize_summary` allowlist as model-output contract.** Model is
  instructed to produce sections `summary:`, `failures:`, `warnings:`.
  `normalize_summary()` enforces this at parse time regardless of model
  output — any other section header (e.g., `next steps:`,
  `recommendations:`) is silently dropped. **Contract boundary between
  model output and what reaches the user's terminal.** Cluster
  expectation: deterministic post-processing of LLM output with allowlist
  filtering.

- **11-family command classification.** `select_command_family()` maps
  executable basename to `git`, `node-js`, `python`, `rust`, `go`,
  `ruby`, `java-kotlin`, `c-cpp`, `csharp`, `swift`, or `generic-shell`.
  Family key is included in summary prompt for context-specific
  summarization.

- **Truncation: head + tail with elision marker.** `OMX_SPARKSHELL_SUMMARY_MAX_LINES`
  = 400, `OMX_SPARKSHELL_SUMMARY_MAX_BYTES` = 24000. Stdout and stderr
  each independently truncated. Preserves head + tail (cuts middle).

### Substrate-edge orchestration

- **Configuration-layer-with-hooks architecture.** Not an agent framework
  — a small entry-point pattern adding 39 skills, 30 role prompts, 5 MCP
  servers, and a 5-crate Rust workspace on top of an unmodified Codex
  CLI. **The substrate (Codex CLI) is unmodified**; all orchestration
  is in the layer above.

- **`$team` runtime as opt-in, not default.** README explicitly states
  "$team is not the default onboarding path." Multi-agent is available
  but de-prescribed for new users.

- **Named workflow modes with deterministic transition policy.**
  `deep-interview`, `ralplan`, `ralph`, `team`, `autopilot`, `ultrawork`,
  `ultraqa` governed by an explicit transition allowlist
  (`AUTO_COMPLETE_TRANSITIONS` 7 entries + `ALLOWED_OVERLAP_PAIRS` 1 entry
  + ultrawork-overlaps-everything short-circuit) — multiple orchestration
  patterns coexist with deterministic transition policy preventing
  illegal mode shifts.

- **Code-vs-prompts split at implementation level.** MCP servers, the
  4-stage keyword detector (deterministic pattern matching with
  inline regex literals, not semantic classification), the deterministic
  workflow-transition decision matrix, and the Rust sparkshell harness
  all sit deterministically alongside the LLM-driven Codex tool loop.
  **The split is concrete and code-cited**, not aspirational.

## Cluster framework anchoring (observation-only)

The cluster catalogue (`docs/redesign/2-candidates/clusters.md`) currently
anchors on 8-9 deep-dive systems. omx now joins that set as the 9th-or-10th
deep-dive system (depending on whether PAI's cycle-100 deep-dive is
counted; both reach deep-dive parity at cycle 100/101). Per cycle-99/100
discipline, **single-system observations from omx are NOT auto-promoted
to cross-system convergence**; they are recorded here as observations
pending convergence-evidence from cluster-catalogue update.

Likely cluster cells where omx contributes evidence (deferred to cycle
102/103 cluster-catalogue rebuild):

- **Cluster A (Algorithm-as-code-construct):** omx contributes the 8-branch
  deterministic `decideAutoresearchOutcome()` decision tree and the
  `evaluateWorkflowTransition()` decision matrix. Both are pure-logic
  decision functions, distinct from PAI's classifier-mediated mode
  dispatch (cycle 100 NEW sub-shape) and from openclaw's prompt-driven
  phase progression. Potential NEW sub-shape: *deterministic-decision-tree*
  as cluster A sub-shape distinct from classifier-mediated-routing and
  prompt-driven-phasing.

- **Cluster F (substrate-coverage):** omx is configuration-layer-over-Codex-CLI;
  PAI is principles-encoded-Claude-Code-substrate; omc is configuration-layer-over-Claude-Code.
  **3-system convergence on substrate-edge thin-wrapper-with-deep-hooks
  pattern** (omx + omc + PAI, with PAI being substrate-deeper but still
  Claude-Code-substrate-targeted). Sub-axis: hooks/skills/MCP-servers/Rust-edge
  quad observed in omx (hooks + skills + 5 MCP servers + 5 Rust crates),
  parallel to PAI's two-category-capability + Hooks/Skills/Tools/Agents
  quad noted cycle 100. Cross-system: 2-2-instance evidence for thin-wrapper
  substrates ship hooks + skills + MCP + Rust at scale.

- **Cluster H (Learn→Improve closure):** omx contributes the autoresearch
  iteration ledger (append-only with per-iteration keep/discard/abort
  decisions and `decision_reason` strings) — a different shape from
  PAI's hook-driven WorkCompletionLearning + SatisfactionCapture pair
  (cycle 100 NEW sub-shape: feedback-signal-inference). omx's pattern is
  *post-iteration-evaluator-driven decision tree*, recorded with explicit
  reasons; PAI's is *signal-inference-from-user-prompt*. Potential NEW
  cluster H sub-shape: *evaluator-driven-keep-discard* distinct from
  hook-driven-feedback-inference.

- **Cluster I (substrate-coverage diversity):** Substrate diversity now
  spans personal-assistant (PAI), agent-framework (AutoGen / LangGraph),
  research-code (Voyager), spec-first-orchestrator (Symphony first-pass),
  configuration-layer-over-CLI (omx + omc both at first-pass-or-deeper).

omx specifically does NOT contribute evidence to:

- **Cluster J (semantic-retrieval architecture):** omx uses markdown wiki
  with explicit categories and search-first retrieval (similar to PAI's
  BM25-anti-RAG stance) — not a vector store. Aligns with PAI on the
  anti-RAG architectural choice. Potential 2-system convergence on
  *deliberate-anti-RAG-with-keyword-retrieval* sub-shape if omx's MCP
  wiki search mechanism is re-read at deeper depth (currently inferred
  from architectural posture; not code-cited yet).

## Anchoring caveats

- **Configuration layer, not agent framework.** Patterns reflect a
  configuration-and-hooks substrate that wraps an external CLI.
  Compare AutoGen/LangGraph (frameworks defining their own runtime) or
  Voyager (research code with its own loop). Some patterns (workflow
  modes; per-mode state) are tightly coupled to that substrate.

- **TypeScript with Rust workspace.** Most omx code is TypeScript; the
  Rust workspace is a 5-crate set with omx-sparkshell most prominent.
  Architectural ideas transfer; library affordances do not.

- **Substrate-tied: Linux-with-/proc.** PID-based session staleness
  detection reads `/proc/<pid>/status`. Non-Linux uses weaker fallback.
  Adapting omx patterns to other OSes requires designing a portable
  liveness check.

- **Single-author project (`Yeachan-Heo`).** External evidence-base is
  one author's design choices over time. Patterns may reflect single-author
  judgment rather than consensus across many contributors. Mitigation:
  cycle-63 deep read covers code + git history + AGENTS.md commit-protocol
  evidence with multiple verification anchors.

- **Code-citation-specifics need light double-check at adoption time.**
  PR #2874 verification surfaced 5 instances of structural-fabrication-with-correct-direction
  (KOREAN_IME_MAP non-existent constant; EXPLICIT_SKILL_RE non-existent
  constant + slightly-wrong inline regex; AUTO_COMPLETE_TRANSITIONS magnitude-1-missing;
  ALLOWED_OVERLAP_PAIRS structure-wrong). **The architectural patterns
  ARE all real** — code-level prose is correct. But adoption that depends
  on the *specific* constant name, regex, or set membership should re-read
  the source rather than copy from PR #2874's pseudo-code blocks.

- **Plugin mode swaps developer-instructions constant.** `OMX_DEVELOPER_INSTRUCTIONS`
  vs `OMX_PLUGIN_DEVELOPER_INSTRUCTIONS` distinction reflects two
  deployment modes. Patterns observed are primarily from the non-plugin
  mode; plugin mode behavior may differ in subtle ways.

- **Memory-server and code-intel-server not deeply read.** `src/mcp/memory-server.ts`
  (15.8 KB) and `src/mcp/code-intel-server.ts` (25.1 KB) are listed as
  unread in PR #2874 §9.2. Patterns from these subsystems are not in
  scope of this absorption.

- **Five Rust crates, only one read at depth.** omx-sparkshell read in
  full; omx-explore / omx-mux / omx-runtime-core / omx-runtime have
  Cargo.toml metadata noted but source unread. Architectural patterns
  observed are sparkshell-centric.

## Deeper-read queue (cycle-63 deferred items)

PR #2874 §9.2 explicitly lists 14 unread files / directories:

| File / Directory | Size | Why deferred |
|---|---|---|
| `src/mcp/memory-server.ts` | 15.8 KB | Not fetched — memory model not required for this cycle |
| `src/mcp/code-intel-server.ts` | 25.1 KB | Not fetched |
| `src/hooks/__tests__/` | 45 KB dir | Test directory listed but individual test files not read |
| `skills/` | 20.3 KB dir | Directory listing too large; individual skill SKILL.md files not read |
| `prompts/` | (unknown) | Not listed |
| `src/state/workflow-transition-reconcile.ts` | 5918 bytes | Referenced in `STATE_MODEL.md` as 5-step reconciliation impl but not read |
| `src/modes/base.ts` | (unknown) | Referenced in docs but not read |
| `crates/omx-explore/src/` | (unknown) | Directory listed, source not read |
| `crates/omx-mux/` | (unknown) | Not read (used by sparkshell but not examined) |
| `crates/omx-runtime/` | (unknown) | Not read |
| `crates/omx-runtime-core/` | (unknown) | Not read |
| `crates/omx-sparkshell/src/threshold.rs` | 2151 bytes | Threshold default value not confirmed from source |
| `crates/omx-sparkshell/src/exec.rs` | 2819 bytes | Command execution wrapper |
| `crates/omx-sparkshell/src/registry/` | (dir) | Not read |

PR §9.7 also notes `templates/AGENTS.md` was truncated at ~20 KB (file is
24.6 KB, so final ~4.6 KB unread including likely `<execution_protocols>`
tail and any auto-generated `<!-- OMX:MODELS:START/END -->` block).

Deeper-read priorities for any cycle-102+ second-pass:

1. **`workflow-transition-reconcile.ts`** (5918 bytes) — implements the
   load-bearing 5-step reconciliation sequence; STATE_MODEL.md describes
   the ordering but the actual transition mechanics are in this file.
2. **`omx-explore` and `omx-mux` crate sources** — pair with sparkshell
   to complete the Rust-substrate-edge picture. omx-mux is sparkshell's
   only dep; understanding the mux abstraction is part of understanding
   the spark path.
3. **`memory-server.ts`** (15.8 KB) — second largest MCP server; the
   omx memory model isn't covered by cycle-63 evidence base. Adapting
   omx patterns that touch persistent memory needs this read.
4. **`AGENTS.md` final ~4.6 KB** — verifies the `<execution_protocols>`
   tail and `<!-- OMX:MODELS:START/END -->` overlay block content.
5. **Five-crate workspace dependency graph** — Cargo.toml at workspace
   root + per-crate Cargo.tomls would clarify the workspace-internal
   dependency tree (sparkshell → mux is documented; the others' deps
   aren't).

## Cycle 101 absorption record

- **Sources for absorption:** PR
  [#2874](https://github.com/EvaLok/schema-org-json-ld/pull/2874) (913
  lines, 9 lenses) on never-merged branch
  `copilot/redesign-research-cycle-63-deeper-read`. PR closed without
  merge per absorption convention.
- **Verification:** 26 quantitative + structural claims spot-checked
  against `Yeachan-Heo/oh-my-codex` at commit `d1863f72` via `gh api`;
  net 21 EXACT/within-drift + 5 structural-fabrication-with-correct-direction.
- **Absorption note:**
  [`_notes/cycle-101-oh-my-codex-deeper-read-absorption.md`](../../_notes/cycle-101-oh-my-codex-deeper-read-absorption.md).
- **Required-reads-completion + sister-project parity** — both
  redesign-prompt-named required reads (openclaw cycle 43 / PAI cycle 100)
  AND both Eva-named further reads (oh-my-codex cycle 101 / oh-my-claudecode
  cycle 99-first-pass) at first-pass-or-deeper depth. **Cycle 16
  deliverable-size asymmetry is now fully resolved across the cycle-26-vintage
  dispatch set + Eva-named reads.**
