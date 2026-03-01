import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { MathSolver } from "../../src/schema/MathSolver";
import { SolveMathAction } from "../../src/schema/SolveMathAction";

describe("MathSolver", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new MathSolver(
			"https://example.com/math-solver",
			"https://example.com/usage",
			new SolveMathAction("https://example.com/solve", "x+1=2"),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const potentialAction = obj.potentialAction as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toEqual(["MathSolver", "LearningResource"]);
		expect(obj.url).toBe("https://example.com/math-solver");
		expect(obj.usageInfo).toBe("https://example.com/usage");
		expect(potentialAction["@type"]).toBe("SolveMathAction");
	});

	it("omits optional fields when null", () => {
		const schema = new MathSolver(
			"https://example.com/math-solver",
			"https://example.com/usage",
			new SolveMathAction("https://example.com/solve", "x+1=2"),
			null,
			null,
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("name");
		expect(obj).not.toHaveProperty("inLanguage");
		expect(obj).not.toHaveProperty("learningResourceType");
		expect(obj).not.toHaveProperty("assesses");
	});

	it("includes optional fields when set", () => {
		const schema = new MathSolver(
			"https://example.com/math-solver",
			"https://example.com/usage",
			[
				new SolveMathAction("https://example.com/solve/algebra", "x+1=2"),
				new SolveMathAction(
					"https://example.com/solve/geometry",
					"a^2+b^2=c^2",
				),
			],
			"Math Helper",
			"en",
			"Practice Problem",
			["Algebra", "Geometry"],
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const potentialAction = obj.potentialAction as Record<string, unknown>[];

		expect(obj.name).toBe("Math Helper");
		expect(obj.inLanguage).toBe("en");
		expect(obj.learningResourceType).toBe("Practice Problem");
		expect(obj.assesses).toEqual(["Algebra", "Geometry"]);
		expect(potentialAction).toHaveLength(2);
		expect(potentialAction[0]?.["@type"]).toBe("SolveMathAction");
		expect(potentialAction[1]?.["@type"]).toBe("SolveMathAction");
	});
});
