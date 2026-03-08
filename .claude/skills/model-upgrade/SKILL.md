---
name: model-upgrade
description: Checklist for changing the default Copilot coding agent model. Use when upgrading from one model to another (e.g. gpt-5.3-codex to gpt-5.4) to ensure all locations are updated consistently.
user-invocable: false
---

# Changing the Default Copilot Agent Model

When upgrading the default model used by the Copilot coding agent, multiple locations must be updated in sync. Missing any one of these causes the orchestrator to silently dispatch with the old model.

## Locations to update

### 1. System prompt — dispatch examples

File: `.github/workflows/orchestrator-prompt.md`

The `agent_assignment` JSON blocks in the "How to dispatch agent tasks" section contain a `"model"` field. Update both examples (create-and-assign, assign-existing).

Search for: `"model": "<old-model>"`

### 2. System prompt — model selection table

Same file. The table under "Model selection" lists available models with descriptions. Update the table rows and the prose below it (`Prefer <model> for routine work...`).

### 3. System prompt — operating principles

Same file. Operating principle #6 references the preferred model by name. Update it.

### 4. `record-dispatch` tool — CLI default

File: `tools/rust/crates/record-dispatch/src/main.rs`

The `--model` CLI argument has a `default_value`. This is the most critical location — when the orchestrator calls `bash tools/record-dispatch` without `--model`, it gets whatever default is hardcoded here.

Search for: `#[arg(long, default_value = `

### 5. `record-dispatch` tool — test fixtures

Same file, `#[cfg(test)]` section. Test fixtures use model strings in `sample_state()` and in `build_dispatch_patch()` calls. Update these to match the new default for consistency.

### 6. `process-merge` tool — test fixtures

File: `tools/rust/crates/process-merge/src/main.rs`

The `sample_state()` function in the test module contains `"model"` fields in agent session objects. Update for consistency.

### 7. Verify after changes

```bash
cd tools/rust && cargo test --quiet
```

All tests must pass after the update.

## Locations to NOT update

- **`backfill-sessions`** — This tool reconstructs historical session data. Its test fixtures reflect actual past dispatches and should retain the model that was used at the time.
- **`docs/state.json`** — Historical `agent_sessions` entries record the model that was actually used. Do not retroactively change them. Only new dispatches will use the new model.
- **Worklog/journal entries** — These are historical records. Never alter them.

## Quick grep to find all references

```bash
grep -rn '"<old-model>"' tools/rust/crates/ .github/workflows/orchestrator-prompt.md
```

## Common mistake

Updating only the system prompt but not the `record-dispatch` default. The system prompt tells the orchestrator what model to prefer, but if `record-dispatch --model` has a stale default, the orchestrator may omit `--model` from the CLI call and silently use the old model.
