import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { Question } from "../../src/schema/Question";

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
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("acceptedAnswer");
		expect(obj).not.toHaveProperty("suggestedAnswer");
		expect(obj).not.toHaveProperty("author");
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
			acceptedAnswer: new Answer("4"),
			suggestedAnswer: [new Answer("3"), new Answer("5")],
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
});
