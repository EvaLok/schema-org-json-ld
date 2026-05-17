# v2-state-dispatch-archive — design scope

**Status:** design scope (cycle 163, 2026-05-17). Tool not yet built.
**Cycle origin:** cycle 163 Track 1 (substantive focal). Closes cycle 161 forward priority #3 (renumbered to cycle 163+ priority #1 in [`cycle-162-state-audit-session-start-wiring-and-dispatch-brief-addendum.md`](cycle-162-state-audit-session-start-wiring-and-dispatch-brief-addendum.md#forward-priorities-for-cycle-163)).
**Predecessor scopes:** [`v2-state-dispatch-policy-enforcement.md`](v2-state-dispatch-policy-enforcement.md) §4.2 Option B + §4.3 step 2; [`v2-state-retention-policy.md`](v2-state-retention-policy.md) §4 Axis 6 + §7; [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md) §3 halt-class catalog.
**Pairs with:** v2-state-audit (cycle 159, in production) as the **archival counterpart** — audit reports, archive acts.

## 1. Problem statement

Cycle 161 design scope [`v2-state-dispatch-policy-enforcement.md`](v2-state-dispatch-policy-enforcement.md) §4.2 selected **Option B** (periodic age-based archival sweep, separate tool) as the recommended approach for bringing the live `agent_sessions[]` population from 930 entries (922 terminal, 3 live, 5 other-status) down to a bounded retention window. The cycle 163 measurement is unchanged: 930 total, 819 merged, 62 failed, 25 closed-without-pr, 15 closed, 6 closed-without-merge, 2 reviewed-awaiting-eva, 1 in-flight.

The current state surface violates v2-state-retention-policy §4 Axis 6 hard threshold (500). Cycle 162 wired v2-state-audit at v2-cycle-runner session-start with halt on `Hard` severity, but the wiring is fail-open against the existing breach — the audit reports `hard` and the runner halts in real execution. The interim observable-but-tolerated stance (cycle 161 §4.3) requires the archival mechanism to land before the policy can be enforced without disabling routine dispatch operations.

This document scopes the archival mechanism. It does NOT build the tool, run the backlog archival, or patch the policy thresholds; those are cycle 164+, 164+ (sequenced after build), and 165+ respectively.

## 2. Tool boundary

`v2-state-dispatch-archive` is a **single-purpose terminal-entry archival tool** for the `agent_sessions[]` axis of `docs/state.json`. It does ONE thing: move terminal entries older than a threshold from the live array to a dated archive file under `docs/state-archive/`.

### 2.1 In scope

- Read `docs/state.json` and select archival-eligible entries from `agent_sessions[]`.
- Write selected entries to a dated archive file at `docs/state-archive/dispatches-<YYYY-MM-DD>.json`.
- Mutate `docs/state.json` to remove archived entries from `agent_sessions[]`.
- Emit a per-invocation summary report (JSON via `--json` or human-readable text).
- Respect a `--dry-run` flag that reports planned action without mutating state.
- Support `--age-days N` to tune the archival cutoff (default 30 days).
- Support `--status <list>` to scope archival to specific terminal statuses (default: all terminal — see §3).

### 2.2 Out of scope (explicit non-doings)

- Does NOT modify `in_flight` or `reviewed_awaiting_eva` entries regardless of age (these are live).
- Does NOT modify entries that lack a timestamp from which age can be derived (see §3.3 fallback).
- Does NOT modify any other axis from v2-state-retention-policy §4 (axes 1-5 retain cycle 158 thresholds with no archival tool yet).
- Does NOT modify any state.json top-level field other than `agent_sessions[]`.
- Does NOT delete archive files once written (archives are append-immutable; pruning archive files is a future operator-action).
- Does NOT call v2-state-audit, dispatch-task, or any other tool (single-purpose discipline).
- Does NOT wire itself into v2-cycle-runner at session-start (the runner integration is a separate cycle; this tool stands alone as a CLI).

## 3. Sweep criteria

### 3.1 Terminal-status whitelist

An entry is **archival-eligible by status** if its `status` field matches one of:

| Status | Lifecycle | Archive? |
|---|---|---|
| `merged` | TERMINAL (success) | yes |
| `failed` | TERMINAL (Copilot pipeline failure) | yes |
| `closed_without_pr` | TERMINAL (closed before PR) | yes |
| `closed` | TERMINAL (closed other) | yes |
| `closed_without_merge` | TERMINAL (PR opened, closed unmerged) | yes |
| `in_flight` | LIVE (Copilot working) | NO |
| `reviewed_awaiting_eva` | LIVE (review pending Eva) | NO |

