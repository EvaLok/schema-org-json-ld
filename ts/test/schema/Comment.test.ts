import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Comment } from "../../src/schema/Comment";
import { InteractionCounter } from "../../src/schema/InteractionCounter";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";

describe("Comment", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Comment({ text: "Nice post!" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Comment");
		expect(obj.text).toBe("Nice post!");
	});

	it("omits optional fields when null", () => {
		const schema = new Comment({
			text: "Nice post!",
			author: null,
			datePublished: null,
			url: null,
			dateModified: null,
			image: null,
			video: null,
			comment: null,
			interactionStatistic: null,
			sharedContent: null,
			creativeWorkStatus: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("comment");
		expect(obj).not.toHaveProperty("interactionStatistic");
	});

	it("supports self-referential comments and interactionStatistic single", () => {
		const schema = new Comment({
			text: "Top level",
			author: new Person({ name: "Alice" }),
			comment: [new Comment({ text: "Nested reply" })],
			interactionStatistic: new InteractionCounter("LikeAction", 10),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const author = obj.author as Record<string, unknown>;
		const comment = obj.comment as Record<string, unknown>[];
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>;

		expect(author["@type"]).toBe("Person");
		expect(comment[0]?.["@type"]).toBe("Comment");
		expect(interactionStatistic["@type"]).toBe("InteractionCounter");
	});

	it("supports author as Organization and interactionStatistic as array", () => {
		const schema = new Comment({
			text: "Top level",
			author: new Organization({ name: "Example Org" }),
			interactionStatistic: [
				new InteractionCounter("LikeAction", 10),
				new InteractionCounter("ShareAction", 3),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const author = obj.author as Record<string, unknown>;
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>[];

		expect(author["@type"]).toBe("Organization");
		expect(interactionStatistic).toHaveLength(2);
		expect(interactionStatistic[0]?.["@type"]).toBe("InteractionCounter");
	});
});
