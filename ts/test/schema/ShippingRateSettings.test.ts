import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ShippingRateSettings } from "../../src/schema/ShippingRateSettings";

describe("ShippingRateSettings", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ShippingRateSettings();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ShippingRateSettings");
	});

	it("omits optional fields when null", () => {
		const schema = new ShippingRateSettings(null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("orderPercentage");
		expect(obj).not.toHaveProperty("weightPercentage");
	});

	it("includes all fields when set", () => {
		const schema = new ShippingRateSettings(2.5, 1.25);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.orderPercentage).toBe(2.5);
		expect(obj.weightPercentage).toBe(1.25);
	});
});
