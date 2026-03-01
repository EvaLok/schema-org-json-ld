import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { SolveMathAction } from "../../src/schema/SolveMathAction";

describe("SolveMathAction", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new SolveMathAction(
			"https://example.com/solve",
			"x^2 + 2x + 1 = 0",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("SolveMathAction");
		expect(obj.target).toBe("https://example.com/solve");
		expect(obj["mathExpression-input"]).toBe("x^2 + 2x + 1 = 0");
		expect(obj).not.toHaveProperty("mathExpressionInput");
	});

	it("omits optional fields when null", () => {
		const schema = new SolveMathAction(
			"https://example.com/solve",
			"x^2 + 2x + 1 = 0",
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("eduQuestionType");
	});

	it("includes all fields when set", () => {
		const schema = new SolveMathAction(
			"https://example.com/solve",
			"x^2 + 2x + 1 = 0",
			"algebra",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.eduQuestionType).toBe("algebra");
	});

	it("supports eduQuestionType as string array", () => {
		const schema = new SolveMathAction(
			"https://example.com/solve",
			"x^2 + 2x + 1 = 0",
			["algebra", "quadratic"],
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.eduQuestionType).toEqual(["algebra", "quadratic"]);
	});
});
