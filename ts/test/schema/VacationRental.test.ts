import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Accommodation } from "../../src/schema/Accommodation";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { Brand } from "../../src/schema/Brand";
import { PostalAddress } from "../../src/schema/PostalAddress";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";
import { VacationRental } from "../../src/schema/VacationRental";

describe("VacationRental", () => {
	const containsPlace = new Accommodation({
		occupancy: new QuantitativeValue(4),
	});

	it("produces minimal JSON-LD output with all 6 required fields", () => {
		const schema = new VacationRental({
			name: "Beach House",
			identifier: "vr-123",
			image: ["https://example.com/house.jpg"],
			latitude: 40.7128,
			longitude: -74.006,
			containsPlace,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const containsPlaceObj = obj.containsPlace as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("VacationRental");
		expect(obj.name).toBe("Beach House");
		expect(obj.identifier).toBe("vr-123");
		expect(obj.image).toEqual(["https://example.com/house.jpg"]);
		expect(obj.latitude).toBe(40.7128);
		expect(obj.longitude).toBe(-74.006);
		expect(containsPlaceObj["@type"]).toBe("Accommodation");
	});

	it("omits optional fields when null", () => {
		const schema = new VacationRental({
			name: "Beach House",
			identifier: "vr-123",
			image: ["https://example.com/house.jpg"],
			latitude: 40.7128,
			longitude: -74.006,
			containsPlace,
			additionalType: null,
			address: null,
			aggregateRating: null,
			brand: null,
			checkinTime: null,
			checkoutTime: null,
			datePublished: null,
			description: null,
			knowsLanguage: null,
			review: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("address");
		expect(obj).not.toHaveProperty("brand");
		expect(obj).not.toHaveProperty("review");
	});

	it("includes optional fields and review as array", () => {
		const schema = new VacationRental({
			name: "Beach House",
			identifier: "vr-123",
			image: ["https://example.com/house.jpg"],
			latitude: 40.7128,
			longitude: -74.006,
			containsPlace,
			address: new PostalAddress({ streetAddress: "1 Ocean Ave" }),
			aggregateRating: new AggregateRating(4.8),
			brand: new Brand("Coastal Stays"),
			knowsLanguage: ["en", "es"],
			review: [new Review("Jane", new Rating(5))],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const address = obj.address as Record<string, unknown>;
		const review = obj.review as Record<string, unknown>[];

		expect(address["@type"]).toBe("PostalAddress");
		expect((obj.aggregateRating as Record<string, unknown>)["@type"]).toBe(
			"AggregateRating",
		);
		expect((obj.brand as Record<string, unknown>)["@type"]).toBe("Brand");
		expect(review[0]?.["@type"]).toBe("Review");
		expect(obj.knowsLanguage).toEqual(["en", "es"]);
	});
});
