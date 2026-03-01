import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { BreadcrumbList } from "../../src/schema/BreadcrumbList";
import { ListItem } from "../../src/schema/ListItem";

describe("BreadcrumbList", () => {
it("produces minimal JSON-LD output with required fields only", () => {
const schema = new BreadcrumbList([new ListItem(1)]);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;
const itemListElement = obj.itemListElement as Record<string, unknown>[];

expect(obj["@context"]).toBe("https://schema.org/");
expect(obj["@type"]).toBe("BreadcrumbList");
expect(itemListElement).toHaveLength(1);
expect(itemListElement[0]?.["@type"]).toBe("ListItem");
});

it("preserves nested ListItem values", () => {
const schema = new BreadcrumbList([
new ListItem(1, "Home", "https://example.com"),
new ListItem(2, "Products", "https://example.com/products"),
]);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;
const itemListElement = obj.itemListElement as Record<string, unknown>[];

expect(itemListElement).toHaveLength(2);
expect(itemListElement[0]?.position).toBe(1);
expect(itemListElement[1]?.name).toBe("Products");
});

it("includes all fields when set in nested items", () => {
const schema = new BreadcrumbList([
new ListItem(
1,
"Home",
"https://example.com",
"https://example.com/list-item/home",
),
]);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;
const itemListElement = obj.itemListElement as Record<string, unknown>[];

expect(itemListElement[0]?.url).toBe("https://example.com/list-item/home");
});
});
