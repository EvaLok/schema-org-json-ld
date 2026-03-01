import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DefinedRegion } from "../../src/schema/DefinedRegion";
import { MonetaryAmount } from "../../src/schema/MonetaryAmount";
import { OfferShippingDetails } from "../../src/schema/OfferShippingDetails";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { ShippingDeliveryTime } from "../../src/schema/ShippingDeliveryTime";

describe("OfferShippingDetails", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new OfferShippingDetails(new DefinedRegion("US"));
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingDestination = obj.shippingDestination as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("OfferShippingDetails");
		expect(shippingDestination["@type"]).toBe("DefinedRegion");
		expect(shippingDestination.addressCountry).toBe("US");
	});

	it("omits optional fields when null", () => {
		const schema = new OfferShippingDetails(
			new DefinedRegion("US"),
			null,
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("shippingRate");
		expect(obj).not.toHaveProperty("deliveryTime");
		expect(obj).not.toHaveProperty("doesNotShip");
	});

	it("includes all fields when set", () => {
		const schema = new OfferShippingDetails(
			new DefinedRegion("US", "CA"),
			new MonetaryAmount("USD", 4.99),
			new ShippingDeliveryTime(
				new QuantitativeValue(1, "DAY"),
				new QuantitativeValue(3, "DAY"),
			),
			false,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingRate = obj.shippingRate as Record<string, unknown>;
		const deliveryTime = obj.deliveryTime as Record<string, unknown>;

		expect(shippingRate["@type"]).toBe("MonetaryAmount");
		expect(shippingRate.value).toBe(4.99);
		expect(deliveryTime["@type"]).toBe("ShippingDeliveryTime");
		expect(obj.doesNotShip).toBe(false);
	});
});
