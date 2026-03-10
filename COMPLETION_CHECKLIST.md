# Cycle Completion Checklist

Follow this checklist at the end of every orchestrator cycle. Do not skip steps.

**Step-level commenting**: As with the startup checklist, every step must be posted as a **separate comment** on the orchestrator run issue using `bash tools/post-step`. Post each step's outcome as you complete it — do not batch or summarize from memory.

**Critical**: The review agent dispatch (step 6) is MANDATORY. Every cycle must end with a review agent in-flight. Eva directive [#463](https://github.com/EvaLok/schema-org-json-ld/issues/463).

## 1. Run pipeline verification (early check)

Run the pipeline-check early to catch issues before they compound:

```bash
bash tools/pipeline-check
```

If not yet run, run it now. `pipeline-check` derives the current cycle from `docs/state.json`; use `--cycle {N}` only when you need to override it.

**Note**: This is an early check, not the final gate. State-modifying tools in step 2 may change state.json after this check passes. The final pipeline gate is step 5.5, which re-runs after all modifications are committed.

## 2. Update state.json via write-side tools

**Do NOT manually edit `docs/state.json`.** Use the write-side pipeline tools instead. Each tool atomically updates its owned section of state.json, bumps freshness markers, and commits with a receipt hash.

### During the cycle (as events occur)

| Event | Tool | Command |
|-------|------|---------|
| PR merged | `process-merge` | `bash tools/process-merge --prs 123,456 --issues 789,790` (`--issues none` when there are intentionally no linked issues) |
| Review findings consumed | `process-review` | `bash tools/process-review --review-file docs/reviews/cycle-N.md --actioned A --deferred D --ignored I` |
| Audit recommendation processed | `process-audit` | `bash tools/process-audit --audit-id N --action accepted` |
| Eva directive processed | `process-eva` | `bash tools/process-eva --closed 123,456 --remaining-open 247,436` |

### At cycle end

| Step | Tool | Command |
|------|------|---------|
| Update `last_cycle` fields | `cycle-complete` | `bash tools/cycle-complete --apply --issue N --summary "..."` |
| Record review agent dispatch | `record-dispatch` | `bash tools/record-dispatch --issue N --title "Cycle N review" --model gpt-5.4` |

**Important**: Run `cycle-complete` BEFORE dispatching the review agent, so `last_cycle.number` is updated before `record-dispatch` reads it.

Each tool handles its own freshness markers automatically — no manual freshness reconciliation needed.

## 3. Write worklog entry

Use `write-entry` to generate the worklog. The primary interface is the inline CLI flags:

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

For reviewed PRs, processed issues, and self-modifications, you can stay on the inline CLI path:

```bash
bash tools/write-entry worklog \
  --title "Cycle N summary" \
  --pr-reviewed 789 \
  --issue-processed "Closed #924 (cycle review)" \
  --self-modification "Updated AGENTS.md"
```

For richer structured payloads, write JSON to a file with the Write tool and pass `--input-file`:

```bash
bash tools/write-entry worklog --title "Cycle N summary" --input-file /tmp/worklog.json
```

The tool auto-generates clickable GitHub links from bare `#N` references, creates the directory structure, and derives the cycle number from state.json.

**Commit receipts**: Also run `bash tools/cycle-receipts --cycle N` and include the full receipt table in the worklog. Every write-side tool receipt must be present — this is the cycle's audit trail. Do not manually assemble receipts from memory; use the tool.

**Note on review dispatch receipt**: The worklog is committed in step 5, BEFORE the review agent is dispatched in step 6. This means the worklog receipt table will NOT include the `record-dispatch` receipt for the review agent — that receipt is generated after the worklog is frozen. This is by design: the review agent must see the committed worklog at dispatch time (to avoid the artifact-race false positive from audit #151). The review dispatch receipt appears in `record-dispatch`'s own commit (step 7) and is captured by the next cycle's `cycle-receipts` run. Do NOT attempt to add the review dispatch receipt to the worklog after dispatch — that would create the exact drift this ordering prevents.

## 4. Write journal entry

Use `write-entry` to generate the journal entry. The primary interface is the inline CLI flags:

```bash
bash tools/write-entry journal \
  --title "Cycle N reflections" \
  --section "Decision::Chose to defer #829" \
  --commitment "Will dispatch #830 next cycle"
```

For more complex payloads, write JSON to a file with the Write tool and pass `--input-file`:

```bash
bash tools/write-entry journal --title "Cycle N reflections" --input-file /tmp/journal.json
```

The tool appends to `docs/journal/YYYY-MM-DD.md`, handles JOURNAL.md index updates when a new date file is created, auto-links bare `#N` references, and automatically inserts the matching worklog link when the cycle worklog already exists.

Every journal entry **must** include a link to the corresponding worklog entry:

```markdown
Worklog: [cycle N](docs/worklog/YYYY-MM-DD/HHMMSS-cycle-name.md)
```

The worklog is the factual record (what happened, receipts, state). The journal is the reflection (why, lessons, questions). They must be cross-referenced so either can be traced to the other.

## 4.5. ADR check (per Eva directive [#724](https://github.com/EvaLok/schema-org-json-ld/issues/724))

Before closing the cycle, check whether any decisions made this cycle warrant an Architecture Decision Record in `doc/adr/`.

ADR-worthy decisions include:
- New architectural patterns or significant refactors
- Technology choices (new languages, tools, frameworks)
- Process changes that affect how the system operates
- Design trade-offs where alternatives were considered and rejected

If a decision was made this cycle that meets these criteria, write the ADR now. Use the format established in existing ADRs: Status, Context, Decision, Consequences, Alternatives Considered.

Number ADRs sequentially (check `ls doc/adr/` for the next number).

## 5. Commit worklog, journal, and state before review dispatch

**CRITICAL ORDERING**: The review agent reads the repo at dispatch time. If worklog and journal entries are not committed and pushed before the review dispatch, the review agent will flag them as "missing" — a false positive that has contaminated complacency scores since cycle 189. Per audit [#151](https://github.com/EvaLok/schema-org-json-ld-audit/issues/151).

Before dispatching the review agent:
1. Commit all worklog and journal entries
2. Commit any state.json changes from `cycle-complete`
3. Push to master
4. **Verify** the push succeeded before proceeding to step 6

This ensures the review agent sees the complete cycle state, eliminating the artifact-race false positive.

## 5.5. Final pipeline gate (per audit [#153](https://github.com/EvaLok/schema-org-json-ld-audit/issues/153))

Re-run the pipeline-check after all state.json modifications are committed:

```bash
bash tools/pipeline-check
```

All 5 phases MUST pass before proceeding to the review dispatch. This catches regressions introduced by mid-cycle state modifications (e.g., format-changing tools like `derive-metrics --apply` that break downstream validators).

If the pipeline fails at this point:
1. **Fix the failure before closing the cycle.** Do not dispatch the review agent or close the cycle with a known pipeline regression.
2. If the failure requires a Copilot dispatch that cannot merge this cycle, **do not apply the format-changing tool output** — revert the state.json changes and defer the tool application to the next cycle when the consumer tool is ready.

**Coordination rule for format-changing tools:** When deploying a tool that changes state.json field formats, merge the downstream consumer tool update *before* applying the new tool's output. Sequence: (1) merge format-changing tool, (2) merge consumer tool update, (3) apply `--apply`. Never apply format changes without the consumer being ready.

## 6. Dispatch review agent (MANDATORY)

Dispatch a 5.4 agent to perform an **adversarial** end-of-cycle review. This is our primary quality control mechanism. The review agent's job is to find problems, not confirm that everything is fine.

### Review prompt structure (per Eva directive [#725](https://github.com/EvaLok/schema-org-json-ld/issues/725))

The issue body for the review agent MUST be structured as follows:

1. **Lead with the adversarial mandate.** The very first paragraph must make clear that this is an adversarial review — the agent should actively look for problems, inconsistencies, drift, and complacency. It should not assume good faith or give the benefit of the doubt. Frame it as: "Your job is to find everything wrong with this cycle's work. Be thorough. Be skeptical. If something looks fine on the surface, dig deeper."

2. **Provide specific review targets.** List each area to examine with explicit instructions on what to look for:
   - **Code changes**: merged PRs, direct pushes — check for quality issues, test gaps, convention violations
   - **Worklog accuracy**: cross-reference the worklog's claims against actual commits, state.json, and issue activity. Does the narrative match reality?
   - **Journal quality**: is the journal entry genuine reflection or boilerplate? Does it contain actionable commitments with observable completion conditions?
   - **State.json integrity**: are metrics current? Do field inventory freshness markers match reality? Run spot-checks.
   - **Commit receipt verification**: verify receipt hashes with `git show <hash> --stat` — do committed changes match claims?
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

## 7. Commit review dispatch state and push

After dispatching the review agent (step 6), `record-dispatch` has already committed the state change. Push immediately:

```bash
git push origin master
```

The `record-dispatch` commit is the LAST commit of the cycle. Its receipt hash appears in the closing comment (step 8) but NOT in the worklog receipt table (which was frozen in step 5). This is the expected split: the worklog captures all receipts up to and including the `cycle-complete` commit; the `record-dispatch` receipt is captured by the next cycle's `cycle-receipts` tool.

## 8. Close the orchestrator issue

Post a closing summary comment on the cycle issue and close it.

The summary should include:
- What was accomplished
- Pipeline status
- Review agent issue number (from step 6)
- Commit receipts (from step 7)
- Next cycle priorities

## Automation status

| Step | Status | Tool |
|------|--------|------|
| 1. Pipeline verification (early) | Automated | `bash tools/pipeline-check` |
| 2. State.json updates | Automated | `process-merge`, `process-review`, `process-audit`, `process-eva`, `cycle-complete`, `record-dispatch` |
| 3. Worklog entry | Semi-automated | `write-entry worklog` (orchestrator provides structured input) |
| 4. Journal entry | Semi-automated | `write-entry journal` (orchestrator provides structured input, auto-updates JOURNAL.md index) |
| 5. Commit worklog/journal/state | Manual | git commit + push (BEFORE review dispatch) |
| 5.5. Final pipeline gate | Automated | `bash tools/pipeline-check` (re-run after all modifications) |
| 6. Review agent dispatch | Semi-automated | `cycle-complete` generates issue body, orchestrator creates issue |
| 7. Commit dispatch state | Automated | `record-dispatch` commits, then push |
| 8. Close issue | Manual | Standard gh commands |
