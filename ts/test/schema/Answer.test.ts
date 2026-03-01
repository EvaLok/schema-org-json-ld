import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";

describe("Answer", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Answer("Use semantic HTML where possible.");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Answer");
		expect(obj.text).toBe("Use semantic HTML where possible.");
	});

	it("omits optional fields when null", () => {
		const schema = new Answer(
			"Use semantic HTML where possible.",
			null,
			null,
			null,
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("url");
		expect(obj).not.toHaveProperty("upvoteCount");
		expect(obj).not.toHaveProperty("datePublished");
		expect(obj).not.toHaveProperty("dateModified");
	});

	it("includes all fields with Person author", () => {
		const schema = new Answer(
			"Use semantic HTML where possible.",
			new Person({ name: "Jane Doe" }),
			"https://example.com/answers/1",
			42,
			"2026-03-01",
			"2026-03-02",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const author = obj.author as Record<string, unknown>;

		expect(author["@type"]).toBe("Person");
		expect(obj.url).toBe("https://example.com/answers/1");
		expect(obj.upvoteCount).toBe(42);
		expect(obj.datePublished).toBe("2026-03-01");
		expect(obj.dateModified).toBe("2026-03-02");
	});

	it("supports Organization author", () => {
		const schema = new Answer(
			"Use semantic HTML where possible.",
			new Organization({ name: "Example Org" }),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const author = obj.author as Record<string, unknown>;

		expect(author["@type"]).toBe("Organization");
		expect(author.name).toBe("Example Org");
	});
});
