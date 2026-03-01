import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AggregateOffer } from "../../src/schema/AggregateOffer";

describe("AggregateOffer", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new AggregateOffer(9.99, "USD");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("AggregateOffer");
		expect(obj.lowPrice).toBe(9.99);
		expect(obj.priceCurrency).toBe("USD");
	});

	it("omits optional fields when null", () => {
		const schema = new AggregateOffer(9.99, "USD", null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("highPrice");
		expect(obj).not.toHaveProperty("offerCount");
	});

	it("includes all fields when set", () => {
		const schema = new AggregateOffer(9.99, "USD", 19.99, 42);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.highPrice).toBe(19.99);
		expect(obj.offerCount).toBe(42);
	});
});
