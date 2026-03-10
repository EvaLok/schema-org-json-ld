# Coding Agent Instructions

You are implementing features for `schema-org-json-ld`, a PHP and TypeScript library that generates schema.org JSON-LD structured data for Google Rich Results.

**For TypeScript work**, also read `AGENTS-ts.md` for TypeScript-specific conventions.

## Repository Structure

```
php/
  src/
    v1/
      Schema/           # PHP schema type classes (Product, Offer, Brand, etc.)
      Enum/             # Backed string enums (ItemAvailability, DayOfWeek, etc.)
      JsonLdGenerator.php   # Serialization engine
      TypedSchema.php       # Abstract base class — do NOT modify
  test/
    unit/               # PHPUnit tests
    samples/            # Sample JSON-LD output files
ts/
  src/
    schema/           # TypeScript schema type classes
    enum/             # TypeScript string enums
    JsonLdGenerator.ts    # Serialization engine
    TypedSchema.ts        # Abstract base class — do NOT modify
    index.ts              # Barrel export
  test/
    schema/           # Vitest tests
```

- **PHP namespace**: `EvaLok\SchemaOrgJsonLd`
- **PHP autoloading**: PSR-4 via Composer (`php/src/` maps to namespace root)
- **TypeScript package**: `@evabee/schema-org-json-ld`
- **Default branch**: `master`

## How Serialization Works

**CRITICAL**: Schema classes do NOT implement `toArray()` or any serialization methods. The `JsonLdGenerator` class handles all serialization by reflecting on the public properties of the schema class. You only need to define the class with constructor-promoted properties.

`JsonLdGenerator::SchemaToJson(schema: $schema)` does the following automatically:
- Adds `@context` and `@type` (from the class constant `A_SCHEMA_TYPE` — supports both string and array values)
- Skips null properties and empty arrays
- Recursively serializes nested `TypedSchema` instances
- Extracts `.value` from backed string enums
- Handles arrays of schema objects and primitives
- Remaps property names via `PROPERTY_MAP` if defined (for hyphenated JSON-LD names)

**You do not need to write any serialization logic.**

## Coding Standards

- **PHP 8.1+** minimum — use modern PHP features
- **`declare(strict_types=1);`** — required in ALL PHP files, enforced by PHP-CS-Fixer
- **Tab indentation** — the entire codebase uses tabs (not spaces) for indentation. This applies to all PHP files: class bodies, method bodies, test files, everything. Match the style in existing files like `php/src/v1/Schema/Brand.php`.
- Constructor promotion for all schema data classes
- Enums (backed string enums) for constrained values — stored in `php/src/v1/Enum/`
- Nullable parameters with `null|Type` syntax (this is the convention used throughout the existing codebase)
- No `mixed` types — be explicit
- Type-hint everything: parameters, return types, properties

## Schema Implementation Pattern

When implementing a new schema.org type:

1. **Check the spec**: Visit `https://schema.org/<TypeName>` for the canonical property list
2. **Check Google docs**: Visit Google's structured data docs to see which properties are required/recommended for Rich Results
3. **Create the class** in `php/src/v1/Schema/` extending `TypedSchema`
4. **Set `A_SCHEMA_TYPE`** — override the class constant with the schema.org type name
5. **Use constructor promotion** — all properties as promoted constructor parameters
6. **Required params first, optional params last** — optional params default to `null`
7. **No methods needed** — JsonLdGenerator handles serialization automatically
8. **Write tests** in `php/test/unit/` — test the JSON-LD output via `JsonLdGenerator::SchemaToJson()`
9. **Add sample output** in `php/test/samples/` if the type is complex enough to warrant it

### Reference example — simple type (Brand)

```php
<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Brand extends TypedSchema {

    public const A_SCHEMA_TYPE = 'Brand';

    public function __construct(
        public string $name,
        public null|string $description = null,
    ) {

    }
}
```

### Reference example — type with nested schemas (Product)

```php
<?php

namespace EvaLok\SchemaOrgJsonLd\v1\Schema;

use EvaLok\SchemaOrgJsonLd\v1\TypedSchema;

class Product extends TypedSchema {
    public const A_SCHEMA_TYPE = 'Product';

    /**
     * @param string[] $image
     * @param Offer[] $offers
     */
    public function __construct(
        public string $name,
        public array  $image,
        public string $description,
        public string $sku,
        public array  $offers,
        public null|Brand $brand = null,
        public null|string $mpn = null,
        public null|QuantitativeValue $weight = null,
    ) {
    }
}
```

