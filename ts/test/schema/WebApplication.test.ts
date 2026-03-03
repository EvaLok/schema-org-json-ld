import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { Offer } from "../../src/schema/Offer";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";
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

	it("serializes free app offers with zero price", () => {
		const schema = new WebApplication({
			name: "Free web app",
			offers: new Offer({
				url: "https://example.com/free-web",
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
		const schema = new WebApplication({
			name: "Rated web app",
			offers: offer,
			aggregateRating: new AggregateRating({
				ratingValue: 4.9,
				ratingCount: 2400,
			}),
			review: new Review({
				author: "Alex Reviewer",
				reviewRating: new Rating({ ratingValue: 5 }),
				reviewBody: "Great web app experience.",
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
		expect(review.reviewBody).toBe("Great web app experience.");
		expect(reviewRating.ratingValue).toBe(5);
	});

	it("serializes applicationCategory and operatingSystem together", () => {
		const schema = new WebApplication({
			name: "Business web app",
			offers: offer,
			aggregateRating: null,
			applicationCategory: "BusinessApplication",
			operatingSystem: "Web",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj.applicationCategory).toBe("BusinessApplication");
		expect(obj.operatingSystem).toBe("Web");
	});
});
