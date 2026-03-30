import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ShippingRateSettings } from "../../src/schema/ShippingRateSettings";

describe("ShippingRateSettings", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ShippingRateSettings({});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ShippingRateSettings");
	});

	it("omits optional fields when null", () => {
		const schema = new ShippingRateSettings({
			orderPercentage: null,
			weightPercentage: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("orderPercentage");
		expect(obj).not.toHaveProperty("weightPercentage");
	});

	it("includes all fields when set", () => {
		const schema = new ShippingRateSettings({
			orderPercentage: 2.5,
			weightPercentage: 1.25,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.orderPercentage).toBe(2.5);
		expect(obj.weightPercentage).toBe(1.25);
	});

	it("serializes only orderPercentage when weightPercentage is omitted", () => {
		const schema = new ShippingRateSettings({
			orderPercentage: 1.5,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.orderPercentage).toBe(1.5);
		expect(obj).not.toHaveProperty("weightPercentage");
	});

	it("serializes zero values", () => {
		const schema = new ShippingRateSettings({
			orderPercentage: 0,
			weightPercentage: 0,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.orderPercentage).toBe(0);
		expect(obj.weightPercentage).toBe(0);
	});

	it("preserves decimal precision", () => {
		const schema = new ShippingRateSettings({
			orderPercentage: 2.3456,
			weightPercentage: 7.8912,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.orderPercentage).toBe(2.3456);
		expect(obj.weightPercentage).toBe(7.8912);
	});
});
