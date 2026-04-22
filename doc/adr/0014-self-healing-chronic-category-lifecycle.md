# ADR 0014: Self-Healing Chronic-Category Lifecycle

Date: 2026-04-23

## Status

Accepted

## Context

Since ADR 0013 reclassified `chronic-category-currency` and `deferral-accumulation` as non-blocking at the C5.5 close-out gate, those detectors still fire with FAIL or WARN status and still block through other paths — most importantly, `dispatch-task`'s own pipeline-check invocation (the `record-dispatch::enforce_pipeline_gate` call) continues to refuse new Copilot dispatches when `chronic-category-currency` reports FAIL. This manifested as issue #2542 (cycle 499) and its re-pose #2622 (cycle 521).

The partial fix from #2542 (in-flight dispatch downgrade, `pipeline-check/src/main.rs:4852-4864`) resolves the deadlock only when a category already has an active agent session tracking it. It does not resolve the three failure modes we have repeatedly observed:

1. **Bookkeeping drift.** A structural fix for a chronic category lands (PR merges), but the orchestrator does not call `process-review --update-chronic-category` to bump `verification_cycle`. The detector continues to report the entry as stale even though the underlying defect has been fixed. Current example: `review-evidence` (verification_cycle 466, 62 cycles stale, corresponding structural fix `verify-review-events` shipped long ago).

2. **Malformed entries.** A chronic entry is created with `verification_cycle: 0` and empty `root_cause`, representing no real fix commitment. The detector treats these as legitimately stale and blocks dispatch indefinitely. Current example: `worklog-accuracy/scope-boundary` (vc=0, empty root_cause, updated_cycle 510, currently 528 cycles stale).

3. **Too-broad categories.** A category is so broadly scoped that no single structural dispatch can converge it. Repeated retry dispatches produce no closure. Current example: `code-quality` (verification_cycle 490, 38 cycles stale, root cause spans multiple tools and test shapes).

Each failure mode requires a form of intervention that the current system lacks a tool for, and all three have historically required `question-for-eva` issues that accumulate with slow response time. This conflicts with the stated design goal (see `feedback_system_purpose_framing`): the orchestrator system should self-improve autonomously, with Eva acting as a last-resort safety valve rather than a first responder.

Additional hard constraints:

- `review_agent.chronic_category_responses.entries[]` is append-only. Existing entries must never be mutated. See `feedback_chronic_entries_append_only`.
- The orchestrator cannot modify its own system prompt; prompt changes require Eva's direct action (`orchestrator-prompt-editing` skill).

Any fix that depends on editing existing entries, or on new orchestrator behaviors that the current system prompt does not describe, cannot be implemented by the orchestrator alone.

## Decision

Introduce a **self-healing chronic-category lifecycle** with five components, implemented directly by Eva (not dispatched) because the current orchestrator system prompt does not describe these behaviors:

1. **Event-sourced state for chronic categories.** Add four append-only sibling collections under `review_agent.chronic_category_responses`: `verification_events[]`, `validity_observations[]`, `recalibration_proposals[]`, and `recalibration_executions[]`. Existing `entries[]` remain frozen; their historical `verification_cycle` values are superseded by the latest matching record in `verification_events[]`. Pipeline-check computes current state by replaying the logs.

2. **Evidence-based verification.** When a merged PR's dispatch has `--addresses-finding <N:M>` matching a chronic category, `process-merge` appends a `verification_events[]` record. The `chronic-category-currency` detector reads this log to compute staleness rather than relying on a manually-set `verification_cycle`. Bookkeeping drift becomes structurally impossible.

3. **Validity detection.** A new pipeline-check phase `chronic-entry-validity` flags structurally-malformed entries (vc=0 with stale updated_cycle, empty root_cause, no cited PRs, no `addresses_finding` coverage anywhere in history). Flagged entries are routed into the recalibration path and stop contributing to `chronic-category-currency` FAIL computation.

4. **Recalibration agent with audit gating.** A new standalone agent (distinct from the review agent and the audit agent) accepts recalibration tasks triggered when a category is chronic-stuck for M cycles with no in-flight fix and no active validity observation. The agent analyzes the category's cross-cycle history and returns exactly one of: `split`, `redispatch`, `retire`, `escalate`. The audit agent reviews the proposal; its disposition (`approve` / `concerns` / `reject` / silent→SLA) determines whether the orchestrator executes.

