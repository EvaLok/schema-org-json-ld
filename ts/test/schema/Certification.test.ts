import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Certification } from "../../src/schema/Certification";
import { Organization } from "../../src/schema/Organization";
import { Rating } from "../../src/schema/Rating";

describe("Certification", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Certification(
			"AWS Certified Solutions Architect",
			new Organization({ name: "AWS" }),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Certification");
		expect(obj.name).toBe("AWS Certified Solutions Architect");
	});

	it("omits optional fields when null", () => {
		const schema = new Certification(
			"AWS Certified Solutions Architect",
			new Organization({ name: "AWS" }),
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("certificationIdentification");
		expect(obj).not.toHaveProperty("certificationRating");
	});

	it("includes all fields when set", () => {
		const schema = new Certification(
			"AWS Certified Solutions Architect",
			new Organization({ name: "AWS" }),
			"CSA-12345",
			new Rating(5),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const certificationRating = obj.certificationRating as Record<
			string,
			unknown
		>;

		expect(obj.certificationIdentification).toBe("CSA-12345");
		expect(certificationRating["@type"]).toBe("Rating");
	});
});
