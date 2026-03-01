import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AggregateRating } from "../../src/schema/AggregateRating";
import { BroadcastEvent } from "../../src/schema/BroadcastEvent";
import { Clip } from "../../src/schema/Clip";
import { HowToStep } from "../../src/schema/HowToStep";
import { InteractionCounter } from "../../src/schema/InteractionCounter";
import { NutritionInformation } from "../../src/schema/NutritionInformation";
import { Organization } from "../../src/schema/Organization";
import { Rating } from "../../src/schema/Rating";
import { Recipe } from "../../src/schema/Recipe";
import { Review } from "../../src/schema/Review";
import { VideoObject } from "../../src/schema/VideoObject";

describe("Recipe", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Recipe({
			name: "Pancakes",
			image: ["https://example.com/pancakes.jpg"],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Recipe");
		expect(obj.name).toBe("Pancakes");
		expect(obj.image).toEqual(["https://example.com/pancakes.jpg"]);
	});

	it("omits optional fields when null", () => {
		const schema = new Recipe({
			name: "Pancakes",
			image: ["https://example.com/pancakes.jpg"],
			author: null,
			datePublished: null,
			description: null,
			prepTime: null,
			cookTime: null,
			totalTime: null,
			keywords: null,
			recipeYield: null,
			recipeCategory: null,
			recipeCuisine: null,
			recipeIngredient: null,
			recipeInstructions: null,
			nutrition: null,
			aggregateRating: null,
			review: null,
			video: null,
			expires: null,
			hasPart: null,
			publication: null,
			ineligibleRegion: null,
			interactionStatistic: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("recipeInstructions");
		expect(obj).not.toHaveProperty("interactionStatistic");
	});

	it("supports recipeInstructions with HowToStep array and single review/interactionStatistic", () => {
		const schema = new Recipe({
			name: "Pancakes",
			image: ["https://example.com/pancakes.jpg"],
			author: new Organization({ name: "Example Kitchen" }),
			recipeInstructions: [
				new HowToStep("Mix ingredients"),
				new HowToStep("Cook on skillet"),
			],
			nutrition: new NutritionInformation(),
			aggregateRating: new AggregateRating(4.7),
			review: new Review("Jane", new Rating(5)),
			video: new VideoObject({
				name: "Pancake Tutorial",
				thumbnailUrl: ["https://example.com/thumb.jpg"],
				uploadDate: "2026-01-01",
			}),
			hasPart: [new Clip("Intro", 0, "https://example.com/video#t=0", 30)],
			publication: new BroadcastEvent(true),
			interactionStatistic: new InteractionCounter("LikeAction", 42),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const recipeInstructions = obj.recipeInstructions as Record<
			string,
			unknown
		>[];
		const review = obj.review as Record<string, unknown>;
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>;

		expect(recipeInstructions).toHaveLength(2);
		expect(recipeInstructions[0]?.["@type"]).toBe("HowToStep");
		expect(review["@type"]).toBe("Review");
		expect(interactionStatistic["@type"]).toBe("InteractionCounter");
	});

	it("supports review and interactionStatistic as arrays", () => {
		const schema = new Recipe({
			name: "Pancakes",
			image: ["https://example.com/pancakes.jpg"],
			review: [new Review("Jane", new Rating(5))],
			interactionStatistic: [new InteractionCounter("LikeAction", 42)],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const review = obj.review as Record<string, unknown>[];
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>[];

		expect(review[0]?.["@type"]).toBe("Review");
		expect(interactionStatistic[0]?.["@type"]).toBe("InteractionCounter");
	});
});