5. **Bounded Eva escalation.** When audit rejects the same-shape proposal across M=5 cycles, when audit remains silent past N=3 cycles (SLA), or when the recalibration agent explicitly escalates, the orchestrator auto-files a `question-for-eva` issue with a standardized body (proposal link, audit disposition history, structured decision menu). A Phase 5 detector upgrade distinguishes answered-awaiting-close from unanswered so the blocker list reflects real need, not stale OPEN state.

### Rationale

**Why event-sourcing.** Append-only is a hard constraint. Any design that "refreshes" or "updates" existing entries breaks the ledger property. Event-sourcing is the natural fit: declarations, observations, proposals, and executions are all immutable records; current state is computed by reduction. This matches the existing pattern in `agent_sessions`, `dispatch_log_latest`, and `review_agent.history`.

**Why a new recalibration agent rather than extending the review agent.** Cycle review (instance-level, per-cycle) and recalibration (meta-level, cross-cycle) have different judgment shapes. Extending would couple recalibration reliability to the review agent's known limitations around `journal-quality` grading. A separate agent matches the already-established pattern of per-concern agents (review + audit). Build cost is higher but correctness per the "do things right" directive is the priority given how long the system has been stuck.

**Why gating audit rather than advisory.** Advisory would push disagreement-resolution onto the orchestrator, which has no reliable basis to adjudicate review-vs-audit disputes. Gating collapses the orchestrator's role to deterministic execution of ratified proposals. The SLA timeout prevents indefinite stall, converting hard deadlock into scheduled escalation.

**Why fixed N=20 retirement bar rather than per-category.** We lack the data to set per-category bars defensibly today. Fixed threshold is auditable, gameproof, and migration-friendly — per-category can be introduced later as an optional override with real data on how retirements behaved in practice. N=20 sits slightly past the existing FAIL threshold (>17 cycles stale), providing a small buffer before declaring a category dormant.

**Why separate escalation issues rather than a running log.** The existing `question-for-eva` lifecycle works when used correctly; the "10 stale blockers" problem is a detector bug (OPEN-state keying), not a structural issue. A running log would hide that bug rather than fix it. Separate issues preserve per-question acknowledgement, referentiality from other artifacts, and the existing close-after-ack convention. Phase 5's detector upgrade addresses the visibility bug directly.

### Rejected alternatives

- **Unified `chronic_events[]` collection with typed `kind` field** instead of four separate ledgers. Considered and rejected: four collections with fixed schemas are easier to validate with state-invariants and easier to read in state.json. A unified collection with runtime type dispatch pushes validation complexity into tool code.

- **Retrofit tool that populates new fields on existing entries.** Rejected because it violates the append-only rule. Addressed by making event-sourced state fully backward-compatible — legacy entries are read as-is; their historical fields are frozen and superseded by events going forward.

- **Extending the review agent with a recalibration task type.** Considered and rejected. See rationale above.

- **Advisory audit role.** Considered and rejected. See rationale above.

- **Per-category retirement thresholds.** Considered and rejected for day-one. May be revisited as an additive extension after fixed-N experience.

- **Running log issue for Eva escalations.** Considered and rejected. See rationale above.

- **Auto-escalation to Eva as the primary unblock mechanism** (Layer C in the pre-decision analysis). Rejected because it contradicts the minimize-Eva-loop directive. Escalation remains as the last-resort safety valve under Layer 5 only.

## Implementation Phases

Implementation proceeds in five phases. Phases 1, 2, and partial 3 constitute the **minimum viable self-healing milestone** — at that point, two of three currently-stuck entries resolve autonomously and the third has a structured proposal ready for execution.

### Phase 1 — Evidence-based verification

**Deliverables**:
- Add `verification_events[]` append-only collection to `state_schema`.
- In `process-merge` (and/or `record-dispatch`): when a merged PR's dispatch has `--addresses-finding <N:M>` and the finding's category matches an active chronic entry, append a `verification_events[]` record with the PR number, merge timestamp, and cycle.
- In `pipeline-check::chronic_category_currency_status`: compute verification staleness from `verification_events[]` first, falling back to the entry's historical `verification_cycle` only when no event exists for that category.
- State invariants: every `verification_events[]` record must reference a merged PR; records are append-only (no supersession except via later events for the same category).

