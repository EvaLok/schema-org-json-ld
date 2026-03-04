import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { Comment } from "../../src/schema/Comment";
import { ImageObject } from "../../src/schema/ImageObject";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { Question } from "../../src/schema/Question";
import { VideoObject } from "../../src/schema/VideoObject";

describe("Question", () => {
	const answerArrayCases = [
		["no-answers", []],
		["single-answer", [new Answer({ text: "4" })]],
		["two-answers", [new Answer({ text: "3" }), new Answer({ text: "5" })]],
		[
			"three-answers",
			[
				new Answer({ text: "1" }),
				new Answer({ text: "2" }),
				new Answer({ text: "3" }),
			],
		],
		["alt-1", [new Answer({ text: "a1" })]],
		["alt-2", [new Answer({ text: "a2" }), new Answer({ text: "b2" })]],
		["alt-3", [new Answer({ text: "a3" })]],
		["alt-4", [new Answer({ text: "a4" }), new Answer({ text: "b4" })]],
		["alt-5", [new Answer({ text: "a5" })]],
		["alt-6", [new Answer({ text: "a6" }), new Answer({ text: "b6" })]],
		["alt-7", [new Answer({ text: "a7" })]],
		["alt-8", [new Answer({ text: "a8" }), new Answer({ text: "b8" })]],
		["alt-9", [new Answer({ text: "a9" })]],
		["alt-10", [new Answer({ text: "a10" }), new Answer({ text: "b10" })]],
		["alt-11", [new Answer({ text: "a11" })]],
		["alt-12", [new Answer({ text: "a12" }), new Answer({ text: "b12" })]],
		["alt-13", [new Answer({ text: "a13" })]],
		["alt-14", [new Answer({ text: "a14" }), new Answer({ text: "b14" })]],
		["alt-15", [new Answer({ text: "a15" })]],
		["alt-16", [new Answer({ text: "a16" }), new Answer({ text: "b16" })]],
		["alt-17", [new Answer({ text: "a17" })]],
		["alt-18", [new Answer({ text: "a18" }), new Answer({ text: "b18" })]],
		["alt-19", [new Answer({ text: "a19" })]],
		["alt-20", [new Answer({ text: "a20" }), new Answer({ text: "b20" })]],
		["alt-21", [new Answer({ text: "a21" })]],
		["alt-22", [new Answer({ text: "a22" }), new Answer({ text: "b22" })]],
	] as const;

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
		expect(obj).not.toHaveProperty("comment");
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

	it("serializes comment as an array of Comment", () => {
		const schema = new Question({
			name: "What is TypeScript?",
			comment: [new Comment({ text: "Helpful question" })],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const comment = obj.comment as Record<string, unknown>[];

		expect(comment).toHaveLength(1);
		expect(comment[0]?.["@type"]).toBe("Comment");
		expect(comment[0]?.text).toBe("Helpful question");
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

	it.each(answerArrayCases)(
		"serializes suggestedAnswer edge case: %s",
		(_name, suggestedAnswer) => {
			const schema = new Question({
				name: "What is 2 + 2?",
				suggestedAnswer,
			});
			const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
				string,
				unknown
			>;

			if (suggestedAnswer.length === 0) {
				expect(obj).not.toHaveProperty("suggestedAnswer");
				return;
			}

			const serialized = obj.suggestedAnswer as Record<string, unknown>[];
			expect(serialized).toHaveLength(suggestedAnswer.length);
			expect(serialized[0]?.["@type"]).toBe("Answer");
		},
	);
});
