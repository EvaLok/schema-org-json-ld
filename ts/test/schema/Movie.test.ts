import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { Movie } from "../../src/schema/Movie";
import { Person } from "../../src/schema/Person";
import { Rating } from "../../src/schema/Rating";
import { Review } from "../../src/schema/Review";

describe("Movie", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Movie({
			name: "Interstellar",
			image: "https://example.com/interstellar.jpg",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Movie");
		expect(obj.name).toBe("Interstellar");
		expect(obj.image).toBe("https://example.com/interstellar.jpg");
	});

	it("omits optional fields when null", () => {
		const schema = new Movie({
			name: "Interstellar",
			image: "https://example.com/interstellar.jpg",
			aggregateRating: null,
			dateCreated: null,
			datePublished: null,
			director: null,
			review: null,
			description: null,
			actor: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("aggregateRating");
		expect(obj).not.toHaveProperty("director");
		expect(obj).not.toHaveProperty("actor");
	});

	it("supports actor as Person array", () => {
		const schema = new Movie({
			name: "Interstellar",
			image: "https://example.com/interstellar.jpg",
			aggregateRating: new AggregateRating(4.8),
			director: new Person({ name: "Christopher Nolan" }),
			review: new Review("Jane", new Rating(5)),
			actor: [
				new Person({ name: "Matthew McConaughey" }),
				new Person({ name: "Anne Hathaway" }),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const actor = obj.actor as Record<string, unknown>[];
		const aggregateRating = obj.aggregateRating as Record<string, unknown>;

		expect(aggregateRating["@type"]).toBe("AggregateRating");
		expect(actor).toHaveLength(2);
		expect(actor[0]?.["@type"]).toBe("Person");
	});
});
