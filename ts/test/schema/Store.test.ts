import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DayOfWeek } from "../../src/enum/DayOfWeek";
import { OpeningHoursSpecification } from "../../src/schema/OpeningHoursSpecification";
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

	it("serializes opening hours across multiple days with DayOfWeek enum values", () => {
		const schema = new Store({
			name: "Example Store",
			address,
			openingHoursSpecification: [
				new OpeningHoursSpecification({
					dayOfWeek: DayOfWeek.Monday,
					opens: "09:00",
					closes: "18:00",
				}),
				new OpeningHoursSpecification({
					dayOfWeek: DayOfWeek.Tuesday,
					opens: "09:00",
					closes: "18:00",
				}),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const hours = obj.openingHoursSpecification as Record<string, unknown>[];

		expect(hours).toHaveLength(2);
		expect(hours[0]?.dayOfWeek).toBe("https://schema.org/Monday");
		expect(hours[1]?.dayOfWeek).toBe("https://schema.org/Tuesday");
	});

	it("preserves Store type with extended inherited properties", () => {
		const schema = new Store({
			name: "Example Store",
			address,
			description: "Neighborhood store",
			menu: "https://example.com/cafe-menu",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@type"]).toBe("Store");
		expect(obj.description).toBe("Neighborhood store");
		expect(obj.menu).toBe("https://example.com/cafe-menu");
	});

	it("supports department as an array of nested stores", () => {
		const schema = new Store({
			name: "Main Store",
			address,
			department: [
				new Store({
					name: "Pharmacy",
					address: new PostalAddress({
						streetAddress: "123 Main Street - Unit B",
					}),
				}),
			],
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;
		const department = obj.department as Record<string, unknown>[];

		expect(department[0]?.["@type"]).toBe("Store");
		expect(department[0]?.name).toBe("Pharmacy");
	});
});
