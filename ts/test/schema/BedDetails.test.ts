import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { BedDetails } from "../../src/schema/BedDetails";

describe("BedDetails", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new BedDetails({ numberOfBeds: 2 });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("BedDetails");
		expect(obj.numberOfBeds).toBe(2);
	});

	it("omits optional fields when null", () => {
		const schema = new BedDetails({ numberOfBeds: 2 });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("typeOfBed");
	});

	it("includes all fields when set", () => {
		const schema = new BedDetails({ numberOfBeds: 2, typeOfBed: "Queen" });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.typeOfBed).toBe("Queen");
	});
});
