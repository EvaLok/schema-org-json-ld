---
name: ts-schema-implementation
description: Step-by-step procedure for implementing a new schema.org type in the TypeScript version of this library, including class structure, enum creation, test writing, and code style.
---

# Implement a Schema.org Type (TypeScript)

Step-by-step procedure for implementing a new schema.org type in the TypeScript version of this library. Read `AGENTS-ts.md` for full conventions before starting.

## Step 1: Understand the type

- Read the Google Rich Results docs URL from the issue
- Read the schema.org spec URL from the issue
- Identify required, recommended, and optional properties
- Identify property types: string, number, boolean, array, enum, Date/string, or nested schema
- Check the PHP implementation in `php/src/v1/Schema/{TypeName}.php` for reference — the TS version should mirror it

## Step 2: Check for dependencies

- Does this type reference other TypedSchema classes?
- Are they already implemented in `ts/src/schema/`?
- If not, they need to be created too (or the issue should specify using a simpler type like `string` temporarily)
- Check the PHP version's dependencies in `php/src/v1/Schema/` for the full dependency list

## Step 3: Write failing tests first (TDD)

File: `ts/test/schema/{TypeName}.test.ts`

Write tests before the implementation. These tests will fail initially because the class doesn't exist yet.

Template:
```typescript
import { describe, it, expect } from 'vitest';
import { JsonLdGenerator } from '../../src/JsonLdGenerator';
import { TypeName } from '../../src/schema/TypeName';

describe('TypeName', () => {
  it('produces minimal JSON-LD output with required fields only', () => {
    const schema = new TypeName('required-value');
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj['@context']).toBe('https://schema.org/');
    expect(obj['@type']).toBe('TypeName');
    expect(obj.requiredProp).toBe('required-value');
  });

  it('omits optional fields when null', () => {
    const schema = new TypeName('required-value');
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj).not.toHaveProperty('optionalField');
  });

  it('includes all fields when set', () => {
    const schema = new TypeName('required-value', 'optional-value');
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj.optionalField).toBe('optional-value');
  });

  it('serializes nested schemas correctly', () => {
    const nested = new NestedType('nested-value');
    const schema = new TypeName('required-value', null, nested);
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj.nestedProp['@type']).toBe('NestedType');
    expect(obj.nestedProp.name).toBe('nested-value');
  });

  it('serializes enums to schema.org URLs', () => {
    const schema = new TypeName('required-value', null, null, SomeEnum.Value);
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj.enumProp).toBe('https://schema.org/Value');
  });
});
```

Must test:
1. **Minimal construction**: Only required properties, verify JSON has `@context`, `@type`, and required fields
2. **Null omission**: Optional properties when `null` are NOT present in output
3. **Full construction**: All properties set, verify all appear in output
4. **Nested schemas**: Verify nested TypedSchema instances serialize correctly
5. **Enums**: Verify enums serialize to schema.org URLs
6. **PHP parity**: JSON-LD output should match the PHP version for the same inputs

## Step 4: Create the schema class

File: `ts/src/schema/{TypeName}.ts`

Template:
```typescript
import { TypedSchema } from '../TypedSchema';

export class TypeName extends TypedSchema {
  static readonly schemaType = 'TypeName';

  constructor(
    // Required properties first (no default value)
    public readonly requiredProp: string,
    // Optional properties last (default to null)
    public readonly optionalProp: string | null = null,
    public readonly nestedProp: NestedType | null = null,
    public readonly enumProp: SomeEnum | null = null,
  ) {
    super();
  }
}
```

Rules:
- Extend `TypedSchema`
- Set `static readonly schemaType` to the exact schema.org type name
- Use `public readonly` for all constructor parameters
- Required parameters first (no default value), optional parameters last (default to `null`)
- Use `Type | null` union syntax for optionals (NOT `?:` optional syntax)
- For array properties, use `readonly Type[]` with default `= []`
- Do NOT add serialization methods — `JsonLdGenerator` handles everything
- Do NOT modify `JsonLdGenerator.ts` or `TypedSchema.ts` unless the issue specifically asks

## Step 5: Create enum types (if needed)

File: `ts/src/enum/{EnumName}.ts`

Template:
```typescript
export enum EnumName {
  CaseName = 'https://schema.org/CaseName',
  AnotherCase = 'https://schema.org/AnotherCase',
}
```

Rules:
- String enum with full schema.org URLs as values
- PascalCase for enum name and member names matching schema.org names
- One enum per file
- Export from barrel (`ts/src/index.ts`)

## Step 6: Update barrel exports

File: `ts/src/index.ts`

Add exports for all new classes and enums:
```typescript
export { TypeName } from './schema/TypeName';
export { EnumName } from './enum/EnumName';
```

Keep exports sorted alphabetically within each section (schemas, then enums).

## Step 7: Make tests pass

Run tests and iterate until all pass:
```bash
npm run test
```

All existing tests plus new tests must pass. Fix any failures before proceeding.

## Step 8: Run linter and build

```bash
npx biome check
npm run build
```

Both must pass cleanly. Biome handles formatting and linting. Do NOT manually fix style issues that Biome can auto-fix — run `npx biome check --write` instead.

## Step 9: Verify PHP parity

Compare your TypeScript JSON-LD output against the PHP version:
- Same `@context` and `@type`
- Same property names (camelCase, matching schema.org)
- Same serialization of nested objects, arrays, and enums
- Null/undefined properties omitted in both

## Common mistakes to avoid

1. **Adding serialization methods** — don't; JsonLdGenerator uses property enumeration
2. **Using `any` type** — be explicit with all types
3. **Forgetting `schemaType`** — this determines the `@type` in JSON-LD output
4. **Using default exports** — this project uses named exports only
5. **Using `require()`** — this is an ESM-first project
6. **Using `?:` for optional params** — use `Type | null = null` (matches PHP convention, ensures property exists with explicit null)
7. **Modifying JsonLdGenerator.ts or TypedSchema.ts** — never modify these unless specifically instructed
8. **Forgetting to update index.ts** — all new public types must be exported from the barrel
9. **Writing tests after implementation** — write tests FIRST (TDD), then implement to make them pass
10. **Mismatched property names vs PHP** — check the PHP implementation to ensure property names are identical
