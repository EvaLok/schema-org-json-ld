# Shared build helper for Rust tool wrappers.
# Source this file, then call: ensure_binary <crate-name>
#
# Sets BINARY to the path of the release binary, rebuilding if:
#   1. The binary does not exist, OR
#   2. Any Rust source file (.rs) or Cargo.toml under tools/rust/crates/,
#      or the workspace Cargo.toml at tools/rust/Cargo.toml,
#      is newer than the binary (source-freshness check).
#
# Usage:
#   source "$SCRIPT_DIR/_build-helper.sh"
#   ensure_binary "cycle-complete"
#   exec "$BINARY" ...

ensure_binary() {
	local crate_name="$1"
	local tools_dir
	tools_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
	BINARY="$tools_dir/rust/target/release/$crate_name"

	if [ -x "$BINARY" ]; then
		# Rebuild if any source file is newer than the binary
		local newer
		newer=$(find "$tools_dir/rust/crates" "$tools_dir/rust/Cargo.toml" \( -name "*.rs" -o -name "Cargo.toml" \) -newer "$BINARY" -print -quit 2>/dev/null || true)
		if [ -n "$newer" ]; then
			echo "Source changed since $crate_name was built, rebuilding..." >&2
			cargo build --release -p "$crate_name" --manifest-path "$tools_dir/rust/Cargo.toml" >&2
		fi
	else
		echo "Pre-built binary not found, building $crate_name..." >&2
		cargo build --release -p "$crate_name" --manifest-path "$tools_dir/rust/Cargo.toml" >&2
	fi
}