### Reference example — backed string enum

```php
<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\v1\Enum;

enum ItemAvailability: string {
    case InStock = 'https://schema.org/InStock';
    case OutOfStock = 'https://schema.org/OutOfStock';
    case Discontinued = 'https://schema.org/Discontinued';
}
```

## Testing Expectations

- **Framework**: PHPUnit 10.x
- **Run tests**: `composer run test-unit`
- **Test structure**: One test class per schema type, named `<TypeName>Test.php`
- **What to test**:
  - Constructor creates valid object
  - `JsonLdGenerator::SchemaToJson()` produces valid JSON with correct `@type`
  - Required properties are present in output
  - Optional/nullable properties are omitted when null
  - Nested schema objects serialize correctly
  - Enum values serialize to their schema.org URL form
- **Validate output**: JSON-LD output should match Google Rich Results Test expectations

### Test pattern

```php
<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\YourType;
use PHPUnit\Framework\TestCase;

class YourTypeTest extends TestCase
{
    public function testMinimalOutput(): void
    {
        $schema = new YourType(requiredParam: 'value');
        $json = JsonLdGenerator::SchemaToJson(schema: $schema);
        $this->assertIsString($json);

        $obj = json_decode($json);
        $this->assertEquals('https://schema.org/', $obj->{'@context'});
        $this->assertEquals('YourType', $obj->{'@type'});
        $this->assertEquals('value', $obj->requiredParam);
    }

    public function testOptionalFieldsOmittedWhenNull(): void
    {
        $schema = new YourType(requiredParam: 'value');
        $json = JsonLdGenerator::SchemaToJson(schema: $schema);
        $obj = json_decode($json);
        $this->assertObjectNotHasProperty('optionalField', $obj);
    }

    public function testFullOutput(): void
    {
        $schema = new YourType(
            requiredParam: 'value',
            optionalField: 'present',
        );
        $json = JsonLdGenerator::SchemaToJson(schema: $schema);
        $obj = json_decode($json);
        $this->assertEquals('present', $obj->optionalField);
    }
}
```

## Running Tests

You MUST run tests before finishing your work:

```bash
composer run test-unit
```

PHP 8.3 and all composer dependencies are pre-installed in your environment via `copilot-setup-steps.yml`. Run `composer run test-unit` after implementing your changes and confirm all tests pass (both new and existing). Do not mark your PR as ready if tests fail.

## Code Style

This project uses **PHP-CS-Fixer** to enforce consistent code style. A CI job will reject PRs with style violations.

**Before committing**, run the auto-fixer:

```bash
composer run cs-fix
```

This automatically fixes all formatting issues (indentation, brace placement, import ordering, whitespace). You do not need to manually fix style — just run this command. You can verify style compliance without auto-fixing via `composer run cs-check`.

Key style rules enforced by the fixer:
- Tab indentation (not spaces)
- Opening braces on same line (`class Foo {`, `function bar() {`)
- Alphabetically ordered imports
- No unused imports
- Short array syntax (`[]` not `array()`)
- PER-CS2.0 base ruleset

## Agent Environment

Your session runs in a GitHub Actions runner with the following pre-installed (via `.github/copilot-setup-steps.yml`):

- **PHP 8.3** + Composer + all dependencies (`composer install` already run)
- **Node.js 22** + npm + all dependencies (`npm ci` already run in `ts/`)
- **Bun** (latest)
- **Rust stable** + all tools pre-compiled (`cargo build --release` already run in `tools/rust/`)

**You are expected to verify your work locally before pushing.** Run the appropriate test and lint commands:

- PHP: `composer run test-unit`, `composer run cs-fix`, `composer run phpstan`
- TypeScript: `cd ts && npm test`, `cd ts && npm run lint`, `cd ts && npm run build`
- Rust tools: `cargo test --manifest-path tools/rust/Cargo.toml`

Do not push code that you haven't verified passes tests and lint. The CI pipeline will catch failures, but verifying locally first saves a round-trip.

## Quality Checklist

Before marking your PR as ready:

