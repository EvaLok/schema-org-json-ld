import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { BroadcastEvent } from "../../src/schema/BroadcastEvent";

describe("BroadcastEvent", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new BroadcastEvent(true);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("BroadcastEvent");
		expect(obj.isLiveBroadcast).toBe(true);
	});

	it("omits optional fields when null", () => {
		const schema = new BroadcastEvent(true, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("startDate");
		expect(obj).not.toHaveProperty("endDate");
	});

	it("includes all fields when set", () => {
		const schema = new BroadcastEvent(
			true,
			"2026-01-01T12:00:00Z",
			"2026-01-01T13:00:00Z",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.startDate).toBe("2026-01-01T12:00:00Z");
		expect(obj.endDate).toBe("2026-01-01T13:00:00Z");
	});
});
