# Orchestrator Tools

Shell scripts for the orchestrator to standardise repetitive operations.

**Note**: These tools require `bash` in the workflow's allowed commands list. Until `Bash(bash tools/*)` is added to the orchestrator workflow permissions, these scripts serve as documented procedures. Use the `gh` / `git` / `jq` commands from the STARTUP_CHECKLIST directly instead.

## `agent-status`

Check the status of Copilot agent sessions.

```bash
bash tools/agent-status              # Overview: all in-flight sessions + concurrency
bash tools/agent-status <PR_NUMBER>  # Detailed status for a specific PR
bash tools/agent-status --help       # Help
```

## `update-state`

Atomically update `docs/state.json` and create worklog entries.

```bash
bash tools/update-state --read                               # Print current state
bash tools/update-state --add-implemented Video 45 44         # Add type to implemented
bash tools/update-state --add-in-progress Video 44            # Mark type as in-progress
bash tools/update-state --clear-in-progress Video             # Clear in-progress
bash tools/update-state --add-agent-session 44 gpt-5.3-codex "Video"
bash tools/update-state --remove-agent-session 44             # After merge
bash tools/update-state --set-cycle 43 "Cycle 6 summary"     # Update last_cycle
bash tools/update-state --set-field test_count 75             # Set any field
bash tools/update-state --commit "Update state after merge"   # Commit + push
bash tools/update-state --help                                # Help
```

## `dispatch-agent`

Create a GitHub issue and assign the Copilot coding agent.

```bash
bash tools/dispatch-agent --title "Add Video schema" --body-file /tmp/video-spec.md
bash tools/dispatch-agent --title "Complex task" --body-file spec.md --model gpt-5.3-codex
bash tools/dispatch-agent --help
```

## `review-pr`

PR review workflow helper. Checks agent status, marks PR ready for review (triggering CI), waits for CI, and optionally merges.

```bash
bash tools/review-pr <PR_NUMBER>             # Check status, mark ready if agent finished
bash tools/review-pr <PR_NUMBER> --wait-ci   # Also wait for CI to complete
bash tools/review-pr <PR_NUMBER> --merge     # Wait for CI and squash-merge if it passes
bash tools/review-pr --help
```

## `comment-issue`

Post a comment on a GitHub issue or PR. Wraps `gh api` with proper JSON encoding.

```bash
bash tools/comment-issue <NUMBER> "Comment text"
bash tools/comment-issue <NUMBER> --file /tmp/comment.md
bash tools/comment-issue --help
```

## `session-info`

Gather and display session metadata (run ID, timestamp, model).

```bash
bash tools/session-info              # Print all session info
bash tools/session-info --json       # Output as JSON
bash tools/session-info --run-id     # Print just the run ID
bash tools/session-info --help
```

## `post-opening`

Post the formatted opening comment for an orchestrator session.

```bash
bash tools/post-opening <ISSUE_NUMBER>
bash tools/post-opening --help
```

## `qc-check`

Check for QC reports from the QC orchestrator and manage QC communication.

```bash
bash tools/qc-check                           # Check for new QC reports
bash tools/qc-check --outbound                # List our open QC requests
bash tools/qc-check --inbound                 # List our QC acknowledgements
bash tools/qc-check --request "Title" "Body"  # Create a validation request
bash tools/qc-check --ack 8 "Description"     # Acknowledge a QC report
bash tools/qc-check --help
```

## `create-issue`

Create a GitHub issue with proper JSON encoding and labels.

```bash
bash tools/create-issue --title "Title" --body "Body text" --label "agent-task"
bash tools/create-issue --title "Title" --body-file /tmp/spec.md --label "qc-inbound"
bash tools/create-issue --help
```
