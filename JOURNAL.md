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

### Second batch: ImageObject and Person

Dispatched and merged both within the same cycle. After adding the tab indentation rule to AGENTS.md, neither PR had style issues — confirming that explicit AGENTS.md guidance is more effective than implicit reference code.

### Updated timing data

| Task | Types | Files | Duration | Notes |
|------|-------|-------|----------|-------|
| ImageObject | 1 | 2 | ~5 min | Cycle 4 batch 2, clean |
| Person | 1 | 2 | ~9 min | Cycle 4 batch 2, clean |

Single-type tasks average 5-9 minutes. Multi-type tasks average 7-8 minutes. No clear correlation between complexity and duration at this scale.

### Milestone: All shared sub-types complete

All shared sub-types identified in the initial roadmap are now implemented. This is a significant inflection point — we can now dispatch parent types without dependency blockers. The "shared sub-types first" strategy worked exactly as planned.

### Eva's PHP-CS-Fixer

Eva added a `.php-cs-fixer.dist.php` config and a CI lint job during this cycle. This is a positive development for code quality. Observations:
- She fixed CS issues on the ImageObject files post-merge
- Person files will likely need the same treatment
- Future AGENTS.md update needed: instruct agents to run `vendor/bin/php-cs-fixer fix` or at least match the CS rules
- The CS-Fixer config is in the repo at `.php-cs-fixer.dist.php` — I should read it to understand the rules

### Pattern: Two batches per cycle

This cycle proved that two sequential dispatch-review-merge batches are achievable within a single 75-minute session. The workflow is:
1. Dispatch batch 1 (2 tasks) → wait ~10 min → review both → merge
2. Dispatch batch 2 (2 tasks) → wait ~10 min → review both → merge
3. Update state and close

Total productive output: 4 merged PRs, 8 new types, 20 new tests. This is a good throughput for the workflow.

---

## 2026-02-24 — Fifth Cycle

**Context**: Fifth cycle. Clean slate — no in-flight sessions. Eva input to handle.

### CS-Fixer integration is working

Eva's PHP-CS-Fixer addition (#29) was the key process improvement this cycle. After adding `composer run cs-fix` to AGENTS.md's Code Style section and Quality Checklist, all 4 agents in both batches ran cs-fix proactively. Zero style issues in any of the 4 PRs. This eliminates the revision-for-style-fix pattern seen in Cycle 4 (FAQPage). The lesson: explicit, actionable instructions in AGENTS.md are highly effective.

### Inheritance pattern works for schema subtypes

Article/NewsArticle/BlogPosting demonstrated that schema.org type inheritance maps well to PHP class inheritance. NewsArticle and BlogPosting are 3-line classes that extend Article and override only `A_SCHEMA_TYPE`. The constructor is inherited. Tests verify each subtype produces the correct `@type` value.

### Agent timing update

| Task | Types | Files | Duration | Notes |
|------|-------|-------|----------|-------|
| Article+NewsArticle+BlogPosting | 3 | 6 | ~10 min | Cycle 5 batch 1, clean |
| Event+Place+EventStatusType | 3 | 5 | ~8 min | Cycle 5 batch 1, clean |
| LocalBusiness+Geo+OpeningHours+DayOfWeek | 4 | 7 | ~25 min | Cycle 5 batch 2, clean |
| Recipe+NutritionInfo+HowToStep | 3 | 6 | ~25 min | Cycle 5 batch 2, clean |

Notable: batch 2 tasks took ~25 min each vs ~8-10 min for batch 1. These were the most complex types dispatched (16-17 properties each with multiple sub-types). Both agents remained in draft state well past their last commit. The orchestrator proactively marked them ready and merged after local test verification.

### CI workflow runs show as `action_required`

The "Test and Build" CI workflow on Copilot PRs consistently shows `action_required` status. Since the orchestrator verifies tests locally before merging, this isn't blocking. Worth noting for Eva if she wants to adjust Actions settings.

### 12 Google types implemented

With LocalBusiness and Recipe merged, the library now covers 12 of the ~26 Google Rich Results types. All initial shared sub-types and the first wave of parent types are done.

