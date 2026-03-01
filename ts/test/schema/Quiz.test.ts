import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AlignmentObject } from "../../src/schema/AlignmentObject";
import { Question } from "../../src/schema/Question";
import { Quiz } from "../../src/schema/Quiz";

describe("Quiz", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Quiz([new Question({ name: "Question 1" })]);
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
		const schema = new Quiz(
			[new Question({ name: "Question 1" })],
			null,
			null,
			null,
			null,
		);
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("about");
		expect(obj).not.toHaveProperty("educationalAlignment");
		expect(obj).not.toHaveProperty("description");
	});

	it("includes all fields when set", () => {
		const schema = new Quiz(
			[new Question({ name: "Question 1" })],
			"Algebra",
			new AlignmentObject("teaches", "Linear equations"),
			"Math quiz",
			"A short math quiz",
		);
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
});
