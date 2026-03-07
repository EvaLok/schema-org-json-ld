# Formal Tool Audit — Cycle 181 (10-cycle boundary)

Produced in response to cycle 180 review finding `formal-tool-audit-gap`.

## Tool inventory

| Tool | Rust crate | Purpose |
|------|-----------|---------|
| `cycle-start` | cycle-start | Initialize cycle: claim number, post opening comment, produce situation report |
| `cycle-complete` | cycle-complete | End-of-cycle: generate state patches, session duration, review agent body |
| `cycle-status` | cycle-status | Query cycle state, concurrency, Eva comments, agent status |
| `pipeline-check` | pipeline-check | Orchestrate all 5 verification phases |
| `metric-snapshot` | metric-snapshot | Verify 9 file-count metrics against state.json |
| `check-field-inventory-rs` | check-field-inventory | Validate field inventory completeness |
| `housekeeping-scan` | housekeeping-scan | Detect stale issues, orphan PRs, dead branches |
| `state-invariants` | state-invariants | Validate 11 structural invariants in state.json |
| `process-review` | process-review | Record review findings in state.json history |
| `process-audit` | process-audit | Record audit processing decisions |
| `process-eva` | process-eva | Track Eva directive issue lifecycle |
| `process-merge` | process-merge | Update copilot metrics after PR merges |
| `record-dispatch` | record-dispatch | Record agent dispatches in state.json |
| `write-entry` | write-entry | Scaffold worklog/journal entries |
| `commit-state-change` | (shell) | Git commit+push for state changes |
| `backfill-sessions` | backfill-sessions | Backfill missing session data |
| `parity-check.ts` | (TypeScript) | PHP/TS output parity comparison |

**Total: 17 tools** (15 Rust crates + 1 shell wrapper + 1 TypeScript tool)

## Checklist step coverage

| Step | Description | Coverage | Tool(s) |
|------|-------------|----------|---------|
| 0 | Cycle init | HIGH | `cycle-start` |
| 0.5 | Review agent check | MODERATE | `process-review` (records only; manual PR/issue lifecycle) |
| 1 | Eva input | MODERATE | `process-eva` (tracks state; manual polling/acting) |
| 1.1 | Eva comments | MODERATE | `cycle-start` includes in situation report |
| 1.5 | Conditional approvals | NONE | Fully manual |
| 2 | Recover context | MODERATE | Tools for metrics; manual file reading |
| 2.5 | Pipeline check | HIGH | `pipeline-check` (5 sub-tools) |
| 3 | Agent work status | LOW-MODERATE | `cycle-status` provides data; manual `gh` commands for lifecycle |
| 4 | QC repo | LOW | `cycle-status` tracks; manual investigation/response |
| 5 | Audit repo | MODERATE | `process-audit` records; manual polling/evaluation |
| 5.5-5.12 | Various gates | LOW-NONE | Mostly manual verification |
| 6 | Re-examine | NONE | Inherently manual (reflection) |
| 7 | Housekeeping | MODERATE | `housekeeping-scan` detects; manual closure |
| 8 | Concurrency | MODERATE | `cycle-status` counts; manual gating |
| 9 | Plan work | NONE | Inherently manual (planning) |
| 10 | Cycle complete | HIGH | `cycle-complete` + `write-entry` |

## Maturity assessment

- **Stage 3 (Pipeline)**: Steps 0, 2.5, 10 — fully pipelined via composite tools
- **Stage 2 (Tool)**: Steps 0.5, 1, 5, 7 — dedicated tools exist, manual orchestration remains
- **Stage 1 (Partial tool)**: Steps 3, 4, 8 — tools provide data, actions are manual
- **Stage 0 (Manual)**: Steps 1.5, 5.5-5.12, 6, 9 — no tool coverage

## What is still manual (candidates for automation)

### High priority (done every cycle)
1. **PR review lifecycle** (step 3): Checking `copilot_work_finished`, marking ready, waiting for CI, merging — 4+ manual `gh` commands per PR. A `process-pr` tool could automate the status checking and ready-marking.
2. **Housekeeping actions** (step 7): `housekeeping-scan` detects problems but doesn't fix them. A `--fix` mode could close stale issues, delete dead branches, and close orphan PRs automatically.
3. **Issue/PR commenting and closing**: Multiple manual `gh api` calls per cycle for posting signed comments and closing issues. A `post-comment` tool could handle signing and posting.

### Medium priority (done frequently)
4. **QC repo polling and response** (step 4): Manual `gh api` calls to poll QC repo, create `qc-inbound` issues, and close feedback loops. Could be partially automated.
5. **Audit repo polling** (step 5): Similar to QC — manual polling and response creation.

### Low priority (rare or inherently manual)
6. **Conditional approval reconciliation** (step 1.5): Rare — only when Eva provides conditional approval.
7. **Pre-publish gates** (steps 5.9-5.12): Only run before publishes.
8. **New-language prerequisite gate** (step 5.5): Only run when adding a language.
9. **Re-examine assumptions** (step 6) and **Plan work** (step 9): Inherently manual reasoning.

## Changes since last audit (cycle 170 boundary)

### New tools since cycle 170
- `cycle-start` — replaced manual opening comment + state initialization
- `process-review`, `process-audit`, `process-eva`, `process-merge` — state management tools
- `record-dispatch` — dispatch tracking
- `write-entry` — worklog/journal scaffolding
- `commit-state-change` — standardized commit+push
- `backfill-sessions` — historical data repair
- Session duration tracking added to `cycle-complete` (this cycle)

### Tool maturity progress
The project moved from ~5 tools at cycle 170 to 17 tools at cycle 181. The `process-*` family and `cycle-start` were the most impactful additions — they eliminated the largest categories of manual state.json editing.

## Follow-up dispatches

1. **Housekeeping --fix mode**: Extend `housekeeping-scan` to optionally close stale issues and delete dead branches. This eliminates 2-5 manual `gh` commands per cycle.
2. **PR lifecycle tool**: A `process-pr` tool that checks Copilot status, marks PRs ready, and reports CI status — reducing the manual PR review workflow from 4+ commands to 1.
3. **Field-inventory verification receipts**: Per review finding `field-inventory-restamp-evidence`, build a mechanism that produces a machine-readable receipt when fields are bulk-refreshed.

## Conclusion

Tool maturity has improved significantly. The core cycle lifecycle (init, pipeline check, completion) is at Stage 3 (Pipeline). The state management layer (`process-*` tools) moved manual state.json editing to Stage 2 (Tool). The main remaining gaps are in the PR lifecycle workflow (step 3) and housekeeping action execution (step 7). Steps 6 and 9 are inherently manual and should remain so.
