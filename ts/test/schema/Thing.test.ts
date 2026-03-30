import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Thing } from "../../src/schema/Thing";

describe("Thing", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Thing({ name: "Generic Thing" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Thing");
		expect(obj.name).toBe("Generic Thing");
	});

	it("has no optional fields to include in output", () => {
		const schema = new Thing({ name: "Only Required" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual(["@context", "@type", "name"]);
	});

	it("includes all fields when set", () => {
		const schema = new Thing({ name: "Complete Thing" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("Complete Thing");
	});

	it("serializes an empty string name", () => {
		const schema = new Thing({ name: "" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("");
	});

	it("only includes @context, @type, and name", () => {
		const schema = new Thing({ name: "Widget" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual(["@context", "@type", "name"]);
	});

	it("preserves the exact name value", () => {
		const schema = new Thing({ name: "Thing 42 / sample" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("Thing 42 / sample");
	});
});