**Exit criteria**: `review-evidence` entry is automatically refreshed the next time a PR addressing `review-evidence` merges.

### Phase 2 — Validity detection

**Deliverables**:
- New pipeline-check phase `chronic-entry-validity`. Invalidity rules:
  - `verification_cycle == 0` AND `updated_cycle < current_cycle - 20`
  - `root_cause` is empty or trivially-short AND `chosen_path == "structural-fix"`
  - No PR citations in `rationale` AND gap > FAIL threshold AND no in-flight dispatch tracking the category
- Add `validity_observations[]` append-only collection. Pipeline-check appends observations when invalidity detected; observations persist until a superseding observation is appended.
- In `chronic_category_currency_status`: exclude entries with an active invalidity observation from the FAIL computation.
- Pipeline-check output surfaces invalid entries separately from stale ones.

**Exit criteria**: `worklog-accuracy/scope-boundary` is flagged INVALID, stops contributing to gate FAIL, and is visibly queued for recalibration.

### Phase 3 — Recalibration infrastructure

Phase 3 is decomposable into independently-shippable sub-phases.

**Phase 3a — State schema**:
- Add `recalibration_proposals[]` and `recalibration_executions[]` append-only collections.
- Proposal fields: `cycle`, `category`, `action` (`split` | `redispatch` | `retire` | `escalate`), proposal artifact path, audit_disposition (initially `pending`), SLA_expires_cycle.
- Execution fields: `cycle`, `proposal_ref`, `executed_action`, receipts.
- State invariants: every execution references an `approved` proposal; no execution without audit approval; SLA expiry fires at the specified cycle.

**Phase 3b — Orchestrator tool: `propose-recalibration`**:
- Takes `--category <NAME>`, creates a `recalibration-task`-labeled GitHub issue with the category's full history, and appends to `recalibration_proposals[]` with status `pending`.
- Invoked automatically by pipeline-check when a category has been stuck past threshold for M cycles with no in-flight dispatch and no validity observation.

**Phase 3c — Recalibration agent**:
- New system prompt defining the four response types, evidence requirements for each, and output format (`docs/recalibrations/cycle-N-category.md` with YAML frontmatter).
- GitHub Action workflow that fires on `recalibration-task` issues and produces a PR with the proposal artifact.
- Retirement proposals must cite reason code (`dormant` or `malformed`) and corresponding evidence per Decision 4.

**Phase 3d — Orchestrator execution verbs**:
- `execute-recalibration split --parent CATEGORY --children A,B,C`: appends new chronic entries for each child with explicit parent reference; appends `recalibration_executions[]` row.
- `execute-recalibration retire --category CATEGORY --reason dormant|malformed`: appends a retirement execution record; pipeline-check excludes retired categories from subsequent currency computation.
- `execute-recalibration redispatch --category CATEGORY --spec-path <path>`: emits a standard Copilot dispatch with the refined spec and appends execution record.
- State invariants: split executions must have matching new entries; retirement executions must have matching reason and verifiable evidence.

### Phase 4 — Audit-agent check loop

**Deliverables**:
- Audit agent prompt extension: when a `recalibration-proposal` issue appears, produce one of `approve` / `concerns: <reasons>` / `reject: <reasons>`.
- Orchestrator tool to read audit dispositions and route the proposal accordingly (see Decision 3 protocol).
- SLA tracker: when a proposal's SLA cycle arrives with no audit response, auto-file escalation per Decision 5.
- Pipeline-check invariant: no `recalibration_executions[]` row exists without a corresponding `approve` disposition on its proposal.

**Exit criteria**: end-to-end: a chronic-stuck category produces a recalibration proposal → audit responds → orchestrator executes or escalates. Verified on at least one real case (likely `code-quality`).

### Phase 5 — Detector polish and escape-valve refinement

