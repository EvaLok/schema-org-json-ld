import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Thing } from "../../src/schema/Thing";

describe("Thing", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Thing("Generic Thing");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Thing");
		expect(obj.name).toBe("Generic Thing");
	});

	it("has no optional fields to include in output", () => {
		const schema = new Thing("Only Required");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual(["@context", "@type", "name"]);
	});

	it("includes all fields when set", () => {
		const schema = new Thing("Complete Thing");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("Complete Thing");
	});
});
