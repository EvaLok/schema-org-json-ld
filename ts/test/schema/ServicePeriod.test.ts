import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DayOfWeek } from "../../src/enum/DayOfWeek";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { ServicePeriod } from "../../src/schema/ServicePeriod";

describe("ServicePeriod", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ServicePeriod();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ServicePeriod");
	});

	it("omits optional fields when null", () => {
		const schema = new ServicePeriod(null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("duration");
		expect(obj).not.toHaveProperty("businessDays");
		expect(obj).not.toHaveProperty("cutoffTime");
	});

	it("includes all fields when set", () => {
		const schema = new ServicePeriod(
			new QuantitativeValue(2, "DAY"),
			[DayOfWeek.Monday, DayOfWeek.Friday],
			"17:00:00Z",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const duration = obj.duration as Record<string, unknown>;

		expect(duration["@type"]).toBe("QuantitativeValue");
		expect(duration.value).toBe(2);
		expect(obj.businessDays).toEqual([
			"https://schema.org/Monday",
			"https://schema.org/Friday",
		]);
		expect(obj.cutoffTime).toBe("17:00:00Z");
	});
});
