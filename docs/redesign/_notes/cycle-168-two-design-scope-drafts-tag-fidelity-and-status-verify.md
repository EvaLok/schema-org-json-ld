# Cycle 168 _notes — two design-scope drafts (v2-prompt-tag-semantic-fidelity + v2-cycle-runner status+verify)

**Cycle:** 168 (two-track composition; HARDENING-AT-21 if both tracks land — both did)
**Date:** 2026-05-17
**Cycle issue:** [#2981](https://github.com/EvaLok/schema-org-json-ld/issues/2981)
**Predecessor:** cycle 167 (commit `da007bdb`)
**Forward priorities closed:** cycle 167 priority #2 (v2-prompt-tag-semantic-fidelity tool design scope, L2.5 cycle 148 — 19-cycle carry-forward) + cycle 167 priority #3 (status + verify v2-cycle-runner subcommand design scope, cycle 149 §4 + §9.9 — 18-cycle carry-forward).

## Cycle shape

Cycle 168 is a **straight-pair two-design-scope-drafts** variant: both Tracks main-side, both textual design-scope documents, both close ≥18-cycle carry-forwards. RECURRENCE-AT-2 of the `two-design-scope-drafts` composition shape introduced cycle 158 NOVEL@1 (9-cycle gap; cycles 158 + 168). Intervening cycles 159-167 used other shapes (bounded-mechanical + design-scope; bounded-mechanical + dispatch; external-deliverable-absorption pair; etc.).

This is also straight-pair HARDENING-AT-7 (cycles 162-168) and two-track-composition HARDENING-AT-21 (17 consecutive post cycle 151 exception: 152-168). 53rd consecutive HONORING of named forward priority (cycles 115-168).

No dispatch this cycle (cycle 166's PR #2979 remains DRAFT with no CI; cycle 168+ priority #1 absorption deferred). Audit HEAD unchanged at `bc8fda63` (cycle 167 reading; cycle 168 re-read confirmed); priority #8 (audit-engagement single-track variant) does not fire.

## Track 1 — v2-prompt-tag-semantic-fidelity design scope

Cycle 168+ forward priority #2 (was cycle 166+ #4; cycle 165+ #5; ... ultimately L2.5 from cycle 148 PR #2951 absorption — 19-cycle carry-forward 148→167 first design-scope authored cycle 168).

**Target file:** `docs/redesign/_notes/v2-prompt-tag-semantic-fidelity.md` (298 lines, direct-push commit `b01fe06c`).

**Key design decisions:**
- **Standalone binary over extension:** `v2-prompt-tag-semantic-fidelity` as a sibling to `v2-prompt-contract-check`, not an extension flag on it. Rationale: single-responsibility (contract-check = channel-schema alignment; tag-fidelity = tag-content semantic alignment). Different axes; coupling violates the discipline that justifies the per-primitive split (cycle 161 X5 carveout precedent; cycle 165 dispatch-archive vs dispatch-sync sibling precedent).
- **TOML manifest at `prompts/v2/tag-semantics.toml`:** separates declarative semantics from prompt content. Declares per-tag intent + required-in-roles / allowed-in-roles / expected-children. Implementation cycle 169+ extracts initial manifest by scanning current `prompts/v2/*-prompt.xml`; manifest baseline is canonical state preservation, future iterations declare drift via `<semantic-adaptation-note>` children.
- **Two-tier check:** Tier 1 mechanical (unknown-tag / missing-required / missing-expected-child / role-not-allowed; fail-closed under `--strict`); Tier 2 advisory (length-plausibility + keyword-overlap heuristics; warning-only).
- **`<semantic-adaptation-note>` pass-with-note semantics:** presence suppresses Tier 1 unknown-tag check but does NOT suppress Tier 2 heuristics (existence of note acknowledges divergence; doesn't prove fidelity).
- **CLI shape:** `check / schema / list-tags` subcommands; JSON + text output; --strict gate.
- **Test plan:** unit + integration + real-prompt regression layers.
- **Six open questions** deferred to implementation: manifest location, Tier 2 threshold calibration, cross-role override policy, CI integration timing, parsing-utility sharing with contract-check, adaptation-note text-quality enforcement.
- **Ordering vs other v2-* tools:** lands BEFORE cycle 168+ priority #10 (honesty-pass on v2 role prompts), so honesty-pass can consume tool findings.

**Out of scope per design:** does NOT implement; does NOT author manifest; does NOT modify v2 prompts; does NOT extend contract-check; does NOT couple to router schema; does NOT consume v1 production prompts.

**Provenance density:** Track 1 explicitly grounds in cycle 148 L2.5 deferral rationale + cycle 153 first-end-to-end gate that justified the wait + cycles 155-165 critique-absorption stream evidencing the need (drift accumulates as documentary debt; critique sessions re-discover it). Value proposition framed as **prevention** so future critique compute is spent on substantive concerns rather than re-finding tag-misnaming.

## Track 2 — v2-cycle-runner status + verify subcommand design

Cycle 168+ forward priority #3 (cycle 167+ priority #5; ... ultimately cycle 149 §4 + §9.9 high-level scope — 18-cycle carry-forward 150→167; sharpened to implementation-ready design cycle 168).

**Target file:** `docs/redesign/_notes/v2-cycle-runner-status-verify-design.md` (261 lines, direct-push commit `76f5e0f1`).

**Key design decisions:**
- **`status` subcommand contract:** answers "what is the runner's most recent or specified cycle's execution state?" Default reads `state/v2-cycle-runner/last-cycle.json` (O(1)); `--cycle N` does linear scan of `state/v2-cycle-runner/cycle-history.json`; `--include-primitives` extends to 4 primitive state surfaces. Text + JSON output schemas specified.
- **`verify` subcommand contract:** answers "did cycle N complete cleanly?" Explicit `--cycle N` required (no accidental "latest" passes). 9-assertion algorithm: cycle-N-in-history / status-completed / no-halt-step / no-halt-class / no-halted-after-role / all-10-substeps-traced / state-audit-not-hard / super-step-history-has-cycle-N / per-role-history-has-cycle-N.
- **`--strict` semantics:** non-strict default exits 0 with `verdict: "dirty"` in JSON (CI-friendly: failures surface in output); `--strict` exits 1 (gate-friendly: any failure halts).
- **Cycle 149 §9.2 resolution (primitive discovery):** PATH lookup is the default; reuses existing `--primitive-bin` override layer; **no new infrastructure**. Verify does NOT shell to primitives — it reads their state files directly.
- **Cycle 149 §9.9 resolution (verify in-binary vs audit-side):** in-binary main-side verify is fast self-check at cycle-close; audit-side counterpart is separate adversarial / cross-cycle pattern check on its own cron. Different consumers, different needs, different trust models. Audit-side counterpart is out of scope for this design and not blocking.
- **State files consumed table** (5 files: last-cycle.json + cycle-history.json + super-step.json + roles/*-history.json + super-step-history.json) with owners and read patterns.
- **Test plan:** 14 unit tests (6 status + 8 verify) + 1 integration test reusing existing run-dry-run fixture.
- **Six open questions** for implementation: `--all` sweep modes, orphan-state detection, color output, schema-version evolution policy, commit-discipline carveout reaffirmation.
- **Stale-doc-string follow-up** at `tools/rust/crates/v2-cycle-runner/src/main.rs:413-416` (says `run` is "deferred" when it's been implemented since cycle 151); implementation PR cycle 169+ should fix.

**Out of scope per design:** does NOT implement; does NOT modify run/init/schema subcommands; does NOT change state file shapes; does NOT add CI gates; does NOT define audit-side counterpart; does NOT touch v1 cycle-runner.

**Implementation effort estimate:** ~250-400 LOC + tests; one focused cycle likely sufficient.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 — v2-prompt-tag-semantic-fidelity.md | 298 LOC | `b01fe06c` |
| Track 2 — v2-cycle-runner-status-verify-design.md | 261 LOC | `76f5e0f1` |
| `cycle-168-*.md` (this _notes) | ~205 lines (cycle-close estimate) | cycle-close |
| Total cycle 168 textual output | ~764 lines | 3 substantive commits expected |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 168 datapoints: two design-scope drafts, both 261-298 LOC. Comparable to cycle 158 (the prior `two-design-scope-drafts` instance — Track 1 230 LOC v2-state-retention-policy + Track 2 ~150 LOC v2-critique-task-class-taxonomy). Cycle 168 is slightly larger per-Track (avg 280 LOC vs cycle 158's 190 LOC), consistent with both Track 1 (19-cycle carry-forward) and Track 2 (18-cycle carry-forward) absorbing accumulated context.

## Pattern updates this cycle

- **`two-track-composition` HARDENING-AT-21** cycle 168. 17 consecutive post cycle 151 exception (152-168). 22 of 23 in arc 146-168 (only cycle 151 was single-track).
- **`straight-pair-closure-as-default-two-track-shape` HARDENING-AT-7** cycle 168 (162-168). Modal shape across 7 consecutive cycles.
- **`two-design-scope-drafts` composition variant RECURRENCE-AT-2** cycle 168. First instance cycle 158 NOVEL@1; second instance cycle 168 (9-cycle gap). The composition variant is now exercised across non-adjacent cycles with intervening cycles using other composition shapes — RECURRENCE not chained HARDENING.
- **`tool-first-live-invocation-surfaces-state-drift-in-adjacent-tool` NOT-EXERCISED-FOR-DISCOVERY** cycle 168 (no new latent-tool first-live this cycle). Carries forward at NOVEL@1.
- **`straight-pair-with-dispatch-variant` NOT-EXERCISED** cycle 168 (no dispatch). Carries forward at RECURRENCE-AT-2.
- **`directive-2937-copilot-role-bifurcation` NOT-EXERCISED** cycle 168 (no new Copilot instance). Carries forward at RECURRENCE-AT-2.
- **`audit-as-priority-1-input` NOT-EXERCISED** cycle 168 (audit HEAD unchanged at `bc8fda63`). Carries forward at HARDENING-AT-3.
- **`external-deliverable-absorption-pair` NOT-EXERCISED** cycle 168. Carries forward at NOVEL@1.
- **`directive-2937-track-1-dispatch-fit-application` NOT-EXERCISED** cycle 168. Carries forward at NOVEL@1.
- **`tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED** cycle 168. Carries forward at RECURRENCE-AT-6.
- **`agree-act-now-bounded-fix-via-tracksecond-pattern` NOT-EXERCISED** cycle 168. Carries forward at RECURRENCE-AT-7.
- **`pre-flight-vs-mid-cycle-halt-distinction` NOT-EXERCISED** cycle 168. Carries forward at RECURRENCE-AT-2.
- **`fail-open-on-tool-internal-anomalies-but-fail-closed-on-policy-violations` NOT-EXERCISED** cycle 168. Carries forward at NOVEL@1.
- **`boundary-error-text-classifier-mismatch-found-via-test-implementation` NOT-EXERCISED** cycle 168. Carries forward at NOVEL@1.
- **`agree-defer-priority-becomes-live-after-blocker-clears` NOT-EXERCISED** cycle 168. Carries forward at NOVEL@1.

## Process honoring

- **53rd consecutive cycle of HONORING named forward priority** (cycles 115-168).
- **81st bottleneck-asynchronous cycle** (78-168).
- **58th non-per-candidate-sharpening cycle** (111-168).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 168).
- Cycle 128 lesson preserved (.scratch/ via Write tool — 1 ephemeral file at session-start; further ephemerals at cycle-close).
- Cycle 133 lesson preserved (no cargo invocations cycle 168 — this is a design-scope-only cycle with no code changes).
- Cycle 134 lesson application: audit-repo HEAD via `gh api repos/EvaLok/schema-org-json-ld-audit/commits/master` at session-start; `bc8fda63` unchanged from cycle 167.
- **Cycle 137 lessons re-validated cycle 168 22-cycle-running** (137 + 147-168).
- **Cycle 149 in-cycle CWD-drift lesson not exercised cycle 168** (no cargo; no cd).
- Cycle 151 date-test-value lesson preserved (no test code touched).
- **Cycle 152 disk-format-read-source lesson preserved cycle 168** (read v2-cycle-runner/src/main.rs via Read tool at specific line offsets; no wrapper-script invocations).
- Cycle 153 multi-Edit-fallback lesson NOT exercised (no Edits — both Tracks were Write of new files).
- Cycle 154 `gh api graphql` subprocess pattern NOT exercised cycle 168 (no dispatch).
- Cycle 155 dispatch-return-detection lesson preserved.
- Cycle 156 anti-overstatement audit enumeration discipline preserved (this _notes explicitly enumerates "What cycle 168 does NOT do" + state-files-consumed table for verify + assertion list).
- Cycle 157 audit-HEAD-check at session-start preserved (executed; `bc8fda63` unchanged).
- Cycle 158 design-scope precedent re-exercised cycle 168 (two-design-scope-drafts shape — second instance after 9-cycle gap).
- Cycle 159 test-pattern-mirror lesson preserved (Track 2 test-plan section references existing v2-cycle-runner test idioms).
- Cycle 160 schema-duplication observation preserved.
- Cycle 161 paired-closure pattern preserved (both Tracks close paired carry-forward priorities).
- Cycle 161 X5 carveout pattern preserved AND cited in Track 1 §3.1 + Track 2 §8 §7 (commit-discipline carveout reaffirmed in verify scope).
- Cycle 162 state-audit session-start wiring preserved (Track 2 design references existing wiring; no changes proposed).
- Cycle 163 implementation-discovery-via-testing lesson NOT applicable (no implementation cycle 168).
- Cycle 164 forward-priority-renumbering preserved (this _notes renumbers cycle 167's list).
- Cycle 164 dispatch-receipt atomic-commit pattern NOT exercised (no dispatch).
- Cycle 165 PR-absorption-shape preserved as reference for cycle 169+ PR #2979 absorption.
- Cycle 166 first-live-invocation-shape preserved (no new latent-tool first-live; observation forward-watch from cycle 167 carries; if cycle 169+ adds a third instance, pattern naming triggered).
- Cycle 167 latent-tool-observation forward-watch preserved (carries with no exercise this cycle).
- Journal-immutability discipline preserved (cycle 168 APPENDS to today's journal containing cycles 163-167; prior cycle sections NOT back-edited).
- **Two-track composition continued** — cycle 168 is 17th consecutive post cycle 151 exception (HARDENING-AT-21).
- **SECTION 6b list housekeeping NOT invoked cycle 168** — 6 open issues (cycle issue + dispatch #2978 + 4 standing input-from-eva); 1 open PR (#2979 DRAFT). No closure candidates surfaced.

## In-session issues and recoveries

- No permission-gate recoveries cycle 168 (no wrapper-script invocations; no cargo).
- No output-redirection rejections cycle 168.
- All `git add` / `git commit` / `git push` operations clean cycle 168 (3 commits expected: Track 1 + Track 2 + cycle-close).
- All Write operations clean cycle 168 (2 new files via Write; 0 Edits).
- All `gh api` / `gh issue list` / `gh issue comment` operations clean cycle 168.

## Cycle 168 ARTIFACTS

- `docs/redesign/_notes/v2-prompt-tag-semantic-fidelity.md` — Track 1 new (298 LOC; commit `b01fe06c`).
- `docs/redesign/_notes/v2-cycle-runner-status-verify-design.md` — Track 2 new (261 LOC; commit `76f5e0f1`).
- `docs/redesign/_notes/cycle-168-two-design-scope-drafts-tag-fidelity-and-status-verify.md` — new (this _notes; ~205 lines at cycle-close).
- `docs/journal/2026-05-17.md` — appended (cycle 168 section).
- `.scratch/cycle168-*.{md,txt}` — ephemerals (session-start; cycle-close issue comment).
- 3 substantive commits: `b01fe06c` (Track 1) + `76f5e0f1` (Track 2) + cycle-close commit.
- 0 issues closed cycle 168 explicitly (cycle issue closes per existing convention).
- 0 dispatches cycle 168.
- 0 cargo invocations cycle 168.
- 0 Edits cycle 168 (both Tracks were Write of new files).
- 2 Writes cycle 168 (Track 1 + Track 2 _notes files).
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.
- ~6 GitHub API operations cycle 168 (session-start orientation + session-end).

## What cycle 168 does NOT do

1. Does NOT implement `v2-prompt-tag-semantic-fidelity` (cycle 169+ implementation work; tracked as new forward priority).
2. Does NOT author the tag-semantics.toml manifest (part of implementation cycle).
3. Does NOT modify any `prompts/v2/*-prompt.xml` files.
4. Does NOT implement `v2-cycle-runner status` or `v2-cycle-runner verify` subcommands (cycle 169+ implementation work; tracked as new forward priority).
5. Does NOT change any state file shapes consumed by status/verify (they consume existing schemas).
6. Does NOT extend `v2-prompt-contract-check` (per Track 1 §3 sibling-binary decision).
7. Does NOT absorb PR #2979 (still DRAFT with no CI; cycle 169+ candidate).
8. Does NOT run `v2-state-dispatch-sync` (no stale in_flight entries surfaced this cycle; cycle 167 reconciliation holds).
9. Does NOT engage audit-repo cross-perspective filing (audit HEAD unchanged at `bc8fda63`).
10. Does NOT progress AGREE-DEFER queue (gated on real-role-session-measurement; not blocked-but-not-actionable).
11. Does NOT advance coordinated retry/timeout/cancellation arc (C13 + X2).
12. Does NOT advance coordinated structured-error-envelope arc (C6 + C7 + C9).
13. Does NOT advance coordinated resume/recovery arc (C11 + C12 + X1).
14. Does NOT design per-axis archival mechanism (cycle 166+ priority #9; carries forward).
15. Does NOT run honesty-pass on v2 role prompts (gated on PR #2979 landing).
16. Does NOT re-run `v2-prompt-contract-check --strict` (gated on PR #2979 landing + honesty-pass).
17. Does NOT deprecate `Channel::required_payload_keys()` legacy method (gated on the above).
18. Does NOT add `v2-state-dispatch-archive --invocation-id` flag (LOW priority; defer).

## Forward priorities for cycle 169+

Inheriting from cycle 167's renumbered list, minus #2 + #3 (closed cycle 168). 2 NEW priorities added (implementation cycles for both Tracks).

1. **PR #2979 absorption** (cycle 166 Track 2 dispatch; was cycle 167+ #1). Carry-forward. Per cycle 165 PR #2975 absorption precedent shape — per-scope-reference verdict ledger against the cycle 164 design scope. Estimated 1 cycle if PR lands cleanly. Cycle 169-170 candidate depending on Copilot turnaround.
2. **`v2-prompt-tag-semantic-fidelity` IMPLEMENTATION** (NEW cycle 168+; Track 1 design landed cycle 168). Author the crate, the TOML manifest baseline from current `prompts/v2/`, unit + integration + real-prompt regression tests. Estimated 1-2 cycles. CARRIES FORWARD until implemented.
3. **`v2-cycle-runner status` + `verify` IMPLEMENTATION** (NEW cycle 168+; Track 2 design landed cycle 168). Implement both subcommands per the 9-assertion verify algorithm + output schemas; update stale doc-string at main.rs:413-416. Estimated 1 cycle. CARRIES FORWARD until implemented.
4. **AGREE-DEFER queue (post-real-role-session-measurement)** (was cycle 167+ #4).
5. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 167+ #5).
6. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 167+ #6).
7. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 167+ #7).
8. **Audit-engagement substantive-focal single-track variant** (was cycle 167+ #8; gate = audit HEAD changes from `bc8fda63`).
9. **Per-axis archival mechanism design scope** (was cycle 167+ #9).
10. **Honesty-pass on v2 role prompts** (was cycle 167+ #10; gated on PR #2979 landing + benefits from priority #2 implementation landing first per Track 1 §10 ordering).
11. **`v2-prompt-contract-check --strict` re-run post-extension** (was cycle 167+ #11; gated on PR #2979 landing + honesty-pass).
12. **Deprecate `Channel::required_payload_keys()` legacy method** (was cycle 167+ #12; gated on PR #2979 landing + honesty-pass + --strict re-run).
13. **`v2-state-dispatch-archive --invocation-id` flag** (was cycle 167+ #13; LOW priority; defer until v2-cycle-runner archival-wiring).

**Cycle 168 forward priorities CLOSED:**
- Cycle 167+ priority #2 (v2-prompt-tag-semantic-fidelity tool design scope) — closed Track 1.
- Cycle 167+ priority #3 (status + verify v2-cycle-runner subcommand design scope) — closed Track 2.

**Cycle 168 new sub-priorities (net of closures):** +2 (both Tracks now have implementation priorities); -2 (both design-scope priorities closed). Net 0 change in list length; new entries at positions #2 + #3.

**Observational forward-watch items preserved:**
- 2-cycle pattern of latent-tool first-live-invocation (cycle 166 + 167); if a third instance appears cycle 169+, name a pattern and consider a forward-priority audit. Cycle 168 did NOT exercise (no first-live invocation).
- `two-design-scope-drafts` composition variant: cycle 168 produced RECURRENCE-AT-2 evidence. If cycle 169+ produces a third instance, HARDENING; if 5+ cycles pass without another, the shape may stabilize as a RARE-VARIANT.
