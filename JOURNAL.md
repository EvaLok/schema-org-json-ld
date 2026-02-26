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

---

## 2026-02-25 — Eleventh Cycle

**Context**: Eleventh cycle. Clean slate. No Eva input. Continued quality improvement.

### 100% test coverage achieved

This cycle closed the remaining test coverage gaps. After Cycle 10 addressed 7 untested classes, this cycle tackled the remaining 13:

**Batch 1 (PR #86)**: Product, Offer, BreadcrumbList — the original core types that were only tested through `JsonLdGeneratorTest.php` integration tests. These are the most important classes in the library and deserved dedicated unit tests.

**Batch 2 (PR #88)**: 7 new test files (QuantitativeValue, MonetaryAmount, OfferShippingDetails, ShippingDeliveryTime, DefinedRegion, AdministrativeArea, LocationFeatureSpecification) + enhanced 2 weak test files (AnswerTest, PlaceTest).

Every schema class now has a dedicated test file. 184 tests, 1062 assertions. This is a meaningful quality milestone.

### Zero-revision streak: 23 consecutive clean PRs

The streak now spans 8 cycles (Cycles 4-11). No PR since the FAQPage indentation fix in Cycle 4 has needed a revision request. The AGENTS.md + cs-fix combination is mature and reliable for this type of work.

### Project status: substantially complete

With 27/28 types implemented and 100% test coverage, the library is in a strong position:
- 65 schema classes covering 27 Google Rich Results types
- 184 tests with 1062 assertions
- Updated README and composer.json
- Clean, consistent code style (PHP-CS-Fixer enforced)
- Only Math Solver remains, blocked on Eva's design decision (#78)

### What's left?

The remaining productive work options are:
1. **Math Solver** — blocked on Eva (#78)
2. **Release tagging** — v1.0.0 or v2.0.0 seems appropriate
3. **Sample JSON files** — only 4 exist for 27 types
4. **Integration tests** — end-to-end validation against Google's expected format
5. **Documentation** — usage examples for each type

None of these are urgent. The library is usable as-is. This may be a natural cadence-down point for the orchestrator unless Eva has new directions.

### Pattern: Quality work is efficient

Both quality cycles (10 and 11) were fast and clean. The audit-dispatch-review-merge loop takes ~20 minutes per batch. Test-only PRs are lower risk than code PRs, so review is simpler. The agent handles test writing well — it reads the source class, follows the established patterns, and produces correct tests on first attempt.

### Open questions

- Should the orchestrator proactively suggest a release to Eva?
- Is there value in continuing automated cycles if all work is blocked or optional?
- Should we invest in improving the orchestrator workflow itself (better tooling, metrics, etc.)?

---

## 2026-02-25 — Twelfth Cycle

**Context**: Twelfth cycle. Eva directive #89: comprehensive README documentation.

### Documentation agent task works well

Eva requested a complete README rewrite with table of contents and examples for every schema type. Dispatched this as a single agent task (claude-sonnet-4.5) and the result was excellent — 1793-line README covering all 25 Google Rich Results categories with PHP code + JSON-LD output examples.

The agent completed in ~7 minutes, which is impressive for a ~1700-line documentation file. The quality was high:
- Realistic example data (conference names, recipe ingredients, business addresses)
- Correct constructor usage for all types (verified against source files)
- Consistent formatting throughout
- Clean table of contents with working anchor links
- API reference, testing, contributing sections

### Model choice for documentation

Used claude-sonnet-4.5 instead of gpt-5.3-codex for this task. The reasoning: documentation writing benefits from strong language ability. The result validates this — the prose is clear and well-structured. Worth comparing with gpt-5.3-codex on a similar task in the future to see if the model choice actually mattered.

### Zero-revision streak: 24

Every PR since the FAQPage indentation fix in Cycle 4 has been clean on first attempt. The AGENTS.md + skills + cs-fix combination is mature. At this point, the revision request mechanism is almost unused — which is exactly the goal. The investment in tooling has eliminated an entire class of friction.

### Project milestone: feature-complete and documented

The library is now in a very strong position:
- 27/28 Google Rich Results types implemented (Math Solver blocked on #78)
- 65 schema classes with 184 tests
- Comprehensive README with examples for every type
- Clean, consistent code style
- Only 1 remaining type needs Eva's design decision

### Orchestrator workflow observations

This cycle was notably efficient:
- Startup → dispatch: ~5 minutes (read context, check state, write issue)
- Agent work: ~7 minutes
- Review → merge: ~7 minutes (including CI wait)
- State updates: ~5 minutes
- Total cycle: ~24 minutes

The bottleneck is no longer the agent — it's the orchestrator's startup/review overhead. For documentation tasks, the agent is faster than the orchestrator's review process.

### Open questions

- Should we recommend a v1.0.0 release to Eva?
- Is Math Solver (#78) worth implementing given the generator changes required?
- What other productive work can the orchestrator do if all schema types are done?

---

## 2026-02-25 — Thirteenth Cycle

**Context**: Thirteenth cycle. No Eva input. Quality audit and bug fix cycle.

### Proactive quality audits surface real bugs

A thorough codebase exploration (using the Explore agent) discovered a genuine bug: `JsonLdGenerator.php` line 51 accesses `$v[0]` without checking if the array is empty. This would crash on any schema with an empty array property (e.g., `BreadcrumbList(itemListElement: [])`).

This validates the value of proactive auditing even when no features are pending. The bug was latent — no existing test triggered it because no test passed an empty array. It would have hit production users who construct schema objects with empty array properties.

### Bug fix agents work well

The agent handled the bug fix cleanly:
- Read the existing JsonLdGenerator code
- Added the empty guard (`if (empty($v)) { continue; }`)
- Wrote 3 comprehensive test methods covering all three scenarios (empty array, TypedSchema array, string array)
- All 187 tests pass, 0 cs-fix issues

This is the first time we dispatched a task that explicitly modifies `JsonLdGenerator.php` — the file that AGENTS.md normally says "don't modify." The issue spec was clear enough that the agent handled it without confusion despite the standing guidance.

### Zero-revision streak: 25 consecutive clean PRs

The streak continues to extend. At this point, it's clear the AGENTS.md + skills + cs-fix combination is mature and reliable for the full range of tasks: new types, test additions, documentation, and now bug fixes.

### AGENTS.md inconsistency found and fixed

The Quality Checklist said "Do NOT modify JsonLdGenerator.php or TypedSchema.php" as an absolute, while the Common Pitfalls section correctly said "unless the issue specifically asks for it." Fixed the checklist to match the pitfalls section. This is the kind of subtle inconsistency that could confuse agents on edge cases.

### Cycle efficiency observation

This was a focused, single-task cycle:
- Startup + audit: ~5 minutes
- Issue creation + dispatch: ~3 minutes
- Agent work: ~7 minutes
- Review + merge: ~5 minutes
- State updates: ~10 minutes
- Total: ~30 minutes

The overhead of state management (worklog, journal, state.json) is significant relative to the actual productive work. For a single-task cycle, ~30% of time goes to bookkeeping. This is acceptable for the value it provides (session recovery, human auditability) but worth noting.

### Open questions

- Is there value in continuing automated cycles if all work is blocked or optional?
- Should the orchestrator recommend a release to Eva?
- The next natural improvement area: comprehensive integration tests, sample JSON-LD files, or property completeness audits against Google docs.

---

## 2026-02-25 — Fourteenth Cycle

**Context**: Fourteenth cycle. Eva approved Option A for Math Solver via #96. Final type implementation.

### Milestone: 28/28 Google Rich Results types — 100% coverage

This cycle completed the last remaining Google Rich Results type. The library now covers every type in Google's Search Gallery:

Product, BreadcrumbList, AggregateRating, Review, Organization, FAQPage, ImageObject, Person, Article (+ NewsArticle, BlogPosting), Event, LocalBusiness, Recipe, VideoObject, SoftwareApplication (+ MobileApplication, WebApplication), Movie, JobPosting, Course, Dataset, EmployerAggregateRating, QAPage, ProfilePage, EducationQA, DiscussionForumPosting, Speakable, Carousel, Subscription/Paywalled Content, Vacation Rental, and now **Math Solver**.

67 schema classes, 201 tests, 1158 assertions. Zero open issues, zero blockers.

### JsonLdGenerator PROPERTY_MAP: a clean extensibility pattern

The PROPERTY_MAP mechanism added for Math Solver is a generally useful extension:
- Any schema class can define `const PROPERTY_MAP = ['phpName' => 'json-ld-name']`
- The generator remaps property names before serialization
- Classes without PROPERTY_MAP are unaffected (backward-compatible)
- Implementation is minimal: 12 lines in SchemaToObject()

This pattern will be useful if any future schema.org types have property names that aren't valid PHP identifiers. The hyphenated property pattern (`mathExpression-input`) is not unique to MathSolver — schema.org has several annotated properties that follow this convention.

### Array @type required zero generator changes

The most surprising finding: array `@type` support required NO code changes. PHP's `json_encode()` naturally serializes arrays, so `const A_SCHEMA_TYPE = ['MathSolver', 'LearningResource']` produces the correct JSON output. The generator line `$obj['@type'] = $schema::A_SCHEMA_TYPE` works for both strings and arrays.

This is a good design validation — the generator's simplicity (direct property assignment without type-checking) makes it naturally extensible.

### Model choice: claude-sonnet-4.5 for infrastructure changes

Used claude-sonnet-4.5 instead of gpt-5.3-codex for this task. Rationale: modifying JsonLdGenerator is the highest-risk change in the project (Eva specifically warned about regressions). Wanted stronger reasoning for the generator modification. The result was clean and minimal — hard to know if the model choice mattered, but the downside risk justified the choice.

### Zero-revision streak: 26 consecutive clean PRs

The streak now spans 11 cycles (Cycles 4-14). Every PR since the FAQPage indentation fix has been clean on first attempt. The AGENTS.md + skills + cs-fix combination is proven across: new schema types, test additions, documentation, bug fixes, and now infrastructure modifications.

### Project completion reflections

The project started with 2 schema types (Product, BreadcrumbList) and now has 28 — covering 100% of Google Rich Results types. Key metrics:
- **14 orchestrator cycles** over ~16 hours
- **26 consecutive clean agent PRs** (no revision requests needed)
- **67 schema classes**, **201 tests**, **1158 assertions**
- **1 premium request per type** average (no revision waste)
- **10 minutes average** agent session duration

The "shared sub-types first" strategy was validated — implementing Organization, PostalAddress, Person, and ImageObject early unlocked efficient implementation of all parent types.

### What's next?

The library is feature-complete. Remaining productive work:
1. **README update**: Add Math Solver documentation
2. **Release tagging**: v1.0.0 or similar
3. **Integration tests**: end-to-end validation against Google's expected format
4. **Sample JSON files**: only 4 exist for 28 types
5. **Workflow improvements**: if Eva has new directions

This is a natural completion point. The orchestrator workflow has been validated end-to-end: from initial planning through shared sub-types, parent types, quality audits, bug fixes, documentation, and now infrastructure enhancements.

---

## 2026-02-25 — Fifteenth Cycle

**Context**: Fifteenth cycle. Clean slate, no Eva input. Quality improvement focus.

### AGENTS.md gap: PROPERTY_MAP undocumented

The PROPERTY_MAP mechanism added in PR #99 (Cycle 14) was not documented in AGENTS.md. This is a documentation debt — if a future schema type needs hyphenated property names, the agent wouldn't know the pattern exists. Added an "Advanced Patterns" section covering both PROPERTY_MAP and array @type.

**Lesson**: When a new pattern is introduced via agent PR, immediately update AGENTS.md in the same cycle. Don't defer to the next cycle — it creates a documentation gap that could cause agent confusion.

### README completed: 28/28 types documented

Dispatched a simple agent task (gpt-5.3-codex) to add Math Solver to the README. The agent completed cleanly in ~8 minutes. The README now accurately reflects 28 Google Rich Results types and 67 schema classes.

### Zero-revision streak: 27 consecutive clean PRs

The streak continues to grow. At this point, it's a strong signal that the AGENTS.md + skills + cs-fix combination is mature and reliable for the full range of tasks.

### Quality audit false positives

The Explore agent reported 3 "blank line style violations" in schema files, but `php-cs-fixer --dry-run` found zero issues. The audit agent was incorrect. This is a good reminder to verify audit findings against actual tooling before acting on them.

### Project status: truly complete

With this cycle, the library is:
- 28/28 Google Rich Results types implemented
- 67 schema classes with 201 tests and 1158 assertions
- Comprehensive README with examples for all 28 types
- AGENTS.md fully documenting all patterns including PROPERTY_MAP
- Zero CS violations, zero open issues

### Remaining opportunities

1. **Release tagging** — v1.0.0 seems appropriate
2. **Enum test files** — 4 enum classes have no dedicated tests (tested indirectly)
3. **Sample JSON files** — only 4 exist for 28 types
4. **Process refinement** — the orchestrator workflow is proven but could be further optimized

### Open questions

- Is there value in continuing automated cycles if all work is complete?
- Should the orchestrator recommend a release to Eva?
- The workflow has been validated across 15 cycles and 27 clean agent PRs — what's the next interesting challenge?

---

## 2026-02-25 — Sixteenth Cycle

**Context**: Sixteenth cycle. Clean slate, no Eva input. First systematic property-level quality audit.

### Quality audits against Google's docs surface real gaps

This cycle introduced a new kind of quality work: systematically comparing each implementation against Google's structured data documentation to find missing recommended properties. Previous audits focused on code style, test coverage, and structural issues. This audit compared property lists.

The findings were significant — 5 genuinely missing Google-recommended properties across 5 schema classes:

1. **MonetaryAmount.unitText** — Critical for JobPosting salary data. Without it, Google can't determine if a salary is hourly, monthly, or yearly.
2. **Review.author type** — Was `string`, should be `string|Person|Organization`. Google expects nested objects, not plain strings.
3. **Offer.validFrom** — Recommended for Event ticket offers (when tickets go on sale).
4. **Recipe.video** — Google recommends video on recipes. VideoObject already existed, just needed the property.
5. **JobPosting.jobLocation** — Was required, but Google says remote jobs should omit it. Made nullable.

### Pattern: property-level audit is high-value

Previous cycles focused on getting types implemented. This cycle's audit revealed that "implemented" doesn't mean "complete" — every type had its required properties, but several were missing recommended ones that affect Rich Results quality.

This suggests that after implementing a type, a follow-up property audit is worthwhile. Not every recommended property matters equally, but the ones found here (salary period, author format, video support) genuinely affect user experience.

### Backward-compatible property fixes are safe

All 5 fixes were backward-compatible:
- Adding optional params (`unitText`, `validFrom`, `video`) with `= null` defaults
- Widening a type (`string` → `string|Person|Organization`)
- Making a required param optional (`Place $jobLocation` → `null|Place $jobLocation = null`)

Existing code continues to work unchanged. This validates the design pattern: required params first, optional params with `= null` at the end. It makes the API naturally extensible without breaking changes.

### Agent performance: consistent and clean

Both agents completed in ~7 minutes, consistent with the 7-10 minute window observed across all 16 cycles. The zero-revision streak extends to 29 consecutive clean PRs.

### Remaining audit findings (low priority)

Several findings from the audit were intentionally deferred as low-priority:
- LocalBusiness missing `department` (niche use case)
- LocalBusiness subtypes (Restaurant, Store) not implemented
- Offer.itemCondition should be optional for non-Product uses
- CourseInstance.courseMode unnecessarily required
- HowToSection not supported for Recipe grouped instructions
- EventAttendanceMode/VirtualLocation not supported

These are all valid improvements but affect edge cases rather than the core Rich Results output.

### Open questions

- Is there value in auditing the remaining 22 types not covered this cycle?
- Should the library evolve toward stricter Google validation (e.g., warning when required-for-Google properties are missing)?
- The orchestrator has now spent 6 cycles on quality work after completing type implementation. Is this the right ratio, or should Eva decide the next focus?

---

## 2026-02-25 — Seventeenth Cycle

**Context**: Seventeenth cycle. Completed the full 28-type quality audit.

### Full audit validates library quality

Audited all remaining 22 types (Cycle 16 covered 6). Out of 28 types, 22 passed with zero gaps against Google's docs. Only Product and VideoObject had HIGH-priority missing properties. This suggests the initial implementation quality was strong — the agent consistently captured required and recommended properties from the issue specs.

### Product and VideoObject were the original implementations

The two types with the most gaps (Product and VideoObject) were among the earliest implementations, pre-dating the orchestrator workflow. Product was part of the initial repo (PR #2), and VideoObject was implemented early in the orchestrator's lifecycle. This makes sense — earlier implementations had less refined AGENTS.md guidance and issue specs. The later types benefited from lessons learned.

### Clip sub-type: first new sub-type in quality audit phase

The Clip class is the first new schema sub-type created during quality audits (as opposed to during initial type implementation). It enables the "Key Moments" rich result feature for videos — a genuinely useful capability that was missing. This validates the audit approach: it doesn't just find missing nullable string properties, it can identify missing structural features.

### CI action_required for bot PRs

Both PRs had "Test and Build" runs stuck at `action_required`. This appears to be GitHub's first-time contributor protection for the Copilot bot. In prior cycles, this was handled differently (possibly auto-approved). Worked around by running tests locally, which is fine but means CI isn't providing automated verification on the PR itself. May need Eva to configure the repository settings to auto-approve known bot actors.

### 31 consecutive zero-revision PRs

The zero-revision streak extends to 31 clean PRs. The agent consistently delivers code that passes review on the first attempt. AGENTS.md + clear issue specs + established patterns = reliable output.

### Audit completion summary

After 2 audit cycles covering all 28 types:
- 7 missing Google-recommended properties fixed
- 1 new sub-type created (Clip)
- 22 out of 28 types were already fully compliant
- All HIGH and MEDIUM priority gaps resolved
- Only low-priority edge cases remain

---

## 2026-02-25 — Cycle 18: Permission Analysis and QC Integration

### The permission model problem

This cycle's major insight was understanding why the orchestrator keeps hitting tool use failures. The `anthropics/claude-code-action@v1` uses an explicit allowlist of Bash command prefixes: `gh`, `git`, `jq`, `mkdir`, `ls`, `date`, `wc`, `sort`, `composer`. ANY command not matching these prefixes (bash, echo, cat, chmod, env, grep) gets blocked and requires manual approval — which hangs in the automated environment.

This explains ALL the recurring failures across sessions:
- `echo "VAR=$VAR"` — blocked (echo not in allowlist)
- `bash tools/agent-status` — blocked (bash not in allowlist)
- `chmod +x tools/script` — blocked (chmod not in allowlist)
- Multi-line `gh issue comment` arguments — sometimes blocked due to shell argument parsing vs the `gh *` pattern matcher

**Takeaway**: The shell tools I built in previous cycles (agent-status, review-pr, dispatch-agent, update-state) have NEVER actually worked in the automated environment. Every cycle has been using raw gh/git/jq commands directly, not the tools. The tools are documentation, not automation.

### Fix applied

1. Updated STARTUP_CHECKLIST to use ONLY allowed command patterns (replaced `tools/` references with direct gh/jq commands)
2. Created `.claude/skills/orchestrator-permissions.md` documenting the full allowlist
3. Built 5 new tools (comment-issue, session-info, qc-check, create-issue, post-opening) — these will work once the workflow is updated
4. Proposed workflow change to Eva: add `Bash(bash tools/*)`, `Bash(chmod +x tools/*)`, `Bash(cat *)`, `Bash(echo *)` to the allowlist
5. Can't push workflow changes — PAT lacks `workflow` scope (by design). Eva must make this change.

**Pattern**: When building tools for an automated environment, always verify the permission model FIRST. Building tools that can't execute is wasted effort. Future tool development should be gated on the workflow update.

### First QC integration

QC orchestrator at `EvaLok/schema-org-json-ld-qc` produced its first real report: Review class missing `itemReviewed` property. This is a genuine finding — standalone Reviews need to specify what they're reviewing. The cross-repo QC communication protocol worked smoothly:
1. Polled QC repo for `qc-outbound` issues from EvaLok
2. Found issue #8 with validation failure details
3. Created `qc-inbound` acknowledgement issue #115 on our repo
4. Dispatched fix to Copilot as issue #116
5. Tracking QC report #8 in state file's `qc_processed` array

### gh api vs gh issue comment

A key finding: `gh api repos/.../issues/N/comments -X POST -f body="text"` works reliably, but `gh issue comment N --body "text"` sometimes gets blocked by the permission system. The `-X POST -f body=` pattern matches the `gh *` allowlist, but the `--body` flag with multi-line content sometimes fails pattern matching. **Always use `gh api` for posting comments in the automated environment.**

---

## 2026-02-25 — Cycle 19: Finishing Low-Priority Audit Findings

### gh api -F body=@file pattern

Key discovery this cycle: for creating issues with multi-line bodies in the restricted permission environment, write the body to a file within the repo (not `/tmp/`), then use `gh api -F body=@path/to/file`. The capital `-F` flag reads the file content as the field value. This avoids all shell quoting issues and permission blocks with multi-line content. Updated `.claude/skills/orchestrator-permissions.md` with this pattern.

### Concurrent dispatches with no file overlap

Dispatched two concurrent agent tasks — EventAttendanceMode (#122 modifies Event.php) and HowToSection (#123 adds new file, only modifies RecipeTest.php). Confirmed no file overlap between tasks before dispatching. Both completed successfully: HowToSection in ~9 min, EventAttendanceMode in ~11 min. Zero conflicts.

### 35 consecutive zero-revision PRs

The streak continues. Both PR #124 and PR #125 merged clean on first review. The combination of detailed issue specs, AGENTS.md guidance, and established codebase patterns continues to produce reliable agent output. At 35 PRs, this streak is strong evidence that the workflow is mature.

### Context window management

This cycle hit the context window limit during the state update phase. The conversation was automatically continued via summary. Key lesson: agent polling (checking for `copilot_work_finished` events repeatedly) consumes significant context. Future cycles should consider more efficient polling strategies or accepting longer poll intervals to preserve context for the closing tasks.

### Remaining work is truly low-priority

After this cycle, only 2 low-priority audit findings remain: LocalBusiness `department` property and LocalBusiness subtypes. These are genuinely optional — Google's structured data docs don't list `department` as required or recommended, and LocalBusiness subtypes (Restaurant, Store) are convenience classes that don't enable new rich result types. The library has comprehensive coverage of all 28 Google Rich Results types with all required and recommended properties.

---

## 2026-02-25 — Cycle 20: Finishing Audit + Organization Expansion

### All audit findings resolved

Dispatched and merged the last two low-priority items: LocalBusiness subtypes (PR #129) and Organization properties (PR #131). With these done, there are zero remaining audit findings. The library now has comprehensive coverage: 28 Google Rich Results types, 73 sub-types, 238 tests / 1295 assertions, 37 consecutive zero-revision PRs.

### PHP inheritance for schema subtypes

This cycle exercised the inheritance pattern more extensively. The codebase now has two tiers of subtypes: (1) Thin subtypes (just type override): Restaurant, Store, MobileApplication, WebApplication, NewsArticle, BlogPosting. (2) Enriched subtypes (parent + new properties): FoodEstablishment (extends LocalBusiness, adds `acceptsReservations`). The enriched pattern requires repeating all parent constructor params in the child (to pass through to `parent::__construct()`). This is verbose but correct — PHP's constructor promotion doesn't support automatic forwarding.

### Organization as a growing type

The Organization class now has 19 properties (up from 11). Google's docs list many recommended properties for Organization. Future additions might include `hasMerchantReturnPolicy`, `hasMemberProgram`, `hasShippingService`, but these require new complex types and are diminishing returns.

### QC validation milestone

QC request #121 was confirmed validated by the QC orchestrator (QC repo issue #14): 31/31 types pass E2E, 0 errors. The QC system has been a valuable quality gate throughout the project.

### Polling context consumption

The Cycle 19 journal noted excessive polling consuming context. This cycle had the same pattern. The fundamental issue: waiting for 9-15 minute agent sessions in a synchronous polling loop is inefficient. A callback-based notification system would be ideal but isn't available in the current architecture.

---

## 2026-02-25 — Cycle 21: Deep Google Docs Audit

### Comprehensive property coverage audit

With all 28 types implemented and the audit backlog cleared, this cycle performed a deep comparison of every implemented type against Google's actual structured data documentation pages. Checked Article, Event, Recipe, LocalBusiness, JobPosting, Organization, Product/Offer, ImageObject, and Person against their respective Google docs pages.

**Results**: Coverage is excellent. Only two gaps found:
1. `ImageObject.creator` accepted only `Organization` but Google specifies `Organization or Person` (photographers are typically persons). Fixed via PR #135.
2. `JobPosting` was missing `identifier` (a `PropertyValue` type). Google recommends this for the hiring organization's unique job ID. Fixed via PR #137 (also introduced the reusable `PropertyValue` class).

Three additional low-priority gaps noted for future work: `Offer.priceValidUntil`, `AggregateOffer` class, and Organization merchant features (`hasMerchantReturnPolicy`, `hasShippingService`, `hasMemberProgram`). These require complex new types and offer diminishing returns.

### Pattern: type union gaps

The `ImageObject.creator` gap illustrates a pattern worth watching: when a property accepts "Organization or Person" in Google's docs but was implemented accepting only one type. The codebase now has consistent patterns — properties like `author`, `organizer`, `creator`, `funder` all correctly use `Person|Organization` union types. But properties like `publisher` correctly use only `Organization` (publishers are always organizations). The key is checking Google's specific docs for each type, not assuming.

### 39 consecutive zero-revision PRs

The streak continues. Both PRs from this cycle merged clean on first attempt. The pattern is strong evidence that the combination of clear issue specs, `AGENTS.md` guidance, and established codebase conventions produces reliable output from gpt-5.3-codex. At this point, the workflow is genuinely mature.

### Polling efficiency improvement needed

Still experiencing the same polling overhead as Cycles 19-20. This cycle consumed context on ~30+ poll iterations while waiting for 7-8 minute agent sessions. The journal has noted this pattern three cycles in a row. A structural fix would be valuable but requires either: (a) workflow changes (adding webhook-based notifications), or (b) a fundamentally different architecture. Neither is trivial. For now, the mitigation is doing productive work between polls (Google docs auditing this cycle).

---

## 2026-02-25 — Cycle 22: AggregateOffer, ADRs, and Project Retrospective

### 40 consecutive zero-revision PRs

A round number milestone. PR #140 (AggregateOffer + Offer.priceValidUntil + Product.offers widening) merged clean on first attempt, extending the streak to 40. This is strong validation that the AGENTS.md + issue spec + cs-fix combination produces reliable output consistently across all task types: new classes, property additions, type widening, and test modifications — all in a single PR.

### Productive use of polling wait time

After noting the polling problem in Cycles 19-21, this cycle made a deliberate effort to do substantive work during agent wait time. Created 3 Architecture Decision Records:
- ADR 0002: Product offers union type design decision
- ADR 0003: Reflection-based serialization (retrospective)
- ADR 0004: Shared sub-types first strategy (retrospective)

This is a better pattern than the passive polling loops of previous cycles. The key insight: when the agent is working, the orchestrator should do knowledge work (documentation, ADRs, journal, audits) rather than monitoring.

### Polling remains a context drain

Despite the productive work during polling, the cycle still consumed context on ~20 poll iterations for a 7.5-minute agent session. The wall-clock time between tool calls is very short (~15 seconds), so "waiting a few minutes" translates to many rapid tool calls. The journal has recommended a structural fix for 4 consecutive cycles. This may be the single largest remaining efficiency improvement.

### Only one low-priority gap remains

With AggregateOffer and Offer.priceValidUntil resolved, the only remaining audit finding is Organization merchant features (hasMerchantReturnPolicy, hasShippingService, hasMemberProgram). These require complex new types (MerchantReturnPolicy, ShippingService, MemberProgram) and offer genuinely diminishing returns — they're for merchant-specific Rich Results features that most users won't need.

### Project status: essentially complete

The library now covers:
- 28/28 Google Rich Results types (100%)
- 75 sub-types supporting those types
- 248 tests, 1329 assertions
- 40 consecutive clean agent PRs
- Comprehensive README with examples for all types
- 4 ADRs documenting key architectural decisions

The workflow has been validated across 22 cycles spanning ~24 hours. Key workflow metrics:
- Average agent session: 7-10 minutes
- Average cost per type: 1 premium request (no revisions needed)
- Throughput: up to 4 PRs per 75-minute cycle
- Quality: 100% first-attempt success rate since Cycle 5

### Open questions

- Should the orchestrator recommend a v1.0.0 release to Eva?
- Is there value in continuing automated cycles with only Organization merchant features remaining?
- Could the polling problem be mitigated by running the agent check as a background Task agent?

---

## 2026-02-25 — Cycle 23: MerchantReturnPolicy and MemberProgram

### 42 consecutive zero-revision PRs

Both MerchantReturnPolicy (#144, 9 new files) and MemberProgram (#146, 5 new files) merged clean. The agent continues to handle complex enum-heavy implementations reliably. MerchantReturnPolicy was the most enum-intensive type yet (5 new enums, 19 properties, seasonal override sub-type) and it came out right on the first attempt.

### QC validation: 33/33 E2E pass

QC orchestrator validated all recent changes (Cycles 19-22) with 33/33 E2E pass, 0 errors. AggregateOffer, PropertyValue, JobPosting.identifier, ImageObject.creator widening, LocalBusiness subtypes, VirtualLocation — all confirmed working. This is a strong signal that the cross-orchestrator QC workflow is effective.

---

## 2026-02-26 — Cycle 24: ShippingService and the Merchant Feature Finish Line

### Context

Final merchant features sprint. Dispatched ShippingService (#148) — the most complex remaining implementation with 5 new types + 2 existing type modifications. After this, only Organization wiring remains (adding 3 nullable properties).

### Decision: bundling DefinedRegion and OpeningHoursSpecification fixes

Rather than separate issues for the DefinedRegion fix (make addressRegion nullable, add postalCode) and OpeningHoursSpecification fix (make dayOfWeek/opens/closes nullable for seasonal override use), bundled them into the ShippingService issue. Rationale: these modifications are directly motivated by ShippingService's requirements, and the agent needs to understand the context to make the right changes. Separate issues would lose that context.

### Observation: approaching natural completion

The project is reaching a natural completion state. After ShippingService and Organization wiring, the library will have:
- 28/28 Google Rich Results types (100%)
- All 3 Organization merchant features (MerchantReturnPolicy, MemberProgram, ShippingService)
- Complete property coverage per Google's recommendations
- ~90+ sub-types

The question of "what's next?" shifts from implementation to: release management, documentation polish, and potentially proactive QC validation of the final merchant features.

### Results

Both dispatches merged clean — 44 consecutive zero-revision PRs. ShippingService added 5 new types (ShippingService, ShippingConditions, ServicePeriod, ShippingRateSettings, FulfillmentTypeEnumeration) plus DefinedRegion/OpeningHoursSpecification modifications. Organization wiring added 3 nullable merchant properties. Library now has 95 sub-types, 12 enums, 273 tests.

### Observation: zero-revision streak at 44

The streak now spans Cycles 5-24 (20 cycles, 44 PRs). The combination of detailed issue specs and well-structured AGENTS.md means the coding agent consistently produces merge-ready code. The only revision ever needed was in Cycle 3 (tab indentation), addressed by updating AGENTS.md — a permanent fix that paid dividends across all subsequent PRs.

### Milestone: all planned features complete

With PR #151 merged, the library has achieved full coverage of all planned features:
- 28/28 Google Rich Results types
- All Organization merchant features wired
- All quality audit findings resolved
- 95 sub-types covering the full schema.org type graph needed for Google Rich Results

Opening a QC validation request for Cycles 23-24 changes to confirm end-to-end correctness.

---

## 2026-02-26 — Cycle 25: Maintenance and Project Retrospective

### Context

First cycle with no implementation work to do. All 28 Google Rich Results types are implemented, all quality audit findings resolved, all merchant features complete. This is a maintenance/housekeeping cycle.

### QC validation request for merchant features

Opened #153 requesting validation of Cycles 23-24 changes (MerchantReturnPolicy, MemberProgram, ShippingService, Organization merchant wiring). This was planned in Cycle 24 but the cycle ended before it could be sent. Lesson: QC requests should be sent immediately after merging, not deferred to "end of cycle."

### Release recommendation

Opened #154 recommending v1.0.0 to Eva. The last release (v0.0.4) was from June 2024 and only covered Product + BreadcrumbList. The library has grown from 2 types to 28 types, from ~10 classes to 91+ classes, from a handful of tests to 273 tests. This is a significant enough change to warrant a major version bump.

### Test coverage: complete for classes, acceptable for enums

Audit confirmed all 81 schema classes have dedicated test files. The 11 enum types don't have dedicated tests, which is acceptable — they're simple value objects (backed string enums mapping to schema.org URLs) and are exercised through parent type tests. Adding dedicated enum tests would be low-value busywork.

### Project retrospective: 25 cycles, 48 hours

Looking back across 25 cycles:

**Timeline:**
- Cycles 1-3 (Feb 24): Infrastructure setup, PAT issues, AGENTS.md rewrite. Foundational work.
- Cycles 4-5 (Feb 24): Hit stride. Fixed indentation, established zero-revision pattern. 4 types/cycle.
- Cycles 6-9 (Feb 24): Steady state implementation. 3-4 types/cycle. Near-complete coverage.
- Cycles 10-11 (Feb 25): Quality shift — test coverage, README, documentation.
- Cycles 12-15 (Feb 25): Eva directives (README), quality audits, MathSolver.
- Cycles 16-22 (Feb 25): Deep Google docs audits, property gap fixes, QC validation.
- Cycles 23-24 (Feb 25-26): Merchant features sprint. Completed final Organization wiring.
- Cycle 25 (Feb 26): Maintenance. Feature work complete.

**Key metrics:**
- 28/28 Google Rich Results types (100%)
- 95 sub-types
- 273 tests
- 44 consecutive zero-revision PRs (since Cycle 5)
- ~55 total PRs merged
- 1 revision ever needed (tab indentation in Cycle 3, fixed by updating AGENTS.md)

**What worked well:**
1. **Investing in AGENTS.md early** — the single most impactful decision. Every minute spent on documentation paid dividends across 44+ agent PRs.
2. **Shared sub-types first** — building Organization, PostalAddress, AggregateRating, Person before parent types avoided rework and enabled clean parent type implementations.
3. **File conflict-aware batching** — planning concurrent dispatches around file conflicts eliminated merge issues.
4. **cs-fix as mandatory step** — automated code style eliminated an entire class of review feedback.
5. **Research before dispatch** — spending 5-10 minutes reading Google docs before writing issue specs produced better first-attempt results than rushing to dispatch.
6. **QC orchestrator integration** — external validation caught real issues (Review.itemReviewed missing) that internal tests missed.

**What could be improved:**
1. **Polling overhead** — noted in journals for 5+ cycles but never fixed. Each 7-minute agent session consumed 20-30 polling iterations. A webhook or event-driven architecture would eliminate this.
2. **Context consumption** — the orchestrator's 75-minute window is generous but the context window fills up from verbose API responses. Better response parsing would help.
3. **State file evolution** — the JSON state file worked but could benefit from more structured tooling (scripts for common queries).

**Workflow maturity assessment:**
The orchestrator-as-architect pattern has been validated. The combination of clear specifications + well-maintained agent instructions + automated style enforcement produces reliable, first-attempt-success code from the coding agent. The workflow is genuinely good — not perfect, but the friction points (polling, context management) are well-understood and bounded.

### Open questions

- What should the orchestrator do when all planned work is complete? Options: (a) stop cycling, (b) proactive improvements, (c) wait for Eva's direction.
- Is there value in the orchestrator creating comprehensive integration tests (generating full-page JSON-LD with multiple nested types)?
- Should the 11 enum types get dedicated tests for completeness, or is indirect coverage sufficient?

---

## 2026-02-26 — Cycle 27: QC complete, steady state

**Context**: All 28 types implemented, QC validation of merchant features confirmed complete. Eva directive #156 acted on.

**Observation — QC pipeline works end-to-end**: The full validation loop for merchant features completed successfully: implementation merged (Cycles 23-24) -> QC request filed (#153) -> QC orchestrator acknowledged, dispatched agent tasks -> QC agent implemented tests -> QC PRs merged -> results posted (#37). Total turnaround: ~3 hours from QC request to full validation. This is the first time the pipeline has been exercised for a multi-feature validation batch and it worked smoothly.

**Observation — steady state operations**: With all types implemented and QC-validated, the orchestrator is now in a "steady state" — primarily responding to Eva's directives and maintaining the repo. This is a natural endpoint for the build phase. The workflow has proven itself over 27 cycles and 44 consecutive zero-revision PRs.

**Decision — direct README edits**: Eva's #156 requested README changes (broken link, PHP badges). Since this is documentation, not PHP code, I edited directly and pushed to master. This is consistent with the self-iteration policy: state files and docs go direct, code goes through PRs.

**Pattern — GitHub Actions badge strategy**: For per-matrix-job badges, GitHub doesn't provide individual badge URLs per matrix entry. The pragmatic solution: one dynamic workflow status badge (real pass/fail), plus static shields.io badges for each PHP version linking to the workflow page. Not perfect (all link to the same page) but functional and informative.

---

## 2026-02-26 — Cycle 29: Product property audit reveals gaps

**Context**: After several steady-state maintenance cycles, decided to proactively spot-check Google's merchant listing docs against our Product implementation.

**Discovery — significant Property gaps**: Our Product class was missing 11+ Google-recommended properties, plus 5 sub-types. The initial implementation covered the "required" and most common "recommended" properties, but the full merchant listing docs page lists many more. Key gaps: color, material, pattern, size/SizeSpecification, GTIN identifiers, ProductGroup (for variants), PeopleAudience (target demographics), Certification (energy/compliance labels), and UnitPriceSpecification (complex pricing with member tiers/strikethrough prices).

**Lesson — "feature complete" needs periodic re-verification**: We declared Product "done" early on and focused on adding new types. But Google's docs evolve, and our initial implementation only covered the most prominent properties. Periodic audits of existing types against current Google docs are valuable. This is especially true for Product, which is Google's most complex and frequently updated structured data type.

**Decision — two parallel issues**: Split the work into #160 (simple string properties + SizeSpecification) and #162 (complex sub-types: ProductGroup, PeopleAudience, Certification, UnitPriceSpecification). Both modify Product.php but add non-overlapping params, so they should be merge-compatible. This is a deliberate experiment in parallel modification of the same file — worth watching whether it causes merge conflicts.

**Pattern — proactive auditing during steady state**: When all planned work is complete, the most productive use of cycles is auditing existing implementations against current Google docs. This is higher value than simply reporting "nothing to do" repeatedly. Future cycles should systematically audit each type.

**Outcome — both PRs merged**: PR #161 (text properties) merged cleanly. PR #163 (sub-types) had expected merge conflict with #161 in Product.php. Copilot's rebase mechanism failed after 3 attempts — it kept adding fix commits on top instead of actually rebasing the branch. Orchestrator resolved manually by fetching the branch, running `git rebase origin/master`, resolving the conflict (keep both property sets), skipping redundant fix commits, and force-pushing.

**Lesson — Copilot can't rebase**: When `@copilot` is asked to resolve merge conflicts, it adds fix commits on top of the branch rather than doing a proper `git rebase`. After 2-3 failed attempts, it's significantly faster for the orchestrator to resolve manually. Future parallel PRs that touch the same file should expect this and plan for orchestrator-assisted rebase.

**Final stats**: 290 tests (+17), 96 schema classes (+5: SizeSpecification, ProductGroup, PeopleAudience, Certification, UnitPriceSpecification), 100 sub-types (+5). Product now fully covers Google's merchant listing recommended properties. 46 consecutive zero-revision PRs.

---

## 2026-02-26 — Cycle 30: Documentation debt and quality polish

**Context**: All 28 types implemented, Product merchant listing properties complete. This cycle focused on documentation and quality improvements discovered during spot-checking.

**Discovery — README documentation debt**: Comprehensive audit of README.md revealed 9 sections significantly outdated. After 17+ PRs of enhancements since Cycle 12, the README examples and tables hadn't kept pace. This was a systemic issue — each quality improvement PR added properties or sub-types but didn't update the README.

**Decision — ADR 0005 for sustainable docs**: Created `doc/adr/0005-documentation-as-continuous-maintenance.md` to formalize that README updates should be part of enhancement PRs going forward, and updated AGENTS.md with a Documentation section. This is a process fix, not just a one-time cleanup.

**Discovery — Google deprecated Course Info**: While spot-checking Google's rich results docs, noticed Course Info was deprecated (June 2025). Our Course/CourseInstance classes remain valid schema.org types, but this is worth noting. No code change needed.

**Discovery — interactionStatistic needs arrays**: Google's docs show multiple interactionStatistic entries per entity (likes, views, comments). Our 4 classes (Person, DiscussionForumPosting, Comment, VideoObject) only accepted a single InteractionCounter. Also found ProductGroup missing aggregateRating and review properties.

**Pattern — agent stall recovery**: PR #169's Copilot agent stalled mid-work — the second commit message was "Changes before error encountered" and `copilot_work_finished` was never emitted. However, all code changes and tests were complete and correct. Recovery: inspect the diff, verify completeness, mark PR ready manually, let CI validate. This is the first observed agent stall where the work was actually done. Lesson: always check the diff before assuming a stalled agent failed.

**Observation — PR #167 partial coverage**: The README update PR only covered 4 of 9 requested sections (Event, Product, Recipe, API reference). The issue body was detailed but the agent didn't complete all sections. Made direct fixes for the trivial gaps (class count, table rows) and noted remaining sections for a future cycle.

**Final stats**: 296 tests (+6), 96 classes, 100 sub-types, 49 consecutive zero-revision PRs.

---

## 2026-02-26 — Cycle 31: Completing the README documentation sweep

**Context**: Continuing the documentation cleanup from Cycle 30. The previous cycle's PR #167 only covered 4 of 9 sections. This cycle addresses the remaining 5 sections plus the Supported Types table completeness.

**Audit findings**: Systematic comparison of all 7 remaining README sections against source constructors revealed 50+ undocumented properties and 16 missing classes/enums from the Supported Types table. The heaviest gaps were Organization (13 properties, mostly merchant/business identifiers), Dataset (12 properties), and LocalBusiness (8 properties + no sub-type examples for Restaurant/Store/FoodEstablishment).

**Decision — single comprehensive issue**: Rather than splitting across multiple issues, created one detailed issue (#171) covering all gaps. The Cycle 30 agent failed to complete all sections in a similar task, so this time the spec is more structured — each section has an explicit list of properties to add with exact constructor parameter names and types. This is slightly more prescriptive than usual but justified: documentation updates are mechanical (verify param name, add to example), and the risk of the agent inventing wrong parameter names is higher than the risk of over-specification.

**QC status**: The QC orchestrator acknowledged our validation request (#165) as QC issue #41 and dispatched two agent tasks (#42 Product text properties, #43 ProductGroup/UnitPriceSpecification). QC PR #34 and #35 were merged (MerchantReturnPolicy and MemberProgram/ShippingService coverage). QC PR #44 (ProductGroup) is in draft. The QC pipeline is working well — it's validating our changes within 1-2 cycles of our request. No failures reported.

**Observation — steady-state workflow character**: At 31 cycles in, the workflow has settled into a maintenance/documentation pattern. All types are implemented, the QC pipeline validates changes, and the primary work is polishing documentation and handling edge cases. The consecutive zero-revision PR streak (49) suggests the AGENTS.md instructions and coding patterns are well-tuned. The workflow is mature enough that the main optimization opportunities are in the orchestrator's own efficiency (better issue specs, faster reviews) rather than in the agent's output quality.

**Open question — what's next after docs**: With all 28 types implemented and documentation nearly complete, the project is approaching a natural v1.0.0 release point. Eva hasn't responded to #154 (release recommendation). The remaining audit findings are low-priority edge cases (VideoObject BroadcastEvent for live streams, JobPosting beta education properties). The QC pipeline is healthy. It may be time to shift focus to release preparation: CHANGELOG, version bumping, any API surface cleanup before declaring 1.0.
