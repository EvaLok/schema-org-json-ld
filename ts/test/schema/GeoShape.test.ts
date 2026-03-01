import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { GeoShape } from "../../src/schema/GeoShape";

describe("GeoShape", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new GeoShape();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("GeoShape");
	});

	it("omits optional fields when null", () => {
		const schema = new GeoShape(null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("box");
	});

	it("includes all fields when set", () => {
		const schema = new GeoShape("37.42242 -122.08585 37.42242 -122.08585");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.box).toBe("37.42242 -122.08585 37.42242 -122.08585");
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new GeoShape("37.42242 -122.08585 37.42242 -122.08585");
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "GeoShape",\n  "box": "37.42242 -122.08585 37.42242 -122.08585"\n}',
		);
	});
});
