# Cycle 181 — C6+C7+C9 coordinated structured-error-envelope arc design scope

**Status:** design-scope landing (Track 1 substantive). Cycle 181 priority #4 per cycle 180 close (with #2997 AUTH-resolution unresolved). Document: `docs/redesign/_notes/v2-structured-error-envelope-arc.md` at 391 LOC.

**Cycle issue:** [#2998](https://github.com/EvaLok/schema-org-json-ld/issues/2998)
**Predecessor:** cycle 180 (`3f0ecf56`, v2-role-driver live-spawn cycle 2 reconciliation)
**Reference forward priority:** `cycle-180-live-spawn-implementation-cycle-2.md` §forward priorities #4

## 1. Substantive changes per layer

### 1.1 New design-scope file `v2-structured-error-envelope-arc.md`

Authored at 391 lines covering 12 sections per the established design-scope template (cycle 176 timeout-arc 290 LOC + cycle 178 live-spawn-arc 548 LOC are the precedents; this scope sits within the band):

- §1 Problem statement, with verbatim code citation of the current `classify_failure` at `v2-cycle-runner/src/main.rs:696-711` (line numbers drifted from cycle-155-cited 510-525 due to cycle-177 PrimitiveInvoker refactor + cycle-179 v2-primitive-invoker extraction; cycle-181 Explore agent re-measured)
- §2 Design framework alignment (axis-3 state primitives + axis-5 failure-mode handling + axis-6 one-writer-per-channel preservation + CORE-DESIGN-PRINCIPLE alignment)
- §3 Envelope shape — `ErrorEnvelope` struct + `ErrorClass` enum (11 variants: existing 6 from `FailureClass` + new Auth/Config/Io/Protocol/Unknown). JSON-on-stderr with last-line-tail parse convention. `--error-format <text|json>` opt-in flag default-text backward-compat.
- §4 Per-class taxonomy + `details` schema table (11 classes × expected detail keys × emitting primitives × retry policy)
- §5 Per-primitive impact analysis (v2-channel-router, v2-role-driver, v2-cycle-runner caller-side, v2-primitive-invoker dependency-clean, plus migration discipline for other v2-*)
- §6 Cross-cutting design questions (JSON-vs-exit-code-ranges, per-line-tail-parse rationale, free-form-Map-vs-typed-enum, human-field rationale, bottom-up cycle order rationale)
- §7 Acceptance criteria — per-cycle + overall arc
- §8 Honest LOC estimate (raw ~1700 / honest 2000-3400 per cycle-179 RECURRENCE-AT-4 multiplier data + this scope's mid-thoroughness level prediction 1.4-1.7×)
- §9 Open questions OQ-SEE-1 through OQ-SEE-8
- §10 Implementation cycle plan (5-6 cycle estimate: 182 cycle 1 / 183 cycle 2 / 184 cycle 3 / 185 cycle 4 / 186+ potential reconciliation)
- §11 Forward priorities post-arc (X4 observability partial unblock, C11+C12+X1 resume arc enabled, polyglot-ready, exit-code-defense optional)
- §12 Provenance + cross-references

### 1.2 Inputs synthesized

The design scope synthesizes inputs from:

- **Cycle 155 critique absorption** §C6 (taxonomy too coarse) + §C7 (stderr-keyword brittleness) + §C9 (exit-code ignored), all AGREE-DEFER pending coordinated arc, plus §X4 (observability) noted as "possibly bundled with C6 work"
- **Cycle 180 OQ-LS-3 reconciliation** — the real claude-code envelope shape `{subtype: "success", is_error: true, result: "Not logged in"}` is concrete prior art for the structured envelope pattern (cited in §1.2 of the design as the urgency-grounding triggering finding)
- **Cycle 181 Explore agent measurement** — comprehensive map of current error/stderr/exit-code paths across 4 v2-* crates (v2-cycle-runner 3681 LOC + v2-channel-router 1424 LOC + v2-role-driver 2440 LOC + v2-primitive-invoker 621 LOC), located:
  - `classify_failure` actual current site at lines 696-711 (substring matching only, exit-code unused)
  - 6-variant `FailureClass` enum to be extended (Transient / RoleSessionEmpty / ChannelWriteRejected / SuperStepOutOfOrder / StateBoundExceeded / Timeout)
  - 8-variant `RouterError` enum mapping to envelope classes
  - 11-variant `DriverError` enum (5 pre-cycle-179 + 6 cycle-179 live-spawn) mapping to envelope classes
  - Existing structured-output prior art: `CycleReport` (v2-cycle-runner) and `TimeoutDiagnostic` + `SignalEscalation` (v2-primitive-invoker, cycle 177)

### 1.3 What the design scope does NOT do

Per §1.3 of the design itself — explicitly out-of-scope items:
- Backward-compat removal of text-on-stderr default
- Exit-code-range semantics (defense-in-depth deferred to follow-on)
- Schema-promotion of envelope fields beyond `{primitive, class, details, human}` until reader appears
- Cross-language polyglot envelope migration (incidentally enabled by JSON; no extra work)
- Streaming envelopes mid-execution
- X4 observability dashboard build (envelope's class field is the input; the aggregator is a separate cycle)
- Replacing internal `RouterError`/`DriverError` enums (envelope is emission-shape; internal types remain)

## 2. Substantive measurements

### 2.1 LOC delta this cycle

- `docs/redesign/_notes/v2-structured-error-envelope-arc.md`: +391 doc LOC (new file)
- `docs/redesign/_notes/cycle-181-c6-c7-c9-structured-error-envelope-design-scope.md`: this file (~150 doc LOC; Track 2)
- `docs/journal/2026-05-19.md`: ~70 lines appended (cycle 181 entry; Track 2)
- Source code delta: 0 LOC (design scope; no implementation in cycle 181)

### 2.2 Design scope estimate calibration

Design scope itself (391 LOC) lands within precedent band [290, 548]. Implementation estimate per scope §8:
- Raw: ~1700 LOC across 4 implementation cycles
- Honest budget: 2000-3400 LOC at 1.2-2.0× multiplier
- Prediction (mid-thoroughness scope per cycle-179 pattern): 1.4-1.7× raw → 2400-2900 LOC actual

This is the SETUP for `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-5 measurement at cycle 185+ close (when cycle-4 implementation lands).

### 2.3 8 OQ-SEE-* items deferred

Following cycle-178 pattern (10 OQ-LS-* items) and cycle-176 pattern (no OQ list — simpler scope), cycle 181 lands with 8 enumerated open questions deferred to cycle-2-4 of implementation:
- OQ-SEE-1: envelope byte-length verification
- OQ-SEE-2: pre-envelope warning + envelope tail-parse interaction
- OQ-SEE-3: `parse_claude_code_envelope` shape refactor option
- OQ-SEE-4: should `details.upstream_envelope` embed full claude-code envelope (security-adjacent — default NO)
- OQ-SEE-5: schema version field (default NO)
- OQ-SEE-6: `Unknown` class retry policy (default retry-once)
- OQ-SEE-7: degraded-classification logging (default yes, once-per-cycle counter)
- OQ-SEE-8: flag vs env-var for `--error-format` (default flag)

These set up forward-watch for `design-scope-honesty-hedge-survives-implementation-cycle` RECURRENCE-AT-3 at cycle 182+ — does implementation honor the hedges or quietly elide them.

## 3. Pattern updates this cycle

### 3.1 Continuing-shape patterns

- **`two-track-composition` HARDENING-AT-34** cycle 181. 30th consecutive two-track post-cycle-151 single-track exception. Track 1 (substantive design scope) + Track 2 (this _notes + journal). Pattern continues to hold across the long redesign arc.

- **`session-start-CI-check-discipline` HARDENED operates in orientation-prelude mode** cycle 181. Standing step #2 confirmed: `gh run list --branch master --workflow "Rust CI" --limit 3` returned GREEN top-row at `3f0ecf56` SUCCESS, confirming cycle 180's commit held. No RECURRENCE-AT-N tracking; pattern is now orientation-prelude furniture.

- **66th consecutive HONORING of named forward priority** cycle 181 (+1 from cycle 180's 65th). Cycle 180 forward priority #4 ("if #2997 NOT resolved by cycle 181: coordinated structured-error-envelope arc design scope") was the named priority; cycle 181 honored it directly.

### 3.2 RECURRENCE-AT-N updates

- **`coordinated-arc-design-scope-pairs-deferred-items` RECURRENCE-AT-4** cycle 181 (was RECURRENCE-AT-3 cycle 180). The pattern shape — original-defer → coordinated-design → implementation-cycles — fires on a fresh distinct arc this cycle. The pair-vs-followup distinction noted in cycle 180 §3.1 is INFORMED by today's instance: cycle 181 is a DESIGN-ONLY cycle (no implementation; pair closes at cycle 182+). Cycle 180's third instance was a cycle-2-followup completing the second arc; cycle 181 is a cycle-1-design opening a third arc. Both shapes valid; pattern variance is "pair-cycle-1-design" vs "cycle-2+-followup-of-existing-arc" — naming refinement deferred to RECURRENCE-AT-5 cycle 182+ when more data accumulates.

- **`design-scope-honesty-hedge-survives-implementation-cycle` NOT-EXERCISED cycle 181** but PRECONDITION-SET. Cycle 181 design scope contains 8 OQ-SEE-* enumerated hedges (matching cycle-178 design's 10 OQ-LS-* hedges). Whether cycle 182+ implementation honors these (per cycle 179 honored cycle-178's hedges) or quietly elides them is the cycle-182-185 forward-watch question. RECURRENCE-AT-3 deadline cycle 188.

- **`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` NOT-EXERCISED cycle 181** but PRECONDITION-SET. Cycle 181 is design scope only (391 doc LOC, 0 source LOC). The measurement window opens cycle 182 (first implementation cycle) and closes at cycle 185+ (final implementation cycle) with full LOC ratio. RECURRENCE-AT-5 deadline cycle 188 — fifth data point will inform multiplier-refinement question raised cycle 179.

### 3.3 NEW pattern candidates

- **`deferred-arc-cycles-from-original-defer-to-design-scope`** NOVEL@1 cycle 181. The C6+C7+C9 arc was named in cycle 155 (2026-05-02) and reaches design scope cycle 181 (2026-05-19) — 26-cycle aging. The C13+X2 (timeout) arc was named cycle 155 and reached design cycle 176 (2026-05-17) — 21-cycle aging. Pattern: defer-aged-from-naming-to-design has variance in the ~20-25 cycle band for coordinated arcs of similar scope. Forward-watch cycles 181-189 for RECURRENCE-AT-2 when next deferred arc (likely C11+C12+X1 resume/recovery) reaches design — predicted cycle range 196-205 based on cycle 155 naming + ~25 cycle aging. Operationally useful for planning when to schedule next-deferred-arc design.

- **`design-scope-template-stabilizes-across-arcs`** NOT-NAMED-YET candidate. Cycle 176 timeout-arc, cycle 178 live-spawn-arc, cycle 181 structured-error-envelope-arc all follow the same ~10-section template: problem-statement / trait-or-shape evolution / per-primitive-impact / acceptance-criteria / honest-LOC-estimate / OQ-list / implementation-cycle-plan / provenance. The template is now established by cycle 181. Recurrence test: next design scope (whether C11+C12+X1 or another emergent arc) following the same shape moves this from candidate-shape to NOVEL@1 named pattern. Defer naming until clear it's a deliberate template vs incidental similarity.

### 3.4 Forward-watch decay

- `eva-directive-overrides-implicit-arc-serialization` NOVEL@1 (cycle 178) → RECURRENCE-AT-2 (cycle 179) NOT-EXERCISED cycle 180 + 181 (2 of 6 consumed before deadline cycle 185).
- `gitignore-extension-from-orchestrator-tempfile-sandbox-friction` NOVEL@1 (cycle 178) NOT-EXERCISED cycle 179 + 180 + 181 (3 of 6 consumed; DEADLINE cycle 184).
- `as-needed-verification-of-HARDENED-principle-on-tool-rerun` RECURRENCE-AT-2 (cycle 178) NOT-EXERCISED cycles 179 + 180 + 181 (3 of 6 consumed; DEADLINE cycle 184).
- `design-tacit-assumption-falsified-during-implementation` NOVEL@1 (cycle 180) NOT-EXERCISED cycle 181 (1 of 6 consumed; DEADLINE cycle 187).
- `cargo-spawned-subprocess-bypasses-tool-permission-intercept` NOVEL@1 (cycle 180) NOT-EXERCISED cycle 181 (1 of 6 consumed; DEADLINE cycle 187).
- `shared-crate-extraction-from-isolated-reducer-rule-precedent` NOVEL@1 (cycle 179) NOT-EXERCISED cycle 180 + 181 (2 of 6 consumed; DEADLINE cycle 186). Cycle 182+ implementation cycle 1 of THIS arc creates `v2-error-envelope` shared crate — likely RECURRENCE-AT-2 trigger.

## 4. What cycle 181 does NOT do

- Does NOT modify `cycle-runner` (legacy v1; forbidden zone preserved during Phase 3).
- Does NOT modify `.github/workflows/` or the orchestrator prompt.
- Does NOT implement any of the cycle-181 design — implementation is cycles 182-185+.
- Does NOT create `tools/rust/crates/v2-error-envelope/` — that's cycle 182's substantive focal.
- Does NOT modify any v2-* primitive source code.
- Does NOT close [#2997](https://github.com/EvaLok/schema-org-json-ld/issues/2997) — AWAITING Eva response on OQ-LS-AUTH; the cycle-180 priority-#2 wait continues.
- Does NOT advance v2-role-driver live-spawn arc (cycle 180 closed cycle-2 reconciliation; cycle-2 first-live-spawn success-path acceptance remains blocked on #2997).
- Does NOT dispatch a Copilot feedback session on this design scope — the audit repo (cross-repo read) will see this design on its next cycle and can critique then; precedent from cycle 176 and cycle 178 (neither dispatched feedback on their design scopes).
- Does NOT escalate any cycle 181 decision to Eva (EVA-DEFAULT-AUTONOMY).

## 5. Forward priorities for cycle 182+

1. **Master Rust CI green-state maintenance** — HARDENED orientation-prelude exercise.

2. **AWAIT #2997 Eva response on OQ-LS-AUTH** — cycle-180 priority #2 continues. Filed 2026-05-19 03:00 UTC; cycle 181 close at 2026-05-19 05:30 UTC. Still within the 5-cycle BETWEEN-CHECKPOINTS autonomy band (would expire cycle 185-186). The implicit-arc-serialization convention says one substantive arc at a time; if #2997 resolves during this arc, pause C6+C7+C9 work for live-spawn cycle-2 success-path acceptance.

3. **C6+C7+C9 implementation cycle 1 (NEW priority #3)** — create `tools/rust/crates/v2-error-envelope/` shared crate with `ErrorEnvelope` + `ErrorClass` types + Serialize/Deserialize round-trip tests. Bottom-up migration per design §6.5. Estimate ~350 raw LOC. Per design §8 prediction: 1.4-1.7× implementation multiplier → 500-600 actual LOC for cycle 1.

4. **C13+X2 cycle 2 of implementation** — `elapsed_ms` schema promotion when a reader appears. Deferred until post-AUTH resolution OR cycle 185+ if AUTH stalls. Cycle 181 design's `details` schema for `timeout` class explicitly mirrors cycle-177 `TimeoutDiagnostic` fields — when v2-cycle-runner reads `Timeout` envelope, `elapsed_ms` becomes a reader-justified field promotion. May be naturally absorbed into the cycle-185+ C6+C7+C9 cycle 4 implementation.

5. **Coordinated resume/recovery arc (C11+C12+X1)** — next deferred-arc-to-design candidate per `deferred-arc-cycles-from-original-defer-to-design-scope` pattern; predicted design-scope landing cycle 196-205 if 20-25 cycle aging recurs. Carries forward.

6. **C13+X2 implementation Cycle 3+ + non-spawn-blocked AGREE-DEFER queue items** — carry forward. The 24+ cycle AGREE-DEFER queue gated on `post-real-role-session-measurement` evidence is partially related: structured-error envelope from primitives is the OBSERVABILITY substrate that real-session measurement would feed into. Bundle when natural.

7. **Audit-engagement, per-axis archival, deprecate-legacy-method, workflow-trigger-upgrade, --all sweeps, reconcile improvements, --invocation-id flag** — carry forward from cycle 178+ enumeration. Full enumeration carries from cycle 178+ unchanged.
