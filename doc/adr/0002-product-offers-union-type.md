# 2. Product offers union type for AggregateOffer support

Date: 2026-02-25

## Status

Accepted

## Context

Google's Product structured data supports two pricing patterns:

1. **Individual offers**: An array of `Offer` objects, each with a specific price
2. **Aggregate offer**: A single `AggregateOffer` object with `lowPrice`/`highPrice` for price ranges

The existing `Product` class accepted only `/** @var Offer[] */ public array $offers`. Adding `AggregateOffer` support requires a design decision about the `$offers` property type.

Options considered:

- **A) Union type `array|AggregateOffer`**: The `$offers` parameter accepts either an array of `Offer` objects OR a single `AggregateOffer`. This matches Google's schema where a product has either individual offers or an aggregate.
- **B) Separate property**: Add a new `$aggregateOffer` property alongside the existing `$offers`. This avoids union types but means either property could be set, requiring documentation to explain they're mutually exclusive.
- **C) Make AggregateOffer extend Offer**: Use inheritance so AggregateOffer is-an Offer. This is semantically wrong — an aggregate offer is not a single offer.

## Decision

Option A: Use `array|AggregateOffer` union type for `Product.$offers`.

## Consequences

- Product constructor signature: `public array|AggregateOffer $offers`
- Doc comment: `/** @var Offer[]|AggregateOffer $offers */`
- `JsonLdGenerator` already handles both arrays of `TypedSchema` and single `TypedSchema` instances, so no generator changes needed
- Matches the schema.org semantics: a product's offers are either individual or aggregated
- PHP 8.0+ union types provide compile-time type safety
- Existing code using `Product(offers: [...])` continues to work unchanged
