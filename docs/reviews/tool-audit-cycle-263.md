# Tool Audit — Cycle 263

**Date**: 2026-03-15  
**Scope**: 28 Rust crates in `tools/rust/crates/` (excluding the `state-schema` library crate)  
**Method**: Ran `cargo test -p <crate> --manifest-path tools/rust/Cargo.toml` for every tool crate, then reviewed each crate's `Cargo.toml`, primary Rust source file, shell wrapper under `tools/`, and `pipeline-check` integration points. All 28 crates currently pass their package tests.

## Per-tool table

| Crate | Tests | Test Count | Coverage Assessment | Issues Found |
|---|---|---:|---|---|
| `backfill-sessions` | yes | 4 | poor | Only 4 tests for GitHub/state backfill logic; no integration coverage. |
| `check-agent-prs` | yes | 7 | adequate | Covers core classification and CI parsing, but there is no broader integration coverage. |
| `check-commitments` | yes | 3 | poor | Just 3 tests for journal parsing and deferred-escalation logic. |
| `check-field-inventory` | yes | 8 | adequate | 8 tests cover thresholds and stale detection, but deeper malformed-input cases are still light. |
| `cross-repo` | yes | 9 | adequate | Good fail-closed parsing tests, but only 9 total for two audit/reporting modes. |
| `cycle-close` | yes | 8 | poor | Critical end-of-cycle side effects with only 8 tests and no end-to-end coverage of the full close-out flow. |
| `cycle-complete` | yes | 24 | good | 24 passing tests including 1 integration test; good happy-path and reconciliation/error-path coverage. |
| `cycle-phase` | yes | 6 | adequate | Small tool with 6 tests; covers valid transitions and invalid phase rejection. |
| `cycle-receipts` | yes | 7 | adequate | 7 tests cover receipt collection basics, but malformed-history/shallow-clone edge cases remain light. |
| `cycle-start` | yes | 24 | good | 24 tests cover startup patch generation, directive parsing, and stale-close-out handling. |
| `cycle-status` | yes | 21 | good | 21 passing tests including 1 integration test; strong coverage for action-item and gate classification. |
| `derive-metrics` | yes | 6 | adequate | 6 tests cover derived-metric mismatches and fail-closed unknown status handling, but the suite is still small for a pipeline gate. |
| `dispatch-review` | yes | 3 | poor | Only 3 tests for a tool that creates GitHub issues and records state changes. |
| `housekeeping-scan` | yes | 11 | good | 11 focused tests exercise stale/orphan detection; no obvious production-path issue stood out. |
| `metric-snapshot` | yes | 10 | adequate | 10 tests hit fix-up generation and pointer validation; good but not exhaustive for a pipeline gate. |
| `pipeline-check` | yes | 57 | good | 57 tests give broad happy-path and failure classification coverage for the main orchestration gate. |
| `post-step` | yes | 27 | good | 27 tests cover CLI validation, fail-closed comment posting, and step-id handling. |
| `process-audit` | yes | 5 | poor | Only 5 tests and they mostly cover idempotency/state patching, not command failure paths. |
| `process-audit-inbound` | yes | 6 | adequate | 6 tests cover stale accepted-item discovery and parser rejection cases. |
| `process-eva` | yes | 8 | adequate | 8 tests cover close/no-change interactions, but broader malformed-state cases are limited. |
| `process-merge` | yes | 15 | adequate | 15 tests cover invariant checks and issue flag parsing; good core coverage without integration tests. |
| `process-review` | yes | 40 | good | 40 tests cover parsing, categorization, and validation rules across many review shapes. |
| `receipt-validate` | yes | 7 | adequate | 7 tests cover missing/malformed receipt rows and fail-closed behavior; parser remains format-coupled. |
| `record-dispatch` | yes | 22 | good | 22 tests cover invariant failures, duplicate handling, and dispatch patch application. |
| `refresh-field-inventory` | yes | 16 | good | 16 tests cover verification, refresh-only behavior, and non-write failure handling. |
| `state-invariants` | yes | 39 | good | Strong semantic validation coverage; the main gap is limited integration beyond unit-style fixtures. |
| `validate-docs` | yes | 15 | good | 15 tests cover receipt, pipeline-status, and worklog/journal validation failures; no obvious code-quality issue stood out. |
| `write-entry` | yes | 76 | good | 76 passing tests including 1 integration test; strongest coverage in the workspace despite large file-I/O surface. |

## Purpose and integration matrix

