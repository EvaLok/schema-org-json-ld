---
name: orchestrator-prompt-editing
description: How to modify the orchestrator's system prompt. Use when behavioral issues are identified in journal/review entries, or when the orchestrator's instructions need updating. Requires human oversight — the orchestrator cannot modify its own prompt.
user-invocable: false
---

# Editing the Orchestrator System Prompt

The orchestrator's system prompt lives at `.github/workflows/orchestrator-prompt.md`. This file is loaded as the orchestrator's briefing every cycle. Changes here directly affect orchestrator behavior.

## Key facts

- **Location**: `.github/workflows/orchestrator-prompt.md` — NOT the workflow YAML (`.github/workflows/orchestrator-cron.yml`)
- **The workflow YAML** only creates a trigger issue with a short body (`@claude Run your orchestrator cycle. Follow STARTUP_CHECKLIST.md.`). The prompt file is the actual system prompt.
- **Human-only change**: The orchestrator cannot modify this file itself (it's under `.github/workflows/`, which requires Eva to merge). If the orchestrator identifies a need to change its own prompt, it must create a `question-for-eva` issue describing the proposed change and rationale.

## When to edit

Edit the prompt when journal entries or review findings reveal **recurring behavioral patterns** that checklists and tools haven't fixed. Indicators:

- The same class of finding appears across 3+ consecutive review cycles
- The orchestrator acknowledges a pattern in its journal but doesn't change behavior
- Tools exist but aren't being used (adoption gaps)
- The orchestrator interprets or filters data instead of reporting canonical values

## How to edit

1. **Read the journal entries** that document the behavioral issue — identify the specific recurring pattern
2. **Read the current prompt** at `.github/workflows/orchestrator-prompt.md`
3. **Edit the prompt file** — add directives in the appropriate section (see structure below)
4. **Do NOT edit the workflow YAML** (`.github/workflows/orchestrator-cron.yml`) — the prompt content belongs in the `.md` file
5. **Commit and push to master** — Eva will need to merge if it goes through a PR, or it can be pushed directly if the human is doing it

## Prompt structure

The prompt is organized in this order:

1. **Identity and role** — who the orchestrator is
2. **Priorities** — primary (workflow quality) and secondary (schema implementations)
3. **Behavioral directives** — non-negotiable constraints on behavior
4. **Tool-first philosophy** — principles for automation
5. **Environment** — runtime context, repos, communication
6. **Coding agent** — how to dispatch and review work
7. **Operating principles** — general guidelines

**Behavioral directives** (section 3) is the right place for constraints that address recurring failures. Each directive should:

- Name the specific behavior being constrained
- State the rule clearly and unconditionally
- Reference the historical pattern that motivated it (so future readers understand why)
- Be framed as a hard constraint, not a suggestion

## What NOT to do

- **Do not edit the workflow YAML to embed prompt content** — the YAML creates the trigger issue; the `.md` file is the prompt
- **Do not let the orchestrator self-modify its prompt** — this requires human oversight. The orchestrator can propose changes via `question-for-eva` issues, but a human must review and apply them
- **Do not add procedural steps here** — those belong in `STARTUP_CHECKLIST.md` or `COMPLETION_CHECKLIST.md`. The prompt is for identity, priorities, philosophy, and behavioral constraints
- **Do not duplicate checklist content** — if a rule is already enforced by a checklist step or a tool, it doesn't need to be in the prompt. The prompt is for behavioral patterns that checklists and tools haven't fixed
