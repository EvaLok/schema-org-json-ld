import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Article } from "../../src/schema/Article";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { WebPageElement } from "../../src/schema/WebPageElement";

describe("Article", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Article({ headline: "Breaking News" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Article");
		expect(obj.headline).toBe("Breaking News");
	});

	it("omits optional fields when null", () => {
		const schema = new Article({
			headline: "Breaking News",
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
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("image");
		expect(obj).not.toHaveProperty("hasPart");
	});

	it("supports author as Person, Organization, and array", () => {
		const personAuthor = new Article({
			headline: "Person authored",
			author: new Person({ name: "Alice" }),
		});
		const personObj = JSON.parse(
			JsonLdGenerator.schemaToJson(personAuthor),
		) as Record<string, unknown>;
		const person = personObj.author as Record<string, unknown>;
		expect(person["@type"]).toBe("Person");

		const organizationAuthor = new Article({
			headline: "Org authored",
			author: new Organization({ name: "Example Org" }),
		});
		const organizationObj = JSON.parse(
			JsonLdGenerator.schemaToJson(organizationAuthor),
		) as Record<string, unknown>;
		const organization = organizationObj.author as Record<string, unknown>;
		expect(organization["@type"]).toBe("Organization");

		const arrayAuthor = new Article({
			headline: "Multi authored",
			author: [
				new Person({ name: "Alice" }),
				new Organization({ name: "Example Org" }),
			],
		});
		const arrayObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arrayAuthor),
		) as Record<string, unknown>;
		const authors = arrayObj.author as Record<string, unknown>[];
		expect(authors).toHaveLength(2);
		expect(authors[0]?.["@type"]).toBe("Person");
		expect(authors[1]?.["@type"]).toBe("Organization");
	});

	it("supports hasPart as single and array", () => {
		const singlePart = new Article({
			headline: "Single part",
			hasPart: new WebPageElement(true, ".main"),
		});
		const singleObj = JSON.parse(
			JsonLdGenerator.schemaToJson(singlePart),
		) as Record<string, unknown>;
		const singleHasPart = singleObj.hasPart as Record<string, unknown>;
		expect(singleHasPart["@type"]).toBe("WebPageElement");

		const arrayPart = new Article({
			headline: "Array part",
			hasPart: [new WebPageElement(true, ".main")],
		});
		const arrayObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arrayPart),
		) as Record<string, unknown>;
		const arrayHasPart = arrayObj.hasPart as Record<string, unknown>[];
		expect(arrayHasPart[0]?.["@type"]).toBe("WebPageElement");
	});
});
