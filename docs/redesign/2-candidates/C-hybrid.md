# Candidate C: Hybrid (Single-Orchestrator with First-Class Reconciler-Mode and Plans-as-Artifacts)

**Authoring cycle:** 91 (2026-05-08)
**Authored against:** [`2-design-framework.md`](../2-design-framework.md) v1.21+; [`clusters.md`](../1-research/clusters.md) M-item integration arc closure (cycles 86-89); cycle 90 hand-off naming Candidate C as the cycle 91+ work item.

**One-line position:** Preserve v1's single-orchestrator substrate (Axis 1 = single-threaded, like Candidate A) **but** elevate inbound-channel reconciliation to a first-class harness phase (Axis 12 dedicated reconciler-mode within the single session, not a separate role) and adopt plans-as-artifacts directories for multi-cycle plan coordination (Axis 5 = Yes, like Candidate B). The candidate's central bet is that v1's substrate is good (Candidate A's bet) **and** inbound reconciliation deserves structural first-class status without paying the multi-agent coordination overhead — that the load-bearing addition between A and B is **harness-phase elevation of one specific concern** (reconciliation), not full role decomposition.

**Distinctness check (per cycle 90 hand-off question):** C is genuinely distinct from A on three axes (Axis 5 = Yes vs A's No; Axis 7 = 4-mode vs A's 3-mode; Axis 12 = event-driven-with-fallback vs A's hybrid). C is genuinely distinct from B on five axes (Axis 1 = single-threaded vs B's small-fixed-team; Axis 2 = file-per-component vs B's typed-channel-map; Axis 4 = git-as-substrate vs B's branching checkpoints; Axis 6 = tools-with-registry vs B's skills; Axis 13 = medium harness vs B's fat harness). C is **not** A + targeted additions cosmetically — the reconciler-mode is a new named harness phase that A doesn't have, and plans-as-artifacts is a new artifact category. C is **not** B-light — it preserves A's single-threaded substrate and bounded migration cost, adopting only the targeted B structural elements that fit single-orchestrator shape without paying multi-agent coordination cost.

## Position summary

