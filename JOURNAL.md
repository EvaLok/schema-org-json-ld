# Journal

Reflective log for the schema-org-json-ld orchestrator. This is not a changelog — it captures patterns, challenges, decisions, and lessons learned.

---

## 2026-02-24 — First Cycle

**Context**: This is the first orchestrator cycle. All infrastructure (workflow, prompts, checklist, AGENTS.md) was set up earlier today by Eva.

**Observations**:
- The repo is clean and well-structured. Product and BreadcrumbList implementations are solid reference points for the coding agent.
- The existing `TypedSchema` base class and `JsonLdGenerator` utility provide a clear pattern to follow.
- Enum handling (backed string enums serializing to schema.org URLs) is already working well.

**Decision — shared sub-types first**: Starting with AggregateRating and Review/Rating as the first agent dispatches. These are simple types with high leverage — they're prerequisites for Product enhancement (adding reviews/ratings) and for multiple parent types (Recipe, LocalBusiness, SoftwareApplication). This follows the orchestrator prompt's guidance to build shared sub-types before parent types.

**Decision — dispatch plan**: Two concurrent agent tasks for the first batch. AggregateRating is standalone; Review depends on a Rating sub-type which should be included in the same issue since it's small. Keeping these as separate issues for clean PRs.

**Open question**: The state file format (JSON at `docs/state.json`) is a first iteration. May need to evolve as I learn what information I reference most frequently between sessions.

**State file location**: `docs/state.json` — documented here for future sessions.
