import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { MonetaryAmount } from "../../src/schema/MonetaryAmount";

describe("MonetaryAmount", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new MonetaryAmount("USD");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("MonetaryAmount");
		expect(obj.currency).toBe("USD");
	});

	it("omits optional fields when null", () => {
		const schema = new MonetaryAmount("USD", null, null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("value");
		expect(obj).not.toHaveProperty("minValue");
		expect(obj).not.toHaveProperty("maxValue");
		expect(obj).not.toHaveProperty("unitText");
	});

	it("includes all fields when set", () => {
		const schema = new MonetaryAmount("USD", 10.5, 5, 20, "per item");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.currency).toBe("USD");
		expect(obj.value).toBe(10.5);
		expect(obj.minValue).toBe(5);
		expect(obj.maxValue).toBe(20);
		expect(obj.unitText).toBe("per item");
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new MonetaryAmount("USD", 10.5, 5, 20, "per item");
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "MonetaryAmount",\n  "currency": "USD",\n  "value": 10.5,\n  "minValue": 5,\n  "maxValue": 20,\n  "unitText": "per item"\n}',
		);
	});
});
