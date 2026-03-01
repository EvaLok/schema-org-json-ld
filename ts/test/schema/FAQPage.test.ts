import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { FAQPage } from "../../src/schema/FAQPage";
import { Question } from "../../src/schema/Question";

describe("FAQPage", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new FAQPage([new Question({ name: "What is this?" })]);
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const mainEntity = obj.mainEntity as Record<string, unknown>[];

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("FAQPage");
		expect(mainEntity[0]?.["@type"]).toBe("Question");
		expect(mainEntity[0]?.name).toBe("What is this?");
	});

	it("omits null nested optional fields", () => {
		const schema = new FAQPage([
			new Question({
				name: "What is this?",
				acceptedAnswer: null,
				text: null,
			}),
		]);
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const question = (obj.mainEntity as Record<string, unknown>[])[0] as Record<
			string,
			unknown
		>;

		expect(question).not.toHaveProperty("acceptedAnswer");
		expect(question).not.toHaveProperty("text");
	});

	it("includes full nested question data", () => {
		const schema = new FAQPage([
			new Question({
				name: "What is this?",
				acceptedAnswer: new Answer("A FAQ entry."),
			}),
		]);
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const acceptedAnswer = (
			(obj.mainEntity as Record<string, unknown>[])[0] as Record<string, unknown>
		).acceptedAnswer as Record<string, unknown>;

		expect(acceptedAnswer["@type"]).toBe("Answer");
		expect(acceptedAnswer.text).toBe("A FAQ entry.");
	});
});
