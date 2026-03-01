import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { PostalAddress } from "../../src/schema/PostalAddress";
import { Restaurant } from "../../src/schema/Restaurant";

describe("Restaurant", () => {
	const address = new PostalAddress({
		streetAddress: "123 Main Street",
	});

	it("produces Restaurant JSON-LD with required fields", () => {
		const schema = new Restaurant({
			name: "Example Bistro",
			address,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@type"]).toBe("Restaurant");
		expect(obj.name).toBe("Example Bistro");
		expect(obj.address).toBeDefined();
	});

	it("produces full Restaurant output", () => {
		const schema = new Restaurant({
			name: "Example Bistro",
			address,
			servesCuisine: "Italian",
			menu: "https://example.com/menu",
			acceptsReservations: true,
			priceRange: "$$",
			aggregateRating: new AggregateRating(4.7, null, null, 145),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const aggregateRating = obj.aggregateRating as Record<string, unknown>;

		expect(obj["@type"]).toBe("Restaurant");
		expect(obj.servesCuisine).toBe("Italian");
		expect(obj.menu).toBe("https://example.com/menu");
		expect(obj.acceptsReservations).toBe(true);
		expect(obj.priceRange).toBe("$$");
		expect(aggregateRating["@type"]).toBe("AggregateRating");
		expect(aggregateRating.ratingValue).toBe(4.7);
		expect(aggregateRating.ratingCount).toBe(145);
	});

	it("inherits LocalBusiness fields while preserving Restaurant type", () => {
		const schema = new Restaurant({
			name: "Example Bistro",
			address,
			telephone: "+1-555-1234",
			description: "Neighborhood restaurant",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@type"]).toBe("Restaurant");
		expect(obj.telephone).toBe("+1-555-1234");
		expect(obj.description).toBe("Neighborhood restaurant");
	});
});
