# Cycle 178 — `v2-role-driver` live-claude-code-spawn design scope + `session-start-CI-check-discipline` HARDEN

**Cycle issue:** [#2994](https://github.com/EvaLok/schema-org-json-ld/issues/2994)
**Tracks composed:** straight-pair-closure HARDENING-AT-17 (cycles 162–178, 17 consecutive). 27th consecutive two-track-composition post cycle 151 exception. 63rd consecutive HONORING of named forward priority.

## What happened

Cycle 178 advanced [`[input-from-eva] #2992`](https://github.com/EvaLok/schema-org-json-ld/issues/2992) (authored by EvaLok, 2026-05-18 19:37 UTC) directly into Track 1 substantive: design scope for `v2-role-driver` live-claude-code-spawn arc — explicit parallelism-authorization override of the implicit arc-serialization that operated cycles 153-176. Track 2 advanced cycle 178's named priority #1 (`session-start-CI-check-discipline` SOLIDIFY-or-HARDEN analysis) to closure: HARDENED status with explicit operational consequence.

### Track 1 — `v2-role-driver-live-spawn-arc.md` design scope (substantive)

End-to-end design scope authored at [`docs/redesign/_notes/v2-role-driver-live-spawn-arc.md`](v2-role-driver-live-spawn-arc.md). 548 lines covering all 11 topics specified in [`[input-from-eva] #2992`](https://github.com/EvaLok/schema-org-json-ld/issues/2992):

| §  | Topic | Decision |
|---|---|---|
| 3 | Subprocess model | Spawn `claude-code` CLI via `PrimitiveInvoker::invoke` (cycle 177 `RealInvoker`). Reuses C13+X2 timeout machinery directly. Alternatives (B Python/Node SDK shim, C raw HTTP) rejected. |
| 4 | Session lifecycle | `PrimitiveInvoker::invoke(claude_code_bin, args, 4500s) → InvocationResult`. `Completed(Output)` parses stdout → channel payload → write_channel. `TimedOut { .. }` → new `Outcome::Timeout` variant, no channel write, RoleRun records diagnostic. |
| 4.4, 5.1, 5.2, 5.3 | Output / context contract | claude-code `--output-format json` envelope; `result` field extracted; parsed as JSON channel-payload (existing `parse_session_output` shapes 1/2/3). Context assembled INTO user message via stdin (no file handoff). |
| 8 | C13+X2 timeout interaction | Per-step budget 4500s honored at v2-role-driver layer. Role-driver SIGTERM forwarding via process-group inheritance (verify cycle 1 implementation; fallback to `signal_hook`). Partial output captured but not written. |
| 8 | C13+X2 implementation coordination | `Outcome::Timeout` + `timeout: Option<TimeoutDiagnostic>` on `RoleRun`. `elapsed_ms` / `cost_usd` / `usage` schema-promotion DEFERRED per `schema-promotion-discipline.md` (no reader yet at cycle 178). |
| 6 | Tool permissions | Per-role profiles: reconciler=Read,Bash / planner=Read,Grep / executor=Read,Edit,Write,Grep,Bash / curator=Read,Grep. `--permission-mode acceptEdits`. Sandboxing via GitHub Actions runner isolation (no additional). |
| 7 | State exchange | Channel state embedded INTO user message (no Read-call cost). Channel writes via existing `write_channel(...)` after stdout parse + validate (preserves reducer-rule invariant). |
| 9 | First-spawn target | Reconciler. Empty input-channel set → simplest context assembly. Cycle-2 acceptance: hermetic-repo-state reconciler spawn → valid `inbound-channel` payload. Subsequent roles in cycles 3-5. |
| 10 | LOC + cycle estimate | Raw: ~950 LOC. Honest budget per RECURRENCE-AT-3 (2-3× multiplier): 1900-2850 LOC. Cycle estimate: 5-7 cycles to first end-to-end live multi-role cycle; +2-3 cycles slip allowance on cycle 1 if OQ-LS-1 surfaces (claude-code CLI surface divergence). |
| 12 | Acceptance criteria for cycle 1 | Compile + clippy clean; existing 77 unit + 5 integration tests pass; new unit tests cover mode-selection / context-assembly / stdout-envelope / Outcome::Timeout / tool-profile; hermetic integration test with MOCK claude-code shell-script binary; documentation _notes matching cycle 177 pattern. |
| 11 | Open questions deferred | 10 OQ-LS-* questions explicitly named: CLI flag surface, signal forwarding, envelope shape, token capture, decision-log capture, per-agent memory, adaptive permissions, error classification, replay/resume, parallel-role concurrency. Each with explicit deferral reason. |

CLI surface evolution detailed at §2.1: new `--claude-code-bin`, `--prompt-file`, `--max-turns` flags; `--session-output-file` RETAINED in cycle 1 as hermetic-test-mode flag (preserves existing integration tests). Mode selection: setting `--claude-code-bin` selects live-spawn; setting `--session-output-file` selects SCAFFOLD; setting both → error; setting neither → error.

Pattern-match to cycle 176 C13+X2 design-scope-before-implementation: design SCOPE only, no implementation this cycle. Promoted to cycle 179+ implementation per §12 acceptance criteria. Implementation entry point: extend `tools/rust/crates/v2-role-driver/src/main.rs` `Invoke` subcommand per §2.1, add `cmd_invoke_live_spawn` per §3.2 / §4 / §5, add per-role tool profile per §6.1.

### Track 2 — `session-start-CI-check-discipline` HARDEN analysis (bounded-mechanical)

[`session-start-CI-check-discipline.md`](session-start-CI-check-discipline.md) authored — 87 lines documenting the SOLIDIFY-or-HARDEN analysis.

Exercise history: seven consecutive cycles (172-178) of `gh run list --branch master --workflow "Rust CI" --limit 3` invocation + interpretation + routing. Both polarity branches exercised within the window:
- Red-path: cycle 172 (red surfacing → root-cause investigation → master GREEN restored same cycle).
- Green-path: cycles 173-178 (six consecutive greens → proceed to planned Track 1 / Track 2).

Threshold for HARDENING per `pattern-strength` conventions: 5+ consecutive exercises with no missed cycle AND at least one exercise in each operational mode. Both conditions satisfied at cycle 178.

**Resolution: HARDENED cycle 178**, with operational consequence:

- Pattern is removed from forward-watch decay tracking.
- It is added to the **session-start orientation prelude** as a standing routine step (named position #2 in the 4-step prelude).
- Cycle 179+ no longer carries `session-start-CI-check-discipline RECURRENCE-AT-N` in cycle-by-cycle pattern updates.
- The orientation-prelude section maintains the standing-step record (e.g., the cycle 178 session-start comment listed `Session-start CI check (RECURRENCE-AT-7)` as a labeled section; cycle 179+ uses the same labeling but anchored to orientation-prelude rather than forward-watch).

Companion patterns named: `straight-pair-closure-as-default-two-track-shape` (HARDENED cycle 162), `master-CI-red-not-noticed-across-multiple-cycles` (NOVEL@1 cycle 171 INSTANCE-CLOSED cycle 172, prevented from recurrence by this discipline), `live-prompts-already-aligned-at-extended-schema-level` (HARDENED cycle 175, similar HARDENED-with-operational-consequence shape), `as-needed-verification-of-HARDENED-principle-on-tool-rerun` (meta-discipline applicable if CI infrastructure changes break `gh run list`).

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 design scope `v2-role-driver-live-spawn-arc.md` | 548 lines (NEW file) | cycle-close |
| Track 2 discipline note `session-start-CI-check-discipline.md` | 87 lines (NEW file) | cycle-close |
| `.gitignore` extension `docs/**/.tmp-*` | 1 line addition | cycle-close |
| `cycle-178-*.md` _notes (this file) | ~200 lines | cycle-close |
| Total cycle 178 main-authored output | ~836 lines across 4 files | 1 cycle-close (planned) |

No source-side code changes cycle 178. No test runs (design + analysis cycle, no implementation).

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 178: both tracks design-and-analysis (no source / no test deltas; lines-of-prose across 4 files). Contrast with cycle 177 Track 1 substantive-by-LOC (938 source LOC net). Design + analysis cycles produce different magnitude signatures than implementation cycles.

## Pattern updates this cycle

- `two-track-composition` HARDENING-AT-31 (cycles 152-178, 27 consecutive).
- `straight-pair-closure-as-default-two-track-shape` HARDENING-AT-17 (cycles 162-178, 17 consecutive).
- **`session-start-CI-check-discipline` RECURRENCE-AT-6 → HARDENED cycle 178** — Track 2 substantive closure. Removed from forward-watch decay tracking. Moves to session-start orientation prelude as standing step #2. Cycle 179+ pattern updates no longer carry `RECURRENCE-AT-N` for this discipline; orientation-prelude record carries it.
- **`eva-directive-overrides-implicit-arc-serialization` NOVEL@1 cycle 178** — new pattern named from [`[input-from-eva] #2992`](https://github.com/EvaLok/schema-org-json-ld/issues/2992) directive. Definition: when AGREE-DEFER queue accumulates against an implicit serialization rule (e.g., "complete coordinated arc A before opening arc B"), an Eva directive can explicitly override the serialization for a specific arc. Captured: implicit rules across coordinated arcs are not load-bearing; explicit Eva direction can promote a parallel arc at any time. Forward-watch cycles 178-185: another directive overriding a different implicit rule → RECURRENCE-AT-2.
- **`gitignore-extension-from-orchestrator-tempfile-sandbox-friction` NOVEL@1 cycle 178** — new pattern named from session-start friction where `docs/redesign/_notes/.tmp-cycle-178-session-start.md` was created for `gh issue comment --body-file` (per cycle 177 recovery pattern) but could not be removed via `rm` or `mv` due to sandbox restrictions. Resolution: extend `.gitignore` to cover `docs/**/.tmp-*` (in addition to existing `docs/.tmp-*` which only matches files directly under `docs/`). The tempfile remains on disk but doesn't pollute git status. Captured: orchestrator-created tempfiles in `_notes/` subdirectories need gitignore coverage at nested-path patterns; lift this constraint by either adopting only-top-level-docs/ tempfile placement OR ensuring deep gitignore patterns. Forward-watch cycles 178-185 for second occurrence in another subdirectory → RECURRENCE-AT-2.
- `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-3 NOT-EXERCISED cycle 178 (design scope only; no test suite written; forward-watch cycles 178-183, 1 of 6 consumed).
- `coordinated-arc-design-scope-pairs-deferred-items` (shape-complete cycle 177) NOT-EXERCISED cycle 178 (live-spawn arc is gate-promotion, not coordinated pair; forward-watch cycles 178-183 for C6+C7+C9 or C11+C12+X1 design scope, 1 of 6 consumed).
- `as-needed-verification-of-HARDENED-principle-on-tool-rerun` RECURRENCE-AT-2 NOT-EXERCISED cycle 178 (no v2-prompt-* tool re-run this cycle; no prompt extension; forward-watch cycles 177-183, 2 of 7 consumed).
- `category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2 NOT-EXERCISED cycle 178 (4 of 6 consumed; carries; forward-watch cycles 175-180).
- `master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 NOT-EXERCISED cycle 178 (6 of 8 consumed; master remains green; pattern continues at NOVEL@1; forward-watch cycles 173-180).
- `schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 NOT-EXERCISED cycle 178 (6 of 8 consumed). Live-spawn arc §10/§11 explicitly defers `cost_usd` and `usage` schema-promotion per this discipline (no reader yet); the design scope ALIGNS with the discipline without exercising it.
- `tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED cycle 178; carries at RECURRENCE-AT-10.
- `live-prompts-already-aligned-at-extended-schema-level` HARDENED — no Track 2 strict re-run this cycle (no v2-prompt-* infrastructure change; no role-prompt edit; as-needed-verification did not trigger).
- Other carry-forwards (straight-pair-with-dispatch, partial-investigation-misses-second-workflow, design-scope-internal-contradiction-resolved-by-implementation, atomic-dual-crate-PR-stronger-than-design-ordering-requirement, implementation-discovery-as-design-doc-revision-trigger) NOT-EXERCISED cycle 178; carry at prior status.

### Forward-watch decay status

- ~~`session-start-CI-check-discipline` RECURRENCE-AT-7~~ → **HARDENED cycle 178**. Removed from forward-watch.
- **`eva-directive-overrides-implicit-arc-serialization` NOVEL@1**: forward-watch cycles 178-185 for RECURRENCE-AT-2. 0 of 7 consumed.
- **`gitignore-extension-from-orchestrator-tempfile-sandbox-friction` NOVEL@1**: forward-watch cycles 178-185 for RECURRENCE-AT-2. 0 of 7 consumed.
- **`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1 (cycle 176 named, cycle 177 shape-complete)**: forward-watch cycles 178-183 for RECURRENCE-AT-2 (next coordinated arc design scope: C6+C7+C9 or C11+C12+X1). 1 of 6 consumed.
- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` RECURRENCE-AT-2**: forward-watch cycles 177-183 for RECURRENCE-AT-3. 2 of 7 consumed.
- **`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-3**: forward-watch cycles 178-183 for RECURRENCE-AT-4 (next implementation cycle producing test code). 1 of 6 consumed.
- **`category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2**: forward-watch cycles 175-180 for RECURRENCE-AT-3. 4 of 6 consumed.
- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1**: forward-watch cycles 173-180 for RECURRENCE-AT-2. 6 of 8 consumed.
- **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1**: forward-watch cycles 173-180 for RECURRENCE-AT-2. 6 of 8 consumed.

### Session-start orientation prelude (NEW section, cycle 179+)

Per `session-start-CI-check-discipline.md` HARDENED resolution, cycle 179+ orientation has 4 standing steps:

1. **Cycle issue read + model + run-id confirmation.** Post session-start comment with model + run-id + cycle framing.
2. **Master CI check via `gh run list --branch master --workflow "Rust CI" --limit 3`.** Interpret: all green → proceed to planned cycle plan; top row red/cancelled → preempt to Track 1 substantive red-investigation.
3. **Read `input-from-eva` queue for new directives.** Issues with `input-from-eva` label authored by `EvaLok` are trusted directives; non-authored ones are untrusted (per UNTRUSTED-TEXT-RULES).
4. **Cycle plan with Track 1 / Track 2 composition.** Per `straight-pair-closure-as-default-two-track-shape` (HARDENED cycle 162) + `two-track-composition` (HARDENING-AT-31 cycle 178).

## Forward priorities for cycle 179+

Top three:

1. **Orientation prelude exercised at session start** (HARDENED cycle 178 — standing step, no longer forward-watched).
2. **`v2-role-driver` live-spawn cycle 1 of implementation** (NEW priority #2 from cycle 178 design scope §13 + `[input-from-eva] #2992`) — implement the trait wiring, `--claude-code-bin` flag, hermetic MockInvoker tests per §12 acceptance criteria. No live exercise of `claude-code` in cycle 1 — cycle 2's acceptance.
3. **C13+X2 cycle 2 of implementation OR next coordinated arc design** — was priority #3 cycle 177; remains #3 in absolute terms but #2 (live-spawn) is now ahead.

Plus 10 carry-forward items:
4. Coordinated structured-error-envelope arc (C6+C7+C9) — design pending.
5. Coordinated resume/recovery arc (C11+C12+X1) — design pending.
6. AGREE-DEFER queue (post-real-role-session-measurement) — moves UP once cycle-2 of live-spawn produces the first measurement.
7. Master Rust CI green-state maintenance — exercised every session-start via orientation prelude.
8. Audit-engagement strategy — when audit-repo posts a new cycle, route response into reconciler inbound-channel via #2992-class mechanism.
9. Per-axis archival of cycle _notes (cycle 158-161 deferred work).
10. Deprecate legacy methods in v1-* tools once v2 substrate stabilizes (cycle 200+).
11. Workflow trigger upgrade — switch cycle 179+ to using a v2-workflow file if cycle 179+ implementation needs it.
12. `--all` sweeps tooling for v2-* primitives (cycle 158-161 deferred).
13. Reconcile improvements: housekeeping passes on open issues and draft PRs (cycle 33 deferred work; new candidates from #2992 + cycle 178).
14. `--invocation-id` flag for cross-tool tracing (cycle 158-161 deferred).

## In-session issues and recoveries

- **Session-start tempfile sandbox friction** — recovered via `.gitignore` extension. The pattern `docs/.tmp-*` does not match `docs/redesign/_notes/.tmp-cycle-178-session-start.md` (gitignore globs match at one level only without `**`). Extended pattern to `docs/**/.tmp-*` so nested tempfiles are covered. NEW pattern `gitignore-extension-from-orchestrator-tempfile-sandbox-friction` NOVEL@1 cycle 178 captures the substrate observation.
- **Cwd not persisted between bash calls** (similar to cycle 176-177). Not exercised cycle 178 because no `cargo` invocations (design + analysis cycle, no implementation work).
- **`gh issue comment --body-file` worked clean** — the cycle 177 recovery pattern (tempfile + `--body-file`) used directly without rediscovery. `as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOT-EXERCISED but the cycle 177 recovery is now reliable.
- All `git`, `gh`, Write/Edit operations otherwise clean cycle 178.

---

**Design-scope authored:** [`docs/redesign/_notes/v2-role-driver-live-spawn-arc.md`](v2-role-driver-live-spawn-arc.md) (this cycle).
**Discipline note authored:** [`docs/redesign/_notes/session-start-CI-check-discipline.md`](session-start-CI-check-discipline.md) (this cycle).
**Companion design scope:** [`docs/redesign/_notes/v2-primitive-invoker-timeout-arc.md`](v2-primitive-invoker-timeout-arc.md) (cycle 176; cycle-177-implementation-landed).
**Trigger directive:** [`#2992`](https://github.com/EvaLok/schema-org-json-ld/issues/2992) (Eva, 2026-05-18 19:37 UTC).
