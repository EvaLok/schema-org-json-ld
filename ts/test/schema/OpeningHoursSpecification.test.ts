import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DayOfWeek } from "../../src/enum/DayOfWeek";
import { OpeningHoursSpecification } from "../../src/schema/OpeningHoursSpecification";

describe("OpeningHoursSpecification", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new OpeningHoursSpecification();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("OpeningHoursSpecification");
	});

	it("omits optional fields when null", () => {
		const schema = new OpeningHoursSpecification(null, null, null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("dayOfWeek");
		expect(obj).not.toHaveProperty("opens");
		expect(obj).not.toHaveProperty("closes");
		expect(obj).not.toHaveProperty("validFrom");
		expect(obj).not.toHaveProperty("validThrough");
	});

	it("includes all fields when set", () => {
		const schema = new OpeningHoursSpecification(
			DayOfWeek.Monday,
			"09:00",
			"17:00",
			"2026-01-01",
			"2026-12-31",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.dayOfWeek).toBe("https://schema.org/Monday");
		expect(obj.opens).toBe("09:00");
		expect(obj.closes).toBe("17:00");
		expect(obj.validFrom).toBe("2026-01-01");
		expect(obj.validThrough).toBe("2026-12-31");
	});
});
