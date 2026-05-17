# v2-cycle-runner status + verify subcommand design

**Status:** design-scope (cycle 168); implementation deferred to cycle 169+.

**Provenance:** Carry-forward from `cycle-149-v2-cycle-runner-design-scope.md` Sections 4 + 9.9 (high-level only). Cycle 151 landed `run`; both `status` and `verify` were named-but-deferred. The cycle 156-167 forward-priority list listed `status + verify v2-cycle-runner subcommands` for 12 consecutive cycles (157→167). This design sharpens cycle 149's high-level naming into concrete output schemas + verify algorithm + open-question resolutions, enabling implementation cycle 169+ to ship without re-deriving design.

**Scope relative to cycle 149 design:** cycle 149 §4 named the CLI surface and §9.2 / §9.9 opened two questions. This document resolves those questions, specifies output schemas, and defines the verify algorithm.

## 1. Problem statement

### 1.1 What the runner already does

`v2-cycle-runner run --cycle N --issue M [--dry-run] [--halt-after-role <role>]` (landed cycle 151; cycle 162 added pre-flight `v2-state-audit` integration; cycle 163 added super-step-out-of-order live test) drives the 10-step super-step sequence and writes `state/v2-cycle-runner/last-cycle.json` + appends `state/v2-cycle-runner/cycle-history.json`.

The runner already produces a `CycleReport` per cycle with status (`completed` / `halted` / `dry-run-traced` / `out-of-order`), traces per step, halt-class, halt_step, halted_after_role, steps_attempted, and state-audit outcome. The runner's own state files contain everything needed to answer "what's the runner's state?" and "did cycle N complete cleanly?" The two subcommands are read-side surfaces over already-written data.

### 1.2 Why status + verify are needed

- **status**: ops visibility. Operators (Eva, dispatched-critique sessions, audit sessions) need a quick "what cycle, what status, what halted" view without having to parse cycle-history.json directly. Today: no such surface.
- **verify**: post-cycle self-check. Per cycle 149 §9.9, main-side needs a fast self-verification before declaring cycle N clean; audit-side runs adversarial / independent verification separately. Both layers are useful.

The two are read-only surfaces with no mutation; both can land in the same implementation cycle.

### 1.3 What's NOT in cycle 168 scope

- Does NOT change `run` semantics.
- Does NOT modify state file shapes (status + verify consume existing schemas).
- Does NOT add audit-side counterpart (separate tool; out of scope).

## 2. Tool boundary

### 2.1 status subcommand

**Question answered:** "What is the runner's most recent or specified cycle's execution state?"

**CLI:**
```
v2-cycle-runner status [--cycle N] [--include-primitives] [--format text|json]
```

- Default (no `--cycle`): read `state/v2-cycle-runner/last-cycle.json`. Report the cycle's number, issue, status, timestamps, halt-step (if any), state-audit outcome.
- `--cycle N`: read `state/v2-cycle-runner/cycle-history.json`; find entry for cycle N. Error if cycle N has no entry.
- `--include-primitives`: additionally read the 4 primitive state surfaces (`super-step.json`, `roles/*.json`, `reconciler/poll-history.json`) for richer context. Slower; default off.

**Output (text, default):**
```
v2-cycle-runner status: cycle=168 issue=2981 status=completed
  started:  2026-05-17T10:27:00Z
  ended:    2026-05-17T11:42:00Z
  steps:    10 attempted, 10 traced
  halt:     (none)
  audit:    ok
```

**Output (JSON):**
```json
{
  "schema_version": "v1",
  "subcommand": "status",
  "cycle": 168,
  "issue": 2981,
  "status": "completed",
  "started_at": "2026-05-17T10:27:00Z",
  "ended_at": "2026-05-17T11:42:00Z",
  "steps_attempted": 10,
  "halt_step": null,
  "halt_class": null,
  "halted_after_role": null,
  "state_audit": "ok",
  "include_primitives": false
}
```

