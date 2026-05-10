# [question-for-eva] Add `tools/dispatch-task` and `tools/dispatch-review` to runtime allowlist (Copilot dispatches blocked across cycles 112 and 113)

**From:** main-orchestrator (redesign cycle 113, 2026-05-10)
**Category:** Infrastructure / CI configuration (per `EVA-DEFAULT-AUTONOMY` and `direct-push-zones` forbidden-zone)
**Urgency:** medium — does not block cycle 113's substantive work; blocks high-leverage iteration substrate (Copilot adversarial-feedback dispatches) for cycle 114+

## Summary

The redesign prompt's `COPILOT-DISPATCHES` and `COPILOT-DISPATCH-METHOD` sections explicitly authorize feedback-only and research-only Copilot dispatches via `tools/dispatch-task --skip-pipeline-gate`. The runtime permission allowlist in [`.github/workflows/orchestrator.yml`](https://github.com/EvaLok/schema-org-json-ld/blob/master/.github/workflows/orchestrator.yml) (lines 66-87) does not include either `tools/dispatch-task` or `tools/dispatch-review`. Cycle 112 first observed this when its adversarial feedback dispatch on the Phase 2 selection draft was blocked despite the body file being authored and the action being authorized by the redesign prompt. **Cycle 113 reconfirmed the block is structural** by retrying the same dispatch — the runtime requires interactive approval which cannot be granted in the cron-triggered automation context.

## Proposed fix

Two-line addition to [`.github/workflows/orchestrator.yml`](https://github.com/EvaLok/schema-org-json-ld/blob/master/.github/workflows/orchestrator.yml) at line 83 (after `Bash(tail *)`):

```yaml
                  "Bash(tools/dispatch-task *)",
                  "Bash(tools/dispatch-review *)",
```

This matches the existing pattern (`Bash(<command> *)`). Both tools encapsulate the GraphQL Copilot-assignment mechanism per ADR 0016; without these entries, the only working dispatch path is blocked, and the prompt's `COPILOT-DISPATCH-METHOD` forbids the alternative (`gh issue create` directly) because cycles 63-92 demonstrated that path produces stuck dispatches.

## Why this is a question-for-eva, not a self-resolved PR

I attempted the standard forbidden-zone fix path: open a PR you merge.

1. Created branch `cycle-113-allow-dispatch-task-runtime` with the two-line diff above.
2. Committed locally.
3. **`git push -u origin cycle-113-allow-dispatch-task-runtime` was rejected** by GitHub:

> ! [remote rejected] cycle-113-allow-dispatch-task-runtime -> cycle-113-allow-dispatch-task-runtime (refusing to allow a Personal Access Token to create or update workflow `.github/workflows/orchestrator.yml` without `workflow` scope)
> error: failed to push some refs to 'https://github.com/EvaLok/schema-org-json-ld.git'

`ORCHESTRATOR_PAT` does not have `workflow` scope, so this orchestrator session cannot create a PR that modifies `.github/workflows/`. The local branch has been cleaned up; the proposed diff is preserved here in `docs/redesign/_notes/cycle-113-q4eva-runtime-allowlist.md`.

**This is double structural friction:**
- Level 1 (runtime allowlist gap) — blocks `tools/dispatch-task` execution
- Level 2 (PAT `workflow` scope gap) — blocks the orchestrator-side PR fix for level 1

## Resolution paths Eva can choose

**Option A (recommended):** Apply the two-line allowlist change manually (commit directly to master with your account, since you have the necessary scope). Lowest-friction path; preserves PAT scope as-is. The orchestrator does not need PAT `workflow` scope for normal operation — adding it only to enable this one fix would be over-broad.

**Option B:** Add `workflow` scope to `ORCHESTRATOR_PAT`. Enables the orchestrator to open future workflow-change PRs autonomously. Slightly broader privilege than necessary for the immediate need; acceptable if you anticipate further workflow changes during the redesign phase that the orchestrator should be able to propose via PR.

**Option C:** Decline both. Copilot dispatches remain blocked across the redesign phase. Cycle 113's substantive work was not blocked by this — the cycle did central-bet stress-test on the equivalent-reconciliation-outcomes claim and surfaced a finding. But the next-most-leverage iteration substrate (external adversarial feedback on the selection draft) is structurally unavailable, which weakens `ITERATION-UNTIL-APPROVAL`'s third activity ("Solicit critique: dispatch a Copilot feedback-only session"). This option is honest if you judge that audit-as-peer cross-repo critique (cycle 216 expected ~17h from now) is sufficient external critique substrate for the candidate-selection checkpoint.

## What is not blocked

- All cycle 113 substantive work landed (selection draft updated; 2-candidates/README.md updated; cycle 113 _notes + journal will follow).
- Audit-as-peer cross-repo critique is unaffected (audit reads main repo each cycle; no dispatch needed).
- Cycle 113 is the twenty-fifth consecutive cycle with bottleneck-external posture; the dispatch friction does not change that.

## Forward dependency

The cycle 112 adversarial feedback dispatch body file is preserved at [`docs/redesign/_notes/cycle-112-dispatch-1-body.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/_notes/cycle-112-dispatch-1-body.md) (~140 lines, two adversarial lenses in parallel — argue-for-flipping-to-C + stress-test-cycle-112-reasoning). It is ready to dispatch the moment the runtime allowlist permits. Cycle 113 has additional stress-test material (the equivalent-reconciliation-outcomes processing-vs-detection finding) that would fit a follow-up dispatch body once the allowlist resolves.

## Per `BETWEEN-CHECKPOINTS` 5-cycle autonomy default

If no Eva response within 5 of my cycles (~1.25 calendar days at ~4 cycles/day), I will proceed under Option C (continue without dispatch substrate; rely on audit-as-peer cross-repo critique as primary external lens). The dispatch body files remain ready for retroactive landing if Option A or B materializes later.

— main-orchestrator, redesign cycle 113
