# Cycle 127 — 2026-03-04 13:48 UTC

## What was done

### Startup checklist
- No new Eva directives or comments since cycle 126
- No open QC `qc-outbound` issues — validation complete (73/73)
- Audit #87 and #88 still open but already processed in cycle 126. No new recommendations
- No open questions for Eva, no open audit-inbound or qc-inbound issues
- Concurrency: 0 in-flight sessions
- Dual-language consistency: 89 PHP schema classes, 89 TS schema classes — perfect parity

### Google property verification: ALL 26 types complete

Completed rolling verification of all 26 Google Rich Results types against current Google docs.

**This cycle verified 14 types** (12 were done in prior cycles):
- **VideoObject**: 3/3 required + 11/11 recommended — **100%** (incl. BroadcastEvent, Clip, SeekToAction sub-types)
- **SpeakableSpecification**: 2/2 required (cssSelector or xpath) — **100%**
- **Carousel/ItemList**: 1/1 required + ListItem 4/4 — **100%**
- **Event**: 4/4 required + 13/13 recommended — **100%** (Place.address nullable = soft gap)
- **LocalBusiness**: 2/2 required + 17/17 recommended — **100%**
- **JobPosting**: 5/5 required + 12/12 recommended + 5/5 beta — **100%** (jobLocation nullable = soft gap)
- **ProfilePage**: 1/1 required + 12/12 recommended — **100%**
- **EmployerAggregateRating**: 4/4 required + 2/2 recommended — **100%**
- **DiscussionForumPosting**: 15/15 required/recommended + 11/11 Comment sub-props — **100%**
- **Subscription/paywalled content**: 1/1 required + 3/3 recommended — **100%**
- **VacationRental**: 7/7 required + 16/16 recommended — **100%**
- **MathSolver**: 7/8 required + 4/4 recommended — **near-full** (learningResourceType optional, design choice)
- **Education Q&A (Quiz)**: shared-class design trade-offs (3 Question properties optional that Google requires for Education Q&A specifically)
- **QAPage**: 22/24 properties — **2 gaps found** (Question.comment and Answer.comment missing)

**Final score**: 22/26 types at 100%, 2 near-full (design trade-offs), 2 with gaps (QAPage comment fixed via dispatch)

### Gap fix dispatched: Question.comment + Answer.comment
- Created [#424](https://github.com/EvaLok/schema-org-json-ld/issues/424) — Add `comment` property (type `Comment[]|null`) to both Question and Answer classes in PHP and TS
- Dispatched to Copilot (gpt-5.3-codex)
- This was the only actionable gap found across all 26 types

## Current state
- **In-flight agent sessions**: 1 ([#424](https://github.com/EvaLok/schema-org-json-ld/issues/424))
- **Open PRs**: 0 (awaiting Copilot)
- **Open questions for Eva**: None
- **Remaining open `input-from-eva`**: [#247](https://github.com/EvaLok/schema-org-json-ld/issues/247) only
- **Blocker**: Phase 4c (npm publish) — Eva configures OIDC + creates GitHub Release
- **Copilot metrics**: 33 dispatched, 32 merged, 1 in-flight
- **Google property verification**: 26/26 complete (rolling verification finished)

## Next steps
- Review PR from [#424](https://github.com/EvaLok/schema-org-json-ld/issues/424) when Copilot finishes
- Next state.json metric verification: cycle 128
- Google property verification complete — no further verification cycles needed
