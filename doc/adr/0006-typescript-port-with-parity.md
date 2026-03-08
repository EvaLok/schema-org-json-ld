# 6. TypeScript Port with 1:1 JSON-LD Parity

Date: 2026-03-01

## Status

Accepted

## Context

The PHP library covered all 28 Google Rich Results schema types. To reach a broader developer audience (Node.js, frontend, full-stack TypeScript projects), a TypeScript port was needed. The key design question: should the TypeScript version be a reimagining or an exact port?

Google's Rich Results Test is the acceptance test for JSON-LD output. Any divergence between PHP and TypeScript output for the same input would mean one version produces invalid structured data. Users switching between languages need identical behavior.

## Decision

Port every PHP schema class to TypeScript with 1:1 JSON-LD output parity. Specifically:

1. **Same class names and property names** across both languages
2. **Identical `toArray()` / `toJsonLd()` output** for the same inputs — verified by the QC orchestrator
3. **Same enum values and serialization** — enums produce identical string values
4. **Language-idiomatic patterns** where they don't affect output — TypeScript uses `readonly` properties, builder-style setters, and native `Map` where PHP uses associative arrays
5. **Separate npm package** (`@anthropic-ai/schema-org-json-ld`) alongside the existing Composer package

Parity is validated by the QC orchestrator comparing JSON-LD output from both languages for all 73 standalone-testable types (88 total minus 12 enums minus 3 building-block types).

## Consequences

- **Positive**: Users get identical behavior regardless of language choice
- **Positive**: QC parity testing catches regressions in either language
- **Positive**: Adding a new type in one language immediately defines the spec for the other
- **Negative**: Every PHP change must be mirrored in TypeScript (and vice versa) — dual maintenance
- **Negative**: Some PHP patterns don't translate idiomatically (e.g., union types, associative arrays)
- **Trade-off**: Strict parity constrains TypeScript API design, but the constraint is worth the consistency

## Alternatives Considered

1. **Independent TypeScript implementation**: Design TypeScript API from scratch. Would produce more idiomatic code but risk output divergence and double the testing burden.
2. **Auto-generation from shared schema**: Generate both PHP and TypeScript from a common definition. Too complex for the current project size and would produce less readable code.
3. **TypeScript-only going forward**: Abandon PHP. Excludes the existing PHP user base.
