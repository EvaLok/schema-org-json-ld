import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { GeoCoordinates } from "../../src/schema/GeoCoordinates";
import { LocalBusiness } from "../../src/schema/LocalBusiness";
import { OpeningHoursSpecification } from "../../src/schema/OpeningHoursSpecification";
import { PostalAddress } from "../../src/schema/PostalAddress";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";

describe("LocalBusiness", () => {
	const address = new PostalAddress({
		streetAddress: "1 Main St",
		addressLocality: "Townsville",
		addressCountry: "US",
	});

	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new LocalBusiness({ name: "Cafe", address });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const addressObj = obj.address as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("LocalBusiness");
		expect(obj.name).toBe("Cafe");
		expect(addressObj["@type"]).toBe("PostalAddress");
	});

	it("omits optional fields when null", () => {
		const schema = new LocalBusiness({
			name: "Cafe",
			address,
			url: null,
			telephone: null,
			description: null,
			image: null,
			priceRange: null,
			geo: null,
			openingHoursSpecification: null,
			aggregateRating: null,
			review: null,
			menu: null,
			servesCuisine: null,
			logo: null,
			email: null,
			sameAs: null,
			department: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("url");
		expect(obj).not.toHaveProperty("review");
		expect(obj).not.toHaveProperty("department");
	});

	it("supports department as single and array", () => {
		const singleDepartment = new LocalBusiness({
			name: "Cafe",
			address,
			department: new LocalBusiness({
				name: "Bakery",
				address,
			}),
		});
		const singleObj = JSON.parse(
			JsonLdGenerator.schemaToJson(singleDepartment),
		) as Record<string, unknown>;
		const singleDepartmentObj = singleObj.department as Record<string, unknown>;
		expect(singleDepartmentObj["@type"]).toBe("LocalBusiness");

		const arrayDepartment = new LocalBusiness({
			name: "Cafe",
			address,
			department: [
				new LocalBusiness({
					name: "Bakery",
					address,
				}),
			],
		});
		const arrayObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arrayDepartment),
		) as Record<string, unknown>;
		const departments = arrayObj.department as Record<string, unknown>[];
		expect(departments[0]?.["@type"]).toBe("LocalBusiness");
	});

	it("supports review as single and array", () => {
		const singleReviewSchema = new LocalBusiness({
			name: "Cafe",
			address,
			geo: new GeoCoordinates(40.0, -73.0),
			openingHoursSpecification: [new OpeningHoursSpecification()],
			aggregateRating: new AggregateRating(4.2),
			review: new Review("Jane", new Rating(5)),
		});
		const singleReviewObj = JSON.parse(
			JsonLdGenerator.schemaToJson(singleReviewSchema),
		) as Record<string, unknown>;
		const singleReview = singleReviewObj.review as Record<string, unknown>;
		expect(singleReview["@type"]).toBe("Review");

		const arrayReviewSchema = new LocalBusiness({
			name: "Cafe",
			address,
			review: [new Review("Jane", new Rating(5))],
		});
		const arrayReviewObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arrayReviewSchema),
		) as Record<string, unknown>;
		const reviews = arrayReviewObj.review as Record<string, unknown>[];
		expect(reviews[0]?.["@type"]).toBe("Review");
	});
});
