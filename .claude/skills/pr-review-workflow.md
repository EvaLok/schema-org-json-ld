# Skill: PR Review Workflow

Step-by-step procedure for reviewing PRs created by the Copilot coding agent.

## Overview

CI workflows (tests, linting) only run on PRs that are **ready for review** — not on draft PRs. The Copilot agent creates PRs in draft state. The orchestrator must follow this sequence:

1. Wait for the agent to finish work
2. Mark the PR as ready for review (triggers CI)
3. Wait for CI workflows to complete
4. Review the code and CI results
5. Merge or request revisions

## Step 1: Wait for agent to finish

Check for `copilot_work_finished` in the PR timeline:

```bash
gh api repos/EvaLok/schema-org-json-ld/issues/{PR}/timeline --paginate \
  --jq '[.[] | select(.event | test("copilot_work")) | {event: .event, created_at: .created_at}]' \
  | tail -5
```

Or use the tool: `tools/agent-status <PR_NUMBER>`

**Do not proceed until `copilot_work_finished` appears.** The agent pushes multiple commits during a session — reviewing mid-session wastes effort.

## Step 2: Mark PR as ready for review

Once the agent finishes, remove the draft status:

```bash
gh pr ready <PR_NUMBER> --repo EvaLok/schema-org-json-ld
```

This triggers CI workflows (test-unit, cs-check, etc.) on the PR branch.

## Step 3: Wait for CI workflows

Check CI status:

```bash
gh pr checks <PR_NUMBER> --repo EvaLok/schema-org-json-ld
```

Wait for all checks to complete. Expected checks:
- **Test and Build** — runs `composer run test-unit`
- **Lint** — runs `composer run cs-check`

If checks fail, review the failure before proceeding. Common failures:
- Test failures → the agent's code has bugs
- CS-check failures → the agent didn't run `composer run cs-fix`

## Step 4: Review the code

Fetch the PR diff:

```bash
gh pr diff <PR_NUMBER> --repo EvaLok/schema-org-json-ld
```

Review checklist:
- [ ] Correct `@type` value in `A_SCHEMA_TYPE`
- [ ] Required properties match Google Rich Results docs
- [ ] Constructor uses promotion for all properties
- [ ] `null|Type` syntax (not `?Type`)
- [ ] Required params first, optional params (with `= null`) last
- [ ] Array properties have `/** @var Type[] */` doc comments
- [ ] No `toArray()` or other serialization methods
- [ ] Tests cover: minimal output, null omission, full output, nested schemas
- [ ] Enums use backed string values with schema.org URLs
- [ ] No modifications to `JsonLdGenerator.php` or `TypedSchema.php`

## Step 5: Merge or request revisions

**If everything passes:**

```bash
gh pr merge <PR_NUMBER> --repo EvaLok/schema-org-json-ld --squash --delete-branch
```

**If revisions are needed:**

Batch all feedback into a single `@copilot` comment (each comment costs 1 premium request):

```bash
gh pr comment <PR_NUMBER> --repo EvaLok/schema-org-json-ld \
  --body "@copilot Please fix the following:
1. ...
2. ...
3. ..."
```

Then go back to Step 1 — wait for the agent to finish the revision before re-reviewing.

## Using the review-pr tool

The `tools/review-pr` script automates Steps 1-3:

```bash
tools/review-pr <PR_NUMBER>           # Check status, mark ready if agent finished
tools/review-pr <PR_NUMBER> --merge   # Also merge if CI passes
tools/review-pr --help
```
