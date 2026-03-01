import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Comment } from "../../src/schema/Comment";
import { DiscussionForumPosting } from "../../src/schema/DiscussionForumPosting";
import { ImageObject } from "../../src/schema/ImageObject";
import { InteractionCounter } from "../../src/schema/InteractionCounter";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { VideoObject } from "../../src/schema/VideoObject";

describe("DiscussionForumPosting", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new DiscussionForumPosting({
			author: new Person({ name: "Jane" }),
			datePublished: "2026-03-01",
			text: "Post text",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const author = obj.author as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("DiscussionForumPosting");
		expect(author["@type"]).toBe("Person");
		expect(obj.text).toBe("Post text");
	});

	it("omits optional fields when null", () => {
		const schema = new DiscussionForumPosting({
			author: new Person({ name: "Jane" }),
			datePublished: "2026-03-01",
			text: "Post text",
			headline: null,
			url: null,
			dateModified: null,
			image: null,
			video: null,
			comment: null,
			interactionStatistic: null,
			isPartOf: null,
			sharedContent: null,
			creativeWorkStatus: null,
			mainEntityOfPage: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("headline");
		expect(obj).not.toHaveProperty("image");
		expect(obj).not.toHaveProperty("interactionStatistic");
	});

	it("supports Organization author and interactionStatistic as single value", () => {
		const schema = new DiscussionForumPosting({
			author: new Organization({ name: "Forum Org" }),
			datePublished: "2026-03-01",
			text: "Post text",
			image: new ImageObject({ contentUrl: "https://example.com/image.jpg" }),
			video: new VideoObject({
				name: "Video",
				thumbnailUrl: ["https://example.com/thumb.jpg"],
				uploadDate: "2026-03-01",
			}),
			comment: [new Comment({ text: "Nice post" })],
			interactionStatistic: new InteractionCounter("LikeAction", 5),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const author = obj.author as Record<string, unknown>;
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>;
		const comment = (obj.comment as Record<string, unknown>[])[0] as Record<
			string,
			unknown
		>;

		expect(author["@type"]).toBe("Organization");
		expect(interactionStatistic["@type"]).toBe("InteractionCounter");
		expect(comment["@type"]).toBe("Comment");
	});

	it("supports interactionStatistic as an array", () => {
		const schema = new DiscussionForumPosting({
			author: new Person({ name: "Jane" }),
			datePublished: "2026-03-01",
			text: "Post text",
			interactionStatistic: [new InteractionCounter("LikeAction", 5)],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const interactionStatistic = obj.interactionStatistic as Record<
			string,
			unknown
		>[];

		expect(interactionStatistic[0]?.["@type"]).toBe("InteractionCounter");
	});
});
