import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { EducationalOccupationalCredential } from "../../src/schema/EducationalOccupationalCredential";

describe("EducationalOccupationalCredential", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new EducationalOccupationalCredential({
			credentialCategory: "bachelor degree",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("EducationalOccupationalCredential");
		expect(obj.credentialCategory).toBe("bachelor degree");
	});

	it("serializes an empty credential category", () => {
		const schema = new EducationalOccupationalCredential({
			credentialCategory: "",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.credentialCategory).toBe("");
	});

	it("only includes @context, @type, and credentialCategory", () => {
		const schema = new EducationalOccupationalCredential({
			credentialCategory: "industry certificate",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual([
			"@context",
			"@type",
			"credentialCategory",
		]);
	});

	it("preserves the exact credentialCategory value", () => {
		const schema = new EducationalOccupationalCredential({
			credentialCategory: "Level 3 - advanced certification",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.credentialCategory).toBe("Level 3 - advanced certification");
	});
});
