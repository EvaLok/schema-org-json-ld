import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DefinedRegion } from "../../src/schema/DefinedRegion";

describe("DefinedRegion", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new DefinedRegion("US");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("DefinedRegion");
		expect(obj.addressCountry).toBe("US");
	});

	it("omits optional fields when null", () => {
		const schema = new DefinedRegion("US", null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("addressRegion");
		expect(obj).not.toHaveProperty("postalCode");
	});

	it("includes all fields when set", () => {
		const schema = new DefinedRegion("US", "NY", "10001");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.addressRegion).toBe("NY");
		expect(obj.postalCode).toBe("10001");
	});

	it("supports addressRegion as string array", () => {
		const schema = new DefinedRegion("US", ["CA", "NV", "AZ"]);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.addressRegion).toEqual(["CA", "NV", "AZ"]);
	});
});
