# Skill: Orchestrator Permission Model

## Allowed commands

The orchestrator workflow only permits these Bash command prefixes:

| Prefix | Examples |
|--------|----------|
| `gh *` | `gh api ...`, `gh issue list ...`, `gh pr view ...` |
| `git *` | `git add`, `git commit`, `git push`, `git branch` |
| `jq *` | `jq '.field' file.json`, `jq -n --arg ...` |
| `mkdir *` | `mkdir -p docs/worklog/2026-02-25` |
| `ls *` | `ls tools/`, `ls -la file` |
| `date *` | `date -u '+%Y-%m-%d'` |
| `wc *` | `wc -l file` |
| `sort *` | `sort file` |
| `composer *` | `composer run test-unit` |

WebFetch is allowed for `schema.org`, `developers.google.com`, `search.google.com` domains only.

## NOT allowed (will require user approval and block)

- `bash`, `sh` — cannot run scripts directly
- `echo`, `printf` — cannot produce text output
- `cat`, `head`, `tail` — cannot read files (use Read tool instead)
- `grep`, `rg` — cannot search (use Grep tool instead)
- `chmod` — cannot change permissions
- `env`, `printenv` — cannot inspect environment
- `curl`, `wget` — cannot make HTTP requests (use `gh api` or WebFetch)
- Any other command not in the allow list

## Workarounds

### Posting comments on issues
Instead of `gh issue comment`, use `gh api` with jq-encoded body:
```bash
gh api "repos/EvaLok/schema-org-json-ld/issues/NUMBER/comments" -X POST \
  --input <(jq -n --arg body "Comment text" '{"body": $body}')
```

**IMPORTANT**: The `--input <(jq ...)` pattern uses process substitution which requires `bash`. This currently doesn't work either. Use the simpler form:
```bash
gh api "repos/EvaLok/schema-org-json-ld/issues/NUMBER/comments" -X POST -f body="Comment text"
```

### Reading environment variables
Use `tools/session-info` (when `bash` is allowed) or output session info via `git log` / `date` commands.

### Creating issues with multi-line bodies
Write the body to a file with the Write tool, then use `gh api` with `-F` (capital F) to read file content:
```bash
gh api "repos/REPO/issues" -X POST -f title="Title" -F body=@docs/my-body.md -f "labels[]=agent-task"
```
**IMPORTANT**: The file must be within the repo working directory (not `/tmp/`). The `-F key=@file` syntax reads the file as the field value. Use `-f` (lowercase) for inline string values and `-F` (uppercase) for file-based values.

**Avoid**: `jq --rawfile` and output redirection (`>`) — both are blocked by the security sandbox. Also avoid files in `/tmp/` as jq cannot read from there.

### Running shell scripts
Currently NOT possible. All operations must use allowed commands directly.
When `Bash(bash tools/*)` is added to the workflow, use: `bash tools/scriptname args`

## Proposed workflow additions
These have been proposed to Eva via PR with label `workflow-change`:
- `Bash(bash tools/*)` — run orchestrator tools
- `Bash(echo *)` — basic text output
- `Bash(cat *)` — pipe file content
- `Bash(chmod *)` — set file permissions
