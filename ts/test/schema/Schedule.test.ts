import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Schedule } from "../../src/schema/Schedule";

describe("Schedule", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Schedule("P1D");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Schedule");
		expect(obj.repeatFrequency).toBe("P1D");
	});

	it("omits optional fields when null", () => {
		const schema = new Schedule("P1D", null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("repeatCount");
		expect(obj).not.toHaveProperty("startDate");
		expect(obj).not.toHaveProperty("endDate");
	});

	it("includes all fields when set", () => {
		const schema = new Schedule("P1D", 10, "2026-01-01", "2026-01-10");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.repeatCount).toBe(10);
		expect(obj.startDate).toBe("2026-01-01");
		expect(obj.endDate).toBe("2026-01-10");
	});
});
