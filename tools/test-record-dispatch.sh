#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
WRAPPER_SOURCE="$SCRIPT_DIR/record-dispatch"

fail() {
	echo "FAIL: $1" >&2
	exit 1
}

assert_logged_arg() {
	local expected="$1"
	local log_path="$2"

	grep -Fx -- "$expected" "$log_path" >/dev/null || fail "expected '$expected' in $(basename "$log_path")"
}

assert_no_logged_arg() {
	local unexpected="$1"
	local log_path="$2"

	if grep -Fx -- "$unexpected" "$log_path" >/dev/null; then
		fail "did not expect '$unexpected' in $(basename "$log_path")"
	fi
}

run_wrapper() {
	local stdout_path="$1"
	local stderr_path="$2"
	local status=0
	shift 2

	: >"$ARGS_LOG"
	: >"$GH_CALLS_LOG"
	PATH="$TEST_BIN_DIR:$PATH" WRAPPER_ARGS_LOG="$ARGS_LOG" GH_CALLS_LOG="$GH_CALLS_LOG" GH_OUTPUT="$GH_OUTPUT" GH_FAIL="$GH_FAIL" \
		bash "$WRAPPER_UNDER_TEST" "$@" >"$stdout_path" 2>"$stderr_path" || status=$?
	return "$status"
}

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

TEST_TOOLS_DIR="$TMP_DIR/tools"
TEST_BIN_DIR="$TMP_DIR/bin"
mkdir -p "$TEST_TOOLS_DIR" "$TEST_BIN_DIR" "$TMP_DIR/repo"

cp "$WRAPPER_SOURCE" "$TEST_TOOLS_DIR/record-dispatch"
chmod +x "$TEST_TOOLS_DIR/record-dispatch"
WRAPPER_UNDER_TEST="$TEST_TOOLS_DIR/record-dispatch"

cat <<EOF > "$TEST_TOOLS_DIR/_build-helper.sh"
ensure_binary() {
	BINARY="$TEST_BIN_DIR/record-dispatch-binary"
}
EOF

cat <<'EOF' > "$TEST_BIN_DIR/record-dispatch-binary"
#!/usr/bin/env bash
set -euo pipefail
for arg in "$@"; do
	printf '%s\n' "$arg" >> "$WRAPPER_ARGS_LOG"
done
printf 'stub-output\n'
EOF
chmod +x "$TEST_BIN_DIR/record-dispatch-binary"

cat <<'EOF' > "$TEST_BIN_DIR/gh"
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$GH_CALLS_LOG"
if [ "${GH_FAIL:-0}" = "1" ]; then
	exit 1
fi
printf '%s' "${GH_OUTPUT:-}"
EOF
chmod +x "$TEST_BIN_DIR/gh"

ARGS_LOG="$TMP_DIR/args.log"
GH_CALLS_LOG="$TMP_DIR/gh-calls.log"

# Test 1: Regex patterns auto-detect review-finding references.
PATTERN_FIXTURES="$(cat <<'EOF'
cycle 367 review finding F1|367:1
Fixes cycle-368 review F2|368:2
addresses finding 369:3|369:3
review finding F4 from cycle 370|370:4
EOF
)"

while IFS='|' read -r issue_text expected_ref; do
	[ -n "$issue_text" ] || continue
	GH_OUTPUT="$issue_text"
	GH_FAIL=0
	STDOUT_PATH="$TMP_DIR/pattern.stdout"
	STDERR_PATH="$TMP_DIR/pattern.stderr"

	run_wrapper "$STDOUT_PATH" "$STDERR_PATH" --repo-root "$TMP_DIR/repo" --issue 123 --title "Example dispatch" || fail "expected auto-detect case '$issue_text' to succeed"

	assert_logged_arg "--repo-root" "$ARGS_LOG"
	assert_logged_arg "$TMP_DIR/repo" "$ARGS_LOG"
	assert_logged_arg "--issue" "$ARGS_LOG"
	assert_logged_arg "123" "$ARGS_LOG"
	assert_logged_arg "--title" "$ARGS_LOG"
	assert_logged_arg "Example dispatch" "$ARGS_LOG"
	assert_logged_arg "--addresses-finding" "$ARGS_LOG"
	assert_logged_arg "$expected_ref" "$ARGS_LOG"
	grep -Fq "Auto-detected review finding reference: cycle ${expected_ref%%:*} finding ${expected_ref##*:}" "$STDERR_PATH" || fail "expected auto-detect notice for '$issue_text'"
	assert_logged_arg 'api repos/EvaLok/schema-org-json-ld/issues/123 --jq .title + " " + (.body // "")' "$GH_CALLS_LOG"
