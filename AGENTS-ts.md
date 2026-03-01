# TypeScript Coding Agent Instructions

You are implementing features for the TypeScript version of `schema-org-json-ld`, a library that generates schema.org JSON-LD structured data for Google Rich Results.

This file supplements the main `AGENTS.md` with TypeScript-specific conventions. Read both files.

## Repository Structure (polyglot layout)

```
# Root-level config (alongside composer.json)
package.json              # @evabee/schema-org-json-ld
tsconfig.json             # TypeScript compiler config
biome.json                # Biome linter/formatter config

# TypeScript source and build config
ts/
  src/
    schema/           # Schema type classes (Product, Offer, Brand, etc.)
    enum/             # Enum definitions (ItemAvailability, DayOfWeek, etc.)
    JsonLdGenerator.ts    # Serialization engine
    TypedSchema.ts        # Abstract base class — do NOT modify
    index.ts              # Barrel export (re-exports all public types)
  test/
    schema/           # Vitest tests per schema type
    samples/          # Sample JSON-LD output files
  tsup.config.ts          # Build tool config (dual ESM/CJS)
  vitest.config.ts        # Test framework config
```

- **Module format**: ESM-first, dual ESM/CJS via tsup
- **Package scope**: `@evabee/schema-org-json-ld`
- **Default branch**: `master`

## Language Configuration

- **TypeScript**: strict mode (`"strict": true` in tsconfig.json)
- **Target**: ES2022 (aligns with Node 20+ features)
- **Node minimum**: 20 (also tested on 24)
- **Linter/formatter**: Biome (not ESLint/Prettier)
- **Test framework**: Vitest
- **Build tool**: tsup (dual ESM/CJS output)

## Architecture: Mirroring PHP

The TypeScript version mirrors the PHP library's architecture 1:1 for Phase 1. Each PHP schema class has a corresponding TypeScript class with the same name, properties, and behavior.

### Schema classes

TypeScript schema classes use `readonly` constructor parameters instead of PHP constructor promotion.

#### Small types (≤5 optional properties): positional constructor

```typescript
import { TypedSchema } from '../TypedSchema';

export class Brand extends TypedSchema {
  static readonly schemaType = 'Brand';

  constructor(
    public readonly name: string,
    public readonly description: string | null = null,
  ) {
    super();
  }
}
```

#### Large types (>5 optional properties): options object constructor

PHP has named parameters, so callers can skip optional args: `new Recipe(name: 'Cake', cookTime: 'PT1H')`. TypeScript has only positional args — passing `null` 12 times to reach a specific parameter is unusable. Use an options object instead:

```typescript
import { TypedSchema } from '../TypedSchema';

export interface RecipeOptions {
  name: string;
  description?: string | null;
  image?: string | ImageObject | null;
  cookTime?: string | null;
  prepTime?: string | null;
  totalTime?: string | null;
  recipeYield?: string | null;
  // ... remaining optional properties
}

export class Recipe extends TypedSchema {
  static readonly schemaType = 'Recipe';

  public readonly name: string;
  public readonly description: string | null;
  public readonly image: string | ImageObject | null;
  public readonly cookTime: string | null;
  public readonly prepTime: string | null;
  public readonly totalTime: string | null;
  public readonly recipeYield: string | null;

  constructor(options: RecipeOptions) {
    super();
    this.name = options.name;
    this.description = options.description ?? null;
    this.image = options.image ?? null;
    this.cookTime = options.cookTime ?? null;
    this.prepTime = options.prepTime ?? null;
    this.totalTime = options.totalTime ?? null;
    this.recipeYield = options.recipeYield ?? null;
  }
}
```

**Threshold rule**: Count total optional properties. If >5, use an options object. If ≤5, positional params are fine. When in doubt, use an options object — it's always more readable.

The options interface is exported alongside the class for consumer use. Name it `{TypeName}Options`.

Rules:
- Extend `TypedSchema`
- Set `static readonly schemaType` to the exact schema.org type name
- Use `public readonly` for all properties
- For small types (≤5 optional): use `public readonly` constructor parameters
- For large types (>5 optional): use an options object with a `{TypeName}Options` interface
- Required parameters have no default value; optional parameters default to `null`
- Use `Type | null` union syntax for optionals (NOT `?:` optional syntax on class properties)
- In options interfaces, use `?:` for optional fields (this is the interface contract, not the class property)
- Array properties use TypeScript array types: `readonly Offer[]`
- Do NOT add serialization methods — `JsonLdGenerator` handles everything

### Multi-type schemas

For types requiring multiple `@type` values, use a string array:

```typescript
export class MathSolver extends TypedSchema {
  static readonly schemaType = ['MathSolver', 'LearningResource'];
  // ...
}
```

### Property name mapping

For schema.org properties with hyphenated names (invalid as JS identifiers), use a static `propertyMap`:

```typescript
export class SolveMathAction extends TypedSchema {
  static readonly schemaType = 'SolveMathAction';
  static readonly propertyMap: Record<string, string> = {
    mathExpressionInput: 'mathExpression-input',
  };

  constructor(
    public readonly target: string,
    public readonly mathExpressionInput: string,
  ) {
    super();
  }
}
```

