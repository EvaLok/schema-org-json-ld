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

---

## 2026-02-24 — Second Cycle

**Context**: Second orchestrator cycle, triggered ~15 minutes after the first.

### Challenge: PAT permissions insufficient

Both agent dispatches from the first cycle (#10 AggregateRating, #11 Review/Rating) failed with:
> "The token used to assign the agent doesn't have the necessary permissions."

The ORCHESTRATOR_PAT can create issues and comments, but the Copilot coding agent assignment requires additional permissions (likely actions read/write at minimum). This is a complete blocker for the primary workflow — no code can be dispatched until Eva updates the token.

**Lesson**: Always verify agent dispatch actually succeeds before moving on. The first cycle assumed dispatch worked because the API call returned successfully, but the agent's actual start happens asynchronously. Need to check issue comments for agent response within a reasonable window.

### AGENTS.md was significantly wrong

On deeper inspection of the codebase, I found that AGENTS.md contained several critical inaccuracies:

1. **`toArray()` doesn't exist**: AGENTS.md instructed the agent to "implement `toArray()`" and test its output. But schema classes have NO methods at all — `JsonLdGenerator` serializes by reflecting on public properties. If the agent had followed AGENTS.md literally, it would have added broken methods.
2. **Nullable syntax mismatch**: AGENTS.md said `?Type`, but the codebase consistently uses `null|Type`.
3. **Missing reference code**: No concrete examples were provided for the agent to follow.

This is a significant finding about workflow quality. The AGENTS.md was written from memory/assumptions rather than by reading the actual code. **Always verify guidance against the real codebase before publishing it.**

Fixed AGENTS.md with accurate serialization docs, correct syntax conventions, and concrete reference examples.

### Pattern: Use blocked time productively

When blocked on the primary workflow (agent dispatch), use the time to improve the tooling itself — AGENTS.md, skills, documentation. This is high-leverage work that compounds. A more accurate AGENTS.md means fewer failed agent sessions and fewer revision requests.

### Skills system started

Created `.claude/skills/schema-implementation.md` — a step-by-step procedure for implementing a new schema type. This encodes knowledge that would otherwise be lost between sessions and provides a checklist the coding agent can follow.

### Open questions

- What exact PAT permissions does the Copilot coding agent need? The error message lists "actions, contents, issues, and pull requests" — but fine-grained PATs have many sub-categories. Need to confirm the exact set.
- Should the startup checklist include a "verify agent actually started" step? Currently it just checks timelines, but we should verify the agent didn't post an error message.
