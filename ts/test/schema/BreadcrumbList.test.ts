import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { BreadcrumbList } from "../../src/schema/BreadcrumbList";
import { ListItem } from "../../src/schema/ListItem";

describe("BreadcrumbList", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new BreadcrumbList({
			itemListElement: [new ListItem({ position: 1 })],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("BreadcrumbList");
		expect(itemListElement).toHaveLength(1);
		expect(itemListElement[0]?.["@type"]).toBe("ListItem");
	});

	it("preserves nested ListItem values", () => {
		const schema = new BreadcrumbList({
			itemListElement: [
				new ListItem({
					position: 1,
					name: "Home",
					item: "https://example.com",
				}),
				new ListItem({
					position: 2,
					name: "Products",
					item: "https://example.com/products",
				}),
			],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement).toHaveLength(2);
		expect(itemListElement[0]?.position).toBe(1);
		expect(itemListElement[1]?.name).toBe("Products");
	});

	it("includes all fields when set in nested items", () => {
		const schema = new BreadcrumbList({
			itemListElement: [
				new ListItem({
					position: 1,
					name: "Home",
					item: "https://example.com",
					url: "https://example.com/list-item/home",
				}),
			],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement[0]?.url).toBe("https://example.com/list-item/home");
	});

	it("supports single breadcrumb item without item URL for current page", () => {
		const schema = new BreadcrumbList({
			itemListElement: [new ListItem({ position: 1, name: "Checkout" })],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const item = (obj.itemListElement as Record<string, unknown>[])[0] as Record<
			string,
			unknown
		>;

		expect(item.name).toBe("Checkout");
		expect(item).not.toHaveProperty("item");
	});

	it("serializes long breadcrumb chains with ordered positions", () => {
		const schema = new BreadcrumbList({
			itemListElement: [
				new ListItem({ position: 1, name: "Home", item: "https://example.com" }),
				new ListItem({
					position: 2,
					name: "Library",
					item: "https://example.com/library",
				}),
				new ListItem({
					position: 3,
					name: "Books",
					item: "https://example.com/library/books",
				}),
				new ListItem({
					position: 4,
					name: "Fiction",
					item: "https://example.com/library/books/fiction",
				}),
				new ListItem({ position: 5, name: "Classics" }),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement).toHaveLength(5);
		expect(itemListElement[0]?.position).toBe(1);
		expect(itemListElement[4]?.position).toBe(5);
		expect(itemListElement[4]).not.toHaveProperty("item");
	});

	it("includes optional ListItem url while omitting item on final breadcrumb", () => {
		const schema = new BreadcrumbList({
			itemListElement: [
				new ListItem({
					position: 1,
					name: "Products",
					item: "https://example.com/products",
					url: "https://example.com/list-item/products",
				}),
				new ListItem({ position: 2, name: "Current Product" }),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const itemListElement = obj.itemListElement as Record<string, unknown>[];

		expect(itemListElement[0]?.url).toBe("https://example.com/list-item/products");
		expect(itemListElement[1]).not.toHaveProperty("item");
	});
});
