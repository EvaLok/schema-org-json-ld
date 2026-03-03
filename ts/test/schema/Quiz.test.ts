import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AlignmentObject } from "../../src/schema/AlignmentObject";
import { Answer } from "../../src/schema/Answer";
import { Question } from "../../src/schema/Question";
import { Quiz } from "../../src/schema/Quiz";

describe("Quiz", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Quiz({
			hasPart: [new Question({ name: "Question 1" })],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const hasPart = obj.hasPart as Record<string, unknown>[];

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Quiz");
		expect(hasPart[0]?.["@type"]).toBe("Question");
	});

	it("omits optional fields when null", () => {
		const schema = new Quiz({
			hasPart: [new Question({ name: "Question 1" })],
			about: null,
			educationalAlignment: null,
			name: null,
			description: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("about");
		expect(obj).not.toHaveProperty("educationalAlignment");
		expect(obj).not.toHaveProperty("description");
	});

	it("includes all fields when set", () => {
		const schema = new Quiz({
			hasPart: [new Question({ name: "Question 1" })],
			about: "Algebra",
			educationalAlignment: new AlignmentObject({
				alignmentType: "teaches",
				targetName: "Linear equations",
			}),
			name: "Math quiz",
			description: "A short math quiz",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const educationalAlignment = obj.educationalAlignment as Record<
			string,
			unknown
		>;

		expect(obj.about).toBe("Algebra");
		expect(obj.name).toBe("Math quiz");
		expect(obj.description).toBe("A short math quiz");
		expect(educationalAlignment["@type"]).toBe("AlignmentObject");
	});

	it("serializes multiple questions in order", () => {
		const schema = new Quiz({
			hasPart: [
				new Question({
					name: "What is 2 + 2?",
					acceptedAnswer: new Answer({ text: "4" }),
				}),
				new Question({
					name: "What is 3 + 3?",
					acceptedAnswer: new Answer({ text: "6" }),
				}),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const hasPart = obj.hasPart as Record<string, unknown>[];
		const second = hasPart[1] as Record<string, unknown>;
		const secondAnswer = second.acceptedAnswer as Record<string, unknown>;

		expect(hasPart).toHaveLength(2);
		expect(hasPart[0]?.name).toBe("What is 2 + 2?");
		expect(secondAnswer.text).toBe("6");
	});

	it("serializes educationalAlignment with targetUrl", () => {
		const schema = new Quiz({
			hasPart: [
				new Question({
					name: "What is 2 + 2?",
					acceptedAnswer: new Answer({ text: "4" }),
				}),
			],
			educationalAlignment: new AlignmentObject({
				alignmentType: "assesses",
				targetName: "Addition basics",
				targetUrl: "https://example.com/curriculum/addition",
			}),
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const educationalAlignment = obj.educationalAlignment as Record<
			string,
			unknown
		>;

		expect(educationalAlignment.alignmentType).toBe("assesses");
		expect(educationalAlignment.targetUrl).toBe(
			"https://example.com/curriculum/addition",
		);
	});
});
