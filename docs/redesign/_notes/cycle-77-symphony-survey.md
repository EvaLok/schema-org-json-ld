# Cycle 77 — First-pass survey: `openai/symphony`

Primary target: <https://github.com/openai/symphony>  
Survey type: first-pass (repo + spec + impl-shape; not exhaustive deep code read)  
Date of read: 2026-05-08 UTC

Method notes:

- Read root docs (`README.md`, `SPEC.md`) and Elixir docs (`elixir/README.md`, `elixir/AGENTS.md`, `elixir/WORKFLOW.md`).
- Read implementation-shape files in `elixir/lib/` (application entrypoint, orchestrator, workflow/config, tracker adapters, workspace manager, agent runner, path-safety).
- Used GitHub API directory listings for structure/size checks.
- Where a claim comes from docs but was not fully verified in code at survey depth, it is labeled explicitly.

This deliverable follows the dispatch-required 12-lens structure in order: repo glance, spec architecture, Elixir architecture, spec-as-contract, workflow contract, tracker integration, workspace isolation, lifecycle/phasing, trust posture, 22-pattern cross-reference, Symphony-specific new patterns/hypothesis calls, and anchoring caveats.

---

## 1. Repo at a glance

### 1.1 Top-level structure and framing

The root structure matches the dispatch pre-sweep shape: `.codex/`, `.github/`, `README.md`, `SPEC.md`, and `elixir/` are present in repo-root listing (`openai/symphony` API contents dump, lines 2-113). `SPEC.md` is materially larger than `README.md` (size 80204 vs 1731), reinforcing that spec is the load-bearing artifact in this project shape (`/tmp/symphony/root-contents.json:67-91`).

The root README positions Symphony as organizational automation, not an interactive helper: “isolated, autonomous implementation runs” so teams “manage work instead of supervising coding agents” (`openai/symphony/README.md:3-4`). It also explicitly links Symphony to harness-engineering posture (`README.md:17-19`) and marks current release posture as preview/trusted-environment oriented (`README.md:10-12`).

### 1.2 Relationship to spec and implementation plurality

README “Option 1” invites reimplementation in any language and directly points implementors to `SPEC.md` (`README.md:21-27`). “Option 2” labels Elixir as an “experimental reference implementation” rather than sole canonical runtime (`README.md:28-35`). This is direct evidence that the repo surface is intentionally split into (a) language-agnostic contract and (b) one concrete implementation.

### 1.3 Relationship to Codex and Linear

Root README’s demo framing centers Codex-supervised work runs and Linear board polling (`README.md:8`). Elixir README gives the concrete execution path: poll Linear, create per-issue workspace, start Codex app-server in that workspace, send workflow prompt, keep running until done (`elixir/README.md:16-21`).

`.codex` is not incidental: root contains a `.codex/skills` directory and `.codex/worktree_init.sh` (`/tmp/symphony/codex-contents.json:3-33`), and the Elixir README calls out `../.codex/` as repository-local skills/setup helpers (`elixir/README.md:167-168`).

### 1.4 External project metadata snapshot

Current GitHub metadata (at survey read time) from repository search:

- Created: `2026-02-26T21:54:00Z`
- Language: `Elixir`
- Stars: `22513`
- Forks: `2097`
- Open issues count field: `5`  
  (search API output for `openai/symphony`, fetched during this survey).

### 1.5 Cluster anchoring from glance layer

- **Cluster D (documentation honesty):** high confidence present. README explicitly marks preview/trusted-environment caveat (`README.md:10-12`) and defers to a detailed spec contract (`README.md:25-27`).
- **Cluster F/G (org + role asymmetry):** present at framing level via “teams manage work” and explicit handoff framing in docs (`README.md:3-4`, `SPEC.md:41-42`).
- **Cluster I (policy boundary surface):** early signal present from trusted-vs-sandbox posture language (`SPEC.md:31-34`), to be validated against implementation in sections 7/9.

---

## 2. Architecture per the spec

### 2.1 Operational frame

Spec section 1 defines Symphony as a long-running automation daemon that reads tracker work, creates isolated per-issue workspace, and runs coding-agent sessions inside that workspace (`SPEC.md:18-20`). It explicitly sets boundary: scheduler/runner + tracker reader; ticket write logic typically sits with the agent/tooling layer (`SPEC.md:38-40`).

Spec goals in 2.1 provide an architecture-level requirements checklist: bounded concurrency polling, single authoritative in-memory orchestrator state, deterministic workspace lifecycle, state-change-driven stop behavior, retry/backoff, WORKFLOW.md-driven runtime contract, and restart recovery without persistent DB (`SPEC.md:48-56`).

### 2.2 Named components (spec 3.1) with short operational summaries