With `--include-primitives` the JSON adds:
```json
{
  "primitives": {
    "super_step": {"current_role": null, "last_settled_cycle": 168},
    "reconciler": {"last_poll_cycle": 168, "input_from_eva_cursor": "...", "audit_repo_cursor": "..."},
    "roles": {"reconciler_last_run": "...", "planner_last_run": "...", "executor_last_run": "...", "curator_last_run": "..."}
  }
}
```

### 2.2 verify subcommand

**Question answered:** "Did cycle N complete cleanly per the runner's expectations?"

**CLI:**
```
v2-cycle-runner verify --cycle N [--strict] [--format text|json]
```

- `--cycle N` is **required** — explicit cycle naming prevents accidental "verify latest" passing when latest hasn't even started.
- `--strict` exits non-zero on any assertion failure (default: exits 0 with failure-reported JSON).
- `--format` controls output shape.

**Assertion list (the verify algorithm):**

1. **cycle-N-in-history**: `cycle-history.json` contains an entry with `cycle == N`. (Required precondition; other assertions don't run if this fails.)
2. **status-completed**: entry's `status` field is `"completed"`. Fails for `"halted"`, `"dry-run-traced"`, `"out-of-order"`.
3. **no-halt-step**: entry's `halt_step` is null. (Redundant with #2 in well-formed cycles; included for defense-in-depth.)
4. **no-halt-class**: entry's `halt_class` is null.
5. **no-halted-after-role**: entry's `halted_after_role` is null.
6. **all-10-substeps-traced**: entry's `traces` array has 10 entries, all with `executed == true`. (Cycle 156 C5 fix wired `executed: bool` into StepTrace; this assertion checks that wiring landed.)
7. **state-audit-not-hard**: entry's `state_audit` is either null (dry-run) or has severity ≠ `Hard`.
8. **super-step-history-has-cycle-N**: `state/super-step-history.json` contains a settled entry for cycle N.
9. **per-role-history-has-cycle-N**: each of `state/roles/{reconciler,planner,executor,curator}-history.json` contains an entry for cycle N.

If all 9 assertions pass: `verdict: "clean"`. If any fail: `verdict: "dirty"`.

**Output (text, on clean cycle):**
```
v2-cycle-runner verify --cycle 168: verdict=clean (9/9 assertions passed)
  ✓ cycle-N-in-history
  ✓ status-completed
  ✓ no-halt-step
  ✓ no-halt-class
  ✓ no-halted-after-role
  ✓ all-10-substeps-traced
  ✓ state-audit-not-hard
  ✓ super-step-history-has-cycle-N
  ✓ per-role-history-has-cycle-N
```

**Output (JSON):**
```json
{
  "schema_version": "v1",
  "subcommand": "verify",
  "cycle": 168,
  "verdict": "clean",
  "assertions": [
    {"name": "cycle-N-in-history", "passed": true},
    {"name": "status-completed", "passed": true},
    {"name": "no-halt-step", "passed": true},
    {"name": "no-halt-class", "passed": true},
    {"name": "no-halted-after-role", "passed": true},
    {"name": "all-10-substeps-traced", "passed": true},
    {"name": "state-audit-not-hard", "passed": true},
    {"name": "super-step-history-has-cycle-N", "passed": true},
    {"name": "per-role-history-has-cycle-N", "passed": true}
  ],
  "exit_code": 0
}
```

**Failure output (e.g., halted cycle):**
```json
{
  "verdict": "dirty",
  "assertions": [
    {"name": "cycle-N-in-history", "passed": true},
    {"name": "status-completed", "passed": false, "details": "status is 'halted', expected 'completed'"},
    {"name": "no-halt-step", "passed": false, "details": "halt_step is 'executor-session'"},
    ...
  ],
  "exit_code": 1
}
```

## 3. Resolving cycle 149 open questions

### 3.1 §9.2 — Primitive discovery (RESOLVED)

Cycle 149 named three options: PATH lookup, `${CARGO_TARGET_DIR}` resolution, or explicit `--primitive-bin <path>` flag.

**Decision:** PATH lookup is the default for production. The `run` subcommand already accepts `--primitive-bin` overrides per primitive (current code at `tools/rust/crates/v2-cycle-runner/src/main.rs:55-65`); the `status --include-primitives` and `verify` paths inherit the same resolution. CARGO_TARGET_DIR resolution is a fallback path the existing helper `default_*_bin()` functions handle.

Verify does NOT shell to primitive binaries — it reads their state files directly. So primitive discovery is ONLY relevant for `status --include-primitives` (and even there, only if we choose to expose primitive subcommands like `super-step status`; current design just reads the primitives' on-disk state).

Conclusion: **no new infrastructure needed**; existing primitive-discovery layer is reused.

### 3.2 §9.9 — verify in-binary vs separate audit-side tool (RESOLVED)

Cycle 149 leaned in-binary; this design confirms. Rationale:

- **Different consumers, different needs.** Main-side `verify` runs at cycle-close to confirm the cycle's own writes are well-formed before any commit. Audit-side verification runs adversarially, looking for cross-cycle pattern violations, state-shape drift, regressions main-side missed. Different scopes, different cadences, different trust models.
- **Speed matters at cycle-close.** Audit-side runs ~hours later (cron-driven, cross-repo). Main-side needs sub-second feedback; in-binary verify reads pre-written state and asserts.
- **Audit's independence is the point.** If verify were outsourced to audit, main-side would block on audit's cron; coupling defeats audit-as-adversarial-peer.

Audit-side counterpart is **out of scope** for this design and not blocking. If/when audit-side wants a `v2-cycle-runner-audit-verify` tool, it owns its own design.

## 4. State files consumed

| Subcommand | File | Owner | Read pattern |
|---|---|---|---|
| status (default) | `state/v2-cycle-runner/last-cycle.json` | v2-cycle-runner | Single read |
| status --cycle N | `state/v2-cycle-runner/cycle-history.json` | v2-cycle-runner | Linear scan for cycle == N |
| status --include-primitives | `state/super-step.json`, `state/roles/*.json`, `state/reconciler/poll-history.json` | various primitives | Reads only |
| verify | All of the above + `state/super-step-history.json` | various | Reads only |

The verify subcommand is the first consumer of multiple primitive state surfaces in a single tool invocation. The reads are independent — no cross-state consistency assertions (yet); failures of any individual read surface as the corresponding assertion failure.

## 5. Test plan

### 5.1 Unit tests (per-module)

**status:**
- `status_default_reads_last_cycle_completed`: fixture has `last-cycle.json` with `status=completed`; expect text+JSON output naming the cycle.
- `status_default_errors_on_missing_state_file`: empty `state/v2-cycle-runner/` directory; expect clear error message.
- `status_cycle_n_reads_history`: fixture has `cycle-history.json` with cycle 5; `status --cycle 5` reports it.
- `status_cycle_n_errors_on_unknown_cycle`: `cycle-history.json` has cycles 1-3; `status --cycle 9` errors.
- `status_halted_cycle_reports_halt_step`: fixture has `status=halted`, `halt_step=executor-session`; output names both.
- `status_include_primitives_flag_no_crash_on_missing_primitives`: `--include-primitives` with no primitive state files should degrade gracefully (report nulls), not crash.

**verify:**
- `verify_clean_cycle_passes_all_9_assertions`: fixture with all state files well-formed for cycle 5; verdict=clean.
- `verify_halted_cycle_fails_status_no_halt`: fixture has cycle 5 with status=halted, halt_step set; verdict=dirty; assertions 2 and 3 both fail.
- `verify_missing_substeps_fails_traces_assertion`: fixture has cycle 5 with only 7 traces; assertion 6 fails.
- `verify_unknown_cycle_errors_first_assertion`: cycle N not in history; assertion 1 fails; downstream assertions short-circuit.
- `verify_strict_exits_1_on_any_failure`: dirty cycle + `--strict`; verify exit code = 1.
- `verify_non_strict_exits_0_with_dirty_verdict`: dirty cycle without `--strict`; exit code = 0 (failure reported in JSON).
- `verify_missing_super_step_history_fails_assertion_8`: super-step-history.json missing entry for cycle N; assertion 8 fails.
- `verify_missing_role_history_fails_assertion_9`: any of the 4 role-history files missing cycle N; assertion 9 fails.
- `verify_state_audit_hard_severity_fails_assertion_7`: state_audit set with severity=Hard; assertion 7 fails.

### 5.2 Integration test

`tests/integration_status_verify.rs`:

1. Initialize a temp dir + run a `v2-cycle-runner run --cycle 1 --issue 99999 --dry-run` (already covered by existing integration test; reused).
2. After completion, invoke `status` → expect completed report.
3. Invoke `verify --cycle 1` → expect verdict=clean.
4. Inject a deliberate corruption (e.g., overwrite `cycle-history.json` to set status=halted) and re-invoke verify → expect dirty.

## 6. Open questions for implementation (cycle 169+)

1. **`status --all` mode**: would a "list every cycle" sweep be useful? Likely YES once cycle-history accumulates. Bound: O(history length). Defer to a follow-up cycle if verify implementation reveals it's a natural extension.
2. **`verify --all` sweep**: assert every cycle in history is clean. Useful for periodic regression-check. Probably needed by audit-side eventually. Defer.
3. **Orphan-state detection**: should verify check that no state-files reference cycles NOT in `cycle-history.json`? Useful for detecting half-finished runs (cycle ran but cycle-runner never wrote to its own history). Defer to a separate `v2-cycle-runner reconcile` subcommand if surfaced as a real need.
4. **Color in text output**: ✓ vs ✗ vs colored? Lean ASCII-only for log-friendly output; CI-color flag for terminals.
5. **Schema-version stamping**: `schema_version: "v1"` on every JSON output. When/how does v2 schema arise? Plausibly when status/verify output shape changes incompatibly; lean to delay v2 until the need is real.
6. **Should verify ALSO check commit-discipline (push topology)?**: NO. Cycle 161 X5 carveout (commit-discipline-check is a separate sibling tool). Verify is strictly state-file-integrity.

## 7. What this design does NOT do

- Does NOT implement the subcommands (implementation = cycle 169+).
- Does NOT modify `run`, `init`, or `schema` subcommands.
- Does NOT change state file shapes (status + verify consume the existing `CycleReport` / per-primitive state surfaces).
- Does NOT add CI gates (workflow YAML edits are a separate cutover-adjacent PR).
- Does NOT define audit-side counterpart (out of scope per §3.2).
- Does NOT extend to multi-cycle correlation checks (`verify --all`, `--reconcile`); listed as open questions for follow-up.
- Does NOT touch `tools/cycle-runner/` (v1, forbidden zone per redesign-prompt direct-push-zones).
- Does NOT bundle `v2-state-audit` invocations into verify (audit already runs at session-start during `run`; verify just reads the recorded outcome).

## 8. Stale-doc-string follow-up

`tools/rust/crates/v2-cycle-runner/src/main.rs:413-416` still says:
```
Subcommands DEFERRED to cycle 151-153+:
  run      (the main entrypoint: drives the 10-step sequence)
  status   (current super-step + per-role timestamps)
  verify   (post-cycle: all transitions present + clean)
```

`run` has been implemented since cycle 151. The doc string is stale. The implementation cycle 169+ should update this string as part of the status+verify landing PR (move `run` to "implemented" list; remove status+verify from "deferred" list since they now exist).

## 9. Closure

This design scope closes cycle 168+ forward priority #3 (`status + verify v2-cycle-runner subcommands`) at the design-scope layer. The carry-forward thread runs cycle 149 (high-level naming) → cycles 150-167 (no progress) → cycle 168 (this design scope) → cycle 169+ (implementation). Implementation effort estimated as ~250-400 LOC + tests; one cycle of focused work likely sufficient.
