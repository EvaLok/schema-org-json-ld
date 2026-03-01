import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { AggregateOffer } from "../../src/schema/AggregateOffer";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { Brand } from "../../src/schema/Brand";
import { Certification } from "../../src/schema/Certification";
import { Offer } from "../../src/schema/Offer";
import { Organization } from "../../src/schema/Organization";
import { PeopleAudience } from "../../src/schema/PeopleAudience";
import { Product } from "../../src/schema/Product";
import { ProductGroup } from "../../src/schema/ProductGroup";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";
import { SizeSpecification } from "../../src/schema/SizeSpecification";

describe("Product", () => {
	const offer = new Offer({
		url: "https://example.com/product",
		priceCurrency: "USD",
		price: 19.99,
		availability: ItemAvailability.InStock,
	});

	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Product({
			name: "T-Shirt",
			image: ["https://example.com/image.jpg"],
			description: "Blue shirt",
			sku: "SKU-123",
			offers: [offer],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>[];

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Product");
		expect(obj.name).toBe("T-Shirt");
		expect(offers[0]?.["@type"]).toBe("Offer");
	});

	it("omits optional fields when null", () => {
		const schema = new Product({
			name: "T-Shirt",
			image: ["https://example.com/image.jpg"],
			description: "Blue shirt",
			sku: "SKU-123",
			offers: [offer],
			brand: null,
			mpn: null,
			weight: null,
			aggregateRating: null,
			review: null,
			color: null,
			material: null,
			pattern: null,
			size: null,
			inProductGroupWithID: null,
			gtin: null,
			gtin8: null,
			gtin12: null,
			gtin13: null,
			gtin14: null,
			isbn: null,
			isVariantOf: null,
			audience: null,
			hasCertification: null,
			subjectOf: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("brand");
		expect(obj).not.toHaveProperty("review");
		expect(obj).not.toHaveProperty("hasCertification");
	});

	it("supports AggregateOffer, single review/certification, SizeSpecification, and isVariantOf", () => {
		const schema = new Product({
			name: "T-Shirt",
			image: ["https://example.com/image.jpg"],
			description: "Blue shirt",
			sku: "SKU-123",
			offers: new AggregateOffer(10, "USD", 20, 5),
			brand: new Brand("BrandCo"),
			weight: new QuantitativeValue(1, "KGM"),
			aggregateRating: new AggregateRating(4.5),
			review: new Review("Jane", new Rating(5)),
			size: new SizeSpecification("M"),
			isVariantOf: new ProductGroup({ name: "T-Shirt family" }),
			audience: new PeopleAudience("unisex"),
			hasCertification: new Certification(
				"Eco Certified",
				new Organization({ name: "Cert Org" }),
			),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>;
		const review = obj.review as Record<string, unknown>;
		const hasCertification = obj.hasCertification as Record<string, unknown>;
		const size = obj.size as Record<string, unknown>;
		const isVariantOf = obj.isVariantOf as Record<string, unknown>;

		expect(offers["@type"]).toBe("AggregateOffer");
		expect(review["@type"]).toBe("Review");
		expect(hasCertification["@type"]).toBe("Certification");
		expect(size["@type"]).toBe("SizeSpecification");
		expect(isVariantOf["@type"]).toBe("ProductGroup");
	});

	it("supports Offer[] offers, review/certification arrays, and string size", () => {
		const schema = new Product({
			name: "T-Shirt",
			image: ["https://example.com/image.jpg"],
			description: "Blue shirt",
			sku: "SKU-123",
			offers: [offer],
			review: [new Review("Jane", new Rating(5))],
			hasCertification: [
				new Certification(
					"Eco Certified",
					new Organization({ name: "Cert Org" }),
				),
			],
			size: "M",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>[];
		const review = obj.review as Record<string, unknown>[];
		const hasCertification = obj.hasCertification as Record<string, unknown>[];

		expect(offers[0]?.["@type"]).toBe("Offer");
		expect(review[0]?.["@type"]).toBe("Review");
		expect(hasCertification[0]?.["@type"]).toBe("Certification");
		expect(obj.size).toBe("M");
	});
});
