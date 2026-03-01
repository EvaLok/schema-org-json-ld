import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { Offer } from "../../src/schema/Offer";
import { WebApplication } from "../../src/schema/WebApplication";

describe("WebApplication", () => {
	const offer = new Offer({
		url: "https://example.com/app",
		priceCurrency: "USD",
		price: 1.99,
		availability: ItemAvailability.InStock,
	});

	it("produces WebApplication JSON-LD with required fields", () => {
		const schema = new WebApplication({
			name: "Example web app",
			offers: offer,
			aggregateRating: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("WebApplication");
		expect(obj.name).toBe("Example web app");
	});

	it("omits inherited optional fields when null", () => {
		const schema = new WebApplication({
			name: "Example web app",
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
		expect(obj).not.toHaveProperty("review");
	});

	it("inherits SoftwareApplication options while preserving WebApplication type", () => {
		const schema = new WebApplication({
			name: "Example web app",
			offers: [offer],
			aggregateRating: null,
			applicationCategory: "BusinessApplication",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>[];

		expect(obj["@type"]).toBe("WebApplication");
		expect(offers[0]?.["@type"]).toBe("Offer");
		expect(obj.applicationCategory).toBe("BusinessApplication");
	});
});
