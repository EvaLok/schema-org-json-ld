import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { AdministrativeArea } from "../../src/schema/AdministrativeArea";
import { JobPosting } from "../../src/schema/JobPosting";
import { MonetaryAmount } from "../../src/schema/MonetaryAmount";
import { Organization } from "../../src/schema/Organization";
import { Place } from "../../src/schema/Place";
import { PropertyValue } from "../../src/schema/PropertyValue";

describe("JobPosting", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new JobPosting({
			title: "Software Engineer",
			description: "Build great products.",
			datePosted: "2026-03-01",
			hiringOrganization: new Organization({ name: "Example Corp" }),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("JobPosting");
		expect(obj.title).toBe("Software Engineer");
		expect(obj.description).toBe("Build great products.");
		expect(obj.datePosted).toBe("2026-03-01");
	});

	it("omits optional fields when null", () => {
		const schema = new JobPosting({
			title: "Software Engineer",
			description: "Build great products.",
			datePosted: "2026-03-01",
			hiringOrganization: new Organization({ name: "Example Corp" }),
			jobLocation: null,
			baseSalary: null,
			employmentType: null,
			validThrough: null,
			applicantLocationRequirements: null,
			jobLocationType: null,
			directApply: null,
			identifier: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("jobLocation");
		expect(obj).not.toHaveProperty("baseSalary");
		expect(obj).not.toHaveProperty("employmentType");
		expect(obj).not.toHaveProperty("validThrough");
		expect(obj).not.toHaveProperty("applicantLocationRequirements");
		expect(obj).not.toHaveProperty("jobLocationType");
		expect(obj).not.toHaveProperty("directApply");
		expect(obj).not.toHaveProperty("identifier");
	});

	it("includes all optional fields when set", () => {
		const schema = new JobPosting({
			title: "Software Engineer",
			description: "Build great products.",
			datePosted: "2026-03-01",
			hiringOrganization: new Organization({ name: "Example Corp" }),
			jobLocation: new Place("Remote"),
			baseSalary: new MonetaryAmount("USD", 120000),
			employmentType: "FULL_TIME",
			validThrough: "2026-04-01",
			applicantLocationRequirements: new AdministrativeArea("US"),
			jobLocationType: "TELECOMMUTE",
			directApply: true,
			identifier: new PropertyValue("job-id", "SE-123"),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const jobLocation = obj.jobLocation as Record<string, unknown>;
		const baseSalary = obj.baseSalary as Record<string, unknown>;
		const applicantLocationRequirements = obj.applicantLocationRequirements as Record<
			string,
			unknown
		>;
		const identifier = obj.identifier as Record<string, unknown>;

		expect(jobLocation["@type"]).toBe("Place");
		expect(baseSalary["@type"]).toBe("MonetaryAmount");
		expect(obj.employmentType).toBe("FULL_TIME");
		expect(obj.validThrough).toBe("2026-04-01");
		expect(applicantLocationRequirements["@type"]).toBe("AdministrativeArea");
		expect(obj.jobLocationType).toBe("TELECOMMUTE");
		expect(obj.directApply).toBe(true);
		expect(identifier["@type"]).toBe("PropertyValue");
	});
});
