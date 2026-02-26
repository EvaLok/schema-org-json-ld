# Cycle 29 — 2026-02-26T07:47Z

## Summary

Twenty-ninth orchestrator cycle. Proactive audit of Google merchant listing docs revealed significant gaps in Product recommended properties. Dispatched 2 agent tasks to address them.

## What happened

### Startup

1. No `input-from-eva` issues found.
2. Recovered context from Cycle 28 worklog — clean state, feature-complete, waiting on Eva #154.
3. No open PRs, no open Copilot issues. 0 in-flight agent sessions.
4. No new QC outbound reports from QC repo.
5. No new QC inbound issues on our repo.
6. Question for Eva #154 (release recommendation) still open, no response.

### Google Search Gallery check

Same 26 types as before. No new types added.

### Product merchant listing audit (new work found!)

Spot-checked our Product/Offer implementations against Google's merchant listing docs page. Found several Google-recommended properties we haven't implemented:

**Missing from Product**: color, material, pattern, inProductGroupWithID, gtin/gtin8/gtin12/gtin13/gtin14/isbn, size, isVariantOf, audience, hasCertification

**Missing sub-types**: SizeSpecification, ProductGroup, PeopleAudience, Certification, UnitPriceSpecification

**Missing from Offer**: priceSpecification (for UnitPriceSpecification)

These are all documented as "recommended" on the Google merchant listing page but were not in our initial implementation scope (which focused on required + the most common recommended properties).

### Agent dispatches

Dispatched 2 concurrent agent tasks:

1. **#160** — Add Product text properties (color, material, pattern, size, inProductGroupWithID, GTIN identifiers, isbn) + SizeSpecification sub-type. Simple string additions + one small class.

2. **#162** — Add ProductGroup, PeopleAudience, Certification, UnitPriceSpecification sub-types + wire Product.isVariantOf, Product.audience, Product.hasCertification, Offer.priceSpecification.

Both assigned to gpt-5.3-codex. Structured the issues so they can be developed concurrently — #160 only adds string properties, #162 only adds sub-type properties. Both modify Product.php but should be merge-compatible since they add different non-overlapping parameters.

## Decisions

1. **Split into 2 issues**: Separated simple text properties (#160) from complex sub-types (#162) to keep each issue focused and allow parallel development. Both modify Product.php but add non-overlapping parameters.

2. **Included GTIN identifiers**: Google docs list gtin/gtin8/gtin12/gtin13/gtin14/isbn as recommended. While these are rarely used in practice, they're trivial to add (just nullable strings) and complete our coverage.

3. **Did NOT include 3DModel/subjectOf**: This is a niche feature (3D product viewers) and adds complexity. Deferred for now.

## Final state

- **Implemented types**: 28 Google Rich Results types — 100% coverage
- **In-flight agent work**: 2 sessions (#160, #162)
- **Open questions for Eva**: #154 (release recommendation — no response yet)

## Next steps

- Next cycle: Check if #160 and #162 PRs are ready for review
- Review and merge PRs when Copilot finishes
- If both merge cleanly, consider requesting QC validation for Product enhancements
- Note: Both PRs modify Product.php — may need to handle merge conflicts if second PR doesn't account for first
