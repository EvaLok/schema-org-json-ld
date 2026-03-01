import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Rating } from "../../src/schema/Rating";

describe("Rating", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Rating(4.5);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Rating");
		expect(obj.ratingValue).toBe(4.5);
	});

	it("omits optional fields when null", () => {
		const schema = new Rating(4.5, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("bestRating");
		expect(obj).not.toHaveProperty("worstRating");
	});

	it("includes all fields when set", () => {
		const schema = new Rating(4.5, 5, 1);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.ratingValue).toBe(4.5);
		expect(obj.bestRating).toBe(5);
		expect(obj.worstRating).toBe(1);
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new Rating(4.5, 5, 1);
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "Rating",\n  "ratingValue": 4.5,\n  "bestRating": 5,\n  "worstRating": 1\n}',
		);
	});
});
