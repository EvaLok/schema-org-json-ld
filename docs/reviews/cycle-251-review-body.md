## Adversarial Review Mandate

Your job is to find everything wrong with this cycle's work. Be thorough. Be skeptical. If something looks fine on the surface, dig deeper. This is an adversarial review — actively look for problems, inconsistencies, drift, and complacency. Do not assume good faith or give the benefit of the doubt.

## Cycle 251 Summary

- **Cycle issue**: EvaLok/schema-org-json-ld#1200
- **PRs merged**: EvaLok/schema-org-json-ld#1199 (cycle 250 review artifact), EvaLok/schema-org-json-ld#1197 (write-entry auto-derive PRs, SHA validation)
- **Direct pushes to master**: record-dispatch dedup guard (8cac295), review history update (24ba4da), cycle-complete (e304084), docs (e78aea3)
- **Dispatched**: None (review-only cycle)

## Review Targets

### 1. Code changes
- **record-dispatch dedup guard** (8cac295): Added duplicate issue check in `apply_dispatch_patch()`. Verify the dedup logic is correct, test coverage is adequate, and no edge cases are missed (e.g., what happens if someone legitimately needs to re-dispatch for the same issue after closing the first session?).
- **write-entry PR #1197** (merged): Auto-derives PRs merged/reviewed from process-merge receipts, validates SHA references in generated markdown. Check the implementation quality: does it handle edge cases? Are tests meaningful?

### 2. Worklog accuracy
- File: `docs/worklog/2026-03-14/030605-cycle-251-review-merge-dedup-fix.md`
- Cross-reference claims against actual commits, state.json, and issue activity. Does the narrative match reality?
- Verify receipt table completeness using `bash tools/cycle-receipts --cycle 251 --repo-root .`
- **Specific check**: The worklog should now show PRs merged/reviewed sections populated by the new auto-derivation. Verify this worked correctly.

### 3. Journal quality
- File: `docs/journal/2026-03-14.md`
- Is the journal genuine reflection or boilerplate? Does it contain actionable commitments with observable completion conditions?
- **Specific check**: The cycle 250 review found that the journal overstated what #1196 would solve. Does the cycle 251 journal accurately describe the scope of the write-entry improvements and the dedup fix?

### 4. State.json integrity
- Verify copilot_metrics match agent_sessions array (resolved + in_flight == total_dispatches)
- Check review_agent.history: was cycle 250 entry added correctly? Are disposition classifications accurate?
- Run `bash tools/state-invariants` and `bash tools/metric-snapshot`

### 5. Process adherence
- Did the orchestrator follow its own checklist? Did it use tools when tools exist?
- Did the orchestrator post per-step comments? Count step comments on EvaLok/schema-org-json-ld#1200.
- The orchestrator directly fixed the dedup bug in record-dispatch lib.rs. Was this appropriate for a direct push, or should it have been dispatched to Copilot?

### 6. Complacency detection
- chronic journal-quality is at 5/6 in last 6 reviews. The orchestrator claims PR #1197 addresses the root cause. Is this accurate, or is this another instance of overstating closure?
- The dedup fix addresses the state-integrity finding from cycle 250. Is the fix genuinely sufficient, or are there edge cases where duplicates could still occur?
- Are chronic categories (journal-quality 5/6, worklog-accuracy 4/6) being genuinely addressed or just acknowledged?

### 7. Commit receipt verification
- Verify receipt hashes resolve and match claims
- Run `bash tools/cycle-receipts --cycle 251 --repo-root .`

## Complacency scoring cap
If the cycle overrode any FAIL or blocking-level pipeline gate (including pipeline-check or state-invariants), the maximum complacency score is 3/5 regardless of other factors.

## Output format
Commit your findings as `docs/reviews/cycle-251.md` using this template for each finding:

```
## N. [category-name] Finding title

**File**: path/to/file:line
**Evidence**: what was observed
**Recommendation**: concrete action
```

End with a justified complacency score (1-5). Three deeply investigated findings with evidence are more valuable than ten surface-level observations.

Do NOT attempt to post issue comments — commit the review file as your only output.
