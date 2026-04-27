# Cycle 5 working notes — state.json field-by-field categorization (F12 catalog start)

**Cycle**: redesign cycle 5 (2026-04-27)
**Source**: `docs/state.json` top-level keys at cycle 5 startup (commit `e83af66e`, post-cycle-545 close-out state).
**Purpose**: F12 catalog completion is OQ7 — cycle 4 named state.json field-by-field categorization as the cycle 5 starting move. Every field categorized as **D** (defense — added in response to a specific failure mode), **P** (primitive — intrinsic to the schema-org-json-ld domain or basic cycle mechanics), or **M** (mixed — has both characters, or is a primitive whose shape was warped by defense accretion). The catalog is the missing input to Phase 2 candidate evaluation per F12.

## Method

For each top-level key in `docs/state.json` (42 total — cycle 1's "38+" estimate was close):

1. Identify the failure mode it defends against (if any), with citation if known.
2. Identify the v1 mechanism that writes to it (a tool, a checklist step, or both).
3. Classify D / P / M.
4. Note the v2 transfer-or-remove decision (preliminary — to be refined in candidate-design phase).

The M category is deliberately broad: a field that's *primarily* primitive but has accumulated defense-shaped sub-keys lives in M. The point is to flag fields needing explicit transfer-or-eliminate decisions in Phase 2.

## Per-field catalog

### Defenses (D)

| # | Key | Defends against | Mechanism | v2 decision (preliminary) |
|---|---|---|---|---|
| 1 | `agent_sessions` (922 items append-only) | Losing track of dispatched sessions; F4 frozen-artifact lifecycle reads this for receipt tables | `record-dispatch`, `merge-pr`, `process-merge` write; multiple checkers read | **Reshape** — keep dispatch ledger but with bounded retention (e.g., last 50) and explicit reconciliation (closed/superseded/expired transitions) |
| 2 | `audit_dropped` | Silent dropping of audit acceptances (F12 seed) | `process-audit` writes | **Eliminate** — F11 framing: state evolves continuously; "dropped" is just one terminal state of an acceptance, not a separate field |
| 3 | `audit_processed` (197 items) | Missed audit acceptances (F12 catalog explicit) | `process-audit` writes; review-agent and audit cross-check | **Reshape** — bounded retention, single ledger with state machine (open/processed/dropped/superseded) |
| 4 | `cycles_since_last_forward_work` (count=134, threshold=10) | Schema-work stagnation (F7); added cycle 411 per audit #345 | metric-snapshot computes from git log | **Reshape** — F7 v2 success criterion makes this a primary surfaced metric, not a buried state field; computed on demand by a tool, not stored |
| 5 | `deferred_findings` (128 items append-only) | Losing track of deferred review findings (F9 disposition state) | `process-review` writes; review next cycle reads | **Reshape** — bounded retention; F9's review-and-disposition treadmill is itself the failure mode this defends, so v2's review surface determines what shape (if any) this needs |
| 6 | `eva_input_issues` (closed_prior_cycles, closed_this_cycle, remaining_open) | Missed Eva directives | `process-eva` writes | **Reshape** — F2 root: every channel needs poller + state-transition; this should be poller output, not append-only history |
| 7 | `field_inventory` | State field freshness drift (F12 catalog explicit) | metric-snapshot writes; pipeline-check reads | **Eliminate** — meta-defense (a defense against the state shape itself drifting); v2 minimal state means freshness drift is not the failure mode |
| 8 | `in_flight_sessions` (count=3) | Capacity overrun in dispatch | `record-dispatch`/`merge-pr` write | **Eliminate or compute on demand** — derivable from open dispatched issues; storage is duplication |
| 9 | `last_eva_comment_check` (timestamp) | F2 detection failure (F12 catalog explicit, audit #439) | `check-eva-responses` tool writes | **Reshape** — needs poller still (F2 root unaddressed at architecture level), but the *timestamp* is implementation detail not state-of-record |
| 10 | `last_tool_audit_cycle` (number=403) | Tool freshness drift | manual / scheduled tool sweep | **Eliminate** — v2 should not have "tool audit" as a thing tracked in state |
| 11 | `open_questions_for_eva` (3 items: 436, 699, 808, 809, 2039) | F2 (Eva blocker tracking); but Eva said the queue was closed (#2741) and three are still tracked here | `process-eva`, possibly stale | **Eliminate** — v2 Eva-channel design replaces this with poller-driven state; the current 3 stale items are evidence the field accreted past its working semantics |
| 12 | `pending_audit_implementations` (3 items) | Accepted recommendations not yet adopted (PR #2718, F12 catalog) | `accepted-audit-adoption` gate writes/reads | **Reshape** — folds into the unified audit-acceptance ledger (#3 above) |
| 13 | `pre_python_clean_cycles` (4 keys — clean cycle counter for Python port readiness) | Premature port-launch into language with insufficient stabilization | mode/stabilization tooling | **Eliminate** — Python port is post-cutover work; defense against premature port is not a v2 concern (v2's work is itself the new-language-launch problem viewed differently) |
| 14 | `review_agent` (4 sub-keys: chronic_category_responses, history=356 items, last_review_cycle, description) | Review finding disposition drift (F12 catalog explicit) | `process-review`, `dispatch-review` write | **Reshape or eliminate** — depends on v2's review surface; the 356-item history is evidence of unbounded accumulation |
| 15 | `review_dispatch_consecutive` (count=5) | Runaway review dispatch (every cycle dispatches review → F9 treadmill) | `dispatch-review` writes | **Eliminate** — F9 root: v2's review surface is redesigned, not capped |
| 16 | `review_events_verified_through_cycle` (number=545) | Review-event verification gap; F12 catalog explicit; state-invariants check | `verify-review-events` writes | **Eliminate** — meta-defense on the review event flow; v2 review surface determines whether anything like this is needed |
| 17 | `step_comment_acknowledged_gaps` (8 items) | Silent missed step comments (F12 catalog explicit, F3 root) | `post-step` / step-comment-acknowledgment tool writes | **Eliminate** — F6 root: v2 reduces step-commenting to essential checkpoints; the gap-tracking field disappears with the surface |
| 18 | `qc_processed` (6 items) | Missed QC interaction processing | `cross-repo` reads QC repo, writes here | **Reshape** — QC interaction is preserved in v2; needs same poller+reconciliation pattern as audit |
| 19 | `qc_requests_pending` (1 item) | Missed pending QC requests | same as #18 | **Reshape** — folds into unified QC ledger with state machine |

**D total: 19 of 42 = 45%**

### Primitives (P)

| # | Key | Domain meaning | v2 decision |
|---|---|---|---|
| 1 | `constructor_refactoring` (5 keys) | Tracking constructor refactoring across schema classes | **Keep** — domain work tracking |
| 2 | `release` (1 key) | Release metadata | **Keep** |
| 3 | `schema_status` (14 keys) | Schema implementation progress (implemented, in_progress, phpstan_level, etc.) | **Keep** — domain primitive; the field-by-field shape may simplify but the existence is intrinsic |
| 4 | `schema_version` | schema.org version tracking | **Keep** |
| 5 | `test_count` (5 keys) | PHP/TS test counts | **Keep** — domain primitive (slight defense flavor in `last_verified_note` field; the verification mechanism is the defense, not the count) |
| 6 | `total_enums` | Enum count | **Keep** (or compute on demand) |
| 7 | `total_schema_classes` | Class count | **Keep** (or compute on demand) |
| 8 | `total_schema_types` | Type count | **Keep** (or compute on demand) |
| 9 | `total_standalone_testable_types` | Testable type count | **Keep** (or compute on demand) |
| 10 | `total_sub_types` | Sub-type count | **Keep** (or compute on demand) |
| 11 | `total_testable_types` | Testable type count | **Keep** (or compute on demand) |
| 12 | `total_testable_types_note` | Note explaining the testable-type denominator | **Keep** as documentation |
| 13 | `typescript_plan` (10 keys) | TS port plan and progress | **Keep** — domain primitive (multi-language port is preserved) |

**P total: 13 of 42 = 31%**

Note: 7 of these 13 (#6–12) are pure counts that could be computed on demand from schema source. v2 can store them or compute them; the choice doesn't affect the defense/primitive split — they are intrinsic state.

### Mixed (M)

These are the ambiguous cases — primarily primitive but warped by defense accretion, or primarily mechanical but carrying some defense character. Each needs an explicit transfer-or-eliminate decision in Phase 2.

| # | Key | Primary character | Defense flavor | v2 decision (preliminary) |
|---|---|---|---|---|
| 1 | `blockers` (1 item: npm publish workflow) | Primitive (release blocker) | Slight (defends against forgetting unresolved blockers) | **Reshape** — single-blocker tracking is fine; the "1 item" pattern shows it's not really being used as a queue |
| 2 | `cycle_issues` (1 item) | Mechanical (current cycle issue number) | None | **Eliminate** — re-derivable from GitHub label query |
| 3 | `cycle_phase` (4 keys: completed_at, cycle, phase, phase_entered_at) | Mechanical | F11 root (cycle as artificial completion signal): this field encodes the artificial boundary | **Eliminate** — F11 design implication: state evolves continuously, no cycle-phase state-of-record |
| 4 | `dispatch_log_latest` (timestamp) | Mechanical (latest dispatch timestamp) | None | **Eliminate** — derivable from agent_sessions max-by-timestamp |
| 5 | `last_cycle` (5 keys: duration_minutes, issue, number, summary, timestamp) | Mechanical (last cycle summary) | F11 root: post-close mutation target (per F4 evidence) | **Reshape** — last cycle representation is fine; the *frozen* representation that F4 documents as failing is what should change |
| 6 | `previous_cycle_issue` (number=2728) | Mechanical | None | **Eliminate** — derivable from GitHub or from agent_sessions history |
| 7 | `project_mode` (9 keys) | Primitive (current mode: redesign/normal/stabilization) | Defense (clean_cycle_counter, clean_cycle_criteria, burn_in_target — mode-gating defenses) | **Reshape** — mode itself is primitive; the cycle-counter sub-keys are defenses against premature mode transition. v2's mode design determines whether this is needed at all (v2 might not have stabilization mode if cycle abandonment is impossible by construction) |
| 8 | `publish_gate` (11 keys: audit_signoff, qc_ack, validated_at, validated_commit, status, etc.) | Primitive (publishing state) | Defense (multi-party signoff requirements as data — could be a procedure instead) | **Reshape** — publishing state is genuine domain primitive; the gate-as-data shape may simplify when v2 expresses publishing as a tool-driven process |
| 9 | `qc_status` (10 keys: report_57, report_72, request_165, request_200, etc.) | Primitive (QC interaction state) | Append-only accumulation (same shape as F5) | **Reshape** — QC interaction state is needed; the unbounded accumulation isn't |
| 10 | `tool_pipeline` (7 keys: c5_5_gate, eva_directive, phases, publish_gate, status, blocks_publish, etc.) | Mechanical | Almost entirely defense (c5_5_gate is the F4 freeze mechanism, eva_directive flag, phases tracking — every sub-key defends against a procedural failure) | **Eliminate** — F1+F4+F6 roots: v2 has no checklist gates of this shape; the field is the v1 procedural surface made of state |

**M total: 10 of 42 = 24%**

## Aggregate findings

| Category | Count | Percentage |
|---|---|---|
| Defense (D) | 19 | 45% |
| Primitive (P) | 13 | 31% |
| Mixed (M) | 10 | 24% |
| **Total** | **42** | 100% |

**Defense-character total: D + (M with defense flavor) ≈ 26 of 42 (62%)** of state.json's top-level fields exist primarily as defense against a specific v1 failure mode. The "primitive" core is **13 of 42 (31%)** — and 7 of those 13 (54% of primitives) are pure counts that could be computed on demand.

The state shape is dominantly procedural-residue. F5's claim ("the state shape encodes the failures, not the system. Each field is a scar") is supported by this measurement: the majority of v1's state-of-record is defense, not domain.

## v2 implications

1. **A v2 with a designed state model (F5 implication, this measurement)** would have approximately **13 fields** (the P column) plus a small set of cycle-mechanics fields shared across many of the M-decisions (current cycle reference, last cycle reference). The 19 defenses + 10 mixed-shaped-by-defense reduce to perhaps 4–6 fields in v2:
   - Schema progress (P, kept)
   - Test counts / version (P, kept)
   - QC interaction ledger (one ledger replacing qc_processed + qc_requests_pending + qc_status)
   - Audit interaction ledger (one ledger replacing audit_processed + audit_dropped + pending_audit_implementations)
   - Cycle / dispatch ledger (one ledger with bounded retention replacing agent_sessions + cycle_issues + cycle_phase + previous_cycle_issue + last_cycle + dispatch_log_latest + in_flight_sessions)
   - Mode (project_mode minus the stabilization-counter sub-keys, IF mode is preserved)
   That's a v2 state model of ~5 ledgers + ~10 atomic fields — a 4× reduction from v1's 42 keys.

2. **Most "Eliminate" decisions follow from a shared-root v2 decision.** F11 (cycle as continuous-state, not hard-boundary) eliminates `cycle_phase`, `cycle_issues`, `previous_cycle_issue`, possibly `last_cycle`. F2 (every channel needs a poller) eliminates `last_eva_comment_check`, reshapes `eva_input_issues`. F1 (tools handle procedure) eliminates `tool_pipeline`, `step_comment_acknowledged_gaps`, `field_inventory`, `review_events_verified_through_cycle`. Each shared root collapses several state fields simultaneously.

3. **Some "Reshape" decisions need Phase 2 candidate-design input.** The QC and audit ledger shapes depend on the v2 cross-repo communication design. The dispatch ledger shape depends on whether v2 keeps dispatched-Copilot-as-a-mechanism. Phase 2 candidates should each propose a state model; this catalog gives the constraint set those proposals must address.

4. **The 197-item `audit_processed` and 922-item `agent_sessions` and 356-item `review_agent.history` and 128-item `deferred_findings` are evidence for the F5 hypothesis at scale.** Append-only structures with no bounded retention are the structural failure mode F5 names. v2 must either (a) bound retention by design (LRU, size cap, time-window), (b) move history to git-log-derived computation, or (c) explicitly accept the unbounded-accumulation cost. Per F1: a tool that mechanically caps retention is structurally stronger than a constraint that says "keep retention reasonable."

## Caveats

- **One-cycle snapshot.** This catalog is state.json at cycle-545 close. Field-existence evidence is durable; the size measurements (922 items, 356 entries, etc.) reflect this specific snapshot.
- **Categorization judgment.** D / P / M boundary is a judgment call. I tried to be conservative — fields with any genuine defense character go in D or M, not P. Anyone reviewing this should expect to find 2–4 categorizations they'd argue. The aggregate finding (defense-dominance) is robust to those margins.
- **Phase 2 inputs.** The "v2 decision (preliminary)" column is tentative. Phase 2 candidate-design will firm or change these per design choice. The catalog's load-bearing contribution is the failure-mode-per-field column, not the v2 decision.
- **F12 catalog completion.** This is one of three sub-catalogs F12 references (state.json fields ✓; pipeline-check sub-checks; recurrence-escalation citations in checklists). Cycles 6+ should complete the other two. The state.json catalog is the largest and most defense-dominant of the three; the others are expected to confirm the pattern at smaller scale.

## What carries forward

- **F12 retrospective update**: add the 19/13/10 numbers and the 4×-reduction estimate to F12's body. Reference this notes file.
- **F5 strengthening**: F5 currently claims "the state shape encodes the failures, not the system." This catalog provides the measured backing. Update F5 with the 62% defense-character finding.
- **Cycle 6 candidate**: pipeline-check sub-checks categorization (~32 sub-checks per cycle 1 estimate). Same method, smaller surface.
- **Cycle 7 candidate**: checklist recurrence-escalation citations chronologically (cycle 4's measurement covered C3 + C5/C5.5; extend to STARTUP_CHECKLIST and to the prompt itself).
- **Phase 2 input**: this catalog is now ready as a constraint set for candidate-design. Each Phase 2 candidate must address the 19 D + 10 M decisions.
