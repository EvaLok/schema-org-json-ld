# Cycle 361 Review

## 1. [worklog-accuracy] The worklog ships the same receipt-event miscount that cycle 360 had already identified

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-25/183056-cycle-361-review-merge-receipt-counting-fix-dispatch.md:38-46
**Evidence**: The scope note says `receipt events: 1 dispatch, 3 merges, 2 reviews`, but a fresh `bash tools/cycle-receipts --cycle 361 --repo-root .` returns six receipts in scope: `cycle-start`, two `process-merge`, one `process-review`, `cycle-complete`, and `cycle-tagged`. There is no dispatch receipt in the through-`cycle-complete` window, and the rendered table directly below the note does not contain three merge rows or two review rows either. The journal then explains the root cause after the fact (`docs/journal/2026-03-25.md:378-380`), which means the cycle understood the defect but still published the bad count unchanged.
**Recommendation**: Stop freehanding the prose receipt summary. Generate the note from the same filtered receipt list used to render the table, and fail worklog generation when the prose counts diverge from the rows.

## 2. [worklog-accuracy] The receipt table itself is incomplete even within its own stated scope

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-25/183056-cycle-361-review-merge-receipt-counting-fix-dispatch.md:40-46
**Evidence**: The table lists only five rows and stops at `cycle-complete`. After fetching full history as required, `bash tools/cycle-receipts --cycle 361 --repo-root .` reports a sixth in-scope receipt: `cycle-tagged | e42678f | state(verify-review-events): verified review events through cycle 361 [cycle 361]`. The worklog’s own exclusion note only carves out post-C5.1 docs, `record-dispatch`, and review-body commits; `e42678f` is none of those. This means the artifact is not just narratively wrong — the canonical receipt table it was supposed to mirror was truncated.
**Recommendation**: Render the receipt table mechanically from `cycle-receipts` output rather than manually copying selected rows, and add a row-count check so in-scope receipts cannot silently disappear.

## 3. [complacency-detection] The deferred `Issues processed` defect is acknowledged and then dropped from any observable follow-through

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-25.md:376-393
**Evidence**: Cycle 360 review finding 2 said the worklog’s `Issues processed: None` section contradicted a real closed issue (`docs/reviews/cycle-360.md:9-13`). Cycle 361 records that as `F2 (worklog-accuracy): "Issues processed: None" contradicts closed #1754 — deferred, needs auto-issues expansion`, but the concrete commitments immediately below only cover merging `#1765` and continuing to use `--addresses-finding`. The cycle-complete snapshot in `docs/state.json` also shows the latest dispatch was `#1765 Fix receipt event counting to use tool names only`, with no companion issue or dispatch for the deferred auto-issues expansion work. That is chronic-category acknowledgment without a tracked completion condition.
**Recommendation**: When a review finding is deferred, require the journal/worklog to either (a) create a linked follow-up issue/dispatch in the next-step list or (b) record an explicit owner, trigger, and recheck cycle. “Deferred” alone is not an actionable commitment.

## Complacency score

**2/5** — Cycle 361 did some real verification: `state-invariants` passed, `metric-snapshot` passed, `copilot_metrics` math matches `agent_sessions`, and the issue thread contains 25 distinct step comments through `C8`. But the cycle still published a receipt summary that its own journal admits is false, omitted an in-scope receipt from the table that a canonical tool already exposes, and let a second worklog-accuracy defect fall out of the concrete follow-through entirely. That is not a total process collapse, but it is chronic artifact sloppiness plus acknowledgement-without-resolution, so the score should drop below the previous cycle rather than hold steady.