1. **Workflow Loader** — reads `WORKFLOW.md`, parses front matter + prompt body (`SPEC.md:73-77`).  
2. **Config Layer** — typed getters, defaults/env resolution, pre-dispatch validation (`SPEC.md:78-82`).  
3. **Issue Tracker Client** — candidate fetch, state refresh, terminal-state cleanup query, normalization (`SPEC.md:83-88`).  
4. **Orchestrator** — owns poll tick, in-memory runtime state, dispatch/retry/reconciliation decisions (`SPEC.md:89-94`).  
5. **Workspace Manager** — path mapping, directory guarantee, hooks, cleanup (`SPEC.md:95-100`).  
6. **Agent Runner** — workspace + prompt + app-server launch + updates to orchestrator (`SPEC.md:101-106`).  
7. **Status Surface (optional)** — human-readable runtime view (`SPEC.md:107-110`).  
8. **Logging** — structured runtime logs (`SPEC.md:111-113`).

### 2.3 State model and lifecycle primitives in spec

Spec explicitly distinguishes internal orchestration states from tracker workflow states (`SPEC.md:598-607`), then defines internal states (`Unclaimed`, `Claimed`, `Running`, `RetryQueued`, `Released`) and transition triggers (`Poll Tick`, worker exits, retry timer, reconciliation refresh, stall timeout) (`SPEC.md:608-687`).

Run lifecycle is named-phase (`PreparingWorkspace` → `BuildingPrompt` → `LaunchingAgentProcess` → `InitializingSession` → `StreamingTurn` → terminal outcomes) (`SPEC.md:639-654`), giving explicit phase-boundary vocabulary (Cluster A/C-relevant).

### 2.4 Storage/persistence posture from spec

Spec says authoritative scheduler state is in-memory (`SPEC.md:262-274`) while restart recovery is tracker/filesystem-driven without durable orchestrator DB (`SPEC.md:693-694`, `SPEC.md:55-56`). Workspace persistence is explicit: reused across runs; successful runs do not auto-delete (`SPEC.md:824-825`).

### 2.5 Spec-defined policy boundaries

Safety invariants are explicit and normative: run agent only at per-issue workspace path, keep workspace inside workspace root, sanitize workspace keys (`SPEC.md:890-905`). Approval/sandbox posture is intentionally implementation-defined but must be documented (`SPEC.md:1023-1031`), with explicit “must not stall indefinitely” requirements for approval/input events (`SPEC.md:1029-1031`, `SPEC.md:1091-1093`).

### 2.6 Cluster anchoring from spec layer

- **A/C:** strongly present via named transitions and phase lifecycle (`SPEC.md:639-687`).
- **B:** present via in-memory authority + filesystem/tracker recovery + persistent workspaces (`SPEC.md:262-274`, `SPEC.md:693-808`, `SPEC.md:824-825`).
- **D:** present via explicit goals/non-goals and normative contract semantics (`SPEC.md:44-67`, `SPEC.md:7-14`).
- **I:** present at design-contract level via workspace invariants + approval/sandbox policy requirements (`SPEC.md:890-905`, `SPEC.md:1023-1033`).

---

## 3. Architecture per the Elixir reference implementation

### 3.1 High-level module shape

Top-level application starts `Task.Supervisor`, `WorkflowStore`, `Orchestrator`, `HttpServer`, and `StatusDashboard` under one-for-one supervision (`elixir/lib/symphony_elixir.ex:26-39`). This directly realizes spec components 1/4/7/8 at process boundaries.

`elixir/lib` file-size shape indicates the operational center of gravity:

- `orchestrator.ex` (52KB)
- `status_dashboard.ex` (60KB)
- `codex/app_server.ex` (30KB)
- `workspace.ex` (14KB)
- `config/schema.ex` (16KB)
- `linear/client.ex` (16KB)  
  (`/tmp/symphony/elixir-lib-tree.tsv:11-33`).

At first-pass depth, this supports “spec contract + orchestrator-centric runtime” reading rather than thin wrapper reading.

### 3.2 Spec component → implementation map

- **Workflow Loader:** `SymphonyElixir.Workflow` parse/load path + `WorkflowStore` caching/reload (`workflow.ex:47-83`, `workflow_store.ex:1-4`, `workflow_store.ex:96-152`).
- **Config Layer:** `SymphonyElixir.Config` + `Config.Schema` typed embeds and semantic validation (`config.ex:29-99`, `config/schema.ex:264-289`).
- **Issue Tracker Client:** `SymphonyElixir.Tracker` boundary + `Linear.Adapter` behavior implementation + `Linear.Client` GraphQL transport/normalization (`tracker.ex:8-45`, `linear/adapter.ex:6-47`, `linear/client.ex:106-185`).
- **Orchestrator:** `SymphonyElixir.Orchestrator` GenServer with running/claimed/retry maps, tick/reconciliation/retry logic (`orchestrator.ex:24-42`, `orchestrator.ex:224-298`, `orchestrator.ex:773-897`).
- **Workspace Manager:** `SymphonyElixir.Workspace` create/validate/hooks/remove and path checks (`workspace.ex:13-25`, `workspace.ex:358-384`).
- **Agent Runner:** `SymphonyElixir.AgentRunner` workspace+hooks+Codex session+continuation loop (`agent_runner.ex:32-42`, `agent_runner.ex:79-131`).
- **Status Surface + Logging:** dashboard modules + docs contract (`elixir-lib-tree.tsv:27,44`, `elixir/docs/logging.md:11-40`).

### 3.3 BEAM/OTP substrate usage (H2 evidence)

