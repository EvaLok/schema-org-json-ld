import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { EducationalOccupationalCredential } from "../../src/schema/EducationalOccupationalCredential";

describe("EducationalOccupationalCredential", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new EducationalOccupationalCredential({
			credentialCategory: "bachelor degree",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("EducationalOccupationalCredential");
		expect(obj.credentialCategory).toBe("bachelor degree");
	});
});
