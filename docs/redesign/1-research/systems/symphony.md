# openai/symphony — issue-driven autonomous coding-agent harness

[← back to Phase 1 index](../../1-research.md)

**Status: first-pass; spec + impl-shape; deeper-read queue documented.**
A Copilot research-only dispatch landed cycle 98 as
PR [#2873](https://github.com/EvaLok/schema-org-json-ld/pull/2873)
(originating issue [#2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851),
cycle-77 dispatch under Eva directive
[#2775](https://github.com/EvaLok/schema-org-json-ld/issues/2775)).
Per the absorption convention (PR closed without merge; deliverable
preserved on never-merged branch), the deliverable lives on branch
`copilot/redesign-research-first-pass-survey-cycle-77` at commit
`2d940994` as `docs/redesign/_notes/cycle-77-symphony-survey.md`
(644 lines; 12-lens structure: repo glance, spec architecture, Elixir
impl architecture, spec-as-contract test, workflow contract, tracker
integration, workspace isolation, lifecycle/phasing, trust posture,
22-pattern cross-reference, NEW patterns + hypothesis verdicts,
anchoring caveats). This per-system file summarizes the deliverable
in the per-system shape and cites it as the primary evidence base.

**Verification status (cycle 98):** load-bearing factual claims
spot-checked against the repository — SPEC.md size 80204 ✓ (claimed
80204), README.md size 1731 ✓ (claimed 1731), elixir/lib file sizes
(orchestrator.ex 52564 ≈ claimed 52KB, status_dashboard.ex 60589 ≈
claimed 60KB, workspace.ex 14626 ≈ claimed 14KB), config/schema.ex
16800 ≈ claimed 16KB, linear/client.ex 16285 ≈ claimed 16KB,
codex/app_server.ex 30051 ≈ claimed 30KB, GitHub metadata
(created 2026-02-26T21:54:00Z ✓ exact, language Elixir ✓, stars 22513
vs current 22641 — drift consistent with elapsed time, forks 2097 vs
current 2114 — same). Eight quantitative claims verified to
byte-level precision; this is verification-success of the same shape
as cycle-97 PR #2877 (calibration-first methodology, distinct from
cycle-96 PR #2878 fabrication-magnitude failure).

## Sources read so far

- Root docs: `README.md`, `SPEC.md`.
- Elixir docs: `elixir/README.md`, `elixir/AGENTS.md`,
  `elixir/WORKFLOW.md`.
- Implementation-shape files: `elixir/lib/symphony_elixir.ex`
  (entrypoint), `orchestrator.ex` (52KB, runtime authority),
  `workflow.ex` + `workflow_store.ex` (workflow loader/cache),
  `tracker.ex` + `linear/adapter.ex` + `linear/client.ex`
  (tracker boundary + Linear adapter), `workspace.ex` + `path_safety.ex`
  (workspace lifecycle + symlink-aware path safety),
  `agent_runner.ex` (per-issue agent runtime + Codex thread),
  `config/schema.ex` (typed config layer).
- GitHub API directory listings for structure/size verification.
- Repository search API for project metadata snapshot.

## Project framing

Symphony is **organizational issue-driven automation**, not an
interactive coding helper. The README positions it as "isolated,
autonomous implementation runs" so teams "manage work instead of
supervising coding agents" (`README.md:3-4`). The runtime polls a
tracker (Linear in v1), creates per-issue isolated workspaces, and
runs Codex agent sessions inside each workspace until the issue
reaches a workflow-defined terminal state.

The repo deliberately splits into **two layers**:

1. **Language-agnostic contract** (`SPEC.md`, ~80KB) — RFC 2119
   normative spec stating what any conforming Symphony implementation
   must do. README "Option 1" actively invites reimplementation in
   any language (`README.md:21-27`).
2. **Reference Elixir implementation** — labeled "experimental
   reference" (`README.md:28-35`), not sole canonical runtime. OTP
   supervision tree + GenServer runtime authority + Task.Supervisor
   per-issue worker lifecycle (`symphony_elixir.ex:26-39`,
   `orchestrator.ex:24-42`).

This is structurally distinct from oh-my-codex/oh-my-claudecode
(thin layer over CLI), AutoGen/LangGraph (framework defining own
runtime), and Voyager (research code with its own loop). Symphony
is **spec-first contract architecture with one reference impl** — a
substrate plurality posture cycle-77 surfaces as a NEW pattern not
captured by the cycle-26 22-pattern catalogue.

## Patterns observed

### State, memory, history

- **Authoritative in-memory orchestrator state, no persistent
  orchestrator DB.** `SPEC.md:262-274` requires authoritative
  scheduler state in-process; `SPEC.md:55-56`/`693-695` says restart
  recovery is tracker + filesystem driven. Implementation realizes
  this in the orchestrator GenServer state struct
  (`orchestrator.ex:29-42`) plus startup terminal cleanup
  (`orchestrator.ex:67`, `orchestrator.ex:882-896`).
- **Tracker as truth for issue state; filesystem as truth for
  workspace artifacts; in-memory as truth for runtime claims.**
  Three-tier persistence model where each tier has a defined source
  of truth and reconciliation rules between them.
- **Workspace persistence across runs.** `SPEC.md:824-825` says
  successful runs do not auto-delete; workspace reused across runs.
  This is "persistent per-issue workpad" rather than ephemeral
  tempdir.
- **Live workflow reload with last-known-good retention.**
  `SPEC.md:524-540` requires dynamic reload without restart;
  implementation polls every second and on parse failure logs error
  while keeping prior effective config (`workflow_store.ex:11`,
  `workflow_store.ex:83-94`, `workflow_store.ex:151-152`,
  `elixir/README.md:148-150`).

### Orchestration & system shape

- **Named-phase lifecycle vocabulary.** Internal states `Unclaimed`,
  `Claimed`, `Running`, `RetryQueued`, `Released` with explicit
  transition triggers (`SPEC.md:608-687`); run lifecycle phases
  `PreparingWorkspace` → `BuildingPrompt` → `LaunchingAgentProcess`
  → `InitializingSession` → `StreamingTurn` → terminal outcomes
  (`SPEC.md:639-654`).
- **Reconciliation-before-dispatch tick pattern.** Each poll tick:
  refresh runtime config → reconcile running issues → validate
  config + fetch candidates → dispatch eligible work → manage
  retries (`orchestrator.ex:225-230`, `orchestrator.ex:519-531`,
  `orchestrator.ex:773-810`).
- **Continuation on same live thread under explicit cap.** Spec
  continuation model (`SPEC.md:628-634`) says worker may continue
  multiple turns on same live thread while issue remains active;
  implementation does this up to `agent.max_turns`
  (`agent_runner.ex:92-145`), using continuation guidance rather
  than re-sending full prompt.
- **Bounded concurrency multi-issue dispatch.**
  `Task.Supervisor`-managed per-issue workers with concurrency cap
  (`SPEC.md:48`, `SPEC.md:711`, `orchestrator.ex:740-747`).

### Documentation honesty

- **Spec-as-contract architecture.** `SPEC.md` is operationally
  detailed (problem framing → goals/non-goals → component
  architecture → domain model → workflow contract → state machine
  → scheduling → workspace safety → agent protocol → tracker
  contract → prompt/observability), opens with explicit RFC 2119
  interpretation clause (`SPEC.md:7-10`), defines "implementation-
  defined" obligations explicitly (`SPEC.md:12-14`), and binds
  multiple normative requirements per subsystem.
- **Goals/non-goals discipline as scope-fence.** Spec actively
  constrains expected interpretation: not a rich control plane
  (`SPEC.md:60-63`), no built-in business logic for ticket editing
  (`SPEC.md:63-64`), no single mandated sandbox/approval posture
  (`SPEC.md:65-67`).
- **Workflow document as in-repo operations contract.**
  `elixir/WORKFLOW.md` is process policy encoded in source-controlled
  workflow artifact: state routing rules (`Todo`, `In Progress`,
  `Human Review`, `Merging`, `Rework`, `Done`) at `WORKFLOW.md:106-127`,
  completion-bar gating at `WORKFLOW.md:262-270`, guardrails at
  `WORKFLOW.md:274-290`. Goes beyond "generic prompt engineering."

### Trust posture & security defaults

- **Trust posture plurality with implementation-defined choices.**
  Spec explicitly refuses universal trust policy
  (`SPEC.md:31-34`, `SPEC.md:65-67`, `SPEC.md:1023-1031`).
- **Workspace safety as hard runtime invariant with symlink-aware
  canonicalization.** Path validation canonicalizes paths and
  rejects root-equal, outside-root, and symlink-escape cases
  (`workspace.ex:358-379`, `path_safety.ex:4-15`,
  `path_safety.ex:28-34`). Workspace identifier sanitized to
  `[A-Za-z0-9._-]` via substitution (`workspace.ex:206-208`).
- **Default-deny-style technical defaults** (Elixir reference
  posture): reject-style approval policy default, `thread_sandbox`
  default `workspace-write`, `turn_sandbox_policy` default rooted
  to issue workspace, `networkAccess: false` in default turn policy
  (`elixir/README.md:113-117`, `config/schema.ex:162-173`,
  `config/schema.ex:482-490`).
- **Tracker-state-driven stop/start gating.** Implementation
  terminates active runs when issue enters terminal state, non-active
  state, or becomes unroutable (`orchestrator.ex:347-366`); terminal
  transitions optionally trigger workspace cleanup
  (`orchestrator.ex:415-426`).

### Quality & discipline

- **Quality gate as required step.** `elixir/AGENTS.md:9` enforces
  `make all` quality gate; `elixir/AGENTS.md:37-40` requires specs
  for `def`. Workflow completion bar requires validation/checks
  green and feedback sweep complete before `Human Review`
  (`WORKFLOW.md:227-233`, `WORKFLOW.md:262-270`).
- **Iteration ceilings with explicit ceilings.** `agent.max_turns`
  cap (`WORKFLOW.md:30`); retry backoff bounds
  (`SPEC.md:415-421`, `orchestrator.ex:928-939`).
- **Handoff state vs blocker state distinction.** `Human Review`
  modeled as normal handoff state, not blocker state — workflow
  continues to operate while waiting (`SPEC.md:41-42`,
  `WORKFLOW.md:110`, `WORKFLOW.md:241-247`).

### Agent architecture

- **OTP/BEAM substrate exploited.** `use Application` supervision
  tree (`symphony_elixir.ex:20-39`); GenServer runtime state owner
  for orchestrator (`orchestrator.ex:6`, `orchestrator.ex:24-42`);
  `Task.Supervisor` for per-issue worker process spawning/termination
  (`symphony_elixir.ex:28`, `orchestrator.ex:694-699`,
  `orchestrator.ex:507-514`); `WorkflowStore` as own `GenServer`
  for hot reload + last-known-good (`workflow_store.ex:6`,
  `workflow_store.ex:151-152`); process-message communication
  between runner and orchestrator (`agent_runner.ex:55-72`,
  `orchestrator.ex:166-201`). At first-pass depth, BEAM is used for
  OTP-native concurrency/supervision, not as generic functional
  syntax.
- **Tracker adapter boundary.** `SymphonyElixir.Tracker` defines
  callback contract (`tracker.ex:8-12`); delegates via `adapter()`
  selected by config kind (`tracker.ex:40-44`). Available adapters:
  `Linear.Adapter` (default), `Tracker.Memory` (tests/local dev).
  Architecturally prepared for non-Linear, but production path
  remains Linear-first; non-Linear adapters require new behavior
  impl + config semantics + normalization parity.

## Hypothesis verdicts (per cycle-77 dispatch)

The deliverable's H1-H4 hypothesis verdicts at first-pass depth:

- **H1 (organizational-substrate vocabulary distinct from solo-author
  systems): CONFIRMED 5/5.** Spec-driven design artifact distinct
  from README, language-agnostic stance, RFC 2119 normative
  vocabulary, explicit goals/non-goals, implementation-defined
  plurality on trust posture.
- **H2 (Elixir/BEAM substrate exploitation): CONFIRMED (strong).**
  OTP supervision tree, GenServer orchestration owner,
  `Task.Supervisor` worker lifecycle, process message passing for
  runtime updates, typed config layering exceed substrate-exploitation
  threshold.
- **H3 (cluster I substrate-correlation prediction): CONFIRMED
  (moderate-to-strong).** At least 3 cluster-I sub-shapes visible:
  workspace isolation as policy enforcement, tracker-state-driven
  stop/start gating, trust-posture plurality with
  implementation-defined approval/sandbox choices. Symphony would
  be **third convergent case** for cluster-I substrate-correlation
  (after openclaw + OpenAI harness writeup) IF it reaches
  deep-dive parity in a follow-up read.
- **H4 (spec-as-contract pattern): CONFIRMED (strong).**
  Operationally-detailed spec, explicit language-agnostic posture,
  explicit RFC 2119 normative contract.

## Anchoring caveats

- **First-pass depth, not deep-dive parity.** The cycle-77 read
  covered repo + spec + impl-shape but **deferred** full
  clause-by-clause conformance audit of `codex/app_server.ex`
  against `SPEC.md` MUSTs, full audit of `status_dashboard.ex`
  against monitoring guidance, exhaustive proof of tracker
  abstraction beyond current `memory` adapter, and full RFC2119
  clause-by-clause cross-check. A deeper read is the prerequisite
  for treating Symphony as a 9th deep-dive system in the cluster
  catalogue.
- **Multi-issue org-automation target vs single-repo redesign
  context.** Symphony is built for concurrent multi-issue automation
  (`SPEC.md:48`, `SPEC.md:711`); redesign context is one repo with
  orchestrator cycles. Raw concurrency-slot policies do not
  transfer; named lifecycle states + retry semantics +
  reconciliation-before-dispatch pattern do.
- **Linear-specific implementation behind tracker abstraction.**
  Tracker abstraction exists but production path is Linear-first
  (`linear/client.ex:13-55`, `config.ex:125-130`). Tracker
  contract shape transfers; Linear query specifics do not.
- **BEAM/OTP substrate coupling.** Concrete primitives
  (`GenServer`, `Task.Supervisor`, supervision tree) cannot be
  copied directly to GitHub Actions cron substrate. Conceptual
  equivalents (single authoritative state owner, bounded worker
  lifecycle, explicit retry-timer ownership) still transfer.
- **RFC 2119 normative spec vs descriptive prompt style.** Symphony
  uses RFC 2119 normative spec language; redesign prompt tradition
  is more descriptive. Selective normative clauses for safety- or
  process-critical behavior appear beneficial; not every redesign
  artifact should become standards-language heavy.
- **`Human Review` handoff vs question-for-eva blocking.** Symphony
  models `Human Review` as normal handoff state with continued
  operation; redesign's question-for-eva is primarily blocker.
  Semantic equivalence should not be assumed; explicit handoff-state
  modeling is still useful as a design option.

## Cross-reference to cycle-26 22-pattern catalogue

Per the cycle-77 deliverable (section 10), Symphony's mapping
against the cycle-26 oh-my-codex 22 named patterns:

- **PARALLEL (7):** explicit stop conditions with named escalation
  paths; evidence-backed completion not assertion-backed; iteration
  limits with explicit ceiling; parallel delegation over
  sequential; session/root scope state with explicit reconciliation;
  autonomy directive as first line of orchestration doc;
  compatibility layer separate from authoritative state.
- **ADAPTED (6):** workflow as named keyword keywords with
  transition policy; context snapshot grounding before execution;
  behavioral prompt-contract with test coverage; MCP transport
  failure has defined fallback; explicit false-green detection for
  install readiness; coverage gate for critical modules.
- **ABSENT (6):** file-backed migration with one-way compatibility
  windows; document-refresh warning integrated into commit path;
  deslop pass as mandatory post-completion step; wiki as persistent
  searchable local knowledge base; lore-format commit signing for
  agent-authored commits; one survey-depth-absent call.
- **NOT-COMPARABLE (4):** pre-execution gate for underspecified
  requests; autoresearch as bounded supervisor/candidate loop;
  triage advisory routing without keyword activation; deterministic
  keyword detection over heuristic prompt parsing.

Interpretation: Symphony is **not** "same-author architecture echo"
of oh-my-codex. Parallel overlap appears mostly in
lifecycle/quality/autonomy primitives; ABSENT/NOT-COMPARABLE
clusters around keyword-mode UX (Symphony is tracker-state-driven,
not keyword-triggered) and substrate-specific patterns (one-way
file migration, wiki, deslop) that don't fit Symphony's
spec-first/team-org substrate.

## NEW patterns surfaced by Symphony (single-system, pending elevation)

Per the cycle-77 deliverable (section 11.1), patterns Symphony adds
that are not captured by the cycle-26 22-pattern baseline:

1. **Spec-first language-agnostic contract as primary artifact.**
   `SPEC.md` (80KB) defines portable service contract; README
   "Make your own" invitation; RFC 2119 normative framing.
2. **Dual-contract architecture: global spec + repo-local workflow
   contract.** `SPEC.md` defines portable service contract;
   `WORKFLOW.md` carries per-deployment policy + prompt runtime.
3. **Tracker/filesystem restart recovery without orchestrator DB.**
   Authoritative state in-memory; recovery from tracker + filesystem
   on startup.
4. **Workspace safety as hard runtime invariant with symlink-aware
   canonicalization.** Path containment + symlink-escape detection
   in implementation, not just policy in docs.
5. **Live workflow reload with last-known-good retention as
   reliability primitive.** Spec requires no-crash invalid reload;
   impl logs error and retains current config.
6. **Continuation on same live Codex thread across multiple turns
   under explicit cap.** Continuation guidance rather than full-
   prompt resend.

These are **single-system observations** at first-pass depth. Per
the cross-system convergence discipline, single-system patterns are
held in `_notes` pending elevation via deeper-read or deeper
adversarial-on-adversarial review. Patterns 1-2 (spec-first +
dual-contract) are the strongest candidates for cross-system
elevation if a future deep-dive system is found that exhibits
similar normative-contract architecture (none of the existing 8
deep-dive systems match this shape).

## To-be-completed (deeper-read queue)

The cycle-77 deliverable explicitly defers four follow-up items to
a deeper-read stage:

1. Full clause-by-clause conformance audit of `codex/app_server.ex`
   against all `SPEC.md` protocol MUST clauses (including the
   dynamic-reload parity caveat flagged in deliverable section 3.4).
2. Full audit of `status_dashboard.ex` and observability API
   against spec's optional monitoring interface guidance.
3. Exhaustive tracker abstraction proof for non-Linear
   implementations beyond current `memory` adapter.
4. Full cross-check of every RFC 2119 clause against implementation
   behavior; this survey confirms many representative clauses, not
   complete formal conformance.

A code-level deeper-read dispatch could be filed under a new
research issue if Symphony moves from first-pass to deep-dive
status. At present, Symphony's first-pass depth is adequate for
Phase 1 corpus expansion (8th deep-dive system + first-pass 9th)
and does not block Phase 2 candidate iteration; a deeper read
would be needed before treating Symphony as full deep-dive parity
in the cluster catalogue (currently 6 deep-dive systems anchor the
catalogue; the 8th deep-dive system openclaw via cycle 43 deeper
read predates the catalogue's 6-system base).
