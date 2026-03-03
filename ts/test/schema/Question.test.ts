import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { ImageObject } from "../../src/schema/ImageObject";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { Question } from "../../src/schema/Question";
import { VideoObject } from "../../src/schema/VideoObject";

describe("Question", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Question({ name: "What is TypeScript?" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Question");
		expect(obj.name).toBe("What is TypeScript?");
	});

	it("omits optional fields when null", () => {
		const schema = new Question({
			name: "What is TypeScript?",
			acceptedAnswer: null,
			suggestedAnswer: null,
			answerCount: null,
			text: null,
			upvoteCount: null,
			author: null,
			datePublished: null,
			dateModified: null,
			eduQuestionType: null,
			image: null,
			video: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("acceptedAnswer");
		expect(obj).not.toHaveProperty("suggestedAnswer");
		expect(obj).not.toHaveProperty("author");
		expect(obj).not.toHaveProperty("image");
		expect(obj).not.toHaveProperty("video");
	});

	it("supports author as Person and Organization", () => {
		const personSchema = new Question({
			name: "Who wrote this?",
			author: new Person({ name: "Jane" }),
		});
		const personObj = JSON.parse(
			JsonLdGenerator.schemaToJson(personSchema),
		) as Record<string, unknown>;
		const personAuthor = personObj.author as Record<string, unknown>;

		expect(personAuthor["@type"]).toBe("Person");

		const organizationSchema = new Question({
			name: "Who published this?",
			author: new Organization({ name: "Example Org" }),
		});
		const organizationObj = JSON.parse(
			JsonLdGenerator.schemaToJson(organizationSchema),
		) as Record<string, unknown>;
		const organizationAuthor = organizationObj.author as Record<
			string,
			unknown
		>;

		expect(organizationAuthor["@type"]).toBe("Organization");
	});

	it("supports suggestedAnswer as an array", () => {
		const schema = new Question({
			name: "What is 2 + 2?",
			acceptedAnswer: new Answer({ text: "4" }),
			suggestedAnswer: [new Answer({ text: "3" }), new Answer({ text: "5" })],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const acceptedAnswer = obj.acceptedAnswer as Record<string, unknown>;
		const suggestedAnswer = obj.suggestedAnswer as Record<string, unknown>[];

		expect(acceptedAnswer["@type"]).toBe("Answer");
		expect(suggestedAnswer).toHaveLength(2);
		expect(suggestedAnswer[0]?.["@type"]).toBe("Answer");
	});

	it("serializes image and video as URL strings", () => {
		const schema = new Question({
			name: "What is TypeScript?",
			image: "https://example.com/question.jpg",
			video: "https://example.com/question.mp4",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj.image).toBe("https://example.com/question.jpg");
		expect(obj.video).toBe("https://example.com/question.mp4");
	});

	it("serializes image and video as schema objects", () => {
		const schema = new Question({
			name: "What is TypeScript?",
			image: new ImageObject({
				contentUrl: "https://example.com/question.jpg",
			}),
			video: new VideoObject({
				name: "Question video",
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
		expect(image.contentUrl).toBe("https://example.com/question.jpg");
		expect(video["@type"]).toBe("VideoObject");
		expect(video.name).toBe("Question video");
	});
});
