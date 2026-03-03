import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { OccupationalExperienceRequirements } from "../../src/schema/OccupationalExperienceRequirements";

describe("OccupationalExperienceRequirements", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new OccupationalExperienceRequirements({
			monthsOfExperience: 24,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("OccupationalExperienceRequirements");
		expect(obj.monthsOfExperience).toBe(24);
	});
});
