# Cycle 457 Review

## 1. [journal-quality] The journal retrofits extra commitment criteria and still marks the carry-forward as “Followed”

**File**: `docs/journal/2026-04-07.md:206-209,243-255`
**Evidence**:
- The quoted prior commitment at lines 206-207 promises only two observable checks under commitment 1: `(a)` the cycle 457 close-out should publish `FAIL→PASS`, and `(b)` `record-dispatch` should update `last_cycle.summary`; commitment 2 is the separate field-inventory verifier.
- The same entry immediately says `**Followed.**` and claims “Cycle 456 commitment 1 had 4 sub-criteria,” then later grades invented `(c)` and `(d)` items at lines 245-251 that do not appear in the quoted commitment.
- Even under the rewritten framing, two of the four sub-criteria are still marked `Pending C5/C5.5`, so the blanket `Followed` label over-credits the cycle while blurring what was actually promised versus what was discovered mid-cycle.
**Recommendation**: Grade only the criteria that were actually committed in the previous journal entry. If new checks emerge during the cycle, record them as new observations or next-cycle commitments instead of retrofitting them into the prior promise and then marking the whole item “Followed.”

## 2. [process-adherence] The cycle still bypassed `dispatch-task` and had to clean up a duplicate issue/PR pair

**File**: `docs/journal/2026-04-07.md:215-227`
**Evidence**:
- The journal says the structural-fix dispatch “took two attempts”: a raw `gh api` call created issue `#2282`, then `dispatch-task` created `#2284`; line 223 explicitly says the first call bypassed `state.json::agent_sessions`.
- GitHub issue `#2282` exists as a closed `agent-task`, and draft PR `#2283` exists as a closed, unmerged Copilot PR spawned from that abandoned dispatch attempt.
- `bash tools/pipeline-check --cycle 457 --json` reports `current-cycle-steps: pass` with 25 mandatory pre-gate steps present on issue `#2278`, so the problem was not missing narration; the orchestrator knowingly used a shortcut even though the state-recording tool already existed, then spent cycle time undoing the duplicate artifacts.
**Recommendation**: Treat tool-backed dispatch as mandatory, not advisory. Add a hard guard in the orchestration path so agent-task creation cannot happen outside `dispatch-task`/`record-dispatch` without simultaneously registering state, and treat duplicate issue/PR cleanup as a process failure rather than an acceptable retry pattern.

## 3. [test-gap] The `refresh-field-inventory` hotfix shipped without a field-specific regression for the new refresh-only entry

**File**: `tools/rust/crates/refresh-field-inventory/src/main.rs:122-126,677-717`
**Evidence**:
- Commit `69c94378` added `step_comment_acknowledged_gaps` to `REFRESH_ONLY_FIELDS`; that constant edit is the whole functional change the cycle uses to claim the chronic field-inventory warning was cleared.
- The adjacent tests still only assert explicit refresh-only handling for `review_agent` and `project_mode`, plus generic uniqueness/documented-reason coverage for the list as a whole.
- `cargo test -p refresh-field-inventory --manifest-path tools/rust/Cargo.toml` passes with 17 tests, but none call `verify_field(..., "step_comment_acknowledged_gaps")` or exercise a stale-field refresh path using the new entry, so the exact branch added in cycle 457 is still unpinned by a focused regression.
**Recommendation**: Add a narrow test that proves `step_comment_acknowledged_gaps` is accepted as refresh-only (and ideally refreshed in place). That would keep future list edits from silently reopening the same chronic warning.

## Complacency score

**2/5** — The cycle did real verification work: `bash tools/state-invariants`, `bash tools/metric-snapshot`, and `bash tools/cycle-receipts --cycle 457 --repo-root .` all line up, receipt hashes resolve, and `pipeline-check --cycle 457 --json` shows full mandatory step coverage on `#2278`. But the close-out still over-credits partially-met commitments, a tool-first rule was violated badly enough to create and then close duplicate dispatch artifacts, and the one direct Rust hotfix that supposedly cleared a chronic warning landed without a field-specific regression test. That is active work, not trustworthy discipline.
