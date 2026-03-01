import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { MobileApplication } from "../../src/schema/MobileApplication";
import { Offer } from "../../src/schema/Offer";

describe("MobileApplication", () => {
	const offer = new Offer({
		url: "https://example.com/app",
		priceCurrency: "USD",
		price: 1.99,
		availability: ItemAvailability.InStock,
	});

	it("produces MobileApplication JSON-LD with required fields", () => {
		const schema = new MobileApplication({
			name: "Example mobile app",
			offers: offer,
			aggregateRating: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("MobileApplication");
		expect(obj.name).toBe("Example mobile app");
	});

	it("omits inherited optional fields when null", () => {
		const schema = new MobileApplication({
			name: "Example mobile app",
			offers: offer,
			aggregateRating: null,
			applicationCategory: null,
			operatingSystem: null,
			datePublished: null,
			review: null,
			description: null,
			screenshot: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("applicationCategory");
		expect(obj).not.toHaveProperty("description");
	});

	it("inherits SoftwareApplication options while preserving MobileApplication type", () => {
		const schema = new MobileApplication({
			name: "Example mobile app",
			offers: [offer],
			aggregateRating: null,
			operatingSystem: "iOS",
			description: "Great app",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>[];

		expect(obj["@type"]).toBe("MobileApplication");
		expect(offers[0]?.["@type"]).toBe("Offer");
		expect(obj.operatingSystem).toBe("iOS");
	});
});
