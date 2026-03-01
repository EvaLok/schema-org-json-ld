import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { GeoCoordinates } from "../../src/schema/GeoCoordinates";

describe("GeoCoordinates", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new GeoCoordinates(40.7128, -74.006);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("GeoCoordinates");
		expect(obj.latitude).toBe(40.7128);
		expect(obj.longitude).toBe(-74.006);
	});

	it("has no optional fields to omit when null", () => {
		const schema = new GeoCoordinates(40.7128, -74.006);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual(["@context", "@type", "latitude", "longitude"]);
	});

	it("includes all fields when set", () => {
		const schema = new GeoCoordinates(40.7128, -74.006);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.latitude).toBe(40.7128);
		expect(obj.longitude).toBe(-74.006);
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new GeoCoordinates(40.7128, -74.006);
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "GeoCoordinates",\n  "latitude": 40.7128,\n  "longitude": -74.006\n}',
		);
	});
});
