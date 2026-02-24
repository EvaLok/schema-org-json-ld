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

- ~~What exact PAT permissions does the Copilot coding agent need?~~ **Resolved**: Eva confirmed read/write for actions, contents, issues, and pull requests. This is the minimum set.
- ~~Should the startup checklist include a "verify agent actually started" step?~~ **Resolved**: Already added in Cycle 2 checklist update.

---

## 2026-02-24 — Third Cycle

**Context**: Third cycle. PAT permissions resolved by Eva. First successful agent dispatches.

### PAT permissions resolved

Eva confirmed in issue #13 that she updated the PAT with read/write for actions, contents, issues, and pull requests. Closed #13.

### First successful dispatches

- **Issue #15 → PR #16**: AggregateRating (copilot_work_started at 16:08:16Z)
- **Issue #17 → PR #18**: Review + Rating (copilot_work_started at 16:08:31Z)

Both agents started successfully. This is the first time the end-to-end dispatch workflow has worked. The issue body format seems adequate — clear specs with properties, file paths, references, and acceptance criteria.

### Bug fix: Copilot author login

The STARTUP_CHECKLIST concurrency check filtered PRs by `author.login == "copilot-swe-agent[bot]"`, but the actual author login for Copilot PRs is `app/copilot-swe-agent`. Fixed.

### Pattern: Issue numbering

Issue numbers and PR numbers share a namespace on GitHub. Creating issue #15 resulted in PR #16 (the next available number). Creating issue #17 resulted in PR #18. This is expected but worth noting — when mapping issues to PRs, the PR number is always the issue number + 1 in practice (assuming no other activity in between).

### Design decision: Review.author as string

Kept `author` as `string` in the Review type rather than `Person|Organization`. Rationale: Person and Organization types don't exist yet, and we want to keep the issue scope manageable. The agent doesn't need to deal with union types of missing classes. We'll upgrade when Person is implemented (priority 4 in the roadmap).

### Open questions

- How long does a typical Copilot agent session take for a simple schema type? Tracking this to calibrate expectations.
- Will the agent create separate test files as specified, or add tests to the existing JsonLdGeneratorTest? The issue spec is clear, but worth verifying.
