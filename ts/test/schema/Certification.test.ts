import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Certification } from "../../src/schema/Certification";
import { Organization } from "../../src/schema/Organization";
import { Rating } from "../../src/schema/Rating";

describe("Certification", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Certification({
			name: "AWS Certified Solutions Architect",
			issuedBy: new Organization({ name: "AWS" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Certification");
		expect(obj.name).toBe("AWS Certified Solutions Architect");
	});

	it("omits optional fields when null", () => {
		const schema = new Certification({
			name: "AWS Certified Solutions Architect",
			issuedBy: new Organization({ name: "AWS" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("certificationIdentification");
		expect(obj).not.toHaveProperty("certificationRating");
	});

	it("includes all fields when set", () => {
		const schema = new Certification({
			name: "AWS Certified Solutions Architect",
			issuedBy: new Organization({ name: "AWS" }),
			certificationIdentification: "CSA-12345",
			certificationRating: new Rating({ ratingValue: 5 }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const certificationRating = obj.certificationRating as Record<
			string,
			unknown
		>;

		expect(obj.certificationIdentification).toBe("CSA-12345");
		expect(certificationRating["@type"]).toBe("Rating");
	});

	it("serializes nested rating fields", () => {
		const schema = new Certification({
			name: "AWS Certified Solutions Architect",
			issuedBy: new Organization({ name: "AWS" }),
			certificationRating: new Rating({
				ratingValue: 5,
				bestRating: 5,
				worstRating: 1,
			}),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const certificationRating = obj.certificationRating as Record<
			string,
			unknown
		>;

		expect(certificationRating["@type"]).toBe("Rating");
		expect(certificationRating.bestRating).toBe(5);
		expect(certificationRating.worstRating).toBe(1);
	});

	it("serializes empty string certification identification", () => {
		const schema = new Certification({
			name: "AWS Certified Solutions Architect",
			issuedBy: new Organization({ name: "AWS" }),
			certificationIdentification: "",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.certificationIdentification).toBe("");
	});
});
