import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { PostalAddress } from "../../src/schema/PostalAddress";
import { Store } from "../../src/schema/Store";

describe("Store", () => {
	const address = new PostalAddress({
		streetAddress: "1 Main St",
		addressLocality: "Townsville",
		addressCountry: "US",
	});

	it("produces Store JSON-LD with required fields", () => {
		const schema = new Store({ name: "Example Store", address });
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Store");
		expect(obj.name).toBe("Example Store");
	});

	it("omits inherited optional fields when null", () => {
		const schema = new Store({
			name: "Example Store",
			address,
			url: null,
			review: null,
			department: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("url");
		expect(obj).not.toHaveProperty("review");
		expect(obj).not.toHaveProperty("department");
	});

	it("inherits full LocalBusiness options while preserving Store type", () => {
		const schema = new Store({
			name: "Example Store",
			address,
			telephone: "+1-555-1234",
			description: "Retail store",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@type"]).toBe("Store");
		expect(obj.telephone).toBe("+1-555-1234");
		expect(obj.description).toBe("Retail store");
	});
});
