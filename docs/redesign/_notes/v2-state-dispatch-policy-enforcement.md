# v2-state-dispatch-policy-enforcement (cycle 161)

**Status:** design scope — pairs with [`v2-state-retention-policy.md`](v2-state-retention-policy.md) §4 Axis 6 and addresses the cycle 159 live-smoke finding (930-entry actual against 500-entry hard threshold).

**Authoring cycle:** 161 (2026-05-16). Closes cycle 160 forward priorities #3 (refuse-to-write hard-threshold enforcement) + #4 (threshold recalibration), paired because they share the same architectural problem.

**Predecessor scopes:** [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md) (halt-class catalog), [`v2-state-retention-policy.md`](v2-state-retention-policy.md) (policy doc).

## 1. Problem statement (why this document exists)

Cycle 158 v2-state-retention-policy §4 named:

- **Axis 6 owner:** `v2-state-dispatch-sync` (extension; currently append-only with no archival).
- **Hard threshold:** 500 dispatches, refuse-to-write.

Cycle 159 v2-state-audit live smoke against the actual repo measured **930 entries** under key `agent_sessions` — already ~2× the hard threshold. The audit tool correctly classifies as `hard` with `halt-session-start` recommendation. The policy was written without checking either (a) the actual storage key (`agent_sessions`, not `dispatches`) or (b) the actual current population.

Cycle 159 §4 Axis 6 patch added the "Storage-key clarification (cycle 159)" recording the gap and naming forward options. Cycle 160 forward priority #3 noted: "the actual `add` happens in v1 `record-dispatch` (frozen zone), not v2-state-dispatch-sync. A design scope cycle may be the right shape rather than direct code work."

That naming is what this document settles.

## 2. Architecture clarification (writer vs transitioner vs enforcer)

The `agent_sessions` array has three distinct operations and three distinct owners. The cycle 158 policy collapsed these into one "owner" line, which produced the misframing.

### 2.1 Writer (new entries)

`tools/rust/crates/record-dispatch/src/lib.rs:270` — `sessions.push(new_session.clone())` for fresh dispatches.

`tools/rust/crates/record-dispatch/src/lib.rs:381` — `sessions.push(patch.agent_session.clone())` for the `--patch` flow.

This crate is **v1, frozen zone per AUTHORITY > direct-push-zones** ("The current production tools and prompts (treat as frozen reference)"). The redesign cannot modify it directly. Any modification requires a workflow-change PR for Eva to merge — a heavier path than the redesign's direct-push zones.

### 2.2 Transitioner (existing entries)

`tools/rust/crates/v2-state-dispatch-sync/src/main.rs` — reconciles `agent_sessions[]` entries with `status: in_flight` against GitHub state, transitioning each to `merged` (with `pr` + `merged_at`) or `closed_without_pr`. **Does not append.** Cycle 154's tool extraction of the cycle 153 Track 2 manual housekeeping pattern.

This crate IS in the redesign direct-push zone, but the operations it performs (status transitions on existing entries) are not the operations that grow the axis. Adding `refuse-to-write` to this crate would not prevent v1 record-dispatch from appending the 931st, 932nd, ... entries.

### 2.3 Enforcer (cross-axis ceiling)

`tools/rust/crates/v2-state-audit/src/main.rs` — walks state surface, classifies per-axis, emits severity rollup. Already detects the 930-entry hard breach as of cycle 159 first operational use. **Does not yet halt cycles**; it only reports.

The practical enforcement primitive is **v2-state-audit invoked at v2-cycle-runner session-start, with a halt action wired to the `hard` exit code (3)**. This honors:

- **CORE-DESIGN-PRINCIPLE**: the audit tool's deterministic threshold check belongs in a tool; the orchestrator (and v2-cycle-runner as its run-time conductor) invokes the tool and acts on its signal.
- **Single-writer-per-state-file discipline** (cycle 149 §5): the tool that owns the cross-axis ceiling check (v2-state-audit) is distinct from the per-axis owner (v2-state-dispatch-sync transitions; v1 record-dispatch appends), preserving separation.
- **Halt-class catalog** (cycle 149 §3, extended cycle 158 policy §7 `state-bound-as-halt-reason`): `state-bound-exceeded` is the named halt class for this enforcement.