BEAM-specific orchestration patterns are explicit:

- OTP app supervision entrypoint (`use Application`, child supervision tree) (`symphony_elixir.ex:20-39`).
- `GenServer` runtime state owner for orchestrator (`orchestrator.ex:6`, `orchestrator.ex:24-42`).
- `Task.Supervisor` for per-issue worker process spawning/termination (`symphony_elixir.ex:28`, `orchestrator.ex:694-699`, `orchestrator.ex:507-514`).
- `WorkflowStore` as its own `GenServer` for hot workflow reload + “last known good” semantics (`workflow_store.ex:6`, `workflow_store.ex:151-152`).
- Process-message communication between runner and orchestrator (`agent_runner.ex:55-58`, `agent_runner.ex:63-72`; consumed at `orchestrator.ex:166-201`).

Conclusion at survey depth: Elixir is used for OTP-native concurrency/supervision, not merely as “generic functional syntax.”

### 3.4 Spec↔impl convergence and visible drift

Convergences observed:

- Startup terminal workspace cleanup exists (`orchestrator.ex:67`, `orchestrator.ex:882-896`) matching spec startup cleanup (`SPEC.md:800-808`).
- Reconciliation stops terminal/non-active work and conditionally cleans workspace (`orchestrator.ex:347-366`, `orchestrator.ex:415-426`) matching spec state refresh behavior (`SPEC.md:795-797`).
- Workspace path safety includes root containment + symlink escape detection (`workspace.ex:358-379`, `path_safety.ex:4-15`) matching invariants (`SPEC.md:895-905`).

Potential drift / “deeper read needed”:

- Spec says dynamic reload is REQUIRED and SHOULD defensively revalidate during runtime ops (`SPEC.md:524-540`); implementation has poll-based workflow reload (`workflow_store.ex:83-93`, `workflow_store.ex:117-128`) and runtime config refresh hooks in orchestrator (`orchestrator.ex:76`, `orchestrator.ex:110`), but full parity across every config-sensitive path needs deeper proof.
- Spec names coding-agent protocol responsibilities with strict MUSTs (`SPEC.md:914-960`); first-pass read did not fully audit `codex/app_server.ex` against each MUST clause.

### 3.5 Cluster anchoring from impl layer

- **A/C:** GenServer-owned state + explicit transition handlers confirm lifecycle concretization.
- **B:** retry map + claimed/running maps + filesystem workspace persistence confirm stratified state+artifact model.
- **E:** typed config schema (`Ecto`) and tracker behavior callbacks show explicit typed boundary discipline.
- **I:** path validation, sandbox defaults, and controlled tool contract (`linear_graphql`) indicate enforcement surface beyond narrative docs.

---

## 4. Spec-as-contract discipline (H4 test)

### 4.1 Structural shape and completeness

`SPEC.md` is a full systems contract, not a short architecture note. It includes:

- problem framing + boundaries (`SPEC.md:16-42`)
- goals/non-goals (`SPEC.md:44-67`)
- component architecture (`SPEC.md:69-145`)
- domain model (`SPEC.md:146-288`)
- workflow/config contract (`SPEC.md:289-597`)
- orchestration state machine (`SPEC.md:598-695`)
- scheduling/reconciliation (`SPEC.md:696-809`)
- workspace safety invariants (`SPEC.md:810-905`)
- agent protocol responsibilities (`SPEC.md:906-1128`)
- tracker contract (`SPEC.md:1133-1211`)
- prompt/observability guidance (`SPEC.md:1212-1325`)

This breadth is operationally detailed enough to function as an implementation contract.

### 4.2 Normative language usage and separation from informational prose

Spec opens with explicit RFC 2119 interpretation clause (`SPEC.md:7-10`) and defines “implementation-defined” contract obligations (`SPEC.md:12-14`).

Normative requirements are scoped per subsystem rather than as a single checklist; examples:

- workflow parse validity (`SPEC.md:318`)
- dynamic reload obligations (`SPEC.md:526-540`)
- path safety invariants (`SPEC.md:895-899`)
- agent protocol conformance (`SPEC.md:914-919`)
- tracker adapter required ops (`SPEC.md:1137-1147`)

Informational background is separated via “Design note,” “Important boundary,” “Note,” and “Example high-trust behavior” sections (`SPEC.md:36-42`, `SPEC.md:307-311`, `SPEC.md:772-777`, `SPEC.md:1033-1038`).

### 4.3 Multi-implementation discipline

Signals are explicit and convergent:

- Spec status says “language-agnostic” (`SPEC.md:3`).
- README invites “Make your own” implementation in any language (`README.md:21-27`).
- README presents Elixir as reference implementation (`README.md:28-35`).
- Spec contains abstraction-level guidance for portability (`SPEC.md:114-137`).

This is stronger than “docs after implementation”; this is a contract-first portability posture.

### 4.4 Goals/non-goals discipline and anti-scope clarity

Spec has explicit Non-Goals that actively constrain expected interpretation:

- not a rich control plane or distributed scheduler (`SPEC.md:60-63`)
- no built-in business logic for ticket editing (`SPEC.md:63-64`)
- no single mandated sandbox/approval posture (`SPEC.md:65-67`)

