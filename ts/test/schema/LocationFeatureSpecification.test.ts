import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { LocationFeatureSpecification } from "../../src/schema/LocationFeatureSpecification";

describe("LocationFeatureSpecification", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new LocationFeatureSpecification("Parking", true);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("LocationFeatureSpecification");
		expect(obj.name).toBe("Parking");
		expect(obj.value).toBe(true);
	});

	it("includes all fields when set", () => {
		const schema = new LocationFeatureSpecification("Parking", "reserved");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("Parking");
		expect(obj.value).toBe("reserved");
	});

	it("supports boolean and string union values", () => {
		const booleanSchema = new LocationFeatureSpecification("WiFi", false);
		const stringSchema = new LocationFeatureSpecification("Pool", "seasonal");

		const booleanObj = JSON.parse(
			JsonLdGenerator.schemaToJson(booleanSchema),
		) as Record<string, unknown>;
		const stringObj = JSON.parse(
			JsonLdGenerator.schemaToJson(stringSchema),
		) as Record<string, unknown>;

		expect(booleanObj.value).toBe(false);
		expect(stringObj.value).toBe("seasonal");
	});
});
