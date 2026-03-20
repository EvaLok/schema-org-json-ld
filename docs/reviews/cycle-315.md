# Cycle 315 Review

## 1. [worklog-accuracy] The published `Current state` block is stale again once the cycle-315 review dispatch is recorded

**File**: `docs/worklog/2026-03-20/002908-cycle-315-post-stabilization-merges-and-dispatch.md:29-34`; `docs/state.json:4238-4244,4446-4458`
**Evidence**: The worklog still claims `In-flight agent sessions: 1` and `Copilot metrics: 466 dispatches`, but the canonical state now includes the cycle-315 review dispatch as in-flight issue `#1515` and updates `copilot_metrics` to `dispatch_log_latest: "#1515 ... (cycle 315)"`, `in_flight: 2`, and `total_dispatches: 467`. This is the same post-dispatch drift that cycle 314's review flagged one cycle earlier.
**Recommendation**: Either patch the worklog after review dispatch or relabel `Current state` as a pre-review snapshot so the published artifact stops presenting a transient state as the final cycle state.

## 2. [receipt-integrity] The receipt section mixes cycle-315 scope with cycle-314 session activity, and one listed merge receipt is wrong

**File**: `docs/worklog/2026-03-20/002908-cycle-315-post-stabilization-merges-and-dispatch.md:42-53`; `docs/state.json:4213-4237`
**Evidence**: The scope note says `Docs and record-dispatch commits are structurally excluded`, yet the table still includes `record-dispatch | cecd667`. After fetching full history, `bash tools/cycle-receipts --cycle 315 --repo-root .` returns only four reachable receipts (`d5be944`, `0b35c47`, `3ad128d`, `5f624b4`), confirming that the `process-review`/`process-merge`/`record-dispatch` entries are not cycle-315 receipts. Additionally, `git log --all --oneline --grep='PR #1509 merged'` finds `dcc2eef state(process-merge): PR #1509 merged [cycle 314]`, not the worklog's `74ae091`.
**Recommendation**: Separate "session activity performed during this run" from cycle-scoped receipts, and generate the receipt table directly from `cycle-receipts` output so stale or mistyped hashes cannot leak into the artifact.

## 3. [state-integrity] The manual `review_events_verified_through_cycle` advance to 314 is internally consistent in the committed state

**File**: `docs/state.json:7490-7507`
**Evidence**: `review_agent.history` includes cycle 314 with `finding_count: 5`, `deferred: 2`, `ignored: 2`, and a note that matches the cycle-314 review disposition, `last_review_cycle` is `314`, and `review_events_verified_through_cycle` is also `314`. A fresh `bash tools/state-invariants` run passes all 15 checks, including `review events verified`.
**Recommendation**: Keep using `state-invariants` as the acceptance gate whenever review-event markers are advanced by hand; it caught no latent inconsistency here.

## 4. [process-adherence] The C4.1 bypass was disclosed and used as an explicit stopgap rather than hidden as a green pipeline

**File**: `docs/worklog/2026-03-20/002908-cycle-315-post-stabilization-merges-and-dispatch.md:31-33`; `docs/journal/2026-03-20.md:25-27`
**Evidence**: The worklog explicitly says the pipeline passed only because the doc-validation circular dependency was bypassed with `--pipeline-status`, and the journal explains the underlying recursive call pattern and the intended structural fix. Re-running `bash tools/validate-docs worklog --file docs/worklog/2026-03-20/002908-cycle-315-post-stabilization-merges-and-dispatch.md --cycle 315 --repo-root .` succeeds once the repository has full history, which is consistent with the documented stopgap.
**Recommendation**: Keep the bypass explicit in artifacts until issue `#1513` lands, but avoid narrating it as a normal PASS without the accompanying caveat.

## 5. [journal-quality] The journal is candid about the session-vs-cycle attribution problem instead of pretending the state model and the human narrative align

**File**: `docs/journal/2026-03-20.md:19-27`; `docs/state.json:4728-4733`
**Evidence**: The journal states outright that the session merged two PRs and dispatched one follow-up even though `last_cycle.summary` still says `0 dispatches, 0 merges`, and it explains why: those events were recorded under cycle 314 before cycle 315 formally started. That is materially honest about the model mismatch and gives future reviewers the right frame for comparing worklog prose to canonical state.
**Recommendation**: Preserve this level of explicitness whenever the session timeline and the formal cycle boundary diverge; it makes the artifact auditable even when the model is imperfect.

## 6. [dispatch-quality] The follow-up issue for the doc-validation recursion is concrete enough to implement without guesswork

**File**: `docs/worklog/2026-03-20/002908-cycle-315-post-stabilization-merges-and-dispatch.md:8-10`; `docs/journal/2026-03-20.md:25-27`
**Evidence**: The local artifacts consistently describe the defect as a specific `validate-docs` ↔ `pipeline-check` recursion at close-out C4.1, and the dispatched issue body for `#1513` translates that into exact files, CLI flags, and tests (`pipeline-check`, `validate-docs`, unit tests for excluded steps, and targeted cargo test/build commands). That is the kind of narrow, executable spec that minimizes ambiguity for the implementation agent.
**Recommendation**: Keep dispatches at this granularity: name the concrete files, the exact API/flag to add, and the verification commands required for acceptance.

## 7. [review-processing] Cycle 314's review was recorded with the same 1 actioned / 2 deferred / 2 no-action split described by the source review

**File**: `docs/reviews/cycle-314.md:9-31`; `docs/state.json:7490-7504`
**Evidence**: The source review only recommends action for finding 2, treats findings 3 and 4 as unresolved defects, and leaves findings 1 and 5 as positive observations. The state entry for cycle 314 records `finding_count: 5`, `deferred: 2`, `ignored: 2`, and the note `Finding 2 actioned by PR #1509 (C6.5). Findings 3,4 deferred. Findings 1,5 positive/no action.` The repository uses `ignored` as the internal bucket for no-action findings, so the accounting matches the review.
**Recommendation**: Keep this explicit note style in `review_agent.history`; it makes the deferred-versus-no-action mapping auditable without reopening the source review.

## Complacency score

**4/5** — This cycle's state bookkeeping, review processing, and journal candor are materially better than the stabilization-period norm. But the published worklog still overclaims final-state accuracy after review dispatch, and its receipt table is not trustworthy as written: it mixes scopes, contradicts its own exclusion note, and contains at least one wrong hash.