That is documentation-as-boundary, not just feature list.

### 4.5 Spec→impl binding evidence at first-pass depth

Direct alignments:

- `WORKFLOW.md` as repo-owned runtime contract: spec (`SPEC.md:53`, `SPEC.md:289-311`) and impl loader/store (`workflow.ex:1-4`, `workflow_store.ex:1-4`).
- authoritative in-memory orchestrator state: spec (`SPEC.md:262-274`) and impl state struct (`orchestrator.ex:29-42`).
- tracker/filesystem restart recovery: spec (`SPEC.md:55-56`, `SPEC.md:693-695`) and startup terminal cleanup in impl (`orchestrator.ex:67`, `orchestrator.ex:882-896`).

### 4.6 H4 verdict

**H4 CONFIRMED (strong).**  
All required conditions are met:

- operationally detailed spec (80KB at repo root metadata; `SPEC.md` size 80204 in `root-contents.json:83-87`)
- explicit language-agnostic/multi-impl discipline (`SPEC.md:3`, `README.md:21-35`)
- explicit RFC 2119 normative contract (`SPEC.md:7-14`)

At survey depth, Symphony appears to add a distinct “spec-as-contract” pattern not previously dominant in the earlier Phase 1 deep-dive set.

---

## 5. Workflow contract pattern (`WORKFLOW.md`)

### 5.1 Role of `WORKFLOW.md`

In spec, `WORKFLOW.md` is the repository contract for both runtime settings and prompt body (`SPEC.md:289-325`). In impl, `Workflow` parses front matter and prompt template (`workflow.ex:63-75`), while `WorkflowStore` continuously reloads changes and keeps last known good config (`workflow_store.ex:1-4`, `workflow_store.ex:150-152`).

This is not “just a prompt file”; it is dual-purpose policy/config artifact.

### 5.2 Loading and mutation discipline

Spec requires dynamic reload without service restart (`SPEC.md:524-540`). Implementation runs polling reload every second (`workflow_store.ex:11`, `workflow_store.ex:83-94`) and on parse/read failure logs error while keeping prior effective config (`workflow_store.ex:151-152`).

Elixir README documents same behavior: invalid startup config blocks boot; invalid later reload preserves last-known-good (`elixir/README.md:148-150`).

### 5.3 Workflow body as operational playbook (reference implementation)

`elixir/WORKFLOW.md` is effectively an in-repo operations contract with:

- state routing rules (`Todo`, `In Progress`, `Human Review`, `Merging`, `Rework`, `Done`) (`WORKFLOW.md:106-127`)
- completion bar gating transition to `Human Review` (`WORKFLOW.md:262-270`)
- guardrails on branch reuse, single workpad, and blocking conditions (`WORKFLOW.md:274-290`)

This goes beyond generic prompt engineering; it is process policy encoded in source-controlled workflow artifact.

### 5.4 Cluster anchoring

- **A/C:** named state-routing + transition gates in workflow doc (`WORKFLOW.md:106-127`, `262-270`).
- **D:** documentation-as-policy enforcement is explicit here; rules are imperative and checklist-gated.
- **F/G:** role-asymmetric flow: coding agent execution in some states vs wait/poll behavior in `Human Review`.

---

## 6. Issue-tracker integration discipline

### 6.1 Spec stance

Spec v1 names Linear as current tracker kind while framing adapter contract in generic required operations (`SPEC.md:1137-1147`, `SPEC.md:1150-1168`). That is “Linear-now, adapter-shape intended.”

### 6.2 Implementation shape: adapter boundary exists

`SymphonyElixir.Tracker` defines callback contract for candidate fetch, state refresh, and writes (`tracker.ex:8-12`) and delegates via `adapter()` selected by config kind (`tracker.ex:40-44`).

Available adapters at first-pass depth:

- `Linear.Adapter` (default) (`tracker.ex:43`, `linear/adapter.ex:1-8`)
- `Tracker.Memory` for tests/local dev (`tracker.ex:42`, `tracker/memory.ex:1-7`)

This is real interface-based indirection, not only direct Linear calls.

### 6.3 Linear specificity still significant

`Linear.Client` embeds Linear GraphQL query semantics directly (project slug filter, issue schema fields, viewer query) (`linear/client.ex:13-55`, `linear/client.ex:98-104`). Semantic errors are named as linear-specific categories (`linear_api_status`, `linear_api_request`) (`linear/client.ex:175-184`), and config semantic validation enforces Linear requirements when kind is `linear` (`config.ex:125-130`).

Interpretation: tracker abstraction exists, but production path is still Linear-first.

### 6.4 Pluggability assessment

Could Jira/GitHub Issues be plugged in? **At contract level:** yes (spec adapter requirements are generic enough). **At current Elixir runtime:** partial; tracker boundary exists, but non-Linear implementation would require a new adapter + config semantics + normalization parity. So this is “architecturally prepared, not turnkey.”

### 6.5 Cluster anchoring

- **E:** strong typed boundary via behavior callbacks + normalized issue model.
- **F/I intersection:** policy gating still tied to tracker states; enforcement uses normalized state checks in orchestrator (`orchestrator.ex:632-658`), showing tracker-state as control input to execution policy.

