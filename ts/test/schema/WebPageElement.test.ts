import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { WebPageElement } from "../../src/schema/WebPageElement";

describe("WebPageElement", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new WebPageElement(true, ".article-body");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("WebPageElement");
		expect(obj.isAccessibleForFree).toBe(true);
		expect(obj.cssSelector).toBe(".article-body");
	});

	it("omits optional fields when null", () => {
		const schema = new WebPageElement(false, ".paywalled");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("description");
	});

	it("includes all fields when set", () => {
		const schema = new WebPageElement(false, ".paywalled");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.isAccessibleForFree).toBe(false);
		expect(obj.cssSelector).toBe(".paywalled");
	});
});
