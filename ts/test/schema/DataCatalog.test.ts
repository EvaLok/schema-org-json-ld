import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DataCatalog } from "../../src/schema/DataCatalog";

describe("DataCatalog", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new DataCatalog("Public Datasets");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("DataCatalog");
		expect(obj.name).toBe("Public Datasets");
	});

	it("omits optional fields when null", () => {
		const schema = new DataCatalog("Public Datasets");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("description");
	});

	it("includes all fields when set", () => {
		const schema = new DataCatalog("Open Data");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("Open Data");
	});
});
