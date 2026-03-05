---
name: tool-creation-guidelines
description: Best practices for creating developer tools and scripts in this project. Use when planning or reviewing tools that parse, analyze, or transform code.
user-invocable: false
---

# Tool Creation Guidelines

Best practices for creating developer tools and scripts in this project. These apply to tools in `tools/` and `scripts/` directories.

## Tool-first philosophy

**Tools are the default, not the exception.** If you perform a task manually more than once, build a tool. Even for tasks that require thought and judgment, build "groundwork" tools that gather data, structure inputs, and present summaries — so the orchestrator can spend its effort on reasoning rather than mechanical data collection.

Examples of groundwork tools:
- A tool that scans Eva's recent comments and presents structured output (saves API query construction)
- A tool that diffs Google docs properties against codebase implementations (saves manual comparison)
- A tool that summarizes PR changes with relevant context (saves reading raw diffs)
- A tool that checks all startup checklist steps and reports only anomalies (saves running each step manually)

**Do not gate tool creation on frequency alone.** A tool that saves 2 minutes per cycle pays for itself within a few cycles. The cost of building a small Rust tool is low; the cost of not building it compounds across every future cycle.

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

## Rust tools

For tools that don't need source code parsing (JSON processing, state verification, report generation, API query automation), **Rust is the preferred language**. Rust tools:
- Run in the orchestrator sandbox (`bash tools/<name>` and `cargo *` are both allowed)
- Avoid the sandbox restrictions that block tools like `jq -f`
- Compile to fast, standalone binaries with no runtime dependencies
- Live in the `tools/rust/` Cargo workspace

See the **rust-tooling** skill for the full workflow (workspace structure, shell wrappers, conventions).

## General tool principles

1. **Use existing dependencies first.** Check `package.json` and `composer.json` before adding new packages. TypeScript is already installed — its compiler API is available for free.
2. **Rust for non-AST tools.** For JSON processing, state verification, or report generation, prefer Rust (`tools/rust/crates/`) over jq or shell scripts. Rust tools can run in the orchestrator sandbox; `jq -f` and complex shell constructs cannot.
3. **Ask Eva for help.** If you need new dependencies installed, workflow changes, or permissions — create an issue. Eva is part of the team.
4. **Keep tools focused.** One tool, one purpose. Don't combine unrelated analyses into a single script.
5. **CI-ready exit codes.** Tools should exit 0 on success, non-zero on failure. This enables CI integration.
6. **Preserve output format.** When rewriting a tool's internals, keep the same output format unless there's a specific reason to change it.
7. **Dispatch tool-building to the coding agent.** For non-trivial tools, create an issue spec describing the tool's purpose, inputs, outputs, and constraints, then assign it to the Copilot agent. The agent can build Rust tools, TypeScript analysis scripts, and pipeline components.

## Pipeline composition

Tools should be designed to compose into pipelines:

- **Structured output**: Tools should support `--json` output for machine consumption alongside human-readable output.
- **Exit codes**: Exit 0 on success, non-zero on failure. Pipelines can short-circuit on failure.
- **Consistent `--repo-root` convention**: All tools accept `--repo-root <path>` to locate repo files.
- **Idempotent**: Running a tool twice with the same inputs should produce the same output.
- **Composable**: A tool's output should be usable as input to another tool or to the orchestrator's reasoning.
