import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { ShippingDeliveryTime } from "../../src/schema/ShippingDeliveryTime";

describe("ShippingDeliveryTime", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ShippingDeliveryTime(
			new QuantitativeValue(1, "DAY"),
			new QuantitativeValue(3, "DAY"),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ShippingDeliveryTime");
	});

	it("includes all fields when set", () => {
		const schema = new ShippingDeliveryTime(
			new QuantitativeValue(1, "DAY"),
			new QuantitativeValue(3, "DAY"),
		);
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
		const schema = new ShippingDeliveryTime(
			new QuantitativeValue(null, null),
			new QuantitativeValue(3, "DAY"),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const handlingTime = obj.handlingTime as Record<string, unknown>;

		expect(handlingTime).not.toHaveProperty("value");
		expect(handlingTime).not.toHaveProperty("unitCode");
	});
});
