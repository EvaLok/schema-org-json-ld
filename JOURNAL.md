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

### Agent session timing (answered)

- **Review+Rating (2 types + 2 test files)**: ~7 minutes
- **AggregateRating (1 type + 1 test file)**: ~10 minutes
- Paradoxically, the simpler task took longer. Sample size is too small to draw conclusions. Both are well within the 60-minute timeout.

### Agent test file behavior (answered)

The agent correctly created separate test files as specified (`RatingTest.php`, `ReviewTest.php`, `AggregateRatingTest.php`). It did NOT add to the existing `JsonLdGeneratorTest.php`. The issue spec was clear enough.

### Agent can't run tests (firewall limitation)

The Copilot agent's firewall blocks downloads from `api.github.com` for composer dependencies. This means the agent cannot install PHPUnit and therefore cannot verify its own tests. The orchestrator must verify tests locally before merging. This is a significant limitation — it means the agent can't self-verify, increasing the risk of broken code reaching review.

Possible mitigation: Set up a CI workflow that runs tests on PR branches. This would catch failures before the orchestrator reviews. However, the orchestrator can't push workflow changes directly.

### Observation: null-check style inconsistency

PR #16 used `assertObjectNotHasProperty()` (matching AGENTS.md examples) while PR #18 used `assertFalse(property_exists())`. Both are functionally equivalent. The inconsistency comes from two independent agent sessions following slightly different patterns. Not worth fixing now, but worth noting for AGENTS.md improvement — could add explicit guidance on which assertion to use.

### Pattern: Orchestrator review flow

The review flow that worked well:
1. Wait for `copilot_work_finished` event
2. Fetch branch locally
3. Run tests
4. Review diff
5. Mark ready, approve, squash merge
6. Delete branch

This is reliable and catches issues the agent can't (since it can't run tests).

---

## 2026-02-24 — Fourth Cycle

**Context**: Fourth cycle. Agents can now run tests (Eva set up copilot-setup-steps.yml). Clean slate — no in-flight sessions.

### Eva input: Copilot can now run PHPUnit

Issue #19 from Eva: the `copilot-setup-steps.yml` file pre-installs PHP 8.3 and composer deps before the agent starts. This means agents can now self-verify by running `composer run test-unit`. Updated AGENTS.md with a "Running Tests" section making this explicit.

This resolves the "agent can't run tests" limitation noted in Cycle 3. It's a significant workflow improvement — the agent can now catch test failures before submitting its PR.

### Indentation inconsistency between agents

Two parallel agent sessions produced inconsistent indentation:
- PR #22 (Organization): Used tabs correctly, matching codebase style
- PR #24 (FAQPage): No indentation at all in class/method bodies

This confirms the Cycle 3 observation about style inconsistency between sessions. The fix: added explicit "Tab indentation" rule to AGENTS.md Coding Standards section. Reference code alone isn't sufficient — agents need explicit style guidance.

### First revision loop via @copilot

Successfully used `@copilot` on PR #24 to request indentation fix. The revision session took ~5 minutes. The agent understood the feedback, fixed all 6 files, and tests still passed. This validates the revision workflow.

**Cost**: 2 premium requests for the FAQPage issue (initial + revision). Worth it for code quality, but indicates room for improvement in AGENTS.md to prevent the need for revisions.

### Agent timing observations (more data)

| Task | Types | Files | Duration | Notes |
|------|-------|-------|----------|-------|
| AggregateRating | 1 | 2 | ~10 min | Cycle 3 |
| Review+Rating | 2 | 4 | ~7 min | Cycle 3 |
| Organization+PostalAddress+ContactPoint | 3 | 6 | ~8 min | Cycle 4, clean |
| FAQPage+Question+Answer | 3 | 6 | ~7 min + 5 min revision | Cycle 4, needed fix |

Session duration seems roughly consistent (7-10 min) regardless of the number of types. The agent handles 3-type bundles as efficiently as single types. This suggests we could potentially bundle more sub-types per issue without significant time penalty.

### Dependency graph progress

With Organization and FAQPage merged, the dependency landscape has shifted:
- **Article** now only needs Person and ImageObject (Organization done)
- **Event** now only needs nothing extra (Organization and PostalAddress both done)
- **LocalBusiness** has all dependencies met (Organization, PostalAddress, AggregateRating, Review)

Event and LocalBusiness could potentially be dispatched now. But following the "shared sub-types first" principle, ImageObject and Person should come next since they unlock even more parent types.

### State file evolution

Added `test_count`, `total_schema_types`, and `total_sub_types` fields to the state file. These provide quick metrics without needing to count entries.