- [ ] All existing tests pass (`composer run test-unit`)
- [ ] Code style is clean (`composer run cs-fix` — run this before committing)
- [ ] PHPStan passes (`composer run phpstan`)
- [ ] New tests added for all new/modified schema types
- [ ] No `mixed` types — all types explicit
- [ ] Constructor promotion used for all properties
- [ ] Nullable properties use `null|Type` syntax (not `?Type`)
- [ ] Optional properties have `= null` default and are listed after required properties
- [ ] Class constant `A_SCHEMA_TYPE` set correctly
- [ ] No serialization methods added — JsonLdGenerator handles everything
- [ ] Enums used for constrained value sets (not magic strings)
- [ ] No breaking changes to existing public API
- [ ] Do NOT modify `JsonLdGenerator.php` or `TypedSchema.php` unless the issue specifically asks for it

### Rust tool quality checklist (per Eva directive #516)

When building or modifying Rust tools (`tools/rust/crates/`), also verify:

- [ ] **Error paths fail-closed** — if a check cannot be performed, report failure, not success
- [ ] **Input validation** — validate external data (state.json fields, CLI args, API responses) before using in commands
- [ ] **No silent error swallowing** — errors are logged to `errors` vec or stderr, never silently ignored
- [ ] **Unit tests for error paths** — test what happens with invalid input, missing data, and command failures
- [ ] **Deprecation-free** — no warnings from `cargo build` or `cargo test`
- [ ] **Action items distinguish error from failure** — if the tool generates action items, error states produce different messages from genuine failures
- [ ] Tests pass: `cargo test -p <tool-name> --manifest-path tools/rust/Cargo.toml`

See `.claude/skills/tool-creation-guidelines/SKILL.md` for the full tool quality assurance guidelines.

## Advanced Patterns

### Array `@type` (multiple types)

Some schema.org types require multiple `@type` values. Set `A_SCHEMA_TYPE` to an array instead of a string — `JsonLdGenerator` handles both automatically.

```php
class MathSolver extends TypedSchema {
	public const A_SCHEMA_TYPE = ['MathSolver', 'LearningResource'];
	// ...
}
```

This produces `"@type": ["MathSolver", "LearningResource"]` in the JSON-LD output.

### PROPERTY_MAP (hyphenated JSON-LD property names)

Some schema.org properties use hyphenated names (e.g., `mathExpression-input`) which are not valid PHP identifiers. Use the `PROPERTY_MAP` constant to map PHP-friendly property names to their JSON-LD equivalents.

```php
class SolveMathAction extends TypedSchema {
	public const A_SCHEMA_TYPE = 'SolveMathAction';
	public const PROPERTY_MAP = [
		'mathExpressionInput' => 'mathExpression-input',
	];

	public function __construct(
		public string $target,
		public string $mathExpressionInput,  // maps to "mathExpression-input" in output
	) {}
}
```

`JsonLdGenerator` automatically checks for `PROPERTY_MAP` and remaps property names before serialization. Classes without `PROPERTY_MAP` are unaffected.

### Class inheritance (extending schema types)

Some schema.org types are subtypes of other types (e.g., `FoodEstablishment` is a subtype of `LocalBusiness`). When this is the case, extend the parent class instead of `TypedSchema`:

```php
class FoodEstablishment extends LocalBusiness {
	public const A_SCHEMA_TYPE = 'FoodEstablishment';

	public function __construct(
		string $name,
		PostalAddress $address,
		// ... all parent params passed through ...
		public null|bool|string $acceptsReservations = null,  // new param
	) {
		parent::__construct(
			name: $name,
			address: $address,
			// ... pass all parent params via named arguments ...
		);
	}
}
```

Key rules:
- **Extend the parent class**, not `TypedSchema` — the parent's properties are inherited
- **Override `A_SCHEMA_TYPE`** with the child's type name
- **Pass all parent constructor params through** via `parent::__construct()` with named arguments
- **Add child-specific params** as new promoted properties (with `public` keyword) at the end of the parameter list
- Parent params are NOT promoted in the child (no `public` keyword) — they're passed to `parent::__construct()`

Existing inheritance chains:
- `LocalBusiness → FoodEstablishment → Restaurant`
- `LocalBusiness → Store`
- `Article → BlogPosting`
- `Article → NewsArticle`
- `SoftwareApplication → MobileApplication`
- `SoftwareApplication → WebApplication`

## Documentation

When adding significant new properties or sub-types to existing schema classes, update `README.md` as part of the same PR:
- Update the **Supported Types** table if new sub-types are added
- Update the class count in the header (line 3) if new classes are created
- Add or expand usage examples in the relevant type section

