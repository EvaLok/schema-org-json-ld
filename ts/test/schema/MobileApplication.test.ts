import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { MobileApplication } from "../../src/schema/MobileApplication";
import { Offer } from "../../src/schema/Offer";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";

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

	it("serializes free app offers with zero price", () => {
		const schema = new MobileApplication({
			name: "Free mobile app",
			offers: new Offer({
				url: "https://example.com/free-mobile",
				priceCurrency: "USD",
				price: 0,
				availability: ItemAvailability.InStock,
			}),
			aggregateRating: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>;

		expect(offers.price).toBe(0);
		expect(offers.priceCurrency).toBe("USD");
	});

	it("serializes aggregateRating with nested review", () => {
		const schema = new MobileApplication({
			name: "Rated mobile app",
			offers: offer,
			aggregateRating: new AggregateRating({
				ratingValue: 4.9,
				ratingCount: 2500,
			}),
			review: new Review({
				author: "Alex Reviewer",
				reviewRating: new Rating({ ratingValue: 5 }),
				reviewBody: "Great app experience.",
			}),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const aggregateRating = obj.aggregateRating as Record<string, unknown>;
		const review = obj.review as Record<string, unknown>;
		const reviewRating = review.reviewRating as Record<string, unknown>;

		expect(aggregateRating.ratingValue).toBe(4.9);
		expect(review.reviewBody).toBe("Great app experience.");
		expect(reviewRating.ratingValue).toBe(5);
	});

	it("serializes applicationCategory and operatingSystem together", () => {
		const schema = new MobileApplication({
			name: "Business mobile app",
			offers: offer,
			aggregateRating: null,
			applicationCategory: "BusinessApplication",
			operatingSystem: "iOS 17+",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj.applicationCategory).toBe("BusinessApplication");
		expect(obj.operatingSystem).toBe("iOS 17+");
	});
});
