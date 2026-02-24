# Coding Agent Instructions

You are implementing features for `schema-org-json-ld`, a PHP library that generates schema.org JSON-LD structured data for Google Rich Results.

## Repository Structure

```
src/
  v1/
    Schema/           # Schema type classes (Product, Offer, Brand, etc.)
    JsonLdGenerator.php
    TypedSchema.php   # Base interface for typed schemas
test/
  unit/               # PHPUnit tests
  samples/            # Sample JSON-LD output files
```

- **Namespace**: `EvaLok\SchemaOrgJsonLd`
- **Autoloading**: PSR-4 via Composer (`src/` maps to namespace root)
- **Default branch**: `master`

## Coding Standards

- **PHP 8.1+** minimum — use modern PHP features
- `declare(strict_types=1);` in every PHP file
- Constructor promotion for data classes
- Enums (backed string enums) for constrained values like `ItemAvailability`, `OfferItemCondition`
- Nullable parameters with `?Type` syntax, not `Type|null`
- No `mixed` types — be explicit
- Type-hint everything: parameters, return types, properties

## Schema Implementation Pattern

When implementing a new schema.org type:

1. **Check the spec**: Visit `https://schema.org/<TypeName>` for the canonical property list
2. **Check Google docs**: Visit Google's structured data docs to see which properties are required/recommended for Rich Results
3. **Create the class** in `src/v1/Schema/` implementing `TypedSchema`
4. **Use constructor promotion** — all properties as promoted constructor parameters
5. **Implement `toArray()`** — return associative array with `@type` key, filtering out null values
6. **Nested schemas**: Properties that are themselves schema types should accept `TypedSchema` instances and call their `toArray()` in the parent's `toArray()`
7. **Write tests** in `test/unit/` — test the JSON-LD output structure, required fields, optional fields, nested objects
8. **Add sample output** in `test/samples/` if the type is complex enough to warrant it

## Testing Expectations

- **Framework**: PHPUnit 10.x
- **Run tests**: `composer run test-unit`
- **Test structure**: One test class per schema type, named `<TypeName>Test.php`
- **What to test**:
  - Constructor creates valid object
  - `toArray()` produces correct `@type`
  - Required properties are present in output
  - Optional/nullable properties are omitted when null
  - Nested schema objects serialize correctly
  - Enum values serialize to their schema.org URL form
- **Validate output**: JSON-LD output should match Google Rich Results Test expectations

## Quality Checklist

Before marking your PR as ready:

- [ ] All existing tests pass (`composer run test-unit`)
- [ ] New tests added for all new/modified schema types
- [ ] `declare(strict_types=1)` in every new PHP file
- [ ] No `mixed` types — all types explicit
- [ ] Constructor promotion used where appropriate
- [ ] Null values filtered from `toArray()` output
- [ ] `@type` key present in every schema's `toArray()` output
- [ ] Enums used for constrained value sets (not magic strings)
- [ ] No breaking changes to existing public API

## Common Pitfalls

- **Don't forget `@type`**: Every schema class's `toArray()` must include `'@type' => '<TypeName>'`
- **Null filtering**: Properties set to `null` should not appear in the JSON-LD output. Use `array_filter` or conditional array building.
- **Enum serialization**: Backed string enums should serialize to their schema.org URL form (e.g., `https://schema.org/InStock`), not the enum case name
- **Array properties**: Properties that accept arrays (like `image`, `offers`) should type-hint as `?array` and handle both single items and arrays
- **Don't modify `JsonLdGenerator`** unless the issue specifically asks for it — it's the shared entry point
- **Composer autoload**: New classes are auto-discovered via PSR-4; no need to modify `composer.json` unless adding new top-level namespaces
