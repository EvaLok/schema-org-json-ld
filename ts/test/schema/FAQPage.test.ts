import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Answer } from "../../src/schema/Answer";
import { FAQPage } from "../../src/schema/FAQPage";
import { Question } from "../../src/schema/Question";

describe("FAQPage", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new FAQPage({
			mainEntity: [new Question({ name: "What is this?" })],
		});
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
		const schema = new FAQPage({
			mainEntity: [
				new Question({
					name: "What is this?",
					acceptedAnswer: null,
					text: null,
				}),
			],
		});
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
		const schema = new FAQPage({
			mainEntity: [
				new Question({
					name: "What is this?",
					acceptedAnswer: new Answer({ text: "A FAQ entry." }),
				}),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const acceptedAnswer = (
			(obj.mainEntity as Record<string, unknown>[])[0] as Record<
				string,
				unknown
			>
		).acceptedAnswer as Record<string, unknown>;

		expect(acceptedAnswer["@type"]).toBe("Answer");
		expect(acceptedAnswer.text).toBe("A FAQ entry.");
	});

	it("supports a single-question FAQ structure", () => {
		const schema = new FAQPage({
			mainEntity: [
				new Question({
					name: "How long does shipping take?",
					acceptedAnswer: new Answer({ text: "Shipping takes 2-4 business days." }),
				}),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const question = (obj.mainEntity as Record<string, unknown>[])[0] as Record<
			string,
			unknown
		>;
		const acceptedAnswer = question.acceptedAnswer as Record<string, unknown>;

		expect(obj.mainEntity).toHaveLength(1);
		expect(question.name).toBe("How long does shipping take?");
		expect(acceptedAnswer.text).toBe("Shipping takes 2-4 business days.");
	});

	it("preserves HTML in acceptedAnswer text", () => {
		const schema = new FAQPage({
			mainEntity: [
				new Question({
					name: "Can I use formatting in answers?",
					acceptedAnswer: new Answer({
						text: "<p>Yes, use <strong>HTML</strong> when needed.</p>",
					}),
				}),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const question = (obj.mainEntity as Record<string, unknown>[])[0] as Record<
			string,
			unknown
		>;
		const acceptedAnswer = question.acceptedAnswer as Record<string, unknown>;

		expect(acceptedAnswer.text).toBe(
			"<p>Yes, use <strong>HTML</strong> when needed.</p>",
		);
	});

	it("serializes large FAQs with five questions", () => {
		const schema = new FAQPage({
			mainEntity: [
				new Question({ name: "Q1", acceptedAnswer: new Answer({ text: "A1" }) }),
				new Question({ name: "Q2", acceptedAnswer: new Answer({ text: "A2" }) }),
				new Question({ name: "Q3", acceptedAnswer: new Answer({ text: "A3" }) }),
				new Question({ name: "Q4", acceptedAnswer: new Answer({ text: "A4" }) }),
				new Question({ name: "Q5", acceptedAnswer: new Answer({ text: "A5" }) }),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const mainEntity = obj.mainEntity as Record<string, unknown>[];
		const fifth = mainEntity[4] as Record<string, unknown>;
		const fifthAnswer = fifth.acceptedAnswer as Record<string, unknown>;

		expect(mainEntity).toHaveLength(5);
		expect(mainEntity[0]?.name).toBe("Q1");
		expect(fifthAnswer.text).toBe("A5");
	});
});
