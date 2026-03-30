import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { SeekToAction } from "../../src/schema/SeekToAction";

describe("SeekToAction", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new SeekToAction({
			target: "https://example.com/watch?v=abc&t={seek_to_second_number}",
			startOffsetInput: "required name=seek_to_second_number",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("SeekToAction");
		expect(obj.target).toBe(
			"https://example.com/watch?v=abc&t={seek_to_second_number}",
		);
		expect(obj["startOffset-input"]).toBe(
			"required name=seek_to_second_number",
		);
		expect(obj).not.toHaveProperty("startOffsetInput");
	});

	it("has the correct schema type", () => {
		const schema = new SeekToAction({
			target: "https://example.com/watch?v=video&t={seek_to_second_number}",
			startOffsetInput: "required name=seek_to_second_number",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@type"]).toBe("SeekToAction");
	});

	it("serializes both required fields with the mapped property name", () => {
		const schema = new SeekToAction({
			target: "https://example.com/watch?v=video&t={seek_to_second_number}",
			startOffsetInput: "required name=seek_to_second_number",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Object.keys(obj)).toEqual([
			"@context",
			"@type",
			"target",
			"startOffset-input",
		]);
		expect(obj.target).toBe(
			"https://example.com/watch?v=video&t={seek_to_second_number}",
		);
		expect(obj["startOffset-input"]).toBe(
			"required name=seek_to_second_number",
		);
	});

	it("serializes empty required strings", () => {
		const schema = new SeekToAction({
			target: "",
			startOffsetInput: "",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.target).toBe("");
		expect(obj["startOffset-input"]).toBe("");
	});
});
