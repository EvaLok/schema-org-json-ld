# PAI (danielmiessler/Personal_AI_Infrastructure, 12k★ / 1.6k forks / TypeScript)

[← back to Phase 1 index](../../1-research.md)

**Status: deep-dive (v5.0.0).** Cycle-71 dispatch deeper-read landed
cycle 100 as PR
[#2875](https://github.com/EvaLok/schema-org-json-ld/pull/2875)
(originating issue
[#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842),
cycle-71 dispatch under cycle-69 deferred-deeper-reads queue plan).
Per the absorption convention (PR closed without merge; deliverable
preserved on never-merged branch), the deliverable lives on branch
`copilot/redesign-research-deep-read-pai` as
`docs/redesign/_notes/cycle-71-pai-deeper-read.md` (667 lines; 9-lens
structure: memory architecture, algorithm code construct, learn→improve
closure, skills/agents stratification, decision hierarchy enforcement,
Tools/Packs substrate, .claude/ Claude Code integration, spec/test/evals,
anchoring caveats; with H1/H2/H3 hypothesis verdicts and a 17 founding-
principle verdict table). This per-system file summarizes the deliverable
in the per-system shape and cites it as the primary evidence base.

The cycle-14 README-only stub (84 lines, 16-principle list) is
superseded by this deep-dive read. The earlier framing remains
historically accurate as a README-surface read; v5.0.0 has substantially
extended the principle list (16 → 17, with revised Principle 1 "PAI is
the Life Operating System") and deeply codified what was previously stated
as goals.

**Verification status (cycle 100):** load-bearing factual claims spot-checked
against the live `danielmiessler/Personal_AI_Infrastructure` repo via
`gh api` before integration, following the cycle-96/97/98/99 verification-
discipline pattern.

| Class | Subject | Verdict |
|---|---|---|
| Top-level Tools/ | claim 2 source files | ✓ EXACT (4 items: 2 .ts files + README + image) |
| PAI/TOOLS/ | claim "70+" tools | ✓ within range (72 items) |
| Hooks count | claim 37 .hook.ts/.sh | ✓ within drift (38 actual; PR's snapshot ~37) |
| Skills count | claim 45 directories | ✓ EXACT (45) |
| MEMORY/ subdirs | claim "16 typed directories" | ⚠ MAGNITUDE-1-OFF (15 source-checked-in; LEARNING/ runtime-created at first SatisfactionCapture/WorkCompletionLearning fire) |
| Algorithm versions | claim v5.7.0–v6.3.0 in dir | ✓ EXACT (LATEST + v5.7.0 + v6.0.0 + v6.1.0 + v6.2.0 + v6.3.0 + supporting docs) |
| Algorithm v6.3.0 size | claim 46KB | ✓ EXACT (46068 bytes) |
| Inference.ts existence | claim canonical LLM interface | ✓ EXACT (18125 bytes, exists) |
| ContextReduction.hook.sh | claim Bash interception via RTK | ✓ EXACT (11866 bytes, exists, .sh shell-script as PR specified) |
| SatisfactionCapture.hook.ts | claim in `hooks/` (not PAI/TOOLS/) | ✓ EXACT (18277 bytes in hooks/) |
| IsaFormat.md size | claim 33KB | ✓ EXACT (33322 bytes) |
| Releases v5.0.0 + v4.0.3 | both published | ✓ EXACT (both present alongside earlier versions) |
| Packs/ | claim 50+ category dirs | ✓ within range (52 dirs) |
| Main commit ref | claim `9fb9c86` | ✓ EXACT SHA (PR's date 2026-04-30 vs current commit date 2026-05-02 — minor metadata drift; SHA is the load-bearing reference) |

**Net:** 14 quantitative claims verified — 13 EXACT/within-drift + 1
MAGNITUDE-1-OFF (LEARNING/ subdir is runtime-created not source-checked-in).
The "16 typed directories" claim is direction-accurate (multi-tier typed
memory IS architecturally present) but count-overstated by 1 because
LEARNING/ doesn't exist in the source release until the first hook write.
Methodologically the verification result joins the calibration-first cluster
with cycle-97 / cycle-98 / cycle-99 — PR #2875's author counted file sizes
and queried metadata before producing the survey, with the single direction-
accurate / magnitude-1-off claim being a runtime-vs-source-state distinction
rather than a fabrication.

## Sources read so far

- Cycle 14 (README-only): `README.md` extensive read; 16-principle list extracted
- Cycle 71 dispatch (code-level deeper-read), cycle 100 absorbed:
  - `Releases/v5.0.0/.claude/PAI/ALGORITHM/v6.3.0.md` (46KB doctrine document)
  - `Releases/v5.0.0/.claude/PAI/MEMORY/README.md` (memory architecture overview)
  - `Releases/v5.0.0/.claude/PAI/DOCUMENTATION/ARCHITECTURE_SUMMARY.md` (auto-generated)
  - `Releases/v5.0.0/.claude/PAI/DOCUMENTATION/IsaFormat.md` (33KB ISA spec)
  - `Releases/v5.0.0/.claude/PAI/TOOLS/` directory listing (~72 TypeScript tools)
  - `Releases/v5.0.0/.claude/PAI/TOOLS/Inference.ts` (canonical LLM interface)
  - `Releases/v5.0.0/.claude/hooks/` directory listing (38 .hook.ts/.sh files)
  - Hook code samples: `PromptProcessing.hook.ts`, `SatisfactionCapture.hook.ts`,
    `ContextReduction.hook.sh`, `WorkCompletionLearning.hook.ts`,
    `CheckpointPerISC.hook.ts`, `ISASync.hook.ts`, `DocIntegrity.hook.ts`,
    `SecurityPipeline.hook.ts`, `ContainmentGuard.hook.ts`
  - `Releases/v5.0.0/.claude/skills/` directory listing (45 named skills)
  - `Releases/v5.0.0/.claude/CLAUDE.md` (operational rules + format templates)
  - `Releases/v5.0.0/.claude/PAI/ALGORITHM/optimize-loop.md` (Optimize Loop Protocol v2)
  - `Releases/v5.0.0/.claude/PAI/ALGORITHM/eval-guide.md`
  - `Releases/v5.0.0/.claude/PAI/ALGORITHM/capabilities.md` (closed enumeration)
  - `Packs/` directory listing (52 category packs)
  - Top-level `Tools/` (2 .ts files only — not the load-bearing substrate)
  - GitHub API metadata + directory counts for verification

## Project framing

PAI v5.0.0 is **a `.claude/` directory you copy into `~/.claude/` to
install a Life Operating System for a single user.** The entire system
— skills, hooks, tools, memory, algorithm — lives inside `.claude/PAI/`
after install. The repo's top-level structure is primarily a distribution
vehicle (`Releases/v5.0.0/` deployable snapshot, `Releases/v4.0.3/` prior
stable, `Packs/` distributable skill packs, `README.md` public-facing).

The cycle-14 stub described PAI as "TypeScript/Bun project with top-level
`Tools/`, `Packs/`, `.claude/`, `Releases/`." The deeper read substantially
corrects this: the load-bearing capability substrate is `.claude/PAI/TOOLS/`
(72 TypeScript CLI tools), `.claude/hooks/` (38 hook files), `.claude/skills/`
(45 skill directories), `.claude/PAI/ALGORITHM/` (versioned doctrine),
`.claude/PAI/MEMORY/` (multi-tier typed memory). The top-level `Tools/`
has only 2 .ts source files (`BackupRestore.ts`, `validate-protected.ts`)
— a backup utility plus a path-protection validator, not the Tools layer
referenced by Principle 6 ("Code Before Prompts").

**Version note:** v5.0.0 is substantially larger and architecturally
different from v4.0.3 (the stable release the cycle-14 read reflected):

- Algorithm v3.5.0 (v4.0.3) → **v6.3.0 (v5.0.0)**: substantially more
  codified, ISA system, effort tiers E1-E5, closed capability enumeration,
  HARD gates between phases
- 63 skills (v4.0.3) → **45 skills** (v5.0.0): consolidation, not
  reduction of capability
- 21 hooks (v4.0.3) → **38 hooks** (v5.0.0): roughly doubled, covering
  more lifecycle events
- "AI scaffolding with 16 principles" framing → **"Life Operating System
  with DA + Pulse + Algorithm as gravitational center"** framing
- Principle 1 revised: "User Centricity" → "PAI is the Life Operating
  System"
- New Principle 17: "Science as Cognitive Loop" (Hypothesize→Observe→Learn
  built into session lifecycle via SatisfactionCapture + ISA Changelog
  conjecture/refutation/learning format)

Both the cycle-14 stub framing and the v5.0.0 framing are accurate
snapshots at their respective version points; v5.0.0 has substantially
codified what was previously stated as goals at v4.0.3 (16 of 16
principles persist; many are now backed by hook-level enforcement that
didn't exist at v4.0.3).

**This is the deep-dive 9th system in the corpus.** Cycle-100 absorption
elevates PAI from cycle-14 README-only stub to deep-dive parity, joining
the existing 8 deep-dive systems (openclaw, AutoGen, LangGraph, Voyager,
Cognition Devin, OpenAI harness, Symphony first-pass, oh-my-claudecode
first-pass) — Symphony and oh-my-claudecode remain at first-pass status;
PAI moves to full deep-dive at cycle 100 because the cycle-71 dispatch
covered code-level reading of every load-bearing surface.

## Hypothesis verdicts (cycle-71 dispatch H1 / H2 / H3)

The cycle-71 dispatch tested three hypotheses against PAI v5.0.0 code:

### H1: Cluster J (semantic retrieval as architectural concern) — **REFUTED**

PAI's Memory System does NOT implement semantic retrieval. The two named
retrieval mechanisms in `ARCHITECTURE_SUMMARY.md` are:

- `Tools/MemoryRetriever.ts` — **BM25 keyword-frequency ranking** (Okapi BM25)
- `Tools/KnowledgeGraph.ts` — graph navigation (read-only)

The Algorithm v6.3.0 THINK phase's primary "Knowledge check" uses plain
ripgrep:

```bash
rg -i "TOPIC" ~/.claude/PAI/MEMORY/KNOWLEDGE/ --type md -l
```

PAI README explicitly states the anti-RAG position: *"Filesystem as
context, no RAG — PAI has avoided RAG since June 2025. Rich text with
cross-references, plus fast search like ripgrep, gives us everything
people normally want from RAG."*

**Cluster J does not emerge from PAI evidence.** PAI's memory IS
architecturally interesting (multi-tier typed storage with promotion
ladder; see cluster B augmentation below) but for different reasons
than what cluster J would require.

### H2: Cluster H (Learn → Improve closure as code, not slide-deck) — **CONFIRMED**

PAI implements the Learn → Improve closure as **two production-grade
hooks** that fire on every relevant event:

- **`WorkCompletionLearning.hook.ts`** (SessionEnd) — reads ISA frontmatter,
  extracts ISC pass/fail counts, writes structured Markdown learning
  artifact to `MEMORY/LEARNING/<category>/<YYYY-MM>/<datetime>_work_<slug>.md`.
  EVERY significant session produces a learning artifact automatically.
- **`SatisfactionCapture.hook.ts`** (UserPromptSubmit) — runs Sonnet LLM
  inference on every user prompt against the prior response + recent
  conversation context to infer a 1-10 satisfaction score. Detected
  explicit ratings ("8") fast-path; sentiment inference for everything
  else. Every non-skipped result writes to
  `MEMORY/LEARNING/SIGNALS/ratings.jsonl`. Ratings ≤ 5 trigger structured
  `captureLowRatingLearning()`; ratings ≤ 3 trigger detailed
  `FailureCapture.ts` artifact.

Promotion (LEARNING → KNOWLEDGE) is via `Tools/KnowledgeHarvester.ts`
and `Tools/SessionHarvester.ts --mine`, both explicitly invoked (CLI or
hook), not background-scheduled. Improvement APPLICATION is proposed-for-
review rather than auto-applied: a low rating writes a learning artifact;
it does NOT automatically rewrite the hook or skill that produced the
failure. The DA surfaces relevant learnings in future sessions when
context matches.

**Cluster H upgrades to 5-system convergent** (openclaw / Cognition
Devin / OpenAI harness / Voyager / PAI). PAI contributes a **NEW sub-shape**:
LLM-inferred implicit satisfaction from follow-up behavior (distinct from
explicit human rating, score gates, background gardening, or capability
accumulation).

### H3: Cluster A (Algorithm as code construct, not prompt framing) — **PARTIALLY CONFIRMED**

The Algorithm v6.3.0 is a 46KB **prose-format prompt document**, not a
state machine in the code sense (no `enum Phase`, no `transition()`
function, no event-driven state machine).

But multiple code artifacts ENFORCE the algorithm phases:

1. **`PromptProcessing.hook.ts`** (UserPromptSubmit) — mode classifier
   that runs on every top-level prompt, calls Sonnet via LLM inference,
   and writes `MODE: MINIMAL|NATIVE|ALGORITHM` + `TIER: E1-E5` (when
   ALGORITHM) into `additionalContext` BEFORE the executor sees the prompt.
   The executor is required to honor this output exactly: *"No regex
   fallback. No model judgment."* (`v6.3.0.md`). Fail-safe: any classifier
   error → ALGORITHM E3.
2. **`ISASync.hook.ts`** (PostToolUse Edit/Write) — when the executor
   edits ISA frontmatter (`phase: <new>`), automatically syncs the phase
   to `work.json` AND updates the Kitty terminal tab. Phase transitions
   are observable and recorded.
3. **`CheckpointPerISC.hook.ts`** — every `[ ]` → `[x]` ISC state change
   fires this hook, which auto-commits to all repos in
   `~/.claude/checkpoint-repos.txt`. Mechanical durability gate at
   ISC-transition granularity.
4. **Voice announcements at every phase transition** — embedded in the
   Algorithm as MANDATORY inline `curl` calls to `http://localhost:31337/notify`.
5. **Tier completeness gates (HARD)** — Algorithm v6.3.0 defines hard
   gates on which sections of the ISA must exist at each effort tier
   (E1 requires Goal+Criteria; E4 requires all twelve sections). The
   `CheckCompleteness` ISA workflow enforces this gate; a miss blocks
   `phase: complete`.
6. **Closed-enumeration capability taxonomy (HARD)** — the 19 thinking
   capabilities are a closed list; phantoms (invented names) are counted
   as CRITICAL FAILURE by an audit gate that fires at OBSERVE→THINK boundary.

**Verdict shape:** Algorithm is more than slide-deck framing (versioned
46KB doctrine + hook-enforced phase boundaries + closed-capability
enumeration with audit) but NOT a state machine in the code sense (no
phase-transition object; LLM reads doctrine and follows it, with hooks
enforcing what they can MEASURE — edit events, phase markers — but not
reasoning quality).

**Cluster A NEW sub-shape:** classifier-mediated mode dispatch
(deterministic routing applied at task ingestion BEFORE the executor
acts, distinct from existing cluster A sub-shapes which are
phase-transition-boundary primitives).

## Patterns observed

### Memory & history

- **Multi-tier typed storage with promotion ladder.** `~/.claude/PAI/MEMORY/`
  organized into 15 typed source-checked-in directories (AUTO, BOOKMARKS,
  DATA, KNOWLEDGE, PAISYSTEMUPDATES, PROJECT, RAW, REFERENCE, RELATIONSHIP,
  RESEARCH, SCRATCHPAD, SKILLS, VERIFICATION, WISDOM, WORK). LEARNING/
  is created at runtime when SatisfactionCapture / WorkCompletionLearning
  hooks fire (16 directories total in operational state). Promotion ladder:
  WORK → LEARNING → KNOWLEDGE; explicit invocation via
  `Tools/KnowledgeHarvester.ts` and `Tools/SessionHarvester.ts --mine`
  (no background continuous harvest scheduler).
- **Plain Markdown substrate.** No database, no vector store, no SQLite,
  no embeddings file anywhere in the MEMORY/ tree. All content is
  Markdown files on disk. **Implementation-verified** at directory-listing
  level.
- **BM25 + ripgrep retrieval.** `Tools/MemoryRetriever.ts` provides BM25
  keyword-frequency ranking; THINK phase uses `rg -i "TOPIC"` directly.
  Explicit anti-RAG architectural choice with documented justification
  ("Rich text with cross-references plus fast search gives us everything
  people normally want from RAG").
- **Append-mostly write discipline.** New content writes to LEARNING/,
  WORK/, etc. Promotion to KNOWLEDGE/ requires explicit harvesting steps.
  No automatic background harvest against a score gate (in contrast to
  openclaw's score-gated promotion).
- **No sync invariant guard at boot.** No `MEMORY/` integrity check at
  startup analogous to Voyager's I-V4 (vectordb-count vs JSON-manifest-count
  fail-fast). CheckpointPerISC auto-commits ISA.md edits providing
  durability but not cross-store sync invariant.
- **Per-user, global scoping.** All sessions accumulate into the same
  memory tree (`~/.claude/PAI/MEMORY/`). Task-scoped subdivision via
  `WORK/{slug}` pattern but no per-session isolation at the top level.

### Algorithm & cycle-internal phasing

- **Versioned 46KB doctrine document at `ALGORITHM/v6.3.0.md`** with
  LATEST file pointing to `6.3.0`. Seven phases: OBSERVE → THINK → PLAN
  → BUILD → EXECUTE → VERIFY → LEARN. Algorithm is read by the LLM
  as doctrine; not a code state machine.
- **Effort-tier system E1-E5** with HARD section-completeness gates
  per tier (E1: Goal+Criteria; E4: all 12 sections). The
  `CheckCompleteness` workflow enforces gate; miss blocks `phase: complete`.
- **Classifier-mediated mode dispatch** (NEW sub-shape per H3 verdict)
  — `PromptProcessing.hook.ts` deterministically routes every top-level
  prompt to MINIMAL / NATIVE / ALGORITHM mode with effort tier E1-E5
  before executor sees it.
- **Hook-enforced phase boundaries** — ISASync (PostToolUse), CheckpointPerISC
  (ISC transitions), voice announcements (MANDATORY curl), tier completeness
  gates (HARD). Hooks enforce what they can MEASURE; reasoning quality
  is not gate-checked.
- **Closed-enumeration thinking-capability taxonomy** (19 names; phantoms
  are CRITICAL FAILURE counted by audit gate at OBSERVE→THINK boundary).
  Vocabulary discipline applied to AI capability selection.

### Learn → Improve closure

- **`WorkCompletionLearning.hook.ts`** (SessionEnd) — automatic structured
  learning artifacts for every session. **Implementation-verified.**
- **`SatisfactionCapture.hook.ts`** (UserPromptSubmit) — LLM-inferred
  implicit satisfaction on every prompt; ratings ≤ 5 → captureLowRatingLearning;
  ratings ≤ 3 → FailureCapture.ts. **Implementation-verified.**
- **Promotion (LEARNING → KNOWLEDGE)** — explicit invocation via
  `KnowledgeHarvester.ts` / `SessionHarvester.ts --mine`. No background
  scheduler.
- **Improvement APPLICATION** — proposed-for-review (DA surfaces relevant
  learnings in future sessions when context matches), not auto-applied
  rewriting of hooks/skills.
- **Optimize Loop Protocol v2** — `mode: optimize` formal protocol with
  metric-mode (shell number extraction) and eval-mode (LLM grader rubric)
  for prompts/skills/agents. Explicitly invoked, not background-triggered.

### Skills & agents (stratification)

- **45 named skill directories** at `.claude/skills/`. Each contains
  `SKILL.md` (front door), `Workflows/*.md` (named workflow files),
  `Tools/*.ts` (skill-specific TypeScript CLI tools where applicable).
  Invocation: `Skill("Name", "verb ...")` — explicit named invocation
  by the AI, not automatic keyword detection.
- **Filesystem-discovered registration.** No manifest, no code annotation,
  no centralized registry. `skills/CLAUDE.md` provides routing-context
  table loaded at session start via `LoadContext.hook.ts` `@`-import.
  `USER/SKILLCUSTOMIZATIONS/` allows per-user overrides without modifying
  shared skill directory.
- **Skill self-scaffolding** via `CreateSkill` skill — PAI can extend
  itself by creating new skill directories.
- **Subagent-type taxonomy via `Agent(subagent_type=...)`:**
  - **Forge** — GPT-5.4 via `codex exec` at high reasoning effort,
    quality + completeness specialization
  - **Anvil** — Kimi K2.6, whole-project context work
  - **Cato** — GPT-5.4 in read-only sandbox (`codex exec --sandbox read-only`),
    cross-vendor audit
  - **Engineer** — Claude-family general coding
- **DA (Digital Assistant) personalization** — single named AI persona
  per user; `USER/DA_IDENTITY.md` provides `{{DA_FULL_NAME}}` and
  `{{PRINCIPAL_NAME}}` substitution. Replaced the cycle-14 "multiple agent
  personalities" framing.
- **`capabilities.md`** defines closed enumeration of thinking capabilities
  (19) and delegation capabilities (Forge/Anvil/Cato/...) — formal
  capability taxonomy replacing the old voice-personality system.

### Hooks & lifecycle

- **38 hooks at `.claude/hooks/`** covering all four Claude Code events:
  UserPromptSubmit (8 hooks: PromptProcessing, SatisfactionCapture,
  RepeatDetection, PromptGuard, SecurityPipeline, TaskGovernance,
  SetQuestionTab, LoadContext); PreToolUse (5 hooks: ContextReduction,
  ContainmentGuard, SmartApprover, ContentScanner, AgentInvocation);
  PostToolUse (7 hooks: ISASync, CheckpointPerISC, KVSync,
  ToolActivityTracker, FileChanged, QuestionAnswered, RelationshipMemory);
  Stop (7 hooks: WorkCompletionLearning, SessionCleanup, VoiceCompletion,
  DocIntegrity, StopFailureHandler, UpdateCounts, TelosSummarySync).
- **`ContextReduction.hook.sh`** (PreToolUse Bash) — intercepts ALL Bash
  tool calls, rewrites output through RTK compression for "60-90% token
  reduction." Automatic, invisible optimization on every tool call.
- **`SecurityPipeline.hook.ts`** — runs Pattern, Egress, Rules, Prompt,
  Injection inspectors before dangerous operations reach execution.
  Substrate is single-user personal assistant (different threat model
  from openclaw's multi-actor) but implementation pattern is the same
  hook-enforced blocking before execution.
- **`DocIntegrity.hook.ts`** (Stop) — runs `DocCrossRefIntegrity.ts` +
  `RebuildArchSummary.ts` at every session end. ARCHITECTURE_SUMMARY.md
  is auto-generated from authoritative `PAISystemArchitecture.md`, not
  hand-maintained. Documentation-honesty at architectural level.
- **`settings.json`** binds hook files to Claude Code events. Without
  an entry, a hook file doesn't activate regardless of content.

### Tools layer (deterministic substrate)

- **72 TypeScript CLI tools at `.claude/PAI/TOOLS/`**, all invoked via
  `bun` CLI. Universal pattern: `bun ~/.claude/PAI/TOOLS/ToolName.ts [args]`.
  No tool uses Python.
- **`Inference.ts`** is the single LLM interface for all PAI tools and
  hooks. CLAUDE.md rule enforces: *"Use `bun TOOLS/Inference.ts fast|standard|smart`,
  never import `@anthropic-ai/sdk` directly."* Three tiers: fast (cheap,
  classification/sentiment), standard (default), smart (reasoning-intensive).
  `--mode advisor` flag for commitment-boundary second-opinion check.
- **Cross-cutting tools by category:** Algorithm/session orchestration
  (algorithm.ts, AlgorithmPhaseReport, SessionProgress, PipelineOrchestrator);
  Memory/Knowledge (MemoryRetriever, KnowledgeGraph, KnowledgeHarvester,
  LearningPatternSynthesis, WisdomCrossFrameSynthesizer); Identity/Relationship;
  Content/Media processing; Agent/Delegation (CrossVendorAudit, ForgeProgress);
  ISA/Spec management (Checkpoint, CheckpointPerISC, FeatureRegistry);
  Observability (HealthSnapshot, DocCheck, IntegrityMaintenance, SecretScan,
  CostTracker); User profiling/Telos; Notification/Dashboard.

### Documentation honesty

- **`DocIntegrity.hook.ts`** auto-derives `ARCHITECTURE_SUMMARY.md` at
  every session end via `ArchitectureSummaryGenerator.ts` from authoritative
  `PAISystemArchitecture.md`. The summary cannot drift from the source
  because it's derived, not maintained. **Mechanical doc-honesty primitive.**
- **`FailureCapture.ts`** — durable failure-as-recorded-artifact pattern;
  ratings ≤ 3 trigger structured Markdown failure record with YAML
  frontmatter (`capture_type`, `rating`, `source`, `auto_captured`,
  `tags`). Comparable to Voyager's `failed_tasks.json` (cluster D
  sub-shape).
- **No "I don't know" as structured return type** — IS a constitutional
  rule in `PAI_SYSTEM_PROMPT.md` (the system prompt explicitly grants
  permission to express uncertainty) but not a code-level type or
  exception class.
- **No "What We Will Not Merge" anti-pattern catalog** analogous to
  openclaw's VISION.md. Closest equivalents: CLAUDE.md "Never" rules
  (never npm, never Python without approval, never raw SDK, never nested
  claude subprocess); ContainmentGuard.hook.ts file-system blocks;
  PATTERNS.yaml in USER/SECURITY/. These are enforcement-layer
  prohibitions, not a documented "things we've decided not to do with
  reasoning."
- **No aspirational-vs-implemented markings** — no README sections
  marked "aspirational; not yet implemented." The auto-generated
  ARCHITECTURE_SUMMARY.md disclaimer "Auto-generated by
  ArchitectureSummaryGenerator.ts. Do not edit manually" is a different
  form of doc honesty (derived authoritative source).

## Cluster framework anchoring (cluster A through cluster J)

| Cluster | PAI evidence | Verdict |
|---|---|---|
| **A (cycle-internal phasing primitives)** | Algorithm v6.3.0 seven phases with HARD gates + PromptProcessing.hook.ts classifier-mediated mode dispatch + tier completeness gates + closed thinking-capability enumeration with phantom-as-CRITICAL-FAILURE audit | **NEW SUB-SHAPE: classifier-mediated task dispatch** (deterministic routing applied at task ingestion BEFORE executor acts; distinct from existing phase-transition-boundary sub-shapes) |
| **B (storage-architecture stratification)** | Multi-tier 15-source + 1-runtime MEMORY/ directories; append-mostly write; WORK→LEARNING→KNOWLEDGE promotion ladder; BM25 retrieval (no semantic) | AUGMENTS: multi-mechanism-per-coordinate sub-shape confirmed; semantic-retrieval sub-shape NOT confirmed (H1 refuted) |
| **C (termination/failure modes)** | CheckpointPerISC auto-commits on ISC transitions (granular durability gate at ISC level — finer-grained than session-level); fail-safe on classifier timeout (ALGORITHM E3 default); StopFailureHandler hook | POTENTIAL: granular ISC-transition-level durability gate as new sub-shape candidate |
| **D (documentation honesty)** | FailureCapture.ts for durable failure artifacts; DocIntegrity.hook.ts auto-derives architecture summary; no aspirational markings found; no anti-pattern catalog with non-permanence caveat | AUGMENTS: failure-as-recorded-artifact (FailureCapture.ts) + doc-honesty-via-derivation (ARCHITECTURE_SUMMARY auto-generation) sub-shapes |
| **E** | (cluster E reserved; PAI evidence not strongly cluster-E) | NOT EMERGED |
| **F (stratification axes)** | All 8 cluster F axes present: version (PAI 5.0.0 / Algorithm v6.3.0 / Memory v7.6); task-class (MINIMAL/NATIVE/ALGORITHM); capability-tier (E1-E5); terminology (DA/Principal/ISA/ISC); role (thinking vs delegation capabilities); cost-tier (Inference.ts fast/standard/smart); autonomy-mode (optimize-loop metric-mode vs eval-mode; E1 auto vs E4/E5 advisor-required); capability-layer (Hooks vs Skills vs Tools vs Agents) | **STRONG AUGMENT: all 8 axes confirmed; some with sub-axes not in prior corpus (e.g., thinking/delegation capability split as formal two-category taxonomy within a single tier)** |
| **G** | (cluster G reserved; PAI evidence not strongly cluster-G) | NOT EMERGED |
| **H (post-session feedback / continuous improvement)** | SatisfactionCapture.hook.ts implicit rating inference + WorkCompletionLearning.hook.ts SessionEnd artifacts + Optimize Loop Protocol v2 | **CONFIRMS H2: 5-system convergent (openclaw / Cognition Devin / OpenAI harness / Voyager / PAI); NEW SUB-SHAPE: LLM-inferred implicit satisfaction from follow-up behavior** |
| **I (harness-enforced security/policy boundaries)** | SecurityPipeline.hook.ts with multiple inspectors; ContainmentGuard.hook.ts; PromptGuard.hook.ts | AUGMENTS: confirms cluster I extends to single-user personal-assistant substrate (not only cloud multi-actor) |
| **J (semantic-retrieval architecture)** | H1 REFUTED: BM25 retrieval, no vectors, no embeddings, explicit anti-RAG position since June 2025 | **NOT EMERGED** |

**PAI is the strongest cluster-F augmentation candidate in the corpus.**
All 8 axes have PAI instantiations, some with sub-axes not seen in the
prior 5-system cluster F corpus (e.g., thinking/delegation capability
split as a formal two-category taxonomy within a single tier; capability-
layer split as Hooks/Skills/Tools/Agents quad).

## NEW patterns visible only at code level (10)

The cycle-71 deliverable enumerates 10 patterns NOT visible at README/
principle-list depth:

1. **ISA as spec-test-verification unification** — the Ideal State
   Artifact is simultaneously: spec (Problem/Vision/Criteria), test
   harness (each ISC is one binary tool probe), verification record
   (## Verification section), done condition, and system of record. A
   single artifact covers the entire spec→test→verify lifecycle. No
   parallel acceptance.yaml, no separate test spec needed.
2. **Classifier-mediated mode dispatch** — every top-level prompt
   classified by Sonnet LLM (PromptProcessing.hook.ts) into MINIMAL/NATIVE/
   ALGORITHM with effort tier E1-E5 before executor acts. Executor
   prohibited from overriding without explicit `/e1`-`/e5` flag or
   conversation-context justification. Fail-safe: classifier error →
   ALGORITHM E3.
3. **Closed-enumeration capability taxonomy** — Algorithm v6.3.0 has a
   hard-closed list of 19 named "thinking capabilities" (IterativeDepth,
   ApertureOscillation, FirstPrinciples, SystemsThinking, etc.) plus
   delegation capabilities (Forge, Anvil, Cato). Inventing a name not on
   the list is CRITICAL FAILURE counted by audit gate. Vocabulary
   discipline applied to AI capability selection.
4. **BM25 over RAG — explicit architectural choice** — memory retrieval
   via BM25 keyword ranking and ripgrep, with explicit documentation
   that PAI rejected RAG since June 2025. Justification: *"Rich text
   with cross-references plus fast search like ripgrep gives us
   everything people normally want from RAG."* Deliberate anti-complexity
   stance with documented reasoning.
5. **LLM-inferred implicit satisfaction signal** — SatisfactionCapture
   infers satisfaction rating on EVERY user prompt using a second LLM
   call (Sonnet fast-tier), not just explicit ratings. Prompt includes
   prior response + recent conversation + current prompt. Every non-trivial
   interaction generates 1-10 satisfaction signal regardless of whether
   user rates anything. Neutral = 5, never null.
6. **Cross-vendor auditing (Cato pattern)** — at E4/E5 effort, Algorithm
   spawns Cato (GPT-5.4 in `codex exec --sandbox read-only`) to audit
   primary Claude model's work. Structural architecture for catching
   Anthropic-model blind spots using a competing model as auditor. Audit
   blocks `phase: complete` if any `critical` finding returned.
7. **DocIntegrity auto-derivation** — `ARCHITECTURE_SUMMARY.md` auto-
   generated at every session end from authoritative
   `PAISystemArchitecture.md` by `ArchitectureSummaryGenerator.ts`.
   Documentation cannot drift from source because it's derived, not
   maintained.
8. **Context compression as a hook** — `ContextReduction.hook.sh`
   intercepts ALL Bash tool calls and compresses output through RTK for
   60-90% token reduction. Automatic invisible optimization on every
   tool call; AI never sees uncompressed Bash output.
9. **CheckpointPerISC durability** — every `[ ]` → `[x]` ISC state
   change triggers auto-commit to all repos in `checkpoint-repos.txt`.
   Combined with ISA as system-of-record, every step of progress is
   durably committed in git. No work lost if session crashes mid-Algorithm.
10. **Voice-as-phase-announcement primitive** — algorithm phase transitions
    announced via voice (ElevenLabs TTS through Pulse REST endpoint at
    `localhost:31337/notify`) as MANDATORY action. Voice announcement is
    human-observable signal that phase transition happened; ISA frontmatter
    edit is machine-observable signal. Both required.

When consolidated to genuinely-distinct architectural axes (per cycle-98
honest-reflection discipline), these 10 patterns collapse to **~6
distinct shapes**:

(a) **Spec-test-verification unification** [#1 ISA + #9 CheckpointPerISC,
    both manifestations of single-artifact-as-system-of-record];
(b) **Classifier-mediated dispatch with fail-safe** [#2 mode classifier
    + #6 Cato cross-vendor audit, both deterministic-gate-around-probabilistic-
    behavior at distinct lifecycle points];
(c) **Closed-enumeration vocabulary discipline** [#3 thinking-capability
    taxonomy with phantom-as-failure];
(d) **Anti-complexity architectural choice with documented justification**
    [#4 BM25-over-RAG];
(e) **Mechanical doc-honesty primitives** [#7 DocIntegrity auto-derivation
    + #8 ContextReduction transparent compression, both invisible-side-
    effect-enforcing-honest-state patterns];
(f) **LLM-inferred implicit signal capture** [#5 satisfaction inference
    + #10 voice phase announcements, both signal-emission patterns —
    though #10 is more communication-channel than signal-capture].

Genuinely-novel-vs-corpus contributions: **~5-6 architectural axes**.
NEW sub-shapes #2 (classifier-mediated dispatch) and #5 (LLM-inferred
implicit satisfaction) are the strongest single-system contributions —
both are immediately added to cluster A and cluster H respectively per
the H3 / H2 verdicts.

## 17-principle verdict table

| # | Principle | Verdict | Evidence |
|---|-----------|---------|----------|
| 1 | PAI is the Life Operating System (revised from cycle-14 "User Centricity") | **CONFIRMED** | ARCHITECTURE_SUMMARY.md + PAI_SYSTEM_PROMPT.md; full OS framing with DA, Pulse, Algorithm as gravitational center |
| 2 | The Foundational Algorithm | **CONFIRMED (REFINED)** | Algorithm v6.3.0.md (46KB), seven phases with HARD gates, effort tiers, closed capability enumeration — substantial doctrine, not just framing |
| 3 | Clear Thinking First | **CONFIRMED (REFINED)** | Algorithm's thinking-capability closed enumeration enforces specific thinking discipline; "phantom" thinking names are CRITICAL FAILURE |
| 4 | Scaffolding > Model | **CONFIRMED** | 38 hooks, 72 tools, Inference.ts abstraction, skill architecture — the scaffolding IS the system |
| 5 | Deterministic Infrastructure | **CONFIRMED** | Hooks enforce deterministic events; Inference.ts abstracts LLM calls; BM25 for retrieval (not probabilistic RAG); mode classifier produces deterministic routing |
| 6 | Code Before Prompts | **CONFIRMED** | 72 TypeScript tools in PAI/TOOLS/; CLAUDE.md "bun/bunx always"; `Inference.ts fast\|standard\|smart` instead of raw SDK imports |
| 7 | Spec/Test/Evals First | **PARTIALLY CONFIRMED** | ISA system is strong spec-first mechanism; "evals" are manual/on-demand; **NO automated test suite; no CI test gate** |
| 8 | UNIX Philosophy | **CONFIRMED** | Each tool does one thing (ActivityParser, KnowledgeHarvester, Inference, etc.); all text-interface via bun CLI; no shared state between tools |
| 9 | ENG/SRE Principles | **CONFIRMED** | Observability JSONL logs (mode-classifier.jsonl, ratings.jsonl, OBSERVABILITY/*.jsonl); HealthSnapshot.ts; DocIntegrity auto-checks; CheckpointPerISC durability |
| 10 | CLI as Interface | **CONFIRMED** | All tools invoked via `bun <ToolName>.ts [args]`; no GUI for system operations; Pulse is display-only, not operational |
| 11 | Goal → Code → CLI → Prompts → Agents | **CONFIRMED (ADVISORY)** | Architectural doctrine confirmed; enforced via CLAUDE.md rules and Forge auto-include; **no discriminator function or routing gate** |
| 12 | Custom Skill Management | **CONFIRMED** | 45 skills in `.claude/skills/`; SKILL.md front door pattern; Workflows/*.md; UserSkillCustomizations overlay; CreateSkill self-scaffolding |
| 13 | Custom Memory System | **CONFIRMED (REFINED)** | Multi-tier typed storage confirmed; **BM25 (NOT semantic)**; rich promotion ladder; 15+1 subdirectories — richer than README implied but different shape from what cluster J would require |
| 14 | Custom Agent Personalities | **CONFIRMED (RESTRUCTURED)** | DA system + named subagent types (Forge/Anvil/Cato/Engineer); old multi-personality framing replaced by capability-specialization taxonomy |
| 15 | Science as Meta-Loop | **CONFIRMED** | SatisfactionCapture's implicit rating inference; WorkCompletionLearning structured capture; optimize-loop.md formal A/B protocol; eval-guide.md |
| 16 | Permission to Fail | **PARTIALLY CONFIRMED** | FailureCapture.ts durable artifacts for ratings ≤3; **no structured "I don't know" return type; no aspirational markings; no anti-pattern catalog with non-permanence caveat** |
| 17 | Science as Cognitive Loop *(new in v5.0.0)* | **CONFIRMED** | SatisfactionCapture + WorkCompletionLearning + ISA Changelog conjecture/refutation/learning format = systematic Hypothesize→Observe→Learn cycle built into session lifecycle |

**Net principle-verdict summary:** 13 CONFIRMED (some with REFINED /
RESTRUCTURED qualifications) + 2 PARTIALLY CONFIRMED (#7 Spec/Test/Evals
First — no automated test suite; #16 Permission to Fail — no structured
"I don't know" return type, no aspirational markings, no anti-pattern
catalog) + 0 REFUTED. The PARTIALLY CONFIRMED principles indicate that
even PAI's "Code Before Prompts" stance has limits: the most rigorous
test/evals discipline still leaves automated regression testing as a gap.

## Anchoring caveats

- **PAI is a personal assistant; we are an autonomous orchestrator.**
  PAI's Principal provides goals from outside the system. Our system
  has no human-provided goal stream beyond Eva's occasional input-from-
  eva — the orchestrator generates its own next-cycle work from prior
  cycles. PAI's tight-loop satisfaction capture (every UserPromptSubmit)
  is structurally inapplicable — we have no per-prompt user feedback
  signal. PAI's TELOS/ system (mission/goals/challenges) has no analog
  in our design.
- **Single-user vs. multi-issue pipeline.** PAI manages a single user's
  life across multiple domains simultaneously (health, finances, work,
  relationships). Our system manages a single project's issue pipeline
  sequentially. PAI's stratification by domain maps loosely to our
  stratification by issue-type (PRD/tool/schema/audit) but the scoping
  model differs.
- **Claude Code as the substrate.** PAI is built entirely on Claude
  Code as the execution environment. Our system uses GitHub Actions +
  Claude API. The hook system, CLAUDE.md operational rules, and skill
  invocation patterns are Claude Code-specific. Direct transfer of the
  hook architecture requires Claude Code adoption; adapting the patterns
  (e.g., hook-equivalent pre-processing) would require equivalent
  infrastructure.
- **v5.0.0 is substantially different from v4.0.3** (cycle-14 baseline).
  v5.0.0 has substantially codified what was stated as goals at v4.0.3
  (Algorithm v3.5.0 → v6.3.0; 21 hooks → 38 hooks; "AI scaffolding" →
  "Life Operating System"). Both are accurate snapshots; v5.0.0 is the
  current development branch.
- **No automated test suite** — `.github/workflows/` contains only Claude
  PR review + Claude Code integration (not test execution). No unit tests,
  no integration tests, no coverage reporting, no CI test gate before
  merge. The ISA system is the closest analog but instance-specific (each
  Algorithm run has its own ISA; no shared test suite for PAI's own code).
  This is a significant honesty gap in Principle 7 ("Spec/Test/Evals
  First") — the principle name overpromises relative to what code delivers.
- **MEMORY/ count discrepancy.** PR claims "16 typed directories" but
  source release has 15 source-checked-in directories; LEARNING/ is
  runtime-created. Direction-accurate (multi-tier typed memory IS present)
  but count-overstated by 1. A small precision issue but worth noting
  per cycle-99 direction-vs-magnitude discipline; substantive claim
  unaffected.
- **Confirmation-bias risk flagged in cycle-14** — the cycle-14 read
  warned that "PAI principles' alignment with our CORE-DESIGN-PRINCIPLE
  is striking and concerning." Code-level reading produces a more nuanced
  picture: principles 4/5/6/8/11 ARE architecturally instantiated (hooks,
  TOOLS/, Inference.ts, CLAUDE.md rules, bun-only/TypeScript-only mandates),
  not just marketing surface. But other principles (#7 Spec/Test/Evals
  First; #16 Permission to Fail) are partially confirmed — the alignment
  is real but not 100% across the principle list. The transfer-relevant
  patterns (skill architecture, hook lifecycle, Inference.ts abstraction,
  ISA/ISC system) are genuine architectural contributions; the non-transfer
  findings (no test suite, no score-gated promotion, no semantic retrieval,
  single-user model) are equally genuine architectural differences.
- **Memory shape mismatch.** PAI's "memory" is what *the user* did /
  wanted, plus what the DA learned about the user. Our "persistence" is
  what *the orchestrator* decided / is investigating across cycles. Both
  are multi-session memory but the shape differs substantially. PAI
  patterns transferable: multi-tier typed storage, append-mostly write,
  promotion ladder, plain Markdown substrate, BM25-over-RAG choice
  rationale. Not transferable: per-prompt satisfaction signal, user-
  identity layer, relationship-memory shape.

## To-be-completed (deeper-read queue / open questions)

The cycle-71 dispatch covered all 9 lens areas but flagged several
items for deeper investigation if PAI moves to "primary-reference"
status (rather than corpus-evidence-base status):

1. **PAI v4.0.3 vs v5.0.0 architectural-evolution analysis** — what
   prompted the doubling of hooks, the Algorithm v3.5.0 → v6.3.0 jump,
   the introduction of Pulse and the Life Operating System framing?
   Reading the changelog/release notes between v4.0.3 and v5.0.0 would
   identify which architectural changes were driven by which observed
   limitations. Useful for understanding which extensions are convergent
   evolution vs. PAI-author-specific design choices.
2. **Optimize Loop Protocol v2 in detail** — `optimize-loop.md` defines
   metric-mode and eval-mode for prompts/skills/agents. Tracing the
   actual A/B comparison mechanics (how mutations are generated, how
   scoring rolls up, how recommendations are surfaced for review) would
   inform our design of an analogous "self-improvement loop" for the
   redesign target. Currently understood at protocol-level; mechanism-
   level reading would close the loop.
3. **Cato cross-vendor audit failure semantics** — Cato (GPT-5.4 in
   read-only sandbox) audits primary Claude work at E4/E5. The audit
   blocks `phase: complete` on `critical` findings. Tracing how the
   "primary vs auditor disagreement" escalation protocol works — the
   conflict-surfacing rule, the threshold for auditor override, the
   how-many-times-can-auditor-block-before-human-loop — would inform
   our audit-as-peer integration.
4. **PromptProcessing.hook.ts mode classifier accuracy** — the classifier
   is a Sonnet LLM call producing MODE/TIER/REASON/SOURCE. Empirical data
   on classifier accuracy (false-positives, false-negatives, fail-safe
   trigger rate) would validate the deterministic-routing-around-
   probabilistic-LLM pattern. Currently understood at design-level;
   accuracy-level data would close it.
5. **CheckpointPerISC durability under concurrent ISC transitions** —
   if multiple ISCs flip in quick succession, the auto-commit hook fires
   multiple times. How are concurrent commits handled? Is there a
   queue/serialization mechanism, or does the underlying git command
   handle race conditions? Relevant to our git-safety primitive (every
   commit MUST be pushed in same operation per cycle 524 fix).
6. **`DocCrossRefIntegrity.ts` mechanism** — DocIntegrity.hook.ts auto-
   checks doc cross-references at session end. Tracing how the cross-ref
   graph is built and what counts as a broken reference would inform
   our own approach to keeping documentation self-consistent across
   cycles (currently a manual discipline; PAI mechanizes it).
7. **`Tools/MemoryRetriever.ts` BM25 implementation** — what's the
   token-frequency normalization? Stop-word handling? Field-weighting
   (does title rank higher than body)? Relevant if we choose to adopt
   BM25-over-RAG ourselves; the choice rationale is documented but the
   implementation details are deferred.

PAI is now the **9th deep-dive system** in the corpus. A second-pass
deeper-read on any of the above seven items would deepen the per-system
file further, but the cycle-100 absorption establishes deep-dive parity
with the existing 8-system catalogue (openclaw, AutoGen, LangGraph,
Voyager, Cognition Devin, OpenAI harness, plus PAI). Symphony and
oh-my-claudecode remain at first-pass depth pending their own deeper-
reads (queued for future cycles).

## Cycle-100 absorption + integration record

- **Per-system file (this file)**: rewritten from 84-line cycle-14
  README-only stub to 9th deep-dive system in the corpus, following
  cycle-98 / cycle-99 per-system file shape (verification banner +
  sources read + project framing + hypothesis verdicts + patterns
  organized into family buckets + cluster framework anchoring + NEW
  patterns with consolidation discipline + 17-principle verification
  table + anchoring caveats + deeper-read queue).
- **Cross-system observations** (in `1-research.md`): cluster H elevated
  to 5-system convergent; cluster A receives NEW sub-shape "classifier-
  mediated task dispatch"; cluster F receives augmentation note (PAI
  is strongest cluster-F instantiation candidate, all 8 axes confirmed,
  some with sub-axes not in prior corpus); cluster B receives "BM25-over-
  RAG" architectural-choice annotation; cluster J refuted as emergent
  cluster (no semantic retrieval architectural concern).
- **NEW pattern consolidation discipline applied:** 10 NEW patterns
  honestly consolidated to ~5-6 distinct architectural axes per cycle-98
  honest-reflection discipline.
- **Sister-pattern tracking:** Functional-class shape #19 (research-
  deliverable-absorption-with-verification-success) extends to TESTED
  at 3 instances → HARDENED (cycle 98 Symphony / cycle 99 omc / cycle 100
  PAI deeper). Verification-discipline pattern extends to 5 instances
  HARDENED (cycle 96 catch / 97 verify / 98 verify / 99 verify / 100
  verify).
- **Direction-vs-magnitude discipline applied:** PR's "16 typed
  directories" claim is direction-accurate magnitude-1-off (LEARNING/
  runtime-created not source-checked-in). Recorded as a small precision
  note rather than an absorption blocker.
