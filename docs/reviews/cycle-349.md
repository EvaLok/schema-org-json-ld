# Cycle 349 Review

## 1. [receipt-integrity] The worklog receipt table does not match the canonical cycle-receipts output

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-24/163934-cycle-349-review-merge-and-pipeline-tools.md:38-54
**Evidence**: The worklog records 11 table rows, including duplicated SHAs `44dc6cc` and `70766e8` under two different tool names and a final row labeled `review-events-refresh` for `ba50e8a`. The canonical `bash tools/cycle-receipts --cycle 349 --repo-root .` output returned 9 receipts, with `44dc6cc`, `70766e8`, and `ba50e8a` represented as `cycle-tagged` rows plus `Also` metadata rather than separate primary receipt rows. Issue `#1700` step C5.1 still reported `Receipt validation: PASS` / `Canonical: 9, Worklog: 9, Missing: 0`, so the cycle both shipped a malformed table and falsely concluded the table matched the canonical receipt set.
**Recommendation**: Stop hand-editing the receipt table structure. Write the table directly from `cycle-receipts` output, including the `Also` column, and make receipt validation fail on duplicated SHAs or relabeled rows rather than only counting presence.

## 2. [process-adherence] The worklog's self-modifications section omits the actual tool changes made this cycle

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-24/163934-cycle-349-review-merge-and-pipeline-tools.md:21-23
**Evidence**: The self-modifications section claims only `docs/state.json` changed. But the actual cycle diff through `cycle-complete` (`git diff --name-only 321b7b6b..8d29edd`) includes `tools/rust/Cargo.lock`, `tools/rust/crates/cycle-runner/src/close_out.rs`, `tools/rust/crates/pipeline-check/Cargo.toml`, and `tools/rust/crates/pipeline-check/src/main.rs`. Those are precisely the infrastructure/tooling changes that the worklog is supposed to surface. This is not timing drift: those files were already part of the cycle's committed history before the worklog was written.
**Recommendation**: Derive the self-modifications section from a real git diff over the cycle window instead of summarizing from memory. If infrastructure files changed, list them explicitly; if not, say `None this cycle.`

## 3. [journal-quality] The journal marks the prior commitment as fully followed even though a promised closure did not happen

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-24.md:305-311
**Evidence**: The quoted prior commitment had three parts: merge the two pipeline PRs, close audit-inbound `#1690` after both dispatches merged, and defer the deferral-threshold work until after `#1696`. The journal collapses that into `**Followed.**` and only discusses the two PR merges. But the same cycle's worklog still lists `Close audit-inbound #1690 once commitment 1 dispatched` as a next step, and GitHub issue `#1690` remained open at review time. That means at least one concrete promised action was not completed, yet the follow-through section narrates the whole commitment as satisfied.
**Recommendation**: Evaluate commitment follow-through item by item. When a compound commitment has mixed outcomes, record which parts were completed, which were deferred, and which remain open instead of using a blanket `Followed.` label.

## Complacency score

**3/5.** The cycle's state math is sound: `state-invariants` passes, `metric-snapshot` passes, and `copilot_metrics` reconciles exactly with `agent_sessions`. But the cycle still repeated two chronic failure modes that should already be structurally controlled: hand-shaped receipt/reporting drift and journal follow-through overstatement. Because issue `#1700` also recorded an early `Pipeline check failed` result during the cycle and work continued anyway, the score is capped at **3/5**; the clean final state keeps it from dropping lower, but the recurrence of factual narrative errors means this was not a disciplined close-out.