---

## 7. Workspace isolation pattern

### 7.1 Spec contract

Per-issue path shape and persistence are explicit (`SPEC.md:818-825`). Safety invariants require strict cwd/workspace-root containment and sanitized workspace key (`SPEC.md:890-905`).

### 7.2 Implementation mechanics

Workspace identifier is sanitized to `[A-Za-z0-9._-]` via substitution (`workspace.ex:206-208`), then joined under configured root (`workspace.ex:196-200`). Path validation canonicalizes paths and rejects root-equal, outside-root, and symlink-escape cases (`workspace.ex:358-379`), with symlink-aware canonicalization implemented in `PathSafety` (`path_safety.ex:4-15`, `path_safety.ex:28-34`).

### 7.3 Lifecycle

- Create/reuse workspace with `created?` signal (`workspace.ex:21-24`, `workspace.ex:34-45`).

- `after_create` hook runs only for newly created workspace (`workspace.ex:210-225`).

- Per-attempt hooks (`before_run`, `after_run`) are explicit (`workspace.ex:166-193`).

- Cleanup supports targeted issue workspace removal and startup terminal cleanup integration via orchestrator (`workspace.ex:130-160`, `orchestrator.ex:882-896`).

### 7.4 Remote worker path

Workspace manager supports remote workers over SSH (`workspace.ex:48-79`, `workspace.ex:435-449`) and uses marker-based output parsing for remote path creation (`workspace.ex:412-433`). At first-pass depth this indicates isolation semantics are intended across local and remote hosts, not local-only.

### 7.5 Default-deny/policy interpretation

Config default sandbox policy (when not overridden) is workspace-write with writable roots restricted to workspace and `networkAccess: false` (`config/schema.ex:482-490`). This is an implementation-level deny-by-default network posture in default runtime policy construction.

### 7.6 Cluster anchoring

- **B:** workspace-as-artifact lifecycle and persistence.
- **I:** policy enforcement through path constraints + sandbox default policy.
- **H:** issue-linked workspace survives runs and is cleaned on terminal states.

---

## 8. State machine / lifecycle / phasing

### 8.1 Named phases and transition triggers (spec)

Spec provides explicit orchestration states and transition triggers (`SPEC.md:608-687`), with poll tick sequencing (`SPEC.md:705-713`) and reconciliation behavior (`SPEC.md:791-799`).

### 8.2 Concrete orchestrator behavior (impl)

Orchestrator tick flow:

1. Refresh runtime config (`orchestrator.ex:76`, `orchestrator.ex:110`).
2. Reconcile running issues (`orchestrator.ex:225-226`, `orchestrator.ex:275-297`).
3. Validate config and fetch candidates (`orchestrator.ex:227-230`).
4. Dispatch eligible work (`orchestrator.ex:519-531`, `orchestrator.ex:660-691`).
5. Manage retries with continuation/failure backoff (`orchestrator.ex:773-810`, `orchestrator.ex:928-939`).

### 8.3 Termination predicates and ineligible-state handling

Implementation terminates active runs when issue enters terminal state, non-active state, or becomes unroutable (`orchestrator.ex:347-366`). Terminal transitions optionally trigger workspace cleanup (`orchestrator.ex:415-426`), while non-active/non-terminal transitions terminate without cleanup (`orchestrator.ex:363-365`, `orchestrator.ex:795-797` at spec level).

### 8.4 Handoff state (`Human Review`)

Spec allows successful run to end at workflow-defined handoff state (example `Human Review`) (`SPEC.md:41-42`). The reference `WORKFLOW.md` operationalizes this state with strict transition rules and a “do not code” posture while waiting (`WORKFLOW.md:110`, `WORKFLOW.md:241-247`).

### 8.5 Super-step / continuation semantics

Spec continuation model says worker may continue multiple turns on same live thread while issue remains active (`SPEC.md:628-634`). Agent runner concretely does this up to `agent.max_turns`, using continuation guidance rather than resending full prompt (`agent_runner.ex:92-145`).

### 8.6 Cluster anchoring

- **A:** explicit phase boundaries and continuation loop control.
- **C:** rich lifecycle states and transition predicates.
- **A↔C intersection:** strong, with tracker-state refresh triggering stop/continue outcomes.

---

## 9. Trust posture plurality

### 9.1 Spec-level plurality

Spec explicitly refuses a universal trust policy and requires each implementation to document chosen posture (`SPEC.md:31-34`, `SPEC.md:1023-1031`). Non-goals also reject mandating one approval/sandbox posture (`SPEC.md:65-67`).

### 9.2 Elixir reference posture

Elixir README labels implementation as prototype for evaluation and recommends hardened own implementation (`elixir/README.md:6-9`). It also documents default safety settings:

- reject-style approval policy default
- `thread_sandbox` default `workspace-write`
- `turn_sandbox_policy` default rooted to issue workspace  
  (`elixir/README.md:113-117`, mirrored in config defaults at `config/schema.ex:162-173`, `config/schema.ex:482-490`).

### 9.3 High-trust and sandbox coexistence

