import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AggregateRating } from "../../src/schema/AggregateRating";

describe("AggregateRating", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new AggregateRating({ ratingValue: 4.5 });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("AggregateRating");
		expect(obj.ratingValue).toBe(4.5);
	});

	it("omits optional fields when null", () => {
		const schema = new AggregateRating({ ratingValue: 4.5 });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("bestRating");
		expect(obj).not.toHaveProperty("worstRating");
		expect(obj).not.toHaveProperty("ratingCount");
		expect(obj).not.toHaveProperty("reviewCount");
	});

	it("includes all fields when set", () => {
		const schema = new AggregateRating({
			ratingValue: 4.5,
			bestRating: 5,
			worstRating: 1,
			ratingCount: 120,
			reviewCount: 50,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.bestRating).toBe(5);
		expect(obj.worstRating).toBe(1);
		expect(obj.ratingCount).toBe(120);
		expect(obj.reviewCount).toBe(50);
	});
});