## Enum Conventions

Use TypeScript string enums (direct mapping from PHP backed string enums):

```typescript
export enum ItemAvailability {
  InStock = 'https://schema.org/InStock',
  OutOfStock = 'https://schema.org/OutOfStock',
  Discontinued = 'https://schema.org/Discontinued',
}
```

File: `ts/src/enum/{EnumName}.ts`

Rules:
- String enum with full schema.org URLs as values
- PascalCase for enum name and member names, matching schema.org names
- One enum per file

## Serialization

`JsonLdGenerator.schemaToJson(schema)` mirrors the PHP implementation:
- Adds `@context` and `@type` automatically
- Skips `null` and `undefined` properties and empty arrays
- Recursively serializes nested `TypedSchema` instances
- Extracts enum values automatically
- Applies `propertyMap` remapping if defined

**You do not write serialization logic in schema classes.**

## Naming Conventions

| Concept | Convention | Example |
|---|---|---|
| Classes | PascalCase | `Product`, `FAQPage` |
| Properties | camelCase | `name`, `datePublished` |
| Enums | PascalCase name + PascalCase members | `ItemAvailability.InStock` |
| Files (classes) | PascalCase matching class name | `Product.ts` |
| Files (enums) | PascalCase matching enum name | `ItemAvailability.ts` |
| Test files | PascalCase with `.test.ts` suffix | `Product.test.ts` |
| Barrel exports | `index.ts` | `ts/src/index.ts` |

## Import/Export Conventions

- **ESM-first**: Use `import`/`export`, never `require()`
- **Barrel exports**: All public types re-exported from `ts/src/index.ts`
- **Named exports only**: No default exports (Biome enforces this)
- **Type imports**: Use `import type { ... }` for type-only imports when possible
- **Relative imports**: Use relative paths within the `ts/` tree (no path aliases)

```typescript
// Good
import { Product } from './schema/Product';
import type { TypedSchema } from './TypedSchema';

// Bad
import Product from './schema/Product';  // no default exports
const { Product } = require('./schema/Product');  // no require()
```

## Testing

- **Framework**: Vitest
- **Run tests**: `npm run test` (from repo root)
- **Test structure**: One test file per schema type in `ts/test/schema/`
- **File naming**: `{TypeName}.test.ts`

### What to test

1. **Minimal construction**: Only required properties, verify JSON output has `@context`, `@type`, and required fields
2. **Null omission**: Optional properties when `null` are NOT present in JSON output
3. **Full construction**: All properties set, verify they all appear in JSON output
4. **Nested schemas**: Verify nested `TypedSchema` instances serialize correctly
5. **Enums**: Verify they serialize to schema.org URLs
6. **Output parity**: JSON-LD output should match the PHP version for the same input

### Test pattern

```typescript
import { describe, it, expect } from 'vitest';
import { JsonLdGenerator } from '../../src/JsonLdGenerator';
import { YourType } from '../../src/schema/YourType';

describe('YourType', () => {
  it('produces minimal JSON-LD output', () => {
    const schema = new YourType('value');
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj['@context']).toBe('https://schema.org/');
    expect(obj['@type']).toBe('YourType');
    expect(obj.requiredProp).toBe('value');
  });

  it('omits optional fields when null', () => {
    const schema = new YourType('value');
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj).not.toHaveProperty('optionalField');
  });

  it('includes all fields when set', () => {
    const schema = new YourType('value', 'present');
    const json = JsonLdGenerator.schemaToJson(schema);
    const obj = JSON.parse(json);

    expect(obj.optionalField).toBe('present');
  });
});
```

## Quality Checklist

Before marking your PR as ready:

- [ ] All existing tests pass (`npm run test`)
- [ ] Biome passes (`npx biome check`)
- [ ] Build succeeds (`npm run build`)
- [ ] New tests added for all new/modified schema types
- [ ] No `any` types — all types explicit
- [ ] `readonly` constructor parameters used for all properties
- [ ] Optional parameters use `Type | null` union with `= null` default
- [ ] `static readonly schemaType` set correctly
- [ ] No serialization methods — JsonLdGenerator handles everything
- [ ] Enums use full schema.org URLs as values
- [ ] Barrel export (`index.ts`) updated with new types
- [ ] No breaking changes to existing public API
- [ ] Do NOT modify `JsonLdGenerator.ts` or `TypedSchema.ts` unless the issue specifically asks for it

## Common Mistakes

1. **Adding serialization methods** — don't; JsonLdGenerator uses property enumeration
2. **Using `any`** — be explicit with all types
3. **Forgetting `schemaType`** — this determines the `@type` in JSON-LD output
4. **Using default exports** — this project uses named exports only
5. **Using `require()`** — this is an ESM-first project
6. **Using `?:` for optional class properties** — use `Type | null = null` for class properties (matches PHP convention and ensures the property exists with an explicit null). Note: `?:` IS correct in options interfaces.
7. **Modifying JsonLdGenerator.ts or TypedSchema.ts** — never modify these unless specifically instructed
8. **Using positional constructor for large types** — if a type has >5 optional properties, use an options object constructor instead. Positional constructors with many `null` arguments are unusable.
