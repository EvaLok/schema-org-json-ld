import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { GeoShape } from "../../src/schema/GeoShape";

describe("GeoShape", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new GeoShape({});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("GeoShape");
	});

	it("omits optional fields when null", () => {
		const schema = new GeoShape({ box: null });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("box");
	});

	it("includes all fields when set", () => {
		const schema = new GeoShape({
			box: "37.42242 -122.08585 37.42242 -122.08585",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.box).toBe("37.42242 -122.08585 37.42242 -122.08585");
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new GeoShape({
			box: "37.42242 -122.08585 37.42242 -122.08585",
		});
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "GeoShape",\n  "box": "37.42242 -122.08585 37.42242 -122.08585"\n}',
		);
	});

	it("includes only schema metadata when box is null", () => {
		const schema = new GeoShape({ box: null });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual(["@context", "@type"]);
	});

	it("serializes a valid box string", () => {
		const schema = new GeoShape({
			box: "-43.5 170.0 -35.0 178.6",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.box).toBe("-43.5 170.0 -35.0 178.6");
	});

	it("serializes an empty box string", () => {
		const schema = new GeoShape({ box: "" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.box).toBe("");
	});
});