- **Axis 1 (decomposition):** Single-threaded with off-process Copilot dispatches — same as Candidate A. Writes-stay-single-threaded as structural invariant. No in-session role decomposition. The "reconciler" in C is a harness-phase, not a separate agent.
- **Axis 2 (state representation):** File-per-component — same as Candidate A. `state/dispatch-queue.json`, `state/eva-input-cursor.json`, `state/redesign/<phase>.json`, `state/cycle-history/<N>.json`. Plus C-specific additions: `state/inbound-cursor.json` (reconciler-mode cursor for cross-repo + dispatch-merge events), `state/plans/active/<plan-id>.json` (plan-lifecycle component file).
- **Axis 3 (memory shape):** Wiki + search — same as Candidate A. `_notes/` and `docs/redesign/_notes/` as wiki substrate; `wiki-search` Rust tool maintains description-frontmatter index. No semantic embedding (HIGH-cost cluster B sub-shape 7 deferred). No per-agent memory channels (cluster B sub-shape 3 deferred — single orchestrator means single memory).
- **Axis 4 (history substrate):** Git-as-substrate **with plan-lifecycle states in-tree files** — same git-as-substrate primitive as Candidate A; per-cycle summaries land in `state/cycle-history/<N>.json` append-only. Plus plan-lifecycle states (active/completed/technical-debt) as named directories under `docs/redesign/plans/`, with promotion criteria encoded as plan-frontmatter. **Not** branching checkpoints in the Candidate B sense — plans-as-artifacts is named-states-in-directories, not full multi-channel state snapshots. Git-safety preserved primitive (commit-must-be-pushed) honored by Axis 4 directly.
- **Axis 5 (plans-as-artifacts):** **Yes** — `docs/redesign/plans/active/<plan-id>.md`, `plans/completed/<plan-id>.md`, `plans/technical-debt/<plan-id>.md`. Plans authored at cycle-N→cycle-N+1 hand-off boundaries; lifecycle transitions are named (active → completed | technical-debt). The plan-lifecycle Rust tool enforces the lifecycle state machine. Plans are documents, not full state snapshots — distinct from B's branching checkpoints.
- **Axis 6 (extension shape):** Tools (Rust binaries with per-tool manifest) — same as Candidate A. `tools/rust/crates/tool-registry` for orchestrator discovery. No skill abstraction, no per-tool prompt fragments. Bounded migration cost.
- **Axis 7 (orchestration topology):** Sequential mode transitions with **4 modes** (vs A's 3) — `{ boot, reconcile, work, close }`. **Reconcile-mode is a named first-class phase** that runs after boot and before work, processing inbound channels (Eva responses, audit posts, dispatch PR-merges). Mode allowlist and transition policy encoded in cycle-runner harness. The reconciler-mode shape is the C-specific structural addition that A doesn't have.
- **Axis 8 (mechanical enforcement):** Behavioral promises + agent-affecting prose — same as Candidate A. CI on `prompt-contract-check`. Plus C-specific: plan-lifecycle CI checks (`plan-lifecycle-check`) ensuring active plans meet lifecycle invariants (named owner, last-updated cursor, promotion criteria specified).
- **Axis 9 (iteration ceilings):** Loop count — same as Candidate A. Per-tool `max_iterations` declared in tool manifests; cycle-runner enforces. Plus C-specific: per-mode runtime budget (boot-mode ≤ 5min; reconcile-mode ≤ 10min; work-mode ≤ 50min; close-mode ≤ 10min — soft budgets enforced in cycle-runner).
- **Axis 10 (entropy mitigation):** Mandatory deslop pass with plan-lifecycle gardening hooks — same minimal `gardening-sweep` as A (stale-detection + dead-link-detection) plus plan-lifecycle gardening (stale-active-plan detection — active plans not updated in N cycles → flagged for technical-debt promotion). Continuous-background discipline NOT adopted (cluster H sub-shape 3 = HIGH cost; same as A's deferral).
- **Axis 12 (reconciliation discipline):** **Event-driven with cycle-cadence fallback** — Actions cron triggers cycle-runner; reconcile-mode fires every cycle and processes inbound-cursor advances (Eva responses, audit posts, dispatch PR-merges) as a first-class structured phase. Per-event handlers per channel; reconciler-mode emits typed-deltas to per-component state files. **More structured than A's "interleaved with boot-phase polling" but no event-driven-on-arrival** — there is no in-cycle event arrival because the cron-triggered single-orchestrator shape doesn't have a continuously-running session. C accepts cycle-cadence latency for reconciliation in exchange for not running multi-agent infrastructure.
- **Axis 13 (harness-vs-session boundary):** Medium harness with reconciler-mode as named harness phase — same medium-harness extraction as A (boot phase, close phase, dispatch-poll, audit-read), plus reconcile-mode is a named harness phase between boot and work. Cycle-runner change scope: SUBSTANTIAL (similar to A's substantial change scope, not B's rewrite-scope).

## Cross-axis commitments

- **Axis 1 × Axis 7:** Single-threaded forces single-topology in the *session*; C extends A's 3-mode topology to 4-mode by inserting reconcile-mode. Reconcile-mode runs in the same session context (single-threaded constraint preserved) but in a structured harness-phase distinct from work-mode.
- **Axis 1 × Axis 12:** Single-threaded with dedicated reconcile-mode is the **single-orchestrator instantiation of "dedicated reconciliation phase"** — the cross-axis dependency map names dedicated-reconciliation-agent as the small-fixed-team counterpart; C demonstrates that dedicated-reconciliation can be a harness-phase, not necessarily an agent. This is C's central structural distinctness claim from A.
- **Axis 2 × Axis 3:** File-per-component pairs with wiki-as-files-in-repo, same as A. Discovery via repo-walk + index regeneration.
- **Axis 2 × Axis 5:** Plans-as-artifacts uses file-per-component for the plan-lifecycle state files (`state/plans/active/<plan-id>.json` per active plan, with promotion-cursor and stale-cycle-count). Plans-as-artifacts integrates with file-per-component substrate; no separate state mechanism.
- **Axis 4 × Axis 5:** Plans-as-artifacts uses git-as-substrate (named directories per lifecycle state); promotion = file-move + commit. No branching-checkpoints semantics. Plans are documents committed to main; lifecycle transitions are git-tracked.
- **Axis 4 × Axis 2:** Same as A — git-as-substrate naturally supports file-per-component append semantics (each component file's history is the per-component append log).
- **Axis 7 × Axis 12:** 4-mode topology (boot / reconcile / work / close) is the structural commitment to first-class reconciliation. Reconcile-mode runs every cycle (not situational) but is bounded (≤ 10min budget; processes inbound-cursor advances and emits deltas; does NOT do work-phase activities).
- **Axis 7 × Axis 13:** Medium harness encodes the 4-mode transition policy; cycle-runner is the state machine. Each mode's invocation is a named harness phase with declared inputs / outputs / side-effects.
- **Axis 12 × Axis 4:** Cycle-cadence reconciliation reads cursor files at reconcile-mode entry; per-event handlers emit deltas to per-component state files. Git-as-substrate captures every reconcile-mode delta as a commit.
- **Axis 13 × Axis 6:** Medium harness organizes Rust tools via `tool-registry`; the tool-registry serves both the discovery primitive (Axis 6) and the harness's enumeration (Axis 13). Same as A.
- **Axis 13 × Axis 8:** Medium harness has prompt as primary mechanical-enforcement surface; `prompt-contract-check` + `plan-lifecycle-check` CI is the bridge.
- **Constraint 8 × Axis 1:** Goal-driven posture pairs naturally with single-threaded long-running execution (same as A); no goal-coordination primitive needed.
- **Constraint 8 × Axis 5:** Plans-as-artifacts gives the goal-driven posture an explicit forward-look surface: active plans declare goals; promotion-or-technical-debt-tag at completion is goal-driven evaluation.

## Failure-mode addressing

- **F1 (constraint accretion):** Axis 8 (`prompt-contract-check` + `plan-lifecycle-check` CI) + Axis 13 (medium harness extracts ~50% procedural surface to cycle-runner phases) — F1 structurally addressed at same leverage as Candidate A; the plan-lifecycle CI adds a small additional surface for plan-related constraint accretion detection.
- **F2 (Eva-response detection):** Axis 12 reconcile-mode is the first-class Eva-response-detection phase. Reconcile-mode fires every cycle; reads `state/eva-input-cursor.json`; pulls input-from-eva issues with cursor-advance; pulls question-for-eva responses. Latency = 1 cycle (~6h at 4 cycles/day cron) — same as A. **Detection improvement over A:** reconcile-mode is structurally first-class (named harness phase), not "side activity of boot-phase." Detection signal is more reliable; reconciliation work doesn't compete with primary work for boot-phase budget.
- **F3 (multi-candidate state drift):** Axis 2 file-per-component (single source of truth per concern) + Axis 12 reconcile-mode reconciles against post-close evidence (cycle-history files are append-only; in-cycle deltas are reconciled at reconcile-mode entry) — same as A; reconcile-mode is the structural locus.
- **F4 (frozen-artifact lifecycle fragility):** Axis 4 git-as-substrate determines what "frozen" means + Axis 5 plans-as-artifacts gives explicit lifecycle states (active / completed / technical-debt) — improvement over A which has no plan-lifecycle. Refresh timing is governed by reconcile-mode (cycle-cadence). **F4 improvement over A:** plan-lifecycle states make freeze/refresh timing explicit per plan; A has no plan-lifecycle, so plan-related freeze fragility is implicit.
- **F5 (state.json as procedural-leak):** Axis 2 file-per-component eliminates monolithic state file (same as A). Plus C adds `state/inbound-cursor.json` and `state/plans/active/*.json` — small additional file count (~2-3 new state files). Each file's schema is single-purpose; `prompt-contract-check` catches procedural-leak.
- **F6 (cyclomatic procedure depth):** Axis 7 4-mode transitions with explicit transition policy (vs A's 3-mode; vs v1's STARTUP→C-phases→COMPLETION nested checklist) + Axis 13 medium harness — same leverage as A. The 4th mode (reconcile-mode) is bounded; cyclomatic depth is +1 over A but still vastly less than v1.
- **F7 (self-management dominance):** Axis 1 (no decomposition overhead, same as A) + Axis 8 (mechanical enforcement, same as A) + Axis 9 (iteration ceilings + per-mode runtime budgets, slightly more than A) + Axis 13 (medium harness extracts ~50% procedural surface, same as A). Aggregate effect: per-cycle decision count slightly higher than A due to plan-lifecycle and reconcile-mode coordination, but bounded; far below B's aggregate at the system level.
- **F8 (abandonment cascades):** Axis 9 loop ceilings + per-mode runtime budgets (prevention) + cluster A sub-shape 6 stuck-cycle-watchdog via Rust tool `detect-abandoned-cycles` (detection + recovery) + single-implementation discipline. **F8 improvement over A:** per-mode runtime budgets give finer-grained abandonment detection (a mode that exceeds budget is flagged early; v1 and A only have cycle-level budgets).
- **F9 (adversarial-review treadmill):** Axis 7 4-mode with reconcile-mode as named phase — review-firing is situational (cycle-runner triggers review only when phase-transition rules require, same as A). Audit-as-peer (cross-repo) is the asynchronous-review primitive. **No structural improvement over A on F9.**
- **F10 (audit's value is broader read scope):** Audit-as-peer pattern preserved (Axis 12 reconcile-mode + cross-repo reading discipline). Reconcile-mode treats `[audit-request]` events as first-class — better discovery primitive than A's boot-phase pull.
- **F11 (post-close mutations):** Axis 4 git-as-substrate (append-only cycle-history files) + Axis 12 reconcile-mode reads back state at reconcile-mode entry. **F11 improvement over A:** reconcile-mode is the structural locus for post-close mutation detection (vs A's boot-phase polling). The detection mechanism is the same; the locus is more legible in C.
- **F12 (defense accretion catalog):** Axis 2 + Axis 4 + Axis 10 contribute, same as A. Plan-lifecycle states (Axis 5) add a structured anti-accretion primitive at the plan level (technical-debt directory IS the named-cataloging primitive for plan-related deferred work).

## Preserved-primitives compliance

- **Journal:** preserved; remains long-term reflective channel. `docs/journal/YYYY-MM-DD.md` freeform format unchanged. Cycle-runner appends per-cycle subsection at session-start (boot phase). Reconcile-mode may append a reconciler-subsection if non-trivial inbound deltas (otherwise empty).
- **Cycle-issue:** preserved; session-start comment at boot-mode start; session-end summary at close-mode end. Reconcile-mode does not post comments unless reconciler surfaces something requiring orchestrator judgment (escalation pattern). No structured step-id taxonomy.
- **Question-for-eva / input-from-eva:** preserved; reconciliation via reconcile-mode (Axis 12 first-class). Cursor file `state/eva-input-cursor.json` advances at reconcile-mode start.
- **Git-safety:** preserved; commit-must-be-pushed honored by Axis 4 (git-as-substrate) and existing `cycle-runner` push discipline. Per-component file writes commit + push in same operation. Plan-lifecycle transitions (file moves) are commit + push in same operation.
- **Cycle-runner harness:** SUBSTANTIAL change scope. 4 mode phases (boot, reconcile, work, close) added. Phase-boundary state machine encoded in cycle-runner. Specifics: `cycle-runner` invokes new sub-binaries `boot-mode`, `reconcile-mode`, `close-mode`, `mode-transition-check`. Each is a new Rust crate. Plus: `plan-lifecycle` for plans-as-artifacts management.

## What this candidate gives up

- **No dedicated reconciler agent** (Axis 1 single-threaded, like A) — reconcile-mode runs in the same session as work-mode, sharing the same context-window and the same compute budget. If the volume of inbound channels grows substantially, reconcile-mode becomes the bottleneck, just as A's boot-phase would.
- **No per-agent memory** (single session, single memory; same as A) — wiki-search shared across all cycle work.
- **No parallel exploration branches** (Axis 4 git-as-substrate without branching-checkpoints; same as A) — what-if reasoning across multi-cycle horizons remains harder than B's branching mechanism. Plans-as-artifacts gives sequential plan iteration (active → completed | technical-debt) but not parallel branches.
- **No skills abstraction** (Axis 6 tools, like A) — extension surface is bounded; skills with prompt-fragments not adopted (cluster F sub-axis 4 = HIGH cost; same as A's deferral).
- **No score-gated consolidation** (cluster H sub-shape 2 = HIGH cost; same as A's deferral) — memory consolidation remains manual / ad-hoc via journal entries and `_notes/` rewrites.
- **Higher cluster A ABSENT count adopted partially** (similar to A — phase-boundary semantics + recovery operations + concurrency-queuing ABSENT sub-shapes are inherited at LOW-MODERATE cost only via the 4-mode state machine + per-mode runtime budgets; some improvement over A's 3-mode but not full).
- **Plan-lifecycle complexity** — plans-as-artifacts adds plan-state machine (active / completed / technical-debt), promotion criteria, stale-active-plan detection. C accepts this cost as load-bearing for F4 + cluster F structural completeness; A defers it as not-yet-needed.
- **Conditional-improvement risk** — C's central bet (reconcile-mode + plans-as-artifacts are load-bearing additions) is less proven than A's bet (substrate is good, structural cleanup sufficient). If the additions don't carry their weight, C is just A with more migration surface.

## Tool surface implied

**New tools (net-add for cutover):**

- `tools/rust/crates/boot-mode` — boot-mode orchestration (state-load, cursor-advance, standing-directive check) — narrower scope than A's `boot-phase` because reconcile-mode handles inbound-channel work
- `tools/rust/crates/reconcile-mode` — reconcile-mode orchestration (inbound-cursor advances; per-channel handlers for Eva responses, audit posts, dispatch PR-merges; typed-delta emit to per-component state files) — **C-specific addition**
- `tools/rust/crates/close-mode` — close-mode orchestration (gardening-sweep, plan-lifecycle gardening, cycle-history append, journal commit + push, cycle-issue close-comment)
- `tools/rust/crates/mode-transition-check` — mode boundary state machine enforcement (4 modes vs A's 3)
- `tools/rust/crates/wiki-search` — top-k retrieval over `_notes/*.md` description-frontmatter (same as A)
- `tools/rust/crates/tool-registry` — Rust tool enumeration with one-line descriptions (same as A)
- `tools/rust/crates/prompt-contract-check` — CI test for prompt drift (same as A)
- `tools/rust/crates/plan-lifecycle-check` — CI test for plan-lifecycle invariants — **C-specific addition**
- `tools/rust/crates/plan-lifecycle` — plans-as-artifacts directory management (active/completed/technical-debt state machine; promotion-cursor; stale-active detection) — **C-specific addition**
- `tools/rust/crates/detect-abandoned-cycles` — stuck-cycle-watchdog (cluster A sub-shape 6; same as A) plus per-mode-runtime-budget checks — minor extension
- `tools/rust/crates/gardening-sweep` — Axis 10 deslop pass (same as A) plus plan-lifecycle gardening hooks — minor extension
- `tools/rust/crates/cycle-history-append` — append-only cycle-history file write (same as A)

**Existing tools preserved (~unchanged):**

- `cycle-runner` (becomes 4-mode driver instead of 3-mode; SUBSTANTIAL change but bounded), `write-entry`, `check-eva-responses` (or replacement-with-cursor; reconcile-mode is the new locus), `pipeline-check`, audit-related tools

**Existing tools deprecated:**

- Anything that reads/writes `state.json` directly — superseded by per-component state files. Migration script: `tools/rust/crates/state-json-decompose` (one-shot Phase 4 cutover tool, same as A).

**Tool-count summary:** ~11 new Rust crates (vs A's 9, vs B's 12+ Rust crates plus 20-40 skill crates). Aggregate net-add ~4000-6000 LOC (vs A's 3000-4500, vs B's 10000-20000). Bounded, single-cycle migration scope (vs B's multi-cycle). **Cycle 97 absorption (PR #2877 lens-4 revised range):** PR #2877's tool-surface feasibility critique revised C's aggregate net-add to **5200-8200 LOC** (~37% upper-bound increase) — PR #2877 names C as the **most under-justified estimate posture** of the three candidates. Three drivers: (1) `reconcile-mode` contract spans multi-channel inbound handling (Eva, audit, dispatch merges) plus typed-delta emit — coordination logic, not just phase wrapper; (2) plan-lifecycle pair (`plan-lifecycle` + `plan-lifecycle-check`) has state machine + CI invariants + promotion logic + stale detection — non-trivial test matrix; (3) incremental delta over A is likely low-balled (C's claimed +~1000-1500 LOC over A may drift higher given cross-mode glue + CI additions). PR #2877's net assessment moves C's migration-risk classification from "medium-high predictability" toward **medium**, narrowing the docs-stated A vs C distance. PR #2877's verified workspace calibration (38 v1 crates, median 1081 LOC, mean 2118 LOC) is shared context with A: C's per-crate ~200-500 claim is similarly ~5× simpler than v1 median.

## Migration cost from v1

- **State migration:** decompose `state.json` into per-concern files plus C-specific `state/inbound-cursor.json` and `state/plans/active/*.json` directories. One-shot tool `state-json-decompose` runs at cutover (extends A's tool to also seed inbound-cursor + plans directory). Risk: low.
- **Tool migration:** ~11 new Rust crates. Each is bounded (~200-500 LOC). Aggregate net-add ~4000-6000 LOC. Slightly larger than A; far smaller than B.
- **Prompt migration:** new prompt at `prompts/v2/orchestrator-prompt.xml`, ~50% smaller than v1 (procedural content extracted to harness; same as A). Eva-installable as drop-in replacement. The 4-mode topology means the prompt instructs the orchestrator on mode transitions and per-mode-judgment-call decisions, not full procedural sequences.
- **Workflow migration:** `cycle-runner` invokes 4-mode driver (vs A's 3-mode; vs v1 single-mode); `.github/workflows/orchestrator.yml` updated to invoke new prompt path. Workflow change is a forbidden-zone PR per redesign prompt SECTION 2 — Eva merges. Same workflow-migration risk as A.
- **Journal/Cycle-issue migration:** zero (preserved); reconcile-mode may append journal subsection if non-trivial deltas (otherwise empty).
- **Audit-repo coordination:** zero direct coordination required at cutover (audit reads main per cross-repo discipline; reconcile-mode treating `[audit-request]` events as first-class is a main-side improvement).
- **Phase 3 prototype effort:** medium. Multi-cycle build-out feasible but bounded — the 11 Rust crates can be authored over 5-10 cycles (each crate is 1-2 cycles to scaffold + test), substantially less than B's ~12+ Rust + 20-40 skill build-out.

## P1-P6 evaluation criteria compliance (audit#454 absorption, cycle 89)

- **P1 (A↔B intersection coverage hard gate):** PASS. Axis 4 git-as-substrate + Axis 7 4-mode phase boundaries + per-component file writes at named mode boundaries. Concrete mechanism: `mode-transition-check` enforces that any cluster B write target (per-component file) is committed at a named cluster A boundary (boot/reconcile/work/close mode transition). Sub-pattern coverage: (1) sync-invariants-at-init via boot-mode invariant checks; (3) failure-record-write at retry-exhaustion via `detect-abandoned-cycles` + `cycle-history-append`; (4) watchdog-release-with-state-cleanup via `detect-abandoned-cycles` cleanup hooks; (5) component-local persistence loaded at init via boot-mode per-component state-load. Sub-pattern (2) state-commit at end-of-super-step is dual-cast per audit#454 D2; not separately required. **P1 status:** same as A — PASS via single-orchestrator structural mapping.
- **P2 (cluster I substrate-fit weighting):** PASS — substrate-driven adoption. v1's substrate (GitHub-Actions-anchored, public repo, multi-actor with audit) genuinely benefits from harness-enforced policy (cluster I sub-shape 1) and quality-policy linters (cluster I sub-shape 2). Plus C-specific: plan-lifecycle CI is a substrate-fit addition (the public repo benefits from explicit plan-lifecycle to make multi-cycle work legible to Eva and audit). **P2 status:** same substrate-fit as A; one additional substrate-fit element (plan-lifecycle).
- **P3 (self-management-reduction axis):** PASS-WITH-NOTE. Per-cycle decision count estimate: ~13 LOW + ~10 MODERATE + ~1 HIGH (~24 sub-shapes adopted vs v1's 6 STRONG; vs A's 21; vs B's 43). The PASS verdict tracks A's PASS but with slightly less margin (3 more sub-shapes than A). Compared to v1 baseline: candidate adopts ~24 sub-shapes vs v1's 6 STRONG; offsets with Axis 13 medium harness (extracts ~50% procedural surface, same as A). Net per-cycle decision overhead estimate: ~25-35% lower than current v1 prompt's apparent surface (slightly less reduction than A's 30-40% due to plan-lifecycle and reconcile-mode coordination overhead). **NOTE:** the additional 3 sub-shapes beyond A are the cost of structural distinctness (plan-lifecycle + reconcile-mode + per-mode budgets). The candidate's central uncertainty: whether these 3 sub-shapes carry their weight by improving F4 + F2 + F11 detection legibility, or whether they are pure overhead.
- **P4 (lifecycle-vocabulary completeness with intersection-coverage gate):** PASS-WITH-WORK. Candidate adopts cluster C sub-shape 4 (replay) via plan-lifecycle "replay-this-plan-from-active-state" mechanism + sub-shape 5 (event-trigger) via reconcile-mode per-event handlers. Sub-shapes 1 (terminate), 2 (reset), 3 (fork) NOT adopted. Both A↔C and B↔C intersection coverage present at PARTIAL: replay declares cluster A boundary moment (init-of-named-state) + cluster B effect (per-component state-load with parameter substitution); event-trigger declares cluster A boundary moment (event-arrival) + cluster B effect (typed-delta emit to per-component state). **PASS-WITH-WORK** because cluster C completeness is partial (2/5 sub-shapes); the bypass clause (resume-only) does not directly apply since replay is a non-resume primitive. C is between A's PARTIAL-bypass and B's full-PASS on this criterion.
- **P5 (polarity-pivot exit criterion before cycle 90):** PASS at the authoring level. Candidate authored at cycle 91 — first iteration of post-cycle-90 candidate authoring (cycle 90 was the named exit cycle; cycle 91 continues per `ITERATION-UNTIL-APPROVAL`). Candidate-level not directly applicable.
- **P6 (audit-as-peer preservation slot):**
  - Criterion 1 (substrate isolation for audit role): PASS. Audit repo retained as separate process / cron / context window.
  - Criterion 2 (asynchronous-of-cycle communication discipline): PASS. Axis 12 reconcile-mode honors cross-repo asynchronous communication (cycle-cadence latency).
  - Criterion 3 (cross-repo reading discipline): PASS. No cross-repo posting; both repos read each other.
  - Criterion 4 (discovery primitive completeness, audit#455 gap): PASS. Reconcile-mode subscribes to `[audit-request]` title prefix + `audit-request` label as first-class events. Discovery-failure-mode named in audit#455 is structurally addressed by reconcile-mode treating these as first-class — better than A's boot-phase pull (A's pull is interleaved with other boot-work; C's reconcile-mode is dedicated).
  - Criterion 5 (audit-as-peer evolvability): PASS. Audit autonomy on its own protocol preserved (audit edits its own STARTUP_CHECKLIST without main approval).
  - Aggregate: 5/5 PASS. **Slightly stronger than A's 5/5 PASS-WITH-WORK on Criterion 4** because reconcile-mode is dedicated harness phase, not boot-phase side-activity. **Slightly weaker than B's 5/5 PASS** because B's reconciler agent is structurally a full session-level clean-context-reviewer pattern, while C's reconcile-mode shares context with work-mode.

## M3 v1 strengths preservation (audit#454 absorption, cycle 89)

- **Strength 1 (process-isolation via ephemeral worktrees):** PRESERVED. Substrate-inherited; GitHub Actions ephemeral runner property unchanged.
- **Strength 2 (anti-patterns as published artifact):** PRESERVED-EXTENDED. Plans-as-artifacts technical-debt directory IS the structurally-explicit anti-pattern catalog at the plan level. The retrospective + journal + `_notes/` continue serving as the broader anti-pattern catalog. v2 prompt explicitly preserves "what we will not do" sections. **Extension over A:** technical-debt plans give explicit plan-level anti-pattern semantics that A doesn't have (A's anti-patterns are journal + retrospective only).
- **Strength 3 (walkback as first-class artifact):** PRESERVED-EXTENDED. Axis 4 git-as-substrate + append-only cycle-history files preserve walkback semantics by construction (same as A). Plus plans-as-artifacts technical-debt directory makes walkback structurally legible at the plan level — promoting a plan from active to technical-debt IS a walkback. **Weaker extension than B's branching checkpoints** (B has full branch-level walkback; C has plan-level walkback only).
- **Strength 4 (clean-context audit-as-peer reviewer at session level):** PRESERVED. Mixed substrate / convention inheritance maintained: substrate (separate repo + cron) preserved by Axis 12 reconcile-mode; convention (cross-repo communication discipline) preserved by candidate's Axis 12 explicit discipline declaration. **Same as A** (no extension); **weaker than B** (B has reconciler agent as the structural session-level instantiation; C's reconcile-mode shares context with work-mode).
- **Strength 5 (multi-layer permission-policy enforcement at harness level):** PRESERVED. GitHub Actions + branch protection + claude-code permission system all unchanged. Axis 8 mechanical enforcement adds `prompt-contract-check` + `plan-lifecycle-check` as complementary layers (one more layer than A; same as B's enforcement layer count but C's layers are simpler).

## M2 self-management cost inheritance (audit#454 absorption, cycle 89)

Aggregate per-cycle decision overhead estimate (counting LOW + MODERATE + HIGH cost sub-shapes the candidate adopts):

- **LOW count:** 13 — cluster A sub-shapes 1 + 2 + 9 (super-step semantics PARTIAL via 4-mode boundary state machine — slightly stronger than A's 3-mode; per-key reducers PARTIAL via per-component file writes; process-isolation STRONG inherited); cluster B sub-shapes 2 + 5 + 8 (component-local persistence PARTIAL; repository-as-state STRONG; active-surface vs monotonic-history STRONG via cycle-history); cluster D sub-shapes 1 + 2 (anti-patterns + walkback STRONG — D2 PRESERVED-EXTENDED via plans-as-artifacts technical-debt); cluster G sub-shape 1 (audit-as-peer STRONG); cluster I sub-shape 1 (permission-policy STRONG); plus 2 LOW from new-mechanism integrations (boot/close mode coordination at LOW given deterministic state machine); plus 1 LOW for plan-lifecycle naming (lifecycle states are explicit, no decision overhead per state).
- **MODERATE count:** 10 — cluster A sub-shapes 4 + 5 (typed-channel-merger PARTIAL via per-component file writes — boundary, MODERATE; bounded-retry-with-feedback PARTIAL via loop ceilings + per-mode-runtime-budgets); cluster B sub-shapes 4 + 6 + 9 (sweep-rollup PARTIAL; top-k retrieval ABSENT→PARTIAL via wiki-search; failure-record PARTIAL via detect-abandoned-cycles); cluster C sub-shapes 4 + 5 (replay PARTIAL via plan-lifecycle; event-trigger PARTIAL via reconcile-mode); cluster F plan-lifecycle (Axis 5 = Yes adoption; MODERATE for state-machine coordination); cluster I sub-shape 2 (quality-lint orchestrator-friendly PARTIAL).
- **HIGH count:** 1 — cluster H sub-shape 3 (continuous-background gardening at MODERATE-HIGH; minimal version is gardening-sweep with stale-detection + plan-lifecycle gardening hooks, not full quality-grading rubrics — boundary case; same as A).

**Compared to v1 baseline:** v1 inherits 6 STRONG sub-shapes (process-isolation + repository-as-state + active-surface + anti-patterns + walkback + audit-as-peer-substrate); the candidate inherits 6 STRONG + ~18 PARTIAL adopted sub-shapes (vs A's 15; vs B's 37). The aggregate per-cycle decision overhead is **moderately higher than A's** (3 more PARTIAL sub-shapes: replay, event-trigger first-class, plan-lifecycle) and **substantially lower than B's**. The candidate's enable-schema-work answer: **YES, primarily via Axis 13 medium harness extracting ~50% of the current v1 prompt's procedural surface** (same primary mechanism as A). Net per-cycle decision overhead is moderately higher than A (~5-15% increase) but bounded; far below B's at the system level.

**Compared to A:** the +3 PARTIAL sub-shapes (replay, event-trigger first-class, plan-lifecycle) are the structural cost of C's distinctness from A. The benefit: F4 + F2 + F11 detection legibility improvements; cluster C P4 partial coverage; plan-lifecycle anti-pattern legibility. Whether the benefit is load-bearing is the candidate's central uncertainty.

## What this candidate gains over A (specific structural advantages)

1. **F2 (Eva-response detection) more reliable** — reconcile-mode is a named first-class harness phase; A's boot-phase pulls Eva responses as part of broader boot-work and may compete with other boot-phase activities for budget. C separates concerns.
2. **F4 (frozen-artifact lifecycle) explicit at plan level** — plans-as-artifacts gives active/completed/technical-debt named states; A has no plan-lifecycle, so plan-related freeze fragility is implicit.
3. **P4 (lifecycle-vocabulary completeness) better partial coverage** — C adopts cluster C sub-shapes 4 + 5 via plan-lifecycle + reconcile-mode; A bypasses cluster C entirely.
4. **P6 Criterion 4 (discovery primitive) stronger** — reconcile-mode treats `[audit-request]` events as first-class; A's boot-phase pull is interleaved.
5. **M3 Strength 2 (anti-patterns) extended to plan-level legibility** — technical-debt plans are structurally-explicit anti-pattern catalog; A has anti-patterns in journal/retrospective only.
6. **F8 (abandonment cascades) finer-grained detection** — per-mode runtime budgets allow earlier abandonment detection vs A's cycle-level budget.

## What this candidate gives up vs B (specific structural concessions)

1. **No dedicated reconciler agent** — reconcile-mode shares context-window with work-mode; if reconciliation logic grows, it competes for budget. B's reconciler agent has its own context-window.
2. **No branching checkpoints** — plans-as-artifacts gives sequential lifecycle, not parallel branches. Multi-cycle what-if reasoning across branches is harder.
3. **No skills abstraction** — extension surface is bounded to Rust tools.
4. **No multi-mechanism per-coordinate memory** — single orchestrator means single memory.
5. **No score-gated consolidation** — memory consolidation is manual.
6. **No continuous-background curator role** — gardening is per-cycle deslop pass + plan-lifecycle gardening hooks, not a dedicated continuously-running concern.

## Honest reflection

This candidate is the **middle path** — structurally distinct from both A and B on multiple axes. The central bet: v1's substrate is good (A's bet) AND inbound reconciliation deserves first-class harness phase status without paying multi-agent overhead (a structurally weaker version of B's bet). If both bets are right, C is the right design. If A's bet is right but reconciliation does NOT need first-class harness phase status, C is A with extra migration surface for marginal benefit. If B's bet is right (multi-agent decomposition is load-bearing), C is too conservative.

It is the candidate with **medium migration cost and medium cutover predictability** (~11 Rust crates, ~4000-6000 LOC, single-cycle scope per crate). Phase 3 prototype effort is bounded but larger than A. Rollback is straightforward (delete v2 artifacts; revert workflow YAML).

C is **not** A + targeted additions cosmetically — three structural distinctions (Axis 5 Yes; Axis 7 4-mode; Axis 12 event-driven-with-fallback) make C a different design with its own central bet, not a refinement of A. C is **not** B-light — five structural distinctions (Axis 1, 2, 4, 6, 13 all different from B) preserve A's substrate-bet and bounded migration cost.

The cycle 90 hand-off named the authoring question: "Whether C is genuinely distinct from A + targeted additions, or just a partial-B." Answer: **genuinely distinct from A on three axes; preserved-distinct from B on five axes; the middle path is its own design space, not a hybrid in the dilution sense**.

Cycle 91 produces this candidate as the third design. Eva and the audit-repo orchestrator review all three before candidate-selection checkpoint per the redesign prompt's `<audit-as-peer>` directive and `ITERATION-UNTIL-APPROVAL` discipline. The 3-candidate set is the redesign prompt's "ideally 3" target reached. Further iteration cycles will sharpen all three candidates and solicit critique before the candidate-selection checkpoint reaches Eva approval.

## Cycle 92 sharpening: central-bet validation (F2 + F4 + F11 detection legibility)

C's central bet is that **reconcile-mode + plans-as-artifacts carry their weight by improving F2 (Eva-response detection), F4 (frozen-artifact lifecycle), and F11 (post-close mutations) detection legibility** beyond what A's substrate-only-cleanup achieves. Cycle 91 hand-off named this as C's highest-leverage iteration target: the structural advantages over A are documented but the **concrete metric for "carrying their weight" was estimate-level**. This section grounds the central bet in measurable per-failure-mode metrics. Direction-vs-magnitude discipline (cycle 91 methodological pattern) applies: structural improvement direction is validated by enumeration; magnitude requires Phase 3 prototype validation.

### F2 (Eva-response detection) — measurable how

**Metric:** detection-latency in cycles between Eva-comment-time and orchestrator-acknowledgment-time, plus reliability-rate (% of Eva-comments detected within 1 cycle of arrival).

**A's expected behavior:** boot-phase pull is interleaved with other boot-work (state-load, journal-read, audit-cursor advance, standing-directive check). Eva-response detection is one of ~9 boot-phase responsibilities. Latency = cycle-cadence (~6h at 4 cycles/day cron). Reliability is bounded by boot-phase completion — if boot-phase aborts before reaching Eva-response pull, detection slips by 1 cycle.

**C's expected behavior:** reconcile-mode is a named first-class harness phase between boot and work. Eva-response detection is one of ~3 reconcile-mode responsibilities (Eva-response pull + audit-post pull + dispatch-PR-merge events). Latency = same cycle-cadence (no in-cycle event arrival in either A or C). Reliability is structurally improved: reconcile-mode runs after boot completes (boot-failure mode doesn't slip Eva-response detection); reconcile-mode has its own ~10min budget allocation (Eva-response pull doesn't compete with audit-cursor advance for boot-phase budget).

**Quantified estimate of F2 improvement (C vs A):**
- Latency: identical at cycle-cadence (both ~6h). **No improvement on absolute latency.**
- Reliability: **the cycle 92 absolute numbers (A ~95% / C ~99%) are unanchored estimates.** The retrospective's F2 section documents a specific incident (6+ days of missed background Eva responses) but does not state a general reliability rate. The ~95% rate assumes boot-phase exhaustion in ~1 of 20 cycles without referencing a measured failure-count observation. The ~99% rate assumes reconcile-mode edge-case failure at half the boot-exhaustion rate, also without basis. Per cycle 96 absorption (PR #2878 Finding 9), the **~4pp reliability improvement claim is re-classified as "ambiguous-without-prototype"** — the structural argument that a dedicated harness phase is more reliable than an interleaved boot-phase responsibility holds (direction), but the absolute numbers and the ratio between them are anchored guesses, not measurements. The ~4pp differential is within reasonable guess error of either estimate.
- Detection legibility (cycle-issue-comment record): A's pull is buried in boot-phase summary; C's reconcile-mode produces dedicated subsection if non-trivial deltas. **Higher legibility for Eva auditing.** (Categorical, not quantified.)

### F4 (frozen-artifact lifecycle) — measurable how

**Metric:** plan-state-distribution per cycle (count of plans in active / completed / technical-debt states); stale-active-plan detection lag (cycles between plan-becomes-stale and orchestrator-promotes); promotion-decision quality (% of technical-debt-promoted plans that were indeed deferred work, not just abandoned).

**A's expected behavior:** no plan-lifecycle. Plans live in `_notes/` and `docs/redesign/_notes/` as freeform documents with no named lifecycle states. Stale-detection is implicit (orchestrator may notice during gardening-sweep, but no structured trigger). Frozen-artifact fragility manifests as `_notes/` documents that stop being updated — orchestrator may not notice without explicit per-cycle review.

**C's expected behavior:** plan-lifecycle states (active / completed / technical-debt) are named directories. `plan-lifecycle` Rust tool tracks last-updated cursor per active plan. Stale-active-plan detection is structured: plans not updated in N cycles flagged for orchestrator review. Promotion-decisions go through plan-lifecycle CI (`plan-lifecycle-check` ensures named owner, last-updated cursor, promotion criteria).

**Quantified estimate of F4 improvement (C vs A):**
- Plan-state legibility: A has implicit lifecycle (zero named states); C has 3 named states. **Categorical improvement.**
- Stale-detection lag: A relies on gardening-sweep (~per-cycle) but with no structured trigger (~3-5 cycle slip in practice from cycle 87 _notes-aging observation); C has structured trigger (~1 cycle slip — staleness fires at next cycle's reconcile-mode). **~3-4 cycle reduction in detection lag.**
- Promotion-decision quality: A has no promotion concept; C has structured promotion with CI-checked invariants. **Categorical improvement.**

### F11 (post-close mutations) — measurable how

**Metric:** post-close-mutation-detection-rate (% of mutations to in-tree state files between cycle close and next-cycle boot that are detected by reconcile-mode); reconcile-mode delta-emit rate (count of typed-deltas per cycle from reconcile-mode); missed-mutation-rate (post-close mutations that never get reconciled).

**A's expected behavior:** boot-phase reads state files at cycle-start. Post-close mutations (e.g., Eva edits a `state/standing-directives.json` between cycles, or a Copilot dispatch PR-merge updates dispatch-queue cursor) are noticed in boot-phase but the noticing is interleaved with other boot-work. Detection is coarse-grained (file-state at boot-time vs file-state at last-close-time). **Detection rate ~85-90%** based on cycle 75-83 cold-reader observation that some post-close mutations require explicit re-read mid-cycle when downstream-decisions surface stale state.

**C's expected behavior:** reconcile-mode reads state files in dedicated phase. Per-channel handlers compare last-cycle's state-cursor vs current state. Typed-delta emit captures every state-file mutation. Detection is structured (per-channel handler per state-file).

**Quantified estimate of F11 improvement (C vs A):**
- Detection rate: A at ~85-90%; C at ~95-98% (per-channel handlers reduce miss rate). **~5-13 percentage point improvement.**
- Delta-emit legibility: A's noticing is implicit (orchestrator may journal "post-close mutation detected" but no structured record); C's typed-deltas are first-class records in cycle-history. **Categorical legibility improvement.**
- Missed-mutation-rate: A has implicit recovery (cold-reader cycle catches missed mutations later, with multi-cycle slip); C has structured detection per cycle. **~2-3 cycle reduction in mutation-recovery slip.**

### Aggregate central-bet validation

C's central bet — that reconcile-mode + plans-as-artifacts carry their weight — is **structurally validated by per-failure-mode enumeration**:

- F2: ~4pp reliability improvement + higher legibility (real but small).
- F4: categorical improvement on plan-lifecycle legibility + ~3-4 cycle reduction in stale-detection lag (substantial).
- F11: ~5-13pp detection-rate improvement + categorical legibility improvement (substantial).

**Direction:** structural advantages over A are real (validated by enumeration). The 3 additional sub-shapes adopted by C beyond A (replay PARTIAL via plan-lifecycle; event-trigger PARTIAL via reconcile-mode; plan-lifecycle MODERATE state-machine coordination) **carry their weight on F4 and F11**; F2 improvement is more marginal.

**Magnitude:** structural enumeration suggests improvements are real but the absolute magnitudes (4pp / 3-4 cycle / 5-13pp) are estimates from cycle observation not Phase 3 prototype measurement. Magnitude validation requires running paired cycles (A-style boot-poll vs C-style reconcile-mode) on the same workload.

### Plan-authoring discipline taxonomy (cycle 107 specification, addressing Risk 2)

**Background:** Risk 2 named C's conditional-improvement concern: F4 stale-detection-lag improvement (~3-4 cycle reduction) is conditional on cycles producing structured plans. Cycles that don't author plans inherit A's F4 behavior — the improvement degrades to ~0pp on those cycles. The cycle 92 sharpening identified the conditional but did not specify (a) which cycle-types must author/update plans, (b) what compliance rate counts as "discipline holds", (c) how compliance is measured non-circularly. This section closes Risk 2 at the specification level by decomposing cycles into a 5-type taxonomy with per-type plan-discipline expectations, specifying compliance-rate thresholds as ratio-based per cycle 103/106 pattern (now extended from quantity-ratios to discipline-compliance rates), specifying per-cycle classification annotation + external-observer reproducibility check + CI sweep as the verification mechanism, and treating the taxonomy itself as subject to a warm-up window per cycle 106 NOVEL shape #24. Per cycle 96 discipline-bar-too-low: thresholds are pre-agreed before measurement. Cycle 107 closure mirrors cycle 103/105/106 closure shape (substrate-decomposition + falsifiable bound + verification procedure + status); the recurring shape is the **fourth instance of the `risk-closure-at-specification-level` functional-class shape**, and the **first instance applying it to a discipline-conditional risk-shape type** (vs the 3 prior closures of quantity-bounded risk-shape types). Transfer test: does shape #21 hold across risk-shape types in addition to risk-domain types?

**Choice 1: Cycle-type taxonomy as 5-type substrate-decomposition.**

Cycles vary substantially in what plan-discipline applies. The cycle 92 risk treated "cycles authoring plans" as a binary observation without specifying which cycles count. Under the cycle-type taxonomy:

| Type | Description | Plan-discipline expectation | Cycle character dependence |
|---|---|---|---|
| 1 — Substantive-novel | Initiates new direction, new structural change, or multi-cycle research/build arc | Plan-authoring REQUIRED (new active plan created OR existing draft elevated to active) | High-load cycle character |
| 2 — Substantive-iteration | Refines existing plan, sharpens an existing candidate-claim, deepens an existing closure | Plan-update REQUIRED (active plan touched within cycle) | Steady-state cycle character |
| 3 — Bounded-mechanical | Output is mechanical (tracker updates, restructure, mechanical edits) covered by no plan or by a parent plan | Plan-update OPTIONAL (update parent plan if applicable; no new plan required) | Mechanical cycle character |
| 4 — Reactive-only | Substantive output is responding to external arrivals (Eva input, audit critique, dispatch return) | Plan-update REQUIRED on affected plan, OR technical-debt promotion if arrival triggers walkback | External-arrival-dependent |
| 5 — No-substantive | Bottleneck-blocked; no substantive work possible | NO-PLAN annotation REQUIRED (positive declaration) | Bottleneck-dependent |

Looking at cycles 90-106 retrospectively (Phase 2 stretch): ~4 Type-1 (cycles 90, 91, 93, 94) + ~13 Type-2 (cycles 92, 95-106) + 0 Type-3/4/5 in the 17-cycle window. Phase 1 cycles 14-89 included substantial Type-3 (bounded-mechanical: cycles 23, 32, 33, 46, 56, 60, 61) and Type-4 (reactive-only: cycles 7, 27, 31, 41, 43, 85) populations.

**Alternative considered and rejected:** treat plan-discipline as binary (cycles either author or don't). Rejected because the binary observation conflates substantive-novel (where plan-authoring is the discipline) with bounded-mechanical (where it isn't expected). Without the taxonomy, the "discipline holds" claim is uninterpretable — a high authoring rate could mean discipline is good OR could mean the cycle-distribution is skewed toward Type 1.

**Alternative considered and rejected:** classify cycles only at retrospective analysis (no per-cycle classification). Rejected because retrospective classification is not falsifiable — classifying after seeing whether the plan was authored creates self-fulfilling categorization. Per-cycle classification at session-end (before knowing the prototype-level discipline-rate) is the falsifiability requirement.

**Choice 2: Per-cycle-type compliance rate as ratio-based bound.**

Per cycle 103/106 ratio-based threshold pattern, compliance rates extend the ratio discipline from quantity-ratios to discipline-compliance rates:

- **Type 1 (substantive-novel) compliance rate**: % of Type-1 cycles authoring or elevating a plan
  - ≥90% steady-state → discipline holds
  - 70-90% → discipline at risk; investigation triggered
  - <70% → discipline refuted; cycle 92 conditional fires (F4 improvement degrades to A baseline)
- **Type 2 (substantive-iteration) compliance rate**: % of Type-2 cycles updating an active plan
  - ≥90% steady-state → discipline holds
  - 70-90% → discipline at risk
  - <70% → discipline refuted
- **Type 3 (bounded-mechanical)**: no compliance bound; observational only
- **Type 4 (reactive-only) compliance rate**: % of Type-4 cycles updating affected plan or promoting
  - ≥80% steady-state → discipline holds (lower bar than Type 1/2; reactive cycles may legitimately have no affected plan if arrival is fully novel)
  - 60-80% → discipline at risk
  - <60% → discipline refuted
- **Type 5 (no-substantive) annotation rate**: % of Type-5 cycles with explicit NO-PLAN annotation
  - ≥95% → discipline holds (annotation IS the only discipline for Type 5)
  - <95% → discipline refuted

**Alternative considered and rejected:** uniform compliance rate across all types (e.g., ≥90% everywhere). Rejected because Type 4's "discipline holding" admits more legitimate exception cases (fully-novel arrivals) than Type 1/2; a uniform threshold would be either too lenient on Type 1/2 or too strict on Type 4. The asymmetry is structurally grounded.

**Alternative considered and rejected:** absolute count thresholds (e.g., "at least N plan-authoring cycles per N cycles"). Rejected because absolute counts depend on the cycle-type distribution; a stretch of mostly Type-3 bounded-mechanical cycles would fail an absolute count without indicating any actual discipline failure. Ratio-based thresholds (per cycle 103) are robust to distribution shifts.

**Choice 3: Stale-active-plan promotion lag as separate quantitative bound.**

Independent of plan-authoring/update compliance rates, stale-active-plan promotion lag is the F4 metric directly grounding the cycle 92 "~3-4 cycle reduction in detection lag" claim:

- **Stale detection: triggered when an active plan has not been updated in N cycles** (N = 5 by default; configurable per plan)
- **Promotion-lag bound: ≤3 cycles steady-state** between detected staleness and orchestrator decision (promote-to-completed | promote-to-technical-debt | mark-active-with-justification)
- **Pre-agreed thresholds:**
  - Promotion lag ≤3 cycles → F4 stale-detection-lag improvement validated (matches cycle 92 estimate)
  - 4-6 cycles → F4 improvement supported (within reasonable bounds; cycle 92 claim weakened)
  - >6 cycles → F4 improvement refuted (degrades to A's gardening-sweep behavior)

**Alternative considered and rejected:** auto-promote stale active plans without orchestrator decision. Rejected because auto-promotion without judgment loses the discipline benefit (technical-debt directory becomes a dump of forgotten plans rather than a structured anti-pattern catalog per M3 Strength 2 EXTENDED). The orchestrator-decision-required step is the discipline.

**Alternative considered and rejected:** weaker promotion-lag bound (≤5 cycles). Rejected because cycle 92's claim was "~3-4 cycle reduction in detection lag"; a ≤5 cycle bound matches A's baseline performance, voiding the C-vs-A improvement claim and making the cycle 92 claim non-falsifiable.

**Choice 4: Per-cycle classification annotation + external-observer reproducibility check + CI sweep as verification procedure.**

Each cycle session-end produces a structured classification annotation:

```jsonc
{
  "cycle": <cycle-id>,
  "cycle-type": 1 | 2 | 3 | 4 | 5,
  "plan-authoring-activity": "authored" | "elevated" | "updated" | "no-plan-needed" | "no-substantive",
  "active-plans-touched": [<plan-id>, ...],
  "stale-active-plans-detected": [{plan-id, last-updated-cycle, decision}, ...]
}
```

To address self-classification circularity (parallel to cycle 103's self-report-by-role-session-AI concern on B's counting protocol), the verification procedure has three layers:

1. **Per-cycle annotation by orchestrator** (primary; logged at session-end via `cycle-type-annotation` Rust tool).
2. **External-observer reproducibility check at Phase 3 measurement** — an external classifier (Eva, audit-orchestrator, or independent Copilot dispatch) re-classifies a sample (≥20% of cycles) without seeing the orchestrator's annotation. Inter-rater disagreement >20% indicates classification-rubric-stability concern; ≤10% is the steady-state acceptance threshold (matching cycle 103's inter-rater reliability check pattern).
3. **CI sweep (`cycle-type-annotation-check`)** verifies each cycle has a classification annotation; missing annotations are themselves a discipline-failure signal counted toward Type 5 non-compliance.

**Alternative considered and rejected:** rely on orchestrator self-classification alone (no external observer). Rejected because that's exactly the cycle 96 self-report-circularity concern — the orchestrator that authored the plan-discipline taxonomy is the same orchestrator measuring compliance. Self-classification is the primary signal; external-observer is the falsifiability mechanism.

**Alternative considered and rejected:** rely on session-end summary text (no structured annotation). Rejected because session-end summaries are unstructured prose; aggregating compliance rates from prose requires LLM-classification per cycle, which is itself a discipline-rubric-stability concern and substantially more expensive.

**Choice 5: Cycle-type taxonomy stability as warm-up-window discipline (cycle 106 shape #24 transfer).**

The cycle-type taxonomy may itself be unstable in early Phase 3 cycles as observed cycle-distributions diverge from the 5-type design. This is a direct application of cycle 106 NOVEL shape #24 (`risk-closure-with-warm-up-window`):

- **Steady-state taxonomy stability past cycle 5 of Phase 3 measurement**: ≤10% of cycles classified into "other" or "boundary case"
- **Higher rates in cycles 1-5 (taxonomy-tightening period)** are expected; sustained "other" rate >10% past cycle 5 indicates the 5-type taxonomy is under-specified

The discipline transfers: pre-agreed warm-up window (cycles 1-5) + pre-agreed post-warm-up threshold (≤10% other-rate steady-state). Shape #24 promotes from NOVEL@1 (cycle 106) to TESTED@2 (cycle 107).

**Alternative considered and rejected:** measure taxonomy stability from cycle 1 with no warm-up. Rejected per cycle 106 reasoning (rubric-tightening period is structurally expected; no-warm-up conflates rubric-evolution with structural-instability).

**Alternative considered and rejected:** treat the taxonomy as fixed (no stability check). Rejected because the taxonomy is candidate-authored — a guess about cycle-distributions — and Phase 3 may surface cycle-types not in the 5-type design. Without the stability check, the compliance-rate measurement is uninterpretable when "other" rates are high. The stability check is the meta-falsifiability that makes the closure honest.

**Choice 6: Discipline-conditional rubric-fragility separate diagnostic.**

Discipline-conditional closures depend more heavily on substrate-decomposition correctness than quantity-bounded closures (where rubric is the dependency). The cycle-type taxonomy IS the rubric for compliance — if it's wrong, the compliance rate is uninterpretable. This is structurally distinct from cycle 103/106 quantity-bounded closures where rubric ambiguity affects both numerator and denominator equally (ratio-robust): for discipline-conditional, rubric ambiguity affects WHICH cycles are expected-to-comply but not WHICH cycles complied — a one-sided ambiguity that ratio-based bounds don't fully resolve.

**Diagnostic table (combining compliance rate with taxonomy stability):**

| Compliance rate (Type 1+2) | Taxonomy stability (other-rate) | Diagnostic |
|---|---|---|
| ≥90% | ≤10% steady-state | Discipline holds; F4 improvement validated |
| ≥90% | >10% steady-state | Compliance rate uninterpretable; taxonomy is wrong (revise taxonomy before reading compliance) |
| 70-90% | ≤10% steady-state | Discipline at risk; investigation triggered (likely cycle-distribution skew or genuine discipline weakness) |
| <70% | ≤10% steady-state | Discipline refuted; F4 improvement degrades to A baseline; cycle 92 conditional fires |
| <70% | >10% steady-state | Discipline AND taxonomy both refuted; cycle 92 conditional fires AND closure shape requires revision |

**Alternative considered and rejected:** treat compliance rate alone as sufficient (no taxonomy-stability check). Rejected because high "other" rate masks discipline failure: a 95% compliance rate measured over only 60% of cycles (40% other-classified) reflects cherry-picked compliance rather than discipline. The two-dimensional diagnostic preserves discipline-vs-taxonomy distinction.

**Alternative considered and rejected:** combine taxonomy stability into compliance rate (e.g., count "other" cycles as non-compliant). Rejected because that conflates two failure modes (taxonomy-wrong vs discipline-failed) into one rate, losing diagnostic resolution. The two-axis diagnostic is more informative for revising the candidate after Phase 3 measurement.

### Risk 2 status post-cycle-107

- **Direction continues to hold** by construction: F4 improvement is conditional on plan-discipline holding. The mechanism Risk 2 named is real.
- **Magnitude refined** from "F4 improvement is conditional on cycles producing structured plans" to:
  - 5-type cycle taxonomy with per-type plan-discipline expectations (Choice 1).
  - Per-type compliance rates as ratio-based thresholds: Type 1+2 ≥90% steady-state / Type 4 ≥80% steady-state / Type 5 NO-PLAN annotation ≥95% (Choice 2).
  - Stale-active-plan promotion lag ≤3 cycles steady-state (Choice 3) — directly grounds cycle 92's "~3-4 cycle reduction" claim.
  - Per-cycle classification annotation + external-observer reproducibility ≥80% / inter-rater disagreement ≤10% steady-state + CI sweep as verification procedure (Choice 4).
  - Cycle-type taxonomy stability ≤10% other-rate steady-state past cycle 5 of measurement (Choice 5; warm-up window per cycle 106 shape #24).
  - Discipline-conditional rubric-fragility diagnostic table (Choice 6) preserves discipline-vs-taxonomy failure distinction.
- **Mitigation specification:**
  1. Cycle-type taxonomy applied per-cycle (Choice 1).
  2. Per-cycle classification annotation logged at session-end + CI sweep + external-observer reproducibility check (Choice 4).
  3. Per-type compliance rate measurement at Phase 3 (Choice 2).
  4. Stale-active-plan promotion lag tracking (Choice 3).
  5. Cycle-type taxonomy stability check past warm-up (Choice 5).
  6. Combined-readings diagnostic for discipline-vs-taxonomy fragility (Choice 6).
- **Mitigation specification is now concrete enough for Phase 3 prototype validation** (the taxonomy is applied per-cycle from prototype cycle 1; compliance rates are measurable by cycle 5+ steady state).
- **Operational closure deferred to Phase 3 prototype** when:
  - The cycle-type taxonomy meets actual cycle-distribution observations across ≥10 prototype cycles.
  - Per-type compliance rates are observed and compared against the cycle 107 thresholds.
  - Stale-active-plan promotion lag is measured against ≤3 cycles bound.
  - External-observer inter-rater disagreement is measured against ≤10% bound.
- **Shape #21 transfer verdict:** the structural form transfers (substrate-decomposition + falsifiable bound + verification + status); the methodological extensions are within-shape sub-applications, not new shapes. **Shape #21 promotes from HARDENED-at-3-instances (cycle 106) to HARDENED-at-4-instances (cycle 107)** — now spanning substantively different *risk-shape types* (3 quantity-bounded + 1 discipline-conditional) in addition to substantively different *risk-domain types* (meta-counting / tool-registry / coordination-stability / plan-authoring-discipline). Each candidate (A/B/C) now has at least one second-iteration sharpening closure, balancing falsifiability completeness across the three. **Shape #24 (`risk-closure-with-warm-up-window`) promotes from NOVEL@1 (cycle 106) to TESTED@2 (cycle 107)** via the cycle-type taxonomy stability check (Choice 5).

### Validation plan (cycle 93+ Phase 3 prototype work)

The estimates above are derived from cycle observation (cycles 75-91 cold-reader + boot-phase failure modes) and structural-enumeration of A's vs C's mode-topology, not yet validated by Phase 3 prototype paired-cycle measurement. Cycle 93+ Phase 3 prototype effort should:

1. Author smallest-viable `reconcile-mode` Rust crate with per-channel handlers (Eva-response, audit-post, dispatch-PR-merge).
2. Author smallest-viable `plan-lifecycle` Rust crate with active/completed/technical-debt state machine.
3. Run paired cycles: A-prototype (3-mode boot/work/close) vs C-prototype (4-mode boot/reconcile/work/close) on the same workload (e.g., a candidate-sharpening cycle similar to cycle 92). **Workload-confounding caveat (added cycle 96 per PR #2878 missing-risk for C):** "same workload" is not achievable in practice — real cycles have variable inbound events (Eva responses, audit critique landings, dispatch outputs). Paired-cycle measurement therefore requires either (a) replaying a recorded inbound stream synthetically against both prototypes, or (b) running paired cycles on consecutive cron triggers (so inbound state is identical at boot) and accepting that one-cycle-old vs current state still differs.
4. Measure F2/F4/F11 metrics across paired cycles; compare against estimates above.
5. **Hard refutation thresholds (added cycle 96 per PR #2878 Finding 18 — external falsification criterion, replacing the previous "within 50% of estimate-magnitude" threshold flagged as too permissive):** central-bet is validated if BOTH F4 stale-detection-lag improvement is **≥ 1 cycle** AND F11 detection-rate improvement is **≥ 3pp**. Either condition failing reverts the candidate to "A + plans-as-artifacts" (drop reconcile-mode). Both conditions failing refutes C's central bet outright. F2 reliability is held aside per the "ambiguous-without-prototype" re-classification above; if F2 produces a measurable improvement ≥ 2pp at paired-cycle measurement (above noise floor), it joins the central-bet validation; otherwise it is excluded from the validation criterion.
6. **Plan-authoring discipline measurement (added cycle 107 per cycle 107 closure — Choices 1-2):** classify each prototype cycle into one of 5 types per Choice 1 taxonomy at session-end; aggregate compliance rates per Choice 2 thresholds. C's F4 improvement validation requires Type 1+2 compliance rate ≥90% steady-state past cycle 5 of measurement; Type 4 ≥80%; Type 5 NO-PLAN annotation ≥95%.
7. **Stale-active-plan promotion lag tracking (added cycle 107 per Choice 3):** measure cycles between detected staleness (active plan not updated in ≥5 cycles) and orchestrator decision (promote-to-completed | promote-to-technical-debt | mark-active-with-justification). Verify against ≤3 cycle bound; this is the cycle 92 "~3-4 cycle reduction" claim's direct measurement.
8. **External-observer reproducibility check (added cycle 107 per Choice 4):** at Phase 3 measurement, an external classifier (Eva, audit-orchestrator, or independent Copilot dispatch) re-classifies a sample (≥20% of prototype cycles) without seeing orchestrator self-classification. Inter-rater disagreement >10% steady-state indicates classification-rubric-stability concern.
9. **Cycle-type taxonomy stability check (added cycle 107 per Choice 5 — warm-up window pattern):** track "other"-rate (cycles falling outside the 5-type taxonomy). Past cycle 5 of measurement, verify ≤10% other-rate steady-state. Higher rates trigger taxonomy revision before reading compliance rates per Choice 6 diagnostic.
10. **Plan-lifecycle change event logging (added cycle 110 per Risk 5 closure Choice 4):** classify each plan-lifecycle change event during Phase 3 prototype evolution into one of 5 types (Type 1 bounded-mechanical / Type 2 state-addition / Type 3 informational-field / Type 4 transition-rule-evolution / Type 5 emergency-hot-fix); log per the JSONC schema in Risk 5 closure Choice 4 with paired-CI-rule status and drift-window timing.
11. **CI-uncovered ratio U measurement (added cycle 110 per Risk 5 closure Choice 2):** at each commit affecting plan-lifecycle, compute U = uncovered/total invariants in spec; verify U ≤ 0.05 steady-state. CI sweep `plan-lifecycle-invariant-coverage-check` emits U per commit.
12. **Drift-window D measurement (added cycle 110 per Risk 5 closure Choice 3):** for Type 2/4/5 plan-lifecycle events, measure cycles between change event and paired-CI-update OR reclassification; verify D ≤ 3 (Type 2) / ≤ 5 (Type 4) / Type 5 reclassified-or-paired within ≤5 cycles.
13. **Plan-lifecycle-event classification stability check (added cycle 110 per Risk 5 closure Choice 5 — warm-up window pattern):** track other-rate (events falling outside the 5-type taxonomy). Past cycle 5 of measurement, verify ≤10% other-rate steady-state. Higher rates trigger plan-lifecycle change taxonomy revision before reading U per Choice 6 diagnostic.

If validation reveals all three improvements are real, C's PASS-WITH-NOTE on P3 holds. If F2 improvement is null (~0pp reliability difference) but F4 + F11 hold, C's distinctness from A still holds via plans-as-artifacts (Axis 5) but reconcile-mode (Axis 7 + Axis 12) is weakened — the candidate may consolidate to "A + plans-as-artifacts" rather than full middle-path.

### Plan-lifecycle CI-invariant coverage discipline (cycle 110 specification, addressing Risk 5)

Per cycle 105/106/107/108/109 closure pattern (substrate-decomposition + falsifiable bound + verification + status), Risk 5 is closed at the specification level. Cycle 92 framing identified the conditional ("if plan-lifecycle CI invariants drift, F4 detection legibility degrades to A's implicit baseline") but did not specify (a) what counts as a plan-lifecycle change that requires paired CI update vs purely informational addition, (b) what coverage rate counts as "CI-invariant discipline holds" per change type, (c) how coverage is measured non-circularly given the orchestrator both authors plan-lifecycle changes and self-reports CI-update status.

C Risk 5 is the **third discipline-conditional risk closure** after C Risk 2 (cycle 107) and A Risk 4 (cycle 109). The three share structure: a discipline (plan authoring / tool extraction / CI-invariant pairing) that applies to a subset of events with rubric-asymmetric ambiguity (rubric tightening affects WHICH events count as expected-to-comply, but each event's compliance is separately observable). Closure follows cycle 109's structure with substrate-content adapted to CI-invariant pairing. **C Risk 5 is also the first closure applying the discipline-conditional shape to a CODE-INVARIANT-COORDINATION substrate** (prior discipline-conditional closures addressed orchestrator-behavior substrates: cycle authoring discipline / prompt extraction discipline). Whether substrate-novelty introduces shape-distinguishing differences is itself a closure-level observation.

**Deferral arc:** 18 cycles (cycle 92 framing → cycle 110 closure). Compares to cycle 105's 8-cycle arc, cycle 106's 14-cycle arc, cycle 107's 15-cycle arc, cycle 108's 16-cycle arc, and cycle 109's 18-cycle arc (also 18 cycles, but from cycle 91; cycle 110 from cycle 92 makes the same calendar arc). Cycle 110's closure depends on cycle 103 three-layer verification + cycle 107 discipline-conditional substrate (warm-up window + rubric-fragility diagnostic) + cycle 108 two-tier metric discipline (drift-window as secondary derived) + cycle 109 5-type extension taxonomy template + cycle 109 candidate-pattern `exclude-by-type`. Cycle 110 absorbs **5 prior closures' substrate** — one more than cycle 109 (which absorbed 4), reflecting cycle 109's pattern-extraction also being available substrate.

#### Choice 1: 5-type plan-lifecycle change taxonomy as substrate-decomposition

Plan-lifecycle changes vary in what CI-update discipline applies. Designed 5-type taxonomy (sibling of cycle 109's extension taxonomy with substrate-content adapted to plan-lifecycle):

| Type | Description | Trigger | Discipline | Compliance bar |
|---|---|---|---|---|
| 1 bounded-mechanical | Trivial state-name rename / field-name rename / transition-rule already codified | Refactor-only commits | Paired CI update MUST exist in same commit | 100% (any unpaired Type 1 is contract violation) |
| 2 state-addition (novel-deterministic) | New plan-lifecycle state added (e.g., `deferred` alongside `active`/`completed`/`technical-debt`) | Phase 3+ state-machine evolution | Paired CI invariant added at commit OR within drift-window ≤3 cycles | ≥95% steady-state |
| 3 informational-field (judgment-surface analog) | Purely informational field added (description, justification, free-text notes — no invariant to check) | Plan annotation enrichment | NO paired CI update required (informational fields have no invariants) | ≤10% later-reclassification to Type 2 |
| 4 transition-rule-evolution (tool-deprecation analog) | Transition graph evolves (e.g., `completed` becomes non-terminal via `completed → re-opened`; or stale-detection threshold changed) | Plan-lifecycle semantics revision | CI transition-validation update documented at change AND re-validated against full transition graph within ≤5 cycles | ≥90% documented; ≥80% re-validated within window |
| 5 emergency / hot-fix | Ad-hoc invariant change under time pressure (e.g., temporary suspension of stale-detection during incident) | Incident response | Paired CI follow-up REQUIRED within ≤5 cycles, OR Type 5 reclassified to Type 1/2/3/4 | ≥80% reclassified-or-paired within ≤5 cycles |

**Alternatives rejected:**
- Binary CI-coverage discipline (every plan-lifecycle change pairs with CI or doesn't) — conflates Type 1 (mechanical refactor) with Type 3 (informational, no invariant to check); loses the discipline-applicability distinction that makes the diagnostic interpretable.
- 3-type simpler taxonomy (deterministic / informational / emergency) — collapses Type 1+2 (both deterministic-but-different time discipline) and Type 4+5 (both reactive but different cause).
- Retrospective-only classification — not falsifiable; classifying after seeing CI drift is circular per cycle 96 self-report-circularity concern.

**Substrate partition:** discipline-applicable Types 1 + 2 + 4 + 5 (paired-CI requirement applies; ~85% of expected change volume) + discipline-non-applicable Type 3 (informational; pairing would itself violate CI-as-rule-checker principle since informational fields have nothing to check; ~15% of expected volume). Type 3 is the discipline-conditional analog to cycle 107 Type 5 (NO-PLAN annotation IS the discipline) and cycle 109 Type 3 (judgment-surface NOT-extracted IS the discipline). **Candidate-pattern `exclude-by-type`** (cycle 109 NOVEL @ TESTED@3) promotes to **HARDENED@4** spanning cycle 106 Type A mechanical-excluded / cycle 107 Type 5 NO-PLAN-as-discipline / cycle 109 Type 3 judgment-surface-not-extracted / cycle 110 Type 3 informational-field-no-CI-invariant.

#### Choice 2: CI-uncovered ratio U as primary falsifiable bound (rubric-symmetric)

Per cycle 103/106/108/109 ratio-based threshold pattern:

U = (CI-uncovered plan-lifecycle invariants) / (total plan-lifecycle invariants in spec)

Pre-agreed thresholds:
- U ≤ 0.05 (≥95% covered): direction-validated; CI-coverage discipline holds; F4 detection legibility preserved
- U ∈ (0.05, 0.20]: at-risk; CI-invariant drift in progress; investigate per Choice 6
- U > 0.20: refuted; CI-coverage discipline failed; F4 detection legibility degrades to A's implicit baseline; C's plan-lifecycle improvement is undermined

U is rubric-symmetric in the cycle 108/109 sense: rubric tightening affects both numerator (uncovered invariants) and denominator (total invariants) equally. If "invariant" is redefined upward (only state-machine-validating invariants count, excluding stylistic checks), uncovered count drops AND total count drops; ratio is rubric-robust within the chosen unit.

**Alternatives rejected:**
- Absolute uncovered count as primary — depends on plan-lifecycle complexity; a richer plan-lifecycle can have more uncovered invariants without violating coverage discipline. Ratio is more interpretable.
- Type-1-specific compliance rate as primary — captures one type's discipline only; misses Type 2/4/5 drift modes; consistent with cycle 109 reasoning.
- Aggregate `plan-lifecycle-check` LOC count — rubric-asymmetric (refactoring CI rules affects LOC without changing coverage).
- Coverage ratio C = covered/total instead of U = uncovered/total — directionality flip from cycle 109 P (lower-is-better); chose U to maintain directional consistency across closures (lower-is-better for both P and U; cleaner cross-closure interpretation).

#### Choice 3: Drift-window D as secondary derived metric (cycle 108 two-tier discipline transfer)

For Type 2/4/5 plan-lifecycle changes, drift-window D = (cycles between change event and paired-CI-update OR reclassification). Per cycle 108 Choice 3 / cycle 109 Choice 3 two-tier primary-vs-secondary-derived metric discipline:

- D ≤ 3 cycles for Type 2: aggregate drift bounded; consistent with Type 2 ≥95% compliance
- D ∈ (3, 5] for Type 4: bounded under window
- D > 5 for Type 4/5: change has aged into permanent CI-coverage gap; reclassify or accept U regression

D is **secondary derived (informational)**, NOT primary falsificational, because:
- D depends on auxiliary measurements (change-event timing + paired-CI-update timing — both separately measured).
- CI-uncovered ratio U (Choice 2) already captures aggregate coverage discipline regardless of per-change drift.

Cycle 108 / cycle 109 two-tier metric discipline promotes from **TESTED@2 → HARDENED@3** at cycle 110. The methodological pattern (when a metric depends on auxiliary measurements, demote to secondary derived to avoid refutation conflation) holds at THIRD instance with substrate-content adapted from iteration-multiplier (cycle 108) → drift-window for prompt-extension (cycle 109) → drift-window for plan-lifecycle CI (cycle 110). **The pattern is now substrate-domain-general**: holds across iteration-events, prompt-extensions, AND CI-invariant pairings.

**Alternatives rejected:**
- Make drift-window primary falsifiable — conflates change-event count with paired-CI-update timing; refutation on D alone would not distinguish "many changes, fast follow-up" from "few changes, slow follow-up" (diagnostically equivalent under D, structurally distinct under per-type compliance).
- Omit drift-window entirely — loses diagnostic resolution for Type 4/5 reactive changes; binary "paired or not" misses the time-pressure interaction.

#### Choice 4: Three-layer verification (cycle 103/107/108/109 transfer)

Per-plan-lifecycle-event log schema:
```jsonc
{
  "event-type": 1 | 2 | 3 | 4 | 5,
  "plan-lifecycle-element": "state" | "field" | "transition-rule" | "stale-detection-rule",
  "element-name": "<element identifier>",
  "trigger-cause": "<cycle-event description>",
  "cycle": <cycle-id>,
  "paired-ci-rule": "<ci-rule-name>" | null,
  "paired-ci-status": "exists" | "follow-up-required-by-cycle-N" | "not-required",
  "drift-window-start-cycle": <cycle-id> | null,
  "drift-window-end-cycle": <cycle-id> | null,
  "decision-rationale": "<why this event type vs another>"
}
```

Three-layer verification (inheriting cycle 103/107/108/109 pattern intact):
1. Per-plan-lifecycle-event logging by orchestrator at change time (primary).
2. External-observer reproducibility check at Phase 3 measurement (≥20% sample re-classified by Eva, audit-orchestrator, or independent Copilot dispatch; inter-rater disagreement ≤10% steady-state past cycle 5 of measurement).
3. CI sweep (`plan-lifecycle-invariant-coverage-check` — verifies each declared plan-lifecycle invariant in the spec has a corresponding CI rule, tracks drift-windows for Type 2/4/5, and emits the U metric per commit).

**Three-layer verification at HARDENED-at-5 across 5 closure types** (cycle 103 counting protocol, cycle 107 plan-authoring discipline, cycle 108 iteration events, cycle 109 extension events, cycle 110 plan-lifecycle CI events). Cycle 109 named candidate functional-class shape #25 (`three-layer-closure-verification`) at NOVEL@1; cycle 110 promotes shape #25 from NOVEL@1 → **TESTED@2** with substrate-content adapted from prompt-extension events (cycle 109) to plan-lifecycle CI events (cycle 110). At TESTED@2 with high cross-substrate generality, shape #25 strengthens its case for promotion from "shape #21 sub-element" to its own functional-class shape; cycle 111+ may HARDEN.

**Alternatives rejected:**
- Self-classification alone — exact cycle 96 self-report-circularity concern; orchestrator self-grading on whether its own plan-lifecycle change was Type 1 vs Type 2 is the same shape as B's self-counting concern.
- Manual tracking via journal entries — unstructured, not CI-checkable, classification quality varies cycle-to-cycle.
- Instrument every commit affecting plan-lifecycle (full diff classification) rather than change-specific events — introduces classification overhead on non-structural commits (typo fixes, formatting); per cycle 106 mechanical-vs-structural reasoning, mechanical edits are excluded from the structural unit.

#### Choice 5: CI-invariant rubric stability as warm-up window discipline (cycle 107/108/109 shape #24 transfer)

Direct application of cycle 107/108/109 shape #24:
- Warm-up window: cycles 1-5 of Phase 3 measurement. Plan-lifecycle change classification rubric is being tightened (what counts as Type 1 mechanical-rename vs Type 2 state-addition; whether stale-detection threshold change is Type 4 transition-evolution or Type 1 mechanical; whether emergency hot-fix that gets reclassified as informational is Type 5 success or Type 3 misclassification).
- Steady-state: past cycle 5 of measurement. Per-type compliance rates apply. Inter-rater disagreement on event type ≤10% expected.
- Combined-readings against rubric stability: see Choice 6.

**Shape #24 promotes HARDENED@4 (cycle 109 extension-classification stability) → HARDENED@5 (cycle 110 plan-lifecycle-event-classification stability)** spanning 5 protocol-stability types: measurement-bound stability (cycle 106) + rubric stability (cycle 107) + classification stability (cycle 108) + extension-rubric stability (cycle 109) + plan-lifecycle-event-rubric stability (cycle 110).

Critical methodological observation: cycle 110 is **rubric-asymmetric** like cycle 107 and cycle 109 (discipline-conditional rubric-fragility) — rubric tightening affects WHICH events count as expected-to-comply (Type 1 vs Type 3 boundary; or Type 2 vs Type 4 distinction) but each event's pairing/non-pairing is separately observable. The discipline-conditional risk-shape type now has 3 instances all showing rubric-asymmetric character at the warm-up boundary; cycle 109 _notes observation 7 (discipline-conditional commonalities at 2 instances) extends to **3 instances confirming the rubric-asymmetric character is structural, not coincidental**.

**Alternatives rejected:**
- Measure plan-lifecycle changes from cycle 1 with no warm-up — conflates rubric-evolution with CI-coverage-instability per cycle 106/107/108/109 reasoning.
- Treat plan-lifecycle change classification rubric as fixed (no stability check) — without stability check, Type-distribution and per-type compliance rates are uninterpretable when classification disagreement is high.

#### Choice 6: Combined-readings diagnostic preserving discipline-conditional rubric-fragility distinction (third instance)

Per cycle 107 Choice 6 / cycle 109 Choice 6 discipline-conditional rubric-fragility diagnostic, this is the **third instance** of the discipline-conditional rubric-fragility pattern. C Risk 5 has the same one-sided rubric ambiguity as C Risk 2 and A Risk 4: rubric tightening affects WHICH events count as expected-to-comply (Type 1/2/4/5 vs Type 3 boundary; or Type 2 vs Type 4 distinction) but each event's compliance is separately observable.

A 4-quadrant matrix:

| CI-uncovered ratio U | Plan-lifecycle-event rubric stability (other-rate past cycle 5) | Diagnostic |
|---|---|---|
| U ≤ 0.05 | ≤ 10% other-rate | C Risk 5 direction validated; CI-coverage discipline holds; F4 detection legibility preserved |
| U ≤ 0.05 | > 10% other-rate | Coverage is cherry-picked over a rubric-unstable subset; revise plan-lifecycle change taxonomy before reading U-value (possibly Type 3 misclassification masking Type 2 drift) |
| U > 0.20 | ≤ 10% other-rate | Risk 5 fires; CI-coverage discipline failed; magnitude refuted (rubric is sound) |
| U > 0.20 | > 10% other-rate | Both rubric and discipline fail; revise plan-lifecycle change taxonomy first; if revised rubric still produces U > 0.20, accept that C's F4 improvement degrades to A's baseline |

Discipline-conditional rubric-fragility diagnostic promotes from **TESTED@2 (cycle 109) → HARDENED@3 (cycle 110)**. The diagnostic structure (combined-readings table preserving discipline-vs-rubric-stability distinction) holds across three different discipline-conditional risk substrates — plan-authoring (cycle 107), prompt-extension (cycle 109), CI-invariant-coverage (cycle 110). Methodological observation: the two-axis combined-readings table is the **structural diagnostic shape** for discipline-conditional rubric-fragility — one axis is the primary falsifiable bound (compliance rate / P / U), the other is rubric-stability (other-rate / inter-rater disagreement). At HARDENED@3 across 3 substrate domains, the diagnostic generalizes.

**Alternatives rejected:**
- CI-uncovered ratio U alone — high "other" rate masks discipline failure (cherry-picked coverage over a rubric-unstable subset).
- Combine taxonomy stability into U (count "other" as Type-2 non-compliant) — conflates two failure modes; loses diagnostic resolution for revising the candidate post-Phase-3.
- Use cycle 108's combined-readings (per-role decision count × iteration events) — orthogonal metrics; doesn't apply to CI-invariant-coverage substrate where the relevant axes are U and rubric-stability.

### Risk 5 status post-cycle-110

- **Direction continues to hold** by construction: F4 improvement is conditional on plan-lifecycle CI invariants tracking the spec. The mechanism Risk 5 named is real.
- **Magnitude refined** from "if plan-lifecycle CI invariants drift, F4 detection legibility degrades to A's implicit baseline" to: 5-type plan-lifecycle change taxonomy / per-type compliance rates / CI-uncovered ratio U bounded ≤0.05 / drift-window D as secondary derived metric / three-layer verification / warm-up-window plan-lifecycle-event classification stability / combined-readings diagnostic preserving discipline-conditional rubric-fragility distinction.
- **Mitigation specification is now concrete enough for Phase 3 prototype validation** (plan-lifecycle change events logged from cycle 1 of Phase 3; per-type compliance rates measurable by cycle 5+ steady state; U measurable per-commit).
- **Operational closure deferred** to Phase 3 prototype when plan-lifecycle change-event logging meets actual `plan-lifecycle` crate evolution events AND inter-rater disagreement on event type ≤10% steady-state AND per-type compliance rates measured across ≥10 measurement cycles AND drift-window D measured for Type 2/4/5 changes.
- **Shape #21 transfer verdict:** structural form transfers (substrate-decomposition + falsifiable bound + verification + status); cycle 110 instance is the **third discipline-conditional application** of shape #21 (after cycle 107 plan-authoring discipline and cycle 109 extension-discipline). **Shape #21 promotes HARDENED-at-6 → HARDENED-at-7** spanning 7 risk-domain types (meta-counting-protocol / tool-registry-growth / coordination-stability / plan-authoring-discipline / per-role-iteration / extension-discipline / plan-lifecycle-CI-coverage) AND 2 risk-shape types (4 quantity-bounded + 3 discipline-conditional). The discipline-conditional risk-shape type promotes from TESTED@2 (cycle 109) → **HARDENED@3 (cycle 110)** at the risk-shape-type level — three substrate-distinct discipline-conditional applications confirm the shape's risk-shape-type generality.
- **Shape #24 transfer verdict:** cycle 110 plan-lifecycle-event-classification stability is the **fifth stability type** after measurement-bound (cycle 106) + rubric (cycle 107) + classification (cycle 108) + extension-rubric (cycle 109). **Shape #24 promotes HARDENED@4 → HARDENED@5** spanning 5 protocol-stability types.
- **Shape #25 candidate (`three-layer-closure-verification`) transfer verdict:** cycle 109 named at NOVEL@1; cycle 110 transfers intact. **Shape #25 promotes NOVEL@1 → TESTED@2** with substrate-content adapted from prompt-extension events to plan-lifecycle CI events. At TESTED@2 with high cross-substrate generality, shape #25's case for HARDENING strengthens; cycle 111+ may promote to HARDENED@3.
- **Discipline-conditional rubric-fragility diagnostic** (cycle 107 NOVEL@1 → cycle 109 TESTED@2) promotes **TESTED@2 → HARDENED@3** at third instance with substrate-content adapted from plan-authoring (cycle 107) to extension-discipline (cycle 109) to CI-invariant-coverage (cycle 110). At HARDENED@3 across 3 substrate domains, the diagnostic structure is **the** discipline-conditional rubric-fragility diagnostic shape.
- **Two-tier primary-vs-secondary-derived metric discipline** (cycle 108 NOVEL@1 → cycle 109 TESTED@2) promotes **TESTED@2 → HARDENED@3** at third instance with substrate-content adapted from iteration-multiplier (cycle 108) to drift-window for prompt-extension (cycle 109) to drift-window for plan-lifecycle CI (cycle 110). The pattern is now substrate-domain-general.
- **Candidate-rotation-balance-discipline** (cycle 107 NOVEL@1 → cycle 109 TESTED@2) promotes **TESTED@2 → HARDENED@3** by cycle 110 honoring the cycle 109 _notes-named rotation commitment (C Risk 5 chosen over higher-priority B-side targets to balance closure distribution toward A:2 / B:3 / C:2).
- **Candidate-pattern `exclude-by-type`** (cycle 109 TESTED@3) promotes **TESTED@3 → HARDENED@4** spanning cycle 106 Type A mechanical-excluded / cycle 107 Type 5 NO-PLAN-as-discipline / cycle 109 Type 3 judgment-surface-not-extracted / cycle 110 Type 3 informational-field-no-CI-invariant.

## Risk-closure threshold summary (cycle 121 consolidation)

Cycles 103-110 closed 7 risks across the three candidates at the specification level. C holds 2 of these (Risks 2 and 5); both follow shape #21 (substrate-decomposition + falsifiable bound + verification + status) AND both are discipline-conditional applications of the cycle 107 NOVEL@1 → cycle 110 HARDENED@3 discipline-conditional rubric-fragility diagnostic. The closures are detailed in the per-cycle sections above; this table consolidates them as a single Phase-3-measurement-planning entry point. Risks 1/3/4/6/7/8/9 named at the structural level remain open with direction-supporting arguments only; Phase 3 prototype measurement is the path to operational closure for those as well.

| Risk | Description | Closure cycle | Primary falsifiable threshold | Verification mechanism | Phase 3 operational closure target |
|---|---|---|---|---|---|
| **2** | Plan-authoring discipline conditional; cycles without structured plans inherit A's F4 behavior (lifecycle empty, F4 improvement 0pp) | 107 | 5-type cycle taxonomy compliance rates: **Type 1+2 ≥90% direction-validated** / 70-90% at-risk / <70% refuted; **Type 4 ≥80%**; **Type 5 NO-PLAN annotation ≥95%**; **stale-active-plan promotion lag ≤3 cycles** steady-state (directly grounds cycle 92 "~3-4 cycle reduction" claim) | 5-type cycle taxonomy (substantive-novel / substantive-iteration / bounded-mechanical / reactive-only / no-substantive); per-cycle classification annotation (JSON schema); external-observer reproducibility ≥20% sample with ≤10% inter-rater disagreement; CI sweep; cycle-type taxonomy stability ≤10% other-rate steady-state past cycle 5 (warm-up window shape #24 NOVEL@1 → TESTED@2); combined-readings diagnostic preserving discipline-vs-taxonomy failure distinction | When cycle-type taxonomy meets actual cycle-distribution observations across ≥10 prototype cycles; per-type compliance rates observed against cycle 107 thresholds; stale-promotion lag measured against ≤3 cycle bound; inter-rater disagreement ≤10% steady-state |
| **5** | Plan-lifecycle CI (`plan-lifecycle-check`) drift erodes F4 detection legibility to A's implicit baseline (paired risk with A's Risk 4 at different layer) | 110 | CI-uncovered ratio **U ≤ 0.05** direction-validated / (0.05, 0.20] at-risk / **>0.20 refuted**; drift-window **D ≤ 3** (Type 2) / **≤ 5** (Type 4) / Type 5 reclassified-or-paired within ≤5 cycles (secondary derived per cycle 108/109 two-tier discipline) | 5-type plan-lifecycle change taxonomy (bounded-mechanical / state-addition / informational-field / transition-rule-evolution / emergency-hot-fix); per-plan-lifecycle-event log (JSONC schema); external-observer reproducibility ≥80% with ≤10% disagreement; CI sweep `plan-lifecycle-invariant-coverage-check`; warm-up-window plan-lifecycle-event classification stability ≤10% other-rate steady-state past cycle 5 (shape #24 HARDENED@4 → HARDENED@5); combined-readings 4-quadrant diagnostic preserving discipline-conditional rubric-fragility distinction (third instance after cycles 107/109) | When plan-lifecycle change-event logging meets actual `plan-lifecycle` crate evolution events; inter-rater disagreement on event type ≤10% steady-state past cycle 5; per-type compliance rates measured across ≥10 measurement cycles; drift-window D measured for Type 2/4/5 changes |

**Cross-candidate context:** C's 2 closures position C as discipline-conditional (both Risks 2 + 5 are discipline-conditional rubric-fragility instances). Distribution: A:2 (Risks 3+4) / B:3 (Risks 8+2+4) / C:2 (Risks 2+5) — closure count is approximately balanced; differential lies in risk-shape-type-mix. C's 2 closures both ground the cycle 107 discipline-conditional rubric-fragility diagnostic at NOVEL@1 → HARDENED@3 across substrate-distinct discipline domains (plan-authoring discipline + plan-lifecycle CI-invariant-coverage discipline).

**Substrate-pattern observation:** all 7 closures share the 4-element structure (substrate-decomposition + falsifiable bound + verification + status); shape #21 is HARDENED-at-7 across 7 risk-domain types and 2 risk-shape types per cycle 110 substrate. The 3-layer verification pattern (per-event log + external-observer reproducibility + CI sweep) appears in 4 of 6 closures (cycle 103 / 107 / 108 / 109 / 110) and is the candidate for shape #25 promotion to HARDENED@3 at cycle 111+.

**C-specific note on closure-paired structure:** C's Risk 5 (plan-lifecycle CI) and A's Risk 4 (prompt-contract-check CI) are structurally-paired at different layers — both address CI-invariant-coverage drift, both follow the 5-type taxonomy + falsifiable ratio + drift-window two-tier discipline + three-layer verification + warm-up-window combination. The paired structure is design-input for v2: regardless of which candidate is selected, the CI-invariant-coverage discipline transfers (A's prompt-extension-pairing-check and C's plan-lifecycle-invariant-coverage-check are sibling CI surfaces). This pairing is a cross-candidate substrate that survives Q7 resolution. Per cycle 118 finding (C is uniquely retrospective-failure-mode-grounded), Risks 2 + 5 ground C's plan-lifecycle additions in retrospective-named F4 / F11 failure modes (frozen-artifact lifecycle + post-close mutations) — the closures' specification-level grounding is one of C's Criterion 4+5 advantages over A.

### Risks named at the structural level

- **Risk 1:** the F2 improvement estimate (~4pp reliability) is small enough that prototype measurement may be within noise. If cycle counts during Phase 3 prototype are bounded (~10-20 paired cycles), F2 improvement may not be statistically distinguishable from A's baseline. F4 + F11 improvements are larger magnitude and more measurably distinguishable.
- **Risk 2 (CLOSED at specification level cycle 107):** plan-lifecycle requires plan-authoring discipline — if cycles don't author plans (e.g., absorption cycles, dispatch-poll cycles), the lifecycle is empty and F4 improvement is 0pp. The candidate's F4 improvement is conditional on cycles producing structured plans; cycles that don't (steady-state cycles) inherit A's F4 behavior. **Cycle 107 closes Risk 2 at the specification level** — see the [Plan-authoring discipline taxonomy section](#plan-authoring-discipline-taxonomy-cycle-107-specification-addressing-risk-2) above. The closure decomposes cycles into a 5-type taxonomy with per-type plan-discipline expectations (Type 1 substantive-novel REQUIRED / Type 2 substantive-iteration REQUIRED / Type 3 bounded-mechanical OPTIONAL / Type 4 reactive-only REQUIRED-or-promotion / Type 5 no-substantive NO-PLAN-annotation REQUIRED), specifies pre-agreed compliance-rate thresholds as ratio-based (Type 1+2 ≥90% direction-validated / 70-90% at-risk / <70% refuted; Type 4 ≥80%; Type 5 ≥95% annotation), specifies stale-active-plan promotion lag ≤3 cycles steady-state directly grounding the cycle 92 "~3-4 cycle reduction" claim, specifies per-cycle classification annotation with structured JSON schema + external-observer reproducibility check at Phase 3 + CI sweep as the three-layer verification procedure (addressing self-classification circularity per cycle 96 lesson), specifies cycle-type taxonomy stability ≤10% other-rate steady-state past cycle 5 of measurement (warm-up window per cycle 106 shape #24 — promotes shape #24 NOVEL@1 → TESTED@2), and provides a combined-readings diagnostic distinguishing discipline-failure from taxonomy-wrongness (the discipline-conditional rubric-fragility distinction that quantity-bounded closures don't carry). Risk 2 remains *open at the operational level* until Phase 3 prototype measurement observes the rates across ≥10 prototype cycles; the specification-level closure is the cycle 107 deliverable.
- **Risk 3:** reconcile-mode adds ~10min budget to cycle window (per Axis 9 per-mode runtime budget). If reconcile-mode handlers process inbound events efficiently, the budget is mostly idle (overhead without proportional benefit). If inbound events are heavy (audit critique landing + Eva-response + dispatch-PR-merge in same cycle), reconcile-mode may exceed budget and slip detection to next cycle anyway — degrading the F11 improvement.
- **Risk 4:** the F11 detection-rate-improvement estimate (~5-13pp) is from cycle 75-83 cold-reader observation, which itself was an iteration on A-style behavior. Phase 3 prototype paired-cycle measurement may reveal the cold-reader cycle was already capturing most missed mutations, and reconcile-mode's marginal improvement is smaller than estimated.
- **Risk 5 (CLOSED at specification level cycle 110):** plan-lifecycle CI (`plan-lifecycle-check`) adds CI surface that needs ongoing attention (similar to A's prompt-contract-check risk, but at a different layer). If plan-lifecycle CI invariants drift (e.g., new plan-lifecycle state added without paired CI update), F4 detection legibility degrades to A's implicit baseline. **Cycle 110 closes Risk 5 at the specification level** — see the [Plan-lifecycle CI-invariant coverage discipline section](#plan-lifecycle-ci-invariant-coverage-discipline-cycle-110-specification-addressing-risk-5) above. The closure decomposes plan-lifecycle changes into a 5-type taxonomy (Type 1 bounded-mechanical / Type 2 state-addition / Type 3 informational-field / Type 4 transition-rule-evolution / Type 5 emergency-hot-fix), specifies CI-uncovered ratio U as primary falsifiable bound (U ≤ 0.05 direction-validated / (0.05, 0.20] at-risk / >0.20 refuted), drift-window D as secondary derived metric per cycle 108/109 two-tier discipline (D ≤ 3 Type 2 / ≤ 5 Type 4 / >5 reclassify-or-accept), three-layer verification (per-plan-lifecycle-event log + external-observer reproducibility ≥80% with ≤10% disagreement + CI sweep `plan-lifecycle-invariant-coverage-check`), warm-up-window plan-lifecycle-event classification stability (≤10% other-rate steady-state past cycle 5), and a combined-readings diagnostic preserving discipline-conditional rubric-fragility distinction (third instance after cycles 107/109). Risk 5 remains *open at the operational level* until Phase 3 prototype measurement observes the rates across ≥10 prototype cycles; the specification-level closure is the cycle 110 deliverable.
- **Risk 6:** reconcile-mode and work-mode share the same context-window (Axis 1 single-threaded). If reconcile-mode's deltas are non-trivial, work-mode inherits a contaminated context. C's distinctness from A's boot-phase pull is structural (named harness phase) but not context-isolated; the context-isolation property B claims (per-agent context-window) is NOT inherited by C.
- **Risk 7 (added cycle 96 per PR #2878 absorption — workload-confounding in paired-cycle measurement):** C's validation plan requires paired cycles on the same workload, but real cycles have variable inbound events. If A-prototype and C-prototype cycles happen to have different inbound volumes (Eva-responses, audit critiques, dispatch returns), the F11 and F2 measurements are confounded — the C-prototype could appear better simply because its cycle had more inbound events to detect. The validation methodology must address this either via synthetic replay or paired-on-consecutive-cron sequencing. This is a **structural flaw in the validation methodology**, not just a magnitude risk.
- **Risk 8 (added cycle 96 per PR #2878 absorption — F4 categorical-fact vs operational-consequence conflation):** the F4 "categorical improvement" claim conflates two distinct claims. Fact: A has 0 named plan-states, C has 3 named states (verifiable). Operational consequence: A has "~3-5 cycle slip in practice" based on cycle 87's _notes-aging observation extrapolated to a per-cycle behavioral claim. The categorical fact is real; the operational-consequence extrapolation rests on one cycle's observation. The F4 claim should distinguish these: the named-state structural improvement is categorical; the stale-detection-lag improvement is conditional on the extrapolation holding across cycles, which is itself a measurement question.
- **Risk 9 (added cycle 97 per PR #2877 lens-3 + lens-6 absorption — reconcile-mode coordination logic + abstraction-insufficiency long-term):** PR #2877 names two related concerns about C's `reconcile-mode` crate. (a) **Per-crate scope under-estimate:** the contract spans multi-channel inbound handling (Eva-response detection, audit-post events, dispatch-PR-merge events) plus typed-delta emit per channel — coordination logic that is structurally larger than a phase wrapper. Estimated 200-500 LOC range may be optimistic; orchestration-hub-style crates in the workspace median ~1081 LOC. (b) **Abstraction insufficiency long-term:** one `reconcile-mode` crate may be insufficient abstraction for F2+F11 long-term because the inbound classes (Eva-response polling vs audit-post event subscription vs dispatch-PR-merge webhook handling) have distinct failure semantics. Expect either internal submodule growth (reconcile-mode crate grows beyond 500 LOC) or split pressure (reconcile-mode-eva + reconcile-mode-audit + reconcile-mode-dispatch as separate crates). Either outcome shifts C's per-crate-bounded narrative. **Plan-lifecycle pair:** PR #2877 also names plan-lifecycle integration cost (state machine + CI invariants + promotion logic + stale detection) as under-estimated — non-trivial test matrix; plan-lifecycle-check CI surface needs ongoing attention (similar concern to A's prompt-contract-check drift risk). Mitigation: explicit acknowledgment of `reconcile-mode` as the largest C-specific crate; allocate Phase 3 prototype budget accordingly. **Implication:** C's claim of bounded single-cycle migration scope per crate is more conditional for `reconcile-mode` than for A-shared crates; cycle 93+94 measurements (370 / 435 inc-tests) on A-shared trivial crates do NOT validate C-specific orchestration crates.

These risks are bounded — none threaten C's substrate-bet directly (A's substrate-bet is preserved). They threaten the *magnitude* of central-bet validation (F2 + F4 + F11 improvement vs A baseline) and, in the case of Risk 7, the *measurability* of central-bet validation. **Per cycle 96 absorption (PR #2878 Findings 9-11, 14):** F11 direction is the most honestly grounded single claim across all three candidates (one cited cold-reader observation source); F4 categorical fact is real but operational-consequence is extrapolated; F2 absolute numbers are unanchored estimates. Magnitude is **mixed: F11 prototype-pending; F4 ambiguous (categorical fact ≠ operational consequence); F2 ambiguous-without-prototype**. Cycle 93+ Phase 3 prototype paired-cycle measurement required for empirical magnitude validation, with the workload-confounding fix applied.

### Implication for candidate-selection checkpoint

If Eva approves C at the candidate-selection checkpoint, Phase 3 prototype work should prioritize **reconcile-mode + plan-lifecycle** as the first scaffold (the C-specific structural additions); A-shared scaffold (`tool-registry`, `boot-mode`, `wiki-search`) follows. If prototype measurement reveals F2 + F4 + F11 magnitudes below estimate, C reverts to A + plans-as-artifacts (a smaller Phase 3 prototype) rather than fully middle-path. This rollback path is structurally bounded (delete reconcile-mode crate; collapse 4-mode to 3-mode) and preserves the central-bet validation discipline.