### 2.4 Restated three-layer ownership

| Operation | Owner | Zone | Modification path |
|---|---|---|---|
| Append (new dispatch) | v1 `record-dispatch` | FROZEN | workflow-change PR (heavy) |
| Transition (in_flight → terminal) | v2 `v2-state-dispatch-sync` | direct-push | direct |
| Enforce (cross-axis ceiling + halt) | v2 `v2-state-audit` + `v2-cycle-runner` session-start wiring | direct-push | direct |

The cycle 158 policy line "Owner: v2-state-dispatch-sync" should be read as "owner of the per-axis transition operations, not of the append operations." The cycle 161 patch to the policy doc (deferred to cycle 162+) will make this explicit.

## 3. Current population analysis (cycle 161 live count)

Cycle 161 session-start `jq` measurement of `docs/state.json`:

| Status | Count | Share | Lifecycle |
|---|---|---|---|
| `merged` | 819 | 88.1% | TERMINAL (success path) |
| `failed` | 62 | 6.7% | TERMINAL (Copilot pipeline failure) |
| `closed_without_pr` | 25 | 2.7% | TERMINAL (closed before PR opened) |
| `closed` | 15 | 1.6% | TERMINAL (closed for other reason) |
| `closed_without_merge` | 6 | 0.6% | TERMINAL (PR opened, closed unmerged) |
| `reviewed_awaiting_eva` | 2 | 0.2% | LIVE (review pending Eva) |
| `in_flight` | 1 | 0.1% | LIVE (Copilot working) |
| **Total** | **930** | 100% | — |

Time range: 2026-02-24 → 2026-05-15 (~80 days).

**Key observation:** 922 of 930 entries (99.1%) are in terminal status. Only 3 are operationally live (`in_flight` + `reviewed_awaiting_eva`). The current state has ~3 entries doing work and ~922 entries serving as historical audit-trail.

Recalibration cannot be a flat threshold bump — the threshold targeted "live + recent-history retention" but the actual population is overwhelmingly historical-retention. Archival, not threshold-relaxation, is the dominant lever.

## 4. Recalibration: thresholds vs archival

### 4.1 Cycle 158 vs cycle 161 recalibrated thresholds

Cycle 158 (Axis 6, written without checking actual data):

| Tier | Cycle 158 value | Premise |
|---|---|---|
| Advisory | 50 | "Already substantial" |
| Mandatory | 200 | "Mirrors audit's Step 13.1 archive pattern" |
| Hard | 500 | refuse-to-write |

The premise is right (archive at mandatory, refuse at hard); the values are calibrated against an imagined live-only retention scenario that doesn't match the actual population shape.

Cycle 161 recalibrated (live + bounded-recent-history scenarios):

| Tier | Cycle 161 value | Premise |
|---|---|---|
| Advisory | 100 LIVE entries (non-terminal) | Bounded multi-dispatch concurrency |
| Mandatory | 200 LIVE entries; or 1000 TOTAL (live + recent terminal) | Archival of terminal entries older than 30 days triggered |
| Hard | 500 LIVE entries; or 2000 TOTAL | Refuse-to-write; halt at session-start |

The cycle 161 model **splits the threshold per status class**:

- **Live thresholds** (`in_flight` + `reviewed_awaiting_eva`): policy concerns about concurrent operational state. 500 in-flight dispatches would mean either a runaway dispatch storm or a systemic Copilot timeout pattern. Hard threshold here is genuine "something is wrong."
- **Total thresholds** (live + archived-in-place terminal): policy concerns about disk footprint and audit-trail size. The hard threshold is reached when archival has not happened for a while.

Cycle 161 live measurement against this model: 3 live + 927 terminal = total 930. Under live-threshold: well below advisory (3 << 100). Under total-threshold: between mandatory (1000) and hard (2000) — same severity as cycle 158's flat read, different rationale.

