import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DataDownload } from "../../src/schema/DataDownload";

describe("DataDownload", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new DataDownload("https://example.com/dataset.csv");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("DataDownload");
		expect(obj.contentUrl).toBe("https://example.com/dataset.csv");
	});

	it("omits optional fields when null", () => {
		const schema = new DataDownload("https://example.com/dataset.csv", null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("encodingFormat");
	});

	it("includes all fields when set", () => {
		const schema = new DataDownload(
			"https://example.com/dataset.csv",
			"text/csv",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.encodingFormat).toBe("text/csv");
	});
});
