import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DayOfWeek } from "../../src/enum/DayOfWeek";
import { OpeningHoursSpecification } from "../../src/schema/OpeningHoursSpecification";

describe("OpeningHoursSpecification", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new OpeningHoursSpecification({});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("OpeningHoursSpecification");
	});

	it("omits optional fields when null", () => {
		const schema = new OpeningHoursSpecification({
			dayOfWeek: null,
			opens: null,
			closes: null,
			validFrom: null,
			validThrough: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("dayOfWeek");
		expect(obj).not.toHaveProperty("opens");
		expect(obj).not.toHaveProperty("closes");
		expect(obj).not.toHaveProperty("validFrom");
		expect(obj).not.toHaveProperty("validThrough");
	});

	it("includes all fields when set", () => {
		const schema = new OpeningHoursSpecification({
			dayOfWeek: DayOfWeek.Monday,
			opens: "09:00",
			closes: "17:00",
			validFrom: "2026-01-01",
			validThrough: "2026-12-31",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.dayOfWeek).toBe("https://schema.org/Monday");
		expect(obj.opens).toBe("09:00");
		expect(obj.closes).toBe("17:00");
		expect(obj.validFrom).toBe("2026-01-01");
		expect(obj.validThrough).toBe("2026-12-31");
	});
});
