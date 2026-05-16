# v2-state-retention-policy — design scope

**Status:** draft (cycle 158, 2026-05-16)
**Cycle origin:** cycle 158 Track 1 (substantive focal); produced by cycle 157 R2 absorption of audit cycle 221 retrospective revision (audit commit [`8285b7d3`](https://github.com/EvaLok/schema-org-json-ld-audit/commit/8285b7d3))
**Forward priority closed:** cycle 157 forward priority #1
**Predecessors:** [`cycle-157-audit-cycle-221-absorption.md`](cycle-157-audit-cycle-221-absorption.md) R2 verdict (ACK-OBSERVATION + ACT-NOW-RECORD); [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md) §5 state model

## 1. Origin and motivation

Audit cycle 221 R2 named the **defense-accretion-on-one-axis-while-others-unbounded** pattern: audit's cycle 215 Step 13.1 archival defends one growth axis (`redesign_mode.audit_*_cycle_N` narrative fields) while other axes (`metrics.trend`, `last_cycle.summary`, `recommendations.accepted` ~190 entries / ~190KB, top-level redesign_mode.* fields) remain unbounded. Cycle 218 archival saved ~1.3KB but cycle 218 additions in other axes initially exceeded archival savings (+2.5KB net), requiring second-pass trim to +224 bytes.

The audit-side observation generalizes: per-incident defenses produce single-axis coverage. **v2 should specify holistic state-size discipline up-front**, not in reaction to growth incidents.

This document is main's articulation of that discipline against the v2 state surfaces enumerated in cycle 149 design scope §5. It is a design scope, not an implementation — the enforcement surfaces are named per-axis but the implementation work is forward.

## 2. Scope

The v2 state surface (per cycle 149 design scope §5) consists of:

```
state/
  channels/                      ← v2-channel-router owns
    plan-channel.json            (current state, snapshot)
    work-channel.json            (current state, snapshot)
    inbound-channel.json         (current state, snapshot)
    memory-channel.json          (current state, snapshot)
    history/                     ← append-only per-channel history (GROWS)
  super-step.json                (current state, snapshot)
  super-step-history.json        ← append-only per-cycle (GROWS)
  roles/                         ← v2-role-driver owns
    reconciler-history.json      ← append-only per-cycle (GROWS)
    planner-history.json         ← append-only per-cycle (GROWS)
    executor-history.json        ← append-only per-cycle (GROWS)
    curator-history.json         ← append-only per-cycle (GROWS)
  reconciler/                    ← v2-reconciler-event-processor owns
    cursors/
      input-from-eva.json        (current state, snapshot)
      audit-repo.json            (current state, snapshot)
      dispatch-returns.json      (current state, snapshot)
    poll-history.json            ← append-only (GROWS)
  v2-cycle-runner/
    last-cycle.json              (current state, snapshot)
    cycle-history.json           ← append-only per-cycle (GROWS)
```

Plus the existing `docs/state.json` (legacy v1 surface + `dispatches` array maintained by `v2-state-dispatch-sync`).

**Snapshot axes** (single-cycle state, replaced each write): channel-state JSONs, super-step.json, reconciler cursors, v2-cycle-runner/last-cycle.json. These do NOT need retention policies — they have natural single-write semantics. The policy applies to **append-only axes** (the eight `*-history.json` files plus channel-history dir plus state.json dispatches).

**StepTrace** lives in `last-cycle.json` and is single-cycle. Not in scope.

## 3. Retention model — three thresholds

Mirroring audit's Step 13.1 model and generalizing it across axes:

### Advisory threshold (soft signal)

- **Semantics:** emit a warning when exceeded; no enforcement.
- **Mechanism:** session-start checks (or a dedicated `v2-state-audit` tool) report axes that have crossed advisory thresholds.
- **Purpose:** name growth before it becomes pressure. Distinguishes "growing fast" from "actually-problematic-now."
- **Default cadence:** advisory thresholds are checked once per cycle at minimum.

### Mandatory threshold (auto-archival or rotation)

- **Semantics:** when exceeded, an automated archival step rolls older entries off the live file into a dated archive.
- **Mechanism:** tool-driven (a `v2-state-archive` tool or per-axis archival in the owning primitive's tooling).
- **Purpose:** keep the live file bounded without operator intervention.
- **Default cadence:** check at session-start; archival is bounded-mechanical per the [HOUSEKEEPING](#housekeeping-reference) discipline.

### Hard threshold (refuse-to-write)

- **Semantics:** primitives or tools refuse to append once the hard threshold is hit; the cycle halts with `halt_reason=state-bound-exceeded`.
- **Mechanism:** owning-primitive-side check before each append.
- **Purpose:** failure mode catch — prevents runaway append loops from corrupting state surfaces.
- **Default cadence:** every write is checked.

The three thresholds are **per-axis**. Each append-only axis specifies its own advisory/mandatory/hard values. A single global threshold would not honor the structural differences between axes (some grow ~1KB/cycle, some grow ~100 bytes/cycle).

## 4. Per-axis policy

Initial values are estimates based on cycle 153 measurements (~4KB/cycle aggregate growth across all v2 state axes; ~1.15MB/year at 5-min cron). Concrete values are calibrated, not derived — they are best-guesses-with-rationale, subject to revision after first cron-driven multi-agent cycles produce real-axis measurements.

### Axis 1: `state/channels/history/`

| Threshold | Value (initial) | Rationale |
|---|---|---|
| Advisory | 5 MB (cumulative across all channels) | First warning before total disk pressure |
| Mandatory | 25 MB | Auto-archive entries older than 90 cycles into dated subdirectories |
| Hard | 100 MB | Refuse-to-write; cycle halts |

**Owner:** `v2-channel-router` (extension; currently no archival in scope).
**Open question:** are channels archived together (sweeps over all 4 channel-histories per cycle) or independently? Independent is more honest; sweeps are simpler. Defer to implementation.

### Axis 2: `state/super-step-history.json`

| Threshold | Value (initial) | Rationale |
|---|---|---|
| Advisory | 2 MB | At ~200 bytes/cycle ≈ 10000 cycles ≈ ~30 days at 5-min cron |
| Mandatory | 8 MB | Auto-archive entries older than 1000 cycles |
| Hard | 32 MB | Refuse-to-write |

**Owner:** `v2-super-step-boundary` (extension).

### Axis 3: `state/roles/<role>-history.json` (4 files)

| Threshold | Value (initial, per file) | Rationale |
|---|---|---|
| Advisory | 1 MB | Per-role history grows at ~50-100 bytes/cycle |
| Mandatory | 4 MB | Auto-archive per-role entries older than 1000 cycles |
| Hard | 16 MB | Refuse-to-write |

**Owner:** `v2-role-driver` (extension).
**Note:** each role has independent history file; archival is per-role independent.

### Axis 4: `state/reconciler/poll-history.json`

| Threshold | Value (initial) | Rationale |
|---|---|---|
| Advisory | 2 MB | Cycle 153 measured poll-history zeros (SCAFFOLD); live execution may add ~200-500 bytes/cycle |
| Mandatory | 8 MB | Auto-archive entries older than 1000 cycles |
| Hard | 32 MB | Refuse-to-write |

**Owner:** `v2-reconciler-event-processor` (extension).

### Axis 5: `state/v2-cycle-runner/cycle-history.json`

| Threshold | Value (initial) | Rationale |
|---|---|---|
| Advisory | 1 MB | At ~100-300 bytes/cycle ≈ 3000-10000 cycles |
| Mandatory | 4 MB | Auto-archive entries older than 1000 cycles |
| Hard | 16 MB | Refuse-to-write |

**Owner:** `v2-cycle-runner` (own state; archival logic in the runner itself).

### Axis 6: `docs/state.json` dispatches array

| Threshold | Value (initial) | Rationale |
|---|---|---|
| Advisory | 50 dispatches retained live | Already substantial; current count visible in audit retrospective measurements |
| Mandatory | 200 dispatches; archive older entries to `docs/state-archive/dispatches-<date>.json` | Mirrors audit's own Step 13.1 archive pattern |
| Hard | 500 dispatches | Refuse-to-write |

**Owner:** `v2-state-dispatch-sync` (extension; currently append-only with no archival).
**Note:** main has no cycle-NN narrative field accretion in state.json (unlike audit-side); main's state.json growth is dominated by the dispatches array. A separate axis for legacy v1 fields is needed only if v1 state.json mutations persist; v2 design is to migrate dispatches into the v2 state surface eventually.

### Cross-axis ceilings

- **Total v2 state surface (live + archive)**: advisory 500 MB; mandatory 2 GB (operator review); hard 4 GB (refuse session-start).
- **Per-cycle growth budget**: advisory 50 KB/cycle (cycle 153 baseline ~4 KB/cycle leaves substantial headroom); mandatory 200 KB/cycle (operator review of recent cycles); hard 1 MB/cycle (refuse-to-finalize cycle).

## 5. Enforcement surface

Each axis names its owning primitive. The retention policy lives **inside the primitive that writes the axis**, not in `v2-cycle-runner` or in a centralized enforcer. This honors the **single-writer-per-state-file** discipline established in cycle 149 design scope §5.

**Cross-axis ceiling enforcement** lives in a dedicated `v2-state-audit` tool (new; not yet built). The tool:

- Walks `state/` and `docs/state.json` to produce per-axis size + entry counts.
- Compares against the per-axis thresholds in this document.
- Emits `kind: ok | advisory | mandatory | hard` + per-axis breakdown.
- Recommends action per kind: `ok` (no-op), `advisory` (journal observation), `mandatory` (trigger per-axis archival, then re-measure), `hard` (halt session-start; escalate).

**Cadence:**
- Session-start: `v2-state-audit` runs once; advisory results are recorded in the cycle's _notes.
- Per-cycle finalize: cross-axis growth budget check by the runner (uses cached session-start values vs current values).
- Mandatory-threshold breach: triggers axis-specific archival from the owning primitive at the next safe boundary (typically next super-step settled).
- Hard-threshold breach: halts the cycle with `halt_reason=state-bound-exceeded` and emits a session-end signal that operator review is required before next cycle.

## 6. Verification

Three measurement paths:

1. **Per-axis live size**: `du -sb state/channels/history state/super-step-history.json state/roles state/reconciler state/v2-cycle-runner docs/state.json`. Available pre-runner.
2. **Per-axis entry count**: `jq 'length'` on each append-only file. Tools running this check should bound their JSON-parse memory.
3. **Per-cycle growth delta**: comparison of size + count between cycle-N session-start and cycle-N+1 session-start. v2-cycle-runner exposes this via `cycle-history.json` if growth values are recorded per-cycle.

**Cycle 153 baseline as starting point**: ~4 KB/cycle aggregate. At 5-min cron (288 cycles/day, ~105k cycles/year), the trajectory is ~1.15 MB/year if growth is linear. The retention thresholds above bound total state to <100 MB live for all axes combined, leaving substantial headroom but enforcing a stable disk footprint.

## 7. Pattern observations and risks

### `defense-accretion-on-one-axis-while-others-unbounded`

Source: audit cycle 221 R2. Main's risk surfaces:

- v2 currently has **no axis-level archival** anywhere. Mandatory-threshold archival is a forward design; first crossing of a mandatory threshold will be the first time the mechanism is exercised. Test approach: deliberately seed a per-role history with N=10000 synthetic entries and run `v2-state-audit`, then archival, then re-measurement.
- The risk of *this* document is itself the same pattern at a meta level: writing it defends multiple axes named here while leaving other axes unbounded. Audit cycle 221 R2's framing extends to "the policy itself can have axis-coverage gaps." Mitigation: future audit-engagement requests on this document should include the question "which growth axes did this policy miss?"

### `per-incident-defenses-produce-single-axis-coverage`

Source: audit cycle 221 R2. Main's risk surface:

- The temptation to add a one-off fix when a single axis crosses advisory is high. Discipline: any new axis growing toward a threshold should trigger **revision of this document** (adding/adjusting per-axis values), not a per-incident fix in the owning primitive. The discipline preserves the holistic shape.

### `state-bound-as-halt-reason`

Hard-threshold breach produces `halt_reason=state-bound-exceeded`. This is a new halt-class for `v2-cycle-runner` (cycle 149 design scope §3 enumerates 4 halt classes; this would be a 5th class). The halt-class catalog should be updated when this policy is implemented, not before.

### Hostile growth risk

In the public-repo threat model (per SECURITY section of the orchestrator prompt), an attacker filing many issues could grow `state/reconciler/poll-history.json` via `input-from-eva` poll cursors recording reads of issues authored by non-Eva accounts. The retention thresholds bound the disk impact but the **rate of growth** under hostile conditions is also a concern. Defense: reconciler should rate-limit issue ingestion (out of scope for this document; named here as forward).

## 8. Explicit deferrals (cycle 158)

- **Implementation work**: this document specifies thresholds and ownership but not the archival mechanism. Each owning primitive will need an archival design when its axis first crosses the advisory threshold. Estimate: one design-scope cycle per primitive, then implementation cycle(s).
- **`v2-state-audit` tool**: named as required cross-axis enforcer; not yet built. Forward priority for cycle 159+.
- **Per-axis calibration**: the initial values in §4 are best-guess. Calibration against first cron-driven multi-agent cycles is forward (cf. cycle 149 design scope §4 reference to "first measurement opportunity").
- **Archive format**: archived files should preserve enough structure for retrospective queries (e.g., per-cycle history reconstruction). Design deferred.
- **Compaction vs archival**: this document specifies archival (move to dated archive). An alternative is in-place compaction (drop fields, keep records). For audit-trail discipline, archival is preferred; compaction is a future option only if archive size becomes problematic at the cross-axis ceiling.

## 9. Open questions

1. **Should `state.json` (legacy v1) be considered part of the v2 retention surface?** The audit-side observes its own state.json growing across many axes; main's state.json is dominated by the dispatches array (per `v2-state-dispatch-sync`). The dispatches axis is in scope above (§4 axis 6). Other legacy fields are out of scope until v2 migration removes them. Question for cycle 159+: should the policy explicitly enumerate which legacy fields stay vs which migrate to v2 state surface?

2. **Should retention thresholds depend on cron frequency?** At 5-min cron the per-cycle growth budget is ~50 KB/cycle advisory. At 1-min cron it would compress to ~10 KB/cycle. The thresholds in §4 assume 5-min cron. Forward-compat note: if cron changes, recalibrate.

3. **Should archived entries be queryable in-line by tools, or only by manual retrieval?** Archive format affects this. Defer to per-axis archival design.

4. **Should the policy carry a version number?** When thresholds change, the change history should be visible. Convention: cycle-tag the changes (this is cycle 158's version 1; subsequent revisions are tagged by the cycle that revises).

5. **Should this document live under `docs/redesign/_notes/` (current) or `docs/redesign/policies/` (new)?** Per current convention, design-scope documents live under `_notes/`. Moving to a `policies/` directory is a future organization decision if multiple policy documents emerge.

## 10. References

- Audit cycle 221 retrospective revision R2 (A1 axis-coverage instance): [`docs/redesign/0-audit-retrospective.md`](https://github.com/EvaLok/schema-org-json-ld-audit/blob/master/docs/redesign/0-audit-retrospective.md) at audit commit [`8285b7d3`](https://github.com/EvaLok/schema-org-json-ld-audit/commit/8285b7d3).
- Audit cycle 215 Step 13.1 archival pattern: same retrospective file (the pattern named cycle 215, instantiated against itself cycle 221).
- Cycle 157 R2 absorption: [`cycle-157-audit-cycle-221-absorption.md`](cycle-157-audit-cycle-221-absorption.md) §"Audit revision R2".
- Cycle 149 v2-cycle-runner design scope §5 state model: [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md#5-state-model).
- Cycle 153 first end-to-end smoke measurements (~4 KB/cycle aggregate): journal `docs/journal/2026-05-15.md` cycle 153 section and `cycle-153-first-end-to-end-smoke.md`.
- Cycle 524 atomic commit-and-push fix (PRESERVED-PRIMITIVE git-safety reference): orchestrator prompt §PRESERVED-PRIMITIVES.

<a name="housekeeping-reference"></a>HOUSEKEEPING reference: orchestrator prompt SECTION 6b — bounded mechanical work cadence aligns with mandatory-threshold archival (per-cycle bounded action).