**Deliverables**:
- Upgrade "standing Eva blockers" detector to distinguish three states: unanswered, answered-awaiting-close, in-flight-audit (not yet SLA-expired).
- Escalation counter: rolling-window tracker of Eva-escalations; threshold crossing opens a `[system-health]` meta-issue.
- Standardized escalation issue template per Decision 5.
- Deprecate `feedback_issue_closure` memory once orchestrator prompt is updated to close-after-ack correctly.

## Consequences

### Positive

- The three currently-stuck chronic entries (`review-evidence`, `worklog-accuracy/scope-boundary`, `code-quality`) are resolved through the new mechanism without Eva intervention.
- Bookkeeping drift is eliminated: verification becomes evidence-based and computed, not declarative.
- Malformed entries are detected and routed, not permanently blocking.
- Taxonomy errors have a resolution path (split/retire) that doesn't require human judgment in the common case.
- Eva escalation volume drops substantially; remaining escalations are genuinely novel or disputed.
- Audit trail of every verification, observation, proposal, and execution supports retrospective analysis.

### Negative

- **State.json grows more structured.** Four new top-level collections under `chronic_category_responses`, each with invariants. More surface for future bugs.
- **Audit-agent becomes load-bearing.** Recalibration cannot execute without audit disposition. If audit is broken, recalibrations pile up against the SLA wall. Mitigated by SLA timeout but not eliminated.
- **Recalibration agent introduces new prompt surface** with unknown reliability. Mitigated by audit gating and by the fact that recalibrations should be rare. A bad split or bad retirement is correctable via later events (splits can be re-unified via retirement of children + new parent entry; retirements can be superseded if the category recurs).
- **Latency on self-healing.** Even the happy path for recalibration takes ~2 days (1 day audit cycle, 1 day orchestrator pickup). Acceptable because recalibration is infrequent; not on the critical path for normal work.
- **Append-only ledgers grow unbounded.** Long-term concern, not immediate. Eventually will need archival compaction; out of scope for this ADR.

### Neutral

- ADR 0013's classification of `chronic-category-currency` as non-blocking at C5.5 remains in place and is compatible with this design. This ADR primarily affects the dispatch-task gate path and the state-machine of chronic entries themselves.

## Rollback

The design is additive: no existing behavior is removed, only augmented. If a phase proves unworkable, it can be reverted without disturbing earlier phases.

- **Phase 1 rollback**: revert `process-merge` event-appending; pipeline-check falls back to reading entry `verification_cycle` as before. Existing `verification_events[]` data remains in state.json as inert historical records.
- **Phase 2 rollback**: revert `chronic-entry-validity` phase; existing `validity_observations[]` become inert. No data loss.
- **Phase 3/4 rollback**: disable the `propose-recalibration` trigger; existing proposals remain in state as inert records; no new executions are written. Audit-agent prompt extension can be retained harmlessly (no trigger means no input).
- **Phase 5 rollback**: detector reverts to OPEN-state keying; no data or behavior loss.

In the worst case (complete rollback after partial implementation), chronic-category handling reverts to current behavior with some extra append-only collections carrying inert history — no worse than today.

## Cross-references

- **ADR 0011** — Pipeline Stabilization Program. Establishes the stabilization context within which this ADR operates.
- **ADR 0013** — Non-Blocking Classification for Systemic Pipeline Checks. Addresses the C5.5 side of the chronic-category-currency blocking issue; this ADR addresses the remaining dispatch-task side and the underlying lifecycle problems.
- **Issue #2542** — Original dispatch-task pipeline gate deadlock. Partially resolved via in-flight downgrade; this ADR completes the resolution.
- **Issue #2622** — Re-pose of #2542 with four options. This ADR selects none of the four verbatim but addresses the underlying problem via event-sourcing and recalibration.
- **Issue #2293** — Chronic journal-quality structural responses. Eva selected Option B on 2026-04-19; recalibration agent (Phase 3) is compatible with and subsumes the commitment-level grading structure.
- **Issue #2638** — cycle-start push bug and F4 violation reproduction. Independent of this ADR; answered separately.
- **`feedback_chronic_entries_append_only`** — append-only constraint on existing entries.
- **`feedback_system_purpose_framing`** — self-improvement as the product; minimize human-in-loop.
