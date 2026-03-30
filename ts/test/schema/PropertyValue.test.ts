import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { PropertyValue } from "../../src/schema/PropertyValue";

describe("PropertyValue", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new PropertyValue({ name: "weight", value: "10kg" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("PropertyValue");
		expect(obj.name).toBe("weight");
		expect(obj.value).toBe("10kg");
	});

	it("has no optional fields to include or omit", () => {
		const schema = new PropertyValue({ name: "weight", value: "10kg" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual(["@context", "@type", "name", "value"]);
	});

	it("includes all fields when set", () => {
		const schema = new PropertyValue({ name: "weight", value: "10kg" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("weight");
		expect(obj.value).toBe("10kg");
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new PropertyValue({ name: "weight", value: "10kg" });
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "PropertyValue",\n  "name": "weight",\n  "value": "10kg"\n}',
		);
	});

	it("serializes empty string fields", () => {
		const schema = new PropertyValue({ name: "", value: "" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("");
		expect(obj.value).toBe("");
	});

	it("preserves numeric-like values as strings", () => {
		const schema = new PropertyValue({
			name: "identifier",
			value: "000123",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.value).toBe("000123");
		expect(typeof obj.value).toBe("string");
	});
});
