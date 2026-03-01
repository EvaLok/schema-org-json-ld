import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { VirtualLocation } from "../../src/schema/VirtualLocation";

describe("VirtualLocation", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new VirtualLocation("https://example.com/live");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("VirtualLocation");
		expect(obj.url).toBe("https://example.com/live");
	});

	it("omits optional fields when null", () => {
		const schema = new VirtualLocation("https://example.com/live", null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("name");
	});

	it("includes all fields when set", () => {
		const schema = new VirtualLocation(
			"https://example.com/live",
			"Main Stage",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("Main Stage");
	});
});
