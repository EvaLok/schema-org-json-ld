import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { HowToSection } from "../../src/schema/HowToSection";
import { HowToStep } from "../../src/schema/HowToStep";

describe("HowToSection", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new HowToSection({
			name: "Preparation",
			itemListElement: [new HowToStep({ text: "Mix ingredients" })],
		});
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
		const schema = new HowToSection({
			name: "Preparation",
			itemListElement: [
				new HowToStep({
					text: "Mix ingredients",
					name: null,
					url: null,
					image: null,
					video: null,
					itemListElement: null,
				}),
			],
		});
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
		const schema = new HowToSection({
			name: "Preparation",
			itemListElement: [
				new HowToStep({ text: "Mix ingredients", name: "Step 1" }),
				new HowToStep({ text: "Bake for 20 minutes", name: "Step 2" }),
			],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement).toHaveLength(2);
		expect(itemListElement[0]?.name).toBe("Step 1");
		expect(itemListElement[1]?.name).toBe("Step 2");
	});

	it("serializes an empty section name", () => {
		const schema = new HowToSection({
			name: "",
			itemListElement: [new HowToStep({ text: "Whisk the eggs" })],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("");
	});

	it("preserves a single-step itemListElement array", () => {
		const schema = new HowToSection({
			name: "Mix",
			itemListElement: [new HowToStep({ text: "Combine flour and water" })],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement).toHaveLength(1);
		expect(itemListElement[0]?.text).toBe("Combine flour and water");
	});

	it("serializes nested HowToStep entries with the correct @type", () => {
		const schema = new HowToSection({
			name: "Finish",
			itemListElement: [
				new HowToStep({ text: "Plate the dish" }),
				new HowToStep({ text: "Serve immediately" }),
			],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement[0]?.["@type"]).toBe("HowToStep");
		expect(itemListElement[1]?.["@type"]).toBe("HowToStep");
	});
});
