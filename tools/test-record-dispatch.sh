#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SCRIPT_UNDER_TEST="$SCRIPT_DIR/record-dispatch"
DERIVE_METRICS_SCRIPT="$SCRIPT_DIR/derive-metrics"

fail() {
	echo "FAIL: $1" >&2
	exit 1
}

create_repo() {
	local repo_root="$1"
	local status_one="$2"
	local status_two="$3"

	mkdir -p "$repo_root/docs"
	cd "$repo_root"

	git init -q
	git config user.name "Test User"
	git config user.email "test@example.com"

	cat <<EOF > docs/state.json
{
  "agent_sessions": [
    {
      "issue": 600,
      "title": "Merged change",
      "dispatched_at": "2026-03-01T00:00:00Z",
      "model": "gpt-5.4",
      "status": "$status_one",
      "pr": 700,
      "merged_at": "2026-03-02T00:00:00Z"
    },
    {
      "issue": 601,
      "title": "Closed change",
      "dispatched_at": "2026-03-03T00:00:00Z",
      "model": "gpt-5.4",
      "status": "$status_two"
    }
  ],
  "last_cycle": {
    "number": 164
  },
  "copilot_metrics": {
    "total_dispatches": 2,
    "resolved": 2,
    "merged": 1,
    "closed_without_pr": 1,
    "reviewed_awaiting_eva": 0,
    "in_flight": 0,
    "produced_pr": 1,
    "pr_merge_rate": "100.0%",
    "dispatch_to_pr_rate": "50.0%",
    "dispatch_log_latest": "#601 Closed change (cycle 164)"
  },
  "field_inventory": {
    "fields": {
      "copilot_metrics.in_flight": {
        "last_refreshed": "cycle 163"
      }
    }
  }
}
EOF

	git add docs/state.json
	git commit -q -m "initial state"
}

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

# Test 1: Successful dispatch re-derives metrics in the updated state file.
SUCCESS_REPO="$TMP_DIR/success-repo"
create_repo "$SUCCESS_REPO" "merged" "closed_without_pr"

SUCCESS_STDOUT="$TMP_DIR/success.stdout"
SUCCESS_STDERR="$TMP_DIR/success.stderr"
if bash "$SCRIPT_UNDER_TEST" --repo-root "$SUCCESS_REPO" --issue 602 --title "Example dispatch" --model "gpt-5.4" >"$SUCCESS_STDOUT" 2>"$SUCCESS_STDERR"; then
	SUCCESS_STATUS=0
else
	SUCCESS_STATUS=$?
fi

[ "$SUCCESS_STATUS" -eq 0 ] || fail "expected success exit code 0, got $SUCCESS_STATUS"
grep -Eq 'receipt: [0-9a-f]{7,40}' "$SUCCESS_STDOUT" || fail "expected receipt hash in stdout"
grep -Fq 'Dispatch recorded: #602 "Example dispatch" (model: gpt-5.4).' "$SUCCESS_STDOUT" || fail "expected dispatch summary in stdout"
if grep -Eq '(^Error:|unsupported value)' "$SUCCESS_STDERR"; then
	fail "expected no error output on successful dispatch"
fi

python - <<'PY' "$SUCCESS_REPO/docs/state.json"
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    state = json.load(handle)

metrics = state["copilot_metrics"]
sessions = state["agent_sessions"]

assert len(sessions) == 3, sessions
assert metrics["total_dispatches"] == 3, metrics
assert metrics["resolved"] == 2, metrics
assert metrics["in_flight"] == 1, metrics
assert metrics["produced_pr"] == 1, metrics
assert metrics["dispatch_to_pr_rate"] == "33.3%", metrics
assert metrics["pr_merge_rate"] == "100.0%", metrics
assert sessions[-1]["issue"] == 602, sessions[-1]
assert sessions[-1]["status"] == "in_flight", sessions[-1]
PY

if bash "$DERIVE_METRICS_SCRIPT" --repo-root "$SUCCESS_REPO" --check >/dev/null 2>"$TMP_DIR/derive-check.stderr"; then
	DERIVE_CHECK_STATUS=0
else
	DERIVE_CHECK_STATUS=$?
fi

[ "$DERIVE_CHECK_STATUS" -eq 0 ] || fail "expected derive-metrics --check to pass after wrapper update"

# Test 2: Wrapper fails when derive-metrics fails after the dispatch mutation.
FAIL_REPO="$TMP_DIR/fail-repo"
create_repo "$FAIL_REPO" "merged" "mystery_status"

FAIL_STDOUT="$TMP_DIR/fail.stdout"
FAIL_STDERR="$TMP_DIR/fail.stderr"
if bash "$SCRIPT_UNDER_TEST" --repo-root "$FAIL_REPO" --issue 603 --title "Bad metrics dispatch" --model "gpt-5.4" >"$FAIL_STDOUT" 2>"$FAIL_STDERR"; then
	FAIL_STATUS=0
else
	FAIL_STATUS=$?
fi

[ "$FAIL_STATUS" -ne 0 ] || fail "expected derive-metrics failure to return non-zero exit code"
grep -Fq "unsupported value 'mystery_status'" "$FAIL_STDERR" || fail "expected derive-metrics failure details on stderr"
[ ! -s "$FAIL_STDOUT" ] || fail "expected no stdout when wrapper fails after dispatch"

echo "PASS"
