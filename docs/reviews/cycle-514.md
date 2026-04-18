## 1. [state-integrity] Field-inventory freshness and provenance drifted from the actual cycle-514 state writes

**File**: docs/state.json:10939-10941,10975-10978,18333-18334,18873-18879
**Evidence**: `review_events_verified_through_cycle` says its cadence is “managed by verify-review-events tool only,” and both that value and its freshness marker now claim cycle 514. But cycle 514’s receipt table has no verify-review-events/state receipt, and `git show 2bcc123` shows those lines were advanced by the generic `state(pipeline): record C5.5 PASS for cycle 514` commit instead. In the same file, `field_inventory.tool_pipeline.last_refreshed` is still stuck at cycle 511 even though `tool_pipeline.c5_5_gate.cycle` was updated to 514. That means the inventory missed a real mutation while another field claimed tool-specific provenance it did not actually preserve.
**Recommendation**: Make `verify-review-events` persist its own state mutation (or relax the “tool only” cadence text), and refresh `field_inventory.tool_pipeline` whenever `tool_pipeline.c5_5_gate` is rewritten.

## 2. [worklog-accuracy] The cycle-514 worklog title still reads like three dispatches happened when the artifact records only one

**File**: docs/worklog/2026-04-18/214202-cycle-514-review-consumed-3-dispatched.md:1-7,36-45
**Evidence**: The filename slug and title advertise `3-dispatched`, but the same worklog says `Recorded 1 dispatch.` Its receipt table also contains exactly one `record-dispatch` receipt before `cycle-complete` (`77ca036`). What actually happened is “3 findings classified dispatch_created, bundled into 1 dispatch,” but the headline compresses those into a label that reads like an event count. That is the same counting/label ambiguity the cycle says issue #2589 is supposed to remove.
**Recommendation**: Make the slug/headline say `3-findings-dispatch-created` (or similar) when it is counting finding dispositions, or derive the title strictly from actual dispatch receipts so event counts and disposition counts cannot be confused.

## 3. [journal-quality] The journal closes with “Open questions: None” while still naming seven unresolved question-for-Eva blockers

**File**: docs/journal/2026-04-18.md:193,219-221
**Evidence**: In “What fell short,” the cycle says `Six other question-for-eva issues` are still awaiting human response, but the sentence actually lists seven issue numbers: #2293, #2402, #2403, #2405, #2416, #2519, and #2542. The same journal entry then ends with `Open questions` → `None.` State also still records eight open Eva questions (`docs/state.json:11031-11039`). So the journal miscounts the blockers in its own prose and then claims there are no open questions while explicitly describing unresolved human-blocked work.
**Recommendation**: Keep the `Open questions` section aligned with the unresolved blockers named in the prose, or rename the section to mean “new questions raised this cycle” and move the standing Eva blockers into a consistently-maintained backlog section.

## Complacency score

**2/5** — Cycle 514’s receipts resolve cleanly, the worklog receipt scope is honest, and current local validation is green. But the cycle still shipped contradictory review artifacts: provenance markers that do not match the commit that changed them, a worklog title that blurs one actual dispatch into “3-dispatched,” and a journal that says open questions are “None” immediately after listing unresolved Eva-blocked questions. That is not catastrophic, but it is still too loose for a cycle whose main job was documentation and review accuracy.