Unknown status values are treated as ineligible (fail-safe — better to leave an entry live than to misclassify and archive an operationally-active dispatch).

### 3.2 Age threshold

The default age cutoff is **30 days** before invocation time, using these timestamp fields in priority order:

1. `merged_at` (preferred for `merged` status)
2. `dispatched_at` (fallback if `merged_at` is null or absent)

The `--age-days` flag overrides the default. `--age-days 0` archives all terminal entries regardless of age (operator use; not the default).

### 3.3 Entries with missing timestamps

Three `merged` entries in the cycle 163 live state have `merged_at: null`. The tool treats these per §3.2 priority order: if `merged_at` is null/absent, fall back to `dispatched_at`. If both are absent or unparseable, the entry is **NOT archived** — the missing-timestamp condition is reported in the summary so operator review can decide.

Rationale: silently archiving entries we cannot age-verify trades audit-trail clarity for sweep completeness. The wrong trade for a forensic-record tool.

### 3.4 Selection invariant

After §3.1 status filter and §3.2 age filter, an entry IS archival-eligible. The tool processes all eligible entries in a single sweep (no batch size limit by default). A `--max-entries N` flag bounds the sweep when the operator wants progressive archival (e.g., archive the oldest 200 first, observe, then archive more).

## 4. Archive file schema

### 4.1 Filename convention

`docs/state-archive/dispatches-<YYYY-MM-DD>.json` where `<YYYY-MM-DD>` is the **invocation date** (UTC), not the archived entry's date. Multiple sweeps on the same day append to the same file (see §4.3).

### 4.2 Top-level structure

```json
{
  "archive_version": 1,
  "tool_version": "v2-state-dispatch-archive 0.1.0",
  "archived_at": "2026-05-17T01:23:45Z",
  "source_file": "docs/state.json",
  "criteria": {
    "age_days": 30,
    "status_filter": ["merged", "failed", "closed_without_pr", "closed", "closed_without_merge"],
    "invocation_id": "2026-05-17-cycle-N"
  },
  "entries": [ ... archived entries verbatim ... ]
}
```

The `entries` array preserves each archived entry **byte-for-byte** as it appeared in `agent_sessions[]`. No schema mutation, no field stripping, no compression. The audit-trail value depends on this fidelity.

### 4.3 Multiple sweeps per day

If `docs/state-archive/dispatches-<YYYY-MM-DD>.json` already exists when the tool invokes, the new archive is **appended** to the existing `entries` array (not a separate file). The `archived_at` field updates to the latest sweep time; `criteria.invocation_id` becomes an array if multiple sweeps occurred. The single-file-per-day convention keeps the archive directory tidy across operator-driven progressive archival.

### 4.4 Schema migration

`archive_version: 1` codifies the cycle 163 design. Any schema change increments this field; archive readers must check the version and refuse unknown future versions rather than misinterpreting them. Existing archives are never rewritten; readers handle multiple versions in the historical archive directory.

## 5. Invocation contract

### 5.1 CLI

```
v2-state-dispatch-archive [OPTIONS]

OPTIONS:
  --repo-root <PATH>         Repository root (default: ".")
  --age-days <N>             Archival age cutoff in days (default: 30)
  --status <CSV>             Terminal statuses to archive (default: all terminal per §3.1)
  --max-entries <N>          Maximum entries to archive in this invocation (default: unbounded)
  --archive-dir <PATH>       Archive output directory (default: "docs/state-archive")
  --dry-run                  Plan only; no mutation
  --json                     Machine-readable JSON output to stdout
  -h, --help
```

### 5.2 Exit codes

| Code | Meaning |
|---|---|
| 0 | Success — sweep completed (possibly with zero entries archived) |
| 1 | Configuration error (bad flags, missing repo-root, etc.) |
| 2 | Input error (state.json unparseable, archive directory unwritable) |
| 3 | Concurrent-mutation error (state.json changed mid-sweep — see §6) |
| 4 | Partial failure — archive file written but state.json mutation failed |

### 5.3 JSON output schema

