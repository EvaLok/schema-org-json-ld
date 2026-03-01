import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Brand } from "../../src/schema/Brand";
import { ListItem } from "../../src/schema/ListItem";

describe("ListItem", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ListItem(1);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ListItem");
		expect(obj.position).toBe(1);
	});

	it("omits optional fields when null", () => {
		const schema = new ListItem(1, null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("name");
		expect(obj).not.toHaveProperty("item");
		expect(obj).not.toHaveProperty("url");
	});

	it("supports string URL for item", () => {
		const schema = new ListItem(
			1,
			"Getting started",
			"https://example.com/getting-started",
			"https://example.com/list-item",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe("Getting started");
		expect(obj.item).toBe("https://example.com/getting-started");
		expect(obj.url).toBe("https://example.com/list-item");
	});

	it("supports TypedSchema for item", () => {
		const schema = new ListItem(2, "Brand item", new Brand("Acme"), null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const item = obj.item as Record<string, unknown>;

		expect(item["@type"]).toBe("Brand");
		expect(item.name).toBe("Acme");
	});
});