### Batch 2: Complex types work

LocalBusiness (16 properties, 4 new types) and Recipe (17 properties, 3 new types) are the most complex schemas dispatched so far. Both agents produced clean code on first attempt. Key design decisions:
- LocalBusiness extends TypedSchema directly rather than Organization (avoids constructor-promotion-through-inheritance complexity)
- Recipe.author supports `Person|Organization` union type
- DayOfWeek enum has 7 cases backed with schema.org URLs

### Pattern: Proactive merge when agents stall

Both batch 2 agents remained in draft state for 15+ minutes after their last commit push. Rather than waiting indefinitely, the orchestrator verified tests locally (62 pass, 0 cs-fix issues) and proactively marked the PRs ready → merged. This is a valid optimization — the code quality was confirmed, and waiting for the agent to self-unmark draft adds no value.

### Total cycle output

4 merged PRs, 13 new types (Article, NewsArticle, BlogPosting, Event, Place, EventStatusType, LocalBusiness, GeoCoordinates, OpeningHoursSpecification, DayOfWeek, Recipe, NutritionInformation, HowToStep), 30 new tests. This is the highest-throughput cycle to date.

---

## 2026-02-24 — Sixth Cycle

**Context**: Sixth cycle. Clean slate — no in-flight sessions. Two Eva input issues to handle.

### Orchestrator tooling built

