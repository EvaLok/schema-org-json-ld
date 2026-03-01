import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { Offer } from "../../src/schema/Offer";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";
import { SoftwareApplication } from "../../src/schema/SoftwareApplication";

describe("SoftwareApplication", () => {
	const offer = new Offer({
		url: "https://example.com/app",
		priceCurrency: "USD",
		price: 0,
		availability: ItemAvailability.InStock,
	});

	it("produces minimal JSON-LD output with required fields and nullable aggregateRating", () => {
		const schema = new SoftwareApplication({
			name: "Example App",
			offers: offer,
			aggregateRating: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const offers = obj.offers as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("SoftwareApplication");
		expect(obj.name).toBe("Example App");
		expect(offers["@type"]).toBe("Offer");
		expect(obj).not.toHaveProperty("aggregateRating");
	});

	it("omits optional fields when null", () => {
		const schema = new SoftwareApplication({
			name: "Example App",
			offers: offer,
			aggregateRating: new AggregateRating(4.5),
			applicationCategory: null,
			operatingSystem: null,
			datePublished: null,
			review: null,
			description: null,
			screenshot: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("applicationCategory");
		expect(obj).not.toHaveProperty("review");
		expect(obj).not.toHaveProperty("screenshot");
	});

	it("supports aggregateRating object and offers array", () => {
		const schema = new SoftwareApplication({
			name: "Example App",
			offers: [offer],
			aggregateRating: new AggregateRating(4.5),
			review: new Review("Jane", new Rating(5)),
			description: "A productivity app",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>[];
		const aggregateRating = obj.aggregateRating as Record<string, unknown>;
		const review = obj.review as Record<string, unknown>;

		expect(offers[0]?.["@type"]).toBe("Offer");
		expect(aggregateRating["@type"]).toBe("AggregateRating");
		expect(review["@type"]).toBe("Review");
		expect(obj.description).toBe("A productivity app");
	});
});
