---
name: tool-creation-guidelines
description: Best practices for creating developer tools and scripts in this project. Use when planning or reviewing tools that parse, analyze, or transform code.
user-invocable: false
---

# Tool Creation Guidelines

Best practices for creating developer tools and scripts in this project. These apply to tools in `tools/` and `scripts/` directories.

## Use AST parsers, not regex

**When parsing source code (PHP, TypeScript, or any language), always use established AST parsing libraries** instead of regex-based string matching. This is a firm project convention (Eva directive #378).

### Why

- Regex is fragile against formatting changes (whitespace, comments, multiline constructs)
- Regex cannot handle nested structures reliably (brackets, generics, unions)
- Regex reinvents parsing that battle-tested libraries already solve
- AST parsers handle edge cases (e.g., `extends` clauses, complex type annotations) that regex misses

### Available parsers in this project

| Language | Parser | Package | Notes |
|----------|--------|---------|-------|
| TypeScript | TypeScript Compiler API | `typescript` (already installed) | Use `ts.createSourceFile()` to parse TS/JS files |
| PHP | php-parser (glayzzle) | `php-parser` (dev dependency) | Pure JS PHP 8.x parser |

### What NOT to do

- Do NOT use regex to extract class declarations, constructor parameters, interface properties, enum cases, or type annotations from source files
- Do NOT write custom bracket-depth-tracking string splitters (e.g., `splitTopLevel()`) — the AST handles nesting correctly
- Do NOT parse `PROPERTY_MAP` or `A_SCHEMA_TYPE` constants with regex when an AST gives you structured constant declarations

### When regex is acceptable

Regex is fine for:
- Simple pattern matching in output/log text (not source code)
- Filtering file names or paths
- Post-processing already-extracted string values (e.g., normalising type names)

## General tool principles

1. **Use existing dependencies first.** Check `package.json` and `composer.json` before adding new packages. TypeScript is already installed — its compiler API is available for free.
2. **Ask Eva for help.** If you need new dependencies installed, workflow changes, or permissions — create an issue. Eva is part of the team.
3. **Keep tools focused.** One tool, one purpose. Don't combine unrelated analyses into a single script.
4. **CI-ready exit codes.** Tools should exit 0 on success, non-zero on failure. This enables CI integration.
5. **Preserve output format.** When rewriting a tool's internals, keep the same output format unless there's a specific reason to change it.
