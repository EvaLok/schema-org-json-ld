import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { InteractionCounter } from "../../src/schema/InteractionCounter";

describe("InteractionCounter", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new InteractionCounter({
			interactionType: "https://schema.org/LikeAction",
			userInteractionCount: 123,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("InteractionCounter");
		expect(obj.interactionType).toBe("https://schema.org/LikeAction");
		expect(obj.userInteractionCount).toBe(123);
	});

	it("omits optional fields when null", () => {
		const schema = new InteractionCounter({
			interactionType: "https://schema.org/LikeAction",
			userInteractionCount: 123,
			interactionService: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("interactionService");
	});

	it("includes all fields when set", () => {
		const schema = new InteractionCounter({
			interactionType: "https://schema.org/LikeAction",
			userInteractionCount: 123,
			interactionService: "YouTube",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.interactionService).toBe("YouTube");
	});
});
