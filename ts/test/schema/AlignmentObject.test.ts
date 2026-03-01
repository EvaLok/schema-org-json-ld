import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AlignmentObject } from "../../src/schema/AlignmentObject";

describe("AlignmentObject", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new AlignmentObject("teaches", "Calculus");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("AlignmentObject");
		expect(obj.alignmentType).toBe("teaches");
		expect(obj.targetName).toBe("Calculus");
	});

	it("omits optional fields when null", () => {
		const schema = new AlignmentObject("teaches", "Calculus", null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("educationalFramework");
		expect(obj).not.toHaveProperty("targetUrl");
	});

	it("includes all fields when set", () => {
		const schema = new AlignmentObject(
			"teaches",
			"Calculus",
			"Common Core",
			"https://example.com/targets/calculus",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.educationalFramework).toBe("Common Core");
		expect(obj.targetUrl).toBe("https://example.com/targets/calculus");
	});
});