This keeps documentation in sync with code. See `doc/adr/0005-documentation-as-continuous-maintenance.md`.

## Common Pitfalls

- **Don't implement `toArray()`**: Schema classes have NO methods. The JsonLdGenerator does all serialization via reflection on public properties. If you add a `toArray()` method, it will break the pattern.
- **Don't forget `A_SCHEMA_TYPE`**: Every schema class must set `public const A_SCHEMA_TYPE = 'TypeName'` — this is how JsonLdGenerator determines the `@type` value.
- **Null handling is automatic**: Properties set to `null` are automatically excluded from the JSON-LD output by JsonLdGenerator. Do not add manual null filtering.
- **Enum serialization is automatic**: Backed string enums are automatically serialized to their `.value` (the schema.org URL). Do not manually convert enums.
- **Use `null|Type` not `?Type`**: The existing codebase consistently uses `null|Type` for nullable properties.
- **Array type docs**: For array properties, use `/** @var Type[] */` inline before the promoted property parameter, or `@param Type[] $propertyName` in a PHPDoc block above the constructor. Both patterns are used in the codebase; `@var` inline is more common. PHPStan level max requires iterable value types.
- **Don't modify `JsonLdGenerator`** unless the issue specifically asks for it.
- **Don't modify `TypedSchema`** unless the issue specifically asks for it.
- **Composer autoload**: New classes are auto-discovered via PSR-4; no need to modify `composer.json` unless adding new top-level namespaces.

## Version Coordination (PHP + TypeScript)

The PHP (Composer) and TypeScript (npm) packages version **independently**. They share the same schema implementations but may be at different version numbers.

- **PHP**: versioned via `composer.json` — published to Packagist
- **TypeScript**: versioned via `package.json` — published to npm as `@evabee/schema-org-json-ld`

When adding a new schema type, implement it in **both PHP and TypeScript** in the same issue or as paired issues. This keeps the two packages in feature parity. The QC orchestrator validates parity between the two implementations.

## Documentation Agent

When assigned an issue labeled `cycle-docs`, you are generating worklog and journal entries for an orchestrator cycle. Your job is to produce accurate, fact-based documentation derived from committed state — not narration.

### Data sources (use ONLY these)

- **`docs/state.json`**: Read `last_cycle`, `copilot_metrics`, `cycle_phase` for cycle metadata
- **`git log`**: Use commit history for the cycle's date range to identify what was done
- **`git diff`**: Compare the cycle-start commit against the cycle-complete commit to identify self-modifications to infrastructure files (`tools/`, `STARTUP_CHECKLIST.md`, `COMPLETION_CHECKLIST.md`, `AGENTS.md`, `.claude/skills/`)
- **GitHub issue/PR metadata**: Use `gh` CLI to read merged PRs, processed issues, dispatch records
- **`bash tools/cycle-receipts --cycle N`**: Run this tool to get the commit receipt table
- **Previous journal entry**: Read the most recent `docs/journal/*.md` file to extract commitment chains

### Output format

**Worklog** at `docs/worklog/{date}/{time}-cycle-{N}-summary.md`:
- **What was done**: List of concrete actions with issue/PR links
- **Self-modifications**: List infrastructure file changes found via `git diff`. If no infrastructure files changed, write "None this cycle." Do NOT fabricate modifications.
- **Current state**: In-flight count (from `copilot_metrics.in_flight`), pipeline status
- **Next steps**: Derived from state and open issues
- **Commit receipts**: Output of `cycle-receipts` tool — do NOT manually assemble

**Journal** appended to `docs/journal/{date}.md`:
- Link to the worklog entry
- Commitment section with concrete, observable completion conditions
- Previous commitment follow-through: for each commitment from the last journal entry, state whether it was completed, deferred (with reason), or dropped (with reason)

### Accuracy rules

- **In-flight count**: Read `copilot_metrics.in_flight` from `docs/state.json`. Do NOT count manually.
- **Self-modifications**: Run `git diff` on infrastructure files. If the diff is empty, report "None." If it shows changes, list each changed file.
- **Receipt hashes**: Use `cycle-receipts` tool output verbatim. Do NOT invent or recall hashes from memory.
- **Do NOT post comments**: You are a Copilot coding agent. Create files via PR only.
- **Do NOT fabricate data**: If a data source is unavailable, state that explicitly rather than guessing.
