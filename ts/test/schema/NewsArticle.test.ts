import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { NewsArticle } from "../../src/schema/NewsArticle";
import { Organization } from "../../src/schema/Organization";

describe("NewsArticle", () => {
	it("produces NewsArticle JSON-LD with required article fields", () => {
		const schema = new NewsArticle({ headline: "News headline" });
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("NewsArticle");
		expect(obj.headline).toBe("News headline");
	});

	it("omits inherited optional fields when null", () => {
		const schema = new NewsArticle({
			headline: "News headline",
			author: null,
			datePublished: null,
			dateModified: null,
			image: null,
			description: null,
			publisher: null,
			speakable: null,
			isAccessibleForFree: null,
			hasPart: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("publisher");
		expect(obj).not.toHaveProperty("hasPart");
	});

	it("inherits full Article options while preserving NewsArticle type", () => {
		const schema = new NewsArticle({
			headline: "News headline",
			author: new Organization({ name: "News Org" }),
			datePublished: "2026-03-01",
			description: "Breaking story",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const author = obj.author as Record<string, unknown>;

		expect(obj["@type"]).toBe("NewsArticle");
		expect(author["@type"]).toBe("Organization");
		expect(obj.datePublished).toBe("2026-03-01");
	});
});
