import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { Course } from "../../src/schema/Course";
import { CourseInstance } from "../../src/schema/CourseInstance";
import { Offer } from "../../src/schema/Offer";
import { Organization } from "../../src/schema/Organization";

describe("Course", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Course({
			name: "Intro to TS",
			description: "Learn TypeScript basics",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Course");
		expect(obj.name).toBe("Intro to TS");
		expect(obj.description).toBe("Learn TypeScript basics");
	});

	it("omits optional fields when null", () => {
		const schema = new Course({
			name: "Intro to TS",
			description: "Learn TypeScript basics",
			provider: null,
			offers: null,
			hasCourseInstance: null,
			courseCode: null,
			inLanguage: null,
			totalHistoricalEnrollment: null,
			aggregateRating: null,
			image: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("provider");
		expect(obj).not.toHaveProperty("offers");
		expect(obj).not.toHaveProperty("hasCourseInstance");
	});

	it("supports offers and hasCourseInstance as arrays", () => {
		const schema = new Course({
			name: "Intro to TS",
			description: "Learn TypeScript basics",
			provider: new Organization({ name: "Example Academy" }),
			offers: [
				new Offer({
					url: "https://example.com/course",
					priceCurrency: "USD",
					price: 99,
					availability: ItemAvailability.InStock,
				}),
			],
			hasCourseInstance: [new CourseInstance()],
			aggregateRating: new AggregateRating(4.6),
			inLanguage: "en",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const offers = obj.offers as Record<string, unknown>[];
		const hasCourseInstance = obj.hasCourseInstance as Record<
			string,
			unknown
		>[];
		const provider = obj.provider as Record<string, unknown>;

		expect(provider["@type"]).toBe("Organization");
		expect(offers[0]?.["@type"]).toBe("Offer");
		expect(hasCourseInstance[0]?.["@type"]).toBe("CourseInstance");
	});
});