```json
{
  "tool": "v2-state-dispatch-archive",
  "tool_version": "0.1.0",
  "invoked_at": "...",
  "dry_run": false,
  "archived_count": N,
  "archive_file": "docs/state-archive/dispatches-2026-05-17.json",
  "ineligible": {
    "live_status": N,
    "ineligible_status": N,
    "below_age_threshold": N,
    "missing_timestamp": N
  },
  "state_after": {
    "agent_sessions_total": N,
    "agent_sessions_live": N,
    "agent_sessions_terminal_retained": N
  }
}
```

## 6. Atomicity and recovery

### 6.1 Ordering of writes

Each invocation performs these steps in order:

1. Read `docs/state.json` (parse fully into memory).
2. Compute archival-eligible set per §3.
3. If `--dry-run`, emit report and exit 0.
4. Write archive file `docs/state-archive/dispatches-<YYYY-MM-DD>.json` (atomic via tempfile + rename).
5. Write mutated `docs/state.json` with archived entries removed (atomic via tempfile + rename).
6. Emit report and exit.

Step 4 BEFORE step 5: the archive file exists before the live state loses entries. If step 5 fails after step 4 succeeds, the archive is on disk and the live state still has the (now-duplicate) entries. Re-running the tool will be a no-op on the entries (they are still in `agent_sessions[]` and now also in the archive); the operator can manually de-duplicate by re-running or by direct edit.

Step 5 BEFORE step 4 would risk losing entries entirely if step 4 fails.

### 6.2 Concurrent-mutation detection

Between step 1 (read) and step 5 (write), other tools (v2-state-dispatch-sync, v1 record-dispatch via Copilot dispatch) may mutate `docs/state.json`. Detection:

- Step 5 re-reads `docs/state.json` and computes a hash of the current `agent_sessions[]`.
- Compare against the hash computed at step 1.
- If hashes differ, abort with exit code 3 — DO NOT write the mutation. The archive file from step 4 remains; the live state is untouched.

This is the same atomic-commit-and-push discipline as PRESERVED-PRIMITIVES git-safety, scaled to a per-file mutation.

### 6.3 Lock file

The tool acquires a lock at `docs/state.json.lock` before step 1. Lock is held until step 5 (or step 3 in dry-run). Another tool finding the lock waits up to N seconds (default 30) then exits with exit code 2. Production cron tools should fail-soft and re-try next cycle; the v2-state-dispatch-archive invocation is gated and can re-fire on a future cycle.

## 7. Ordering vs other v2-state-* tools

### 7.1 Relative to v2-state-audit

v2-state-audit reads `docs/state.json` and classifies. It does not mutate. v2-state-dispatch-archive mutates `docs/state.json`. Ordering:

- Session-start: v2-state-audit runs first (per cycle 162 wiring at v2-cycle-runner pre-flight). If audit reports `mandatory` for Axis 6, the runner emits an advisory recording the recommended archival.
- Archival itself is **not session-start-blocking** under `mandatory` (only `Hard` halts per cycle 162). The operator (or a future automated trigger) invokes v2-state-dispatch-archive separately when convenient.
- After v2-state-dispatch-archive runs, the next session-start v2-state-audit reports the post-archive state.

### 7.2 Relative to v2-state-dispatch-sync

v2-state-dispatch-sync transitions `in_flight` → `merged` / `closed_without_pr` etc. It runs per-cycle and does not delete entries. v2-state-dispatch-archive is independent: it operates on terminal entries, which v2-state-dispatch-sync has already finished writing.

