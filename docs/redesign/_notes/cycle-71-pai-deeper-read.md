# Cycle 71 — PAI Deeper Read (danielmiessler/Personal_AI_Infrastructure)

**Date:** 2026-05-08
**Dispatch issue:** [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842)
**Baseline:** `docs/redesign/1-research/systems/pai.md` (cycle-14 README-only stub, 85 lines)
**Repo read:** `danielmiessler/Personal_AI_Infrastructure` at commit `9fb9c86` (main, 2026-04-30)
**Primary source:** v5.0.0 release in `Releases/v5.0.0/` — the current development branch. v4.0.3 (the cycle-14 baseline) is a prior stable; all code citations below are from v5.0.0 unless noted.

---

## Orientation: What PAI Actually Is (Code-Level Correction of the README Survey)

Before the lens sections: the cycle-14 read described PAI as a TypeScript/Bun project with top-level `Tools/`, `Packs/`, `.claude/`, `Releases/`. Code-level reading substantially changes this picture.

**PAI is not a library you import or a service you run. PAI is a `.claude/` directory you copy into `~/.claude/`.** The entire system — skills, hooks, tools, memory, configuration, algorithm — lives inside `.claude/PAI/` after installation. The top-level repo structure is primarily a distribution vehicle:

- `Releases/v5.0.0/` — the deployable snapshot (copy `.claude` → `~/.claude/` → install)
- `Releases/v4.0.3/` — the prior community-facing stable release
- `Packs/` — distributable skill packs (50+ categories) for optional add-on installation
- `Tools/` (top-level) — only 2 files (`BackupRestore.ts`, `validate-protected.ts`); this is NOT the load-bearing capability substrate the cycle-14 deferred read was expecting
- `README.md` — public-facing documentation

The load-bearing capability substrate is:
- `Releases/v5.0.0/.claude/PAI/TOOLS/` — 70+ TypeScript CLI tools, invoked via `bun`
- `Releases/v5.0.0/.claude/hooks/` — 37 TypeScript hook files
- `Releases/v5.0.0/.claude/skills/` — 45 named skill directories
- `Releases/v5.0.0/.claude/PAI/ALGORITHM/` — algorithm versions (v5.7.0 through v6.3.0)
- `Releases/v5.0.0/.claude/PAI/MEMORY/` — 16+ typed memory tier directories

**Version note:** PAI v5.0.0 is a significantly larger and more architecturally different system than v4.0.3. The cycle-14 read's 16 principles are still recognized in v5.0.0's `ARCHITECTURE_SUMMARY.md` as "Founding Principles" (now 17, including a revised Principle 1 "PAI is the Life Operating System"). The framing has shifted from "AI scaffolding with 16 principles" to "Life Operating System with a DA (Digital Assistant), Pulse (Life Dashboard), and Algorithm as gravitational center." The deeper-read maps both framings.

---

## 1. Memory System Architecture — Code-Level (H1 Test)

### Storage Substrate

PAI v5.0.0 memory lives in `~/.claude/PAI/MEMORY/` after install (`Releases/v5.0.0/.claude/PAI/MEMORY/`). The `README.md` at this path states:

> "MEMORY/ is the persistent state layer of PAI — the place where every session, learning, observation, and artifact accumulates over time. Where KNOWLEDGE/ is curated and USER/ is identity, MEMORY/ is the lived record of what the system has actually done. It is segmented into typed subdirectories so each kind of artifact has a predictable home."
> — `Releases/v5.0.0/.claude/PAI/MEMORY/README.md`

The substrate is **plain Markdown files** organized into 16 typed directories:

| Directory | Purpose |
|-----------|---------|
| `WORK/` | Per-task ISA.md artifacts — one slug-named subdirectory per Algorithm run |
| `LEARNING/` | Session-end learning captures (ALGORITHM/, SYSTEM/ sub-categories) |
| `KNOWLEDGE/` | Curated distillations harvested from LEARNING/ and WORK/ |
| `RESEARCH/` | Research artifacts and notes |
| `REFERENCE/` | Reference docs and external material |
| `WISDOM/` | Cross-frame wisdom synthesis |
| `RELATIONSHIP/` | Contact/interaction memory |
| `DATA/` | Structured personal data |
| `BOOKMARKS/` | Saved URLs and references |
| `RAW/` | Unprocessed inputs |
| `SKILLS/` | Per-skill memory and learning |
| `PROJECT/` | Project-specific memory |
| `SCRATCHPAD/` | Ephemeral scratch |
| `VERIFICATION/` | Verification evidence |
| `AUTO/` | Auto-captured entries |
| `PAISYSTEMUPDATES/` | System change records |

All content is **Markdown files** on disk. No database, no vector store, no SQLite, no embeddings file anywhere in the `MEMORY/` tree.

### Retrieval Mechanism

The `ARCHITECTURE_SUMMARY.md` pipeline table for Memory (`Releases/v5.0.0/.claude/PAI/DOCUMENTATION/ARCHITECTURE_SUMMARY.md`) explicitly names two retrieval tools:

> "Tools/MemoryRetriever.ts (BM25 retrieval), Tools/KnowledgeGraph.ts (graph navigation) — read-only"

**BM25 is a keyword-frequency ranking algorithm** (the same family as classic full-text search, Okapi BM25). It is NOT semantic retrieval. BM25 ranks documents by term frequency and inverse document frequency; it does not compute embedding similarity or any semantic similarity measure.

The Algorithm v6.3.0.md THINK phase's "Knowledge check" step (the primary retrieval trigger) uses a plain `rg` (ripgrep) call:

```bash
rg -i "TOPIC" ~/.claude/PAI/MEMORY/KNOWLEDGE/ --type md -l
```
— `Releases/v5.0.0/.claude/PAI/ALGORITHM/v6.3.0.md` (THINK phase section)

This is **structural search by keyword**, not semantic retrieval.

The PAI README (main repo, not Releases/) explicitly states the anti-RAG position:

> "Filesystem as context, no RAG — PAI has avoided RAG since June 2025. Rich text with cross-references, plus fast search like ripgrep, gives us everything people normally want from RAG."

### Write Discipline

Memory is **append-mostly**: new content is written to LEARNING/, WORK/, etc. The Architecture Summary states "Memory compounds across sessions: WORK → LEARNING → KNOWLEDGE" — this is the promotion ladder. Promotion is not automatic on every write; it requires explicit harvesting steps via `Tools/KnowledgeHarvester.ts` (produces `MEMORY/KNOWLEDGE/`) and `Tools/SessionHarvester.ts --mine` (produces `KNOWLEDGE/_harvest-queue/`). Both are invoked explicitly (CLI or hook); there is no continuous background harvesting running against a score gate.

### Consolidation / Promotion Mechanism

There is a multi-tier promotion ladder (WORK → LEARNING → KNOWLEDGE) but no score-gated promotion analogous to openclaw's. The promotion pathway:

1. Algorithm run produces `MEMORY/WORK/{slug}/ISA.md` with ISC pass/fail record
2. `WorkCompletionLearning.hook.ts` fires at `SessionEnd`: reads ISA frontmatter, extracts ISC count, writes `MEMORY/LEARNING/<category>/<YYYY-MM>/<datetime>_work_<slug>.md`
3. `SatisfactionCapture.hook.ts` fires at `UserPromptSubmit`: LLM-inferred sentiment → `MEMORY/LEARNING/SIGNALS/ratings.jsonl`
4. `Tools/KnowledgeHarvester.ts` and `Tools/SessionHarvester.ts --mine` promote from LEARNING to KNOWLEDGE — explicit invocation required

