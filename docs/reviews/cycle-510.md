## 1. [worklog-accuracy/post-dispatch-state] The published cycle summary freezes a pre-dispatch snapshot and then immediately goes stale

**File**: docs/worklog/2026-04-17/213600-cycle-510-review-actioned-3-of-3-with-rollback-honesty.md:6
**Evidence**: The worklog says `No new dispatches.` and its cycle-state block says `In-flight agent sessions: 0` (`:6`, `:24`). The journal repeats the same stance: `No new dispatches. ... Cycle 510 review will dispatch at C6` (`docs/journal/2026-04-17.md:157`). But the same cycle's `state(record-dispatch)` commit `b8ea213` lands four seconds after the docs commit, adds review issue `#2575` to `agent_sessions`, flips `in_flight_sessions` from `0` to `1`, and rewrites `last_cycle.summary` from `0 dispatches` to `1 dispatch` (`docs/state.json:10937-10943`). This is the same stale mixed-timeline failure mode earlier reviews already documented in cycles 186, 281, and 412.
**Recommendation**: Dispatch the review before freezing the worklog/journal, or regenerate the published current-state lines after `record-dispatch` so the final narrative cannot lag behind the final committed state.

## 2. [journal-quality/commitment-grading] The journal grades a commitment as fully followed even though one branch was explicitly walked back

**File**: docs/journal/2026-04-17.md:135
**Evidence**: The quoted prior commitment had two operative parts: action findings if the review returned them, and take direct-push minimal patches if new chronic entries became refreshable. The final journal marks the commitment `**Followed.**` (`:137`) and then immediately says `Did NOT pursue further direct-push minimal patches` because cycle 509 review F1 showed that path was overreach. The cycle issue's own Step 0.6 comment used the more accurate status `followed and walked back`, but that nuance was flattened out in the permanent journal entry.
**Recommendation**: When a commitment branch is deliberately reversed by new evidence, grade it as partial or `followed and revised` rather than plain `Followed`, so the journal preserves the mixed outcome instead of over-crediting compliance.

## 3. [state-integrity/field-inventory] Cycle 510 closed with 13 stale field-inventory markers still unrefreshed

**File**: docs/state.json:10747
**Evidence**: The `field_inventory` table still carries many aged markers from cycles 498/503/508, including `audit_dropped`, `blockers`, `qc_processed`, `qc_requests_pending`, `qc_status`, several `schema_status.*` fields, `step_comment_acknowledged_gaps`, `tool_pipeline`, and `typescript_plan.status` (`docs/state.json:10750-10934`). The cycle's own final C5.5 comment reports `field-inventory` as a WARN with `13 field(s) exceed cadence thresholds`, listing those same entries in raw output. Cycle 510's worklog/journal record only the aggregate pipeline summary, not the fact that the explicit state-integrity target in this review brief remained stale at close-out.
**Recommendation**: Treat `field_inventory` warnings as work that must either be refreshed in-cycle or explicitly called out as unresolved state-integrity debt. If these after-change markers are intentionally allowed to age, tighten the cadence definitions so the inventory matches actual policy instead of repeatedly warning on known stale entries.

## Complacency score

**2/5** — Cycle 510 did some real corrective work: it rolled back an overclaimed chronic refresh, filed a narrower Eva-scope question, and the receipt table itself is defensible. But it still repeated a known stale post-dispatch timeline problem, overgraded a walked-back commitment as fully followed, and left the field-inventory freshness debt sitting in the final gate as a warning instead of resolving or foregrounding it.
