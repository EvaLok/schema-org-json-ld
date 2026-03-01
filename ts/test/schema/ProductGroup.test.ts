import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { Brand } from "../../src/schema/Brand";
import { Offer } from "../../src/schema/Offer";
import { Product } from "../../src/schema/Product";
import { ProductGroup } from "../../src/schema/ProductGroup";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";

describe("ProductGroup", () => {
	const offer = new Offer({
		url: "https://example.com/product",
		priceCurrency: "USD",
		price: 19.99,
		availability: ItemAvailability.InStock,
	});

	const product = new Product({
		name: "T-Shirt Blue S",
		image: ["https://example.com/blue.jpg"],
		description: "Blue shirt size S",
		sku: "SKU-BLUE-S",
		offers: [offer],
	});

	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ProductGroup({ name: "T-Shirt" });
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ProductGroup");
		expect(obj.name).toBe("T-Shirt");
	});

	it("omits optional fields when null", () => {
		const schema = new ProductGroup({
			name: "T-Shirt",
			productGroupID: null,
			variesBy: null,
			hasVariant: null,
			url: null,
			description: null,
			brand: null,
			aggregateRating: null,
			review: null,
			subjectOf: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("productGroupID");
		expect(obj).not.toHaveProperty("hasVariant");
		expect(obj).not.toHaveProperty("aggregateRating");
	});

	it("supports single hasVariant and string variesBy", () => {
		const schema = new ProductGroup({
			name: "T-Shirt",
			productGroupID: "TSHIRT-001",
			variesBy: "color",
			hasVariant: product,
			brand: new Brand("BrandCo"),
			aggregateRating: new AggregateRating(4.4),
			review: new Review("Jane", new Rating(5)),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const hasVariant = obj.hasVariant as Record<string, unknown>;

		expect(obj.variesBy).toBe("color");
		expect(hasVariant["@type"]).toBe("Product");
		expect(hasVariant.name).toBe("T-Shirt Blue S");
	});

	it("supports hasVariant and variesBy as arrays", () => {
		const schema = new ProductGroup({
			name: "T-Shirt",
			variesBy: ["color", "size"],
			hasVariant: [product],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const hasVariant = obj.hasVariant as Record<string, unknown>[];

		expect(obj.variesBy).toEqual(["color", "size"]);
		expect(hasVariant[0]?.["@type"]).toBe("Product");
	});
});
