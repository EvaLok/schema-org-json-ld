# 5. Documentation as Continuous Maintenance

Date: 2026-02-26

## Status

Accepted

## Context

The README was comprehensively written in Cycle 12 (~Feb 25 early AM) covering all 28 Google Rich Results types with examples. However, 17+ subsequent PRs added new properties, sub-types, and classes without corresponding README updates. By Cycle 30, the README was significantly out of date across 9 type sections, with 5 missing sub-types in the supported types table and incorrect class counts.

The gap was discovered during a proactive audit comparing README examples against current source constructor signatures.

## Decision

Documentation updates should be treated as part of each enhancement PR, not deferred to separate documentation cycles. Specifically:

1. **When adding new properties to an existing schema class**: The PR should include a README update if the property is user-facing (not internal plumbing).
2. **When adding new sub-types**: The supported types table should be updated in the same PR.
3. **Class count in header**: Should be updated when new classes are added.
4. **Periodic audits**: Even with per-PR updates, quarterly README audits should verify accuracy.

For the coding agent, this means the issue spec should explicitly mention README updates when new properties or sub-types are significant enough to showcase.

## Consequences

- **Positive**: README stays accurate, users see all available features.
- **Positive**: Smaller, incremental README updates are easier to review than large batch updates.
- **Negative**: Slightly more work per PR (but less total work than periodic catch-up).
- **Trade-off**: Issue specs become slightly longer, but the alternative (stale docs) is worse.

## Alternatives Considered

1. **Batch documentation updates**: What we did — write README once, let it drift. Led to significant staleness after 17+ PRs.
2. **Automated README generation**: Generate examples from tests or source. Too complex for the current project size and would produce less readable output.
3. **Separate CHANGELOG**: Track changes but not update examples. Doesn't help users who reference the README for usage patterns.
