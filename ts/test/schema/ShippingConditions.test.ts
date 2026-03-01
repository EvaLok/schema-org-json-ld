import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DayOfWeek } from "../../src/enum/DayOfWeek";
import { DefinedRegion } from "../../src/schema/DefinedRegion";
import { MonetaryAmount } from "../../src/schema/MonetaryAmount";
import { OpeningHoursSpecification } from "../../src/schema/OpeningHoursSpecification";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { ServicePeriod } from "../../src/schema/ServicePeriod";
import { ShippingConditions } from "../../src/schema/ShippingConditions";
import { ShippingRateSettings } from "../../src/schema/ShippingRateSettings";

describe("ShippingConditions", () => {
	it("produces minimal JSON-LD output", () => {
		const schema = new ShippingConditions();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ShippingConditions");
	});

	it("omits optional fields when null", () => {
		const schema = new ShippingConditions({
			doesNotShip: null,
			numItems: null,
			orderValue: null,
			shippingDestination: null,
			shippingOrigin: null,
			seasonalOverride: null,
			shippingRate: null,
			transitTime: null,
			weight: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("doesNotShip");
		expect(obj).not.toHaveProperty("numItems");
		expect(obj).not.toHaveProperty("orderValue");
		expect(obj).not.toHaveProperty("shippingDestination");
		expect(obj).not.toHaveProperty("shippingOrigin");
		expect(obj).not.toHaveProperty("seasonalOverride");
		expect(obj).not.toHaveProperty("shippingRate");
		expect(obj).not.toHaveProperty("transitTime");
		expect(obj).not.toHaveProperty("weight");
	});

	it("supports shippingRate as ShippingRateSettings", () => {
		const schema = new ShippingConditions({
			shippingRate: new ShippingRateSettings(10, null),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingRate = obj.shippingRate as Record<string, unknown>;

		expect(shippingRate["@type"]).toBe("ShippingRateSettings");
		expect(shippingRate.orderPercentage).toBe(10);
	});

	it("supports shippingRate as MonetaryAmount", () => {
		const schema = new ShippingConditions({
			shippingRate: new MonetaryAmount("USD", 7.99),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingRate = obj.shippingRate as Record<string, unknown>;

		expect(shippingRate["@type"]).toBe("MonetaryAmount");
		expect(shippingRate.value).toBe(7.99);
	});

	it("includes all fields when set", () => {
		const schema = new ShippingConditions({
			doesNotShip: false,
			numItems: new QuantitativeValue(2, "C62"),
			orderValue: new MonetaryAmount("USD", 50),
			shippingDestination: new DefinedRegion("US"),
			shippingOrigin: new DefinedRegion("US", "NY"),
			seasonalOverride: new OpeningHoursSpecification(
				DayOfWeek.Friday,
				"09:00",
				"17:00",
			),
			shippingRate: new ShippingRateSettings(5, 2),
			transitTime: new ServicePeriod(new QuantitativeValue(2, "DAY")),
			weight: new QuantitativeValue(20, "KGM"),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingDestination = obj.shippingDestination as Record<
			string,
			unknown
		>;

		expect(obj.doesNotShip).toBe(false);
		expect(shippingDestination["@type"]).toBe("DefinedRegion");
		expect(obj).toHaveProperty("transitTime");
		expect(obj).toHaveProperty("weight");
	});
});
