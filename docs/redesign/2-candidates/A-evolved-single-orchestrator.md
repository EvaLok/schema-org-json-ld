# Candidate A: Evolved Single-Orchestrator

**Authoring cycle:** 90 (2026-05-07)
**Authored against:** [`2-design-framework.md`](../2-design-framework.md) v1.21+; [`clusters.md`](../1-research/clusters.md) M-item integration arc closure (cycles 86-89).

**One-line position:** Preserve v1's substrate (single Anthropic-model session per cron tick + off-process Copilot dispatches) and **evolve incrementally** along the dimensions where v1 has visible failure modes. The candidate's central bet is that v1's substrate is more aligned with the convergent constraints than the cluster A/B/H gaps suggest, and that the load-bearing design work is structural cleanup of `state.json`, formalization of `_notes/` as memory, and extraction of procedural prompt content into the cycle-runner harness — not architectural decomposition.

## Position summary

- **Axis 1 (decomposition):** Single-threaded linear with off-process dispatches — preserves writes-stay-single-threaded as structural invariant; no in-session role decomposition. Copilot dispatches remain per-task externally-delegated work, not "agents" in the small-fixed-team sense.
- **Axis 2 (state representation):** File-per-component — `state.json` decomposed into per-concern files: `state/dispatch-queue.json` (Copilot work-in-flight), `state/eva-input-cursor.json` (last-seen input-from-eva issue id), `state/redesign/<phase>.json` (per-phase progress), `state/cycle-history/<N>.json` (per-cycle summary, append-only). Each file owned by a single concern; deletion of `state.json` is part of cutover.
- **Axis 3 (memory shape):** Wiki + search — formalize `_notes/` and `docs/redesign/_notes/` as the wiki substrate; add a `top-k retrieval over LLM-generated descriptions` mechanism (cluster B sub-shape 6 v1=ABSENT) via a Rust tool `wiki-search` that maintains an index of `_notes/*.md` with a description-frontmatter field. Journal remains long-term reflective; `_notes/` is medium-term structured. No semantic embedding infrastructure (HIGH-cost cluster B sub-shape 7 deferred).
- **Axis 4 (history substrate):** Git-as-substrate — every commit is the audit trail; per-cycle summaries land in `state/cycle-history/<N>.json` as append-only files (no in-place rewrites). The git-safety preserved primitive (commit-must-be-pushed) naturally honors Axis 4.
- **Axis 5 (plans-as-artifacts):** No (plans live in cycle issues + journal + `_notes/cycle-N-plan-N+1.md` files) — defer plan-as-artifact directories. v1 has not demonstrated need; the redesign's existing pattern of cycle-N→cycle-N+1 hand-off in `_notes/` is sufficient. Phase 3 prototype may surface need; revisit then.
- **Axis 6 (extension shape):** Tools (Rust binaries with per-tool manifest) — preserve `tools/rust/crates/` shape; add lightweight skill-style discovery via `tools/rust/crates/tool-registry/` enumerating tools with one-line descriptions for orchestrator discovery. No plugin / skill-with-prompt abstraction.
- **Axis 7 (orchestration topology):** Sequential mode transitions with deterministic transition policy — modes are { boot, work, close } per cycle, with explicit phase-boundary state machine in cycle-runner. The mode allowlist and transition policy are encoded in the harness, not the prompt. (oh-my-codex `STATE_MODEL.md` shape.)
- **Axis 8 (mechanical enforcement):** Behavioral promises + agent-affecting prose — CI on prompt-contract regression tests (`tools/rust/crates/prompt-contract-check`) ensuring named contracts in the prompt remain present and that no procedural step is added without a paired tool fix. Agent-readable error messages on quality-policy linters (cluster I sub-shape 2; v1=PARTIAL).
- **Axis 9 (iteration ceilings):** Loop count ceilings — per-tool `max_iterations` declared in tool manifests; cycle-runner enforces. Runtime ceiling is the existing cycle-level bound (~75 min); no per-component runtime budget.
- **Axis 10 (entropy mitigation):** Mandatory deslop pass post-completion — gardening sweep at session-close via `tools/rust/crates/gardening-sweep` over `_notes/`, `state/`, and `docs/redesign/`. Quality-grading rubrics deferred (cluster H sub-shape 3 = HIGH cost; v1=ABSENT, candidate adopts as MODERATE-cost minimal version: stale-detection + dead-link-detection only).
- **Axis 12 (reconciliation discipline):** Hybrid — polling for low-frequency Eva (input-from-eva pull at session-start; question-for-eva-response check at session-start), event-driven for cross-repo audit reads via Actions cron schedule (audit cycle posts; main reads on its next cycle). Dispatch-output reconciliation: per-cycle pull + per-PR-merge event.
- **Axis 13 (harness-vs-session boundary):** Medium harness — cycle-runner extracts named procedural patterns (boot phase: read state files + cursor advance + standing-directive check; close phase: gardening-sweep + cycle-history append + journal commit; dispatch-poll: enumerate stuck dispatches with cycle-count hand-off; audit-read: cross-repo cursor advance). Prompt handles novel + judgment + redesign-design-work. Cycle-runner change scope: SUBSTANTIAL.

