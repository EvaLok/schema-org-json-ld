import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Brand } from "../../src/schema/Brand";

describe("Brand", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Brand("ACME");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Brand");
		expect(obj.name).toBe("ACME");
		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "Brand",\n  "name": "ACME"\n}',
		);
	});

	it("includes description when provided", () => {
		const schema = new Brand("ACME", "ACME brand description");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.description).toBe("ACME brand description");
	});

	it("omits description when null", () => {
		const schema = new Brand("ACME");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("description");
	});
});
