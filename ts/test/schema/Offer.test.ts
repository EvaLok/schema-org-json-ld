import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { MerchantReturnEnumeration } from "../../src/enum/MerchantReturnEnumeration";
import { OfferItemCondition } from "../../src/enum/OfferItemCondition";
import { DefinedRegion } from "../../src/schema/DefinedRegion";
import { MerchantReturnPolicy } from "../../src/schema/MerchantReturnPolicy";
import { Offer } from "../../src/schema/Offer";
import { OfferShippingDetails } from "../../src/schema/OfferShippingDetails";
import { UnitPriceSpecification } from "../../src/schema/UnitPriceSpecification";

describe("Offer", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Offer({
			url: "https://example.com/product",
			priceCurrency: "USD",
			price: 19.99,
			availability: ItemAvailability.InStock,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Offer");
		expect(obj.url).toBe("https://example.com/product");
		expect(obj.priceCurrency).toBe("USD");
		expect(obj.price).toBe(19.99);
		expect(obj.availability).toBe("https://schema.org/InStock");
	});

	it("omits optional fields when null", () => {
		const schema = new Offer({
			url: "https://example.com/product",
			priceCurrency: "USD",
			price: 19.99,
			availability: ItemAvailability.InStock,
			itemCondition: null,
			shippingDetails: null,
			validFrom: null,
			priceValidUntil: null,
			priceSpecification: null,
			hasMerchantReturnPolicy: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("itemCondition");
		expect(obj).not.toHaveProperty("shippingDetails");
		expect(obj).not.toHaveProperty("validFrom");
		expect(obj).not.toHaveProperty("priceValidUntil");
		expect(obj).not.toHaveProperty("priceSpecification");
		expect(obj).not.toHaveProperty("hasMerchantReturnPolicy");
	});

	it("includes optional fields when set", () => {
		const schema = new Offer({
			url: "https://example.com/product",
			priceCurrency: "USD",
			price: 19.99,
			availability: ItemAvailability.InStock,
			itemCondition: OfferItemCondition.NewCondition,
			shippingDetails: [new OfferShippingDetails(new DefinedRegion("US"))],
			validFrom: "2026-02-01",
			priceValidUntil: "2026-12-31",
			priceSpecification: new UnitPriceSpecification(19.99, "USD"),
			hasMerchantReturnPolicy: new MerchantReturnPolicy({
				applicableCountry: "US",
				returnPolicyCategory:
					MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			}),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingDetails = obj.shippingDetails as Record<string, unknown>[];
		const priceSpecification = obj.priceSpecification as Record<
			string,
			unknown
		>;
		const hasMerchantReturnPolicy = obj.hasMerchantReturnPolicy as Record<
			string,
			unknown
		>;

		expect(obj.itemCondition).toBe("https://schema.org/NewCondition");
		expect(shippingDetails[0]?.["@type"]).toBe("OfferShippingDetails");
		expect(obj.validFrom).toBe("2026-02-01");
		expect(obj.priceValidUntil).toBe("2026-12-31");
		expect(priceSpecification["@type"]).toBe("UnitPriceSpecification");
		expect(hasMerchantReturnPolicy["@type"]).toBe("MerchantReturnPolicy");
	});

	it("supports priceSpecification as array", () => {
		const schema = new Offer({
			url: "https://example.com/product",
			priceCurrency: "USD",
			price: 19.99,
			availability: ItemAvailability.InStock,
			priceSpecification: [
				new UnitPriceSpecification(19.99, "USD"),
				new UnitPriceSpecification(17.99, "USD", "Sale"),
			],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const priceSpecification = obj.priceSpecification as Record<
			string,
			unknown
		>[];

		expect(priceSpecification).toHaveLength(2);
		expect(priceSpecification[0]?.["@type"]).toBe("UnitPriceSpecification");
		expect(priceSpecification[1]?.["@type"]).toBe("UnitPriceSpecification");
	});
});
