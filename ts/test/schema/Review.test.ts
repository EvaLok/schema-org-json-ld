import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";
import { Thing } from "../../src/schema/Thing";

describe("Review", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Review("Jane Doe", new Rating(5));
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const reviewRating = obj.reviewRating as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Review");
		expect(obj.author).toBe("Jane Doe");
		expect(reviewRating["@type"]).toBe("Rating");
	});

	it("omits optional fields when null", () => {
		const schema = new Review("Jane Doe", new Rating(5), null, null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("reviewBody");
		expect(obj).not.toHaveProperty("datePublished");
		expect(obj).not.toHaveProperty("name");
		expect(obj).not.toHaveProperty("itemReviewed");
	});

	it("supports author as Person and Organization", () => {
		const personAuthorSchema = new Review(
			new Person({ name: "Jane Doe" }),
			new Rating(4),
		);
		const personJson = JsonLdGenerator.schemaToJson(personAuthorSchema);
		const personObj = JSON.parse(personJson) as Record<string, unknown>;
		const personAuthor = personObj.author as Record<string, unknown>;

		expect(personAuthor["@type"]).toBe("Person");

		const organizationAuthorSchema = new Review(
			new Organization({ name: "Example Corp" }),
			new Rating(4),
		);
		const organizationJson = JsonLdGenerator.schemaToJson(organizationAuthorSchema);
		const organizationObj = JSON.parse(organizationJson) as Record<string, unknown>;
		const organizationAuthor = organizationObj.author as Record<string, unknown>;

		expect(organizationAuthor["@type"]).toBe("Organization");
	});

	it("includes all fields and itemReviewed TypedSchema when set", () => {
		const schema = new Review(
			"Jane Doe",
			new Rating(5),
			"Excellent product.",
			"2026-03-01",
			"Great review",
			new Thing("Example Thing"),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemReviewed = obj.itemReviewed as Record<string, unknown>;

		expect(obj.reviewBody).toBe("Excellent product.");
		expect(obj.datePublished).toBe("2026-03-01");
		expect(obj.name).toBe("Great review");
		expect(itemReviewed["@type"]).toBe("Thing");
	});
});
