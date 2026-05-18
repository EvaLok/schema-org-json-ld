# Cycle 176 _notes — C13+X2 coordinated retry/timeout/cancellation arc design scope + `v2-prompt-contract-check --strict` post-extension re-run

**Cycle window:** 2026-05-18 11:15 UTC start, single session.
**Cycle issue:** [#2991](https://github.com/EvaLok/schema-org-json-ld/issues/2991).
**Run ID:** captured at session-start comment.

## Composition shape

Two-track straight-pair-closure. 25th consecutive two-track-composition post-cycle-151 exception (`two-track-composition` HARDENING-AT-29). 15th consecutive straight-pair (`straight-pair-closure-as-default-two-track-shape` HARDENING-AT-15). 61st consecutive HONORING of named forward priority (cycle 175+ priority #3 closed via Track 1 design scope; cycle 175+ priority #8 closed via Track 2 mechanical verification).

| Track | Shape | Priority closed | LOC |
|---|---|---|---|
| Track 1 | substantive design-scope draft | cycle 175+ #3 (C13+X2 design scope) | 290 _notes |
| Track 2 | bounded-mechanical verification | cycle 175+ #8 (`--strict` re-run post-extension) | 0 LOC; 2 tool invocations |

`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1 named cycle 176 — first instance of design-scope authoring that bundles a deferred pair (C13 + X2) per the cycle 155 verdict pairing convention. Forward-watch cycles 176-183 for second instance (cycle 175+ #4 = C6+C7+C9 coordinated structured-error-envelope arc + #5 = C11+C12+X1 coordinated resume/recovery arc are the candidates).

`as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOVEL@1 named cycle 176 — first exercise of the new operational mode established cycle 175 (HARDENED `live-prompts-already-aligned-at-extended-schema-level` converts honesty-pass to as-needed verification triggered by substantive role-prompt or tag-semantics manifest edits). Cycle 176 Track 2 re-ran two relevant tools (`v2-prompt-contract-check --strict`, `v2-prompt-tag-semantic-fidelity check --strict`) post-extension — both clean. Forward-watch cycles 176-183 for second instance (any substantive role-prompt or tag-semantics manifest edit between now and cycle 183 should trigger another re-run, recurring the pattern toward RECURRENCE-AT-2).

## Track 1 — C13+X2 coordinated retry/timeout/cancellation arc design scope (priority #3 FULLY CLOSED)

### Scope

Cycle 175+ priority #3 named the arc carrying forward from cycle 155 absorption. Cycle 176 authors `docs/redesign/_notes/v2-primitive-invoker-timeout-arc.md` — a 290-line design scope covering PrimitiveInvoker trait evolution, FailureClass::Timeout addition, RealInvoker implementation strategy (Pattern A thread-and-channel), default per-step timeout table, signal escalation policy (SIGTERM → 2s grace → SIGKILL), MockInvoker extension for test simulation, migration cost honest accounting, and acceptance criteria for the implementation cycle.

### Load-bearing design decisions

1. **Trait signature returns `InvocationResult` enum, not `Output` directly.** Two variants: `Completed(Output)` for normal exit, `TimedOut { elapsed, partial_stdout, partial_stderr, escalation }` for kills. Existing callers branch on the variant.
2. **`Timeout` is its own `FailureClass`** — not a sub-case of `Transient`. Retry policy: no automatic retry. Rationale documented in §2.3 (timeouts usually mean the budget is wrong, not transience; retrying with same budget doubles latency before halt).
3. **Per-step defaults in §3.3 table.** Channel-router/super-step-boundary/state-audit/reconciler-event-processor: 30-60s. Role-driver: 4500s (matches cycle-runner harness wall clock). write-entry: 30s.
4. **Signal escalation: SIGTERM → 2s grace → SIGKILL.** The 2s grace is internal to RealInvoker; the budget visible to the caller IS the timeout passed in.
5. **Streaming is OUT of scope.** C13 names streaming alongside hangs and signals; this arc addresses hangs + signals only. Streaming deferred until a primitive that produces partial-progress output exists to motivate it.
6. **No async runtime.** std + spawned wait-thread + `mpsc::recv_timeout` is sufficient; no tokio dependency.
7. **Migration LOC estimate: ~330 LOC source+tests cycle 1 of implementation.** Applied `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-2 in §6.3 (budget cycle 1 at ~600-800 LOC actual without raising concern).
8. **`StepTrace` gains `elapsed_ms` + `timeout` fields.** elapsed_ms always present for executed traces (operators want trend data); timeout-diagnostic present only for halt-on-timeout traces.
9. **Cycle 1 of implementation is Unix-only.** Windows portability documented as cycle-1 constraint, not permanent decision.

### What the arc closes vs leaves open

Closes the pair: C13 (L3.1) hides hangs + signals + X2 (no per-step timeout). Does NOT close C13's streaming critique — explicitly deferred.

### Track 1 produced artifacts

- `docs/redesign/_notes/v2-primitive-invoker-timeout-arc.md` (290 lines, 9 sections).
- 0 code changes (design scope only; implementation cycle 177+).
- 0 prompt-content edits.
- 1 cycle-close commit (this _notes + journal + the design scope, planned).

### Compare to prior coordinated-arc-style work

Cycle 155 named three deferred pairs: C13+X2 (this arc), C6+C7+C9 (structured-error-envelope), C11+C12+X1 (resume/recovery). Cycle 176 is the first instance of authoring a coordinated-arc design scope. The pattern of pairing deferred items at design-scope authoring time (rather than processing them serially as singletons) was set by cycle 155's verdict pairing; cycle 176 is the first design-scope realization of that pairing.

`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1 forward-watch: cycles 176-183 (8-cycle window) for second instance. Either the C6+C7+C9 arc or the C11+C12+X1 arc landing as a design scope before cycle 184 RECURs the pattern toward HARDENING.

## Track 2 — `v2-prompt-contract-check --strict` re-run + `v2-prompt-tag-semantic-fidelity check --strict` re-run post-extension (priority #8 FULLY CLOSED)

### Scope

Cycle 175+ priority #8 named "re-run post-extension" to verify the live prompts remain aligned at the contract-check layer after the cycle 173+174 tag-semantics extension work landed. Bounded mechanical: invoke both tools, examine output, log result.

### Results

**`v2-prompt-contract-check --strict`:**

```
v2-prompt-contract-check: 4/4 prompts contract-aligned with v2-channel-router
  ✓ planner-prompt.xml ↔ plan-channel (writer: Planner)
  ✓ reconciler-prompt.xml ↔ inbound-channel (writer: Reconciler)
  ✓ executor-prompt.xml ↔ work-channel (writer: Executor)
  ✓ curator-prompt.xml ↔ memory-channel (writer: Curator)

Deferred in cycle-1 minimal scope:
  - check-3-input-channel-membership: No parseable <source channel="..."> declarations found in current prompts/v2 XML structure; deferred for cycle-1 minimal scope.
  - check-4-input-required-keys: No parseable input-channel key blocks bound to <source channel="..."> were found; deferred for cycle-1 minimal scope.
```

4/4 contract-aligned. Two deferred checks noted (input-channel-membership, input-required-keys) — these were already deferred at the tool's initial scope and remain so; nothing regressed. The deferred-check pattern is honest minimal-scope discipline, not a regression.

**`v2-prompt-tag-semantic-fidelity check --strict`:**

```
v2-prompt-tag-semantic-fidelity — 4 prompts scanned
manifest: prompts/v2/tag-semantics.toml
[PASS] Tier 1: no errors
[PASS] Tier 2: no warnings
Summary: tier1_errors=0 tier1_passes=47 tier2_warnings=0 adaptation_notes=0
exit_code: 0
```

47/0 Tier-1, 0/0 Tier-2 — identical to cycle 174 post-extension counts. The tag-semantics extension landed cycle 174 (stemming + name-attr inclusion) plus the cycle 173 intent-string sharpenings hold green at HEAD.

### Track 2 produced artifacts

- 0 file changes.
- 2 tool invocations recorded in this _notes.
- 0 standalone commits.

### What this confirms

`live-prompts-already-aligned-at-extended-schema-level` HARDENED principle holds at the tool layer after the cycle 173+174 extension work. The principle's operational consequence (honesty-pass converts to as-needed verification) is exercised cleanly cycle 176 — both tools pass; no further work owed at this layer. First instance of `as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOVEL@1.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 design scope `v2-primitive-invoker-timeout-arc.md` | 290 lines | cycle-close |
| Track 1 honesty-assessment context (in this _notes) | ~80 _notes lines | cycle-close |
| Track 2 verification result (in this _notes) | ~40 _notes lines | cycle-close |
| `cycle-176-*.md` _notes total | ~290 lines | cycle-close |
| Total cycle 176 main-authored output | ~580 _notes/doc lines + 0 LOC code | 1 cycle-close (planned) |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 176: Track 1 substantive (design-scope-by-LOC, 290 lines), Track 2 bounded-mechanical (verification-by-execution, 0 LOC). Cycle shape similar to cycle 168 (two-design-scope-drafts) but with one substantive design scope + one bounded mechanical verification rather than two design scopes.

## Pattern updates this cycle

- `two-track-composition` HARDENING-AT-29 (cycles 152-176, 25 consecutive).
- `straight-pair-closure-as-default-two-track-shape` HARDENING-AT-15 (cycles 162-176, 15 consecutive).
- `session-start-CI-check-discipline` RECURRENCE-AT-5 cycle 176 — session-start `gh run list --workflow "Rust CI" --limit 3` confirmed master green on `e7358be5` (cycle 175 close commit). Forward-watch cycles 177-180 for further exercise toward HARDENING-AT-N.
- **`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1 cycle 176** — first instance of design-scope authoring that bundles cycle 155 deferred pairs. Forward-watch cycles 176-183 for RECURRENCE-AT-2 via C6+C7+C9 or C11+C12+X1 design-scope landing.
- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOVEL@1 cycle 176** — first exercise of the new operational mode for HARDENED `live-prompts-already-aligned-at-extended-schema-level`. Forward-watch cycles 176-183 for RECURRENCE-AT-2 via second tool re-run event.
- `category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2 NOT-EXERCISED cycle 176 (2 of 6 consumed; carries; forward-watch cycles 177-180).
- `master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 NOT-EXERCISED cycle 176 (4 of 8 consumed; carries; forward-watch cycles 177-180).
- `schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 NOT-EXERCISED cycle 176 (4 of 8 consumed; carries; forward-watch cycles 177-180).
- `tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED cycle 176; carries at RECURRENCE-AT-10.
- `live-prompts-already-aligned-at-extended-schema-level` HARDENED — exercised cycle 176 via Track 2 re-run (clean); operational mode confirmed.
- `tools/v2-* wrapper directly-pushable as v2 substrate by convention extension` OBSERVATION-AT-1 carries.
- `directive-2937-track-1-or-track-2-dispatch-fit-application` NOT-EXERCISED cycle 176; carries at HARDENING-AT-3.
- `straight-pair-with-dispatch-variant` NOT-EXERCISED cycle 176; carries at HARDENING-AT-3.
- `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` APPLIED-IN-DESIGN cycle 176 (RECURRENCE-AT-2 explicitly cited in §6.3 of the design scope to inform the implementation-cycle LOC budget); pattern not re-instanced at HEAD but DOCUMENTARILY referenced — recording for forward-watch decision: does design-time pre-application count as RECURRENCE or only post-implementation observation? Conservative: do NOT increment count from this citation alone; pattern remains at RECURRENCE-AT-2.
- `clippy-follow-up-commit-post-absorption` NOT-EXERCISED cycle 176; carries at RECURRENCE-AT-2.
- `partial-investigation-misses-second-workflow` NOT-EXERCISED cycle 176; carries at RECURRENCE-AT-2.
- `design-scope-internal-contradiction-resolved-by-implementation` NOT-EXERCISED cycle 176; carries at NOVEL@1.
- `atomic-dual-crate-PR-stronger-than-design-ordering-requirement` NOT-EXERCISED cycle 176; carries at NOVEL@1.
- `implementation-discovery-as-design-doc-revision-trigger` NOT-EXERCISED cycle 176; carries at NOVEL@1 (referenced in §8 acceptance criterion 5 of the design scope as the discipline for implementation-cycle _notes).

### Forward-watch decay status

- **`session-start-CI-check-discipline` RECURRENCE-AT-5**: forward-watch cycles 177-180 for further exercise. If consecutively exercised through cycle 180, becomes HARDENING-AT-N candidate (5 consecutive exercises = HARDEN threshold by current convention).
- **`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1**: forward-watch cycles 176-183; 0 of 8 consumed.
- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOVEL@1**: forward-watch cycles 176-183; 0 of 8 consumed.
- **`category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2**: forward-watch cycles 175-180; 2 of 6 consumed.
- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1**: forward-watch cycles 173-180; 4 of 8 consumed.
- **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1**: forward-watch cycles 173-180; 4 of 8 consumed.

## Forward priorities for cycle 177+

15 items total. Cycle 175+ priorities #3 and #8 closed; #3 promoted to implementation (now #3); #8 fully retired.

1. **Master Rust CI green-state maintenance + `session-start-CI-check-discipline` RECURRENCE-AT-5 forward-watch** (was cycle 175+ #1).
2. **AGREE-DEFER queue** (post-real-role-session-measurement; was cycle 175+ #2).
3. **C13 + X2 IMPLEMENTATION** (cycle 175+ #3 design scope FULLY CLOSED cycle 176; promoted to implementation per `v2-primitive-invoker-timeout-arc.md` §8 acceptance criteria).
4. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 175+ #4).
5. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 175+ #5).
6. **Audit-engagement substantive-focal single-track variant** (was cycle 175+ #6; gate = audit HEAD changes).
7. **Per-axis archival mechanism design scope** (was cycle 175+ #7).
8. **Deprecate `Channel::required_payload_keys()` legacy method** (was cycle 175+ #9; LOW).
9. **Workflow trigger upgrade — add `ready_for_review` to pull_request trigger types** (was cycle 175+ #10; LOW; PR-required).
10. **`--all` sweep modes for status / verify** (was cycle 175+ #11; LOW).
11. **`v2-cycle-runner reconcile` orphan-state detection** (was cycle 175+ #12; LOW).
12. **`v2-state-dispatch-archive --invocation-id` flag** (was cycle 175+ #13; LOW).
13. **`schema-promotion-requires-reader-co-edit-via-named-field` discipline codification** (was cycle 175+ #14; LOW; bounded; discipline note authored cycle 175; lint implementation deferred pending RECURRENCE-AT-2).

**Cycle 176 forward priorities CLOSED:**
- Cycle 175+ priority #3 (C13+X2 coordinated arc) — DESIGN-SCOPE FULLY CLOSED via Track 1 `v2-primitive-invoker-timeout-arc.md`; promoted to implementation for cycle 177+.
- Cycle 175+ priority #8 (`v2-prompt-contract-check --strict` re-run post-extension) — FULLY CLOSED via Track 2 verification (47/0 Tier-1, 4/4 contract-aligned).

**Net list-length change:** -2 closures, +0 new items (the C13+X2 design slot is replaced by C13+X2 implementation, which is a re-shape not a net new entry). Forward priorities list compresses to 13 items (from 14 entering cycle 176).

## Observational forward-watch items

1. **`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1** — first instance cycle 176. Forward-watch: C6+C7+C9 or C11+C12+X1 design-scope authoring in cycles 177-183 RECURs to RECURRENCE-AT-2.

2. **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOVEL@1** — first instance cycle 176. Forward-watch: any substantive role-prompt or tag-semantics manifest edit in cycles 177-183 should re-trigger tool re-runs, producing RECURRENCE-AT-2.

3. **`session-start-CI-check-discipline` RECURRENCE-AT-5** — 5 consecutive cycles of exercise. By convention, RECURRENCE-AT-5 with N=5 consecutive ≥ HARDENING threshold of 5 instances. Cycle 177 either takes pattern to HARDENING-AT-N or surfaces a gap (e.g., a cycle that genuinely doesn't need the CI check, like an off-master cycle). Standard cycle-start exercise expected.

## In-session issues and recoveries

- **Cwd persistence between bash calls**: a `cd tools/rust && cargo run ...` succeeded; the next bash call `cd tools/rust && cargo run ...` failed because the working-directory state did not carry between calls in the harness's serialized invocations. Recovered via `cargo run --manifest-path tools/rust/Cargo.toml -q --bin ...` style (absolute path to Cargo.toml). Pattern observation: avoid relying on cwd-as-state across bash invocations; pass explicit paths. Not a new pattern (well-established harness behavior).
- **`gh issue comment --body` with inline message** worked clean cycle 176 (no tempfile workaround needed — short body).
- All `cargo`, `gh`, `git`, Edit/Write operations otherwise clean cycle 176.
- **Permission prompts avoided cycle 176**: no `tools/v2-*` wrapper invocation attempted; cargo bin invocations via workspace Cargo.toml only.

## What this cycle does NOT do

- Does NOT implement C13+X2 (design scope only; implementation cycle 177+).
- Does NOT touch the live role prompts (Track 2 was read-only verification).
- Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Does NOT close cycle issue #2991 before the cycle-close commit lands (sequential).
- Does NOT escalate to Eva (EVA-DEFAULT-AUTONOMY; both tracks resolvable in-cycle).
- Does NOT measure runtime quality of role prompts (no end-to-end run; deferred to AGREE-DEFER queue post-real-role-session-measurement gate).
- Does NOT add per-step timeout to `v2-cycle-runner` (design scope only; implementation cycle 177+).
- Does NOT touch `classify_failure` (stderr-keyword classifier deferred to the C6+C7+C9 structured-error-envelope arc).
