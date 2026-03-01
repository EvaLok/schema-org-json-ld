import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { ContactPoint } from "../../src/schema/ContactPoint";

describe("ContactPoint", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ContactPoint();
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ContactPoint");
	});

	it("omits optional fields when null", () => {
		const schema = new ContactPoint(null, null, null, null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("telephone");
		expect(obj).not.toHaveProperty("email");
		expect(obj).not.toHaveProperty("contactType");
		expect(obj).not.toHaveProperty("areaServed");
		expect(obj).not.toHaveProperty("availableLanguage");
	});

	it("includes all fields when set", () => {
		const schema = new ContactPoint(
			"+1-800-123-4567",
			"support@example.com",
			"customer support",
			"US",
			"en",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.telephone).toBe("+1-800-123-4567");
		expect(obj.email).toBe("support@example.com");
		expect(obj.contactType).toBe("customer support");
		expect(obj.areaServed).toBe("US");
		expect(obj.availableLanguage).toBe("en");
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new ContactPoint(
			"+1-800-123-4567",
			"support@example.com",
			"customer support",
			"US",
			"en",
		);
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "ContactPoint",\n  "telephone": "+1-800-123-4567",\n  "email": "support@example.com",\n  "contactType": "customer support",\n  "areaServed": "US",\n  "availableLanguage": "en"\n}',
		);
	});
});
