import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Clip } from "../../src/schema/Clip";
import { HowToStep } from "../../src/schema/HowToStep";

describe("HowToStep", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new HowToStep("Mix ingredients");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("HowToStep");
		expect(obj.text).toBe("Mix ingredients");
	});

	it("omits optional fields when null", () => {
		const schema = new HowToStep(
			"Mix ingredients",
			null,
			null,
			null,
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("name");
		expect(obj).not.toHaveProperty("url");
		expect(obj).not.toHaveProperty("image");
		expect(obj).not.toHaveProperty("video");
		expect(obj).not.toHaveProperty("itemListElement");
	});

	it("includes all fields when set", () => {
		const schema = new HowToStep(
			"Mix ingredients",
			"Step 1",
			"https://example.com/steps/1",
			"https://example.com/steps/1.jpg",
			new Clip("Step 1 clip", 0, "https://example.com/video#t=0", 30),
			["Measure flour", "Add eggs"],
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const video = obj.video as Record<string, unknown>;

		expect(obj.name).toBe("Step 1");
		expect(obj.url).toBe("https://example.com/steps/1");
		expect(obj.image).toBe("https://example.com/steps/1.jpg");
		expect(video["@type"]).toBe("Clip");
		expect(video.name).toBe("Step 1 clip");
		expect(obj.itemListElement).toEqual(["Measure flour", "Add eggs"]);
	});
});
