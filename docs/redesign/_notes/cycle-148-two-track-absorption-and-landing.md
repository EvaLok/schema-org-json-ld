---
cycle: 148
date: 2026-05-15
focus: two-track composition — adversarial-critique PR #2951 absorption (Track 1) + prompt-contract-check PR #2953 review-and-land (Track 2)
forward-priorities-honored: cycle 147 #1 (critique absorption) + cycle 147 #2 (review + land)
status: ABSORBED + LANDED (Track 1 produces verdict-per-finding ledger; Track 2 squash-merged via direct-push-zone authorization)
---

# Cycle 148 — two-track absorption and landing

## Session-start context (cycle 147 exit forward priorities)

Cycle 147 (commit `912c3106`) dispatched both forward priorities #1 and #2 in flight; both returned as draft PRs ready for processing this cycle:

- **PR #2951** (issue #2950) — adversarial-critique re-dispatch on the 4-prompt role set. Critique committed as `docs/redesign/_critique/cycle-146-adversarial-critique.md` (223 LOC). The cycle 146 dispatch (#2947 / PR #2948) had been LOST AT POST due to `copilot-feedback-dispatch-comment-write-permission-gap` (NOVEL@1 cycle 147); re-dispatch with corrected delivery channel (commit-as-file vs post-comment) recovered substantive output.
- **PR #2953** (issue #2952) — `v2-prompt-contract-check` crate implementation per cycle 146 design scope (`docs/redesign/_notes/cycle-146-prompt-contract-check-design.md`) + cycle 147 mid-session correction (no `v2-channel-router` modification — `schema --format json` subcommand already exists at `tools/rust/crates/v2-channel-router/src/main.rs:629`).

This is the 3rd consecutive two-track composition cycle (146 dispatch+scope, 147 redispatch+dispatch, 148 absorb+land). NOVEL@3 hardening signal for `two-track-composition`.

## Track 1: PR #2951 critique — per-finding absorption ledger

Critique structure: 3 convergent findings + 5 Lens-1 + 5 Lens-2 + 6 Lens-3 + 4 cross-cutting + 1 counterfactual. 24 distinct evaluable findings.

