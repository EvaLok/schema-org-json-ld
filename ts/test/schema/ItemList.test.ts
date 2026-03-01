import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ItemList } from "../../src/schema/ItemList";
import { ListItem } from "../../src/schema/ListItem";

describe("ItemList", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ItemList([new ListItem(1)]);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ItemList");
		expect(itemListElement).toHaveLength(1);
		expect(itemListElement[0]?.position).toBe(1);
	});

	it("omits optional fields when null", () => {
		const schema = new ItemList([new ListItem(1)], null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("itemListOrder");
		expect(obj).not.toHaveProperty("numberOfItems");
	});

	it("includes all fields when set", () => {
		const schema = new ItemList(
			[new ListItem(1), new ListItem(2)],
			"https://schema.org/ItemListOrderAscending",
			2,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.itemListOrder).toBe("https://schema.org/ItemListOrderAscending");
		expect(obj.numberOfItems).toBe(2);
	});
});