### 4.2 Archival mechanism options

The dominant lever is archival of terminal entries. Three options sketched:

**Option A: per-status archive-on-transition.**
When `v2-state-dispatch-sync sync` transitions an `in_flight` entry to `merged` / `closed_without_pr`, write it to a separate archive file rather than mutating in-place in `agent_sessions[]`. Schema: `docs/state-archive/dispatches-merged.json` etc.

- Pro: archival happens at the natural lifecycle boundary; archive grows monotonically but isolated from `agent_sessions[]`.
- Con: the transition tool now does archival, expanding its scope; v1 record-dispatch's `--patch` flow that updates status (not just transition) needs awareness; queries that "want all dispatches ever" need to read multiple files.

**Option B: periodic age-based archival sweep (separate tool).**
A new `v2-state-dispatch-archive` tool (separate from v2-state-dispatch-sync) periodically moves terminal entries older than N days (suggested N=30) to a dated archive file `docs/state-archive/dispatches-<YYYY-MM-DD>.json`. Runs at session-start when v2-state-audit reports `mandatory`.

- Pro: clean separation of transition (per-cycle, v2-state-dispatch-sync) and archival (gated, v2-state-dispatch-archive); CORE-DESIGN-PRINCIPLE-aligned.
- Con: another tool to build, test, document; archival on-mandatory is a non-trivial design (lock, partial write, recovery).

**Option B is the recommended approach** for the redesign. The implementation cost is bounded (similar surface to v2-state-audit ≈ 1280 LOC including tests), the discipline is clean (one tool, one job), and the trigger condition (v2-state-audit reports `mandatory`) is already well-defined.

**Option C: in-place compaction with summary entries.**
Compress consecutive terminal entries into "epoch" summary entries. E.g., entries 1-100 merged in week W collapse to one entry `{epoch: W, count: 100, statuses: {merged: 95, failed: 5}}`.

- Pro: never grows; queries don't need multi-file reads.
- Con: lossy (loses per-dispatch issue numbers); breaks any tool that reads individual entries; the audit-trail value is largely destroyed.

Option C is **NOT recommended** — the audit-trail value is part of why these entries exist. Public-repo threat model (SECURITY) makes per-dispatch traceability a real concern for forensic review of any incident.

### 4.3 Implementation order

The recommended path forward:

1. **Cycle 162+**: wire v2-state-audit at v2-cycle-runner session-start with `--halt-on-hard` flag. Halt class: `state-bound-exceeded` (named cycle 158 policy §7). Re-uses existing tooling, no new policy decisions.
2. **Cycle 163+**: build `v2-state-dispatch-archive` per Option B above. ~1000 LOC + tests, design-scope precedent from v2-state-audit (cycle 159).
3. **Cycle 164+**: run archival against the live 819-merged backlog; bring TOTAL count from 930 to ~111 (live + recent-terminal). Re-run v2-state-audit; expect `ok` or `advisory` post-archival.
4. **Cycle 165+**: patch v2-state-retention-policy.md §4 Axis 6 with the live+total split thresholds (cycle 161 recalibration above), removing the cycle 159 storage-key-clarification's "forward work" section now that it's resolved.

The 930-entry hard-breach state is **observable-but-tolerated** until step 1 lands; v1 record-dispatch will continue appending in the interim. This is acceptable because:

- Disk footprint remains bounded (state.json is ~280KB at 930 entries; ~6× growth to 5MB at 5500 entries would still be parseable in well under 1s).
- Per-cycle additional growth from v1 record-dispatch is bounded by Copilot-dispatch frequency, currently ~0-1 per redesign cycle.
- The audit-trail value is preserved (Option C explicitly rejected).
- The forensic record is more valuable than the cosmetic enforcement (refusing to dispatch a Copilot session because the audit-trail is too long would be a worse failure mode than tolerating the audit-trail).

## 5. Anti-patterns this design avoids

### 5.1 "Refuse-to-write in v2-state-dispatch-sync"

