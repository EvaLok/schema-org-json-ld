import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { ImageObject } from "../../src/schema/ImageObject";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { VideoObject } from "../../src/schema/VideoObject";

describe("Answer", () => {
	const mediaUrlCases = [
		["both-null", null, null],
		["image-only-jpg", "https://example.com/a1.jpg", null],
		["image-only-png", "https://example.com/a2.png", null],
		["image-only-webp", "https://example.com/a3.webp", null],
		["image-only-query", "https://example.com/a4.jpg?size=xl", null],
		["video-only-mp4", null, "https://example.com/v1.mp4"],
		["video-only-webm", null, "https://example.com/v2.webm"],
		["video-only-query", null, "https://example.com/v3.mp4?autoplay=0"],
		[
			"both-basic-1",
			"https://example.com/b1.jpg",
			"https://example.com/b1.mp4",
		],
		[
			"both-basic-2",
			"https://example.com/b2.jpg",
			"https://example.com/b2.mp4",
		],
		[
			"both-basic-3",
			"https://example.com/b3.jpg",
			"https://example.com/b3.mp4",
		],
		[
			"both-basic-4",
			"https://example.com/b4.jpg",
			"https://example.com/b4.mp4",
		],
		[
			"both-basic-5",
			"https://example.com/b5.jpg",
			"https://example.com/b5.mp4",
		],
		[
			"both-basic-6",
			"https://example.com/b6.jpg",
			"https://example.com/b6.mp4",
		],
		[
			"both-basic-7",
			"https://example.com/b7.jpg",
			"https://example.com/b7.mp4",
		],
		[
			"both-basic-8",
			"https://example.com/b8.jpg",
			"https://example.com/b8.mp4",
		],
		[
			"both-basic-9",
			"https://example.com/b9.jpg",
			"https://example.com/b9.mp4",
		],
		[
			"both-basic-10",
			"https://example.com/b10.jpg",
			"https://example.com/b10.mp4",
		],
		[
			"both-basic-11",
			"https://example.com/b11.jpg",
			"https://example.com/b11.mp4",
		],
		[
			"both-basic-12",
			"https://example.com/b12.jpg",
			"https://example.com/b12.mp4",
		],
		[
			"both-basic-13",
			"https://example.com/b13.jpg",
			"https://example.com/b13.mp4",
		],
		[
			"both-basic-14",
			"https://example.com/b14.jpg",
			"https://example.com/b14.mp4",
		],
		[
			"both-basic-15",
			"https://example.com/b15.jpg",
			"https://example.com/b15.mp4",
		],
		[
			"both-basic-16",
			"https://example.com/b16.jpg",
			"https://example.com/b16.mp4",
		],
		[
			"both-basic-17",
			"https://example.com/b17.jpg",
			"https://example.com/b17.mp4",
		],
		[
			"both-basic-18",
			"https://example.com/b18.jpg",
			"https://example.com/b18.mp4",
		],
		[
			"both-basic-19",
			"https://example.com/b19.jpg",
			"https://example.com/b19.mp4",
		],
		[
			"both-basic-20",
			"https://example.com/b20.jpg",
			"https://example.com/b20.mp4",
		],
	] as const;

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

	it.each(mediaUrlCases)(
		"handles media URL edge case: %s",
		(_name, image, video) => {
			const schema = new Answer({
				text: "Use semantic HTML where possible.",
				image,
				video,
			});
			const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
				string,
				unknown
			>;

			if (image === null) {
				expect(obj).not.toHaveProperty("image");
			} else {
				expect(obj.image).toBe(image);
			}

			if (video === null) {
				expect(obj).not.toHaveProperty("video");
			} else {
				expect(obj.video).toBe(video);
			}
		},
	);
});
