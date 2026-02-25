# 3. Reflection-based serialization via JsonLdGenerator

Date: 2026-02-25

## Status

Accepted (original design, documented retrospectively)

## Context

Schema type classes need to serialize to JSON-LD output. Two common approaches:

1. **Each class implements serialization** (e.g., `toArray()` method on every schema class)
2. **Centralized serialization** via a generator that reflects on class properties

The project chose option 2 with `JsonLdGenerator::SchemaToJson()`.

## Decision

All serialization is handled by `JsonLdGenerator` via PHP reflection on public properties. Schema classes are pure data objects with:

- `const A_SCHEMA_TYPE` for the `@type` value
- Constructor-promoted public properties
- No methods

## Consequences

### Positive

- **Zero serialization code per type**: Adding a new schema type requires only a class with properties. No `toArray()`, no `jsonSerialize()`, no serialization logic.
- **Consistency**: All types serialize identically. No risk of one type handling nulls differently from another.
- **Agent-friendly**: The coding agent never needs to write serialization code, eliminating an entire category of bugs.
- **Extensible**: New features like `PROPERTY_MAP` (for hyphenated names) and array `@type` were added to the generator once and apply to all types.

### Negative

- **Implicit contract**: The relationship between class properties and JSON output is implicit. A developer must understand that public properties map to JSON keys.
- **No per-type customization**: If a type needs special serialization (e.g., conditional property inclusion), it requires generator modifications rather than type-level overrides.
- **Reflection performance**: Reflection is slower than direct property access, though negligible for JSON-LD generation use cases.

### Validated by scale

This approach has scaled to 67+ schema classes and 243 tests without requiring any per-type serialization code. The `PROPERTY_MAP` extension (ADR pending) demonstrated that edge cases can be handled cleanly in the generator without breaking the zero-methods pattern.
