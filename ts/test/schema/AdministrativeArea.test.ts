import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AdministrativeArea } from "../../src/schema/AdministrativeArea";

describe("AdministrativeArea", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new AdministrativeArea("California");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("AdministrativeArea");
		expect(obj.name).toBe("California");
	});

	it("has no optional fields to include or omit", () => {
		const schema = new AdministrativeArea("California");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual(["@context", "@type", "name"]);
	});

	it("includes all fields when set", () => {
		const schema = new AdministrativeArea("California");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("California");
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new AdministrativeArea("California");
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "AdministrativeArea",\n  "name": "California"\n}',
		);
	});
});