### Sync Invariant Discipline

No sync-invariant guard analogous to Voyager's I-V4 (vectordb-count vs JSON-manifest-count fail-fast at boot) is present. No `MEMORY/` integrity check at startup. The CheckpointPerISC hook auto-commits ISA.md edits to git repos listed in `checkpoint-repos.txt`, which provides durability but not a cross-store sync invariant.

### Scoping

Memory is scoped **per-user, global** (all lives in `~/.claude/PAI/MEMORY/`). There is task-scoped subdivision via the WORK/{slug} pattern but no per-session isolation at the top level — all sessions accumulate into the same memory tree.

### No Knowledge File / Knowledge API Terminology

PAI does not use "Knowledge File" or "Knowledge API" as named concepts (these are Cognition Devin's framing). PAI uses "KNOWLEDGE/" directory + `KnowledgeGraph.ts` + `KnowledgeHarvester.ts`. The KNOWLEDGE/ directory is the curated distillation layer; the harvest tools are the "API" surface.

### **H1 Verdict: REFUTED**

PAI's Memory System does NOT implement semantic-retrieval as a top-level architectural concern. The system uses BM25 keyword ranking and ripgrep text search — deliberately avoiding RAG/vector retrieval since June 2025. H1 is refuted; cluster B sub-shape assignment for Voyager's I-V6 (semantic retrieval) stands. Cluster J does not emerge from PAI evidence.

The memory architecture IS architecturally interesting but for different reasons: multi-tier typed storage with a defined promotion ladder (WORK → LEARNING → KNOWLEDGE), append-mostly write discipline, and a curated-vs-raw stratification. This maps to cluster B's existing "multi-mechanism-per-coordinate" and "failure-as-first-class-artifact" sub-shapes, not to a new semantic-retrieval cluster.

---

## 2. Foundational Algorithm — Code Construct or Prompt Framing (H3 Test)

### Current Algorithm State

PAI v5.0.0 uses Algorithm v6.3.0, documented in `Releases/v5.0.0/.claude/PAI/ALGORITHM/v6.3.0.md` (46KB). The LATEST file (`Releases/v5.0.0/.claude/PAI/ALGORITHM/LATEST`) contains only `6.3.0` — the executor reads this first to find the current version file.

The old 7-phase cycle-14 read described is now significantly evolved. v6.3.0's phases are:

**OBSERVE (1/7) → THINK (2/7) → PLAN (3/7) → BUILD (4/7) → EXECUTE (5/7) → VERIFY (6/7) → LEARN (7/7)**

This is structurally the same as the cycle-14 Principle 2 statement. But the internal architecture of each phase has become substantially more codified.

### What "Code Construct" Means in PAI's Context

The Algorithm v6.3.0 is a **prose-format prompt document**, not a state machine in the code sense (no `enum Phase`, no `transition(from: Phase, to: Phase)` function, no event-driven state machine). The LLM reads and follows the document as doctrine.

However, multiple code artifacts ENFORCE the algorithm phases:

**1. PromptProcessing.hook.ts (UserPromptSubmit) — mode classifier**
```
MODE: MINIMAL | NATIVE | ALGORITHM   (always present)
TIER: E1 | E2 | E3 | E4 | E5         (present iff MODE=ALGORITHM)
REASON: <one sentence>
SOURCE: classifier | fail-safe
```
This hook runs on every top-level prompt, calls Sonnet via LLM inference, and writes mode/tier into `additionalContext` before the executor sees the prompt. The executor is required to honor this output exactly ("No regex fallback. No model judgment." — `v6.3.0.md`). This is a **deterministic routing gate** before the algorithm runs.

**2. ISASync.hook.ts (PostToolUse Edit/Write) — phase tracking**
When the executor edits ISA frontmatter (`phase: <new>`), ISASync.hook.ts automatically syncs the phase to `work.json` AND updates the Kitty terminal tab. This enforces that phase transitions are observable and recorded.

**3. CheckpointPerISC.hook.ts — ISC transition durability**
Every `[ ]` → `[x]` ISC state change fires this hook, which auto-commits to all repos in `~/.claude/checkpoint-repos.txt`. This is a mechanical durability gate at the execute-level phase granularity.

**4. Voice announcements at every phase transition** — embedded in the Algorithm as mandatory inline `curl` calls to `http://localhost:31337/notify`. Not optional; the algorithm spec marks these as MANDATORY.

**5. Tier completeness gates (HARD)** — Algorithm v6.3.0 defines HARD gates on which sections of the ISA must exist at each effort tier (E1 requires Goal+Criteria; E4 requires all twelve sections). The `CheckCompleteness` ISA skill workflow enforces this gate; a miss blocks `phase: complete`.

**6. Thinking-capability closed enumeration (v6.3.0) — HARD enforcement** — The 19 thinking capabilities are a closed list; phantoms (invented names) are counted as CRITICAL FAILURE by an audit gate that fires at OBSERVE→THINK boundary.

**Is the Algorithm a state machine?** Not formally — there's no code object with state transitions. But it IS a seven-phase procedure with:
- Mandatory entry actions per phase (voice announcement + ISA frontmatter edit)
- Hard gates between phases (ISC completeness gate, tier completeness gate)
- Side-effect hooks that fire on phase-marking edits (ISASync, CheckpointPerISC)
- Fail-safe behavior at classification time (timeout → ALGORITHM E3 with SOURCE: fail-safe)

This is more than "prompt-stated framing." The hooks enforce observable phase boundaries. The mode classifier runs deterministically before the executor.

**What ISN'T implemented as code:** The algorithm reasoning within each phase — the OBSERVE reverse-engineering, the THINK premortem, the PLAN parallelism scan, the BUILD root-cause checkpoint — all of these are prose instructions that the LLM follows by reading the document. There is no code asserting that the executor actually performed a premortem; only that the phase entry voice call happened.

### Verify Implementation

VERIFY phase in v6.3.0 has four distinct rules:
1. **Live-Probe for User-Facing Artifacts** — tool-verified probe mandatory; browser screenshot, `curl`, `bun test`, etc.
2. **Commitment-Boundary Advisor Calls** — `bun ~/.claude/PAI/TOOLS/Inference.ts --mode advisor` before committing approach and after completing work
3. **Cross-Vendor Audit (Cato, E4/E5 only)** — spawn a separate `Agent(subagent_type="Cato")` running GPT-5.4 in read-only sandbox to cross-audit the work
4. **Conflict-Surfacing** — if Cato contradicts Advisor, explicit escalation protocol

The "automated tests" equivalent is the ISC system: ISCs are the test harness, and `bun test` / `curl` / `Grep` invocations are the test runners. No separate automated test suite.

### Learn Implementation

The LEARN phase's Learning Router classifies each candidate learning and routes it by type (workflow improvements → `MEMORY/LEARNING/ALGORITHM/`, system knowledge → `MEMORY/LEARNING/SYSTEM/`, ISA Changelog → Skill("ISA", "append changelog ..."), etc.). The routing is prompting the LLM to make the classification judgment, then invoking the appropriate write tool.

The WorkCompletionLearning.hook.ts provides automatic capture at SessionEnd independent of whether the LEARN phase was explicitly executed — a safety net for the "Learn" step.

### **H3 Verdict: PARTIALLY CONFIRMED**

The Foundational Algorithm (Observe→Think→Plan→Build→Execute→Verify→Learn) is substantively more than a README framing: it is a versioned doctrine document (now at v6.3.0 with detailed per-phase requirements, effort tiers, HARD gates, closed-enumeration capability lists) with hook-level code enforcement of phase boundaries, ISC transitions, and mode routing. The claim "Algorithm is the gravitational center" in ARCHITECTURE_SUMMARY.md is implementation-backed.

However, the algorithm is NOT a state machine in the code sense — it is a prompt document read and followed by the LLM, with hooks enforcing specific observable events (phase transitions, ISC transitions, durability commits). The distinction: if the LLM skips the THINK premortem without a hook knowing, no code prevents it. The hooks enforce what they can measure (edit events, phase markers); they cannot enforce reasoning quality.

Cluster A implication: PAI contributes a sub-shape distinct from existing 9 sub-shapes — **classifier-mediated mode dispatch** (PromptProcessing.hook.ts routing each prompt to MINIMAL/NATIVE/ALGORITHM with Sonnet-inferred tier before the executor acts). This is a deterministic gate applied at task ingestion, not a phase-transition boundary — different from the existing cluster A sub-shapes (super-step semantics, termination predicates, etc.) but still cycle-internal phasing.

---

## 3. Learn → Improve Closure (H2 Test)

### Where Learn Writes

The LEARN phase has two write paths:
1. **ISA Changelog** — `Skill("ISA", "append changelog ...")` writes a conjecture/refutation/learning entry to the ISA's `## Changelog` section. Format: `conjectured`, `refuted_by`, `learned`, `criterion_now`. All four parts required; the Append workflow refuses partial entries.
2. **Learning Router** — classifies candidate learnings and routes to `MEMORY/LEARNING/<category>/`.

The hook-level backup: `WorkCompletionLearning.hook.ts` fires at every SessionEnd regardless of whether the Algorithm's LEARN phase ran explicitly. It reads `MEMORY/STATE/current-work-{session_id}.json`, finds the ISA.md, extracts ISC pass/fail counts, and writes `MEMORY/LEARNING/<category>/<YYYY-MM>/<datetime>_work_<slug>.md`. This means EVERY significant session produces a learning artifact automatically.

### Where Improve Reads

The Improve path reads through the THINK phase's "Knowledge check":
```bash
rg -i "TOPIC" ~/.claude/PAI/MEMORY/KNOWLEDGE/ --type md -l
```
This is where prior learnings that have been harvested to KNOWLEDGE/ become visible to future Algorithm runs. The THINK phase is instructed to search for relevant notes from prior work before starting analysis.

The KnowledgeHarvester and SessionHarvester tools are the explicit "harvest" step that promotes LEARNING → KNOWLEDGE. These are CLI tools invoked on-demand; no automatic background harvest scheduler is visible in the hooks.

### Satisfaction Signal as Feedback Gate

`SatisfactionCapture.hook.ts` fires on UserPromptSubmit — before the executor responds. It:
1. Detects explicit ratings (bare "8" or "eight") — direct capture to `ratings.jsonl`
2. Detects positive praise ("great job", "excellent") — fast-path rating 8
3. For all other prompts: runs Sonnet LLM inference on the current prompt + recent conversation + last response to infer a 1-10 satisfaction score
4. Writes every non-skipped result to `MEMORY/LEARNING/SIGNALS/ratings.jsonl`
5. For ratings < 5: calls `captureLowRatingLearning()` → structured LEARNING artifact
6. For ratings ≤ 3: calls `FailureCapture.ts` → detailed failure record

The sentiment analysis is a real LLM inference call:
```typescript
const result = await inference({
  systemPrompt: buildSatisfactionPrompt(),  // detailed 10-tier scale
  userPrompt: // last response + recent conversation + current message
  expectJson: true,
  timeout: 15000,
  level: 'fast',
});
```
— `Releases/v5.0.0/.claude/hooks/SatisfactionCapture.hook.ts`

This is **automated implicit feedback extraction** — the system uses AI to assess AI performance on every interaction, without requiring the user to explicitly rate anything.

### Is Improve Automated or Proposed?

The Learn → Improve closure in PAI is **semi-automated**:
- Learning capture: fully automated (hooks fire at SessionEnd, SatisfactionCapture fires at every prompt)
- Learning promotion (LEARNING → KNOWLEDGE): semi-automated (requires explicit `KnowledgeHarvester` invocation)
- Improvement application: proposed (the LEARN phase's Learning Router routes learnings to structured files; improvements to prompts/skills/hooks require the Algorithm to be run again with that scope)

There is NO fully automatic "apply improvement to the system" pathway. A low rating writes a learning artifact; it does NOT automatically rewrite the hook or skill that produced the failure. The improvement is proposed-for-review (the DA surfaces it in a future session when relevant context matches).

The optimize-loop.md defines a separate `mode: optimize` protocol for improving prompts/skills/agents — a full sub-loop with metric scoring, mutation, A/B comparison, and recommendation. This is explicitly invoked, not background-triggered.

### New Sub-Shape Distinct From Prior 4

Compared to cluster H's existing 4 sub-shapes:
- Cognition: tight-cycle meta-feedback (human feedback per task with structured scoring)
- openclaw: score-gated promotion (explicit numeric threshold gates advancement)
- OpenAI harness: continuous-background gardening (proactive maintenance without session trigger)
- Voyager: capability-accumulation (code program library that grows with successful programs)

PAI's contribution is: **LLM-inferred implicit satisfaction capture** (automatic extraction of satisfaction signal from follow-up behavior, using a second LLM call to analyze the user's next prompt against the prior response). This is none of the four — it's not structured human feedback, not a score gate, not background-only, and not code-accumulation. It's inference-of-approval from behavioral signal.

### **H2 Verdict: CONFIRMED**

PAI's Learn → Improve closure is implemented in code, with both prompt-level (Algorithm LEARN phase doctrine) and hook-level (WorkCompletionLearning, SatisfactionCapture) implementations. The closure IS automated in capture, semi-automated in promotion, and proposed-for-review in application. Cluster H upgrades to 5-system convergent with PAI as the fifth system, contributing a new sub-shape: **feedback-signal-inference** (LLM-inferred implicit satisfaction from follow-up behavior, distinct from explicit rating or structured scoring).

The cycle-14 claim "PAI closes the Learn → Improve loop while v1 does not" is confirmed by code. PAI writes structured learning artifacts at session end and uses satisfaction signals to identify improvement opportunities. The closure is incomplete in one dimension — improvement application still requires human-loop decision (the DA doesn't automatically rewrite its own skills) — but the capture and routing are mechanically implemented.

---

## 4. Skill Management + Agent Personalities — Stratification Axes

### Skills Architecture

Skills live in `Releases/v5.0.0/.claude/skills/` with 45 named directories in v5.0.0:

```
Agents, ApertureOscillation, Aphorisms, Apify, ArXiv, Art, AudioEditor,
BeCreative, BitterPillEngineering, BrightData, Browser, CLAUDE.md (ignored),
ContextSearch, Council, CreateCLI, CreateSkill, Daemon, Delegation, Evals,
ExtractWisdom, Fabric, FirstPrinciples, ISA, Ideate, Interceptor, Interview,
IterativeDepth, Knowledge, Loop, Migrate, Optimize, PAIUpgrade,
PrivateInvestigator, Prompting, RedTeam, Remotion, Research, RootCauseAnalysis,
Sales, Science, SystemsThinking, Telos, USMetrics, Webdesign, WorldThreatModel,
WriteStory
```

Each skill directory contains:
- `SKILL.md` — canonical front door: what the skill does, activation conditions, workflows list
- `Workflows/*.md` — named workflow files (e.g., `ISA/Workflows/Scaffold.md`, `ISA/Workflows/Append.md`)
- `Tools/*.ts` — TypeScript CLI tools specific to the skill (where applicable)

Invocation pattern: `Skill("Name", "verb ...")` — explicit named invocation by the AI, not automatic keyword detection. The CLAUDE.md context-routing table lists skills with their activation conditions, but the executor calls them by name.

The `CreateSkill` skill provides a self-scaffolding mechanism — PAI can extend itself by creating new skill directories.

### Skill Registration

Skills are **filesystem-discovered** — no manifest file, no code annotation, no centralized registry. The `skills/CLAUDE.md` (a routing map, not a standard CLAUDE.md) provides a table of skill names to paths. LoadContext.hook.ts loads the routing context at session start via `@`-import.

`USER/SKILLCUSTOMIZATIONS/` allows per-user overrides of skill behavior without modifying the shared skill directory.

### Skill Composition

Skills can invoke other skills: the Algorithm's capability-selection mechanism allows any skill invocation within phases. `Skill("ISA")` is invoked by the Algorithm's OBSERVE phase, which can in turn invoke `Skill("Evals")` or `Skill("Council")`. No formal dependency declaration exists; composition is ad-hoc via LLM reasoning.

No explicit versioning discipline per skill — the overall PAI version is the version boundary. Skills are versioned implicitly by the PAI release.

### Agent Personalities in v5.0.0

The old "Agent Personalities" concept (Principle 14 in cycle-14) is substantially restructured in v5.0.0:

1. **The DA (Digital Assistant)** — the single named AI persona. Each user configures their own DA name and personality via `USER/DA_IDENTITY.md`. The system prompt addresses the DA by `{{DA_FULL_NAME}}` and `{{PRINCIPAL_NAME}}`. This is personalization-per-user, not multiple concurrent personalities.

2. **Subagent types** — named capability specializations invoked via `Agent(subagent_type=...)`:
   - `Forge` — GPT-5.4 via `codex exec` at high reasoning effort, specializes in quality + completeness
   - `Anvil` — Kimi K2.6, for whole-project context work
   - `Cato` — GPT-5.4 in read-only sandbox (`codex exec --sandbox read-only`), cross-vendor audit
   - `Engineer` — Claude-family general coding
   - Various skill-based agents (`claude-code-guide`, etc.)

3. **capabilities.md** defines the closed enumeration of thinking capabilities and delegation capabilities. This replaces the old "voice personality system" with a more formal capability taxonomy.

### Stratification Axes (Cluster F Mapping)

PAI stratifies along these axes (mapping to cluster F's 8 sub-axes):

| Cluster F Axis | PAI Instantiation |
|----------------|-------------------|
| **version** | PAI 5.0.0 / Algorithm v6.3.0 / Memory v7.6 — explicit version tracking |
| **task-class** | MINIMAL / NATIVE / ALGORITHM — three distinct handling modes |
| **capability-tier** | E1-E5 effort levels with hard/soft floors per tier |
| **terminology** | DA (assistant), Principal (user), ISA (artifact), ISC (criterion) — formal vocabulary |
| **role** | Thinking capabilities vs. delegation capabilities (explicit two-category split in capabilities.md) |
| **cost-tier** | Inference.ts `fast` / `standard` / `smart` levels (different model routing) |
| **autonomy-mode** | `optimize-loop.md` metric-mode vs. eval-mode; Algorithm E1 (auto) vs. E4/E5 (advisor-required) |
| **capability-layer** | Hooks (hooks/) vs. Skills (skills/) vs. Tools (PAI/TOOLS/) vs. Agents (Agent() call) |

PAI is a strong cluster F augmentation candidate. All 8 axes have PAI instantiations, some with sub-axes not seen in the existing 5-system cluster F corpus (e.g., the thinking/delegation capability split as a formal two-category taxonomy within a single tier).

---

## 5. Decision Hierarchy (Principle 11) Enforcement — Code Construct or Advisory

### The Hierarchy in Code

Principle 11 (`Goal → Code → CLI → Prompts → Agents`) is confirmed as Founding Principle #12 in `ARCHITECTURE_SUMMARY.md` and embedded in multiple places in the system:

**CLAUDE.md operational rules** (the closest to enforcement):
```
bun/bunx always. Never npm/npx. Zero exceptions.
TypeScript always. Never Python unless {PRINCIPAL.NAME} explicitly approves.
Never hardcode paths. Use ${PAI_DIR}, ${HOME}, relative paths — never ${HOME}/.
Never run `claude` subprocess inline. CLAUDECODE env blocks nested sessions.
```
— `Releases/v5.0.0/.claude/CLAUDE.md`

**PAI/TOOLS/ as the "Code" level substrate** — the 70+ TypeScript CLI tools exist because many tasks CAN be handled deterministically. `Inference.ts` is invoked wherever LLM calls are needed; the tool provides the interface, not inline `@anthropic-ai/sdk` imports. The CLAUDE.md rule "Use `bun TOOLS/Inference.ts fast|standard|smart`, never import `@anthropic-ai/sdk` directly" is the code-level enforcement of the hierarchy.

**PAI/ALGORITHM/ as the "Prompts" level** — the Algorithm document is the structured prompt that manages complex work. The fact that it lives in a separate versioned file (read via LATEST indirection) rather than being inlined in CLAUDE.md is itself a separation of concerns between operational rules (CLAUDE.md) and behavioral doctrine (Algorithm).

**Hooks as the "Code" enforcement layer** — the 37 hooks enforce mechanical invariants (ContainmentGuard, SecurityPipeline, SmartApprover, PromptGuard) that would otherwise require LLM judgment. This is "code before prompts" instantiated at the session-lifecycle level.

### Is the Hierarchy Enforced or Advisory?

The hierarchy is **operationally enforced through convention and CLAUDE.md rules, not through a discriminator function.** There is no routing code that receives a task and outputs "use Code / use CLI / use Prompt / use Agent." The enforcement is:

1. Operational rules in CLAUDE.md prohibit certain antipatterns (no raw SDK, no `npm`, no Python without approval)
2. Hook-level guards prevent certain actions (ContainmentGuard, SecurityPipeline)
3. The CLAUDE.md rule "Build over ask for reversible actions" enforces preference for direct action over LLM deliberation on low-risk tasks
4. Forge auto-include at E3/E4/E5 for coding tasks is a soft enforcement of the "use Code" level (spawn a deterministic code specialist rather than relying on the primary LLM's judgment)

No CI check enforces "this could have been Code, why is it in Prompts." The hierarchy is architectural doctrine maintained through documentation and operational rules rather than through automated enforcement.

---

## 6. Tools/ and Packs/ — Capability Substrate

### Top-Level Tools/ (Not the Dispatch-Expected Directory)

The cycle-14 deferred read expected `Tools/` to contain the deterministic-substrate side of PAI's "Code Before Prompts" principle. The top-level `Tools/` has only:
- `BackupRestore.ts` — backup/restore utility for user configuration
- `validate-protected.ts` — validates protected file zones

The load-bearing capability tools are in `~/.claude/PAI/TOOLS/` (after install), which is `Releases/v5.0.0/.claude/PAI/TOOLS/` in the repo.

### PAI/TOOLS/ Directory (~70 TypeScript tools)

Full listing by category:

**Algorithm/Session orchestration:** `algorithm.ts`, `AlgorithmPhaseReport.ts`, `SessionProgress.ts`, `SessionHarvester.ts`, `PipelineOrchestrator.ts`, `PipelineMonitor.ts`

**Memory/Knowledge management:** `MemoryRetriever.ts` (BM25 retrieval), `KnowledgeGraph.ts` (graph navigation), `KnowledgeHarvester.ts` (LEARNING → KNOWLEDGE promotion), `LearningPatternSynthesis.ts`, `WisdomCrossFrameSynthesizer.ts`, `WisdomDomainClassifier.ts`, `WisdomFrameUpdater.ts`

**LLM inference interface:** `Inference.ts` — the canonical LLM interface (`fast|standard|smart` tiers, `--mode advisor`, `--auto-state`); all LLM calls route through this tool. No direct SDK imports.

**Identity/Relationship:** `DAGrowth.ts`, `DAIdentityGenerator.ts`, `DAInterview.ts`, `DASchedule.ts`, `RelationshipReflect.ts`, `OpinionTracker.ts`

**Content/Media processing:** `ActivityParser.ts`, `TranscriptParser.ts`, `ExtractTranscript.ts`, `GetTranscript.ts`, `SplitAndTranscribe.ts`, `Transcribe-*`, `YouTubeApi.ts`, `AddBg.ts`, `RemoveBg.ts`, `PreviewMarkdown.ts`

**Agent/Delegation:** `Arthur.ts`, `AgentWatchdog.ts`, `AnvilProgress.ts`, `ForgeProgress.ts`, `CrossVendorAudit.ts`

**ISA/Spec management:** `Checkpoint.ts`, `CheckpointPerISC.ts` (ISC transition handler), `FeatureRegistry.ts`, `MigrateApprove.ts`, `MigrateScan.ts`, `ApproveCurrentStateEntries.ts`, `ProposeCurrentStateEntry.ts`

**Observability/Diagnostics:** `HealthSnapshot.ts`, `DocCheck.ts`, `ReferenceCheck.ts`, `IntegrityMaintenance.ts`, `SecretScan.ts`, `BillingPathAssertion.ts`, `CostTracker.ts`, `ComputeGap.ts`, `GetCounts.ts`

**User profiling/Telos:** `GenerateTelosSummary.ts`, `InterviewIdealState.ts`, `InterviewScan.ts`, `Recommend.ts`

**Notification/Dashboard:** `Banner*.ts`, `NeofetchBanner.ts`, `PAILogo.ts`, `LoadSkillConfig.ts`, `FailureCapture.ts`

**Content generation:** `gmail.ts`, `pai.ts`

All tools are TypeScript, all invoked via `bun` CLI. The pattern `bun ~/.claude/PAI/TOOLS/ToolName.ts [args]` is the universal invocation contract. No tool uses Python.

### Key Tool: Inference.ts

`Inference.ts` is the single LLM interface for all PAI tools and hooks. The CLAUDE.md rule "Use `bun TOOLS/Inference.ts fast|standard|smart`, never import `@anthropic-ai/sdk` directly" enforces that ALL LLM calls in the PAI tool layer route through this abstraction. Three tiers:
- `fast` — lower cost, appropriate for classification, sentiment, simple tasks
- `standard` — default quality
- `smart` — highest quality for reasoning-intensive work

The `--mode advisor` flag runs a commitment-boundary second-opinion check — the same advisor call the Algorithm VERIFY phase requires.

### Packs/ Directory

The `Packs/` directory at the top-level repo (not inside Releases/) has 50+ category directories:

```
Agents, ApertureOscillation, Aphorisms, Apify, ArXiv, Art, AudioEditor,
BeCreative, BitterPillEngineering, BrightData, Browser, ContentAnalysis,
ContextSearch, Council, CreateCLI, CreateSkill, Daemon, Delegation, Evals,
ExtractWisdom, Fabric, FirstPrinciples, ISA, Ideate, Interceptor, Interview,
Investigation, IterativeDepth, Knowledge, Loop, Media, Migrate, Optimize,
PAIUpgrade, PrivateInvestigator, Prompting, README.md, RedTeam, Remotion,
Research, RootCauseAnalysis, Sales, Science, Scraping, Security, SystemsThinking,
Telos, Thinking, USMetrics, Utilities, Webdesign, WorldThreatModel, WriteStory
```

`Packs/README.md` describes Packs as the distributable/community-shareable version of skills. The directory structure mirrors `.claude/skills/` closely but adds categories not in v5.0.0's base release (Investigation, Media, Scraping, Security, Thinking, Utilities). Packs appear to be an optional add-on layer — install-on-demand, not bundled with the core release.

Pack activation lifecycle: not explicitly documented in available files. Based on directory structure (mirrors skills/), Packs appear to be installed by copying the pack directory into `~/.claude/skills/` — no manifest format or activation registry was found in the visible files.

---

## 7. `.claude/` — Claude Code Integration Surface

### Structure

`Releases/v5.0.0/.claude/` contains:

```
CLAUDE.md           — Operational procedures, format templates, context routing table
PAI/                — The PAI system (ALGORITHM/, MEMORY/, TOOLS/, DOCUMENTATION/, USER/, etc.)
hooks/              — 37 TypeScript hook files + handlers/ + lib/ + security/
skills/             — 45 named skill directories
settings.json       — Claude Code settings (hook event bindings, tool permissions)
```

**`.claude/` IS PAI's primary substrate, not a Claude Code shim.** The system was designed specifically for Claude Code and uses every Claude Code extension surface: CLAUDE.md, hooks (all four events: UserPromptSubmit, PreToolUse, PostToolUse, Stop), skills, and settings.json. There is no parallel mechanism separate from Claude Code.

### Hook Architecture (37 hooks in v5.0.0)

Full hook list by name:
```
AgentInvocation.hook.ts, CheckpointPerISC.hook.ts, ConfigAudit.hook.ts,
ContainmentGuard.hook.ts, ContentScanner.hook.ts, ContextReduction.hook.sh,
DocIntegrity.hook.ts, ElicitationHandler.hook.ts, FileChanged.hook.ts,
ISASync.hook.ts, InstructionsLoadedHandler.hook.ts, IntegrityCheck.hook.ts,
KVSync.hook.ts, KittyEnvPersist.hook.ts, LastResponseCache.hook.ts,
LoadContext.hook.ts, PreCompact.hook.ts, PromptGuard.hook.ts,
PromptProcessing.hook.ts, QuestionAnswered.hook.ts, RelationshipMemory.hook.ts,
RepeatDetection.hook.ts, ResponseTabReset.hook.ts, RestoreContext.hook.ts,
SatisfactionCapture.hook.ts, SecurityPipeline.hook.ts, SessionCleanup.hook.ts,
SetQuestionTab.hook.ts, SmartApprover.hook.ts, StopFailureHandler.hook.ts,
TaskGovernance.hook.ts, TeammateIdle.hook.ts, TelosSummarySync.hook.ts,
ToolActivityTracker.hook.ts, ToolFailureTracker.hook.ts, UpdateCounts.hook.ts,
VoiceCompletion.hook.ts, WorkCompletionLearning.hook.ts
```
Plus `handlers/`, `lib/`, and `security/` subdirectories.

Hook event mapping:
- **UserPromptSubmit**: PromptProcessing (mode/tier classification), SatisfactionCapture (feedback capture), RepeatDetection, PromptGuard, SecurityPipeline, TaskGovernance, SetQuestionTab, LoadContext
- **PreToolUse**: ContextReduction (Bash output compression via RTK), ContainmentGuard, SmartApprover, ContentScanner, AgentInvocation
- **PostToolUse**: ISASync (phase tracking), CheckpointPerISC (ISC transition commits), KVSync, ToolActivityTracker, FileChanged, QuestionAnswered, RelationshipMemory
- **Stop (session end)**: WorkCompletionLearning, SessionCleanup, VoiceCompletion, DocIntegrity, StopFailureHandler, UpdateCounts, TelosSummarySync

### Key Hook: ContextReduction.hook.sh (PreToolUse Bash)

Intercepts all Bash tool calls and rewrites output through RTK (Rate-Token-Key) compression for "60-90% token reduction." This is the code-level instantiation of the CLAUDE.md note "Context reduction: PreToolUse hook rewrites Bash through RTK for 60-90% token reduction." The hook fires on EVERY `Bash` tool call, not just specific ones.

### Key Hook: SecurityPipeline.hook.ts

Runs multiple security inspectors: Pattern, Egress, Rules, Prompt, Injection inspectors (per ARCHITECTURE_SUMMARY.md Pipeline Topology). This is cluster I (harness-enforced security/policy boundaries) in PAI. The substrate is single-user personal assistant, so the threat model is different from openclaw's multi-actor environment — but the implementation pattern is the same (hook-enforced blocking before dangerous operations reach execution).

### Key Hook: DocIntegrity.hook.ts (Stop)

Runs `DocCrossRefIntegrity.ts` (cross-reference check) + `RebuildArchSummary.ts` (regenerates ARCHITECTURE_SUMMARY.md from PAISystemArchitecture.md) at every session end. The ARCHITECTURE_SUMMARY.md read above is auto-generated, not hand-maintained. This is documentation-honesty at an architectural level: the summary is derived from the authoritative source, not maintained separately.

### CLAUDE.md Role

CLAUDE.md serves two functions:
1. **Operational rules** — non-negotiable invariants (bun, TypeScript, no hardcoded paths, no nested claude subprocess, etc.)
2. **Format templates** — three mode templates (NATIVE, ALGORITHM, MINIMAL) with exact field structures; first token of every response must be the mode header

CLAUDE.md is NOT the constitutional rules — those live in `PAI_SYSTEM_PROMPT.md` (loaded via `--append-system-prompt-file`). CLAUDE.md is the context-level procedure layer below the constitution.

### settings.json

Binds hook files to Claude Code events. This is the deterministic wiring that makes hooks fire — without an entry in settings.json, a hook file doesn't activate regardless of its content.

---

## 8. Spec/Test/Evals + Permission to Fail — Documentation-Honesty Surfaces

### Specs: The ISA System

Principle 7 (Spec/Test/Evals First) is instantiated in PAI v5.0.0 primarily through the ISA (Ideal State Artifact) system. The Algorithm OBSERVE phase produces an ISA with 12 fixed sections including `## Criteria` (the ISC/test spec), before any BUILD work begins. The ISA Skill (`skills/ISA/`) has six workflows: Scaffold, Interview, CheckCompleteness, Reconcile, Seed, Append.

ISA format spec is documented in `DOCUMENTATION/IsaFormat.md` (33KB) — a complete specification of the ISA format with frontmatter contracts, section requirements per tier, and example library.

**The ISA IS the spec AND the test harness.** ISCs are atomic binary criteria where each criterion has a single named tool probe (one `Bash` call, one `Grep`, one `Read`, etc.). "Don't invent parallel artifacts (acceptance.yaml, acceptance.ts, separate test specs) — the ISA already covers this surface." — `v6.3.0.md`

### Tests: None at the Repo Level

PAI has **no automated test suite.** The `.github/workflows/` directory contains only:
- `claude-code-review.yml` — Claude-powered PR review (not test execution)
- `claude.yml` — Claude Code integration (not test execution)

No unit tests, no integration tests, no coverage reporting, no CI test gate before merge. The ISA system is the closest analog to tests but it's instance-specific (each Algorithm run has its own ISA; there's no shared test suite for PAI's own code).

`ALGORITHM/eval-guide.md` (9KB) documents how to evaluate Algorithm quality — but these are manual evaluation guidelines, not automated test runs.

### Evals: The Optimize Loop

The Evals skill (`skills/Evals/`) and `ALGORITHM/eval-guide.md` represent PAI's evaluation layer. The `optimize-loop.md` (Optimize Loop Protocol v2) defines a formal optimization lifecycle: target analysis → autonomous optimization loop → recommendation → learning extraction. The protocol has two modes:
- **Metric Mode** (code files): run shell command, extract number, score
- **Eval Mode** (prompts, skills, agents): LLM grader scoring against rubric

This is closer to the "evals" concept (systematic comparison, graded output) than to unit tests. Evals are explicitly invoked (`mode: optimize` in Algorithm or direct `bun run` of Evals workflows), not automated on commit.

### Permission to Fail

**"I don't know" as a code construct:** Not found as a structured return type or exception class. It IS a constitutional rule in `PAI_SYSTEM_PROMPT.md` — the system prompt explicitly grants permission to express uncertainty. The `FailureCapture.ts` tool provides structured recording of failures but the "I don't know" response is a prompt-level directive, not a code-level type.

**Failure recording as durable artifacts:** YES — strongly confirmed. `FailureCapture.ts` is invoked on ratings ≤ 3, writes structured Markdown to `MEMORY/LEARNING/`. `SatisfactionCapture.hook.ts` writes ALL non-trivial interactions to `ratings.jsonl`. Low-rating learning artifacts are written with structured YAML frontmatter (`capture_type`, `rating`, `source`, `auto_captured`, `tags`). This is comparable to Voyager's `failed_tasks.json` as a failure-as-recorded-artifact pattern — cluster D sub-shape.

**Anti-pattern catalog:** No explicit "What We Will Not Merge" document analogous to openclaw's VISION.md. The closest equivalents are:
- CLAUDE.md's "Never" rules (never npm, never Python without approval, never raw SDK, never nested claude subprocess)
- ContainmentGuard.hook.ts (blocks certain file system operations)
- PATTERNS.yaml in USER/SECURITY/ (security patterns list)

These are enforcement-layer prohibitions, not a documented "things we've decided not to do with explanation."

**Aspirational-vs-implemented markings:** NOT explicitly present. No README sections marked "aspirational — not yet implemented." The ARCHITECTURE_SUMMARY.md disclaimer "Auto-generated by ArchitectureSummaryGenerator.ts. Do not edit manually" is a form of documentation honesty (derived from authoritative source, not hand-maintained).

**Code-level invariants testing PAI's own claims:** The `DocIntegrity.hook.ts` and `ArchitectureSummaryGenerator.ts` create a form of self-testing: if CLAUDE.md references a file path that doesn't exist, `DocCrossRefIntegrity.ts` would catch it. The `IntegrityCheck.hook.ts` and `ConfigAudit.hook.ts` perform similar structural checks. These are the closest thing to "PAI asserting its own invariants" — mechanical checks on documentation and configuration consistency rather than behavioral assertions.

---

## 9. Anchoring Caveats

### PAI is a personal assistant; we are an autonomous orchestrator

The most fundamental difference: **PAI's Principal provides goals from outside the system.** PAI's algorithm starts with "what did the user ask for?" Our redesign operates with an orchestrator that generates its own next-cycle goals from prior state — no external human goal-stream between Eva's occasional inputs. PAI's "User Centricity" (Principle 1) and the DA model presuppose a continuous human guidance stream that our system architecture doesn't have. PAI's tight-loop satisfaction capture (SatisfactionCapture on every UserPromptSubmit) is structurally inapplicable — we have no per-prompt user feedback signal.

### Single-user vs. multi-issue pipeline

PAI manages a single user's life across multiple domains simultaneously. Our system manages a single software engineering project's issue pipeline sequentially. PAI's stratification (by domain: health, finances, work, relationships) maps to our stratification (by issue: PRD, tool, schema, audit) but the scoping model differs substantially. PAI's TELOS/ system (mission, goals, challenges) has no analog in our design — we don't maintain a mission statement or goal hierarchy.

### Claude Code as the substrate

PAI is built entirely on Claude Code as the execution environment. Our system uses GitHub Actions + Claude API. The hook system, CLAUDE.md operational rules, and skill invocation patterns are all Claude Code-specific. Direct transfer of the hook architecture requires Claude Code adoption; adapting the patterns (e.g., hook-equivalent pre-processing) would require equivalent infrastructure.

### v5.0.0 is substantially different from v4.0.3

The cycle-14 read and this dispatch's framing was organized around v4.0.3 (the stable release at cycle-14 time). This read focused on v5.0.0, which is the current development branch. Notable differences:
- Algorithm v3.5.0 (v4.0.3) → v6.3.0 (v5.0.0): substantially more codified, ISA system, effort tiers, closed capability enumeration
- 63 skills (v4.0.3) → 45 skills reorganized (v5.0.0): skill count reduced by consolidation, not reduction of capability
- 21 hooks (v4.0.3) → 37 hooks (v5.0.0): doubled, covering more lifecycle events
- No Pulse dashboard (v4.0.3) → Pulse at port 31337 (v5.0.0): new observability/dashboard layer
- "AI scaffolding with 16 principles" framing → "Life Operating System with DA + Pulse + Algorithm as gravitational center"

The 16 principles (now 17 in the Founding Principles list) are preserved but some have been substantially instantiated by v5.0.0 code where they were only stated as goals in v4.0.3.

### The "confirmation-bias risk" flagged in cycle 14 — verdict

The cycle-14 read warned that "PAI principles' alignment with our CORE-DESIGN-PRINCIPLE is striking and concerning" and flagged confirmation-bias risk. Code-level reading produces a more nuanced picture:

- **Principles 4, 5, 6, 8, 11 (deterministic infrastructure, code before prompts, CLI first, Goal→Code→CLI→Prompts→Agents)**: **CONFIRMED as architecturally instantiated**, not just marketing. The hook architecture, TOOLS/ layer, Inference.ts abstraction, CLAUDE.md operational rules, and bun-only/TypeScript-only mandates collectively implement this principle at code level.

- **Principle 13 (Memory System)**: **REFINED** — memory IS a first-class architectural concern with multi-tier typed storage and a promotion ladder. But NOT semantic-retrieval-based. The memory system is richer than the README implied but different from what cluster J emergence required.

- **Principle 2 (Foundational Algorithm)**: **CONFIRMED AS MORE** than README framing — it's a versioned 46KB doctrine document with hook-enforced phase transitions, not just a slide-deck abstraction. But NOT a state machine. Partially confirmed, partially refined.

- **Principle 12 (Skill Management)**: **CONFIRMED AND EXTENDED** — the skill system is the most directly transferable design pattern. Skills-as-self-activating-composable-units with SKILL.md front doors, workflow files, and optional TypeScript tools is a complete architecture.

- **Principle 7 (Spec/Test/Evals First)**: **PARTIALLY CONFIRMED** — the ISA system is a strong spec-first mechanism. But the "test" and "evals" parts are not automated — no test suite, no CI test gate. The principle as stated overpromises relative to what code delivers.

- **Principle 16 (Permission to Fail)**: **PARTIALLY CONFIRMED** — failure recording IS implemented as durable artifacts (FailureCapture.ts), which is more than the README implied. But "I don't know" as a structured type doesn't exist; no aspirational-vs-implemented markings; no anti-pattern catalog with non-permanence caveat.

The deeper read confirms: PAI's alignment with the CORE-DESIGN-PRINCIPLE is implementation-backed, not marketing-surface only. The transfer-relevant patterns (skill architecture, hook lifecycle, Inference.ts abstraction, ISA/ISC system) are genuine architectural contributions. The non-transfer findings (no test suite, no score-gated promotion, no semantic retrieval, single-user model) are equally genuine architectural differences.

---

## Cluster Framework Anchoring

| Cluster | PAI Evidence | Verdict |
|---------|-------------|---------|
| **A (cycle-internal phasing primitives)** | Algorithm v6.3.0 seven phases with hard gates + PromptProcessing.hook.ts classifier-mediated mode dispatch | NEW SUB-SHAPE: classifier-mediated task dispatch (deterministic routing before executor acts) |
| **B (storage-architecture stratification)** | Multi-tier MEMORY/ directories, append-mostly write, WORK→LEARNING→KNOWLEDGE promotion ladder, BM25 retrieval (no semantic) | AUGMENTS: multi-mechanism-per-coordinate sub-shape confirmed; semantic-retrieval sub-shape NOT confirmed (H1 refuted) |
| **C (termination/failure modes)** | CheckpointPerISC auto-commits on ISC transitions; fail-safe on classifier timeout (ALGORITHM E3 default) | POTENTIAL: CheckpointPerISC is a granular durability gate at ISC-transition level — finer-grained than session-level |
| **D (documentation honesty)** | FailureCapture.ts for durable failure artifacts; DocIntegrity.hook.ts auto-derives architecture summary; no aspirational markings found | AUGMENTS: failure-as-recorded-artifact (FailureCapture.ts) + doc-honesty-via-derivation (ARCHITECTURE_SUMMARY auto-generation) |
| **F (stratification axes)** | All 8 axes present: version/task-class/capability-tier/terminology/role/cost-tier/autonomy-mode/capability-layer | STRONG AUGMENT: all 8 cluster F axes confirmed in PAI, some with sub-axes not in prior corpus |
| **H (post-session feedback / continuous improvement)** | SatisfactionCapture.hook.ts implicit rating inference + WorkCompletionLearning.hook.ts SessionEnd artifacts | CONFIRMS H2: 5-system convergent; NEW SUB-SHAPE: LLM-inferred implicit satisfaction from follow-up behavior |
| **I (harness-enforced security/policy boundaries)** | SecurityPipeline.hook.ts with multiple inspectors; ContainmentGuard.hook.ts; PromptGuard.hook.ts | AUGMENTS: confirms cluster I extends to single-user personal-assistant substrate (not only cloud multi-actor) |
| **J (semantic-retrieval architecture)** | H1 REFUTED: BM25 retrieval, no vectors, no embeddings, explicit anti-RAG position | NOT EMERGED |

---

## New Patterns Visible Only at Code Level

The following patterns were NOT visible at README/principle-list depth:

1. **ISA as spec-test-verification unification** — the ISA (Ideal State Artifact) is simultaneously: spec (Problem/Vision/Criteria), test harness (each ISC is one binary tool probe), verification record (## Verification section), done condition, and system of record. A single artifact covers the entire spec→test→verify lifecycle. No parallel acceptance.yaml, no separate test spec needed.

2. **Classifier-mediated mode dispatch** — every top-level prompt is classified by a Sonnet LLM call (PromptProcessing.hook.ts) into MINIMAL/NATIVE/ALGORITHM with effort tier E1-E5 before the executor sees the prompt. The executor is prohibited from overriding without explicit `/e1`-`/e5` flag or conversation-context justification. Fail-safe: any classifier error → ALGORITHM E3. This is a deterministic pre-processing gate around probabilistic LLM behavior.

3. **Closed-enumeration capability taxonomy** — the Algorithm v6.3.0 has a hard-closed list of 19 named "thinking capabilities" (IterativeDepth, ApertureOscillation, FirstPrinciples, SystemsThinking, etc.) and "delegation capabilities" (Forge, Anvil, Cato, etc.). Inventing a name not on the list ("decomposition", "structured thinking") is a CRITICAL FAILURE counted by an audit gate. This is vocabulary discipline applied to AI capability selection.

4. **BM25 over RAG — explicit architectural choice** — memory retrieval via BM25 keyword ranking and ripgrep, with explicit documentation that PAI has rejected RAG since June 2025. The justification: "Rich text with cross-references, plus fast search like ripgrep, gives us everything people normally want from RAG." This is a deliberate anti-complexity stance with documented reasoning.

5. **LLM-inferred implicit satisfaction signal** — SatisfactionCapture infers satisfaction rating on EVERY user prompt using a second LLM call (Sonnet fast-tier), not just explicit ratings. The prompt includes the prior response, recent conversation context, and the current prompt. Every non-trivial interaction generates a 1-10 satisfaction signal, regardless of whether the user rates anything. Neutral = 5, never null.

6. **Cross-vendor auditing (Cato pattern)** — at E4/E5 effort levels, the Algorithm spawns Cato (GPT-5.4 in `codex exec --sandbox read-only`) to audit the primary Claude model's work. This is a structural architecture for catching Anthropic-model blind spots using a competing model as auditor. The audit blocks `phase: complete` if any `critical` finding is returned.

7. **DocIntegrity auto-derivation** — `ARCHITECTURE_SUMMARY.md` is auto-generated at every session end from the authoritative `PAISystemArchitecture.md` by `ArchitectureSummaryGenerator.ts`, invoked by `DocIntegrity.hook.ts`. This enforces documentation-honesty mechanically: the summary cannot drift from the authoritative source because it's derived, not maintained.

8. **Context compression as a hook** — `ContextReduction.hook.sh` intercepts ALL Bash tool calls and compresses output through RTK for 60-90% token reduction. This is an automatic, invisible optimization applied to every tool call — the AI never sees uncompressed Bash output in its context.

9. **CheckpointPerISC durability** — every `[ ]` → `[x]` ISC state change triggers auto-commit to all repos in `checkpoint-repos.txt`. Combined with the ISA as system-of-record, this means every step of progress is durably committed in git. No work is lost even if the session crashes mid-Algorithm.

10. **Voice-as-phase-announcement primitive** — algorithm phase transitions are announced via voice (ElevenLabs TTS through Pulse's REST endpoint at `localhost:31337/notify`) as a MANDATORY action. The voice announcement is the human-observable signal that phase transition happened; the ISA frontmatter edit is the machine-observable signal. Both are required.

---

## Principle-Verdict Table (Principles 1-17)

| # | Principle | Verdict | Evidence |
|---|-----------|---------|---------|
| 1 | PAI is the Life Operating System | **CONFIRMED** | ARCHITECTURE_SUMMARY.md + PAI_SYSTEM_PROMPT.md; full OS framing with DA, Pulse, Algorithm as gravitational center |
| 2 | The Foundational Algorithm | **CONFIRMED (REFINED)** | Algorithm v6.3.0.md (46KB), seven phases with HARD gates, effort tiers, closed capability enumeration — substantial doctrine, not just framing |
| 3 | Clear Thinking First | **CONFIRMED (REFINED)** | Algorithm's thinking-capability closed enumeration enforces specific thinking discipline; "phantom" thinking names are CRITICAL FAILURE |
| 4 | Scaffolding > Model | **CONFIRMED** | 37 hooks, 70+ tools, Inference.ts abstraction, skill architecture — the scaffolding IS the system |
| 5 | Deterministic Infrastructure | **CONFIRMED** | Hooks enforce deterministic events; Inference.ts abstracts LLM calls; BM25 for retrieval (not probabilistic RAG); mode classifier produces deterministic routing |
| 6 | Code Before Prompts | **CONFIRMED** | 70+ TypeScript tools in PAI/TOOLS/; CLAUDE.md "bun/bunx always"; `Inference.ts fast\|standard\|smart` instead of raw SDK imports |
| 7 | Spec/Test/Evals First | **PARTIALLY CONFIRMED** | ISA system is strong spec-first mechanism; "evals" are manual/on-demand; NO automated test suite; no CI test gate |
| 8 | UNIX Philosophy | **CONFIRMED** | Each tool does one thing (ActivityParser, KnowledgeHarvester, Inference, etc.); all text-interface via bun CLI; no shared state between tools |
| 9 | ENG/SRE Principles | **CONFIRMED** | Observability JSONL logs (mode-classifier.jsonl, ratings.jsonl, OBSERVABILITY/*.jsonl); HealthSnapshot.ts; DocIntegrity auto-checks; CheckpointPerISC durability |
| 10 | CLI as Interface | **CONFIRMED** | All tools invoked via `bun <ToolName>.ts [args]`; no GUI for system operations; Pulse is display-only, not operational |
| 11 | Goal → Code → CLI → Prompts → Agents | **CONFIRMED (ADVISORY)** | Architectural doctrine confirmed; enforced via CLAUDE.md rules and Forge auto-include; no discriminator function or routing gate |
| 12 | Custom Skill Management | **CONFIRMED** | 45 skills in `.claude/skills/`; SKILL.md front door pattern; Workflows/*.md; UserSkillCustomizations overlay |
| 13 | Custom Memory System | **CONFIRMED (REFINED)** | Multi-tier typed storage confirmed; BM25 (NOT semantic); rich promotion ladder; 16+ subdirectories — richer than README implied but different shape from what cluster J required |
| 14 | Custom Agent Personalities | **CONFIRMED (RESTRUCTURED)** | DA system + named subagent types (Forge/Anvil/Cato/Engineer); old multi-personality framing replaced by capability-specialization taxonomy |
| 15 | Science as Meta-Loop | **CONFIRMED** | SatisfactionCapture's implicit rating inference; WorkCompletionLearning structured capture; optimize-loop.md formal A/B protocol; eval-guide.md |
| 16 | Permission to Fail | **PARTIALLY CONFIRMED** | FailureCapture.ts durable artifacts for ratings ≤3; no structured "I don't know" return type; no aspirational markings; no anti-pattern catalog with non-permanence caveat |
| 17 | Science as Cognitive Loop *(new in v5.0.0)* | **CONFIRMED** | SatisfactionCapture + WorkCompletionLearning + ISA Changelog conjecture/refutation/learning format = systematic Hypothesize→Observe→Learn cycle built into session lifecycle |

---

*End of cycle-71-pai-deeper-read.md*
