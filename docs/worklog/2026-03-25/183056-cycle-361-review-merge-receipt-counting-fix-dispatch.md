# Cycle 361 — 2026-03-25 18:30 UTC

## What was done

- Merged cycle 360 review artifact ([PR #1763](https://github.com/EvaLok/schema-org-json-ld/issues/1763), 3 findings, complacency 3/5) and C8 pipeline status fix ([PR #1761](https://github.com/EvaLok/schema-org-json-ld/issues/1761), addresses finding 359:1)
- Processed review findings: F1 dispatch_created ([#1765](https://github.com/EvaLok/schema-org-json-ld/issues/1765) receipt counting fix), F2 deferred (auto-issues expansion), F3 actioned (drop phantom C4.7)
- Cleaned 2 dead branches from merged PRs
- Dispatched [#1765](https://github.com/EvaLok/schema-org-json-ld/issues/1765): Fix receipt event counting to use tool names only (--addresses-finding 360:1)

### PRs merged

- [PR #1763](https://github.com/EvaLok/schema-org-json-ld/issues/1763)
- [PR #1761](https://github.com/EvaLok/schema-org-json-ld/issues/1761)

### Issues processed

- None.

## Self-modifications

- None.

## Pre-dispatch state

*Snapshot before review dispatch — final counters may differ after C6.*
- **In-flight agent sessions**: 1
- **Pipeline status**: PASS (12/12 checks)
- **Copilot metrics**: 565 dispatches, 508 merged
- **Publish gate**: published

## Next steps

1. Review and merge PR from [#1765](https://github.com/EvaLok/schema-org-json-ld/issues/1765) (receipt event counting fix) when Copilot completes
2. Use --addresses-finding on all finding-related dispatches

## Commit receipts

> Note: Scope: cycle 361 commits through cycle-complete — mode normal; phase close_out; receipt events: 1 dispatch, 3 merges, 2 reviews. Receipt table covers commits through cycle-complete (C5.1 snapshot). Post-C5.1 commits (docs, record-dispatch, review-body) are structurally excluded.

| Tool | Receipt | Link |
|------|---------|------|
| cycle-start | 3e0e5f9 | [3e0e5f9](https://github.com/EvaLok/schema-org-json-ld/commit/3e0e5f96f5c128db9129f277d9f919a35b52c77f) |
| process-merge | 2134e75 | [2134e75](https://github.com/EvaLok/schema-org-json-ld/commit/2134e753c2a4f4fbd77bf9d9d150bc2fd5ec7965) |
| process-merge | 70f3a55 | [70f3a55](https://github.com/EvaLok/schema-org-json-ld/commit/70f3a55f56454920c8b0ce45befe6ffccc6a905a) |
| process-review | 00f1100 | [00f1100](https://github.com/EvaLok/schema-org-json-ld/commit/00f11000b09d2269fce19c69f04af36f63c8e1d7) |
| cycle-complete | 5ebf3e3 | [5ebf3e3](https://github.com/EvaLok/schema-org-json-ld/commit/5ebf3e3d4606152679dbf653bb033035a312a7db) |