Root README warning says “testing in trusted environments” (`README.md:10-12`), while technical defaults in Elixir include workspace-root confinement and network-off turn sandbox policy when using default runtime policy builder (`config/schema.ex:482-490`). This is not purely “dangerous by default”; it is a mixed posture: trusted deployment context + bounded technical defaults + configurable pass-through to Codex schema.

### 9.4 Default-deny and capability-tier notes

Observed default-deny style:

- workspace-root enforcement (`workspace.ex:358-379`)
- path sanitization (`workspace.ex:206-208`)
- networkAccess false in default turn policy (`config/schema.ex:487`)
- strict workflow parse/validation gating dispatch (`orchestrator.ex:227-268`, `SPEC.md:542-557`)

Capability-tier × role enforcement at first-pass depth: **partial/weakly explicit**. Strongly explicit role states exist in workflow (`WORKFLOW.md:106-127`), but hard role-based capability matrix in runtime code was not fully confirmed in this pass. Deeper read is needed for definitive claim.

### 9.5 H3 verdict (cluster I substrate-correlation test)

**H3 CONFIRMED (moderate-to-strong).** At least 3 cluster-I sub-shapes are clearly visible:

1. Workspace isolation as policy enforcement (`SPEC.md:890-905`, `workspace.ex:358-379`).
2. Tracker-state-driven stop/start gating (`SPEC.md:51`, `SPEC.md:795-797`, `orchestrator.ex:347-366`).
3. Trust posture plurality with implementation-defined approval/sandbox choices (`SPEC.md:31-34`, `SPEC.md:1023-1031`; Elixir docs/config defaults).

This supports the substrate-correlation strengthening hypothesis (third convergent case after openclaw + OpenAI harness writeup), while still leaving room for deeper-read refinement.

---

## 10. Cross-reference to oh-my-codex 22 patterns

Baseline names/source: cycle-76 supplement comment on #2847 (comment 4383627221), which preserves cycle-26 pattern names verbatim.  
Source URL: <https://github.com/EvaLok/schema-org-json-ld/issues/2847#issuecomment-4383627221>

Legend: **PARALLEL / ADAPTED / ABSENT / NOT-COMPARABLE / NEW**

1. **Workflow as named keyword keywords with transition policy** (verbatim cycle-26 name) → **ADAPTED**. Symphony has explicit workflow state routing and transition policy in `WORKFLOW.md`, but not keyword-triggered mode switching (`WORKFLOW.md:106-127`).
2. **Context snapshot grounding before execution** → **ADAPTED**. Symphony uses persistent workpad bootstrap/update discipline as execution context surface (`WORKFLOW.md:133-155`, `WORKFLOW.md:278-326`), but not `.omx/context` snapshot files.
3. **Explicit stop conditions with named escalation paths** → **PARALLEL**. Symphony has explicit blockers/escape hatch and stop/wait states (`WORKFLOW.md:67`, `WORKFLOW.md:184-195`, `WORKFLOW.md:243-248`).
4. **Evidence-backed completion, not assertion-backed** → **PARALLEL**. Completion bar requires validation/checks green and feedback sweep complete before `Human Review` (`WORKFLOW.md:227-233`, `WORKFLOW.md:262-270`).
5. **Pre-execution gate for underspecified requests** → **NOT-COMPARABLE**. Symphony consumes tracker issues and workflow policy; no direct equivalent of freeform user-prompt underspecification gate.
6. **Iteration limits with explicit ceiling** → **PARALLEL**. `agent.max_turns` and retry backoff ceilings are explicit (`WORKFLOW.md:30`, `SPEC.md:415-421`, `orchestrator.ex:928-939`).
7. **Behavioral prompt contract with test coverage** → **ADAPTED**. Symphony has strong contract docs (`SPEC.md` + `WORKFLOW.md`) and quality gate references (`elixir/AGENTS.md:9`, `elixir/AGENTS.md:29-33`) but this survey did not verify prompt-contract regression test suite parity in depth.
8. **Parallel delegation over sequential** → **PARALLEL**. Core architecture is bounded concurrent multi-issue dispatch (`SPEC.md:48`, `SPEC.md:711`, `orchestrator.ex:740-747`).
9. **Session scope and root scope for state, with explicit reconciliation** → **PARALLEL**. In-memory orchestrator state + per-issue running/retry maps + periodic reconciliation (`orchestrator.ex:29-42`, `orchestrator.ex:275-297`).
10. **File-backed migration with one-way compatibility windows** → **ABSENT** (at survey depth). Did not find explicit one-way migration window mechanism analogous to pattern 10.
11. **Document-refresh warning integrated into commit path** → **ABSENT** (at survey depth). No equivalent pre-commit doc-refresh warning mechanism observed.
12. **Autoresearch as a bounded supervisor/candidate loop** → **NOT-COMPARABLE**. Symphony target is issue-run orchestration, not exploration/research loop.
13. **Deslop pass as a mandatory post-completion step** → **ABSENT**. No explicit mandatory “deslop” stage found in spec/workflow.
14. **Wiki as persistent, searchable local knowledge base** → **ABSENT**. No in-repo persistent wiki subsystem observed in surveyed files.
15. **MCP transport failure has a defined fallback** → **ADAPTED**. Workflow has blocked-access/fallback behavior expectations (`WORKFLOW.md:184-195`), but not same MCP transport fallback architecture.
16. **Explicit false-green detection for install readiness** → **ADAPTED**. Elixir docs differentiate normal checks vs live e2e path (`elixir/README.md:170-204`), but no direct “doctor vs smoke” split identical to pattern 16.
17. **Autonomy directive as first line of orchestration doc** → **PARALLEL**. Workflow instructions enforce unattended autonomous execution posture (`WORKFLOW.md:66`, `WORKFLOW.md:93`).
18. **Triage advisory routing without keyword activation** → **NOT-COMPARABLE**. Symphony routes by tracker state machine, not freeform prompt triage.
19. **Coverage gate for critical modules** → **ADAPTED**. Elixir AGENTS enforces `make all` quality gate and required specs for `def` (`elixir/AGENTS.md:9`, `elixir/AGENTS.md:37-40`), but no direct “critical-module threshold gate” analogue confirmed.
20. **Compatibility layer separate from authoritative state** → **PARALLEL**. Spec and implementation distinguish authoritative in-memory orchestrator state from tracker/workspace artifacts (`SPEC.md:262-274`, `SPEC.md:693-695`).
21. **Deterministic keyword detection over heuristic prompt parsing** → **NOT-COMPARABLE**. Symphony control plane is tracker+workflow-driven, not keyword-triggered workflow activation.
22. **Lore-format commit signing for agent-authored commits** → **ABSENT** (at survey depth). No equivalent commit-trailer enforcement observed in surveyed files.

