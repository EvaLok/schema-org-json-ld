import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { PeopleAudience } from "../../src/schema/PeopleAudience";

describe("PeopleAudience", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new PeopleAudience();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("PeopleAudience");
	});

	it("omits optional fields when null", () => {
		const schema = new PeopleAudience(null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("suggestedGender");
		expect(obj).not.toHaveProperty("suggestedMinAge");
		expect(obj).not.toHaveProperty("suggestedMaxAge");
	});

	it("includes all fields when set", () => {
		const schema = new PeopleAudience("Female", 18, 34);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.suggestedGender).toBe("Female");
		expect(obj.suggestedMinAge).toBe(18);
		expect(obj.suggestedMaxAge).toBe(34);
	});
});
