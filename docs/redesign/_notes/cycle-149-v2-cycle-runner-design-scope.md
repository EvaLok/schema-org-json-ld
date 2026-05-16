---
cycle: 149
date: 2026-05-15
focus: design scope for v2-cycle-runner — the harness that composes the 4 multi-agent primitives + 4 role prompts into a complete cycle execution
forward-priority-honored: cycle 148 #3 (cycle-runner harness rewrite design scope) — Track 1 substantive focal
status: DESIGN SCOPE ONLY — implementation deferred to cycle 150+; this document opens the multi-cycle arc per cycle 148 priorities #2 (first end-to-end run) gating on this rewrite
---

# v2-cycle-runner — Design Scope

## 1. Motivation

The current `tools/rust/crates/cycle-runner` (4690 LOC across `main.rs` / `startup.rs` / `close_out.rs` / `runner.rs` / `steps.rs` / `git.rs` / `review_body.rs`) drives the v1 single-orchestrator pipeline: `startup` runs cycle-start + pipeline-check + housekeeping + status; `close-out` runs C4.1 through C8. Two CLI subcommands; one orchestrator-session model.

Phase 3's Q7-resolved architecture is multi-agent role-specialization (Candidate B, Eva-approved 2026-05-13 per `#2930`). Cycles 140-145 landed the 4 multi-agent topology primitives (`v2-channel-router` 844 LOC, `v2-super-step-boundary` 888 LOC, `v2-role-driver` 1329 LOC, `v2-reconciler-event-processor` 1398 LOC = 4459 LOC) and the 4 role prompts (`planner` 566, `reconciler` 505, `executor` 500, `curator` 699 = 2270 LOC). Cycle 148 landed `v2-prompt-contract-check` (1136 LOC) verifying prompt↔router alignment. **Total v2 substrate now 7865 LOC across 5 crates + 4 prompts** — but nothing currently composes them into an end-to-end cycle execution.

The cycle-runner-rewrite is the conductor that makes the 5-crate / 4-prompt substrate executable as a single cron-driven cycle. Per cycle 148 forward priority #2 ("first end-to-end run + first measurement"), this rewrite is the **gating dependency for all runtime measurement of the v2 architecture**.

## 2. Architectural choice: parallel crate, not in-place rewrite

Two paths:

**Option A — Modify existing `cycle-runner` in place.** Replaces startup + close-out with new multi-agent execution logic in the same binary. The workflow YAML continues to invoke `cycle-runner` and gets v2 behavior automatically.

**Option B — Build new `tools/rust/crates/v2-cycle-runner` parallel crate.** Existing `cycle-runner` untouched and continues to operate for v1 if reverted. Cutover is a workflow-YAML edit (single PR) when v2 is ready.

**Choose Option B.** Reasons:

1. **Direct-push-zone authorization.** `cycle-runner` is forbidden zone per redesign prompt SECTION 2 (current production tools); `tools/rust/crates/v2-*` is direct-push-zone. Option B lets implementation iterate at the same friction level as the 4 v2 primitives that have already landed.
2. **Rollback isolation.** If v2-cycle-runner has a regression in early operation, reverting is a workflow-YAML revert PR, not a tool-internals revert. Per redesign prompt SECTION 1 Phase 4: "v1 retained for ~30 cycles minimum after cutover before deletion is considered" — Option B is the only way to honor that.
3. **Parallel-run capability.** Option B allows running v2-cycle-runner on a side workflow against the same repo before main cutover; Option A forces all-or-nothing.
4. **Mental-model clarity.** The 4 v2 primitives plus v2-prompt-contract-check are all named `v2-*`; calling the conductor `v2-cycle-runner` keeps the convention.

Trade-off accepted: ~120 LOC of `cycle-runner/src/main.rs` CLI scaffolding (subcommand parsing, error handling) will be duplicated. Cost is small and bounded.

## 3. Execution model

### 3.1 Per-cycle super-step sequence