Naive read of cycle 158 policy says "Owner: v2-state-dispatch-sync → add refuse-to-write." This is wrong:

- v2-state-dispatch-sync doesn't append; refuse-to-write here is no-op against the actual growth driver.
- It would catch the symptom (live count via transitions) but miss the root (append count via v1).

### 5.2 "Modify v1 record-dispatch directly"

Tempting but forbidden by AUTHORITY > direct-push-zones. A workflow-change PR could be filed, but the design scope above provides a v2-only enforcement path that works without v1 modification. Honoring the frozen-zone discipline is the better answer.

### 5.3 "Flat threshold bump to 1500 / 2000 / 5000"

Cycle 158's flat threshold model assumed live-retention semantics. A bump that keeps flat thresholds but raises values preserves the misframing — the policy's stated purpose (refuse-to-write at hard) becomes meaningless if hard is set high enough to never trigger.

The live+total split (4.1) honors the actual policy semantics: live-threshold catches operational anomaly; total-threshold catches archival neglect.

### 5.4 "v2-state-dispatch-sync archives at transition"

Considered as Option A above; rejected for redesign because it expands v2-state-dispatch-sync scope. The clean discipline is one tool, one job: transition (v2-state-dispatch-sync) and archive (v2-state-dispatch-archive future) are separate concerns with separate triggers.

## 6. Forward priorities produced by this scope

These extend cycle 160's renumbered forward list. Each item below is a new sub-priority spawned by this design scope; existing inheritance ordering is preserved.

- **v2-state-audit session-start wiring** in v2-cycle-runner (was cycle 159 priority #5, now refined): add `state-audit-on-start` step before super-step phase advance; halt with `halt_reason=state-bound-exceeded` if `v2-state-audit` returns exit 3. New halt class to the cycle 149 enumeration. Estimate: 1 cycle (similar shape to cycle 159 C2 phase-marker addition).
- **v2-state-dispatch-archive tool design scope** (NEW from this document §4.2 Option B): bounded scope for the archive sweep tool. ~200-250 lines design-scope-textual (cycle 158 shape).
- **v2-state-dispatch-archive implementation** (NEW): build the crate, ~1000 LOC + tests (cycle 159 v2-state-audit shape).
- **Backlog archival run** (NEW): one-time invocation of v2-state-dispatch-archive against the 819-merged backlog; expect ~7-week-old merged entries to archive cleanly; preserve `in_flight` + `reviewed_awaiting_eva` + recent-terminal in live `agent_sessions[]`.
- **v2-state-retention-policy.md §4 Axis 6 recalibration patch** (NEW): apply this document's §4.1 recalibrated values once steps 1-4 above land; remove the cycle 159 "forward work" stub from the storage-key clarification.

## 7. What this design scope does NOT do

1. Does NOT modify v1 `record-dispatch` (frozen zone preserved).
2. Does NOT wire v2-state-audit into v2-cycle-runner this cycle (that's cycle 162+; this is design scope only).
3. Does NOT build v2-state-dispatch-archive this cycle (that's cycle 163+).
4. Does NOT run a backlog archival this cycle (that's cycle 164+).
5. Does NOT patch v2-state-retention-policy.md §4 Axis 6 thresholds this cycle (waiting for steps 1-3 to land per §4.3 ordering).
6. Does NOT close cycle 159 priority #5 / #6 from the v2-state-audit live-smoke set — this document refines them and replaces the "v2-state-dispatch-sync refuse-to-write" framing with "v2-state-audit session-start wiring." The renumbered forward list will reflect this.
7. Does NOT consider Option A (transition-time archival) or Option C (in-place compaction) further — both explicitly rejected (4.2).
8. Does NOT propose modifying the dispatch-entry schema (status enum, fields, etc.); recalibration is over thresholds not data shape.
9. Does NOT address any other axis from the cycle 158 policy (axes 1-5 retain cycle 158 thresholds; this scope is Axis 6-specific).
10. Does NOT pre-design the halt-recovery flow for `state-bound-exceeded` (operator-review-required is the cycle 158 stance; cycle 162+ wiring may revisit).
