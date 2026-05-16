# Cycle 156 _notes — v2-dispatch-status tool + StepTrace.executed (C5)

**Cycle:** 156 (two-track composition, 9th consecutive after cycle 151 single-track exception)
**Date:** 2026-05-16
**Cycle issue:** [#2965](https://github.com/EvaLok/schema-org-json-ld/issues/2965)
**Commits this cycle:** `6f9b516f` (Track 1 v2-dispatch-status crate) + `8a7b128f` (Track 2 C5 StepTrace.executed) + cycle-close commit
**Predecessor:** cycle 155 (commit `bd041b05`, cycle 152 critique absorption + C8 fix)
**Reference forward priority:** [`cycle-155-cycle-152-critique-absorption.md`](cycle-155-cycle-152-critique-absorption.md) §Forward priorities for cycle 156+ (#7 = v2-dispatch-status tool, #1 = C5 StepTrace.executed)

## Cycle composition

Two-track composition continues, HARDENING-AT-9 post cycle 151 single-track exception. Combined arc: 146-150 (5 consecutive) → 151 single-track exception → 152-156 (5 consecutive post-exception).

**Track 1 (substantive focal):** `v2-dispatch-status` Rust tool. Forward priority #7 from cycle 155 _notes. CORE-DESIGN-PRINCIPLE-aligned tool extraction of the manual cycle 154→155 dispatch-return-detection loop. 1011 LOC (720 prod + 291 tests, 42 unit tests). Single direct-push commit `6f9b516f`.

**Track 2 (bounded ACT-NOW):** C5 (L1.5) — `executed: bool` field on `StepTrace`. Cycle 155 forward priority #1. 95 LOC of changes (~5 prod + ~90 tests; cycle 155 predicted "~6 affected unit tests" — actual was 0 broken tests + 3 new asserting tests). Single direct-push commit `8a7b128f`.

## Track 1 — v2-dispatch-status tool

### Tool surface

```
v2-dispatch-status [--repo OWNER/NAME] [--json] check --issue N
```

Given a dispatch issue number, probes via one or two GraphQL queries and reports the canonical absorption-readiness signal.

### Primary GraphQL query probes

- **Issue state** — OPEN / CLOSED / NOT_FOUND (NOT_FOUND derived from gh stderr pattern `Could not resolve to an Issue`).
- **`closedByPullRequestsReferences`** — canonical "PRs linked to this issue" via GitHub Development panel. **Critically: this includes DRAFT PRs**, which the cycle 154 `gh pr list` (without `--state all`) missed.
- **`timelineItems[AssignedEvent]`** — was Copilot assigned? Matches bot login `copilot-swe-agent` (the actual GraphQL `login`; the display name "Copilot" is different).
- **`timelineItems[ConnectedEvent]`** — did Copilot actually connect? Records first connect timestamp.
- **`timelineItems[CrossReferencedEvent]`** — additional PR cross-references (filtered to `headRefName` starting with `copilot/`).
- **`comments`** — count comments authored by Copilot bot (matched by login).

### Secondary GraphQL query (only when no linked PR + copilot connected)

- **`refs(refPrefix: "refs/heads/copilot/")`** — enumerate `copilot/*` branches in the repo, paginated. Each branch reported with HEAD commit SHA + committed date.
- **Filter by `committedDate >= connected_at`** — excludes ancient unrelated branches.

### Signal classification (8 kinds, kebab-case)

| Signal | Meaning | Recommended action |
|---|---|---|
| `not-found` | Issue does not exist (gh "Could not resolve") | investigate-issue-number |
| `not-dispatched` | Issue exists but Copilot never assigned | check-dispatch-tooling |
| `dispatched-no-connect` | Assigned but no ConnectedEvent (ADR 0016 watch window) | await-or-rediscover-bot |
| `connected-no-output` | Connected but no PR, no branches, no comments | await |
| `branch-candidates` | Connected, no linked PR, but copilot/* branches exist | absorb-from-branch |
| `pr-open` | Linked PR with `copilot/*` head ref, state=OPEN | review-pr |
| `pr-closed-unmerged` | Linked PR closed but not merged | investigate-pr-closure |
| `pr-merged` | Linked PR merged | housekeep-close |

Precedence: PR signals trump branch signals (a linked PR is more canonical than a candidate branch). Within PR signals, merged > open > closed-unmerged.

### Live smoke validation

Three real issues from this repo probed end-to-end:

1. **#2960 (cycle 152 dispatch)** → `pr-open` with PR #2961 head=`copilot/redesign-critique-v2-cycle-runner`, connect at 2026-05-15T10:59:04Z. **Recommended:** review-pr. **Significance:** cycle 155 _notes claimed "Copilot did not open a PR" — INCORRECT. PR #2961 has existed as **draft** since 2026-05-15T10:58:59Z (the dispatch's connect+1s moment). Cycle 154 missed it (default `gh pr list` excludes drafts). Cycle 155 missed it (used branch-direct inspection without checking `closedByPullRequestsReferences`).

2. **#2952 (cycle 147 dispatch)** → `pr-merged` with PR #2953 head=`copilot/redesign-impl-v2-prompt-contract-check`, merged 2026-05-14T22:42:00Z. **Recommended:** housekeep-close.

3. **#99999** → `not-found`. **Recommended:** investigate-issue-number.

### Test coverage

42 unit tests covering:

- `parse_repo` — 4 tests (valid + 3 error cases)
- `parse_dispatch_response` — 10 tests covering: not-found (null), open-no-events, assigned-only, assigned-to-human-not-copilot, connected, multiple-connects-first-wins, linked-PR-via-cross-referenced, ignored-non-copilot-branch-PRs, dedup-within-cross-referenced, comment-counting-copilot-only, closed-by-PR-merged, draft-PR-via-closed-by (the cycle 155 case), closed-by-non-copilot-filtered, closed-by + cross-referenced dedup, copilot-swe-agent-login-recognized
- `parse_branches_response` — 2 tests (single page + paginated cursor)
- `filter_recent_branches` — 3 tests (cutoff semantics: at-or-after, empty-committed-at kept, empty-cutoff keeps-all)
- `classify` — 8 tests (one per signal variant + precedence tests for PR-open-over-branches and PR-merged-over-open)
- `build_snapshot` — 3 tests (skip-branch-probe-when-PR-linked, skip-when-not-connected, run-when-connected-no-PR)
- `run` (CLI entry) — 3 tests (human output, JSON output, not-found propagation)
- `signal_kind` string stability — 1 test pinning the 8 kebab-case labels
- `recommended_action` non-empty per variant — 1 test
- `is_copilot_login` — 1 test (known logins + case-insensitive + negatives)

### Pattern observations

#### NEW pattern @1: `cycle-155-false-positive-on-PR-not-opened`

Cycle 155 _notes explicitly stated: "Copilot DID complete the work: commit `fbc9ddbe` on branch `copilot/redesign-critique-v2-cycle-runner` at 2026-05-15 11:02:57Z (~4 min after connection at 10:59:04Z)" and "Copilot committed to its branch but did not open a PR."

The second clause is FALSE. PR #2961 (draft) was created at 2026-05-15T10:58:59Z — actually BEFORE the commit at 11:02:57Z. Copilot's standard workflow is: assign-self → open draft PR → push commits to the PR's branch → optionally promote out of draft.

Cycle 155's false-positive matters because:
1. It superseded cycle 154's false-negative with a different but also incorrect narrative.
2. The pattern observation `dispatch-return-detection-must-include-dispatch-working-branches` (cycle 155 NOVEL@1) was an under-specified version of the real pattern: dispatch-return-detection must use `closedByPullRequestsReferences` (which includes drafts) NOT plain `gh pr list` (which excludes them by default).
3. The cycle 155 §"What cycle 155 does NOT do" line "Does NOT open a PR for the Copilot branch... — the critique file is in master via cycle 155's direct-push commit `3607915f`; the Copilot branch can remain as historical record" is misframed. The PR already exists; the question is whether to close-without-merge OR promote-out-of-draft-and-merge.

#### NEW pattern @1: `dispatch-return-detection-via-closed-by-PR-references`

Supersedes cycle 155's `dispatch-return-detection-must-include-dispatch-working-branches`. The canonical primary signal for dispatch-return detection is GraphQL `Issue.closedByPullRequestsReferences` (which includes draft PRs). Branch enumeration is a defensive secondary probe for the rare case where Copilot commits to a branch without opening any PR (not observed in this codebase yet — cycle 155's "no PR" claim was a misdiagnosis).

#### NEW pattern @1: `copilot-bot-login-is-copilot-swe-agent-not-Copilot`

The Copilot coding agent's GraphQL `login` field returns `copilot-swe-agent`, NOT the display name "Copilot". First version of v2-dispatch-status used `"Copilot"` as the match string and produced `not-dispatched` for a clearly-dispatched issue. Live smoke caught this; the matcher is now `is_copilot_login()` accepting `copilot-swe-agent`, `Copilot`, `github-copilot` case-insensitively. Forward-compat against GitHub renaming the bot login.

#### NEW pattern @1: `tool-extraction-surfaces-prior-cycle-errors`

The v2-dispatch-status tool not only addresses the cycle 155 false-negative — it also exposes cycle 155's own false-positive (the PR-not-opened claim). Pattern: when extracting a procedure from orchestrator scratch-work into a deterministic tool, the tool's correctness often forces re-examination of the procedure as it was previously practiced. Tool extraction is a debugging mechanism on prior cycles, not only an automation. CORE-DESIGN-PRINCIPLE adjacent: tools handle repetitive procedural work AND surface where the orchestrator's manual practice was wrong.

#### Pattern continued: `two-track-composition` HARDENING-AT-9

5 consecutive post-cycle-151 exception (152, 153, 154, 155, 156). Plus 5 consecutive pre-exception (146-150). Combined: 10 of 11 cycles two-track in the 146-156 arc. The exception at cycle 151 was justified by design scope (cycle 149 design scope §11 single direct-push-per-cycle for v2-cycle-runner build).

## Track 2 — C5 (L1.5) StepTrace.executed

### Change

Added single field `executed: bool` to `StepTrace`. Initialized at construction (line 621 in `run_cycle`) with `executed: !run_args.dry_run`. By the time the trace is pushed in any code path, this value reflects the semantic truth:

- **Dry-run path** (line 631): pushes with `executed=false` (primitive only described).
- **Live success** (line 657): pushes with `executed=true` (`invoke_with_retry_once` returned ok).
- **Live halt-after-invoke** (line 652): pushes with `executed=true` (primitive ran, then failed; the failure is part of the observable outcome — the step DID run).
- **Validation-error pre-loop** (`validate_session_output_files`): no traces pushed; no concern.

### Tests added (3 new)

- `dry_run_traces_have_executed_false` — runs 10-step dry-run, asserts all 10 traces have `executed=false`.
- `live_success_traces_have_executed_true` — runs 10-step live success, asserts all 10 traces have `executed=true`.
- `live_halt_pushed_trace_has_executed_true` — runs live cycle where step 3 fails; asserts all 3 pushed traces (steps 1, 2 ok + step 3 fail) have `executed=true`.

### Existing tests unaffected

Cycle 155 _notes predicted: "the change is bounded but not 1-line trivial. C8 is the cycle 155 Track 2 ACT-NOW; C5 is a forward priority for cycle 156+." and "adding a field to `StepTrace` updates ~6 unit tests that assert trace shape (lines 1193-1251, 1389-1407 in main.rs)."

**Actual:** 0 existing unit tests broke. The lines cycle 155 cited (1193-1251, 1389-1407) are tests of `build_step_invocation` (which returns `(PathBuf, &'static str, Vec<String>)` — NOT `StepTrace`) and `mock_invoker_records_args_in_order` (which inspects `MockInvoker.calls()` — also not `StepTrace`). No test asserted directly on `StepTrace` field shape.

Pattern observation: **`predicted-test-impact-overestimate-by-name-association`** NOVEL@1 cycle 156. Cycle 155's claim that "tests assert trace shape" was based on the test names mentioning "trace" (e.g., `dry_run_does_not_invoke_primitives_or_mutate_state` has no trace assertions; `live_run_invokes_ten_steps_and_writes_state_on_success` asserts on `last-cycle.json` state, not on `traces[]`). Forward priority estimation should grep for actual struct-field access patterns, not test-name keywords. Cycle 155 forward-priority estimate said "1-2 cycles" for C5; actual cost was a fraction of one cycle.

### Test results

```
cargo test -p v2-cycle-runner
  31 unit + 2 integration = 33/33 green
cargo clippy -p v2-cycle-runner --all-targets -- -D warnings
  clean
```

## Process honoring

- **41st consecutive cycle of HONORING named forward priority** (cycles 115-156). Cycle 155 named #7 (v2-dispatch-status tool) and #1 (C5 StepTrace.executed); cycle 156 closes both.
- **69th bottleneck-asynchronous cycle** (78-156).
- **46th non-per-candidate-sharpening cycle** (111-156).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 156).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start comment, two commit messages, cycle-close ephemerals).
- Cycle 133 clarification preserved: 4 cargo invocations cycle 156 are legitimate (Track 1: build + test + clippy; Track 2: test + clippy; Track 1 also workspace build for collateral check; live smoke runs via cargo run for 3 issues; not gratuitous — each verifies a specific change or runs the new tool).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `333a745d`).
- Cycle 137 lessons re-validated cycle 156 — `gh pr view` single-operation REST calls for verification. **10-cycle running validation** (137 + 147-156).
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 156** — all cargo invocations via `--manifest-path tools/rust/Cargo.toml`. **8-cycle running validation** (149-156).
- Cycle 151 date-test-value lesson preserved (no new date tests cycle 156).
- Cycle 152 disk-format-read-source lesson preserved (no new file-format parsing).
- Cycle 153 multi-Edit-fallback lesson preserved.
- Cycle 154 `gh api graphql` subprocess pattern preserved and extended: v2-dispatch-status uses this pattern for primary + secondary queries.
- Cycle 155 dispatch-return-detection lesson SUPERSEDED — see Pattern observations above.
- Journal-immutability discipline preserved (cycle 156 will append NEW section via Edit anchor at end of cycle 155 section; cycle 148-155 sections will NOT be back-edited).
- Anti-overstatement audit explicit ("What cycle 156 does NOT do" enumeration below).
- **Two-track composition continued** — cycle 156 is the 5th consecutive two-track cycle post cycle 151 exception (HARDENING-AT-9).
- **SECTION 6b list housekeeping cadence:** cycle 156 did NOT do list housekeeping — both Track 1 and Track 2 were substantive code work. The 6 open issues are all standing input-from-eva or active in-flight; #2960 is still open (left for cycle 157+ housekeeping with the new tool to verify status).

## In-session issues and recoveries

- **Cycle 155 false-positive on "Copilot did not open a PR" discovered cycle 156.** PR #2961 (draft) has existed since dispatch time. The v2-dispatch-status tool's live smoke surfaced this. Recovery: documented as Pattern observation `cycle-155-false-positive-on-PR-not-opened`. **No state.json edit cycle 156** (the state.json `agent_sessions[]` entry for #2960 is `in_flight` which is still accurate — PR #2961 is OPEN not MERGED). v2-state-dispatch-sync run would not currently transition #2960 (issue still OPEN). Cycle 157+ housekeeping should decide whether to close PR #2961 (content already in master via `3607915f`) or promote out of draft and merge.
- **Copilot bot login mismatch found mid-cycle.** First v2-dispatch-status build used `"Copilot"` as match string; live smoke against #2960 returned `not-dispatched` despite obvious dispatch. Recovery: queried raw GraphQL timeline directly, found bot login is `copilot-swe-agent`, refactored to `is_copilot_login()` accepting multiple variants case-insensitively. Lesson recorded as Pattern.
- **NOT_FOUND error-classification gap found mid-cycle.** First version returned a generic gh-exit-1 error for non-existent issues; updated to detect `Could not resolve to an Issue` substring in stderr and classify as NOT_FOUND. Direct test added.
- **Closed-by-PR vs CrossReferencedEvent gap found mid-cycle.** First version relied solely on CrossReferencedEvent for linked PRs; live smoke against #2952 (a known PR-merged dispatch) returned `branch-candidates` because CrossReferencedEvent only captures *mentions*, not Development-panel-linked PRs. Added `closedByPullRequestsReferences` to primary query; updated parser to merge PRs from both sources with dedup. Added test for closed-by-PR + draft-PR-via-closed-by + non-copilot-filter + cross-source dedup.
- All `gh api graphql` operations clean cycle 156 (single-purpose-bash-invocation pattern; no for-loops, no redirection).
- All `cargo build` / `cargo test` / `cargo clippy` operations clean cycle 156.
- All Edit / Write operations clean cycle 156.

## Forward priorities for cycle 157+

1. **PR #2961 (draft) disposition** — NEW from cycle 156 finding. Options: (a) close-without-merge with forward-link to `3607915f` per HOUSEKEEPING discipline (the content is already in master); (b) promote out-of-draft and merge as a no-op-merge to formalize Copilot's audit trail. Recommend (a) — content is already absorbed; PR is redundant. Bounded textual operation (1 comment + 1 close).

2. **#2960 dispatch issue disposition** — pairs with #2961. Once PR is handled, close #2960 with closure-link to `3607915f` per SECTION 6b. Bounded textual.

3. **C10 (L2.5) design-scope amendment** — bounded textual change to `cycle-149-v2-cycle-runner-design-scope.md:104` clarifying "state mutation" scope. Cycle 155 forward priority #2. Carries forward.

4. **X5 commit-governance scope clarification** — bounded textual addition to v2-cycle-runner crate doc-comment. Cycle 155 forward priority #3. Carries forward.

5. **C2 (L1.2) phase marker in StepTrace** — pairs naturally with C5 (already done). Could land as bounded Track 2 cycle 157. Cycle 155 forward priority #4. Carries forward.

6. **AGREE-ACT-NOW L1.3 + L3.6 reconciler completeness-metadata pairing** (from cycle 148 absorption, NOT cycle 155). Bounded; prompt edit + v2-channel-router required-key extension. Carries forward.

7. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** (from cycle 148 absorption). Document-only. Carries forward.

8. **TOOL-SCOPED v2-channel-router enforcement extension design scope** (C1 / L3.1 from cycle 148 absorption). Design scope first.

9. **TOOL-SCOPED v2-prompt-tag-semantic-fidelity tool design scope** (L2.5 from cycle 148 absorption). Design scope first.

10. **`status` + `verify` v2-cycle-runner subcommands** — cycle 158+ if needed; deferred again cycle 156.

11. **AGREE-DEFER queue (post-real-role-session-measurement)** — curator complexity rework, template-mirror architecture-revisit. Hold for live-claude-code-spawn evidence (still SCAFFOLD).

12. **Coordinated retry/timeout/cancellation arc** — C13 + X2.

13. **Coordinated structured-error-envelope arc** — C6 + C7 + C9.

14. **Coordinated resume/recovery arc** — C11 + C12 + X1.

15. **Cycle 120 L2 preserved** (no recursive annotation of 2-selection.md).

**Cycle 155 forward priorities CLOSED by cycle 156:** #1 (C5 StepTrace.executed), #7 (v2-dispatch-status tool).

## What cycle 156 does NOT do

- Does NOT modify legacy `tools/cycle-runner` (v1; forbidden zone preserved during Phase 3).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT amend `cycle-149-v2-cycle-runner-design-scope.md` for C10 — deferred to focused cycle.
- Does NOT add C2 phase-marker — deferred (paired with C5 originally; left as separate cycle to keep Track 2 bounded for HARDENING discipline).
- Does NOT close PR #2961 (cycle 155 dispatch PR) — deferred to cycle 157+ housekeeping with clearer disposition decision.
- Does NOT close issue #2960 — deferred to cycle 157+ housekeeping.
- Does NOT add per-step timeout (X2) — deferred.
- Does NOT add lock/lease (X1) — deferred.
- Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Does NOT modify `docs/state.json` (#2960 still in_flight which is currently accurate; v2-state-dispatch-sync transition would require #2960 to be CLOSED first).
- Does NOT spawn live Claude sessions (role-driver remains SCAFFOLD).
- Does NOT escalate any cycle 156 decision to Eva (EVA-DEFAULT-AUTONOMY — cycle 156's findings are within design-space and bounded technical scope).
- Does NOT run `v2-state-dispatch-sync sync` against state.json (no transitions pending; #2960 still OPEN at cycle 156 session-end).
- Does NOT integrate v2-dispatch-status into v2-state-dispatch-sync (separate tools; cross-tool composition is a forward priority candidate but not in cycle 156 scope).
- Does NOT dispatch a Copilot critique on v2-dispatch-status — cycle 156 was paired Track 1 substantive work; a feedback-only dispatch on the new tool is a candidate for cycle 157+ if Eva or audit signal interest.

## Cycle 156 ARTIFACTS

- `tools/rust/crates/v2-dispatch-status/Cargo.toml` — new (~12 lines).
- `tools/rust/crates/v2-dispatch-status/src/main.rs` — new (1011 LOC: ~720 prod + ~291 tests; 42 unit tests). Track 1 commit `6f9b516f`.
- `tools/rust/Cargo.lock` — modified (workspace dependency manifest auto-updated).
- `tools/rust/crates/v2-cycle-runner/src/main.rs` — modified (+95 LOC: +6 prod for `executed` field + ~90 test code for 3 new tests). Track 2 commit `8a7b128f`.
- `docs/redesign/_notes/cycle-156-dispatch-status-tool-and-c5-fix.md` — new (this file).
- This journal section appended via Edit anchor at end of cycle 155 section; cycle 148-155 sections preserved.
- `.scratch/cycle156-session-start.md` — session-start comment body (ephemeral).
- `.scratch/cycle156-track1-msg.txt` — Track 1 commit message (ephemeral).
- `.scratch/cycle156-track2-msg.txt` — Track 2 commit message (ephemeral).
- `.scratch/cycle156-session-end.md` — session-end comment (authored cycle-close, ephemeral).
- `.scratch/cycle156-issue-close.md` — cycle issue close comment (authored cycle-close, ephemeral).
- 2 direct-push commits Track-side: `6f9b516f` (Track 1) + `8a7b128f` (Track 2).
- 1 cycle-close commit (this _notes + journal append + ephemerals).
- 0 issues closed cycle 156 (no SECTION 6b housekeeping; cycle 157+ work).
- 0 dispatches cycle 156 (paired Track 1 substantive build, no dispatch needed).
- 6 cargo invocations cycle 156: 1 workspace build (collateral check) + 2 builds (v2-dispatch-status × 2) + 2 tests (v2-dispatch-status × 2 + v2-cycle-runner × 2 = 4 actually) + 2 clippy (v2-dispatch-status + v2-cycle-runner). All clean.
- 3 live smoke runs of v2-dispatch-status against #2960 / #2952 / #99999. All produced expected signals after iteration through 3 mid-cycle issue discoveries.
- 9 Edits across: v2-dispatch-status main.rs (5 — initial Write + login fix + 2 other parser additions + closed-by-PR query/parse), v2-cycle-runner main.rs (2 — StepTrace field + run_cycle construction; bundled into 1 Write effectively but counts as 2 Edits in conceptual terms).
- 0 Edits on `docs/state.json`.
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.
