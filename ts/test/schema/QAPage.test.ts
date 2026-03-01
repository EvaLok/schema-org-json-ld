import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { Person } from "../../src/schema/Person";
import { QAPage } from "../../src/schema/QAPage";
import { Question } from "../../src/schema/Question";

describe("QAPage", () => {
	it("produces minimal JSON-LD output with accepted answer", () => {
		const schema = new QAPage(
			new Question({
				name: "How do I reset my password?",
				acceptedAnswer: new Answer(
					"Click the forgot password link on the login page.",
				),
			}),
		);
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const mainEntity = obj.mainEntity as Record<string, unknown>;
		const acceptedAnswer = mainEntity.acceptedAnswer as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("QAPage");
		expect(mainEntity["@type"]).toBe("Question");
		expect(mainEntity.name).toBe("How do I reset my password?");
		expect(acceptedAnswer["@type"]).toBe("Answer");
		expect(acceptedAnswer.text).toBe(
			"Click the forgot password link on the login page.",
		);
	});

	it("supports suggested answers", () => {
		const schema = new QAPage(
			new Question({
				name: "What is the best PHP framework?",
				suggestedAnswer: [
					new Answer("Laravel is very popular."),
					new Answer("Symfony is very flexible."),
				],
			}),
		);
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const mainEntity = obj.mainEntity as Record<string, unknown>;
		const suggestedAnswer = mainEntity.suggestedAnswer as Record<
			string,
			unknown
		>[];

		expect(obj["@type"]).toBe("QAPage");
		expect(mainEntity["@type"]).toBe("Question");
		expect(suggestedAnswer).toHaveLength(2);
		expect(suggestedAnswer[0]?.["@type"]).toBe("Answer");
		expect(suggestedAnswer[0]?.text).toBe("Laravel is very popular.");
		expect(suggestedAnswer[1]?.text).toBe("Symfony is very flexible.");
	});

	it("produces full JSON-LD output with all Question fields", () => {
		const schema = new QAPage(
			new Question({
				name: "How do I reset my password?",
				acceptedAnswer: new Answer(
					"Click the forgot password link on the login page.",
				),
				suggestedAnswer: [new Answer("You can also contact support.")],
				answerCount: 2,
				text: "I cannot find the reset password option anywhere on the site.",
				upvoteCount: 42,
				author: new Person({ name: "Jane Doe" }),
				datePublished: "2024-01-15",
				dateModified: "2024-02-01",
			}),
		);
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const question = obj.mainEntity as Record<string, unknown>;
		const author = question.author as Record<string, unknown>;
		const suggestedAnswer = question.suggestedAnswer as Record<
			string,
			unknown
		>[];

		expect(obj["@type"]).toBe("QAPage");
		expect(question.name).toBe("How do I reset my password?");
		expect(suggestedAnswer).toHaveLength(1);
		expect(question.answerCount).toBe(2);
		expect(question.text).toBe(
			"I cannot find the reset password option anywhere on the site.",
		);
		expect(question.upvoteCount).toBe(42);
		expect(author["@type"]).toBe("Person");
		expect(author.name).toBe("Jane Doe");
		expect(question.datePublished).toBe("2024-01-15");
		expect(question.dateModified).toBe("2024-02-01");
	});

	it("omits null question fields from JSON output", () => {
		const schema = new QAPage(new Question({ name: "Simple question?" }));
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const question = obj.mainEntity as Record<string, unknown>;

		expect(question).not.toHaveProperty("acceptedAnswer");
		expect(question).not.toHaveProperty("suggestedAnswer");
		expect(question).not.toHaveProperty("answerCount");
		expect(question).not.toHaveProperty("text");
		expect(question).not.toHaveProperty("upvoteCount");
		expect(question).not.toHaveProperty("author");
		expect(question).not.toHaveProperty("datePublished");
		expect(question).not.toHaveProperty("dateModified");
	});
});
