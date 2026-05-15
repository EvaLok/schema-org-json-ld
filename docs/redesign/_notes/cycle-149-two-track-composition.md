---
cycle: 149
date: 2026-05-15
focus: two-track composition — v2-cycle-runner design scope (Track 1 substantive focal) + across-prompts honesty-pass (Track 2 bounded AGREE-ACT-NOW)
forward-priorities-honored: cycle 148 #3 (Track 1) + cycle 148 #5 (Track 2). #1 (audit cycle 219 absorption) quickly resolved as silent-cycle (no substantive audit content to absorb).
status: COMPOSED + COMMITTED (Track 2 commit `56ebad3c`; Track 1 design scope `docs/redesign/_notes/cycle-149-v2-cycle-runner-design-scope.md`); 4th consecutive two-track-composition cycle
---

# Cycle 149 — two-track composition (design scope + honesty-pass)

## Session-start state

Clean deck at session start: 0 open PRs, 0 open `question-for-eva`, 0 open `agent-task` dispatches in flight. Cycle 148 closed with PR #2951 critique absorption (24 findings, 0 disagreements) + PR #2953 v2-prompt-contract-check landing.

**Forward priority #1 (audit cycle 219 absorption) — quick resolution.** Audit HEAD advanced from `72cda153` (cycle 218, May 13) to `333a745d` (cycle 220, 2026-05-15 04:38 UTC, 36 min before cycle 149 session start). Audit cycle 219 (`#466`) was A4 silent-fail #8 — 0 comments, no commits, 54s deterministic; no substantive content. Audit cycle 220 is audit-internal-state work (closure of `#465` accepted + `#462` accepted-partial as the 192nd/193rd accepted recommendations; state.json archival to bring under 250KB; format-revision commitment for audit cycle 221+). One main-relevant emergent observation from cycle 220: audit names `Copilot-as-adversarial-critique-parallel-pattern` as paralleling audit's own V2 cross-repo audit-engagement format at intra-repo scope (PR #2951 dispatch pattern). **No action required cycle 149; noted in journal.**

## Track 2: Across-prompts honesty-pass (AGREE-ACT-NOW L1.1 + L3.2)

Committed first as the bounded act-now work. Single commit `56ebad3c` covering 5 textual edits across 4 role prompt XMLs:

| File | Lines | Critique finding | Fix |
|---|---|---|---|
| `curator-prompt.xml` | 133-138 | L1.1 policy-backdoor #1 | "unless the harness explicitly provides them as part of cycle-close context" → hard prohibition naming narrow harness-bundled exception |
| `curator-prompt.xml` | 535-539 | L1.1 policy-backdoor #2 | "should be avoided" → "Do NOT re-read prior memory-channel" |
| `reconciler-prompt.xml` | 211-216 | L3.2 cargo-cult #1 | "non-array values cause write rejection" removed; router enforces object + required keys; reconciler self-enforces array type |
| `planner-prompt.xml` | 359-362 | L3.2 cargo-cult #2 | "router's reducer rule requires per-role-tasks ... structurally complete" → router enforces presence only; sub-structure is planner-internal discipline |
| `executor-prompt.xml` | 418-423 | L3.2 cargo-cult #3 | "must be an array (may be empty only for true non-artifact actions). Invalid payload causes write reject and cycle error" → router enforces presence; array typeness + emptiness are executor-internal discipline |

**Verification:** `cargo run -p v2-prompt-contract-check -- --strict` reports 4/4 prompts contract-aligned after edits. Only one edit (reconciler) is inside `<output-contract>` (lines 132-218); the `<validation>` sub-element wasn't being parsed as required-keys metadata by the tool. Cargo verification is legitimate per cycle 148 lesson (verifying prompt edits against the freshly-landed tool; not "cargo on non-Rust-PR cycles").

Closes AGREE-ACT-NOW finding #5 from PR #2951 absorption ledger. Other AGREE-ACT-NOW findings (L1.3, L2.1, L2.4, L3.6, X2) remain queued.

## Track 1: v2-cycle-runner design scope (substantive focal)

Document committed at `docs/redesign/_notes/cycle-149-v2-cycle-runner-design-scope.md` (~290 lines, 13 sections). Honors cycle 148 forward priority #3.

**Key design decisions:**

1. **Parallel crate, not in-place rewrite** (Option B). Direct-push-zone authorization + rollback isolation + parallel-run capability + mental-model clarity all favor a new `tools/rust/crates/v2-cycle-runner` over modifying the existing forbidden-zone `cycle-runner`.

2. **9-step super-step sequence per cycle.** `super-step-init → reconciler-pre-poll → reconciler-session → super-step-advance → planner-session → super-step-advance → executor-session → super-step-advance → curator-session → super-step-settle`. Each step shells out to an existing v2-* primitive; runner does NOT re-implement primitive logic.

3. **4-class failure taxonomy** with distinct responses. transient-tool-error → retry once. role-session-empty-output / channel-write-rejected → halt super-step, commit halt marker, exit cleanly. super-step-out-of-order → hard error, abort without state mutation. **No mid-cycle retry for class 2-4** — let next cycle's reconciler observe halt-state via inbound-channel.