Sequencing: in any cycle where both run, v2-state-dispatch-sync should run first (so transitions are visible to the archive's age computation). Cycle 162 wiring runs v2-state-audit at pre-flight; v2-state-dispatch-sync runs as part of the per-cycle reconciler step (not yet wired); v2-state-dispatch-archive is operator-triggered for the redesign phase. Post-redesign automation may wire archive into a periodic super-step boundary.

### 7.3 Relative to v1 record-dispatch (frozen-zone append driver)

v1 `record-dispatch` appends new entries to `agent_sessions[]` when Copilot dispatches occur. It is unaware of v2-state-dispatch-archive's lock file. The two tools rarely overlap (Copilot dispatches are bounded; archival is rare), but the lock file in §6.3 protects against the rare collision: v1 record-dispatch does NOT take the lock, so the collision protection comes from §6.2's hash check at archive-write time. If v1 record-dispatch appended during the archive sweep, the hash differs and the archive aborts with exit 3, preserving the new append.

## 8. Forward priorities produced by this scope

These extend cycle 162's forward list. Sequencing per cycle 161 §4.3 is preserved.

1. **v2-state-dispatch-archive implementation** (was cycle 162+ priority #2): build the crate per this scope. ~1000 LOC + tests (v2-state-audit shape, cycle 159 precedent). One cycle for skeleton + happy-path tests, possibly one more cycle for atomicity edge cases.
2. **Backlog archival run** (was cycle 162+ priority #3): one-time invocation of v2-state-dispatch-archive against the live 819-merged backlog. Expect ~7-week-old merged entries to archive cleanly. Live + recent-terminal remain in `agent_sessions[]`.
3. **v2-state-retention-policy.md §4 Axis 6 recalibration patch** (was cycle 162+ priority #4): apply cycle 161 §4.1 live+total split thresholds once priorities 1-2 above land. Remove the cycle 159 storage-key-clarification "forward work" stub.

## 9. Anti-patterns this scope rejects

### 9.1 Coupling archive sweep with v2-state-audit

Tempting: have v2-state-audit trigger archive when it reports `mandatory`. Rejected: single-purpose discipline. Audit reports; archive acts. The coupling would re-introduce the cycle 161 §5.1 "wrong owner" anti-pattern (one tool, two jobs).

### 9.2 Archiving live entries

`in_flight` and `reviewed_awaiting_eva` are operationally-live regardless of `dispatched_at` age. Archiving a long-running `in_flight` entry would orphan a Copilot session from the live state surface. The §3.1 whitelist is invariant.

### 9.3 In-place compaction

Rejected in cycle 161 §4.2 Option C and re-rejected here. Audit-trail fidelity is the design priority; per-dispatch issue numbers and PR links must survive archival.

### 9.4 Single-archive-file-forever

Tempting: one `docs/state-archive/dispatches.json` that grows. Rejected: dated files bound the per-file size, support operator-driven sweeps without rewriting the whole archive, and align with the audit-side Step 13.1 dated-archive pattern.

## 10. What this scope does NOT do

1. Does NOT build `v2-state-dispatch-archive` this cycle (forward priority #1 above).
2. Does NOT run a backlog archival this cycle (forward priority #2).
3. Does NOT patch v2-state-retention-policy.md §4 Axis 6 thresholds this cycle (forward priority #3).
4. Does NOT modify v1 `record-dispatch` (frozen zone preserved).
5. Does NOT wire v2-state-dispatch-archive into v2-cycle-runner this cycle (the runner integration is a forward-future cycle; this scope stands the tool up as a CLI).
6. Does NOT design archive-file pruning or compaction (archives are append-immutable per §2.2).
7. Does NOT extend archival to axes 1-5 of v2-state-retention-policy (axis-by-axis archival is per-primitive forward work; this scope is Axis 6-specific).
8. Does NOT modify the dispatch-entry schema or rename `agent_sessions` (key rename is a future v2 migration).
9. Does NOT pre-design the automated trigger surface (operator-driven during redesign; cron/super-step-wired trigger is post-cutover work).
10. Does NOT address the cycle 159 storage-key-clarification's forward-work patch (priority #3 above will).

## 11. References

- Cycle 161 design scope §4.2 Option B + §4.3: [`v2-state-dispatch-policy-enforcement.md`](v2-state-dispatch-policy-enforcement.md).
- Cycle 158 retention policy §4 Axis 6 + §7: [`v2-state-retention-policy.md`](v2-state-retention-policy.md).
- Cycle 162 v2-state-audit session-start wiring (halt-class catalog 5th entry `state-bound-exceeded`): [`cycle-162-state-audit-session-start-wiring-and-dispatch-brief-addendum.md`](cycle-162-state-audit-session-start-wiring-and-dispatch-brief-addendum.md).
- Cycle 159 v2-state-audit implementation precedent (1260 LOC reference): `tools/rust/crates/v2-state-audit/`.
- Cycle 149 halt-class catalog (4 v1 classes + cycle 162 5th `state-bound-exceeded`): [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md) §3.3.
- Cycle 524 atomic commit-and-push fix (PRESERVED-PRIMITIVE git-safety reference, model for §6.1 ordering): orchestrator prompt §PRESERVED-PRIMITIVES.
- Audit cycle 215 Step 13.1 archival pattern (dated-archive precedent, audit-side mirror): [`docs/redesign/0-audit-retrospective.md`](https://github.com/EvaLok/schema-org-json-ld-audit/blob/master/docs/redesign/0-audit-retrospective.md) at audit commit [`8285b7d3`](https://github.com/EvaLok/schema-org-json-ld-audit/commit/8285b7d3).
