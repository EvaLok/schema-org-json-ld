# 4. Shared sub-types first implementation strategy

Date: 2026-02-25

## Status

Accepted (validated across 22 orchestrator cycles)

## Context

The 28 Google Rich Results types share many sub-types. For example:

- `Organization` is used by Article, Event, LocalBusiness, JobPosting, Course, and others
- `PostalAddress` is used by LocalBusiness, Event, JobPosting, Organization
- `Person` is used by Article, Recipe, Review, ProfilePage, DiscussionForum
- `AggregateRating` / `Review` are used by Product, Recipe, LocalBusiness, SoftwareApp
- `ImageObject` is used by Article, Recipe, Video, ProfilePage

Implementing parent types before their sub-types creates dependency blockers. Each parent type issue would need to either: (a) inline sub-type implementations (increasing scope and risk), or (b) use placeholder types that need replacement later.

## Decision

Implement shared sub-types first, then parent types. Sequence:

1. Shared sub-types: AggregateRating, Review/Rating, Organization, PostalAddress, ContactPoint, Person, ImageObject
2. Parent types: all 28 Google Rich Results types
3. Quality audit: verify property completeness against Google docs
4. Edge cases: type widening, new sub-types discovered during audit

## Consequences

### Positive

- **No dependency blockers**: Every parent type dispatch had all needed sub-types available
- **Clean issue specs**: Parent type issues could reference existing classes by name
- **Fewer revision requests**: The agent never had to create placeholder types
- **Natural test progression**: Sub-type tests validated foundations before parent types built on them
- **Efficient agent sessions**: Parent type implementations reused existing patterns, averaging 7-10 minutes

### Negative

- **Delayed visible output**: The first few cycles produced only sub-types (not directly useful for Rich Results)
- **Upfront planning cost**: Required analyzing the dependency graph before writing any code

### Validation

Over 22 cycles: 28 parent types implemented with zero dependency-related failures or blockers. The strategy worked exactly as designed — sub-types unlocked efficient parent type implementation. The 39-PR zero-revision streak is partly attributable to this approach, since agents never had to work around missing types.
