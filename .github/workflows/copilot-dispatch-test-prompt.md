# Copilot Dispatch Mechanism Test

You are validating the Copilot dispatch mechanism from a GitHub Actions runner. This is an isolated mechanism test — not a research dispatch, not an orchestrator cycle. Do nothing beyond the steps below.

Repository: `EvaLok/schema-org-json-ld`

## Procedure

### Step 1 — Create the test issue

If the `dispatch-test` label does not exist, create it first:

    gh label create dispatch-test --color C5DEF5 --description "Copilot dispatch mechanism test issue" 2>/dev/null || true

Then create the issue:

    TIMESTAMP=$(date -u +%Y%m%dT%H%M%SZ)
    TEST_ISSUE_URL=$(gh issue create \
      --repo EvaLok/schema-org-json-ld \
      --title "[dispatch-test] ${TIMESTAMP} Copilot dispatch mechanism verification" \
      --label "agent-task" \
      --label "dispatch-test" \
      --body "Mechanism test issue. Post one comment containing the literal string DISPATCH-TEST-OK. Do not modify any files. Do not open a pull request.")
    TEST_ISSUE_NUMBER=$(echo "${TEST_ISSUE_URL}" | grep -oP '/issues/\K[0-9]+')
    echo "Created test issue #${TEST_ISSUE_NUMBER}: ${TEST_ISSUE_URL}"

### Step 2 — Fetch the issue node ID

    ISSUE_NODE_ID=$(gh api graphql \
      -f query='query($n: Int!) { repository(owner: "EvaLok", name: "schema-org-json-ld") { issue(number: $n) { id } } }' \
      -F n=${TEST_ISSUE_NUMBER} \
      --jq '.data.repository.issue.id')
    echo "Issue node ID: ${ISSUE_NODE_ID}"

### Step 3 — Assign Copilot via GraphQL

REST cannot assign bot logins; the only working path is the `replaceActorsForAssignable` GraphQL mutation.

Stable actor IDs for this repo:
- `BOT_kgDOC9w8XQ` — `copilot-swe-agent`
- `U_kgDOALttcg` — `EvaLok`

    gh api graphql \
      -f query='mutation($id: ID!) { replaceActorsForAssignable(input: {assignableId: $id, actorIds: ["BOT_kgDOC9w8XQ", "U_kgDOALttcg"]}) { assignable { ... on Issue { number assignees(first: 5) { nodes { login } } } } } }' \
      -F id="${ISSUE_NODE_ID}"

If the mutation errors, stop and report the error verbatim.

### Step 4 — Poll for Copilot connection (up to 60s)

    for i in $(seq 1 30); do
      CONNECTED=$(gh api repos/EvaLok/schema-org-json-ld/issues/${TEST_ISSUE_NUMBER}/events \
        --jq '[.[] | select(.actor.login == "Copilot" and .event == "connected")] | length')
      if [ "${CONNECTED}" -gt 0 ]; then break; fi
      sleep 2
    done

### Step 5 — Report

Fetch all Copilot events on the issue:

    gh api repos/EvaLok/schema-org-json-ld/issues/${TEST_ISSUE_NUMBER}/events \
      --jq '.[] | select(.actor.login == "Copilot") | {event, created_at}'

Then post a comment on the test issue with one of these exact outcomes:

- **SUCCESS** if both `assigned` and `connected` events from `Copilot` are present:
  `DISPATCH MECHANISM VERIFIED FROM CI — assigned <ts>, connected <ts>`
- **PARTIAL** if only `assigned` is present:
  `DISPATCH MECHANISM PARTIAL — Copilot assigned at <ts> but did not connect within 60s. Investigate rate limit or auth scope.`
- **FAILURE** if neither event is present:
  `DISPATCH MECHANISM FAILED — neither assigned nor connected event from Copilot. Mutation response: <verbatim>. Recent events: <verbatim>.`

Use `gh issue comment ${TEST_ISSUE_NUMBER} --body "..."`.

End your final assistant message with the issue number and outcome label so Eva can find it.

## Constraints

- Do not close the test issue. Eva closes it manually after review.
- Do not modify any files in the repository.
- Do not run any tools beyond `gh`, `jq`, `date`, `sleep`, `cat`, `echo`, `grep`, `seq`.
- If any step errors, stop immediately, do not retry, and report the error.
