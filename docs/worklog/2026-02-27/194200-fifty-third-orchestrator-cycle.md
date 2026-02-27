# Cycle 53 — 2026-02-27T19:42Z

## Summary

Fifty-third orchestrator cycle. Audited Google Search Gallery — now shows 31 categories (up from 26 in previous audits). All 5 new entries are sub-categories already covered by existing implementations. Found one gap: `Offer` is missing `hasMerchantReturnPolicy` (recommended by Google's Merchant listing docs). Dispatched [#231](https://github.com/EvaLok/schema-org-json-ld/issues/231) to Copilot. [PR #232](https://github.com/EvaLok/schema-org-json-ld/issues/232) merged at 19:57Z. First agent dispatch since Cycle 43 (10 cycles ago).

## Startup checklist results

- **Eva input**: None
- **Open questions**: None
- **Open PRs**: None
- **Agent sessions**: None (before dispatch)
- **QC outbound**: None
- **QC inbound**: None
- **Stale branches**: None (only `master`)
- **Stale issues**: None
- **Concurrency**: 0/2 (pre-dispatch), 1/2 (post-dispatch)

## What happened

### Search Gallery audit

Google's Search Gallery now lists 31 structured data categories. The 5 "new" entries are sub-categories of existing types:

| Gallery Entry | Our Coverage |
|---|---|
| Merchant listing | Product + Offer (original impl) |
| Product variants | ProductGroup + Product (original impl) |
| Loyalty program | MemberProgram + MemberProgramTier (PR #146) |
| Merchant return policy | MerchantReturnPolicy (original impl) |
| Merchant shipping policy | ShippingService + ShippingConditions (PR #149) |

All 5 use types we already implement. No new classes needed.

### Gap found: `hasMerchantReturnPolicy` on `Offer`

Google's [Merchant listing](https://developers.google.com/search/docs/appearance/structured-data/merchant-listing) and [Return policy](https://developers.google.com/search/docs/appearance/structured-data/return-policy) docs both list `hasMerchantReturnPolicy` as a **recommended** property on `Offer` (for per-product return policies). The `MerchantReturnPolicy` class exists and is wired to `Organization`, but not yet to `Offer`.

Dispatched [#231](https://github.com/EvaLok/schema-org-json-ld/issues/231) to add this property.

### Other findings

- `UnitPriceSpecification` already has `validForMemberTier` and `membershipPointsEarned` (loyalty program integration) ✓
- `Organization` already has `hasMemberProgram`, `hasMerchantReturnPolicy`, `hasShippingService` ✓
- `ShippingService` and `ShippingConditions` cover all Google-required properties ✓
- `MerchantReturnPolicy` covers all Google-required and recommended properties ✓

### PR #232 — reviewed and merged

Copilot created [PR #232](https://github.com/EvaLok/schema-org-json-ld/issues/232) with clean changes:
- `Offer.php`: Added `public null|MerchantReturnPolicy $hasMerchantReturnPolicy = null` (+1 line)
- `OfferTest.php`: Added null-omission check + new `testWithMerchantReturnPolicy` test (+24 lines)

CI passed (Test and Build: success). Merged at 19:57Z via squash.

## Current state

- **v1.0.0**: Released
- **Open PRs**: None
- **Agent sessions**: None
- **Tests**: 321, **Classes**: 98, **PHPStan**: level 9
- **Search Gallery coverage**: 31/31 categories (30 excluding Book actions)

## Next steps

1. Consider sending QC request for Offer hasMerchantReturnPolicy validation
2. Respond to any new Eva directives or QC reports
3. Low-priority: JobPosting beta properties, PHPStan max
