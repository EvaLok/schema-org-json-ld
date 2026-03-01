import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { HowToSection } from "../../src/schema/HowToSection";
import { HowToStep } from "../../src/schema/HowToStep";

describe("HowToSection", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new HowToSection("Preparation", [
			new HowToStep("Mix ingredients"),
		]);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("HowToSection");
		expect(obj.name).toBe("Preparation");
		expect(itemListElement).toHaveLength(1);
		expect(itemListElement[0]?.["@type"]).toBe("HowToStep");
	});

	it("omits null fields inside nested HowToStep entries", () => {
		const schema = new HowToSection("Preparation", [
			new HowToStep("Mix ingredients", null, null, null, null, null),
		]);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const firstStep = (
			obj.itemListElement as Record<string, unknown>[]
		)[0] as Record<string, unknown>;

		expect(firstStep).not.toHaveProperty("name");
		expect(firstStep).not.toHaveProperty("url");
		expect(firstStep).not.toHaveProperty("image");
		expect(firstStep).not.toHaveProperty("video");
		expect(firstStep).not.toHaveProperty("itemListElement");
	});

	it("includes all fields when set", () => {
		const schema = new HowToSection("Preparation", [
			new HowToStep("Mix ingredients", "Step 1"),
			new HowToStep("Bake for 20 minutes", "Step 2"),
		]);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement).toHaveLength(2);
		expect(itemListElement[0]?.name).toBe("Step 1");
		expect(itemListElement[1]?.name).toBe("Step 2");
	});
});