| Crate | Purpose | Shell wrapper | Wired into `pipeline-check` |
|---|---|---|---|
| `backfill-sessions` | Backfill historical agent_sessions entries from GitHub metadata. | `tools/backfill-sessions` | No |
| `check-agent-prs` | Batch-check open Copilot PRs for readiness and CI status. | `tools/check-agent-prs` | No |
| `check-commitments` | Extract cycle commitments from journal entries and flag deferred review findings. | `tools/check-commitments` | No |
| `check-field-inventory` | Check docs/state.json field-inventory completeness and staleness thresholds. | `tools/check-field-inventory-rs` | Yes |
| `cross-repo` | Run read-only cross-repo QC and audit polling for the orchestrator. | `tools/cross-repo` | No |
| `cycle-close` | Automate end-of-cycle commit/push/comment/close workflow. | `tools/cycle-close` | No |
| `cycle-complete` | Handle end-of-cycle completion tasks and state patching. | `tools/cycle-complete` | No |
| `cycle-phase` | Transition docs/state.json to a new cycle phase. | `tools/cycle-phase` | No |
| `cycle-receipts` | Collect canonical commit receipts for a cycle for worklogs. | `tools/cycle-receipts` | No |
| `cycle-start` | Run the consolidated startup sequence for a new orchestrator cycle. | `tools/cycle-start` | No |
| `cycle-status` | Report the startup checklist status for the current cycle. | `tools/cycle-status` | Yes |
| `derive-metrics` | Derive copilot_metrics from docs/state.json agent_sessions. | `tools/derive-metrics` | Yes |
| `dispatch-review` | Create cycle-review issues and record the dispatch in state. | `tools/dispatch-review` | No |
| `housekeeping-scan` | Scan for stale issues, orphan PRs, and dead branches. | `tools/housekeeping-scan` | Yes |
| `metric-snapshot` | Verify docs/state.json metrics against repository state. | `tools/metric-snapshot` | Yes |
| `pipeline-check` | Run the unified verification pipeline and aggregate step results. | `tools/pipeline-check` | Self |
| `post-step` | Post checklist step outcomes as issue comments. | `tools/post-step` | No |
| `process-audit` | Record processed audit recommendations in docs/state.json. | `tools/process-audit` | No |
| `process-audit-inbound` | Find unprocessed audit recommendations and stale accepted inbound items. | `tools/process-audit-inbound` | No |
| `process-eva` | Record Eva directive processing in docs/state.json. | `tools/process-eva` | No |
| `process-merge` | Process merged PRs and update copilot_metrics in state. | `tools/process-merge` | No |
| `process-review` | Consume review markdown and update review-agent history. | `tools/process-review` | No |
| `receipt-validate` | Validate worklog receipt tables against canonical cycle receipts. | `tools/receipt-validate` | No |
| `record-dispatch` | Record new Copilot dispatches in docs/state.json. | `tools/record-dispatch` | No |
| `refresh-field-inventory` | Refresh stale field-inventory entries after verification. | `tools/refresh-field-inventory` | No |
| `state-invariants` | Check semantic consistency of docs/state.json. | `tools/state-invariants` | Yes |
| `validate-docs` | Validate worklog and journal artifacts against repository state. | `tools/validate-docs` | Yes |
| `write-entry` | Generate worklog and journal markdown from structured JSON input. | `tools/write-entry` | No |

## Summary

### 1. Tools with no tests

- None. Every audited Rust tool crate has at least one passing test, and all 28 packages passed `cargo test -p <crate>`.

### 2. Tools with poor coverage

- `backfill-sessions` — only 4 tests for a tool that fetches GitHub metadata and reconstructs agent-session state.
- `check-commitments` — only 3 tests for journal parsing and multi-cycle deferred-finding escalation logic.
- `cycle-close` — only 8 tests for a high-side-effect tool that commits, pushes, posts comments, and closes issues.
- `dispatch-review` — only 3 tests despite creating GitHub issues and mutating dispatch state.
- `process-audit` — only 5 tests and little direct coverage of failure paths beyond idempotency/state-patch basics.

### 3. Top 5 improvement recommendations

1. **Increase `cycle-close` coverage first.** It is operationally high-impact, test-light, and lacks an end-to-end regression that exercises the complete close-out flow.
2. **Expand `dispatch-review` and `process-audit` failure-path tests.** Both tools mutate durable state and/or GitHub state, but their current suites are too small to inspire confidence when command or API calls fail.
3. **Add stronger API/error-path coverage to `backfill-sessions`.** The crate reconstructs historical session state from external GitHub data, yet the suite is only 4 tests deep.
4. **Add parser edge-case coverage for `check-commitments` and `cycle-receipts`.** Both rely on markdown/git-history parsing where malformed input, partial history, or shallow-clone cases are important.
5. **Add at least one cross-tool smoke test around the core orchestration chain.** `pipeline-check`, `cycle-status`, `validate-docs`, and `write-entry` are individually tested, but most state-mutating tools are still validated in isolation.

### 4. Pipeline maturity assessment

The Rust toolchain is **moderately mature**: all 28 crates have shell wrappers, and the critical verification path is automated through `pipeline-check`, which directly composes `metric-snapshot`, `check-field-inventory`, `housekeeping-scan`, `cycle-status`, `state-invariants`, `derive-metrics`, and `validate-docs`. That gives the orchestrator a reliable automated gate for repository-state, documentation, and hygiene checks.

The maturity gap is **breadth of composition rather than basic tooling**. Most operational/state-mutating tools (`cycle-close`, `cycle-complete`, `dispatch-review`, `process-audit`, `process-merge`, `process-review`, `record-dispatch`, `write-entry`) are still exercised mainly by unit tests and manual orchestration steps rather than by a single end-to-end smoke suite. In practice, the validation pipeline is strong, but the action pipeline still depends on disciplined operator sequencing.

What is still manual or under-automated:

- End-of-cycle actions are spread across multiple tools rather than being regression-tested as one complete flow.
- GitHub-API writers such as `dispatch-review` and `cycle-close` lack the same depth of fail-closed test coverage seen in `pipeline-check` or `validate-docs`.
- Receipt/journal/worklog generation and validation are individually strong, but there is little automated proof that the full authoring → validation → close-out chain behaves correctly together.

## Coverage methodology notes

- **Test count** comes from `cargo test -p <crate> -- --list` after confirming the package passed `cargo test -p <crate> --manifest-path tools/rust/Cargo.toml`.
- **Coverage assessment** is qualitative: `good` means the suite exercises the main happy path plus multiple error/edge paths; `adequate` means the main path is covered with some negative-path testing; `poor` means the suite is too small or too narrow for the tool's operational complexity.
- **Issues found** focus on actionable findings only: sparse tests, obvious `unwrap()` use on fallible production paths, parser brittleness, or notable integration gaps.
