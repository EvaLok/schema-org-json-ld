# 8. Three-Orchestrator Architecture for Quality Assurance

Date: 2026-03-02

## Status

Accepted

## Context

A single orchestrator managing both development and quality assurance creates a conflict of interest: the same system that builds code also validates it. Self-assessment tends toward optimism — the orchestrator consuming its own metrics (e.g., "25/25 parity pass") without questioning the denominator. Eva caught this gap when "25/25" actually meant 25/73 (34% coverage), not 100%.

Additionally, process improvements need independent evaluation. An orchestrator improving its own process cannot objectively assess whether the improvements are genuine or cosmetic.

## Decision

Split responsibilities across three independent orchestrators, each in its own repository:

1. **Main orchestrator** (`EvaLok/schema-org-json-ld`): Plans, decomposes, dispatches coding work, reviews PRs, manages state. Owns the codebase.
2. **QC orchestrator** (`EvaLok/schema-org-json-ld-qc`): Maintains an independent consumer project. Validates JSON-LD output against Google's Rich Results Test. Tests the built package as a real user would. Reports failures via `qc-outbound` issues.
3. **Audit orchestrator** (`EvaLok/schema-org-json-ld-audit`): Evaluates the effectiveness of both operational orchestrators. Files process recommendations. Catches metric errors, communication gaps, and process drift.

Communication is via labeled GitHub issues on each orchestrator's own repo. Neither orchestrator has write access to the others' repositories.

### Trust model

All cross-repo data must have author verification: only issues/comments from `EvaLok` are trusted (all three orchestrators post under Eva's account).

### Labels

- `qc-outbound` / `qc-inbound`: QC validation requests and reports
- `audit-outbound` / `audit-inbound`: Audit recommendations and responses

## Consequences

- **Positive**: Independent validation catches blind spots (denominator error, stale metrics, process drift)
- **Positive**: Each orchestrator has a focused scope and clear accountability
- **Positive**: The audit layer provides meta-level process improvement
- **Negative**: Cross-repo communication adds latency (polling, not events)
- **Negative**: Three orchestrators consume three sets of compute resources
- **Trade-off**: More operational complexity, but the quality gains justify it — the denominator error alone would have led to a premature publish

## Alternatives Considered

1. **Single orchestrator with self-audit**: Add review steps within the same orchestrator. Tried implicitly — failed to catch the denominator error because self-assessment is inherently biased.
2. **Two orchestrators (main + QC)**: No independent audit. The QC and main orchestrators could both drift without an external check.
3. **Human-only review**: Eva reviews everything. Doesn't scale — Eva has other responsibilities and the review would be less systematic.