## Cross-axis commitments

- **Axis 1 × Axis 7:** Single-threaded forces single-topology in the *session*; multi-pattern topologies live in the harness (cycle-runner orchestrates phase transitions; the session executes within one phase at a time). The orchestration multi-pattern shape is fully harness-side.
- **Axis 2 × Axis 3:** File-per-component pairs with wiki-as-files-in-repo. Both share the filesystem-based substrate. `_notes/*.md` are wiki entries; `state/<concern>.json` files are per-concern state. Discovery is unified via repo-walk + index regeneration on commit.
- **Axis 4 × Axis 2:** Git-as-substrate naturally supports file-per-component append semantics (each component file's history is the per-component append log). No branching checkpoints; the cycle-history append-only files serve the lookback role.
- **Axis 12 × Axis 4:** Polling reconciliation reads cursor files at session-start; event-driven reconciliation triggers via Actions on cross-repo commit. The reconciliation is a session-start phase in cycle-runner, not a separate role.
- **Axis 13 × Axis 6:** Medium harness organizes Rust tools via `tool-registry`; the tool-registry serves both as the discovery primitive (Axis 6) and as the harness's enumeration of available capabilities (Axis 13). One mechanism serves both axes.
- **Axis 13 × Axis 8:** Medium harness has prompt as primary mechanical-enforcement surface area (smaller than fat-harness's code-level enforcement surface). `prompt-contract-check` CI is the bridge: behavioral promises in the prompt are mechanically tested against drift.
- **Constraint 8 × Axis 1:** Goal-driven posture pairs naturally with single-threaded long-running execution; no goal-coordination primitive needed (the single orchestrator IS the goal-coordinator).

## Failure-mode addressing

- **F1 (constraint accretion):** Axis 8 (`prompt-contract-check` CI) + Axis 13 (medium harness extracts procedural constraints to tools) — F1 is structurally addressed by extracting ~50% of the current prompt's procedural content into cycle-runner phases.
- **F2 (Eva-response detection):** Axis 12 hybrid — input-from-eva pull at session-start with cursor advance; question-for-eva-response check via existing `check-eva-responses` (or replacement). Polling latency = 1 cycle (~6h at 4 cycles/day cron).
- **F3 (multi-candidate state drift):** Axis 2 file-per-component (single source of truth per concern) + Axis 12 reconciliation against post-close evidence (cycle-history files are append-only; in-cycle deltas are reconciled at boot phase).
- **F4 (frozen-artifact lifecycle fragility):** Axis 4 git-as-substrate determines what "frozen" means (committed = frozen-by-history). No worklog freeze problem; cycle-history files are append-only. Refresh timing is governed by cycle-runner boot phase.
- **F5 (state.json as procedural-leak):** Axis 2 file-per-component eliminates the monolithic state file. Per-concern files do not absorb procedural intent (each file's schema is single-purpose). `prompt-contract-check` (Axis 8) catches procedural-leak patterns.
- **F6 (cyclomatic procedure depth):** Axis 7 sequential mode transitions with explicit transition policy (3 modes vs v1's STARTUP→C-phases→COMPLETION nested checklist) + Axis 13 medium harness extracts procedure depth from prompt.
- **F7 (self-management dominance):** Axis 1 (no decomposition overhead) + Axis 8 (mechanical enforcement reduces orchestrator constraint-tracking burden) + Axis 9 (iteration ceilings prevent runaway-autonomy) + Axis 13 (medium harness extracts ~50% procedural surface). Aggregate effect: per-cycle decision count drops substantially vs v1.
- **F8 (abandonment cascades):** Axis 9 loop ceilings (prevention) + cluster A sub-shape 6 stuck-cycle-watchdog via Rust tool `detect-abandoned-cycles` (detection + recovery) + single-implementation discipline (the candidate has no parallel implementations). The watchdog is the openclaw-style stuck-session diagnostic.
- **F9 (adversarial-review treadmill):** Axis 7 multi-pattern topology in harness — review-firing is situational (cycle-runner triggers review only when phase-transition rules require), not every-cycle. Audit-as-peer (cross-repo) is the asynchronous-review primitive; in-cycle review is reduced.
- **F10 (audit's value is broader read scope):** Audit-as-peer pattern preserved (Axis 12 hybrid + cross-repo reading discipline). Audit's broader-read property is substrate-given (separate cron schedule + separate context window).
- **F11 (post-close mutations):** Axis 4 git-as-substrate prevents destructive write semantics (append-only cycle-history files) + Axis 12 reconciliation reads back state at boot phase (no frozen-worklog drift).
- **F12 (defense accretion catalog):** Axis 2 + Axis 4 + Axis 10 all contribute. The `gardening-sweep` (Axis 10) is the structural anti-accretion primitive.

## Preserved-primitives compliance

- **Journal:** preserved; remains long-term reflective channel. `docs/journal/YYYY-MM-DD.md` freeform format unchanged. Cycle-runner appends per-cycle subsection at session-start (boot phase).
- **Cycle-issue:** preserved; session-start and session-end comments via cycle-runner boot/close phases. No structured step-id taxonomy.
- **Question-for-eva / input-from-eva:** preserved; reconciliation via Axis 12 hybrid polling at boot phase. Cursor file `state/eva-input-cursor.json` advances on session-start.
- **Git-safety:** preserved; commit-must-be-pushed honored by Axis 4 (git-as-substrate) and existing `cycle-runner` push discipline. Per-component file writes commit + push in same operation.
- **Cycle-runner harness:** SUBSTANTIAL change scope. Boot phase, work phase, close phase added. Phase-boundary state machine encoded in cycle-runner. Specifics: `cycle-runner` invokes new sub-binaries `boot-phase`, `close-phase`, `phase-transition-check`. Each is a new Rust crate.

## What this candidate gives up

- **No dedicated reconciler agent** (Axis 12 reconciliation interleaves with primary work in boot phase) — if the volume of inbound channels grows substantially, the boot phase becomes the bottleneck.
- **No per-agent memory** (single session, single memory) — the wiki-search mechanism is shared across all cycle work; cannot specialize memory access by sub-task.
- **No parallel exploration branches** (Axis 4 git-as-substrate without branching checkpoints) — what-if reasoning across multi-cycle horizons is harder; the cycle-history append-only files give lookback, not lookahead.
- **No plans-as-artifacts directory** (Axis 5 = No) — plan tracking remains in cycle issues + `_notes/`. If multi-cycle plan coordination grows, this becomes friction.
- **Higher cluster A ABSENT count adopted** (cycle 89 cluster A: 6 ABSENT sub-shapes adopted partially — phase-boundary semantics + recovery operations + concurrency-queuing ABSENT sub-shapes are inherited at LOW-MODERATE cost only). Risk: if v1's cluster A failure modes return, the candidate has limited structural defense beyond cluster B intersection coverage.

## Tool surface implied

**New tools (net-add for cutover):**

- `tools/rust/crates/boot-phase` — boot-phase orchestration (state-load, cursor-advance, standing-directive check, gardening-sweep-pre-cycle)
- `tools/rust/crates/close-phase` — close-phase orchestration (gardening-sweep, cycle-history append, journal commit + push, cycle-issue close-comment)
- `tools/rust/crates/phase-transition-check` — phase boundary state machine enforcement
- `tools/rust/crates/wiki-search` — top-k retrieval over `_notes/*.md` description-frontmatter
- `tools/rust/crates/tool-registry` — Rust tool enumeration with one-line descriptions for orchestrator discovery
- `tools/rust/crates/prompt-contract-check` — CI test for prompt drift (named contracts present + no procedural-step-without-tool)
- `tools/rust/crates/detect-abandoned-cycles` — stuck-cycle-watchdog (cluster A sub-shape 6)
- `tools/rust/crates/gardening-sweep` — Axis 10 deslop pass (stale-detection + dead-link-detection)
- `tools/rust/crates/cycle-history-append` — append-only cycle-history file write

**Existing tools preserved (~unchanged):**

- `cycle-runner`, `write-entry`, `check-eva-responses` (or replacement-with-cursor), `pipeline-check`, audit-related tools

**Existing tools deprecated:**

- Anything that reads/writes `state.json` directly — superseded by per-component state files. Migration script: `tools/rust/crates/state-json-decompose` (one-shot Phase 4 cutover tool).

## Migration cost from v1

- **State migration:** decompose `state.json` into per-concern files. One-shot tool `state-json-decompose` runs at cutover. Risk: low (state.json schema is well-known per F12 catalog).
- **Tool migration:** ~9 new Rust crates (above). Per-crate scope ~200-500 LOC empirically validated at **2 of 9 instances** (cycle 93 `v2-tool-registry` 231 prod LOC; cycle 94 `v2-cycle-history-append` 268 prod LOC) — see [`Cycle 93+94 prototype scaffolding: migration-cost validation`](#cycle-9394-prototype-scaffolding-migration-cost-validation) for full empirical detail. Aggregate net-add **~2500-4500 production LOC** with smaller-end bias caveat (the 2 measured crates are structurally simpler; remaining 7 likely larger). **Aggregate including tests: ~7000-12000 LOC** at observed test:prod ratio of 1.5×-1.9× (cycle 95 added). **Dependency-footprint cost is one-time at the first crate**, not per-crate (cycle 94 measurement: subsequent crates reusing the {clap, serde, serde_json, tempfile} dep stack add zero transitive deps).
- **Prompt migration:** new prompt at `prompts/v2/orchestrator-prompt.xml`, ~50% smaller than v1 (procedural content extracted to harness). Eva-installable as drop-in replacement for `.github/workflows/orchestrator-prompt.xml`.
- **Workflow migration:** `cycle-runner` invokes new boot/close phases; `.github/workflows/orchestrator.yml` updated to invoke new prompt path. Workflow change is a forbidden-zone PR per redesign prompt SECTION 2 — Eva merges.
- **Journal/Cycle-issue migration:** zero (preserved).
- **Audit-repo coordination:** zero direct coordination required at cutover (audit reads main per cross-repo discipline; new prompt is just a different artifact for audit to read).

## P1-P6 evaluation criteria compliance (audit#454 absorption, cycle 89)

- **P1 (A↔B intersection coverage hard gate):** PASS. Axis 4 git-as-substrate + Axis 7 sequential phase boundaries + per-component file writes at named phase moments. Concrete mechanism: `phase-transition-check` enforces that any cluster B write target (per-component file) is committed at a named cluster A boundary (boot/work/close phase transition). Sub-pattern coverage: (1) sync-invariants-at-init via boot-phase invariant checks; (3) failure-record-write at retry-exhaustion via `detect-abandoned-cycles` + `cycle-history-append`; (4) watchdog-release-with-state-cleanup via `detect-abandoned-cycles` cleanup hooks; (5) component-local persistence loaded at init via boot-phase per-component state-load. Sub-pattern (2) state-commit at end-of-super-step is dual-cast per audit#454 D2; not separately required.
- **P2 (cluster I substrate-fit weighting):** PASS — substrate-driven adoption. v1's substrate (GitHub-Actions-anchored, public repo, multi-actor with audit) genuinely benefits from harness-enforced policy (cluster I sub-shape 1) and quality-policy linters (cluster I sub-shape 2). The candidate inherits cluster I patterns because the substrate naturally needs them, not aspirationally.
- **P3 (self-management-reduction axis):** PASS. Per-cycle decision count estimate: ~12 LOW + ~8 MODERATE + ~1 HIGH (gardening-sweep is MODERATE in this minimal version, not HIGH). Compared to v1 baseline (16 LOW + 27 MODERATE + 7 HIGH if v1 inherited all 50 sub-shapes — v1 actually inherits 6 STRONG only): candidate adopts ~21 sub-shapes vs v1's 6, but offsets with Axis 13 medium harness (extracts ~50% procedural surface from prompt). Net per-cycle decision overhead estimate: ~30-40% lower than current v1 prompt's apparent surface (the v1 prompt has high apparent surface due to F1 constraint accretion; the candidate's harness extraction is the primary reduction mechanism).
- **P4 (lifecycle-vocabulary completeness with intersection-coverage gate):** PARTIAL — bypasses cluster C gate. Candidate adopts cluster C sub-shape 5 (event-trigger / reactive-bot-comment-pickup) only at PARTIAL level (preserved primitive). Cluster C sub-shapes 1-4 (terminate, reset, fork, replay) are NOT adopted. Resume-only candidate per P4 bypass clause; criterion does not directly apply.
- **P5 (polarity-pivot exit criterion before cycle 90):** PASS at the authoring level (cycle 90 is the exit cycle; this candidate authored at the named exit cycle). Candidate-level not directly applicable.
- **P6 (audit-as-peer preservation slot):**
  - Criterion 1 (substrate isolation for audit role): PASS. Audit repo retained as separate process / cron / context window.
  - Criterion 2 (asynchronous-of-cycle communication discipline): PASS. Axis 12 hybrid honors cross-repo asynchronous communication.
  - Criterion 3 (cross-repo reading discipline): PASS. No cross-repo posting; both repos read each other.
  - Criterion 4 (discovery primitive completeness, audit#455 gap): PASS-WITH-WORK. Candidate adopts `[audit-request]` title prefix + `audit-request` label as the discovery primitive for v2. Cycle-runner boot phase reads main's open issues with this label/title at audit-side cycle start; main's audit-side cursor advances on each audit-side reply.
  - Criterion 5 (audit-as-peer evolvability): PASS. Audit autonomy on its own protocol preserved (audit edits its own STARTUP_CHECKLIST without main approval).
  - Aggregate: 5/5 PASS (one is PASS-WITH-WORK requiring the discovery primitive to be implemented).

## M3 v1 strengths preservation (audit#454 absorption, cycle 89)

- **Strength 1 (process-isolation via ephemeral worktrees):** PRESERVED. Substrate-inherited; GitHub Actions ephemeral runner property unchanged.
- **Strength 2 (anti-patterns as published artifact):** PRESERVED. Axis 5 = No defers plan-as-artifact, but the retrospective + journal + `_notes/` continue serving as the anti-pattern catalog. v2 prompt explicitly preserves "what we will not do" sections.
- **Strength 3 (walkback as first-class artifact):** PRESERVED. Axis 4 git-as-substrate + append-only cycle-history files preserve walkback semantics by construction.
- **Strength 4 (clean-context audit-as-peer reviewer at session level):** PRESERVED. Mixed substrate / convention inheritance maintained: substrate (separate repo + cron) preserved by Axis 12 hybrid; convention (cross-repo communication discipline) preserved by candidate's Axis 12 explicit discipline declaration.
- **Strength 5 (multi-layer permission-policy enforcement at harness level):** PRESERVED. GitHub Actions + branch protection + claude-code permission system all unchanged. Axis 8 mechanical enforcement adds prompt-contract-check as a complementary layer.

## M2 self-management cost inheritance (audit#454 absorption, cycle 89)

Aggregate per-cycle decision overhead estimate (counting LOW + MODERATE + HIGH cost sub-shapes the candidate adopts):

- **LOW count:** 12 — cluster A sub-shapes 1 + 2 + 9 (super-step semantics PARTIAL via phase-boundary state machine; per-key reducers PARTIAL via per-component file writes; process-isolation STRONG inherited); cluster B sub-shapes 2 + 5 + 8 (component-local persistence PARTIAL; repository-as-state STRONG; active-surface vs monotonic-history STRONG via cycle-history); cluster D sub-shapes 1 + 2 (anti-patterns + walkback STRONG); cluster G sub-shape 1 (audit-as-peer STRONG); cluster I sub-shape 1 (permission-policy STRONG); plus 2 LOW from new-mechanism integrations (boot/close phase coordination at LOW cost given deterministic state machine).
- **MODERATE count:** 8 — cluster A sub-shapes 4 + 5 (typed-channel-merger PARTIAL via per-component file writes — boundary, MODERATE; bounded-retry-with-feedback PARTIAL via loop ceilings); cluster B sub-shapes 4 + 6 + 9 (sweep-rollup PARTIAL; top-k retrieval ABSENT→PARTIAL via wiki-search; failure-record PARTIAL via detect-abandoned-cycles); cluster C sub-shape 5 (event-trigger PARTIAL); cluster I sub-shape 2 (quality-lint orchestrator-friendly PARTIAL); cluster H sub-shape 1 (tight-cycle meta-feedback PARTIAL via journal).
- **HIGH count:** 1 — cluster H sub-shape 3 (continuous-background gardening at MODERATE-HIGH; minimal version is gardening-sweep with stale-detection only, not full quality-grading rubrics — boundary case).

**Compared to v1 baseline:** v1 inherits 6 STRONG sub-shapes (process-isolation + repository-as-state + active-surface + anti-patterns + walkback + audit-as-peer-substrate); the candidate inherits 6 STRONG + ~15 PARTIAL adopted sub-shapes. The aggregate per-cycle decision overhead is **moderately higher than v1's actual M2 inheritance** (v1's 6 STRONG only) but **substantially lower than the maximum-adoption baseline** (50 sub-shapes). The candidate's enable-schema-work answer: **YES, primarily via Axis 13 medium harness extracting ~50% of the current v1 prompt's procedural surface**, which is a structural reduction independent of M2 sub-shape adoption.

## Honest reflection

This candidate is the **conservative path**. It commits to the bet that v1's substrate is good and the load-bearing cleanup is structural (state.json, _notes-as-memory, harness extraction) rather than architectural (multi-role decomposition). If that bet is wrong — if the cluster A 6-ABSENT count or the cluster H continuous-background mechanisms are load-bearing for v2 success — this candidate inherits v1's failure surface in those dimensions.

It is also the candidate with the **lowest migration cost and highest cutover predictability**. Phase 3 prototype effort is bounded (~9 new Rust crates, well-defined). Rollback is straightforward (delete v2 artifacts; revert workflow YAML).

Cycle 89 hand-off named this candidate as one of two for cycle 90 authoring. Candidate B (multi-role decomposition) is the **aggressive path** — see [`B-decomposed-multi-role.md`](./B-decomposed-multi-role.md). Cycle 91 added Candidate C (hybrid) — see [`C-hybrid.md`](./C-hybrid.md). Eva and the audit-repo orchestrator review all three before candidate-selection checkpoint.

## Cycle 91 sharpening: Axis 13 medium-harness extraction details

The candidate's P3 PASS claim depends on the Axis 13 medium harness extracting ~50% of the current v1 prompt's procedural surface to cycle-runner phases. Cycle 90 hand-off named this estimate as a candidate weak point for cycle 91+ sharpening; this section grounds the estimate in concrete extraction-vs-retention content. The estimate is at the **structural level** (named procedural patterns extracted) not the **byte level** (literal prompt-text reduction); the byte-level reduction is a downstream consequence and varies with how concisely the orchestrator can describe the extracted contracts in the v2 prompt.

### What the medium harness extracts (boot/work/close phases + dispatch-poll + audit-read)

**Boot-phase responsibilities** — currently in v1 prompt's STARTUP_CHECKLIST.xml:

- Read `state.json` and decompose into per-component file reads (`state/dispatch-queue.json`, `state/eva-input-cursor.json`, `state/redesign/<phase>.json`, `state/cycle-history/<N>.json`)
- Advance `state/eva-input-cursor.json` by pulling `input-from-eva` issues authored by Eva since last-cursor; surface new directives
- Pull `question-for-eva` response check (delegated to `check-eva-responses` or replacement)
- Read recent journal entries (configurable lookback window)
- Read recent commits (configurable lookback window)
- Read audit-repo cycle posts since last-audit-cursor
- Standing-directive check (active `input-from-eva` directives)
- Pre-cycle gardening sweep (stale `_notes/` detection, dead-link detection)
- Post session-start comment on cycle-issue with templated content (model, run id, mode, cycle-composition-shape declaration, observations going in, standing-tasks summary)

**Close-phase responsibilities** — currently in v1 prompt's COMPLETION_CHECKLIST.xml:

- Gardening sweep (deslop pass over `_notes/`, `state/`, `docs/redesign/`)
- Cycle-history append (append-only `state/cycle-history/<N>.json`)
- Journal commit + push (single operation per git-safety preserved primitive)
- Cycle-issue close-comment (templated session-end summary with bottleneck-state honesty + sibling-pattern tracking + cycle-(N+1) plan)
- Cycle-issue close
- Standing-task tracking: open issues count, open PR count, standing input-from-eva directives count

**Dispatch-poll responsibilities** — currently in v1 prompt's various sections:

- Enumerate stuck dispatches (Copilot agent assignments not yet started)
- Cycle-count hand-off ("X+ cycles awaiting Eva's manual Copilot assignment")
- Per-dispatch state advance when output arrives (PR-merge events, comment events)

**Audit-read responsibilities** — currently in v1 prompt's cross-repo discipline section:

- Cross-repo cursor advance (audit cycles read since last-cursor)
- Per-cycle audit-side artifact read (audit's `_notes/`, audit's `docs/redesign/`)
- Audit critique surface (issues with `[redesign-feedback]` title prefix or audit-cycle-post pattern)

**Extracted (estimated XML element count from v1 prompt):**

- STARTUP_CHECKLIST.xml currently has ~180 named steps across S0-S15 step IDs (each step is a named procedural pattern extractable to a deterministic phase)
- COMPLETION_CHECKLIST.xml currently has ~80 named steps across C0-C15 step IDs
- Aggregate: ~260 named procedural steps in v1 STARTUP + COMPLETION checklists
- Of these, ~130-160 are deterministic (read-this-file, write-this-template, advance-this-cursor, enumerate-this-list) and structurally extractable to cycle-runner phases without orchestrator judgment
- Remaining ~100-130 require orchestrator judgment (substantive-focal selection, situational decisions, bottleneck-state assessment, sibling-pattern recognition) and stay in the v2 prompt

### What the medium harness retains (orchestrator-judgment surface)

- **Substantive-focal selection** — what the cycle's primary substantive activity is (deepening cluster annotations, sharpening candidates, integrating audit critique, dispatching research, etc.). Not deterministic; depends on cycle-N hand-off + standing tasks + bottleneck state + audit critique landing.
- **Situational decisions** — when to dispatch Copilot research vs answer in-cycle; when to file a question-for-eva vs resolve autonomously; when to ask audit for critique vs proceed.
- **Sibling-pattern recognition** — which functional-class shapes a cycle is an instance of; whether a methodology pattern advances to TESTED / HARDENED.
- **Honest reflection** — F1-F5 correctives, anti-inheritance corrective, generalization-level discipline. Requires per-cycle judgment about what was actually substantive vs what looked substantive.
- **Cross-repo-communication framing** — when audit-as-peer requests critique; when to surface a structural gap as a `[redesign-feedback]` request; how to frame the request for audit's reading discipline.
- **Cycle-composition-shape declaration** — naming what shape the cycle is, whether it's an instance of a prior shape, whether a new shape is emerging. Depends on the cycle's actual content, not a deterministic check.
- **Bottleneck-state honesty** — is the bottleneck external (Eva's manual Copilot assignment) or internal (orchestrator's choice)? Naming this honestly per cycle requires judgment.
- **Iteration-until-approval discipline** — when an artifact is "stable" vs "needs more iteration"; what specific weak points to sharpen each cycle.

### Estimated v2 prompt size

- v1 prompt (current orchestrator-prompt.xml + STARTUP_CHECKLIST.xml + COMPLETION_CHECKLIST.xml): aggregate ~2400 lines (prompt ~1100 lines + STARTUP ~800 lines + COMPLETION ~500 lines).
- v2 prompt (this candidate's prompt only; no external checklists): estimated ~600-800 lines.
  - Purpose / mission / authority sections: ~150 lines (relatively unchanged from v1's mission section)
  - Preserved-primitives + security: ~120 lines (mostly unchanged)
  - Mode-transition contracts (boot / work / close): ~100 lines (instructing orchestrator on per-mode invocation, not procedural steps)
  - Tool registry reference: ~50 lines (one-line description per tool with invocation contract)
  - Judgment-call surface (substantive-focal selection, situational decisions, sibling-pattern recognition, honest reflection, cross-repo-communication, cycle-composition-shape declaration): ~150 lines
  - Iteration-until-approval discipline + abort criteria + checkpoints: ~80 lines
- **Aggregate v2 prompt**: ~600-800 lines, vs v1's ~2400 lines = ~67-75% reduction at the byte level.
- **Procedural-surface reduction at the structural level**: ~130-160 of ~260 named steps extracted = ~50-62% of named-step extraction.

### Validation plan (cycle 92+ Phase 3 prototype work)

The estimates above are derived from v1 prompt structure analysis, not yet validated by Phase 3 prototype. Cycle 92+ Phase 3 prototype effort should:

1. Author a draft v2 prompt at `prompts/v2/orchestrator-prompt.xml` covering the categories above. Measure actual line count.
2. Author the `boot-mode` Rust crate as the smallest scaffold of the medium-harness extraction. Measure how much procedural content fits into deterministic Rust code vs what spills back into the prompt.
3. Compare actual extraction percentage against the ~50% structural estimate.
4. If extraction percentage falls below 40%, the P3 PASS claim weakens; investigate whether more procedural content can be extracted (potentially shifting toward the fat-harness Axis 13 position) or whether the orchestrator-judgment surface is genuinely larger than estimated.

If the validation reveals that ~50% extraction is achievable, P3 PASS is grounded. If validation reveals < 40% extraction is achievable while preserving orchestrator-judgment surface, the candidate's P3 PASS claim is weaker than asserted and should be revised to PASS-WITH-NOTE or PARTIAL.

### Risks named at the structural level (not yet validated)

- **Risk 1:** the named-step taxonomy in v1 STARTUP/COMPLETION is incomplete (some procedural patterns are not explicitly numbered S/C step IDs but are named in prose). The ~260 step count is a lower bound; the actual procedural surface may be larger.
- **Risk 2:** the boundary between deterministic-extractable and judgment-required is fuzzy for some steps (e.g., "post session-start comment with templated content" is deterministic in template but judgment-required in what to fill in). The ~50% extraction estimate assumes deterministic-extractable cases dominate; if judgment-required cases dominate, extraction is lower.
- **Risk 3:** the v2 prompt's tool-registry reference may grow substantially as the tool count grows (cluster B's per-component-state files alone require ~5 dedicated tool descriptions). Aggregate v2 prompt size is sensitive to tool count.
- **Risk 4:** maintaining the prompt-contract-check CI invariant (`prompt-contract-check` ensures named contracts in the prompt remain present and that no procedural step is added without a paired tool fix) requires ongoing attention; if Phase 3+ prototype extends the prompt to address novel situations, the procedural-surface ratio may regress.

These risks are bounded — none threaten the candidate's substrate-bet directly. They threaten the *magnitude* of the P3 reduction (50% vs 40% vs 30%) but not the *direction* (medium-harness extraction reduces procedural surface). P3 PASS direction is validated; magnitude is sharpened to ~40-50% rather than ~50%, with cycle 92+ prototype required for empirical validation.

## Cycle 93+94 prototype scaffolding: migration-cost validation

The Migration cost section's tool-migration claims (~9 crates, ~200-500 LOC each, ~3000-4500 aggregate net-add) were named at cycle 90 authoring without empirical grounding. Cycle 93 began Phase 3 prototype scaffolding by building `v2-tool-registry`; cycle 94 extended to `v2-cycle-history-append`. This section consolidates the 2-instance empirical findings as cycle 95 propagation. Per `direction-vs-magnitude` discipline (cycle 91 lexicon entry, HARDENED at 5+ instances): direction-supporting at 2 of 9 instances; magnitude-validating awaits remaining 7 crates.

### Per-crate measurements (cycles 93-94)

| Metric | Cycle 93 `v2-tool-registry` | Cycle 94 `v2-cycle-history-append` |
|---|---|---|
| Production LOC | ~231 | ~268 |
| Test LOC (inline + integration) | ~347 | ~510 |
| Total LOC | ~578 | ~804 |
| Test count | 15 | 31 |
| Test:Prod ratio | ~1.5× | ~1.9× |
| Build time (release, dep-cache warm) | ~1.24s | ~1.01s |
| New transitive deps added to workspace lockfile | ~30 (`{clap, serde, serde_json, tempfile}` first establishment) | 0 |

### Validation findings against cycle 90 authoring claims

- **Per-crate scope (~200-500 LOC) — DIRECTION-VALIDATED at 2 instances.** Both crates fall within the stated range. Both are at the smaller end of the tool surface; the 2-of-9 evidence is direction-supporting on per-crate boundedness. Magnitude-validation requires measurement of structurally-larger crates (`wiki-search`, `boot-phase`, `close-phase`).
- **Aggregate production-LOC claim (~3000-4500) — DIRECTION-SUPPORTING; magnitude trajectory may shift below the lower bound.** Trajectory at ~250 LOC/crate × 9 crates = ~2250 production LOC. Caveat: remaining 7 crates likely larger; trajectory may rise. Updated estimate: ~2500-4500 production LOC.
- **Aggregate-with-tests claim — INTRODUCED CYCLE 95.** Implicit in cycle 90 authoring (which named production-LOC only). Test:Prod ratio averaged 1.7× across 2 instances (range 1.5×-1.9×). Aggregate-with-tests trajectory ~7000-12000 LOC (production + tests). This is a substantial revision to the deliverable's apparent scope — the test code is part of the deliverable per `ARTIFACT-COMPOSITION` and should not be hidden.
- **Dependency-footprint risk — REFUTED at second-crate level.** Cycle 93's risk #3 (Cargo.lock churn growing per crate) was framed as per-crate accretion. Cycle 94's measurement: subsequent crates reusing the established dep stack add zero transitive deps. **Implication:** workspace dependency footprint is bounded; argues for `[workspace.dependencies]` convention at cutover (declare common deps once, members reference with `{ workspace = true }`).
- **Build-time aggregate — BOUNDED.** Both crates build in ~1.0-1.2s release mode after dep cache is warm. Aggregate build time for full 9-crate set plausibly bounded at ~10s release (workspace property: shared deps mean only crate-local code recompiles).

### Structural insights surfaced cycles 93-94

1. **Refuse-overwrite as structural append-only enforcement primitive.** v2 write tools implement filesystem-level refuse-overwrite (cycle-history-append being the first; tool-registry is read-only so question didn't arise). This makes Axis 4 (git-as-substrate) more robust at the tool level: the tool itself enforces the append-only invariant; an orchestrator bug or rogue manual edit cannot accidentally overwrite cycle-history. v1 relied on prompt-level convention (procedural step the orchestrator was supposed to follow); v2 promotes to tool-level enforcement. Pattern other v2 write tools should follow.
2. **Pass-through schema as forward-compatibility default.** `cycle-history-append` enforces 3 required fields and lets the rest pass through verbatim. New cycle-history schema fields can be added at the orchestrator level without tool change. Deliberate looseness — schema strictness is a future migration if downstream consumers (e.g., `detect-abandoned-cycles`, trend-analysis tools) demand it. For prototype, the looseness is correct.
3. **CLI-field-overrides-JSON precedence.** `--from-json file.json --field model=overrideX` resolves to CLI override. Useful for callers with mostly-fixed JSON payloads + per-invocation overrides (e.g., wrapper injecting `started_at`). Integration tests document this precedence explicitly.
4. **Workspace dependency-footprint bounded after first crate.** If A's remaining 7 crates converge on the {clap, serde, serde_json, tempfile} dep stack, workspace dependency footprint is bounded at the first-crate establishment. Cutover implication: `[workspace.dependencies]` convention naturally indicated.
5. **Atomic write via tmp + rename.** Both write tools use `target.with_extension("...tmp") + fs::rename` for atomic write. Same-filesystem assumption documented (cross-mount support requires `tempfile::NamedTempFile::persist` upgrade if needed).

### Risks named for the cycle 93+94 evidence base

1. **Smaller-end bias.** The 2 measured crates are the structurally-simplest (read-only catalog access; append-only single-record JSON write). The remaining 7 (`boot-phase`, `close-phase`, `phase-transition-check`, `wiki-search`, `prompt-contract-check`, `detect-abandoned-cycles`, `gardening-sweep`) are likely larger. Trajectory may shift upward beyond ~2500-4500 production LOC.
2. **Test-code amplification widening.** Test:Prod ratio went 1.5× → 1.9× from cycle 93 to cycle 94, partly because cycle-history-append has a larger CLI surface (more options, validation paths, error cases). If subsequent crates have similar surface area expansion, aggregate-with-tests may approach the upper end of ~12000 LOC.
3. **2-of-9 instances is direction-supporting only.** Aggregate-LOC magnitude validation requires more instances. Cycle 96+ should continue prototype scaffolding, prioritizing structurally-larger crates (wiki-search likely largest) to test aggregate-LOC bound.
4. **Workspace-dependency convention not yet decided.** The `[workspace.dependencies]` convention is naturally indicated but not yet adopted in the v2 workspace `Cargo.toml`. Cutover plan should specify this convention explicitly.
5. **`--field` value auto-typing ambiguity inherited from cycle 94.** Values like `true` parse as boolean; `123` as integer. Edge case: a string field intended to hold "true" silently becomes a boolean. Documented per cycle 94 risk #2; defer to `--field-string KEY=VALUE` if recurring.
6. **Non-RFC3339 `started_at` strings accepted.** `cycle-history-append` accepts any non-empty string; malformed timestamp surfaces only at downstream parse. Documented per cycle 94 risk #3; defer until consumer pressure forces stricter validation.

### Sibling-pattern entries surfaced cycles 93-94

- **Prototype-scaffold migration-cost-validation discipline** — TESTED at 2 instances (cycle 93 + cycle 94). Pattern: each prototype crate produces direction-vs-magnitude data; aggregation across N instances grades direction-validated vs magnitude-validated.
- **Direction-vs-magnitude discipline** — extends to 5+ instances (cycle 91 A's Axis 13 + cycle 92 B's per-role decision count + cycle 92 C's central bet + cycle 93 prototype scaffold migration-cost + cycle 94 prototype scaffold migration-cost #2). HARDENED.
- **Refuse-overwrite primitive** — NOVEL at 1 instance (cycle 94). Other v2 write tools should follow the same pattern.
- **Pass-through schema forward-compatibility** — NOVEL at 1 instance (cycle 94).

### Cycle 96+ measurement plan

Continue prototype scaffolding to broaden evidence base from 2 to 3+ instances:

1. **`phase-transition-check`** (state machine validation) — structurally distinct from file-write/file-enumerate tools (validation vs mutation). Estimated ~300-400 LOC. Tests aggregate-LOC magnitude under different surface shape.
2. **`prompt-contract-check`** (CI test for prompt drift) — likely larger (~400-500 LOC). Tests upper end of stated per-crate range.
3. **`wiki-search`** (top-k retrieval) — likely largest in surface area; tests upper bound of per-crate scope claim.

Sequencing prioritizes structurally-distinct tool types over similar-shape repetition. After 3-4 measurements across distinct tool shapes, aggregate-LOC trajectory becomes magnitude-validating rather than direction-supporting.
