import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Clip } from "../../src/schema/Clip";

describe("Clip", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Clip("Intro", 0, "https://example.com/video#t=0");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Clip");
		expect(obj.name).toBe("Intro");
		expect(obj.startOffset).toBe(0);
		expect(obj.url).toBe("https://example.com/video#t=0");
	});

	it("omits optional fields when null", () => {
		const schema = new Clip("Intro", 0, "https://example.com/video#t=0", null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("endOffset");
	});

	it("includes all fields when set", () => {
		const schema = new Clip("Intro", 0, "https://example.com/video#t=0", 90);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.endOffset).toBe(90);
	});
});