Verdict legend:
- **AGREE-ACT-NOW** — finding correct and small enough to fix this cycle or next
- **AGREE-DEFER** — finding correct; action queued to a named future cycle/phase
- **AGREE-RECORD** — finding correct; the action is to record the constraint, not change code
- **AGREE-WITH-CARVEOUT** — finding directionally correct but architecture intentionally accepts the tradeoff
- **DISAGREE** — finding incorrect or based on misreading
- **TOOL-SCOPED** — finding partially addressed by tool landed cycle 148+ (PR #2953)

### Convergent findings (top of file)

**C1: "Prompts overstate enforcement and understate runtime freedom"** — AGREE-RECORD + TOOL-SCOPED.
Critique correct: `v2-channel-router/src/main.rs:362-364, 370-378` only enforces payload-is-object + required-keys-exist; no types, no nested sub-keys. PR #2953 (landing cycle 148) addresses the EASIEST layer (key-set alignment) but does not enforce types or nested shapes. The harder enforcement gap is the deepest valid critique in this lens. Action: record this gap as named follow-up in v2-channel-router enforcement work (cycle 149+ candidate), AND record that prompts should be honestly downgraded where they claim stronger guarantees than the router actually enforces. Concrete fix candidates surface in L3.2 below.

**C2: "Template-mirror parity is causing semantic drift that threatens operator reliability"** — AGREE-DEFER.
Critique correct. Already named cycle 145 as `dispatch-brief-template-mirror-tradeoff` NOVEL@1; this critique sharpens it to "design-level error" (see L2.3). Acting on it now means re-authoring role prompts to drop literal-mirror in favor of role-native taxonomy — a large change against landed prompts. Defer to a named future prompt-iteration cycle (post first-end-to-end-run measurement). The dispatch-brief language correction in L2.4 IS acted on now (forward).

**C3: "Cycle-1 minimal scope is not held as hard boundary; punctured by multi-cycle side effects"** — AGREE-WITH-CARVEOUT.
Critique correct in observation: planner `forward-notes`, executor `dispatch-firing`, curator multi-output telemetry all reach beyond strict single-cycle scope. But the carveout is real: directive #2937 explicitly authorized dispatch-firing as cycle-1 architecture (item #3); planner forward-notes was added intentionally (the orchestrator NEEDS to communicate to its own next cycle); curator's multi-surface output is the close-out cleanup itself which has always been multi-surface. The honest fix is L2.3 / X2: redefine "minimal" honestly — "minimal channel schema, not minimal temporal scope". Action: when authoring docs that frame cycle-1 scope, use the channel-schema definition not temporal-scope definition.

### Lens 1 — Role-boundary discipline integrity

**L1.1: "Curator's no-cross-channel-reads rule is soft, not hard"** — AGREE-ACT-NOW (textual fix in next prompt-iteration cycle).
Concrete textual fix proposed: tighten `prompts/v2/curator-prompt.xml:133-137` and `:535-539`. "Unless harness provides" and "should be avoided" are policy backdoors. Removal is small and direct-push-zone authorized. **Queue for cycle 149+** as a small prompt edit; not blocking the landing of PR #2953.

**L1.2: "Executor dispatch-firing is a sanctioned side-channel that bypasses reducer-governed coordination"** — AGREE-WITH-CARVEOUT.
Critique correct: dispatch-firing is structurally a second coordination plane outside channel-reducer discipline. But this is intentional — directive #2937 explicitly authorized it as cycle-1 architecture. The fix is documentation honesty: the architecture docs need to NAME dispatch-firing as a sanctioned side-channel rather than treat it as inside channel-reducer discipline. Action: record in `2-selection.md` companion doc (NOT in 2-selection.md per cycle 120 L2 — but in a separate architecture-notes doc) that dispatch-firing is an explicit side-channel.

**L1.3: "Reconciler fallback/error path undermines single-cycle determinism"** — AGREE-ACT-NOW (small prompt + router edit candidate).
Critique correct: reconciler `prompts/v2/reconciler-prompt.xml:290-295` allows best-effort continuation with "only confidently collected events"; router enforces only key presence, not source completeness. Concrete fix: reconciler must emit explicit completeness metadata (e.g., `inbound-completeness-marker` required key with values `complete | partial | quiet`). Requires (a) prompt edit + (b) router `required_payload_keys` extension. **Queue for cycle 149+** as a real prompt+router pairing; defer to reconciler-iteration cycle.

**L1.4: "Planner's per-role-task object names work; it does not enforce inter-role coordination semantics"** — AGREE-RECORD + TOOL-SCOPED.
Critique correct: router checks only key presence, not per-role sub-object semantic completeness. This is the same gap as C1. Fix is enforcement-tool work: extend v2-prompt-contract-check OR a new planner-output-validator that verifies per-role sub-object structure (which roles are addressed, action fields, etc.). **Queue as design-input for tool extension cycle 149+.**

**L1.5: "No upstream inputs for reconciler is true in structure, but used as a confidence claim broader than warranted"** — AGREE-RECORD.
Critique correct: the cycle 145 review language overclaimed. "Necessary but not sufficient for boundary integrity." Action: future review summaries should be more careful about the gap between structural absence and behavioral guarantee. Recorded.

### Lens 2 — Template-mirror approach failure modes

**L2.1: "Executor has more semantic mismatches than the two already logged"** — AGREE-ACT-NOW (textual fixes in next prompt-iteration cycle).
Four additional patterns named:
1. Identity-term mismatch at role core (`prompts/v2/executor-prompt.xml:194` says "EXECUTION JUDGMENT" but tag is `<substantive-focal-judgment>`) — concrete textual fix.
2. Decomposition tag mismatch with polymorphic action taxonomy (`:260` `<per-role-tasks-decomposition>` contains execution-shape DSL not per-role decomposition) — concrete textual fix or section rename.
3. Reference-status over-anchors to mirror language (`:492-497`) — small meta-text fix.
4. Constraint vocabulary drift from role function (`:429-433` `dispatch-fit-consideration` is routing logic not a constraint) — section reclassification.
All four are direct-push-zone authorized small edits. **Queue for cycle 149+ as a single executor-iteration commit** (4-fixes-in-one). Not blocking PR #2953 landing.

**L2.2: "Curator shows opposite failure mode: mirror preserved but complexity ballooned"** — AGREE-DEFER.
Curator 699 LOC could have been 400-500 with structural reference + rename freedom. Acting on this means re-authoring curator significantly. Defer to a post-measurement curator-iteration cycle. Worth recording: if first end-to-end-run shows curator semantic noise causes operator errors, this becomes priority. If curator runs cleanly, the size is documentary cost only.

**L2.3: "Template-mirror is the wrong organizing principle for role prompts"** — STRONGEST CLAIM. AGREE-RECORD.
The argument: each role should optimize for decision class / input trust model / output contract hardness / failure semantics — NOT isomorphic anatomy. This is a Phase 2 / Phase 3 candidate-selection-level claim. Re-examining is high-stakes (the cycle 121 candidate selection chose single-prompt → multi-prompt with mirroring). Action: record as named design-input for either (a) a future v2 prompt iteration once first end-to-end-run produces evidence, or (b) flag to Eva if she wants to revisit the prompt-architecture choice now. Lean toward (a) — let runtime data inform the call. Recorded in this _notes; not escalated to question-for-eva because EVA-DEFAULT-AUTONOMY directs resolving design-space questions yourself unless genuinely Eva-only.

**L2.4: "The dispatch brief instruction likely caused this outcome — better instruction proposed"** — AGREE-ACT-NOW.
The proposed alternative wording is concrete and immediately actionable for future dispatches:
> "Preserve contract-equivalent sections (role identity, inputs, output contract, constraints, session structure), but rename/repartition sections so tag names match role semantics exactly. Prefer role-native structure over literal template parity. If a planner section is inapplicable, drop or replace it and justify in meta."

**Action: adopt for all future role-prompt or other multi-instance prompt dispatches.** Recorded as dispatch-brief discipline addendum.

**L2.5: "If mirror retained, impose hard anti-drift discipline via section-tag semantic fidelity check"** — AGREE-RECORD + TOOL-SCOPED.
The proposed check ("every top-level XML tag name must be semantically true for section content; if reused from planner, include explicit `<semantic-adaptation-note>` proving equivalence; reject prompts where tag names one function but content performs another") is a natural extension to v2-prompt-contract-check (or a sibling tool). Concrete name: `v2-prompt-tag-semantic-fidelity`. Out of scope cycle 148 landing; recorded as named tool candidate for cycle 149+ (after first end-to-end measurement to see if the underlying problem actually causes runtime issues).

### Lens 3 — Reducer-rule + cycle-1 minimal scope adherence

**L3.1: "Required-keys alignment is real; behavior-level contract adherence is overstated"** — AGREE-RECORD + TOOL-SCOPED.
Same as C1. PR #2953 (landing this cycle) addresses the easier layer (key-set alignment) — the live tool reports 4/4 prompts contract-aligned. Behavior-level enforcement (types, nested shapes) remains a gap.

**L3.2: "Cargo-cult declarations are present"** — AGREE-ACT-NOW (textual fixes in next prompt-iteration cycle).
Three concrete cargo-culted overclaims cited:
- Reconciler `:211-213` says non-array values cause rejection — not true in router. **Fix: rephrase as "should be array" or remove the claim until router enforces it.**
- Planner `:359-360` attributes structural completeness to router — not true. **Fix: rephrase as planner-internal discipline not router-enforced.**
- Executor `:418-420` implies artifacts array discipline with only narrow emptiness conditions — not runtime-enforced. **Fix: rephrase as executor-internal discipline.**
Three small textual edits. **Queue for cycle 149+ as a single across-prompts honesty-pass commit.** Not blocking PR #2953 landing.

**L3.3: "Executor `dispatches-fired` is a cycle-1 minimal leak, not just optional telemetry"** — AGREE-WITH-CARVEOUT.
Same as C3 + L1.2. Authorized side-channel; framing fix not removal.

**L3.4: "Planner `forward-notes` is explicit multi-cycle horizon data"** — AGREE-WITH-CARVEOUT.
Same as C3. Authorized; framing fix not removal.

**L3.5: "Curator `cycle-close-summary` is under-decomposed with memory payload"** — AGREE-DEFER.
Curator output mixes 3 concerns (channel payload data / execution telemetry / outbound narrative). Could be split into 3 distinct surfaces. Defer to post-measurement curator-iteration cycle (same as L2.2).

**L3.6: "Reconciler quiet/best-effort path can hide collection incompleteness"** — AGREE-ACT-NOW.
Same fix as L1.3 (completeness metadata required key). Same queue.

### Cross-cutting observations

**X1: "Hard boundaries are currently expressed in prose, not checked at boundary tools"** — AGREE.
Headline finding driving (a) PR #2953 landing this cycle (first step) and (b) v2-channel-router enforcement extensions queued for cycle 149+. Recorded.

**X2: "Two architectural truths in conflict — cycle-1 minimal vs multi-cycle dispatch composition"** — AGREE-ACT-NOW.
Honest fix: redefine "minimal" as "minimal channel schema, not minimal temporal scope". Action: when authoring further cycle-1 framing docs or revising existing ones, use the channel-schema definition. (Will not back-edit cycle 146 scope-doc per journal-immutability discipline; new docs use the corrected framing.)

**X3: "Role set optimizes for reviewable structure over executional determinism"** — AGREE-RECORD.
Same point as L2.3. Recorded.

**X4: "Single-writer-per-channel is intact in mapping, but not sufficient as safety claim"** — AGREE-RECORD.
Side effects (dispatches, comments, docs) are outside channel discipline. Architecture acknowledges this (X1, L1.2); the safety claim should not be overstated. Recorded.

### Counterfactual (what would weaken the critique)

Useful self-aware criteria from the critic, paraphrased:
- If `v2-channel-router` (or role-driver pre-write) validated required-key **types** AND nested shapes, contract-overstatement findings would collapse from load-bearing to documentation polish.
- If curator cross-channel-read exceptions were removed entirely, role-boundary critique would be less severe.
- If executor dispatch-firing were isolated to a dedicated role OR explicitly modeled as sanctioned side-channel in architecture docs, role-boundary critique would be less severe.
- If runtime evidence (cycle 148+) shows semantic label mismatches have no measurable effect on role behavior quality, mirror-template costs become mostly aesthetic.

**Action: this counterfactual sets the priority order for cycle 149+ work** — type/shape enforcement extension is the highest-leverage; curator-exception removal is small and direct; dispatch-firing doc is small and direct; runtime evidence comes naturally from first end-to-end run.

## Track 1 absorption summary

24 findings evaluated. Verdict distribution:
- AGREE-ACT-NOW: 7 (L1.1, L1.3, L2.1, L2.4, L3.2, L3.6, X2)
- AGREE-DEFER: 3 (C2, L2.2, L3.5)
- AGREE-RECORD: 6 (L1.5, L2.3, L2.5, X1, X3, X4)
- AGREE-WITH-CARVEOUT: 4 (C3, L1.2, L3.3, L3.4)
- AGREE-RECORD + TOOL-SCOPED: 3 (C1, L1.4, L3.1)
- DISAGREE: 0

**Zero disagreements** is itself a signal: the critique surfaced only valid concerns; the recurrence test for `dispatch-brief-template-mirror-tradeoff` returned a strong critique that hardens both the pattern and the v2 architecture critique. The critique is high-signal — Copilot's adversarial-feedback-only dispatch worked exactly as intended.

**Forward-implications captured into cycle 149+ candidates:**
1. Executor 4-fix textual iteration (L2.1).
2. Across-prompts honesty-pass (L3.2 + L1.1 — remove overclaims and policy backdoors).
3. Reconciler completeness-metadata pairing (L1.3 + L3.6 — prompt edit + router required-key extension).
4. v2-channel-router enforcement extension design scope (C1 / L3.1 / counterfactual — types + nested shapes).
5. `v2-prompt-tag-semantic-fidelity` tool design scope (L2.5).
6. Dispatch-brief discipline addendum adopted (L2.4 wording).
7. Honest cycle-1-scope-redefinition recorded (X2).

## Track 2: PR #2953 review verdict

### Build and test

```
cargo build -p v2-channel-router -p v2-prompt-contract-check  → success
cargo test -p v2-prompt-contract-check                         → 11 unit + 1 integration green
cargo run -p v2-prompt-contract-check -- --strict              → 4/4 prompts contract-aligned
```

### Scope conformance

| Design spec | Implementation | Verdict |
|---|---|---|
| Check 1: output-channel ↔ writer-map | `infer_role_from_prompt_filename` + `map_channel_by_writer` | ✓ |
| Check 2: required-output-keys set equality | `parse_output_contract_required_keys` + `compare_keys` | ✓ |
| Check 3+4: input-channel + input-required-keys | Implemented BUT current prompts use non-parseable input structure; deferred via `skipped_checks` (transparently surfaced in output) | ✓ (cycle-1 minimal correctly scoped) |
| Choice A2: runtime-binary truth source | Uses `v2-channel-router schema --format json` | ✓ |
| Choice B1: proper XML parse | `quick-xml` | ✓ |
| Choice C1: separate crate | `tools/rust/crates/v2-prompt-contract-check/` | ✓ |
| Curator multi-surface fallback | `parse_surface_required_keys_for_channel` (~70 LOC; beyond spec — added by implementer to handle curator's wrapper shape) | ✓ (real curator handling) |
| Unit tests for: match, extra, missing, channel typo, JSON format | 11 unit tests | ✓ |
| Integration test against live prompts + binary | 1 integration test, asserts strict 4/4 alignment | ✓ |

### LOC measurement (recurrence test for `magnitude-prediction-precision-is-shape-dependent-not-flat`)

| Component | Predicted band | Actual | Verdict |
|---|---|---|---|
| Crate prod source (excl. cfg(test)) | 400-600 | ~810 LOC | OVER 35% (1.35× upper) |
| Crate test source (cfg(test) + tests/) | 300-500 | ~290 LOC (197 in-mod + 92 integration) | UNDER 3% (just below band) |
| v2-channel-router `schema --json` subcommand | 30-60 | 0 (subcommand already existed; cycle 147 correction) | N/A |

Prod-LOC overrun reasons (defensible):
- Curator multi-surface fallback (~70 LOC) — not in spec, real value.
- Structured CheckError / Mismatch / JsonReport types (~80 LOC) — proper error handling adds prod LOC for proper quality.
- `parse_input_source_contracts` ~75 LOC for input-checks-3+4 infrastructure even though deferred — implementer chose to leave the structure in place so future enabling is small.

The prediction undershoot on prod is real data for the verification-tool family. Updated band for next verification-tool prediction: 700-1100 LOC prod (matching cycle 147 retroactive estimate after spec correction); 250-500 LOC test.

### Quality observations

- **2 clippy warnings + 1 useless-conversion** — collapsible-match style suggestion (no functional issue), useless OsString conversion (test code). Style nits; not blocking.
- **BTreeSet** for required keys gives alphabetical ordering — sensible default for diff legibility.
- **EXTRA-IN-PROMPT** case marked `(decorative; channel-router does not enforce)` in text output — accurately describes the runtime semantics (router only enforces presence, not absence). Strict mode correctly fails on this.
- **Integration test runs `cargo build` internally** (~3s overhead) — acceptable cost for full-stack verification.

### Adversarial-critique connection

PR #2951 critique's strongest convergent finding (C1: prompts overstate enforcement) is exactly the gap this tool starts to close. PR #2953 addresses the EASIEST layer (key-name alignment). The harder layers (type enforcement, nested shape enforcement) remain unaddressed — and ARE explicitly named in cycle 149+ candidate #4 (v2-channel-router enforcement extension design scope).

### Verdict: LAND (no iteration)

Squash-merge via `gh pr merge 2953 --squash --delete-branch --admin` per cycle 145 direct-push-zone authorization pattern (`tools/v2/` and `tools/rust/crates/v2-*` are direct-push-zones per redesign prompt SECTION 2).

## Housekeeping

- PR #2951 closed cycle 148 with closure-link comment (critique findings absorbed per-finding into this _notes; verdict ledger lives at `docs/redesign/_notes/cycle-148-two-track-absorption-and-landing.md`).
- Issue #2950 auto-closed by GitHub on PR merge (if merged); if PR closed without merge, will close issue with closure-link comment to this _notes.
- Issue #2952 will be closed by PR #2953 merge.
- Issue #2954 (Eva-authored manual trigger at 00:21 UTC May 15; no cycle ran on it) closed cycle 148 with closure-link comment naming #2955 as the cycle 148 issue (the one this cycle ran against; cron-fired at 03:00 UTC).

## Pattern updates

- `two-track-composition` NOVEL@2 cycle 147 → NOVEL@3 cycle 148 (third consecutive two-track cycle; cycle 146 = dispatch+scope, cycle 147 = redispatch+dispatch, cycle 148 = absorb+land). **HARDENING THRESHOLD CROSSED**: NOVEL@3 is the cycle 92 hardening pattern (NOVEL→HARDENING at @3). Recording as HARDENING-AT-3.
- `dispatch-brief-template-mirror-tradeoff` recurrence test RETURNED. Critique L2.3 / L2.4 hardens the original observation: literal-mirror IS structurally inferior to structural-reference-with-rename-freedom for cross-role prompts. **HARDENED.**
- `magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 147 prediction (700-1100 prod) is the corrected band; cycle 148 actual 810 prod within corrected band. **First in-band recurrence for verification-tool family.**
- `proactive-tool-extraction-from-recurring-inspection` NOVEL@1 cycle 146 → DISPATCHED cycle 147 → LANDED cycle 148. Full arc closed. **HARDENING-AT-1 (single instance landing-arc closure).**
- `copilot-feedback-dispatch-comment-write-permission-gap` NOVEL@1 cycle 147 → recurrence test in flight via PR #2951 (commit-as-file recovery channel). PR #2951 successfully delivered substantive critique via commit-as-file, verifying the recovery channel works. **Recovery channel HARDENED-AT-1 cycle 148.**

## Process

- No cargo/clippy on prompt-channel cycles cycle 147 lesson preserved (cycle 148 cargo invocations are review-of-Rust-PR work; legitimate).
- Cycle 120 L2 preserved (2-selection.md untouched).
- Cycle 128 lesson preserved (.scratch via Write tool for ephemeral files).
- Cycle 133 lesson interpreted: "no cargo on non-Rust-PR cycles". Cycle 148 reviews Rust PR; cargo build/test/clippy is legitimate verification. Recording this clarification.
- Cycle 134 lesson preserved (audit-repo HEAD via gh api).
- Cycle 137 lessons preserved (no heredoc redirect / for-loop / shell-syntax-string).
- Journal-immutability preserved (Edit tool with cycle 147 end-line anchor when appending to journal).
- Anti-overstatement audit explicit (this _notes ends with "What cycle 148 does NOT do" enumeration).

## What cycle 148 does NOT do

- Does NOT enact any of the 7 AGREE-ACT-NOW findings (queued for cycle 149+ as separate disciplined commits).
- Does NOT modify role prompts (curator/executor/reconciler/planner XML all untouched).
- Does NOT modify v2-channel-router (no enforcement extension; cycle 149+ candidate).
- Does NOT modify cycle-runner harness (PR-required forbidden zone).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT modify 2-selection.md (cycle 120 L2).
- Does NOT modify cycle 146 scope-doc (journal-immutability discipline).
- Does NOT escalate L2.3 architecture-revisit to Eva (EVA-DEFAULT-AUTONOMY directs resolving design-space questions yourself).
- Does NOT predict cycle 149+ priority order beyond the 7-item forward-implications list.
- Does NOT measure runtime quality of role prompts (no end-to-end run; cycle 149+).
- Does NOT advance HARDENED status of patterns beyond what fits the @3 / arc-closure thresholds.
- Does NOT close audit-cycle-219 blocked-priority (audit HEAD `72cda153` unchanged; cycle 149+ check).

## Forward priorities for cycle 149+

1. **Audit cycle 219 critique absorption** — if audit HEAD advances. Still blocked at `72cda153` from May 13 (audit cron intermittent for 7+ expected cycles).
2. **First end-to-end run + first measurement** — required before any further role-prompt iteration; otherwise we iterate on imagined problems. The cycle-runner harness rewrite (forbidden zone) is the gating dependency; design scope for that rewrite is itself overdue. **Highest-leverage priority cycle 149+.**
3. **Cycle-runner harness rewrite design scope** (forbidden zone; PR-required; multi-cycle arc; need to PR-author the design first).
4. **AGREE-ACT-NOW finding #1: Executor 4-fix textual iteration** (L2.1 — identity-term mismatch, decomposition tag, reference-status, constraint vocabulary). Single commit.
5. **AGREE-ACT-NOW finding #2: Across-prompts honesty-pass** (L1.1 + L3.2 — curator policy-backdoor language, reconciler/planner/executor cargo-culted overclaims). Single commit.
6. **AGREE-ACT-NOW finding #3: Reconciler completeness-metadata pairing** (L1.3 + L3.6 — prompt edit + v2-channel-router `required_payload_keys` extension). Two-PR pairing.
7. **AGREE-ACT-NOW finding #4: Dispatch-brief discipline addendum adopted** (L2.4 wording). Document-only.
8. **AGREE-ACT-NOW finding #5: Honest cycle-1-scope-redefinition recorded** (X2). Document-only.
9. **AGREE-RECORD: side-channel architecture-notes doc** (L1.2 + X4). Document-only.
10. **TOOL-SCOPED follow-up: v2-channel-router enforcement extension design scope** (C1 / L3.1 / counterfactual — types + nested shapes). Design scope first, then implementation.
11. **TOOL-SCOPED follow-up: v2-prompt-tag-semantic-fidelity tool design scope** (L2.5). Design scope first.
12. **AGREE-DEFER queue (post-first-measurement)**: curator complexity rework (L2.2 + L3.5), template-mirror architecture-revisit (L2.3 / C2). Hold until runtime evidence.
13. **Cycle 120 L2 preserved** (no recursive annotation of 2-selection.md).
