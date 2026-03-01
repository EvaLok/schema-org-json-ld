import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { EmployerAggregateRating } from "../../src/schema/EmployerAggregateRating";
import { Organization } from "../../src/schema/Organization";

describe("EmployerAggregateRating", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new EmployerAggregateRating(
			new Organization({ name: "Example Corp" }),
			4.7,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemReviewed = obj.itemReviewed as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("EmployerAggregateRating");
		expect(itemReviewed["@type"]).toBe("Organization");
		expect(obj.ratingValue).toBe(4.7);
	});

	it("omits optional fields when null", () => {
		const schema = new EmployerAggregateRating(
			new Organization({ name: "Example Corp" }),
			4.7,
			null,
			null,
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("ratingCount");
		expect(obj).not.toHaveProperty("reviewCount");
		expect(obj).not.toHaveProperty("bestRating");
		expect(obj).not.toHaveProperty("worstRating");
	});

	it("includes all fields when set", () => {
		const schema = new EmployerAggregateRating(
			new Organization({ name: "Example Corp" }),
			4.7,
			120,
			80,
			5,
			1,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.ratingCount).toBe(120);
		expect(obj.reviewCount).toBe(80);
		expect(obj.bestRating).toBe(5);
		expect(obj.worstRating).toBe(1);
	});
});
