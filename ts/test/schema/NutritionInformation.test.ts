import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { NutritionInformation } from "../../src/schema/NutritionInformation";

describe("NutritionInformation", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new NutritionInformation({});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("NutritionInformation");
	});

	it("omits optional fields when null", () => {
		const schema = new NutritionInformation({
			calories: null,
			fatContent: null,
			saturatedFatContent: null,
			cholesterolContent: null,
			sodiumContent: null,
			carbohydrateContent: null,
			fiberContent: null,
			sugarContent: null,
			proteinContent: null,
			servingSize: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("calories");
		expect(obj).not.toHaveProperty("fatContent");
		expect(obj).not.toHaveProperty("saturatedFatContent");
		expect(obj).not.toHaveProperty("cholesterolContent");
		expect(obj).not.toHaveProperty("sodiumContent");
		expect(obj).not.toHaveProperty("carbohydrateContent");
		expect(obj).not.toHaveProperty("fiberContent");
		expect(obj).not.toHaveProperty("sugarContent");
		expect(obj).not.toHaveProperty("proteinContent");
		expect(obj).not.toHaveProperty("servingSize");
	});

	it("supports partial options object", () => {
		const schema = new NutritionInformation({
			calories: "250 calories",
			proteinContent: "12 g",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.calories).toBe("250 calories");
		expect(obj.proteinContent).toBe("12 g");
		expect(obj).not.toHaveProperty("fatContent");
	});

	it("supports empty options object", () => {
		const schema = new NutritionInformation();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@type"]).toBe("NutritionInformation");
		expect(obj).not.toHaveProperty("calories");
	});

	it("includes all fields when set", () => {
		const schema = new NutritionInformation({
			calories: "250 calories",
			fatContent: "8 g",
			saturatedFatContent: "2 g",
			cholesterolContent: "30 mg",
			sodiumContent: "500 mg",
			carbohydrateContent: "30 g",
			fiberContent: "5 g",
			sugarContent: "10 g",
			proteinContent: "12 g",
			servingSize: "1 cup",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.calories).toBe("250 calories");
		expect(obj.fatContent).toBe("8 g");
		expect(obj.saturatedFatContent).toBe("2 g");
		expect(obj.cholesterolContent).toBe("30 mg");
		expect(obj.sodiumContent).toBe("500 mg");
		expect(obj.carbohydrateContent).toBe("30 g");
		expect(obj.fiberContent).toBe("5 g");
		expect(obj.sugarContent).toBe("10 g");
		expect(obj.proteinContent).toBe("12 g");
		expect(obj.servingSize).toBe("1 cup");
	});
});
