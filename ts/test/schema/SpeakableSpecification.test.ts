import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { SpeakableSpecification } from "../../src/schema/SpeakableSpecification";

describe("SpeakableSpecification", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new SpeakableSpecification();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("SpeakableSpecification");
	});

	it("omits optional fields when null", () => {
		const schema = new SpeakableSpecification(null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("cssSelector");
		expect(obj).not.toHaveProperty("xpath");
	});

	it("includes all fields when set", () => {
		const schema = new SpeakableSpecification(".headline", "/html/body/h1");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.cssSelector).toBe(".headline");
		expect(obj.xpath).toBe("/html/body/h1");
	});

	it("supports string and string[] union values", () => {
		const stringSchema = new SpeakableSpecification(
			".headline",
			"/html/body/h1",
		);
		const arraySchema = new SpeakableSpecification(
			[".headline", ".subheading"],
			["/html/body/h1", "/html/body/h2"],
		);

		const stringObj = JSON.parse(
			JsonLdGenerator.schemaToJson(stringSchema),
		) as Record<string, unknown>;
		const arrayObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arraySchema),
		) as Record<string, unknown>;

		expect(stringObj.cssSelector).toBe(".headline");
		expect(stringObj.xpath).toBe("/html/body/h1");
		expect(arrayObj.cssSelector).toEqual([".headline", ".subheading"]);
		expect(arrayObj.xpath).toEqual(["/html/body/h1", "/html/body/h2"]);
	});
});
