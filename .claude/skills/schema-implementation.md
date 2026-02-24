# Skill: Implement a Schema.org Type

Step-by-step procedure for implementing a new schema.org type in this library.

## Step 1: Understand the type

- Read the Google Rich Results docs URL from the issue
- Read the schema.org spec URL from the issue
- Identify required, recommended, and optional properties
- Identify property types: string, int, float, bool, array, enum, or nested schema

## Step 2: Check for dependencies

- Does this type reference other TypedSchema classes?
- Are they already implemented in `src/v1/Schema/`?
- If not, they need to be created too (or the issue should specify using a simpler type like `string` temporarily)

## Step 3: Create the schema class

File: `src/v1/Schema/{TypeName}.php`

Template:
```php
<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class {TypeName} extends TypedSchema {
    const A_SCHEMA_TYPE = '{TypeName}';

    public function __construct(
        // Required properties first (no default value)
        public string $requiredProp,
        // Optional properties last (default to null)
        public null|string $optionalProp = null,
    ) {
    }
}
```

Rules:
- Extend `TypedSchema`
- Set `A_SCHEMA_TYPE` to the exact schema.org type name
- Use constructor promotion for ALL properties
- Required properties have no default value
- Optional properties default to `null` with `null|Type` syntax
- For array properties, add `/** @var ElementType[] $propName */` doc comment
- Do NOT add any methods — no toArray(), no serialize(), nothing

## Step 4: Create enum types (if needed)

File: `src/v1/Schema/{EnumName}.php`

Template:
```php
<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

enum {EnumName}: string {
    case CaseName = 'https://schema.org/CaseName';
}
```

Rules:
- Backed string enum with full schema.org URLs as values
- PascalCase for case names matching schema.org names

## Step 5: Write tests

File: `test/unit/{TypeName}Test.php`

Must test:
1. **Minimal construction**: Only required properties, verify JSON output has @context, @type, and required fields
2. **Null omission**: Optional properties when null are NOT present in JSON output
3. **Full construction**: All properties set, verify they all appear in JSON output
4. **Nested schemas**: If the type contains other TypedSchema instances, verify they serialize correctly
5. **Enums**: If the type uses enums, verify they serialize to schema.org URLs

Always use `JsonLdGenerator::SchemaToJson(schema: $instance)` for serialization in tests.

## Step 6: Add sample JSON (optional)

File: `test/samples/{TypeName}.json`

Create if the type is complex. Match the expected JSON-LD output format. Used for test assertions via `json_decode(file_get_contents(...))`.

## Step 7: Run tests

```bash
composer run test-unit
```

All existing tests plus new tests must pass. PHP 8.3 and composer dependencies are pre-installed in your environment. You MUST run tests and confirm they pass before finishing your work.

## Common mistakes to avoid

1. Adding a `toArray()` method — don't do this, JsonLdGenerator uses reflection
2. Using `?Type` instead of `null|Type` — the codebase uses `null|Type`
3. Forgetting `A_SCHEMA_TYPE` — this determines the @type in JSON-LD output
4. Adding `@context` manually — JsonLdGenerator adds this automatically
5. Manual null filtering — JsonLdGenerator handles this automatically
6. Modifying JsonLdGenerator.php or TypedSchema.php — never modify these