done <<< "$PATTERN_FIXTURES"

# Test 2: Explicit --addresses-finding takes precedence and skips auto-detection.
GH_OUTPUT="Fixes cycle 999 review finding F9"
GH_FAIL=0
STDOUT_PATH="$TMP_DIR/explicit.stdout"
STDERR_PATH="$TMP_DIR/explicit.stderr"

run_wrapper "$STDOUT_PATH" "$STDERR_PATH" --repo-root "$TMP_DIR/repo" --issue=124 --title "Explicit dispatch" --addresses-finding=400:5 || fail "expected explicit addresses-finding case to succeed"

assert_logged_arg "--repo-root" "$ARGS_LOG"
assert_logged_arg "$TMP_DIR/repo" "$ARGS_LOG"
assert_logged_arg "--issue=124" "$ARGS_LOG"
assert_logged_arg "--title" "$ARGS_LOG"
assert_logged_arg "Explicit dispatch" "$ARGS_LOG"
assert_logged_arg "--addresses-finding=400:5" "$ARGS_LOG"
assert_no_logged_arg "999:9" "$ARGS_LOG"
[ ! -s "$GH_CALLS_LOG" ] || fail "expected gh not to be called when --addresses-finding is explicit"
if grep -Fq "Auto-detected review finding reference:" "$STDERR_PATH"; then
	fail "did not expect auto-detect notice when --addresses-finding is explicit"
fi

# Test 3: GitHub API failures are best-effort and do not block dispatch.
GH_OUTPUT=""
GH_FAIL=1
STDOUT_PATH="$TMP_DIR/fallback.stdout"
STDERR_PATH="$TMP_DIR/fallback.stderr"

run_wrapper "$STDOUT_PATH" "$STDERR_PATH" --repo-root "$TMP_DIR/repo" --issue 125 --title "Fallback dispatch" || fail "expected API failure fallback to succeed"

assert_no_logged_arg "--addresses-finding" "$ARGS_LOG"
assert_logged_arg 'api repos/EvaLok/schema-org-json-ld/issues/125 --jq .title + " " + (.body // "")' "$GH_CALLS_LOG"
if grep -Fq "Auto-detected review finding reference:" "$STDERR_PATH"; then
	fail "did not expect auto-detect notice when gh api fails"
fi

# Test 4: Non-matching issue text does not inject --addresses-finding.
GH_OUTPUT="This issue updates docs for cycle 125 without any review finding reference."
GH_FAIL=0
STDOUT_PATH="$TMP_DIR/no-match.stdout"
STDERR_PATH="$TMP_DIR/no-match.stderr"

run_wrapper "$STDOUT_PATH" "$STDERR_PATH" --repo-root "$TMP_DIR/repo" --issue 126 --title "No match dispatch" || fail "expected non-matching issue text to succeed"

assert_logged_arg "--repo-root" "$ARGS_LOG"
assert_logged_arg "$TMP_DIR/repo" "$ARGS_LOG"
assert_logged_arg "--issue" "$ARGS_LOG"
assert_logged_arg "126" "$ARGS_LOG"
assert_logged_arg "--title" "$ARGS_LOG"
assert_logged_arg "No match dispatch" "$ARGS_LOG"
assert_no_logged_arg "--addresses-finding" "$ARGS_LOG"
assert_logged_arg 'api repos/EvaLok/schema-org-json-ld/issues/126 --jq .title + " " + (.body // "")' "$GH_CALLS_LOG"
if grep -Fq "Auto-detected review finding reference:" "$STDERR_PATH"; then
	fail "did not expect auto-detect notice for non-matching issue text"
fi

echo "PASS"
