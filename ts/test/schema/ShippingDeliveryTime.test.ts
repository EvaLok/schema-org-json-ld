import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { ShippingDeliveryTime } from "../../src/schema/ShippingDeliveryTime";

describe("ShippingDeliveryTime", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ShippingDeliveryTime({
			handlingTime: new QuantitativeValue({ value: 1, unitCode: "DAY" }),
			transitTime: new QuantitativeValue({ value: 3, unitCode: "DAY" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ShippingDeliveryTime");
	});

	it("includes all fields when set", () => {
		const schema = new ShippingDeliveryTime({
			handlingTime: new QuantitativeValue({ value: 1, unitCode: "DAY" }),
			transitTime: new QuantitativeValue({ value: 3, unitCode: "DAY" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const handlingTime = obj.handlingTime as Record<string, unknown>;
		const transitTime = obj.transitTime as Record<string, unknown>;

		expect(handlingTime["@type"]).toBe("QuantitativeValue");
		expect(handlingTime.value).toBe(1);
		expect(handlingTime.unitCode).toBe("DAY");
		expect(transitTime["@type"]).toBe("QuantitativeValue");
		expect(transitTime.value).toBe(3);
		expect(transitTime.unitCode).toBe("DAY");
	});

	it("omits null fields from nested QuantitativeValue objects", () => {
		const schema = new ShippingDeliveryTime({
			handlingTime: new QuantitativeValue({ value: null, unitCode: null }),
			transitTime: new QuantitativeValue({ value: 3, unitCode: "DAY" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const handlingTime = obj.handlingTime as Record<string, unknown>;

		expect(handlingTime).not.toHaveProperty("value");
		expect(handlingTime).not.toHaveProperty("unitCode");
	});

	it("serializes nested QuantitativeValue types with min and max values", () => {
		const schema = new ShippingDeliveryTime({
			handlingTime: new QuantitativeValue({
				minValue: 1,
				maxValue: 2,
				unitCode: "DAY",
			}),
			transitTime: new QuantitativeValue({
				minValue: 3,
				maxValue: 5,
				unitCode: "DAY",
			}),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const handlingTime = obj.handlingTime as Record<string, unknown>;
		const transitTime = obj.transitTime as Record<string, unknown>;

		expect(handlingTime["@type"]).toBe("QuantitativeValue");
		expect(handlingTime.minValue).toBe(1);
		expect(handlingTime.maxValue).toBe(2);
		expect(handlingTime.unitCode).toBe("DAY");
		expect(transitTime["@type"]).toBe("QuantitativeValue");
		expect(transitTime.minValue).toBe(3);
		expect(transitTime.maxValue).toBe(5);
		expect(transitTime.unitCode).toBe("DAY");
	});

	it("serializes zero values in nested QuantitativeValue objects", () => {
		const schema = new ShippingDeliveryTime({
			handlingTime: new QuantitativeValue({
				minValue: 0,
				maxValue: 0,
				unitCode: "DAY",
			}),
			transitTime: new QuantitativeValue({
				minValue: 0,
				maxValue: 0,
				unitCode: "DAY",
			}),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const handlingTime = obj.handlingTime as Record<string, unknown>;
		const transitTime = obj.transitTime as Record<string, unknown>;

		expect(handlingTime.minValue).toBe(0);
		expect(handlingTime.maxValue).toBe(0);
		expect(transitTime.minValue).toBe(0);
		expect(transitTime.maxValue).toBe(0);
	});

	it("preserves the nested quantitative value schema type for empty objects", () => {
		const schema = new ShippingDeliveryTime({
			handlingTime: new QuantitativeValue({}),
			transitTime: new QuantitativeValue({ value: 3, unitCode: "DAY" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const handlingTime = obj.handlingTime as Record<string, unknown>;

		expect(handlingTime["@type"]).toBe("QuantitativeValue");
		expect(handlingTime).not.toHaveProperty("value");
		expect(handlingTime).not.toHaveProperty("unitCode");
	});
});
