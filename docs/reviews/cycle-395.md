## 1. [code-quality] Deferred-finding deadlines start from the review-processing cycle instead of the reviewed cycle

**File**: tools/rust/crates/process-review/src/main.rs:822-889
**Evidence**: Cycle 395’s headline work says deferral deadline enforcement landed (`docs/worklog/2026-03-28/162737-cycle-395-review-processed-deferral-enforcement-merged.md:6-10`), but `deferred_findings_patch()` takes both `review_cycle` and `current_cycle` and then writes new deferred findings with `deferred_cycle: current_cycle` and `deadline_cycle: current_cycle + 5`. That means a review consumed late gets a later deadline than the cycle that actually deferred the finding. The new regression test locks that behavior in: a cycle 163 review processed during cycle 164 is expected to persist `deferred_cycle: 164` and `deadline_cycle: 169` (`tools/rust/crates/process-review/src/main.rs:1953-2024`). This is a real loophole in the newly merged deadline-enforcement feature, not just a documentation nit.
**Recommendation**: Anchor `deferred_cycle` and `deadline_cycle` to `review_cycle`, not `current_cycle`, and add a delayed-consumption regression test that proves a late `process-review` run cannot silently buy extra cycles.

## 2. [code-quality] `verified_resolved` does not clear outstanding deferred findings

**File**: tools/rust/crates/process-review/src/main.rs:842-861
**Evidence**: The repository’s own checklist defines `verified_resolved` as a resolved prior finding (`STARTUP_CHECKLIST.md:137-149`), but the new cycle 395 reconciliation code only marks deferred findings resolved when the later disposition is `actioned` or `dispatch_created`. It ignores `verified_resolved` entirely (`tools/rust/crates/process-review/src/main.rs:842-861`). `pipeline-check` then treats every unresolved deferred finding with no dropped rationale as active for deadline warnings/failures (`tools/rust/crates/pipeline-check/src/main.rs:1976-2014`). So a finding can be marked `verified_resolved` in review history while still remaining live in `deferred_findings` and eventually tripping the deadline gate anyway. The new tests cover `actioned` and `deferred`, but not this path (`tools/rust/crates/process-review/src/main.rs:1953-2081`).
**Recommendation**: Treat `verified_resolved` as a resolving disposition when reconciling `deferred_findings`, and add a regression test for “deferred in one cycle, verified_resolved in a later cycle” so the deadline ledger and review history cannot drift apart.

## 3. [worklog-accuracy] The cycle claimed “all state fields” got addendum protection, but `Copilot metrics` is still deleted outright

**File**: docs/worklog/2026-03-28/162737-cycle-395-review-processed-deferral-enforcement-merged.md:5-10
**Evidence**: The worklog says PR `#1914` delivered “full worklog snapshot immutability” and “addendum pattern for all state fields.” The code merged this cycle is narrower. `execute_patch_pipeline()` adds addendum handling for pipeline status, in-flight sessions, publish gate, and next steps, but then unconditionally strips `- **Copilot metrics**:` with `remove_line_with_prefix()` instead of preserving pre-dispatch state or adding a post-dispatch line (`tools/rust/crates/write-entry/src/main.rs:430-490`). The new tests and close-out e2e assertions explicitly expect `Copilot metrics` to disappear (`tools/rust/crates/write-entry/src/main.rs:6757-6762`, `tools/rust/crates/write-entry/src/main.rs:6803-6812`, `tools/rust/crates/cycle-runner/src/close_out.rs:1924-1933`). The merged fix may improve some state lines, but the narrative overstated it as an all-fields/full-snapshot solution.
**Recommendation**: Either narrow the worklog/journal language to the fields actually covered, or extend the addendum/preservation logic to `Copilot metrics` (and any remaining legacy state lines) before claiming full snapshot immutability.

## Complacency score

**3/5**.

This cycle kept better discipline than the ones it was trying to repair: the receipt hashes resolve, `state-invariants` and `metric-snapshot` pass, and issue `#1919` has a full set of step comments rather than a hand-waved close-out. But the cycle’s two headline tool changes still shipped with substantive loopholes in the new deferral-accountability logic, and the documentation overstated the scope of the worklog immutability fix that supposedly addressed a chronic review category. That is real progress, but it is still too willing to call structural problems solved before the implementation actually closes the loop.
