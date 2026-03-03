import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { ImageObject } from "../../src/schema/ImageObject";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { VideoObject } from "../../src/schema/VideoObject";

describe("Answer", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Answer({ text: "Use semantic HTML where possible." });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Answer");
		expect(obj.text).toBe("Use semantic HTML where possible.");
	});

	it("omits optional fields when null", () => {
		const schema = new Answer({ text: "Use semantic HTML where possible." });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("url");
		expect(obj).not.toHaveProperty("image");
		expect(obj).not.toHaveProperty("video");
		expect(obj).not.toHaveProperty("upvoteCount");
		expect(obj).not.toHaveProperty("datePublished");
		expect(obj).not.toHaveProperty("dateModified");
	});

	it("includes all fields with Person author", () => {
		const schema = new Answer({
			text: "Use semantic HTML where possible.",
			author: new Person({ name: "Jane Doe" }),
			url: "https://example.com/answers/1",
			upvoteCount: 42,
			datePublished: "2026-03-01",
			dateModified: "2026-03-02",
		});
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
		const schema = new Answer({
			text: "Use semantic HTML where possible.",
			author: new Organization({ name: "Example Org" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const author = obj.author as Record<string, unknown>;

		expect(author["@type"]).toBe("Organization");
		expect(author.name).toBe("Example Org");
	});

	it("serializes image and video as URL strings", () => {
		const schema = new Answer({
			text: "Use semantic HTML where possible.",
			image: "https://example.com/answer.jpg",
			video: "https://example.com/answer.mp4",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj.image).toBe("https://example.com/answer.jpg");
		expect(obj.video).toBe("https://example.com/answer.mp4");
	});

	it("serializes image and video as schema objects", () => {
		const schema = new Answer({
			text: "Use semantic HTML where possible.",
			image: new ImageObject({ contentUrl: "https://example.com/answer.jpg" }),
			video: new VideoObject({
				name: "Answer video",
				thumbnailUrl: ["https://example.com/thumb.jpg"],
				uploadDate: "2026-03-01",
			}),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const image = obj.image as Record<string, unknown>;
		const video = obj.video as Record<string, unknown>;

		expect(image["@type"]).toBe("ImageObject");
		expect(image.contentUrl).toBe("https://example.com/answer.jpg");
		expect(video["@type"]).toBe("VideoObject");
		expect(video.name).toBe("Answer video");
	});
});
