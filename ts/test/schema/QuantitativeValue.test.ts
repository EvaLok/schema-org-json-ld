import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";

describe("QuantitativeValue", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new QuantitativeValue({});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("QuantitativeValue");
	});

	it("omits optional fields when null", () => {
		const schema = new QuantitativeValue({
			value: null,
			unitCode: null,
			unitText: null,
			minValue: null,
			maxValue: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("value");
		expect(obj).not.toHaveProperty("unitCode");
		expect(obj).not.toHaveProperty("unitText");
		expect(obj).not.toHaveProperty("minValue");
		expect(obj).not.toHaveProperty("maxValue");
	});

	it("includes all fields when set", () => {
		const schema = new QuantitativeValue({
			value: 10.5,
			unitCode: "HUR",
			unitText: "HOUR",
			minValue: 5,
			maxValue: 20,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.value).toBe(10.5);
		expect(obj.unitCode).toBe("HUR");
		expect(obj.unitText).toBe("HOUR");
		expect(obj.minValue).toBe(5);
		expect(obj.maxValue).toBe(20);
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new QuantitativeValue({
			value: 10.5,
			unitCode: "HUR",
			unitText: "HOUR",
			minValue: 5,
			maxValue: 20,
		});
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "QuantitativeValue",\n  "value": 10.5,\n  "unitCode": "HUR",\n  "unitText": "HOUR",\n  "minValue": 5,\n  "maxValue": 20\n}',
		);
	});
});
