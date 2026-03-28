# Cycle 393 Review

## 1. [code-changes] The pipeline immutability fix only froze one field and left the rest of the cycle-complete snapshot mutable

**File**: tools/rust/crates/write-entry/src/main.rs:423-475; tools/rust/crates/cycle-runner/src/close_out.rs:1922-1935
**Evidence**: `execute_patch_pipeline()` now routes `- **Pipeline status**:` through `patch_or_addendum()`, but it still rewrites `- **In-flight agent sessions**:`, `- **Publish gate**:`, and the entire `## Next steps` section in place. The close-out regression test added with PR #1905 explicitly locks that behavior in: after review dispatch it expects the same worklog to show `- **In-flight agent sessions**: 1` and a new review issue in `## Next steps`, even though those values did not exist at cycle-complete time. This means the "immutability" fix was only partial and the mixed-timeline defect remained structurally encoded in both production code and tests.
**Recommendation**: Treat the full cycle-complete state snapshot as immutable, not just the pipeline-status line. Post-dispatch changes should be appended in a clearly labeled addendum for in-flight count, publish-gate drift, and next steps instead of rewriting the original state block in place.

## 2. [worklog-accuracy] The published cycle 393 worklog still mixes cycle-complete receipts with post-review-dispatch state

**File**: docs/worklog/2026-03-28/122535-cycle-393-review-processed-pipeline-immutability-merged.md:30-40
**Evidence**: The worklog now says there is `1` in-flight agent session and the next step is review issue `#1909`, but the note immediately above the receipt table says the scope is `cycle 393 commits through cycle-complete`. Git history shows those are different snapshots: `9f0d4a64a2d3b3cdd7ce440caeb74fa6e8dc7eab` is the `state(cycle-complete)` commit, `a0a4cd5f` records the later review dispatch for `#1909`, and `8ab4aadf` then refreshes the worklog after that dispatch. Current `docs/state.json` matches the post-dispatch state (`dispatch_log_latest` is `#1909` and `in_flight_sessions` is `1`), so the worklog is no longer a clean cycle-complete artifact even though the receipt table still presents itself that way.
**Recommendation**: Keep the main worklog state block frozen at cycle-complete. If review dispatch happens afterward, append a post-dispatch section with its own timestamp and scope note instead of mutating the original `Cycle state` and `Next steps` sections.

## 3. [complacency-detection] Cycle 393 closed the chronic worklog-accuracy finding before the artifact actually stopped exhibiting it

**File**: docs/worklog/2026-03-28/122535-cycle-393-review-processed-pipeline-immutability-merged.md:7; docs/journal/2026-03-28.md:239-251
**Evidence**: The worklog says the cycle 392 review's `worklog-accuracy` finding was "actioned" via PR #1905, and the journal goes further by saying the root cause was "finally fixed" and that the next review "should not contain a worklog-accuracy finding" related to the same problem. But the final cycle 393 worklog still mixes cycle-complete receipts with post-dispatch state, and the code/tests merged in PR #1905 still deliberately rewrite non-pipeline fields after dispatch. The cycle therefore declared victory on a chronic review category before the observable artifact matched the claim.
**Recommendation**: Do not mark a chronic review finding as actioned until the generated artifact no longer reproduces it. Use a `partial` or still-`deferred` disposition when a fix addresses only one facet of the defect, and make the journal commitment target full snapshot immutability rather than only pipeline-status preservation.

## Complacency score

**3/5**.

The cycle did real work — receipts resolve, step comments were posted for all 26 expected cycle-393 steps on issue `#1908`, and both `state-invariants` and `metric-snapshot` pass. But the review consumption was still complacent: the team declared the chronic `worklog-accuracy` category fixed, merged code and tests that preserved the remaining mixed-timeline behavior, and then published a worklog that still exhibits the defect it said was retired. That is more than a wording issue, but not total negligence, so `3/5` is warranted.