The v2 cycle execution is a single super-step that drives 4 role sessions in fixed order: **reconciler → planner → executor → curator**. (Cycles 140-145 set this ordering; see cycle 142 `_notes/cycle-142-role-driver-scaffold.md` for the rationale. The prompt-contract-check tool's writer-map mirrors it.)

```
cycle-N entry
  │
  ▼
[super-step-init]          v2-super-step-boundary begin --cycle N
  │                          ⇒ creates state/super-step.json (transitioning state)
  ▼
[reconciler-pre-poll]      v2-reconciler-event-processor poll --cycle N
  │                          ⇒ writes state/reconciler/cursors/* + inbound-channel
  │                          (NOTE: this PRE-poll runs BEFORE reconciler role-session
  │                           because reconciler's input IS the polled artifacts;
  │                           naming is currently `reconciler-event-processor` but
  │                           its role is the data-collection stage, distinct from
  │                           the reconciler role-prompt session that follows.)
  ▼
[reconciler-session]       v2-role-driver drive --role reconciler --cycle N
  │                          ⇒ invokes reconciler-prompt.xml with inbound-channel
  │                            state; writes to v2-channel-router for inbound-channel
  │                            payload + per-role-tasks state for next role
  ▼
[super-step-advance-1]     v2-super-step-boundary advance --to planner --cycle N
  │                          ⇒ verifies reconciler outputs present in channels
  ▼
[planner-session]          v2-role-driver drive --role planner --cycle N
  │                          ⇒ invokes planner-prompt.xml; writes plan-channel
  ▼
[super-step-advance-2]     v2-super-step-boundary advance --to executor --cycle N
  ▼
[executor-session]         v2-role-driver drive --role executor --cycle N
  │                          ⇒ invokes executor-prompt.xml; writes work-channel
  │                            + dispatch-firing side channel (per #2937 authorized)
  ▼
[super-step-advance-3]     v2-super-step-boundary advance --to curator --cycle N
  ▼
[curator-session]          v2-role-driver drive --role curator --cycle N
  │                          ⇒ invokes curator-prompt.xml; writes memory-channel
  │                            + cycle-close artifacts (journal, _notes, issue close)
  ▼
[super-step-settle]        v2-super-step-boundary settle --cycle N
  │                          ⇒ moves super-step.json to settled state, appends
  │                            super-step-history.json entry
  ▼
cycle-N exit (clean)
```

Each step shells out to an existing v2-* primitive. The v2-cycle-runner does NOT re-implement any primitive's logic — it sequences them and surfaces failures.

### 3.2 Sub-second-grained durability

After each step, the runner ensures the state-file mutations from the step have been pushed to git before the next step begins. This honors PRESERVED-PRIMITIVES rule git-safety (the cycle 524 corruption-class fix): "Every commit MUST be pushed in the same operation. Local-only commits to state-of-record files are forbidden."

Concretely: each role-session step commits its own state mutations via the primitive it shells out to, then v2-cycle-runner pushes after each `advance` step (or alternatively, the primitives push themselves; the runner just verifies push-cleanliness before advancing).

### 3.3 Failure mode taxonomy

The runner classifies failures into four categories with distinct responses:

| Class | Cause | Runner response |
|---|---|---|
| **transient-tool-error** | Network glitch, gh-rate-limit, ephemeral cargo build fail | Retry once with backoff; on 2nd failure, surface as cycle-error |
| **role-session-empty-output** | Role-driver returns 0 bytes for a role (A4 silent-fail) | Halt super-step; commit a halt marker to super-step-history; cycle ends in `halted-at-role-N` state; next cycle's reconciler will see the halt and treat as inbound |
| **channel-write-rejected** | v2-channel-router rejects payload (missing required key, invalid object) | Halt super-step; commit reject diagnostic; treat like role-session-empty-output |
| **super-step-out-of-order** | Boundary tool refuses transition (reconciler→executor without going through planner) | Hard error; abort cycle without super-step state-machine mutation; runner exits non-zero. The runner MAY persist runner-local observability state (e.g. `state/v2-cycle-runner/last-cycle.json` postmortem record) — that is not "state mutation" in the contractual sense, which refers strictly to super-step state-machine state (`state/super-step.json`, channel state envelopes, role cursors). |

The runner does NOT attempt to recover from class 2-4 within the cycle — it halts cleanly and lets the next cycle's reconciler observe the halt state via inbound-channel. This matches PR #2951 critique L1.3 / L3.6 (reconciler must emit completeness metadata) and is more honest than mid-cycle retry.

**Scope of "state mutation" (cycle 160 clarification, from PR #2951 critique C10 / L2.5 paired ACT-NOW + CARVEOUT verdict in cycle 155 absorption):** the SuperStepOutOfOrder row's "abort cycle without state mutation" applies to super-step state-machine state — the channel state envelopes under `state/channels/`, the super-step status file `state/super-step.json`, the reconciler cursors under `state/reconciler/`, and any per-role artifacts the role driver writes through the channel-router. It does NOT forbid the runner from writing its own local observability state (`state/v2-cycle-runner/last-cycle.json`) recording that an out-of-order halt occurred — that record is a postmortem aid, not contractual machine state, and the boundary tool owns the contractual surface anyway (the runner never directly mutates `state/super-step.json`). Cycle 160 preserves the existing `halt_cycle` implementation that writes `last-cycle.json` for ALL halt classes including out-of-order; this clarification makes the design contract consistent with that behavior rather than the other way around.

### 3.4 No cargo on non-Rust-PR cycles — clarified

Cycle 133 lesson "no cargo on non-Rust-PR cycles" applies to gratuitous cargo invocations during orchestration. The v2-cycle-runner does NOT shell to cargo during cycle execution. The 4 v2 primitives are pre-built binaries (CI builds them); the runner invokes the binaries, not `cargo run`. Building the primitives at cycle-time would (a) be slow, (b) make the runner sensitive to dependency network freshness, (c) violate cycle 133.

The runner's `Cargo.toml` declares the v2 primitives as `[[bin]]` workspace siblings; the v2-cycle-runner finds them via `${CARGO_TARGET_DIR}/debug/v2-*` or via PATH lookup (TBD design choice; see open question 9.2).

## 4. CLI surface

```
v2-cycle-runner run --cycle N --issue M [--dry-run] [--halt-after-role <role>]
v2-cycle-runner status [--cycle N]
v2-cycle-runner verify --cycle N   # post-cycle: assert all 4 super-step transitions present + clean
v2-cycle-runner init   # one-shot: creates state/channels/, state/super-step/, state/roles/, state/reconciler/
```

Subcommand specifics:

- **run**: the main entrypoint. The workflow YAML's invocation. `--cycle N` is the cycle number; `--issue M` is the orchestrator-run cycle issue number; `--dry-run` traces the sequence without state mutation (useful for design validation); `--halt-after-role <role>` short-circuits after a named role for debugging.
- **status**: reports current super-step state and per-role last-run timestamps.
- **verify**: post-cycle sanity check — used by audit / cron-side workflows to confirm cycle-N completed all super-step transitions cleanly.
- **init**: idempotent state initialization. Composes the four primitive-side `init` commands.

## 5. State model

The runner reads/writes the following state files (all paths relative to repo root):

```
state/
  channels/                  ← v2-channel-router owns
    plan-channel.json
    work-channel.json
    inbound-channel.json
    memory-channel.json
    history/                 ← append-only per-channel history
  super-step.json            ← v2-super-step-boundary owns (current transitioning state)
  super-step-history.json    ← v2-super-step-boundary owns (append-only per-cycle)
  roles/                     ← v2-role-driver owns
    reconciler-history.json
    planner-history.json
    executor-history.json
    curator-history.json
  reconciler/                ← v2-reconciler-event-processor owns
    cursors/
      input-from-eva.json
      audit-repo.json
      dispatch-returns.json
    poll-history.json
  v2-cycle-runner/           ← v2-cycle-runner owns
    last-cycle.json          ← cycle number + status + timestamps + halt reason if any
    cycle-history.json       ← append-only per-cycle summary
```

The runner's own state is intentionally minimal: just enough to answer "what cycle number, what status, what halted (if anything)" without re-reading the 4 primitive state surfaces. The detailed state lives where it's owned. This is single-writer-per-state-file discipline (consistent with single-writer-per-channel in v2-channel-router).

## 6. Workflow integration

Current production workflow at `.github/workflows/claude.yml` (TBD: confirm exact filename) invokes `tools/cycle-runner` wrapper that calls `cycle-runner`. Cutover requires editing this YAML to invoke `v2-cycle-runner run --cycle N --issue M` instead.

**The cutover PR is the only forbidden-zone change** required for v2 activation. v2-cycle-runner crate itself, its tests, its tools/cycle-runner wrapper replacement — all direct-push-zone. The YAML edit is a small, isolated PR.

Migration plan (sketch; details in Phase 4 cutover plan, which is the next-but-one milestone):

1. Build v2-cycle-runner crate to working state (cycles 150-152 estimated).
2. Local + CI dry-run validation against synthetic state.
3. Parallel-run window: run v2-cycle-runner manually against the same orchestrator-issue stream as v1, comparing outputs, for ~5-10 cycles.
4. Cutover PR: workflow YAML edit + Eva approval (one of the three CHECKPOINTS — pre-cutover).
5. Post-cutover observation window: v1 cycle-runner retained on disk for ~30 cycles minimum (PRESERVED-PRIMITIVES guidance for v1 retention).

## 7. Test plan

Three layers:

### 7.1 Unit tests (per-module)

- Failure-classification table: each of the 4 failure classes is covered by a test asserting the runner classifies correctly and the response (retry / halt / hard-error) is selected.
- Sub-step sequencing: a fake-primitives test using mocks (or shell stubs) confirms the 9-step sequence executes in exact order.
- State-file write atomicity: a partial-failure injection (simulated kill mid-step) confirms the runner re-orients to the right resume point on next invocation.

### 7.2 Integration test against real primitives

A `tests/integration_cycle.rs` test that:

1. Creates a temp dir with `state/` initialized.
2. Invokes `v2-cycle-runner run --cycle 1 --issue 99999 --dry-run` against real v2 primitive binaries (built by `cargo test`'s setup).
3. Asserts the dry-run trace matches the expected 9-step sequence with each primitive's CLI surface invoked.
4. Asserts no state mutation occurred under `--dry-run`.

A second integration test runs `--cycle 1 --issue 99999` WITHOUT `--dry-run` against a temp fixture, asserts all state files are present and well-formed at exit, asserts super-step-history has one entry with clean-settled status, and asserts no halt marker present.

### 7.3 End-to-end smoke (the "first end-to-end run" milestone)

The actual cycle 152-ish target: run v2-cycle-runner against the real repo state in a side branch / forked workflow, observe one full cycle execution from cron trigger to issue close. This is the **first measurement opportunity** for the v2 architecture. Measurement targets per cycle 148 forward priority #2:

- Per-role token usage
- Per-role wall-clock time
- Per-step state-file size delta
- Halt-rate (cycles that halt at any super-step)
- Channel-write-reject-rate (semantic alignment fidelity)
- Side-channel-leak-rate (dispatches fired outside reducer-governed coordination, per L1.2 X4)

These measurements feed back into the AGREE-DEFER queue from PR #2951 (L2.2 curator complexity, L3.5 curator decomposition, L2.3 / C2 template-mirror architecture-revisit) — the deferral was contingent on runtime evidence.

## 8. Risk register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Primitive CLI APIs drift between scope and implementation | medium | high | Implementation cycle 150 begins with re-reading each primitive's CLI; this design names subcommands at a high-level only |
| v2-role-driver does not yet support all 4 roles via single CLI | unknown | high | Verify cycle 150 entry; if missing, the gap is implementation-cycle-1 work |
| State path conventions clash with v1 (writes both paths) | low | medium | v2 uses `state/` (Cargo-style); v1 uses different paths per `cycle-runner/src/close_out.rs:*`; check and document |
| Cron timing — full cycle exceeds workflow timeout | low | high | Workflow timeout is currently TBD; if 4 role sessions × ~3 min each + tooling ≈ 15+ min, may need workflow timeout extension; profiled in cycle 152 smoke test |
| Halt-state ambiguity — what does next cycle do? | medium | medium | Design assumes reconciler observes halt-state via inbound-channel and treats as inbound; if reconciler-prompt doesn't recognize halt-state inputs, cycle 150+ work to teach it |
| Cycle 524 corruption-class regression — local commits not pushed | low | critical | Each step pushes before runner advances; verification step before advance checks `git status` clean |

## 9. Open questions for cycle 150+ implementation

1. **Primitive discovery mechanism**: PATH lookup vs `${CARGO_TARGET_DIR}` vs explicit `--primitive-bin <path>` flag? PATH is most flexible but requires workflow setup; explicit-path is most robust; CARGO_TARGET_DIR is most ergonomic for local dev. Lean: PATH with `${CARGO_TARGET_DIR}/debug:${CARGO_TARGET_DIR}/release` prepended automatically.
2. **Role-session driver: which orchestrator session model**: v2-role-driver currently scaffolds role sessions; the actual role-session execution may need to spawn an Anthropic-API call (per the curator/planner/etc. prompt expects to be invoked AS a Claude session). The runner needs to know the API-key handling, model choice, max-tokens, etc. — this might be v2-role-driver's responsibility internally, or it might surface to v2-cycle-runner. Check cycle 142 design.
3. **Dispatch-firing side-channel**: per PR #2951 L1.2 + X4, executor-session may fire Copilot dispatches. The runner doesn't orchestrate dispatches — it lets executor-session do that internally via `tools/dispatch-task`. Does this require the executor's session to have GH_TOKEN context? Probably yes; verify the role-driver passes through the right env vars.
4. **Halt-recovery semantics**: if cycle N halts at executor-session, cycle N+1's reconciler observes inbound-channel with halt marker. Does the planner then re-plan from where executor left off, or skip to curator with a documented-incomplete-cycle? Design choice that affects role-prompt input shape — defer to cycle 150 implementation entry.
5. **Per-cycle workflow YAML invocation**: today's claude.yml invokes the orchestrator on issue events. v2-cycle-runner could either be: (a) called BY the orchestrator (orchestrator decides when to step super-steps), or (b) called BEFORE the orchestrator (runner sequences the role sessions; orchestrator is the wrapper). Lean (b): orchestrator-as-conductor is v1; v2 inverts — runner drives, role-prompts ARE the sessions. This is a structural choice and probably needs Eva input at the pre-cutover checkpoint.
6. **Token budget**: each role-session is a Claude session; 4 per cycle × current cron cadence (~4 cycles/day) = 16 sessions/day, vs v1's 4 (one per cycle). Cost impact significant; needs Eva visibility but no Eva-only decision required (the architecture choice was Q7-resolved).
7. **--halt-after-role debugging surface**: useful for local development; should it be available in the production workflow? Lean: yes but only with an env-var gate to prevent accidental halt-after-role in production.
8. **Integration test against real primitives — built-where**: integration test relies on v2-* binaries being built. The test should `cargo build -p v2-channel-router -p v2-super-step-boundary ...` as setup, OR depend on CI to have already built them. Lean: built-by-test-setup (slower but hermetic).
9. **Verify subcommand vs separate audit-side tool**: `v2-cycle-runner verify --cycle N` overlaps with audit's role. Is verify in-binary or outsourced to audit? Design choice — lean in-binary because main-side needs self-verification before declaring a cycle clean, and audit's verification is independent + adversarial; both layers are useful.

## 10. What this design scope does NOT do

- Does NOT implement v2-cycle-runner this cycle.
- Does NOT modify cycle-runner (forbidden zone; preserved during Phase 3).
- Does NOT modify `.github/workflows/` (cutover PR is the only forbidden-zone change, deferred to Phase 4).
- Does NOT settle the 9 open questions; flags them for implementation cycle entry.
- Does NOT define a precise LOC band for the implementation (prior magnitude-prediction-precision data is for verification-tool family 700-1100 prod; runner is a different family — conductor over 4 primitives — and warrants a fresh band-establishment cycle).
- Does NOT commit v2-cycle-runner skeleton this cycle. The skeleton is the first commit of cycle 150 implementation entry, made under direct-push-zone authorization.
- Does NOT pre-allocate the v2-cycle-runner crate dir to avoid churn if cycle 150 design entry surfaces a structural correction.

## 11. Forward to cycle 150+

If audit cycle 220 watch item 3 ("main Phase 3 cycle-1 minimal end-to-end pipeline first execution") is the natural next milestone audit will track, this design scope sets the agenda for the implementation entry:

- **Cycle 150**: Implementation entry. Re-read each v2-* primitive's CLI surface in detail (open question 1, 2, 3). Author Cargo.toml + main.rs skeleton + first subcommand (likely `init`). Single direct-push commit.
- **Cycle 151**: Run subcommand implementation + first unit tests + dry-run subcommand wiring. Single direct-push commit.
- **Cycle 152**: Integration test against real primitives. First end-to-end dry-run against synthetic state.
- **Cycle 153**: First non-dry-run end-to-end smoke against temp state. **First measurement opportunity for v2 architecture.**
- **Cycle 154+**: Iterate based on smoke findings; surface implementation discoveries; fold into AGREE-DEFER reconsideration.

A 5-cycle implementation arc is the optimistic estimate; the realistic estimate is 8-12 cycles given the 9 open questions and the integration-test setup complexity.

## 12. Cross-references

- Cycle 139 _notes (`cycle-139-phase-3-scoping.md`) — Phase 3 prototype scoping decisions.
- Cycle 140-143 _notes — four primitive SCAFFOLDs (channel-router, super-step-boundary, role-driver, reconciler-event-processor).
- Cycle 144-145 _notes — role-prompts authored.
- Cycle 146 _notes (`cycle-146-prompt-contract-check-design.md`) — design scope for the v2-prompt-contract-check tool that landed cycle 148.
- Cycle 148 _notes (`cycle-148-two-track-absorption-and-landing.md`) — 24-finding critique absorption ledger; this design scope honors forward priority #3 from that ledger.
- `docs/redesign/2-selection.md` — Phase 2 Candidate B selection rationale (preserved untouched per cycle 120 L2; this design implements its multi-agent topology).
- `docs/redesign/2-design-framework.md` — design framework patterns; the failure-mode taxonomy here is the natural Phase 3 instantiation of framework patterns A1-A6.

## 13. Cycle 149 pattern updates

- `two-track-composition` HARDENING-AT-3 → HARDENING-AT-4 cycle 149 (4th consecutive two-track-composition cycle: 146 dispatch+scope, 147 redispatch+dispatch, 148 absorb+land, 149 design-scope+honesty-pass). Pattern remains hardened; the new instance is an in-band recurrence post-HARDENING threshold.
- `proactive-document-before-implementation` NOVEL@1 cycle 149 — first explicit cycle where the substantive focal is a design scope DOCUMENT, not a code commit. Past design scopes (cycle 146 prompt-contract-check) were paired with dispatches in the same cycle. Cycle 149 is design-scope-only for the substantive focal; the act-now finding L1.1+L3.2 is bounded fallback work. Watch for recurrence.

## Cycle 149 ARTIFACTS

This file (design scope, ~290 lines). No code commit Track 1 cycle 149. Track 2 commit `redesign(phase-3): cycle 149 Track 2 — across-prompts honesty-pass...` (5 textual edits across 4 role prompt XMLs; 4/4 contract-aligned post-edit per `v2-prompt-contract-check --strict`).