4. **State model with per-primitive ownership.** `state/channels/` (router), `state/super-step.json` + `super-step-history.json` (boundary), `state/roles/` (role-driver), `state/reconciler/` (event-processor), `state/v2-cycle-runner/` (runner's own minimal state — cycle number + status + timestamps + halt reason). Single-writer-per-state-file discipline.

5. **Cutover via single workflow YAML PR.** The v2-cycle-runner crate itself is direct-push-zone; the only forbidden-zone change is editing `.github/workflows/claude.yml` to invoke `v2-cycle-runner run` instead of `cycle-runner`. Phase 4 cutover plan deferred to its own design document at the pre-cutover checkpoint.

6. **9 open questions flagged** for implementation cycle 150+ entry. Critical ones: primitive discovery (PATH vs CARGO_TARGET_DIR), role-session model (does v2-role-driver own the Anthropic-API invocation, or does v2-cycle-runner?), halt-recovery semantics (planner re-plan vs skip to curator). These warrant cycle 150 entry-day work, not pre-scope settling.

7. **5-cycle optimistic / 8-12-cycle realistic implementation arc** with named per-cycle entry points (cycle 150 = init subcommand; cycle 151 = run subcommand + first tests; cycle 152 = integration test; cycle 153 = first end-to-end smoke = first measurement). Match against cycle 148 forward priority #2 ("first end-to-end run + first measurement").

**Track 1 produces zero code commits this cycle.** The design scope is the deliverable. Implementation begins cycle 150 entry as a direct-push-zone code commit creating the v2-cycle-runner crate skeleton. Pre-allocating the crate dir now would risk churn if cycle 150 entry surfaces a structural correction.

## Pattern updates

- `two-track-composition` HARDENING-AT-3 → HARDENING-AT-4 cycle 149 (4th consecutive two-track-composition: 146 dispatch+scope, 147 redispatch+dispatch, 148 absorb+land, 149 design-scope+honesty-pass). In-band recurrence post-hardening; pattern stable.
- `proactive-document-before-implementation` **NOVEL@1 cycle 149** — first explicit cycle where the substantive focal is a design scope DOCUMENT, not a code commit. Cycle 146 prompt-contract-check design was paired with same-cycle dispatch; cycle 149 has no Track 1 code commit at all. Watch for recurrence.
- `audit-cycle-silent-absorption-quick-resolution` NOVEL@1 cycle 149 — first cycle where forward priority #1 (audit absorption) resolves to "noted, no action" within ~5 minutes of session-start orientation. Past audit absorptions were substantive multi-cycle (e.g., cycle 134's M1 absorption was ~330 lines). Quick-resolution is a valid absorption category; watch for recurrence to formalize.
- `magnitude-prediction-precision-is-shape-dependent-not-flat` NOT exercised cycle 149 (no LOC prediction made for the design scope artifact; design-scope-document family is not yet established as a prediction-band-bearing shape).

## Forward priorities for cycle 150+

(Re-ordering of cycle 148's 13-item list, accounting for cycle 149 closures and audit observation.)

1. **v2-cycle-runner implementation entry — cycle 150 substantive focal.** Per `cycle-149-v2-cycle-runner-design-scope.md` section 11, the natural entry is `Cargo.toml` + skeleton `main.rs` + first subcommand (`init`). Direct-push-zone authorized. Open questions 1, 2, 3 need settling at entry.
2. **First end-to-end run + first measurement** (cycle 148 #2). Now scheduled for cycle ~153 in the implementation arc.
3. **AGREE-ACT-NOW finding #1 (L2.1 executor 4-fix textual iteration)** — single commit; direct-push-zone. Can run alongside Track 1 implementation as a bounded Track 2.
4. **AGREE-ACT-NOW finding #6 (L1.3 + L3.6 reconciler completeness-metadata pairing)** — prompt edit + v2-channel-router required-key extension. Two-PR pairing. Forbidden-zone-adjacent if router gets a required-key change (router is direct-push-zone, but the prompt+router pairing should land together).
5. **AGREE-ACT-NOW finding #7 (L2.4 dispatch-brief discipline addendum)** — document-only. Add the L2.4 proposed wording to a canonical place (likely `docs/redesign/_notes/dispatch-brief-discipline.md` or as a section in 2-design-framework.md). Direct-push-zone.
6. **AGREE-ACT-NOW finding #8 (X2 honest cycle-1-scope-redefinition)** — document-only. Channel-schema-not-temporal-scope language adopted in any new cycle-1-framing docs.
7. **AGREE-RECORD: side-channel architecture-notes doc** (L1.2 + X4). Document dispatch-firing as an explicitly-authorized side channel rather than treat it as inside channel-reducer discipline. Document-only.
8. **TOOL-SCOPED follow-up: v2-channel-router enforcement extension design scope** (C1 / L3.1 / counterfactual — types + nested shapes). Design scope first, then implementation. The honesty-pass committed cycle 149 references the COMPLETE arc for this enforcement work.
9. **TOOL-SCOPED follow-up: v2-prompt-tag-semantic-fidelity tool design scope** (L2.5). Design scope first.
10. **AGREE-DEFER queue (post-first-measurement)**: curator complexity rework (L2.2 + L3.5), template-mirror architecture-revisit (L2.3 / C2). Hold until runtime evidence from cycle 153+ smoke.
11. **Cycle 120 L2 preserved** (no recursive annotation of 2-selection.md).

## Process honoring

- 34th consecutive cycle of HONORING named forward priority (cycles 115-149); 4th consecutive two-priority-in-one-cycle honoring.
- 62nd bottleneck-asynchronous cycle (78-149).
- 39th non-per-candidate-sharpening cycle (111-149).
- Cycle 120 L2 preserved (2-selection.md untouched).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start, honesty-pass commit message, plus the cycle-close artifacts authored next).
- Cycle 133 lesson clarified once more: cargo invocations cycle 149 are post-edit prompt-contract-check verification, not gratuitous cargo work on non-Rust-PR cycles.
- Cycle 134 lesson preserved (audit-repo HEAD via gh api).
- Cycle 137 lessons preserved — one for-loop attempt blocked at session-mid (cargo subcommand survey loop), recovered via per-crate ls + grep separately. **Re-validation event for cycle 137 lesson (3rd successive validation since cycle 137).**
- Journal-immutability discipline preserved (cycle 149 appends new section to `docs/journal/2026-05-15.md` via Edit anchor at end of cycle 148 section; cycle 148 section NOT back-edited).
- Anti-overstatement audit explicit (this _notes ends with "What cycle 149 does NOT do" enumeration).

## In-session issues and recoveries

- One bash for-loop attempted (`for crate in v2-*; do ... done` to survey CLI surfaces) blocked with "Contains simple_expansion". Recovered cleanly via individual `ls`/`grep` calls per crate. Cycle 137 lesson re-validated.
- One bash expansion attempted (`echo "RUN=${GITHUB_RUN_ID:-N/A}..."`) blocked with "Contains expansion". Recovered via `gh run list --workflow=claude.yml --limit 3 --json databaseId` direct query.
- One CWD-drift issue: after `cd tools/rust && cargo run ...` for v2-prompt-contract-check verification, subsequent `git status --short` and `git diff --stat prompts/v2/` ran from `tools/rust` and produced confusing path errors. Recovered via `git -C <absolute-repo-root>` pattern. Lesson: **avoid `cd` chains; prefer absolute paths or `--manifest-path` / `git -C` flags.** Adding to in-cycle lessons.

## Cycle 149 ARTIFACTS

- `prompts/v2/curator-prompt.xml` modified (2 edits).
- `prompts/v2/executor-prompt.xml` modified (1 edit).
- `prompts/v2/planner-prompt.xml` modified (1 edit).
- `prompts/v2/reconciler-prompt.xml` modified (1 edit).
- `docs/redesign/_notes/cycle-149-v2-cycle-runner-design-scope.md` authored (Track 1; ~290 lines).
- This file (`docs/redesign/_notes/cycle-149-two-track-composition.md`) — cycle 149 overview.
- Journal section in `docs/journal/2026-05-15.md` (appended via Edit anchor at end of cycle 148 section; cycle 148 immutability preserved).
- `.scratch/` ephemeral files: `session-start-2956.md`, `honesty-pass-commit-msg.txt`, `cycle149-session-end.md`, `cycle149-issue-close.md` (the latter two authored at session end).
- 1 commit + 1 push cycle 149: `56ebad3c` (honesty-pass; Track 2). 1 additional commit + push at cycle close (this _notes file + design scope + journal).
- 0 v2-* code crate modifications cycle 149 (Track 1 is design-scope-document only).
- 0 cargo / clippy / test work cycle 149 beyond post-edit verification (`v2-prompt-contract-check --strict` clean 4/4).

## What cycle 149 does NOT do

- Does NOT implement v2-cycle-runner (design scope only; cycle 150+ implementation entry).
- Does NOT modify cycle-runner (forbidden zone).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT modify v2-channel-router (no enforcement extension; cycle 148 #10 candidate).
- Does NOT modify v2-prompt-contract-check (no extension work; cycle 148 #11 candidate).
- Does NOT enact the 6 remaining AGREE-ACT-NOW findings (L1.3, L2.1, L2.4, L3.6, X2, plus the L1.2 + X4 architecture-notes doc).
- Does NOT close audit-cycle-220 priority as a "substrate absorption" — cycle 220 was audit-internal-state work, not substrate critique for main; no main-side action required.
- Does NOT escalate any cycle 149 design decision to Eva (EVA-DEFAULT-AUTONOMY: design-space questions resolved in-cycle; cycle 150 implementation surfaces real Eva-only-decision moments if any).
- Does NOT modify 2-selection.md (cycle 120 L2 preserved).
- Does NOT predict cycle 150+ priority order with high precision beyond the 11-item forward-priorities list.
- Does NOT measure runtime quality of role prompts (no end-to-end run; cycle 153+).
- Does NOT close cycle issue #2956 in this _notes write (cycle issue closure is the LAST step, after session-end comment).
- Does NOT advance HARDENED status of patterns beyond what fits the @3-threshold rules.
