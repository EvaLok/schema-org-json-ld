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
- **Tool migration:** ~9 new Rust crates (above). Per-crate scope ~200-500 LOC empirically validated at **2 of 9 instances** (cycle 93 `v2-tool-registry` 231 prod LOC; cycle 94 `v2-cycle-history-append` 268 prod LOC) — see [`Cycle 93+94 prototype scaffolding: migration-cost validation`](#cycle-9394-prototype-scaffolding-migration-cost-validation) for full empirical detail. Aggregate net-add **~2500-4500 production LOC** with smaller-end bias caveat (the 2 measured crates are structurally simpler; remaining 7 likely larger). **Aggregate including tests: ~7000-12000 LOC** at observed test:prod ratio of 1.5×-1.9× (cycle 95 added). **Dependency-footprint cost is one-time at the first crate**, not per-crate (cycle 94 measurement: subsequent crates reusing the {clap, serde, serde_json, tempfile} dep stack add zero transitive deps). **Cycle 97 absorption (PR #2877 lens-4 revised range):** PR #2877's tool-surface feasibility critique revised A's aggregate net-add from 3000-4500 to **3600-6200 production LOC** based on integration/test/CI/shared-type overhead under-counting in 2-3 specific crates (`wiki-search`, `boot-phase`, `close-phase`). The 2-of-9 measured crates fall within the smaller-end of A's stated range as predicted; the harder crates (orchestration hubs, external-IO retrieval) may exceed 500 LOC each. Direction (per-crate boundedness) holds at 2 instances; magnitude awaits structurally-distinct crate measurement. PR #2877's verified workspace calibration (38 v1 crates, median 1081 LOC, mean 2118 LOC) anchors the comparison: A claims v2 crates ~5× simpler than v1 median; the bet is defensible under CORE-DESIGN-PRINCIPLE (v2 crates by-design narrower than v1's accreted multi-purpose tools), but specific crates with orchestration-hub or external-IO semantics carry the highest risk.
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

**Extracted (XML element count from v1 prompt — VERIFIED cycle 96 per PR #2878 absorption):**

- STARTUP_CHECKLIST.xml: **35** named `<step>`/`<substep>` IDs across S0-S15 (verified count, not estimate)
- COMPLETION_CHECKLIST.xml: **15** named `<step>`/`<substep>` IDs across C0-C15
- orchestrator-prompt.xml: 0 `<step>`/`<substep>` element IDs (prose+tag structure, not numbered-step structure)
- **Aggregate: 50 named step/substep IDs** in v1 STARTUP + COMPLETION checklists. The cycle 91 authoring claim of "~260" was a 5.2× overcount, fabricated to support the ~50% extraction percentage rather than counted from the files. (See cycle 96 absorption note `_notes/cycle-96-feedback-absorption.md` Finding 1.)
- **Per cycle 91 _notes** (`_notes/cycle-91-candidate-C-and-A-sharpening.md`), the extractable-pattern count from the original sharpening exercise was "~15 from STARTUP + ~10 from COMPLETION + ~5 dispatch-poll + ~5 audit-read" = **~35 extractable patterns**, distinct unit-of-analysis from step IDs (patterns aggregate cross-step responsibilities). Against the verified 50-step denominator, the extraction-percentage calculation is **indeterminate without first agreeing on the unit of analysis** (step IDs vs patterns). The original "~130-160 extractable" figure does not survive verification.
- The judgment-vs-deterministic split below describes the **judgment-required surface** that medium-harness extraction would NOT eliminate, independent of any extraction-percentage claim.

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

- v1 prompt (current orchestrator-prompt.xml + STARTUP_CHECKLIST.xml + COMPLETION_CHECKLIST.xml): aggregate **1,289 lines** (prompt **559** + STARTUP **298** + COMPLETION **432**) — VERIFIED cycle 96 against the actual files. The cycle 91 authoring claim of "~2400 lines (prompt ~1100 + STARTUP ~800 + COMPLETION ~500)" was 1.86× the actual size; component overestimates were 2× / 2.7× / 1.16× respectively. (See cycle 96 absorption note Finding 2.)
- v2 prompt (this candidate's prompt only; no external checklists): estimated ~600-800 lines.
  - Purpose / mission / authority sections: ~150 lines (relatively unchanged from v1's mission section)
  - Preserved-primitives + security: ~120 lines (mostly unchanged)
  - Mode-transition contracts (boot / work / close): ~100 lines (instructing orchestrator on per-mode invocation, not procedural steps)
  - Tool registry reference: ~50 lines (one-line description per tool with invocation contract)
  - Judgment-call surface (substantive-focal selection, situational decisions, sibling-pattern recognition, honest reflection, cross-repo-communication, cycle-composition-shape declaration): ~150 lines
  - Iteration-until-approval discipline + abort criteria + checkpoints: ~80 lines
- **Aggregate v2 prompt**: ~600-800 lines, vs v1's verified **1,289** lines = **~38-53% byte-level reduction**. The cycle 91 authoring claim of "~67-75% reduction" rested entirely on the inflated 2400-line denominator and does not survive verification. The direction (v2 prompt is shorter than v1 total surface) holds; the magnitude is roughly half what was originally claimed.
- **Procedural-surface reduction at the structural level**: **indeterminate** until extraction-count and step-count units of analysis are reconciled (see `Extracted` block above). The cycle 91 authoring "~50-62%" figure used a fabricated 260-step denominator and an unmeasured 130-160 extractable count; both inputs require regrounding before any percentage claim.

### Validation plan (cycle 92+ Phase 3 prototype work)

The estimates above are derived from v1 prompt structure analysis, not yet validated by Phase 3 prototype. Cycle 92+ Phase 3 prototype effort should:

1. Author a draft v2 prompt at `prompts/v2/orchestrator-prompt.xml` covering the categories above. Measure actual line count.
2. Author the `boot-mode` Rust crate as the smallest scaffold of the medium-harness extraction. Measure how much procedural content fits into deterministic Rust code vs what spills back into the prompt.
3. **Decide and document the unit of analysis** (step IDs vs named patterns vs prose-procedural clauses) BEFORE measuring extraction percentage. Per cycle 96 absorption Risk 0, the choice of unit is a candidate claim itself, not a measurement.
4. Measure extraction in the chosen unit against the verified v1 baseline (50 step IDs, 1,289 prompt+checklist lines).
5. **Hard refutation threshold (added cycle 96 per PR #2878 Finding 18 — external falsification criterion):** if the chosen unit measures < **30%** extraction with judgment-surface preservation intact, P3 is **reclassified to PARTIAL-FLAG regardless of direction-validation status**. This is a pre-agreed threshold that does not depend on the orchestrator's own assessment. Between 30% and 50%, P3 is PASS-WITH-NOTE. At ≥ 50%, P3 is full PASS. Specifying this threshold in advance is the discipline absent from the cycle 91 sharpening's soft "weakens" language.
6. **Per-extension-event logging (added cycle 109 per Risk 4 closure):** instrument each prompt-extension event from Phase 3 cycle 1 with the JSONC schema in Choice 4. Type 1/2/4/5 require paired-tool field populated; Type 3 requires explicit `paired-tool: null` + decision-rationale. Per-cycle log produces direct evidence of extension-discipline compliance.
7. **Procedural-surface ratio P measurement (added cycle 109):** compute P = (procedural steps in prompt) / (total prompt step + named-tool count) per-cycle starting Phase 3 cycle 1. Pre-agreed thresholds P ≤ 0.3 / (0.3, 0.5] / >0.5. The unit-of-analysis decision (step 3) determines what counts as a "step" in P's numerator and denominator; P is rubric-symmetric within the chosen unit.
8. **Per-type compliance rate measurement (added cycle 109):** measure per-type compliance rates from Phase 3 cycle 5+ steady-state. Type 1: 100% (any unpaired Type 1 = contract violation, raised at CI). Type 2: ≥95% direction-validated / 80-95% at-risk / <80% refuted. Type 3: ≤10% later-reclassification rate. Type 4: ≥90% documented + ≥80% re-extracted within window. Type 5: ≥80% reclassified-or-paired within ≤5 cycles.
9. **Drift-window D as secondary derived metric (added cycle 109):** measure drift-window D for Type 2/4/5 extensions. D ≤ 3 (Type 2) / D ≤ 5 (Type 4) / D > 5 = aged extension, reclassify or accept ratio regression. D is informational, not refutational; aggregate refutation reads from P (Choice 2) regardless of per-extension D.
10. **Extension-classification rubric stability (added cycle 109):** rubric tightening expected cycles 1-5 (warm-up window per cycle 107/108 shape #24). Past cycle 5, inter-rater disagreement ≤10% steady-state. Combined-readings diagnostic (Choice 6) preserves Risk-1 (unit-of-analysis) vs Risk-4 (extension-discipline) distinction at refutation level.
11. **CI sweep extension (added cycle 109):** extend `prompt-contract-check` to `prompt-extension-pairing-check` — verify Type 1/2 extensions have paired tools at commit time and Type 4/5 follow-up windows are tracked. Missing paired-tool for Type 1/2 fails CI; Type 4/5 follow-up window expiration raises advisory.

If validation reveals ≥ 50% extraction is achievable in the chosen unit, P3 PASS is grounded. Between 30-50%, P3 is PASS-WITH-NOTE. Below 30%, P3 fails outright and the candidate's substrate-bet (medium-harness adequately extracts procedural surface) is refuted; design reverts to either fat-harness Axis 13 or larger v2 prompt.

### Risks named at the structural level (not yet validated)

- **Risk 0 (added cycle 96 per PR #2878 absorption — baseline-measurement risk):** prior to cycle 96, the quantitative baselines (step count, line count) used in this section were unverified estimates that did not survive checking against the actual XML files. Any subsequent percentage-extraction claim depends on (a) a verified count of v1's procedural surface in a chosen unit of analysis (step IDs, patterns, prose-procedural-clauses, or some hybrid) AND (b) a measured count of what extraction actually achieves at prototype scaffolding. Both inputs are now flagged as requiring direct measurement before any further percentage claim. The cycle 96 verification re-grounds the line-count baseline at 1,289 (down from the claimed 2,400) and the step-ID denominator at 50 (down from the claimed 260); the extractable-pattern numerator from cycle 91's _notes was ~35, distinct unit from step IDs. Re-grounding the percentage claim requires reconciling these units.
- **Risk 1:** the named-step taxonomy in v1 STARTUP/COMPLETION is one unit of analysis (50 step+substep IDs measured); other units (named patterns spanning multiple steps; prose-procedural clauses) may give different denominators. The choice of denominator is itself a candidate-claim decision, not a measurement.
- **Risk 2:** the boundary between deterministic-extractable and judgment-required is fuzzy for some steps (e.g., "post session-start comment with templated content" is deterministic in template but judgment-required in what to fill in). Any extraction-percentage estimate assumes deterministic-extractable cases dominate; if judgment-required cases dominate, extraction is lower regardless of denominator chosen.
- **Risk 3:** the v2 prompt's tool-registry reference may grow substantially as the tool count grows (cluster B's per-component-state files alone require ~5 dedicated tool descriptions). Aggregate v2 prompt size is sensitive to tool count. **Cycle 97 sharpening (PR #2877 lens-7):** coordination grows super-linearly past ~15-25 callable units; PR #2877 names ~18-25 as a candidate-independent threshold beyond which one-line registry descriptions are insufficient for safe invocation selection. v2 callable surface (~9 new crates plus retained v1 callable subset) approaches this threshold; cumulative v1 + v2 surface (47 crates total) likely exceeds it. Mitigation: hierarchical registry + compatibility metadata if surface grows beyond ~15 actively-orchestrator-called tools.

  **Cycle 105 second-iteration sharpening (post-cycle-102/104 substrate absorption):** three new candidate-distinguishing tool-adoption decisions surface from cycle 102 cluster A sub-shapes 10+11 (task-ingestion routing) and cluster H sub-shapes 5+6 (feedback inference / evaluator-driven keep-discard), with cycle 104 A↔B-6 making the cluster B coordinated-write target explicit. The substrate-absorption frame consolidates these as concrete tool-count deltas under three adoption choices A could make.

  | Adoption decision | Tool delta if adopted | Self-management cost |
  |---|---|---|
  | Cluster A sub-shape 10 (classifier-mediated dispatch, PAI) | +1 (`classify-cycle-task`) | MODERATE — classifier prompt + taxonomy maintenance |
  | Cluster A sub-shape 11 (deterministic-decision-tree routing, omx) | +1 (`route-cycle-decision`) | LOW — typed-set-membership rules execute without per-cycle calibration |
  | A↔B-6 cluster B coordinated write (cycle 104) | +0 to +2 — classification-record + route-record file targets absorbed by per-component-state writer if shared, else dedicated tools | LOW — append-only writes at session-entry boundary |
  | Cluster H sub-shape 5 (feedback-signal-inference, PAI) | +1 (`infer-feedback-signal`) | MODERATE — inference-prompt evolution as feedback-shapes emerge |
  | Cluster H sub-shape 6 (evaluator-driven keep-discard, omx) | +1 (`evaluator-driven-consolidate`) | MODERATE — evaluator-rubric maintenance |

  **Trajectory bounds:**
  - Cycle 90 baseline: ~9 v2 crates standalone
  - Adoption-floor (none of the new sub-shapes): ~9 v2 crates
  - Classify-then-route pipeline only (10 + 11 + minimal A↔B-6 absorption into existing per-component-state writer): ~11 v2 crates
  - Full adoption (10 + 11 + dedicated A↔B-6 writers + 5 + 6): ~14-15 v2 crates standalone

  **Cross-reference to PR #2877 lens-7 thresholds:**
  - ~15 callable units: registry one-liners begin to fail (lower edge of super-linear band)
  - ~18-25: super-linear coordination across registry surface
  - At full-adoption upper bound (~15 v2 crates), A's standalone surface is at the lower edge of the registry-one-liner failure threshold; combined v1+v2 surface (cycle 97 baseline 47 + ~5 added) reaches ~52, comfortably above super-linear threshold

  **Active-callable vs total-inventory distinction (cycle 105 addition):** the orchestrator-CALLABLE surface and the v2 crate INVENTORY are structurally distinct quantities under A's Axis 13 medium-harness extraction. The 15-tool registry-one-liner threshold applies to the CALLABLE surface (the set of tools whose names/contracts the orchestrator-prompt enumerates and the orchestrator selects among), not the inventory. A's medium-harness can absorb cycle 102/104 sub-shape tools INTERNALLY into orchestration-hub crates:

  - **Boot-phase internalizes** `classify-cycle-task` + `route-cycle-decision` + cluster B classification-record write. The orchestrator sees only `boot-phase` output: a `cycle-context.json` artifact bundling resolved task classification + route + boundary state. The orchestrator does not invoke `classify-cycle-task` or `route-cycle-decision` directly; they execute as boot-phase internal pipeline stages.
  - **Close-phase internalizes** `infer-feedback-signal` + `evaluator-driven-consolidate` (running on the just-completed cycle's artifacts). The orchestrator sees a `cycle-feedback-summary.json` output from close-phase, not the inference-and-consolidation tool surface.

  Under this framing, A's orchestrator-callable surface stays ≤9 across the full sub-shape adoption window (`boot-phase`, `close-phase`, `phase-transition-check`, `wiki-search`, `cycle-history-append`, `gardening-sweep`, `detect-abandoned-cycles`, `prompt-contract-check`, `tool-registry`). The internalized sub-shape tools grow the inventory to ~15 but stay below the orchestrator's selection threshold.

  **Mitigation specification (cycle 105 sharpening of cycle 97 "hierarchical registry + compatibility metadata"):**

  1. **Two-tier registry:** orchestrator-callable tier (≤12 actively-orchestrator-invoked tools) + harness-internal tier (sub-shape-specific tools called by orchestration-hub crates). The orchestrator-prompt references only the callable tier; harness-internal tools are documented in the registry for transparency / debug / audit but not surfaced to orchestrator selection.
  2. **By-cluster registry grouping:** registry output groups callable tools by cluster (A / B / F / H / I / cross-cutting) rather than alphabetical, per PR #2877 lens-7 framing. Reduces selection-among-N to selection-among-N-per-cluster.
  3. **Compatibility metadata as I/O typed declarations:** each tool manifest declares input/output JSON shape (e.g., `consumes: cycle-issue.body, recent-journal-entries`; `produces: cycle-classification.json`). Allows orchestrator to discover tool fit by data-shape rather than by name disambiguation.
  4. **Active-callable bound as candidate-claim:** A commits to keeping orchestrator-callable tools at ≤12 across all sub-shape adoptions; further sub-shape additions beyond cycle 102/104's substrate require harness-internalization to maintain the bound. Falsifiable at Phase 3 prototype: count the orchestrator-prompt's tool-name references and verify ≤12 after cycle 102/104 substrate adoption.

  **Risk 3 status post-cycle-105:** direction continues to hold (tool-registry growth is bounded by structural decision: harness-internalization of sub-shape tools); magnitude refined from "v2 callable surface (~9 new crates) approaches threshold" to "v2 INVENTORY may reach 13-15 with full sub-shape adoption, but orchestrator-CALLABLE surface stays ≤12 by harness-internalization discipline." Mitigation specification is now concrete enough for Phase 3 prototype validation (the falsifiable bound is measurable). Operational closure deferred to Phase 3 prototype when the active-callable boundary discipline meets actual extraction work.
- **Risk 4 (CLOSED at specification level cycle 109):** maintaining the prompt-contract-check CI invariant (`prompt-contract-check` ensures named contracts in the prompt remain present and that no procedural step is added without a paired tool fix) requires ongoing attention; if Phase 3+ prototype extends the prompt to address novel situations, the procedural-surface ratio may regress. **Cycle 109 closure** decomposes prompt-extension events into a 5-type taxonomy (Type 1 bounded-mechanical / Type 2 novel-deterministic-procedural / Type 3 novel-judgment-surface / Type 4 tool-deprecation-driven / Type 5 emergency-hot-fix) under shape #21's substrate-decomposition element + procedural-surface ratio P as primary falsifiable bound (P ≤ 0.3 direction-validated / (0.3, 0.5] at-risk / >0.5 refuted) + per-type compliance rates (Type 1 100% / Type 2 ≥95% / Type 3 ≤10% reclassification / Type 4 ≥90% documented + ≥80% re-extracted / Type 5 ≥80% reclassified-or-paired within ≤5 cycles) + drift-window (cycles between extension event and paired-tool follow-up) as secondary derived metric per cycle 108 two-tier discipline + three-layer verification (per-extension-event log + external-observer reproducibility + CI sweep `prompt-extension-pairing-check`) + warm-up window for extension-classification rubric stability + combined-readings diagnostic preserving cycle 91 unit-of-analysis Risk 1 vs Risk 4 distinction. See [Extension-discipline taxonomy section below](#extension-discipline-taxonomy-cycle-109-specification-addressing-risk-4) for full detail.

These risks are bounded — none threaten the candidate's substrate-bet directly. They threaten the *magnitude* of the P3 reduction (which is now indeterminate without re-grounded baselines) but not the *direction* (medium-harness extraction reduces procedural surface — structurally plausible regardless of denominator). **P3 PASS direction continues to hold on structural argument; magnitude is currently indeterminate and awaits Phase 3 prototype measurement against verified baselines.** Per cycle 96 absorption: the cycle 91 sharpening's "magnitude is sharpened to ~40-50% rather than ~50%" conclusion was internally inconsistent with the README tracker's "~50-62%" label, and both rested on overcounted denominators; the README-tracker entry has been revised in this absorption pass.

### Extension-discipline taxonomy (cycle 109 specification, addressing Risk 4)

Per cycle 105/106/107/108 closure pattern (substrate-decomposition + falsifiable bound + verification + status), Risk 4 is closed at the specification level. Cycle 91 framing identified the conditional ("if Phase 3+ prototype extends the prompt, ratio may regress") but did not specify (a) what counts as a prompt extension that requires paired-tool extraction vs orchestrator judgment, (b) what compliance rate counts as "extraction-discipline holds" per extension type, (c) how compliance is measured non-circularly given the orchestrator both authors extensions and self-reports compliance.

A Risk 4 is the **second discipline-conditional risk closure** after C Risk 2 (cycle 107). Both share structure: a discipline (extraction pairing / plan authoring) that applies to a subset of events (extension events / cycles producing plans) with rubric-asymmetric ambiguity (rubric tightening affects WHICH events count as expected-to-comply, but each event's compliance is separately observable). Closure follows cycle 107's structure with substrate-content adapted to extension-discipline.

**Deferral arc:** 18 cycles (cycle 91 framing → cycle 109 closure). Compares to cycle 105's 8-cycle arc (97 → 105), cycle 106's 14-cycle arc (92 → 106), cycle 107's 15-cycle arc (92 → 107), and cycle 108's 16-cycle arc (92 → 108). The 18-cycle gap reflects A Risk 4's closure dependence on cycle 107 discipline-conditional substrate (warm-up window + rubric-fragility diagnostic) + cycle 108 two-tier metric discipline (drift-window as secondary derived) + cycle 91 unit-of-analysis Risk 1 (the rubric for what "step" means inside P). Closure-of-X depends on closure-of-prerequisites-of-X; cycle 109's 18-cycle arc is the longest of the 6 closures, absorbing 3 prior closures' substrate.

#### Choice 1: 5-type extension taxonomy as substrate-decomposition

Prompt extensions vary in what discipline applies. Designed 5-type taxonomy:

| Type | Description | Trigger | Discipline | Compliance bar |
|---|---|---|---|---|
| 1 bounded-mechanical | New tool reference / contract name added to prompt | Tool added to v2 inventory | Paired tool MUST exist before extension committed | 100% (any unpaired Type 1 is contract violation) |
| 2 novel-deterministic-procedural | New deterministic procedural pattern not in v1 or current prompt | Phase 3+ pattern emergence | Paired tool extraction REQUIRED at extension OR within drift-window ≤3 cycles | ≥95% steady-state |
| 3 novel-judgment-surface | New orchestrator-judgment surface (not deterministic) | Novel-situation response | NO paired tool required (judgment correctly NOT extracted) | ≤10% later-reclassification to Type 2 |
| 4 tool-deprecation-driven | v2 tool removed; prompt absorbs missing capability | Tool deprecation | Re-extraction-or-permanence-decision documented at deprecation; if procedure absorbed, paired-tool follow-up required within ≤5 cycles | ≥90% documented; ≥80% re-extracted within window |
| 5 emergency / hot-fix | Constraint accretion under time pressure | Incident response | Paired-tool follow-up required within ≤5 cycles, OR Type 5 reclassified to Type 2/3/4 | ≥80% reclassified-or-paired within ≤5 cycles |

**Alternatives rejected:**
- Binary extension-discipline (extensions either pair or don't) — conflates Type 1 with Type 3/5; loses the discipline-applicability distinction.
- 3-type simpler taxonomy (deterministic / judgment / emergency) — collapses Type 1+2 (both deterministic but different time discipline) and Type 4+5 (both reactive but different cause).
- Retrospective-only classification — not falsifiable; classifying after seeing prompt growth is circular.

**Substrate partition:** discipline-applicable Types 1 + 2 + 4 + 5 (paired-tool requirement applies; ~80% of expected extension volume) + discipline-non-applicable Type 3 (judgment-surface; pairing would itself violate CORE-DESIGN-PRINCIPLE; ~20% of expected volume). Type 3 is the discipline-conditional analog to cycle 107 Type 5 (NO-PLAN annotation IS the discipline) and cycle 106 mechanical Type A exclusion (mechanical operations excluded from structural unit).

#### Choice 2: Procedural-surface ratio P as primary falsifiable bound (rubric-symmetric)

Per cycle 103/106/108 ratio-based threshold pattern:

P = (procedural steps in prompt) / (total prompt step + named-tool count)

Pre-agreed thresholds:
- P ≤ 0.3 (≥70% extracted to tools): direction-validated; medium-harness extraction discipline holds
- P ∈ (0.3, 0.5]: at-risk; second-iteration prompt examination needed
- P > 0.5: refuted; medium-harness extraction discipline failed; A's central bet undermined

P depends on the unit-of-analysis decision (Risk 1 from cycle 91 sharpening — step IDs vs named patterns vs prose-procedural clauses). P is rubric-symmetric in the cycle 108 sense: rubric tightening affects both numerator (procedural steps) and denominator (total step + tool count) equally. If "step" is redefined upward (only mechanical-action steps count), procedural-step count drops AND total-step count drops; ratio is rubric-robust within the chosen unit.

**Alternatives rejected:**
- Absolute step count as primary — depends on prompt size; a longer prompt can have more procedural steps without violating extraction discipline. Ratio is more interpretable.
- Type-1-specific compliance rate as primary — captures one type's discipline only; misses Type 2/4/5 drift modes.
- Aggregate prompt line count — rubric-asymmetric (formatting changes affect lines without changing procedural surface).

#### Choice 3: Drift-window as secondary derived metric (cycle 108 two-tier discipline transfer)

For Type 2/4/5 extensions, drift-window D = (cycles between extension event and paired-tool follow-up extraction OR reclassification). Per cycle 108 Choice 3 two-tier primary-vs-secondary-derived metric discipline:

- D ≤ 3 cycles for Type 2: aggregate drift bounded; consistent with Type 2 ≥95% compliance
- D ∈ (3, 5] for Type 4: bounded under window
- D > 5 for Type 4/5: extension has aged into permanent absorption; reclassify or accept ratio regression

D is **secondary derived (informational)**, NOT primary falsificational, because:
- D depends on auxiliary measurements (extension timing + paired-tool follow-up timing — both separately measured).
- Procedural-surface ratio P (Choice 2) already captures aggregate extraction discipline regardless of per-extension drift.

Cycle 108 two-tier metric discipline promotes from NOVEL@1 → **TESTED@2 (cycle 109)**. The methodological pattern (when a metric depends on auxiliary measurements, demote to secondary derived to avoid refutation conflation) holds at second instance with substrate-content adapted from iteration-multiplier (cycle 108) to drift-window (cycle 109).

**Alternatives rejected:**
- Make drift-window primary falsifiable — conflates extension event count with paired-tool follow-up timing; refutation on D alone would not distinguish "many extensions, fast follow-up" from "few extensions, slow follow-up" (diagnostically equivalent under D, structurally distinct under per-type compliance rates).
- Omit drift-window entirely — loses diagnostic resolution for Type 4/5 reactive extensions; binary "paired or not" misses the time-pressure interaction.

#### Choice 4: Three-layer verification (cycle 103/107/108 transfer)

Per-extension-event log schema:
```jsonc
{
  "extension-type": 1 | 2 | 3 | 4 | 5,
  "prompt-section": "<section name>",
  "trigger-cause": "<cycle-event description>",
  "cycle": <cycle-id>,
  "paired-tool": "<tool name>" | null,
  "paired-tool-status": "exists" | "follow-up-required-by-cycle-N" | "not-required",
  "drift-window-start-cycle": <cycle-id> | null,
  "drift-window-end-cycle": <cycle-id> | null,
  "decision-rationale": "<why this extension type vs another>"
}
```

Three-layer verification (inheriting cycle 103/107/108 pattern intact):
1. Per-extension-event logging by orchestrator at extension time (primary).
2. External-observer reproducibility check at Phase 3 measurement (≥20% sample re-classified by Eva, audit, or independent Copilot dispatch; inter-rater disagreement ≤10% steady-state past cycle 5 of measurement).
3. CI sweep (`prompt-extension-pairing-check` — extends `prompt-contract-check` to verify Type 1/2 extensions have paired tools at commit time and Type 4/5 follow-up windows are tracked).

Three-layer verification at HARDENED-at-4 across 4 closure types (cycle 103 counting protocol, cycle 107 plan-authoring discipline, cycle 108 iteration events, cycle 109 extension events). Now appears in 4 of 6 closures (cycle 105 used substrate-absorption-tool-count delta as the unit; cycle 106 used per-coordination-decision logging without the three-layer pattern explicitly named — it implicitly applies). Promoted from "shape #21 sub-element" candidate (cycle 108 observation 3) toward functional-class shape #25 candidate.

**Alternatives rejected:**
- Self-classification alone — exact cycle 96 self-report-circularity concern; orchestrator self-grading on whether its own extension was Type 1 vs Type 2 is the same shape as B's self-counting concern.
- Manual tracking via journal entries — unstructured, not CI-checkable, classification-quality varies cycle-to-cycle.
- Instrument every prompt commit (full diff classification) rather than extension-specific events — introduces classification overhead on non-extension commits (typo fixes, formatting); per cycle 106 mechanical-vs-structural reasoning, mechanical edits should be excluded from the structural unit.

#### Choice 5: Extension-classification rubric stability as warm-up window discipline (cycle 107/108 shape #24 transfer)

Direct application of cycle 107/108 shape #24:
- Warm-up window: cycles 1-5 of Phase 3 measurement. Extension-classification rubric is being tightened (what counts as Type 1 paired vs Type 2 novel-deterministic; whether tool-deprecation-with-replacement-tool-added is Type 4 or natural Type 1; whether emergency hot-fix that gets reclassified as Type 3 judgment-surface counts as Type 5 success or Type 3 misclassification).
- Steady-state: past cycle 5 of measurement. Per-type compliance rates apply. Inter-rater disagreement on extension type ≤10% expected.
- Combined-readings against rubric stability: see Choice 6.

Shape #24 promotes HARDENED@3 (cycle 108 iteration-event-classification stability) → **HARDENED@4 (cycle 109 extension-classification stability)** spanning 4 protocol-stability types: measurement-bound stability (cycle 106 reducer-rule revision rate) + rubric stability (cycle 107 cycle-type taxonomy other-rate) + classification stability (cycle 108 iteration-event inter-rater disagreement) + extension-rubric stability (cycle 109 extension-event inter-rater disagreement).

Critical methodological observation: cycle 109 is **rubric-asymmetric** like cycle 107 (discipline-conditional rubric-fragility) — rubric tightening affects WHICH extensions count as expected-to-comply (Type 1 vs Type 3 boundary) but each extension's pairing/non-pairing is separately observable. Cycle 108 was rubric-symmetric (quantity-bounded). Shape #24 (warm-up window) applies to both rubric-symmetric and rubric-asymmetric, but the diagnostic structure differs — see Choice 6.

**Alternatives rejected:**
- Measure extensions from cycle 1 with no warm-up — conflates rubric-evolution with extension-discipline-instability per cycle 106/107/108 reasoning.
- Treat extension-classification rubric as fixed (no stability check) — without stability check, Type-distribution and per-type compliance rates are uninterpretable when classification disagreement is high.

#### Choice 6: Combined-readings diagnostic preserving cycle 91 unit-of-analysis Risk 1 vs Risk 4 distinction (discipline-conditional rubric-fragility diagnostic transfer)

Per cycle 107 Choice 6 discipline-conditional rubric-fragility diagnostic, this is the **second instance** of the discipline-conditional rubric-fragility pattern. A Risk 4 has the same one-sided rubric ambiguity as C Risk 2: rubric tightening affects WHICH events count as expected-to-comply (Type 1/2/4/5 vs Type 3 boundary; or Type 2 vs Type 4 distinction) but each event's compliance is separately observable.

Both Risk 1 (cycle 91 unit-of-analysis) and Risk 4 (extension-discipline) manifest at refutation as P > 0.5. Without combined-readings, refutation at P cannot distinguish causes. Per cycle 107 Choice 6 pattern, a 4-quadrant matrix:

| Procedural-surface ratio P | Extension-rubric stability (other-rate past cycle 5) | Diagnostic |
|---|---|---|
| P ≤ 0.3 | ≤ 10% other-rate | A direction validated; extraction-discipline holds |
| P ≤ 0.3 | > 10% other-rate | Compliance is cherry-picked over a rubric-unstable subset; revise extension taxonomy before reading P-value (possibly Risk 1: unit-of-analysis is wrong) |
| P > 0.5 | ≤ 10% other-rate | Risk 4 fires; extraction-discipline failed; magnitude refuted (rubric is sound) |
| P > 0.5 | > 10% other-rate | Both Risk 1 and Risk 4 fire; A's central bet refuted; revisit candidate or accept >0.5 ratio |

Discipline-conditional rubric-fragility diagnostic promotes from NOVEL@1 (cycle 107 plan-authoring substrate) → **TESTED@2 (cycle 109 extension-discipline substrate)**. The diagnostic structure (combined-readings table preserving discipline-vs-rubric-stability distinction) holds across two different discipline-conditional risk substrates. Methodological observation: the two-axis combined-readings table appears to be the natural diagnostic shape for discipline-conditional rubric-fragility — one axis is the primary falsifiable bound (compliance rate / P), the other is rubric-stability (other-rate / inter-rater disagreement).

**Alternatives rejected:**
- Procedural-surface ratio P alone — high "other" rate masks discipline failure (cherry-picked compliance over a rubric-unstable subset).
- Combine taxonomy stability into P (count "other" as Type-2 non-compliant) — conflates two failure modes; loses diagnostic resolution for revising the candidate post-Phase-3.
- Use cycle 108's combined-readings (per-role decision count × iteration events) — orthogonal metrics; doesn't apply to extension-discipline substrate where the relevant axes are P and rubric-stability.

### Risk 4 status post-cycle-109

- **Direction continues to hold** by construction.
- **Magnitude refined** from "if Phase 3+ prototype extends the prompt to address novel situations, the procedural-surface ratio may regress" to: 5-type extension taxonomy / per-type compliance rates / procedural-surface ratio P bounded ≤0.3/0.5 / drift-window D as secondary derived metric / three-layer verification / warm-up-window extension-classification stability / combined-readings diagnostic preserving Risk-1-vs-Risk-4 distinction.
- **Mitigation specification is now concrete enough for Phase 3 prototype validation** (extension events logged from cycle 1 of Phase 3; per-type compliance rates measurable by cycle 5+ steady state; P measurable per-cycle).
- **Operational closure deferred** to Phase 3 prototype when extension-event logging meets actual prompt-evolution events AND inter-rater disagreement on extension type ≤10% steady-state AND per-type compliance rates measured across ≥10 measurement cycles AND drift-window D measured for Type 2/4/5 extensions.
- **Shape #21 transfer verdict:** structural form transfers (substrate-decomposition + falsifiable bound + verification + status); cycle 109 instance is the second discipline-conditional application of shape #21 (after cycle 107 plan-authoring discipline). **Shape #21 promotes HARDENED-at-5 → HARDENED-at-6** spanning 6 risk-domain types (meta-counting-protocol / tool-registry-growth / coordination-stability / plan-authoring-discipline / per-role-iteration / extension-discipline) AND 2 risk-shape types (4 quantity-bounded + 2 discipline-conditional). Methodological observation: the 4-element closure structure (substrate + bound + verification + status) is robust across both quantity-bounded and discipline-conditional risk-shape types AND across multiple instances of each.
- **Shape #24 transfer verdict:** cycle 109 extension-classification stability is the **fourth stability type** after measurement-bound (cycle 106) + rubric (cycle 107) + classification (cycle 108). **Shape #24 promotes HARDENED@3 → HARDENED@4** spanning 4 protocol-stability types.
- **Discipline-conditional rubric-fragility diagnostic** (cycle 107 NOVEL@1) promotes **NOVEL@1 → TESTED@2** at second instance with substrate-content adapted from plan-authoring (C Risk 2) to extension-discipline (A Risk 4).
- **Two-tier primary-vs-secondary-derived metric discipline** (cycle 108 NOVEL@1) promotes **NOVEL@1 → TESTED@2** at second instance with substrate-content adapted from iteration-multiplier (B Risk 4) to drift-window (A Risk 4).
- **Candidate-rotation-balance-discipline** (cycle 107 NOVEL@1) promotes **NOVEL@1 → TESTED@2** by cycle 109 honoring the cycle 108 _notes-named rotation commitment (A Risk 4 chosen over higher-priority B-side targets to balance closure distribution toward A:2 / B:3 / C:1).

## Risk-closure threshold summary (cycle 121 consolidation)

Cycles 103-110 closed 7 risks across the three candidates at the specification level. A holds 2 of these (Risks 3 and 4); both follow shape #21 (substrate-decomposition + falsifiable bound + verification + status). The closures are detailed in the per-cycle sections above; this table consolidates them as a single Phase-3-measurement-planning entry point. Risks 0/1/2 named at the structural level remain open with direction-supporting arguments only; Phase 3 prototype measurement is the path to operational closure for those as well.

| Risk | Description | Closure cycle | Primary falsifiable threshold | Verification mechanism | Phase 3 operational closure target |
|---|---|---|---|---|---|
| **3** | Tool-registry growth past super-linear coordination threshold (~15-25 callable units) erodes safe invocation selection | 105 | Orchestrator-CALLABLE surface **≤12** across all sub-shape adoptions; v2 INVENTORY can reach 13-15 by harness-internalization | Two-tier registry (callable tier ≤12 + harness-internal tier); orchestrator-prompt tool-name reference count at Phase 3 prototype | When active-callable boundary discipline meets actual extraction work (Phase 3 prototype scaffolding cycles after orchestration-hub crates land) |
| **4** | Prompt-contract-check CI invariant maintenance; novel prompt extensions may regress procedural-surface ratio | 109 | Procedural-surface ratio **P ≤ 0.3** direction-validated / (0.3, 0.5] at-risk / **>0.5 refuted**; per-type compliance: Type 1 100% / Type 2 ≥95% / Type 3 ≤10% reclassification / Type 4 ≥90% documented + ≥80% re-extracted / Type 5 ≥80% reclassified-or-paired within ≤5 cycles; drift-window D ≤3 (Type 2) / ≤5 (Type 4) as secondary derived | 5-type extension taxonomy (bounded-mechanical / novel-deterministic / novel-judgment / tool-deprecation / emergency) + per-extension-event log (JSONC schema) + external-observer reproducibility ≥20% sample with ≤10% inter-rater disagreement + CI sweep `prompt-extension-pairing-check`; warm-up window cycles 1-5; combined-readings 4-quadrant diagnostic preserving Risk-1-vs-Risk-4 distinction | When extension-event logging meets actual prompt-evolution events AND inter-rater disagreement ≤10% steady-state past cycle 5 AND per-type compliance rates measured across ≥10 measurement cycles AND drift-window D measured for Type 2/4/5 |

**Cross-candidate context:** A's 2 closures position A as quantity-bounded (Risk 3 ratio bound) plus discipline-conditional (Risk 4 extension-rubric stability). The discipline-conditional pattern (Risk 4) is the cycle 109 second instance of the cycle 107 NOVEL@1 discipline-conditional rubric-fragility diagnostic — confirms the diagnostic structure is risk-shape-type-general, not C-specific. Distribution: A:2 (Risks 3+4) / B:3 (Risks 8+2+4) / C:2 (Risks 2+5) — closure count is approximately balanced; differential lies in risk-shape-type-mix (A favors discipline-conditional + quantity-bounded mix; B favors quantity-bounded; C favors discipline-conditional).

**Substrate-pattern observation:** all 7 closures share the 4-element structure (substrate-decomposition + falsifiable bound + verification + status); shape #21 is HARDENED-at-7 across 7 risk-domain types and 2 risk-shape types per cycle 110 substrate. The 3-layer verification pattern (per-event log + external-observer reproducibility + CI sweep) appears in 4 of 6 closures (cycle 103 / 107 / 108 / 109) and is the candidate for shape #25 promotion to HARDENED@3 at cycle 110.

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
7. **Specific-crate under-estimate risk (added cycle 97 per PR #2877 lens-1 absorption).** PR #2877's per-crate critique names 3 crates as likely under-estimated: `boot-phase` and `close-phase` (orchestration hubs touching state load + cursor advance + directive checks + journal/issue writes + push semantics — 200-500 LOC optimistic with robust error paths and tests); `wiki-search` (top-k retrieval over `_notes/*.md` description-frontmatter — practical load includes indexing, update strategy, ranking semantics, corruption handling, query contract — likely >500 LOC). Counterbalanced: `phase-transition-check`, `tool-registry`, `cycle-history-append` are likely over-estimated at upper bound (the 2 measured crates fall at lower end of range, supporting this). Net implication: aggregate trajectory may shift toward PR #2877's revised 3600-6200 LOC range. Magnitude validation requires measuring at least one orchestration-hub crate (`boot-phase`) and `wiki-search` to determine whether the under-estimate exceeds the over-estimate offset.
8. **V1-baseline-calibration anchoring (added cycle 97 per PR #2877 calibration).** PR #2877 verified that v1's existing 38 Rust crates have median ~1081 LOC and mean ~2118 LOC (verified independently this cycle: 38 crates, median 1081.5, mean 2118; 7/38 i.e. 18% under 500 LOC). A's claim of 200-500 LOC per v2 crate is ~5× simpler than v1's median. The bet is defensible under CORE-DESIGN-PRINCIPLE: v1 crates accreted complexity over time (e.g., `pipeline-check` at 16,487 LOC, `write-entry` at 10,919 LOC, `cycle-runner` at ~5,200 LOC); v2 crates designed under tools-handle-rote / orchestrator-handles-judgment should be narrower by construction. But the bet is conditional — if v2 crates accumulate complexity at the same rate as v1, A's per-crate-bound is wrong and aggregate net-add doubles. Phase 3 prototype for an orchestration-hub crate (`boot-phase` or `close-phase`) tests this directly.

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
