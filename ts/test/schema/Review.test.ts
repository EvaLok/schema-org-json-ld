import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemList } from "../../src/schema/ItemList";
import { ListItem } from "../../src/schema/ListItem";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";
import { Thing } from "../../src/schema/Thing";

describe("Review", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Review({
			author: "Jane Doe",
			reviewRating: new Rating({ ratingValue: 5 }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const reviewRating = obj.reviewRating as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Review");
		expect(obj.author).toBe("Jane Doe");
		expect(reviewRating["@type"]).toBe("Rating");
	});

	it("omits optional fields when null", () => {
		const schema = new Review({
			author: "Jane Doe",
			reviewRating: new Rating({ ratingValue: 5 }),
			contentReferenceTime: null,
			reviewBody: null,
			datePublished: null,
			name: null,
			itemReviewed: null,
			positiveNotes: null,
			negativeNotes: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("reviewBody");
		expect(obj).not.toHaveProperty("datePublished");
		expect(obj).not.toHaveProperty("name");
		expect(obj).not.toHaveProperty("itemReviewed");
		expect(obj).not.toHaveProperty("positiveNotes");
		expect(obj).not.toHaveProperty("negativeNotes");
		expect(obj).not.toHaveProperty("contentReferenceTime");
	});

	it("supports author as Person and Organization", () => {
		const personAuthorSchema = new Review({
			author: new Person({ name: "Jane Doe" }),
			reviewRating: new Rating({ ratingValue: 4 }),
		});
		const personJson = JsonLdGenerator.schemaToJson(personAuthorSchema);
		const personObj = JSON.parse(personJson) as Record<string, unknown>;
		const personAuthor = personObj.author as Record<string, unknown>;

		expect(personAuthor["@type"]).toBe("Person");

		const organizationAuthorSchema = new Review({
			author: new Organization({ name: "Example Corp" }),
			reviewRating: new Rating({ ratingValue: 4 }),
		});
		const organizationJson = JsonLdGenerator.schemaToJson(
			organizationAuthorSchema,
		);
		const organizationObj = JSON.parse(organizationJson) as Record<
			string,
			unknown
		>;
		const organizationAuthor = organizationObj.author as Record<
			string,
			unknown
		>;

		expect(organizationAuthor["@type"]).toBe("Organization");
	});

	it("includes all fields and itemReviewed TypedSchema when set", () => {
		const schema = new Review({
			author: "Jane Doe",
			reviewRating: new Rating({ ratingValue: 5 }),
			reviewBody: "Excellent product.",
			datePublished: "2026-03-01",
			name: "Great review",
			itemReviewed: new Thing({ name: "Example Thing" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemReviewed = obj.itemReviewed as Record<string, unknown>;

		expect(obj.reviewBody).toBe("Excellent product.");
		expect(obj.datePublished).toBe("2026-03-01");
		expect(obj.name).toBe("Great review");
		expect(itemReviewed["@type"]).toBe("Thing");
	});

	it("includes positiveNotes as ItemList with ListItems", () => {
		const schema = new Review({
			author: "Jane Doe",
			reviewRating: new Rating({ ratingValue: 5 }),
			positiveNotes: new ItemList({
				itemListElement: [
					new ListItem({ position: 1, name: "Consistent results" }),
					new ListItem({ position: 2, name: "Still sharp after many uses" }),
				],
			}),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const positiveNotes = obj.positiveNotes as Record<string, unknown>;
		const items = positiveNotes.itemListElement as Record<string, unknown>[];

		expect(positiveNotes["@type"]).toBe("ItemList");
		expect(items).toHaveLength(2);
		expect(items[0]["@type"]).toBe("ListItem");
		expect(items[0].position).toBe(1);
		expect(items[0].name).toBe("Consistent results");
		expect(items[1].position).toBe(2);
		expect(items[1].name).toBe("Still sharp after many uses");
	});

	it("includes negativeNotes as ItemList with ListItems", () => {
		const schema = new Review({
			author: "Jane Doe",
			reviewRating: new Rating({ ratingValue: 5 }),
			negativeNotes: new ItemList({
				itemListElement: [
					new ListItem({ position: 1, name: "No child protection" }),
				],
			}),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const negativeNotes = obj.negativeNotes as Record<string, unknown>;
		const items = negativeNotes.itemListElement as Record<string, unknown>[];

		expect(negativeNotes["@type"]).toBe("ItemList");
		expect(items).toHaveLength(1);
		expect(items[0]["@type"]).toBe("ListItem");
		expect(items[0].position).toBe(1);
		expect(items[0].name).toBe("No child protection");
	});

	it("includes contentReferenceTime when set", () => {
		const schema = new Review({
			author: "Jane Doe",
			reviewRating: new Rating({ ratingValue: 5 }),
			contentReferenceTime: "2024-07-15T14:00:00+02:00",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.contentReferenceTime).toBe("2024-07-15T14:00:00+02:00");
	});

	it("includes both positiveNotes and negativeNotes together", () => {
		const schema = new Review({
			author: "Jane Doe",
			reviewRating: new Rating({ ratingValue: 4 }),
			positiveNotes: new ItemList({
				itemListElement: [
					new ListItem({ position: 1, name: "Consistent results" }),
					new ListItem({ position: 2, name: "Still sharp after many uses" }),
				],
			}),
			negativeNotes: new ItemList({
				itemListElement: [
					new ListItem({ position: 1, name: "No child protection" }),
				],
			}),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const positiveNotes = obj.positiveNotes as Record<string, unknown>;
		const negativeNotes = obj.negativeNotes as Record<string, unknown>;

		expect(positiveNotes["@type"]).toBe("ItemList");
		expect(positiveNotes.itemListElement as unknown[]).toHaveLength(2);
		expect(negativeNotes["@type"]).toBe("ItemList");
		expect(negativeNotes.itemListElement as unknown[]).toHaveLength(1);
	});
});
