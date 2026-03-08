# 7. Rust Tooling for Orchestrator Infrastructure

Date: 2026-03-04

## Status

Accepted

## Context

The orchestrator runs in a GitHub Actions sandbox with restricted permissions. Shell constructs like pipes (`|`), command substitution (`$()`), heredocs (`<<`), and `jq -f` are blocked. Early tools were written as `jq` scripts and shell scripts, but these hit sandbox restrictions repeatedly — `jq -f` was blocked, pipes were blocked, and complex shell logic was unreliable.

The orchestrator needed tools that could: parse JSON (state.json is 800+ lines), perform complex logic (field inventory validation, metric comparison), and produce structured output — all within the sandbox's allowed commands (`bash`, `cargo`, `gh`, `git`, `jq` for simple queries).

## Decision

Build all orchestrator tools as compiled Rust binaries in a Cargo workspace at `tools/rust/`. Key design choices:

1. **Cargo workspace** with `members = ["crates/*"]` — new crates are auto-discovered
2. **Shared `state-schema` crate** providing strongly-typed serde structs for state.json — all tools share the same types
3. **Shell wrappers** at `tools/<name>` that locate and run the release binary, building it if missing
4. **CI pre-builds** all Rust tools before the orchestrator session starts, so tools are instantly available
5. **`clap` for CLI parsing** — consistent argument handling across all tools
6. **Invoked via `bash tools/<name>`** — `bash` is in the allowed commands list

Current tools: `metric-snapshot`, `check-field-inventory-rs`, `cycle-status`, `housekeeping-scan`, `pipeline-check`, `cycle-complete`, `cycle-start`, `record-dispatch`, `process-merge`, `process-review`, `process-audit`, `process-eva`.

## Consequences

- **Positive**: No sandbox permission issues — compiled binaries run directly
- **Positive**: Type safety catches state.json schema drift at compile time
- **Positive**: Tools can be composed into pipelines (pipeline-check orchestrates 5 sub-tools)
- **Positive**: Copilot agent can build new tools given the `rust-tooling` skill
- **Negative**: Compilation adds ~30s to first use if not pre-built by CI
- **Negative**: Rust has a steeper learning curve than shell scripts
- **Trade-off**: More upfront work per tool, but dramatically more reliable than shell/jq alternatives

## Alternatives Considered

1. **jq scripts**: Simple and fast to write, but `jq -f` is blocked in the sandbox and complex logic in inline `jq` is unreadable.
2. **Node.js/TypeScript scripts**: Would work (Node is available), but adds a dependency on the TS build system for infrastructure tools. Rust keeps infrastructure separate from library code.
3. **Python scripts**: Python is available in Actions runners, but not in the orchestrator's allowed commands list.
