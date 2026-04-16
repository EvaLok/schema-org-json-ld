# Cycle 505 Review

## 1. [code-change-quality] The new stale-session writer can mark unrelated sessions stale

**File**: tools/rust/crates/cycle-runner/src/startup.rs:513-535
**Evidence**:
- `pipeline-check` builds the authoritative drift message as `agent session issue #N "title" is closed on GitHub but still marked in_flight`, so the quoted title is part of the string being parsed (`tools/rust/crates/pipeline-check/src/main.rs:4348-4352`).
- The new parser deliberately extracts **every** `#N` token from that whole detail string: line 517 says “We take every `#N` occurrence”, and lines 523-533 implement exactly that.
- Live `docs/state.json` already contains many agent-session titles with extra issue references, e.g. `#311: Port QAPage + Restaurant to TypeScript (audit #37 fix)`, `#587: Tool: cycle-complete --apply (write-side pipeline, step 1 of #586)`, and `#2317: Tool: pipeline-check structural hardening ...`.
- `mark_stale_agent_sessions_in_state` then cross-references every parsed number against `agent_sessions` and writes `last_seen_stale_at_cycle` to matching in-flight rows (`tools/rust/crates/cycle-runner/src/startup.rs:476-510`). A stale session whose title mentions another issue can therefore stamp an unrelated session as stale and cause a false FAIL on the next cycle.
**Recommendation**: Parse only the leading `agent session issue #N` identifier emitted by `pipeline-check`, or otherwise ignore `#N` tokens inside the quoted title. Add a regression test with a title containing another issue reference.

## 2. [journal-quality] The cycle 505 journal carries forward the wrong deferred findings and wrong fixes

**File**: docs/journal/2026-04-16.md:221,248-249
**Evidence**:
- The consumed review for cycle 504 says F1 is a **worklog-accuracy** defect about inventing a dependency disproved by git ancestry, and F3 is a **journal-quality** defect about collapsing mixed commitment outcomes into `**Not followed.**` (`docs/reviews/cycle-504.md:3-30`).
- The cycle 505 journal rewrites those deferred findings into different problems: line 221 says F1 is about “Pre-dispatch state” snapshot timing and F3 is “fact-claim drift in the cycle 503 journal”.
- The backlog then assigns the wrong remediation plan: line 248 maps cycle-504 F1 to the `write-entry journal` per-commitment-status fix, which is actually the remedy for cycle-504 F3; line 249 maps cycle-504 F3 to a vague “two inaccuracy categories” design task that is not what the review artifact requested.
- This is not a harmless wording difference: the journal is the carry-forward planning artifact for the next cycle, and it is now pointing future work at the wrong implementation targets.
**Recommendation**: Generate deferred-finding carry-forward text from the consumed review artifact or structured state, not from hand-written paraphrases. At minimum, preserve each finding’s original category and remediation target verbatim when deferring it.

## 3. [state-integrity] `state.json` collapses the consumed cycle-504 review into generic chronic-category boilerplate

**File**: docs/state.json:17938
**Evidence**:
- The cycle-504 history note says: `F1 worklog-accuracy and F3 journal-quality deferred (chronic categories; same structural root cause as prior 10+ cycles of deferrals).`
- The actual review artifact records two specific findings with different evidence and recommendations: F1 is the worklog’s fictitious dependency on commit `eb3fedf0`, while F3 is the journal’s mixed-outcome `**Not followed.**` collapse (`docs/reviews/cycle-504.md:3-30`).
- By reducing both to “same structural root cause as prior 10+ cycles”, the ledger loses the concrete reason those findings were deferred and makes the consumed review look like generic chronic churn instead of two distinct unresolved defects.
- This matters because future cycles are instructed to use `state.json` as the authoritative ledger of deferred work; once the specifics are erased here, later worklogs and journals predictably drift, exactly as cycle 505’s journal already did.
**Recommendation**: Preserve finding-specific summaries or structured references in `review_agent.history` instead of flattening deferred findings into generic chronic-category prose. The state ledger should retain enough detail to reconstruct what was actually deferred.

## Complacency score

**2/5** — The score is capped at **3/5** because cycle 505 used an admin override for PR `#2556` and worked through a startup `pipeline-check` failure. Within that cap, **2/5** is justified: the cycle did keep a full step-comment trail and its receipt table is now complete, but it also introduced a real stale-session parsing bug and both the journal and `state.json` drifted away from the review they were supposed to consume.