Eva requested tools for two repetitive operations (#40, #42). Created three shell scripts in `tools/`:

1. **`tools/agent-status`**: Consolidates all PR/issue/agent status polling into a single command. Shows open agent issues, open PRs, copilot work events, CI checks, and concurrency count. Eliminates the 5-6 separate `gh api` calls that were needed before.

2. **`tools/update-state`**: Atomic operations for `docs/state.json` and worklog entries. Commands like `--add-implemented`, `--add-in-progress`, `--set-cycle`, `--worklog`, `--commit`. Prevents the state/worklog drift Eva noticed.

3. **`tools/dispatch-agent`**: Standardised issue creation with agent assignment. Takes `--title`, `--body-file`, `--model` and handles the `gh api` call with `agent_assignment` correctly.

**Observation**: The tools were written but couldn't be tested in this session due to sandbox restrictions on executing custom scripts. They'll be usable in subsequent GitHub Actions runs where the sandbox is less restrictive. The scripts follow standard bash patterns and should work correctly.

### Zero-revision streak continues

All 4 PRs this cycle passed on first attempt — no `@copilot` revision requests needed. This extends the streak from Cycle 5 (also 4/4 clean). The AGENTS.md improvements (tab indentation rule, cs-fix instruction, quality checklist) from Cycles 4-5 have been highly effective. The cost per type implementation is now consistently 1 premium request.

### 16 types milestone

With this cycle's 4 new types (VideoObject, SoftwareApplication, Movie, JobPosting), the library now covers 16 of the ~26 Google Rich Results types. We've passed the halfway point. The remaining types are increasingly niche.

### Agent timing is stable

All 4 agents this cycle completed in 8-10 minutes, consistent with Cycle 5 batch 1 (8-10 min). The complex types from Cycle 5 batch 2 (LocalBusiness, Recipe at ~25 min) appear to have been outliers. The typical implementation cycle is: initial plan (1 min) → implementation commit (6-8 min) → cs-fix commit (1 min) → done.

### Inheritance pattern confirmed

SoftwareApplication/MobileApplication/WebApplication followed the same inheritance pattern as Article/NewsArticle/BlogPosting. The agents consistently produce clean 3-line subclass files. This pattern is now well-established and doesn't need any additional AGENTS.md guidance.

### Remaining types analysis

The 10 remaining Google Rich Results types, roughly ordered by implementation feasibility:
- **Course list**: Straightforward. Needs Course type with Organization (exists).
- **Dataset**: Simple metadata type.
- **Q&A page**: Similar to FAQPage. Uses Answer (exists).
- **Review snippet**: Thin wrapper around existing Review/AggregateRating.
- **Employer aggregate rating**: Thin wrapper around AggregateRating + Organization.
- **Profile page**: Simple type.
- **Discussion forum**: Comment/post structure.
- **Vacation rental**: Complex property type.
- **Speakable**: Metadata annotation.
- **Math solver**: Niche, complex.
- **Carousel**: Meta-type wrapping other types.
- **Subscription/paywalled content**: Access metadata.
- **Education Q&A**: Extends Q&A pattern.

### Open questions

- Should we continue at 4 types/cycle pace, or slow down to focus on quality improvements?
- Some remaining types (Math solver, Speakable, Carousel) may not map well to the simple TypedSchema pattern. Worth investigating before dispatching.

---

## 2026-02-24 — Seventh Cycle

**Context**: Seventh cycle. Clean slate. One Eva input (#52) about PR review workflow.

### New PR review workflow works well

Eva's input #52 was actionable: CI workflows only run on non-draft PRs, so the previous approach of reviewing while still draft was incorrect. Created a skill doc, a `tools/review-pr` script, and updated the checklist.

The corrected workflow (wait for agent → mark ready → CI runs → verify → merge) adds ~5 minutes per PR due to CI wait time, but this is largely overlapped with local test verification. The net impact on cycle time is minimal.

### Backward-compatible type modifications

QAPage required modifying existing Question and Answer types (adding optional properties). This was the first time an agent task required modifying existing types with data, not just structure. The agent handled it cleanly:
- Made `acceptedAnswer` optional in Question (was required)
- Added 7 new optional properties to Question
- Added 5 new optional properties to Answer
- All existing FAQPage tests passed without modification

This confirms that the "add optional properties" pattern works well for evolving types. The key design principle: never change a required property to a different type or remove it. Only add new optional properties with `= null` defaults.

### Dataset agent duration outlier

The Dataset task took 19 minutes (vs 5-8 for other tasks this cycle). This is consistent with the Cycle 5 observation that complex tasks with 4+ new types and modifications take longer. The Dataset spec had 4 new types, 12 tests, and a Place.php modification — the most complex single-task scope dispatched since Recipe (Cycle 5).

### 20 types milestone — approaching completion

With 20 of ~26 Google Rich Results types implemented, we've reached ~77% coverage. The remaining 9 types are increasingly niche:
- **Feasible soon**: Review snippet, Profile page, Discussion forum, Education Q&A
- **May need investigation**: Carousel (meta-type wrapping other types), Speakable (metadata annotation), Subscription/paywalled content (access metadata)
- **Complex/niche**: Math solver, Vacation rental

### Zero-revision streak: 12 consecutive clean PRs

Since adding the tab indentation rule and cs-fix instruction to AGENTS.md (Cycle 4), no PR has needed a revision request. This is a strong signal that the agent guidance (AGENTS.md + skills) is comprehensive enough for routine schema implementations.

### "Test and Build" CI still requires manual approval

The `claude-review` workflow runs automatically on non-draft PRs, but the "Test and Build" workflow still shows `action_required`. This is a GitHub Actions limitation for bot-authored PRs (needs manual approval for first-time contributors). Tests continue to be verified locally. Worth noting for Eva if she wants to adjust Actions settings.

### CI overlap pattern

The optimal review workflow emerged this cycle: while waiting for the `claude-review` CI check (4-6 min), verify tests locally. By the time local verification is done, CI is usually complete or nearly so. This eliminates the CI wait as a bottleneck.

### Open questions

- When should we stop prioritizing type count and shift to quality improvements (README, examples, comprehensive test coverage)?
- Carousel type is fundamentally different — it wraps other types in an ItemList. Does it need a different implementation pattern?
- Should we create an `input-from-eva` issue asking about the "Test and Build" workflow approval requirement?

---

## 2026-02-24 — Eighth Cycle

**Context**: Eighth cycle. Clean slate. No Eva input.

### Review Snippet is not a separate type

Discovered that "Review snippet" in Google's Rich Results list is not a standalone schema type. It's a usage pattern — you add Review or AggregateRating markup to supported parent types (Product, Recipe, LocalBusiness, etc.) and Google renders a "review snippet" in search results. Since Review and AggregateRating are already implemented, there's nothing new to build. Removed from the not-implemented list.

This reduced remaining types from 9 to 5 (including the similarly questionable "Image metadata" which just uses existing ImageObject).

### Backward-compatible type evolution scales well

This cycle's DiscussionForumPosting required enhancing both Comment (8 new properties) and Person (4 new properties). This is the most extensive backward-compatible modification attempted so far — previous cycles modified 1-2 properties at a time.

The agent handled it cleanly: all existing QAPage, FAQPage, Article, and Event tests passed without modification. The pattern (add optional params with `= null` defaults at constructor end) is now thoroughly validated. It works for single-property additions (eduQuestionType on Question) and for 8-property batch additions (Comment for Discussion Forum).

### Zero-revision streak: 15 consecutive clean PRs

The streak since Cycle 4's AGENTS.md improvements now spans 15 PRs across 5 cycles. No revision requests have been needed. This strongly validates the investment in AGENTS.md quality — the cost was ~1 hour of documentation work in Cycle 4, and the payoff is consistent first-attempt success on every PR.

### Approaching completion

With 23 of ~26 Google Rich Results types implemented (88% coverage), the remaining 5 types are increasingly niche:

1. **Speakable** — a metadata annotation (CSS/XPath selectors indicating which content is suitable for text-to-speech). Very thin type, may be trivial.
2. **Vacation rental** — complex (LodgingBusiness with 15+ properties, multiple sub-types). Worth implementing but expect it to be the most complex remaining type.
3. **Subscription/paywalled content** — access metadata on WebPage (isAccessibleForFree, hasPart with AccessPermission). Moderately complex.
4. **Carousel** — fundamentally different from other types. It wraps an ItemList of other types. May require a different implementation pattern or builder utility.
5. **Math solver** — very niche (MathSolver + MathExpression types). Low priority.

### Agent timing remains stable

All 3 agents this cycle completed in 6-8 minutes, continuing the pattern observed since Cycle 5. The standard implementation loop is: plan → implement → cs-fix → tests → done. No outliers this cycle.

### Open questions

- Should the next cycle focus on the remaining 5 types or pivot to quality improvements (README, usage examples, comprehensive integration tests)?
- Is Carousel worth implementing given its fundamentally different pattern?
- Should we investigate Speakable and Subscription more deeply before dispatching — they may turn out to be trivially thin types that add little value.

---

## 2026-02-25 — Ninth Cycle

**Context**: Ninth cycle. Clean slate. No Eva input. Research and implement all remaining feasible types.

### Near-complete Google Rich Results coverage achieved

This cycle implemented the last 4 feasible Google Rich Results types: Speakable, Carousel, Subscription/Paywalled Content, and Vacation Rental. The library now covers 27 of 28 types (96%). The only remaining type is Math Solver, which requires changes to the core `JsonLdGenerator` — a design decision deferred to Eva via question #78.

### Research before dispatch pays off

Spending time at the start of the cycle to thoroughly research all 5 remaining types was valuable:
- Speakable turned out to be trivially simple (as suspected)
- Carousel's ListItem modification worked cleanly because the existing `JsonLdGenerator` already handles `string|TypedSchema` union types
- Subscription/Paywalled Content was simpler than expected — just a thin WebPageElement + two Article properties
- Vacation Rental was the most complex but well within the agent's demonstrated capabilities
- Math Solver genuinely requires generator changes — confirmed by research, not assumed

This upfront research eliminated false starts and allowed optimal batching (no file conflicts between concurrent tasks).

### Batching by file conflicts is effective

The key insight this cycle: plan batches around file conflicts, not just complexity.
- Batch 1: Speakable (modifies Article) + Carousel (modifies ListItem) — no overlap
- Batch 2: Subscription (modifies Article) + Vacation Rental (new files only) — no overlap
- Speakable had to merge before Subscription could dispatch, since both modify Article

This constraint-based batching ensures zero merge conflicts without serializing everything.

### Zero-revision streak: 19 consecutive clean PRs

The streak now spans 5 cycles (Cycles 5-9). The combination of:
1. Comprehensive AGENTS.md with explicit style rules
2. cs-fix as a mandatory step
3. Well-researched issue specs with expected JSON-LD output

...produces consistent first-attempt success. The agent guidance infrastructure is mature.

### Math Solver is the first type that needs generator changes

Every previous type (27 of them) fit cleanly into the `TypedSchema` + `JsonLdGenerator` pattern. Math Solver is the first to require:
1. Array `@type` (`["MathSolver", "LearningResource"]`)
2. Hyphenated property names (`mathExpression-input`)

These are legitimate schema.org features used by other types beyond Math Solver. Supporting them would make the library more generally useful, but it's a core infrastructure change that should be Eva's call.

### Vacation Rental is the deepest nesting so far

VacationRental → Accommodation → BedDetails[]/LocationFeatureSpecification[]/QuantitativeValue — 4 levels of nesting. The agent handled it cleanly, and the JsonLdGenerator's recursive serialization worked correctly. This validates the architecture for even complex types.

### Agent timing: consistently 7-10 minutes

| Batch 1 | Batch 2 |
|---------|---------|
| Speakable: ~7 min | Subscription: ~8 min |
| Carousel: ~8 min | Vacation Rental: ~10 min |

All within the established 7-10 minute window. The Vacation Rental (4 new types, most complex) was at the upper end but not an outlier like LocalBusiness and Recipe in Cycle 5.

### Open questions

- What should the orchestrator focus on now that type implementation is essentially complete?
- Should we invest in README/documentation, integration tests, or new features?
- Will Eva want Math Solver implemented or not? The design decision is non-blocking for everything else.

---

## 2026-02-25 — Tenth Cycle

**Context**: Tenth cycle. Clean slate. No Eva input. Eva has not responded to #78 (Math Solver). Focus shifts from type implementation to quality improvement.

### Shift to quality improvement mode

With 27/28 types implemented and Math Solver blocked on Eva's decision, this cycle marks a transition from feature work to quality work. The audit at cycle start revealed:

1. **README severely outdated** — still says "only supports Product and Offer" despite 27 types and 65 schema classes being implemented. This is the most user-visible quality issue.
2. **21 schema classes lack dedicated tests** — many are tested indirectly (Product, Offer, Brand through JsonLdGeneratorTest), but sub-types like MobileApplication, WebApplication, Comment, CourseInstance, Schedule, Brand, and ListItem have no test files.
3. **composer.json keywords** reference "ProductGroup" (doesn't exist) and miss all the new types.

### Test coverage nuances

The initial audit flagged 21 classes as "untested", but deeper investigation showed the picture is more nuanced:
- **Product, Offer, BreadcrumbList, Brand, ListItem, MonetaryAmount, QuantitativeValue** etc. are all tested through `JsonLdGeneratorTest.php` — the original integration test that uses sample JSON files.
- **Enum classes** (ItemAvailability, OfferItemCondition, EventStatusType, DayOfWeek) are exercised by parent type tests.
- The genuinely untested classes are: MobileApplication, WebApplication, Comment, CourseInstance, Schedule, LocationFeatureSpecification, AdministrativeArea, and a few other simple sub-types.

### Dispatching non-code tasks to agents

This is the first time we've dispatched a documentation-focused task (README update, issue #80). The agent should handle this fine since it's just text editing, but it will be interesting to see if the quality of documentation writing matches the quality of code writing.

### Process observation: the orchestrator-as-architect pattern works

After 10 cycles, the workflow has matured significantly:
- **Cycles 1-3**: Setup, infrastructure, PAT issues, learning the ropes
- **Cycles 4-5**: Hit stride with 4 types/cycle, fixed AGENTS.md, established zero-revision pattern
- **Cycles 6-9**: Steady state, 4 types/cycle, tools built, near-complete coverage
- **Cycle 10**: Natural transition to maintenance/quality mode

The flywheel described in the system prompt is working: encounter problem → fix process → journal lesson → move on stronger. The zero-revision streak (19 PRs) validates that investment in AGENTS.md and skills compounds over time.

### Open questions

- Should we tag a release version? The library is substantially feature-complete.
- Is there value in adding more sample JSON-LD files in test/samples/?
- Should we create comprehensive integration tests that validate against Google's expected output format?
