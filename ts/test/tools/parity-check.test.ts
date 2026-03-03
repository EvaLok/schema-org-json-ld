import { readdirSync } from "node:fs";
import path from "node:path";
import { describe, expect, it, vi } from "vitest";

import { run } from "../../../tools/parity-check";

describe("parity-check tool", () => {
	it("reports schema and enum counts for both languages", () => {
		const repoRoot = path.resolve(__dirname, "..", "..", "..");
		const phpSchemaCount = readdirSync(
			path.join(repoRoot, "php/src/v1/Schema"),
		).filter((file) => file.endsWith(".php")).length;
		const tsSchemaCount = readdirSync(
			path.join(repoRoot, "ts/src/schema"),
		).filter((file) => file.endsWith(".ts")).length;
		const phpEnumCount = readdirSync(
			path.join(repoRoot, "php/src/v1/Enum"),
		).filter((file) => file.endsWith(".php")).length;
		const tsEnumCount = readdirSync(path.join(repoRoot, "ts/src/enum")).filter(
			(file) => file.endsWith(".ts"),
		).length;

		const output: string[] = [];
		const logSpy = vi
			.spyOn(console, "log")
			.mockImplementation((message: unknown) => {
				output.push(String(message));
			});
		const code = run();
		logSpy.mockRestore();

		expect(code === 0 || code === 1).toBe(true);
		expect(output.join("\n")).toContain("=== Schema Parity Report ===");
		expect(output.join("\n")).toContain(`PHP classes: ${phpSchemaCount}`);
		expect(output.join("\n")).toContain(`TS classes: ${tsSchemaCount}`);
		expect(output.join("\n")).toContain(
			`Enums: ${phpEnumCount}/${tsEnumCount}`,
		);
		expect(output.join("\n")).toContain("=== Summary ===");
	});
});
