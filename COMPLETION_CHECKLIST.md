# Cycle Completion Checklist

Follow this checklist at the end of every orchestrator cycle. Do not skip steps.

**Critical**: The review agent dispatch (step 5) is MANDATORY. Every cycle must end with a review agent in-flight. Eva directive [#463](https://github.com/EvaLok/schema-org-json-ld/issues/463).

## 1. Run pipeline verification

Confirm the pipeline-check was run this cycle and passed:

```bash
bash tools/pipeline-check --cycle {N}
```

If not yet run, run it now. All 5 phases must pass before completing the cycle.

## 2. Update state.json

Update these fields in `docs/state.json`:

- `last_cycle.number` — current cycle number
- `last_cycle.timestamp` — current UTC timestamp
- `last_cycle.issue` — this cycle's issue number
- `copilot_metrics.in_flight` — should be 0 at cycle end (or 1 if review agent was just dispatched)
- Any other fields that changed this cycle
- **Field inventory freshness reconciliation** (per review cycle 142, finding #2; escalated per audit #106): For every **tracked field group** updated in state.json, also update its corresponding `field_inventory.fields.*.last_refreshed` to the current cycle number. This is an **atomic invariant** — any tracked field edit without a matching freshness update is a bug. Field inventory entries use **grouped coverage**: a single entry like `copilot_metrics` covers all subfields (`in_flight`, `note`, `dispatch_log_latest`, etc.). When any subfield within a group changes, update the group's `last_refreshed`. Not every leaf-level subfield needs its own inventory entry — only the top-level group key needs one. This prevents cadence drift while keeping the inventory manageable.

## 3. Write worklog entry

Create a worklog entry at `docs/worklog/YYYY-MM-DD/HHMMSS-cycle-name.md` with:

- **What was done**: Summary of cycle activities
- **Self-modifications**: Any changes to infrastructure files (AGENTS.md, skills, checklists, etc.)
- **Current state**: In-flight sessions, pipeline status, metrics
- **Next steps**: What the next cycle should prioritize

## 4. Write journal entry

Append to `docs/journal/YYYY-MM-DD.md` with reflections on:

- Challenges encountered
- Decisions made and their rationale
- Patterns observed
- Open questions

## 5. Dispatch review agent (MANDATORY)

Dispatch a 5.3-codex agent to perform an end-of-cycle review. This agent's findings will be waiting at the start of the next cycle. The review agent should examine:

1. **Code changes this cycle** — any merged PRs, direct pushes, quality concerns
2. **Worklog entry** — is it accurate, complete, and honest about what happened?
3. **Journal entry** — does it reflect genuine learning or is it formulaic/complacent?
4. **State.json** — are metrics accurate? Any stale fields?
5. **Commit receipts** — verify any receipt hashes listed in the closing comment with `git show <hash> --stat`. Confirm the committed changes match what the orchestrator claimed (per Eva directive [#538](https://github.com/EvaLok/schema-org-json-ld/issues/538))
6. **Infrastructure** — are AGENTS.md, skills, and checklists consistent with actual practice?
7. **Complacency check** — are we falling into patterns of going through the motions without genuine improvement?

### How to dispatch

**CRITICAL**: Copilot coding agents CANNOT post issue comments. They always create PRs. The review agent must commit its findings as a markdown file.

Write the issue body to a temp file, then create using `gh api --input`.

The issue body MUST instruct the agent to:
- **Commit findings as `docs/reviews/cycle-NNN.md`** — this is the only reliable output path
- NOT attempt to post issue comments (this will silently fail)
- The PR containing the review file is the deliverable

The issue body should include:
- Current cycle number and issue link
- List of PRs merged this cycle (if any)
- Paths to this cycle's worklog and journal entries
- Specific areas of concern (if any)
- Output format: commit a structured markdown file at `docs/reviews/cycle-{N}.md`

Label the issue `agent-task` and `cycle-review`.

**Important**: The next cycle consumes review findings by reading the review file from the merged PR (or from the PR branch if not yet merged).

## 6. Commit and push all state with receipts

Commit all changes and push to master. When state.json is modified, use `tools/commit-state-change` to produce a verifiable receipt hash.

### Commit receipts (per Eva directive [#538](https://github.com/EvaLok/schema-org-json-ld/issues/538))

When updating state.json, commit via the receipt utility and record the hash:

```bash
# After applying state.json updates:
bash tools/commit-state-change --tool-name "cycle-complete" --summary "cycle N state updates" --cycle N
```

Record the receipt hash in the cycle closing comment. For `metric-snapshot --fix`, the wrapper handles the commit automatically and outputs `commit-receipt: <hash>`.

| Step | Tool | Commit receipt |
|------|------|----------------|
| Pipeline verification | `pipeline-check` | (read-only, no receipt) |
| State.json update | `commit-state-change` | _______ |
| Metric fix (if needed) | `metric-snapshot --fix` | _______ |
| Worklog + journal | manual commit | _______ |
| Review agent dispatch | manual commit | _______ |

Include filled-in receipts in the closing comment so the review agent can verify them with `git show <hash>`.

### Other files

For non-state.json changes (worklog, journal, infrastructure), commit normally:

```bash
git add docs/worklog/ docs/journal/ [other changed files]
git commit -m "Cycle N: worklog, journal, [summary]"
git push origin master
```

## 7. Close the orchestrator issue

Post a closing summary comment on the cycle issue and close it.

The summary should include:
- What was accomplished
- Pipeline status
- Review agent issue number (from step 5)
- Commit receipts (from step 6)
- Next cycle priorities

## Automation status

| Step | Status | Tool |
|------|--------|------|
| 1. Pipeline verification | Automated | `bash tools/pipeline-check` |
| 2. State.json updates | Semi-automated | `bash tools/cycle-complete` generates patches |
| 3. Worklog entry | Semi-automated | `bash tools/write-entry worklog` |
| 4. Journal entry | Semi-automated | `bash tools/write-entry journal` |
| 5. Review agent dispatch | Semi-automated | `bash tools/cycle-complete` generates issue body |
| 6. Commit with receipts | Semi-automated | `bash tools/commit-state-change` for state.json |
| 7. Close issue | Manual | Standard gh commands |