Cross-reference summary:

- PARALLEL: 7 (3,4,6,8,9,17,20)
- ADAPTED: 6 (1,2,7,15,16,19)
- ABSENT: 6 (10,11,13,14,22 + one survey-depth absent call)
- NOT-COMPARABLE: 4 (5,12,18,21)

Interpretation: Symphony is not “same-author architecture echo”; many oh-my-codex patterns are absent/not-comparable due to different substrate and organizational goal. Parallel overlap appears mostly in lifecycle/quality/autonomy primitives, not in keyword-mode UX patterns.

---

## 11. NEW patterns specific to Symphony + hypothesis verdicts

### 11.1 New patterns not captured by cycle-26 catalog

1. **Spec-first language-agnostic contract as primary artifact**  
   Evidence: `SPEC.md` status/language-agnostic + RFC2119 framing + README “make your own” call (`SPEC.md:3`, `SPEC.md:7-14`, `README.md:21-27`).  
   Why new: stronger explicit cross-implementation contract than prior cataloged keyword/hook patterns.

2. **Dual-contract architecture: global spec + repo-local workflow contract**  
   Evidence: `SPEC.md` defines portable service contract; `WORKFLOW.md` carries per-deployment policy+prompt runtime (`SPEC.md:289-325`, `workflow.ex:63-75`, `WORKFLOW.md:1-37`).

3. **Tracker/filesystem restart recovery without orchestrator DB**  
   Evidence: explicit goal + state machine rules + startup cleanup path (`SPEC.md:55-56`, `SPEC.md:693-695`, `orchestrator.ex:882-896`).

4. **Workspace safety as hard runtime invariant with symlink-aware canonicalization**  
   Evidence: path containment + symlink-escape detection (`workspace.ex:358-379`, `path_safety.ex:28-34`).

5. **Live workflow reload with last-known-good retention as reliability primitive**  
   Evidence: spec requires no-crash invalid reload semantics (`SPEC.md:539-540`); workflow store logs reload failure and retains current config (`workflow_store.ex:151-152`).

6. **Continuation on same live Codex thread across multiple turns under explicit cap**  
   Evidence: spec continuation semantics (`SPEC.md:630-634`); runner implementation (`agent_runner.ex:92-145`).

### 11.2 H1 (organizational-substrate vocabulary distinct from solo-author) verdict

**H1 CONFIRMED (5/5).**

Required checks:

- spec-driven design artifact distinct from README ✅ (`root-contents.json:83-90`, `README.md:21-35`)
- language-agnostic/spec-first stance ✅ (`SPEC.md:3`, `README.md:23-27`)
- RFC 2119 normative vocabulary ✅ (`SPEC.md:7-10`)
- explicit goals/non-goals split ✅ (`SPEC.md:44-67`)
- implementation-defined plurality (trust posture etc.) ✅ (`SPEC.md:12-14`, `SPEC.md:31-34`, `SPEC.md:1023-1031`)

### 11.3 H2 (Elixir/BEAM substrate exploitation) verdict

**H2 CONFIRMED (strong).**

Observed BEAM-specific patterns exceed threshold:

- OTP supervision tree (`symphony_elixir.ex:26-39`)
- GenServer orchestration owner (`orchestrator.ex:6`, `orchestrator.ex:24-42`)
- Task.Supervisor worker process lifecycle (`symphony_elixir.ex:28`, `orchestrator.ex:694-699`)
- process message passing for runtime updates (`agent_runner.ex:55-72`, `orchestrator.ex:166-201`)
- typed config layering in dedicated schema module (`config/schema.ex:264-289`)

