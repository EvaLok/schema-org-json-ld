# Cycle Completion Checklist

Follow this checklist at the end of every orchestrator cycle. Do not skip steps.

**Step-level commenting**: As with the startup checklist, every step must be posted as a **separate comment** on the orchestrator run issue using `bash tools/post-step`. Post each step's outcome as you complete it — do not batch or summarize from memory.

**Critical**: The review agent dispatch (step C6) is MANDATORY. Every cycle must end with a review agent in-flight. Eva directive [#463](https://github.com/EvaLok/schema-org-json-ld/issues/463).

## C1. Run pipeline verification (early check)

Run the pipeline-check early to catch issues before they compound:

```bash
bash tools/pipeline-check
```

If not yet run, run it now. `pipeline-check` derives the current cycle from `docs/state.json`; use `--cycle {N}` only when you need to override it.

**Note**: This is an early check, not the final gate. State-modifying tools in step C2 may change state.json after this check passes. The final pipeline gate is step C5.5, which re-runs after all modifications are committed.

Post this step: `bash tools/post-step --issue {N} --step "C1" --title "Pipeline early check" --body "..."`

## C2. Update state.json via write-side tools

**Do NOT manually edit `docs/state.json`.** Use the write-side pipeline tools instead. Each tool atomically updates its owned section of state.json, bumps freshness markers, and commits with a receipt hash.

### During the cycle (as events occur)

| Event | Tool | Command |
|-------|------|---------|
| PR merged | `process-merge` | `bash tools/process-merge --prs 123,456 --issues 789,790 --merged-at "2026-03-16T05:00:00Z"` (`--issues none` when there are intentionally no linked issues; always pass `--merged-at` with the actual GitHub `merged_at` timestamp to ensure accurate session state) |
| Copilot task dispatched | `record-dispatch` | `bash tools/record-dispatch --issue N --title "..." --model gpt-5.4` |
| Review findings consumed | `process-review` | `bash tools/process-review --review-file docs/reviews/cycle-N.md --actioned A --deferred D --ignored I --dispatch-created DC --actioned-failed AF --verified-resolved VR` |
| Audit recommendation processed | `process-audit` | `bash tools/process-audit --audit-id N --action accepted` |
| Eva directive processed | `process-eva` | `bash tools/process-eva --closed 123,456 --remaining-open 247,436` |

### At cycle end

| Step | Tool | Command |
|------|------|---------|
| Update `last_cycle` fields | `cycle-complete` | `bash tools/cycle-complete --apply --commit --issue N --summary "..."` |
| Record review agent dispatch | `record-dispatch` | `bash tools/record-dispatch --issue N --title "Cycle N review" --model gpt-5.4` |

**Important**: Run `cycle-complete` BEFORE dispatching the review agent, so `last_cycle.number` is updated before `record-dispatch` reads it.

Each tool handles its own freshness markers automatically — no manual freshness reconciliation needed.

Post this step: `bash tools/post-step --issue {N} --step "C2" --title "State updates" --body "..."`

## C3. Write documentation entries

Write worklog and journal entries using `write-entry`:

```bash
bash tools/write-entry worklog \
  --title "Cycle N summary" \
  --done "Merged PR #123" --done "Processed audit #155" \
  --pr-merged 123 --pr-merged 456 \
  --next "Review PR from #825" \
  --pipeline "PASS (6/6)" \
  --in-flight 1 \
  --receipt "cycle-start:abc1234"
```

```bash
bash tools/write-entry journal \
  --title "Cycle N reflections" \
  --section "Decision::Chose to defer #829" \
  --commitment "Will dispatch #830 next cycle"
```

The `write-entry` tool auto-derives self-modifications from git history, validates receipts against `cycle-receipts` output, and handles JOURNAL.md index updates.

Post this step: `bash tools/post-step --issue {N} --step "C3" --title "Documentation entries" --body "..."`

## 4. (Removed — journal now part of Step C3)

Journal is written as part of Step C3 via `write-entry journal`. The tool appends to `docs/journal/YYYY-MM-DD.md`, handles JOURNAL.md index updates, auto-links bare `#N` references, and automatically inserts the matching worklog link.

## C4.1. Validate documentation entries (per review finding: worklog-accuracy chronic)

After writing worklog and journal entries, validate them before committing. This is a **blocking gate** — do not proceed to step C5 if validation fails.

```bash
bash tools/validate-docs worklog --file docs/worklog/YYYY-MM-DD/HHMMSS-title.md --cycle N --repo-root .
```

```bash
bash tools/validate-docs journal --file docs/journal/YYYY-MM-DD.md --repo-root .
```

If either validation fails:
1. Fix the issue (missing receipts, stale self-modifications, etc.)
2. Re-run validation until it passes
3. Only then proceed to step C5

**Why:** The `worklog-accuracy` category appeared in 4+ consecutive review cycles (235-239). The root cause is that worklogs were committed without running the validator that exists specifically to catch these errors. This step closes the gap by making validation mandatory before commit.

Post this step: `bash tools/post-step --issue {N} --step "C4.1" --title "Documentation validation" --body "..."`

## C4.5. ADR check (per Eva directive [#724](https://github.com/EvaLok/schema-org-json-ld/issues/724))

Before closing the cycle, check whether any decisions made this cycle warrant an Architecture Decision Record in `doc/adr/`.

ADR-worthy decisions include:
- New architectural patterns or significant refactors
- Technology choices (new languages, tools, frameworks)
- Process changes that affect how the system operates
- Design trade-offs where alternatives were considered and rejected

If a decision was made this cycle that meets these criteria, write the ADR now. Use the format established in existing ADRs: Status, Context, Decision, Consequences, Alternatives Considered.

Number ADRs sequentially (check `ls doc/adr/` for the next number).

Post this step: `bash tools/post-step --issue {N} --step "C4.5" --title "ADR check" --body "..."`

## C5. Commit worklog, journal, and state before review dispatch

**CRITICAL ORDERING**: The review agent reads the repo at dispatch time. If worklog and journal entries are not committed and pushed before the review dispatch, the review agent will flag them as "missing" — a false positive that has contaminated complacency scores since cycle 189. Per audit [#151](https://github.com/EvaLok/schema-org-json-ld-audit/issues/151).

Before dispatching the review agent:
1. Commit all worklog and journal entries
2. Commit any state.json changes from `cycle-complete`
3. Push to master
4. **Verify** the push succeeded before proceeding to step C6

This ensures the review agent sees the complete cycle state, eliminating the artifact-race false positive.

Post this step: `bash tools/post-step --issue {N} --step "C5" --title "Pre-dispatch commit and push" --body "..."`

## C5.1. Receipt table validation (per chronic worklog-accuracy — 8+ consecutive reviews)

After the docs commit (step C5), validate that the worklog receipt table is complete within its defined scope:

```bash
bash tools/receipt-validate --cycle N --worklog docs/worklog/YYYY-MM-DD/HHMMSS-title.md
```

The receipt table scope is all commits through `cycle-complete`, excluding:
- The **docs commit** (`docs(cycle-N): ...`) — created at step C5, after the worklog is written
- The **record-dispatch commit** (`state(record-dispatch): ...`) — created at step C6

These are **structurally excluded** because they are created after the worklog is generated. Their absence is expected, not a defect.

If the tool reports genuinely missing receipts (receipts that should be in the table but aren't), fix the worklog before proceeding.

**Why:** The `worklog-accuracy` finding appeared in 8+ consecutive reviews because the receipt table was incomplete. The root cause is a temporal ordering constraint: the worklog cannot contain the SHA of the commit that writes it. This step validates completeness within the achievable scope, closing the chronic finding loop.

Post this step: `bash tools/post-step --issue {N} --step "C5.1" --title "Receipt validation" --body "..."`

## C5.5. Final pipeline gate (per audit [#153](https://github.com/EvaLok/schema-org-json-ld-audit/issues/153))

Re-run the pipeline-check after all state.json modifications are committed:

```bash
bash tools/pipeline-check
```

All 5 phases MUST pass before proceeding to the review dispatch. This catches regressions introduced by mid-cycle state modifications (e.g., format-changing tools like `derive-metrics --apply` that break downstream validators).

If the pipeline fails at this point:
1. **Fix the failure before closing the cycle.** Do not dispatch the review agent or close the cycle with a known pipeline regression.
2. If the failure requires a Copilot dispatch that cannot merge this cycle, **do not apply the format-changing tool output** — revert the state.json changes and defer the tool application to the next cycle when the consumer tool is ready.

**Coordination rule for format-changing tools:** When deploying a tool that changes state.json field formats, merge the downstream consumer tool update *before* applying the new tool's output. Sequence: (1) merge format-changing tool, (2) merge consumer tool update, (3) apply `--apply`. Never apply format changes without the consumer being ready.

Post this step: `bash tools/post-step --issue {N} --step "C5.5" --title "Final pipeline gate" --body "..."`

## C5.6. Stabilization counter update (per ADR 0011)

**Conditional step**: Only runs when `project_mode.mode` is `"stabilization"` in `docs/state.json`.

After the final pipeline gate (C5.5), evaluate whether this cycle qualifies as "clean":

1. **pipeline-check passed**: Step C5.5 returned exit 0 (all phases pass)
2. **No tool/infrastructure dispatches**: No PRs modifying Rust tool crates, shell wrappers, or pipeline tooling were dispatched OR merged during this cycle

If BOTH conditions are met:
- Increment `project_mode.clean_cycle_counter` by 1
- Append the current cycle number to `project_mode.consecutive_clean_cycles`

If EITHER condition fails:
- Reset `project_mode.clean_cycle_counter` to 0
- Clear `project_mode.consecutive_clean_cycles` to `[]`
- Log the reason for the reset in the journal

Update state.json via a targeted `jq` write or a future tool. Commit the state change as part of the C5 docs commit (or as a separate atomic commit).

**Target**: 50 consecutive clean cycles. When `clean_cycle_counter >= 50`, create a `question-for-eva` issue recommending transition out of stabilization mode.

Post this step: `bash tools/post-step --issue {N} --step "C5.6" --title "Stabilization counter" --body "..."`

## C6. Dispatch review agent (MANDATORY)

Dispatch a 5.4 agent to perform an **adversarial** end-of-cycle review. This is our primary quality control mechanism. The review agent's job is to find problems, not confirm that everything is fine.

### Stabilization observation mode (per ADR 0011)

**When `project_mode.mode` is `"stabilization"`**: The review agent still runs with full adversarial scope, but its findings are for observation only. Prepend the following block to the review issue body BEFORE the adversarial mandate:

> **OBSERVATION MODE (ADR 0011):** Log all findings in the standard structured format, but do NOT classify any finding as requiring immediate action. All findings are logged for post-stabilization triage. The orchestrator will NOT dispatch fixes or mark findings as actioned during stabilization. Your role this cycle is forensic documentation, not remediation.

In subsequent cycles, when processing findings from an observation-mode review at step 0.5, classify ALL findings as `deferred` with note "stabilization observation mode — logged for post-stabilization triage." Do not dispatch fixes, do not mark as actioned, do not trigger chronic category escalation. The findings accumulate for bulk triage when stabilization ends.

### Review prompt structure (per Eva directive [#725](https://github.com/EvaLok/schema-org-json-ld/issues/725))

The issue body for the review agent MUST be structured as follows:

1. **Lead with the adversarial mandate.** The very first paragraph must make clear that this is an adversarial review — the agent should actively look for problems, inconsistencies, drift, and complacency. It should not assume good faith or give the benefit of the doubt. Frame it as: "Your job is to find everything wrong with this cycle's work. Be thorough. Be skeptical. If something looks fine on the surface, dig deeper."

2. **Provide specific review targets.** List each area to examine with explicit instructions on what to look for:
   - **Code changes**: merged PRs, direct pushes — check for quality issues, test gaps, convention violations
   - **Worklog accuracy**: cross-reference the worklog's claims against actual commits, state.json, and issue activity. Does the narrative match reality?
   - **Journal quality**: is the journal entry genuine reflection or boilerplate? Does it contain actionable commitments with observable completion conditions?
   - **State.json integrity**: are metrics current? Do field inventory freshness markers match reality? Run spot-checks.
   - **Commit receipt verification**: verify receipt hashes with `git show <hash> --stat` — do committed changes match claims? **Receipt table scope**: the worklog receipt table covers all commits through `cycle-complete`. The docs commit (`docs(cycle-N): ...`) and record-dispatch commit (`state(record-dispatch): ...`) are **structurally excluded** — they are created after the worklog is written and cannot appear in their own table. This is an inherent temporal constraint, not a defect. Do NOT flag their absence as a worklog-accuracy issue. Instead, verify that all OTHER cycle receipts (cycle-start, process-merge, process-review, etc.) are present and correct.
   - **Infrastructure consistency**: are AGENTS.md, skills, checklists, and tools consistent with actual practice?
   - **Process adherence**: did the orchestrator follow its own checklist? Did it use tools when tools exist? Did it skip steps?
   - **Complacency detection**: are we going through the motions? Are findings being "noted" but not fixed? Are deferred items accumulating?

3. **Require structured output with an explicit format contract.** Each finding must follow this exact markdown template (per [audit #157](https://github.com/EvaLok/schema-org-json-ld-audit/issues/157)):

   ```markdown
   ## N. [category-name] Finding title

   **File**: path/to/file:line
   **Evidence**: what was observed
   **Recommendation**: concrete action
   ```

   The `[category-name]` tag MUST appear in the heading line inside square brackets. As a fallback, `process-review` also accepts a separate `Category: category-name` line within the finding body, but the inline `[category]` format is preferred. The complacency score (1-5) must be justified with evidence in a dedicated section.

   **Complacency scoring cap** (per [audit #198](https://github.com/EvaLok/schema-org-json-ld-audit/issues/198)): If the cycle overrode any FAIL or blocking-level pipeline gate (including `pipeline-check` or `state-invariants`), the maximum complacency score is **3/5** regardless of other factors. Gate overrides demonstrate that the orchestrator treated structural enforcement as optional — which is the definition of complacency, even if the override was "justified" at the time. Include this constraint verbatim in the review dispatch spec.

4. **Encourage depth over breadth.** Three deeply investigated findings with evidence are more valuable than ten surface-level observations.

### How to dispatch

**CRITICAL**: Copilot coding agents CANNOT post issue comments. They always create PRs. The review agent must commit its findings as a markdown file.

Write the issue body to a temp file, then create using `gh api --input`.

The issue body MUST instruct the agent to:
- **Commit findings as `docs/reviews/cycle-NNN.md`** — this is the only reliable output path
- NOT attempt to post issue comments (this will silently fail)
- The PR containing the review file is the deliverable

The issue body should include:
- The adversarial review mandate (first paragraph — see above)
- Current cycle number and issue link
- List of PRs merged this cycle (if any)
- Paths to this cycle's worklog and journal entries
- Specific areas of concern (if any)
- Output format: commit a structured markdown file at `docs/reviews/cycle-{N}.md`

Label the issue `agent-task` and `cycle-review`.

**Important**: The next cycle consumes review findings by reading the review file from the merged PR (or from the PR branch if not yet merged).

Post this step: `bash tools/post-step --issue {N} --step "C6" --title "Review dispatch" --body "..."`

## C7. Commit review dispatch state and push

After dispatching the review agent (step C6), `record-dispatch` has already committed the state change. Push immediately:

```bash
git push origin master
```

The `record-dispatch` commit is the LAST commit of the cycle. Its receipt hash appears in the closing comment (step C8) but NOT in the worklog receipt table (which was frozen in step C5). This is the expected split: the worklog captures all receipts up to and including the `cycle-complete` commit; the `record-dispatch` receipt is captured by the next cycle's `cycle-receipts` tool.

Post this step: `bash tools/post-step --issue {N} --step "C7" --title "Dispatch state push" --body "..."`

## C8. Close the orchestrator issue

Post a closing summary comment on the cycle issue and close it.

The summary should include:
- What was accomplished
- Pipeline status
- Review agent issue number (from step C6)
- Commit receipts (from step C7)
- Next cycle priorities

Post this step: `bash tools/post-step --issue {N} --step "C8" --title "Cycle close-out" --body "..."`

## Phase transitions

The cycle state machine is: `work -> close_out -> complete`.

- `cycle-complete --apply` transitions from `work` to `close_out`
- `record-dispatch` transitions from `close_out` to `complete`

No manual `cycle-phase` calls are needed for the standard flow (since cycle 226).

## Automation status

| Step | Status | Tool |
|------|--------|------|
| C1. Pipeline verification (early) | Automated | `bash tools/pipeline-check` |
| C2. State.json updates | Automated | `process-merge`, `process-review`, `process-audit`, `process-eva`, `cycle-complete`, `record-dispatch` |
| C3. Worklog + Journal | Semi-automated | `write-entry worklog` + `write-entry journal` (orchestrator provides structured input) |
| C4.1. Validate docs | Automated | `validate-docs worklog` + `validate-docs journal` (blocking gate) |
| C5. Commit worklog/journal/state | Manual | git commit + push (BEFORE review dispatch) |
| C5.1. Receipt table validation | Automated | `bash tools/receipt-validate` (validates receipt table scope completeness) |
| C5.5. Final pipeline gate | Automated | `bash tools/pipeline-check` (re-run after all modifications) |
| C6. Review agent dispatch | Semi-automated | `cycle-complete` generates issue body, orchestrator creates issue |
| C7. Commit dispatch state | Automated | `record-dispatch` commits, then push |
| C8. Close issue | Manual | Standard gh commands |
| Phase transition to complete (not a post-step ID) | Automated | `cycle-complete` → close_out, `record-dispatch` → complete (auto, since cycle 226) |
