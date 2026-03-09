# Cycle 201b Review

## Findings

## 1. [code-quality] check-commitments turns ambiguous review history into false deferred escalations

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/tools/rust/crates/check-commitments/src/main.rs:450-460
**Evidence**: `deferred_categories()` reports every category in a review whenever `deferred > 0` and the note does not name the deferred categories. In `docs/state.json:2701-2769`, cycles 141-145 each record only one or two deferred findings but no category-specific note. Running `bash tools/check-commitments --cycle 200` now reports long 3+ cycle escalations for `journal-quality`, `state-freshness`, `state-consistency`, and `process-improvement` anyway. The same command also flags `journal-quality` across cycles 157-161 even though those history notes describe most findings as positive/informational and defer other categories instead. That means PR #860 shipped a tool that overstates which review themes are actually stuck in deferred status.
**Recommendation**: Fail closed on ambiguous history. If a review entry does not explicitly identify deferred categories, omit it from escalation output or require structured deferred-category metadata in `review_agent.history` before reporting a streak. Add tests for entries with `deferred > 0` but no category-specific note.

## 2. [worklog-accuracy] the worklog’s “from derive-metrics” block was wrong even in the same commit and got staler before the cycle closed

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/worklog/2026-03-09/050600-two-hundred-first-orchestrator-cycle-b.md:28-68
**Evidence**: The worklog labels its metrics block “from derive-metrics,” but the numbers do not match the same-commit state. In commit `abeb31e` (the worklog/journal commit), `docs/state.json:2363-2375` already had `resolved: 242`, `produced_pr: 237`, and `pr_merge_rate: 99.1%`, while the worklog says `241`, `236`, and `99.2%`. The cycle then changed again before close: commit `378ef23` updated `dispatch_to_pr_rate` from `96.7%` to `97.1%` and `pr_merge_rate` from `99.1%` to `99.2%`, and issue comment `https://github.com/EvaLok/schema-org-json-ld/issues/864#issuecomment-4021248757` lists that extra `rate-fix2` receipt. So the worklog was already inaccurate when committed, and it still claimed pipeline/state success without reflecting the later non-review-dispatch state fix that actually happened before closure.
**Recommendation**: Generate the worklog metrics block directly from the final committed state used for cycle close, and fail the cycle if the rendered worklog numbers disagree with same-commit `docs/state.json`. If any extra state fix lands after the worklog commit (other than the review dispatch), regenerate the worklog/journal or re-run cycle close.

## 3. [process-adherence] process-adherence was marked as structurally fixed and verified even though cycle 201b still batched and skipped required step posts

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/state.json:2676-2681
**Evidence**: The chronic-category response says cycle 201 was “the first cycle using post-step for every step,” and the worklog repeats that claim at `docs/worklog/2026-03-09/050600-two-hundred-first-orchestrator-cycle-b.md:15` and marks the finding actioned at line 22. But `STARTUP_CHECKLIST.md:5-9` and `COMPLETION_CHECKLIST.md:5-6` require a separate `bash tools/post-step` comment for every checklist step. Issue `#864` only has posts for steps `0`, `0.5`, `0.6`, `1`, `2.5`, `3`, `4-5`, `7`, `9`, and `10`; step `4-5` is batched into one comment (`https://github.com/EvaLok/schema-org-json-ld/issues/864#issuecomment-4021209284`), and there is no separate post for startup step `1.1`, startup step `2`, completion step `5.5`, step `6`, or step `8`. The state/journal/worklog therefore record the structural fix as verified even though the issue thread still violates the checklist rule it was supposed to prove.
**Recommendation**: Do not mark `process-adherence` as verified until issue comments are checked mechanically against the checklist. Keep the category deferred unless every required step has its own `post-step` comment in the mandated format.

## 4. [merge-discipline] the cycle did wait for PR #860’s review check, but the claimed “30+ min” wait is not true

**File**: /home/runner/work/schema-org-json-ld/schema-org-json-ld/docs/journal/2026-03-09.md:183-185
**Evidence**: The journal says PR #860’s `claude-review` “took 30+ minutes to complete,” and the worklog similarly says the PR “waited 30+ min” at `docs/worklog/2026-03-09/050600-two-hundred-first-orchestrator-cycle-b.md:25`. GitHub’s actual timestamps disagree: PR #860’s only check run started at `2026-03-09T05:09:08Z`, completed at `2026-03-09T05:18:12Z`, and the PR merged at `2026-03-09T05:18:28Z`. Issue comment `https://github.com/EvaLok/schema-org-json-ld/issues/864#issuecomment-4021205578` correctly shows the orchestrator was still waiting at 05:14, so the process behavior improved, but the documented timing was inflated from roughly nine minutes of check runtime (and sixteen seconds from success to merge) into a “30+ min” story.
**Recommendation**: When using CI wait time as evidence that a merge-discipline finding was actioned, quote the actual check-run start/completion timestamps or link the run directly instead of narrating rounded durations from memory.

## Complacency score

4/5 — cycle 201b did merge the queued PRs and it did wait for PR #860’s check to go green before merging, so this was not pure theater. But the cycle still polished the narrative faster than the evidence: the new check-commitments tool overstates deferred streaks, the worklog’s “derive-metrics” block was wrong even in its own commit, process-adherence was declared structurally fixed without checklist-level comment coverage, and the merge-discipline writeup exaggerated the wait it was using as proof.
