# Tool Audit — Cycle 236

**Date**: 2026-03-12
**Auditor**: main-orchestrator (Claude Opus 4.6)
**Trigger**: Audit #214 (tool audit counter without artifact)
**Previous artifact**: [tool-audit-cycle-181.md](tool-audit-cycle-181.md) (17 tools)

## Summary

| Category | Count |
|---|---|
| Rust tool crates (with shell wrappers) | 29 |
| Shell-only tools | 1 |
| TypeScript-only tools | 1 |
| Rust library crates (`state-schema`) | 1 |
| **Total tools** | **31** |
| Build status | All compile, 0 errors, 0 warnings |

## Count history and reconciliation

| Cycle | Claimed count | Explanation |
|---|---|---|
| 181 | 17 | Documented artifact. Only Rust tool crates existed at the time. |
| 222 | 32 | No artifact. Likely counted `state-schema`, `config.json`, test scripts. |
| 235 | 28 | No artifact. Likely counted only a subset without consistent methodology. |
| **236** | **31** | **This artifact.** Methodology: all executables in `tools/` excluding `_build-helper.sh` and `test-*.sh`, plus all Rust tool crates in `tools/rust/crates/` excluding `state-schema` (library). |

The count decreased from 32 to 28 between cycles 222 and 235 because the counting methodology was inconsistent. No tools were actually removed — the previous counts simply used different inclusion criteria.

## Full inventory

| # | Tool | Type | Shell wrapper | Rust crate | Status |
|---|---|---|---|---|---|
| 1 | backfill-sessions | Rust + shell | `tools/backfill-sessions` | `crates/backfill-sessions` | OK |
| 2 | check-agent-prs | Rust + shell | `tools/check-agent-prs` | `crates/check-agent-prs` | OK |
| 3 | check-commitments | Rust + shell | `tools/check-commitments` | `crates/check-commitments` | OK |
| 4 | check-doc-pr | Rust + shell | `tools/check-doc-pr` | `crates/check-doc-pr` | OK |
| 5 | check-field-inventory | Rust + shell | `tools/check-field-inventory-rs` | `crates/check-field-inventory` | OK |
| 6 | commit-state-change | Shell-only | `tools/commit-state-change` | N/A | OK |
| 7 | cross-repo | Rust + shell | `tools/cross-repo` | `crates/cross-repo` | OK |
| 8 | cycle-close | Rust + shell | `tools/cycle-close` | `crates/cycle-close` | OK |
| 9 | cycle-complete | Rust + shell | `tools/cycle-complete` | `crates/cycle-complete` | OK |
| 10 | cycle-phase | Rust + shell | `tools/cycle-phase` | `crates/cycle-phase` | OK |
| 11 | cycle-receipts | Rust + shell | `tools/cycle-receipts` | `crates/cycle-receipts` | OK |
| 12 | cycle-start | Rust + shell | `tools/cycle-start` | `crates/cycle-start` | OK |
| 13 | cycle-status | Rust + shell | `tools/cycle-status` | `crates/cycle-status` | OK |
| 14 | derive-metrics | Rust + shell | `tools/derive-metrics` | `crates/derive-metrics` | OK |
| 15 | dispatch-docs | Rust + shell | `tools/dispatch-docs` | `crates/dispatch-docs` | OK |
| 16 | dispatch-review | Rust + shell | `tools/dispatch-review` | `crates/dispatch-review` | OK |
| 17 | housekeeping-scan | Rust + shell | `tools/housekeeping-scan` | `crates/housekeeping-scan` | OK |
| 18 | metric-snapshot | Rust + shell | `tools/metric-snapshot` | `crates/metric-snapshot` | OK |
| 19 | parity-check | TypeScript | `tools/parity-check.ts` | N/A | OK |
| 20 | pipeline-check | Rust + shell | `tools/pipeline-check` | `crates/pipeline-check` | OK |
| 21 | post-step | Rust + shell | `tools/post-step` | `crates/post-step` | OK |
| 22 | process-audit | Rust + shell | `tools/process-audit` | `crates/process-audit` | OK |
| 23 | process-audit-inbound | Rust + shell | `tools/process-audit-inbound` | `crates/process-audit-inbound` | OK |
| 24 | process-eva | Rust + shell | `tools/process-eva` | `crates/process-eva` | OK |
| 25 | process-merge | Rust + shell | `tools/process-merge` | `crates/process-merge` | OK |
| 26 | process-review | Rust + shell | `tools/process-review` | `crates/process-review` | OK |
| 27 | record-dispatch | Rust + shell | `tools/record-dispatch` | `crates/record-dispatch` | OK |
| 28 | refresh-field-inventory | Rust + shell | `tools/refresh-field-inventory` | `crates/refresh-field-inventory` | OK |
| 29 | state-invariants | Rust + shell | `tools/state-invariants` | `crates/state-invariants` | OK |
| 30 | validate-docs | Rust + shell | `tools/validate-docs` | `crates/validate-docs` | OK |
| 31 | write-entry | Rust + shell | `tools/write-entry` | `crates/write-entry` | OK |

## Discrepancies

- **None blocking.** All 29 Rust crates have shell wrappers and all shell wrappers point to valid crates.
- **Naming asymmetry (minor)**: `check-field-inventory-rs` (wrapper) vs `check-field-inventory` (crate). Intentional — the `-rs` suffix distinguishes from a former jq-based tool.

## Evidence

- `cargo build --manifest-path tools/rust/Cargo.toml` — 0 errors, 0 warnings
- `cargo test -p pipeline-check --manifest-path tools/rust/Cargo.toml` — 49/49 pass (after cycle 236 test fix)
- `cargo test -p write-entry --manifest-path tools/rust/Cargo.toml` — 58/58 pass (after PR #1111 merge)

## Methodology

Counted as a "tool": any executable file in `tools/` or binary crate in `tools/rust/crates/` that is intended for orchestrator use. Excluded:
- `_build-helper.sh` (internal build infrastructure)
- `test-*.sh` (test helpers, not tools)
- `config.json` (configuration, not a tool)
- `state-schema` crate (library dependency, not a standalone tool)