### 11.4 H3 (cluster I substrate-correlation prediction) verdict

**H3 CONFIRMED** (see section 9.5).  
Cluster-I sub-shapes are visible in both spec contract and implementation controls.

### 11.5 H4 (spec-as-contract) verdict

**H4 CONFIRMED** (see section 4.6).  
Symphony provides a clear spec-as-contract pattern with language-agnostic portability intent and normative requirement surface.

### 11.6 Cluster augmentation call

Symphony appears to augment:

- **Cluster A:** explicit phase and continuation boundary semantics.
- **Cluster B:** layered persistence model (in-memory authority + filesystem artifacts + tracker truth).
- **Cluster C:** lifecycle states and transition triggers with reconciliation semantics.
- **Cluster D:** documentation-as-policy strongly realized in `SPEC.md` + `WORKFLOW.md`.
- **Cluster E:** typed boundary enforcement (`Ecto` schema + behavior contracts).
- **Cluster F/G:** role/state-asymmetric operational contract in workflow states.
- **Cluster H:** workspace and PR/check artifacts treated as proof-of-work channels (documented in README/workflow).
- **Cluster I:** policy boundary instrumentation around workspace, sandbox, and tracker-state gating.

---

## 12. Anchoring caveats (transferability vs discounting)

### 12.1 Multi-issue orchestration target vs single-repo redesign target

Difference:

- Symphony is built for concurrent multi-issue automation (`SPEC.md:48`, `SPEC.md:711`).
- Redesign context is one repo with orchestrator cycles.

Discount:

- Raw concurrency-slot policies are not directly transferable as-is.

Transfer:

- Named lifecycle states, explicit retry semantics, and reconciliation-before-dispatch pattern are transferable architecture primitives.

### 12.2 Linear dependency vs GitHub Issues substrate

Difference:

- Symphony v1 spec is Linear-specific in tracker kind contract (`SPEC.md:1150-1156`).

Discount:

- Linear query details and state labels are not directly reusable.

Transfer:

- tracker adapter boundary contract shape (`fetch_candidate_issues`, `fetch_issues_by_states`, `fetch_issue_states_by_ids`) is transferable (`SPEC.md:1137-1147`, `tracker.ex:8-27`).

### 12.3 Trust posture plurality vs fixed CI substrate

Difference:

- Symphony intentionally supports implementation-defined trust posture (`SPEC.md:31-34`, `SPEC.md:1023-1031`).
- Redesign environment has tighter predetermined CI/security boundaries.

Discount:

- Full approval/sandbox knobs may not map 1:1.

Transfer:

- Requirement to document and enforce policy boundaries explicitly remains highly transferable (Cluster D↔I style).

### 12.4 Elixir harness vs “agent-as-substrate” redesign context

Difference:

- Symphony runtime is explicit BEAM/OTP harness.
- Redesign context is largely prompt/tool orchestration on GitHub Actions substrate.

Discount:

- OTP process primitives cannot be copied directly.

Transfer:

- conceptual equivalents (single authoritative state owner, bounded worker lifecycle, explicit retry timer ownership) still transfer.

### 12.5 Team/org operational frame vs single primary human stakeholder

Difference:

- Symphony is explicitly team operational frame with `Human Review`/`Merging` lifecycle (`WORKFLOW.md:110-112`, `WORKFLOW.md:241-249`).

Discount:

- some multi-actor queue-state conventions may overfit team environments.

Transfer:

- explicit handoff states and completion bars reduce ambiguity even in single-stakeholder systems.

### 12.6 Normative spec vs descriptive prompt style

Difference:

- Symphony uses RFC2119 normative spec language (`SPEC.md:7-10`).
- Redesign prompt tradition is more descriptive.

Discount:

- not every redesign artifact should become standards-language heavy.

Transfer:

- selective normative clauses for safety-critical/process-critical behavior appear highly beneficial (clearer policy intent, lower drift risk).

### 12.7 `Human Review` handoff vs question-for-eva blocking semantics

Difference:

- Symphony models `Human Review` as normal handoff state (`SPEC.md:41-42`, `WORKFLOW.md:110`).
- redesign’s question-for-eva is primarily blocker/escalation mechanism.

Discount:

- semantic equivalence should not be assumed.

Transfer:

- explicit handoff-state modeling is still useful as a design option to reduce “stuck between done and merged” ambiguity.

---

## Survey-level caveats / deeper-read queue

Items intentionally deferred (first-pass scope boundary):

1. Full clause-by-clause conformance audit of `codex/app_server.ex` against all `SPEC.md` protocol MUST clauses (including the dynamic-reload parity caveat flagged in section 3.4).
2. Full audit of `status_dashboard.ex` and observability API against spec’s optional monitoring interface guidance.
3. Exhaustive tracker abstraction proof for non-Linear implementations beyond current `memory` adapter.
4. Full cross-check of every RFC2119 clause against implementation behavior; this survey confirms many representative clauses, not complete formal conformance.

These are appropriate deeper-read tasks if/when Symphony moves from survey to deep-dive stage.
