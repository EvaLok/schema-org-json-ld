#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SCRIPT_UNDER_TEST="$SCRIPT_DIR/commit-state-change"

fail() {
	echo "FAIL: $1" >&2
	exit 1
}

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

TEST_REPO="$TMP_DIR/repo"
mkdir -p "$TEST_REPO/docs"
cd "$TEST_REPO"

git init -q
git config user.name "Test User"
git config user.email "test@example.com"

cat <<'EOF' > docs/state.json
{
  "cycle": 1
}
EOF

git add docs/state.json
git commit -q -m "initial state"

# Test 1: Successful commit returns short hash and expected commit message.
cat <<'EOF' > docs/state.json
{
  "cycle": 2
}
EOF

SUCCESS_STDOUT="$TMP_DIR/success.stdout"
SUCCESS_STDERR="$TMP_DIR/success.stderr"
if bash "$SCRIPT_UNDER_TEST" --tool-name "metric-snapshot" --summary "update copilot metrics" --cycle 155 >"$SUCCESS_STDOUT" 2>"$SUCCESS_STDERR"; then
	SUCCESS_STATUS=0
else
	SUCCESS_STATUS=$?
fi

[ "$SUCCESS_STATUS" -eq 0 ] || fail "expected success exit code 0, got $SUCCESS_STATUS"
SUCCESS_HASH="$(cat "$SUCCESS_STDOUT")"
echo "$SUCCESS_HASH" | grep -Eq '^[0-9a-f]{7}$' || fail "expected 7-char hex hash on stdout, got '$SUCCESS_HASH'"

LATEST_MESSAGE="$(git log -1 --pretty=%s)"
EXPECTED_MESSAGE="state(metric-snapshot): update copilot metrics [cycle 155]"
[ "$LATEST_MESSAGE" = "$EXPECTED_MESSAGE" ] || fail "unexpected commit message: '$LATEST_MESSAGE'"

# Test 2: No changes returns success, empty stdout, and warning on stderr.
NOCHANGE_STDOUT="$TMP_DIR/nochange.stdout"
NOCHANGE_STDERR="$TMP_DIR/nochange.stderr"
if bash "$SCRIPT_UNDER_TEST" --tool-name "metric-snapshot" --summary "no changes" --cycle 156 >"$NOCHANGE_STDOUT" 2>"$NOCHANGE_STDERR"; then
	NOCHANGE_STATUS=0
else
	NOCHANGE_STATUS=$?
fi

[ "$NOCHANGE_STATUS" -eq 0 ] || fail "expected no-change exit code 0, got $NOCHANGE_STATUS"
[ ! -s "$NOCHANGE_STDOUT" ] || fail "expected no hash output when there are no changes"
grep -Fq "Warning: no changes to docs/state.json; nothing to commit" "$NOCHANGE_STDERR" || fail "expected no-change warning on stderr"

# Test 3: Missing args returns non-zero and prints usage.
MISSING_STDOUT="$TMP_DIR/missing.stdout"
MISSING_STDERR="$TMP_DIR/missing.stderr"
if bash "$SCRIPT_UNDER_TEST" --tool-name "metric-snapshot" --cycle 157 >"$MISSING_STDOUT" 2>"$MISSING_STDERR"; then
	MISSING_STATUS=0
else
	MISSING_STATUS=$?
fi

[ "$MISSING_STATUS" -ne 0 ] || fail "expected missing args to return non-zero exit code"
grep -Fq "Usage:" "$MISSING_STDERR" || fail "expected usage message on stderr for missing args"

echo "PASS"
