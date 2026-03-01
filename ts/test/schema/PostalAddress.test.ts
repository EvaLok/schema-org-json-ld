import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { PostalAddress } from "../../src/schema/PostalAddress";

describe("PostalAddress", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new PostalAddress({});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("PostalAddress");
	});

	it("omits optional fields when null", () => {
		const schema = new PostalAddress({
			streetAddress: null,
			addressLocality: null,
			addressRegion: null,
			postalCode: null,
			addressCountry: null,
			postOfficeBoxNumber: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("streetAddress");
		expect(obj).not.toHaveProperty("addressLocality");
		expect(obj).not.toHaveProperty("addressRegion");
		expect(obj).not.toHaveProperty("postalCode");
		expect(obj).not.toHaveProperty("addressCountry");
		expect(obj).not.toHaveProperty("postOfficeBoxNumber");
	});

	it("includes all fields when set", () => {
		const schema = new PostalAddress({
			streetAddress: "1600 Amphitheatre Parkway",
			addressLocality: "Mountain View",
			addressRegion: "CA",
			postalCode: "94043",
			addressCountry: "US",
			postOfficeBoxNumber: "123",
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.streetAddress).toBe("1600 Amphitheatre Parkway");
		expect(obj.addressLocality).toBe("Mountain View");
		expect(obj.addressRegion).toBe("CA");
		expect(obj.postalCode).toBe("94043");
		expect(obj.addressCountry).toBe("US");
		expect(obj.postOfficeBoxNumber).toBe("123");
	});

	it("matches PHP parity JSON-LD output", () => {
		const schema = new PostalAddress({
			streetAddress: "1600 Amphitheatre Parkway",
			addressLocality: "Mountain View",
			addressRegion: "CA",
			postalCode: "94043",
			addressCountry: "US",
			postOfficeBoxNumber: "123",
		});
		const json = JsonLdGenerator.schemaToJson(schema);

		expect(json).toBe(
			'{\n  "@context": "https://schema.org/",\n  "@type": "PostalAddress",\n  "streetAddress": "1600 Amphitheatre Parkway",\n  "addressLocality": "Mountain View",\n  "addressRegion": "CA",\n  "postalCode": "94043",\n  "addressCountry": "US",\n  "postOfficeBoxNumber": "123"\n}',
		);
	});
});
