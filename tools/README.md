# Orchestrator Tools

Shell scripts for the orchestrator to standardise repetitive operations.

## `agent-status`

Check the status of Copilot agent sessions.

```bash
tools/agent-status              # Overview: all in-flight sessions + concurrency
tools/agent-status <PR_NUMBER>  # Detailed status for a specific PR
tools/agent-status --help       # Help
```

## `update-state`

Atomically update `docs/state.json` and create worklog entries.

```bash
tools/update-state --read                               # Print current state
tools/update-state --add-implemented Video 45 44         # Add type to implemented
tools/update-state --add-in-progress Video 44            # Mark type as in-progress
tools/update-state --clear-in-progress Video             # Clear in-progress
tools/update-state --add-agent-session 44 gpt-5.3-codex "Video"
tools/update-state --remove-agent-session 44             # After merge
tools/update-state --set-cycle 43 "Cycle 6 summary"     # Update last_cycle
tools/update-state --set-field test_count 75             # Set any field
tools/update-state --commit "Update state after merge"   # Commit + push
tools/update-state --help                                # Help
```

## `dispatch-agent`

Create a GitHub issue and assign the Copilot coding agent.

```bash
tools/dispatch-agent --title "Add Video schema" --body-file /tmp/video-spec.md
tools/dispatch-agent --title "Complex task" --body-file spec.md --model claude-opus-4.5
tools/dispatch-agent --help
```

## `review-pr`

PR review workflow helper. Checks agent status, marks PR ready for review (triggering CI), waits for CI, and optionally merges.

```bash
tools/review-pr <PR_NUMBER>             # Check status, mark ready if agent finished
tools/review-pr <PR_NUMBER> --wait-ci   # Also wait for CI to complete
tools/review-pr <PR_NUMBER> --merge     # Wait for CI and squash-merge if it passes
tools/review-pr --help
```

Implements the full review sequence: agent finished? → mark ready → CI passes? → merge.
