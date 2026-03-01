import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { FoodEstablishment } from "../../src/schema/FoodEstablishment";
import { PostalAddress } from "../../src/schema/PostalAddress";

describe("FoodEstablishment", () => {
	const address = new PostalAddress({
		streetAddress: "1 Main St",
		addressLocality: "Townsville",
		addressCountry: "US",
	});

	it("produces FoodEstablishment JSON-LD with required fields", () => {
		const schema = new FoodEstablishment({ name: "Cafe", address });
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("FoodEstablishment");
		expect(obj.name).toBe("Cafe");
	});

	it("omits optional fields when null", () => {
		const schema = new FoodEstablishment({
			name: "Cafe",
			address,
			acceptsReservations: null,
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj).not.toHaveProperty("acceptsReservations");
		expect(obj).not.toHaveProperty("url");
	});

	it("supports acceptsReservations as boolean and string", () => {
		const booleanSchema = new FoodEstablishment({
			name: "Cafe",
			address,
			acceptsReservations: true,
		});
		const booleanObj = JSON.parse(
			JsonLdGenerator.schemaToJson(booleanSchema),
		) as Record<string, unknown>;
		expect(booleanObj.acceptsReservations).toBe(true);

		const stringSchema = new FoodEstablishment({
			name: "Cafe",
			address,
			acceptsReservations: "Reservations required",
		});
		const stringObj = JSON.parse(
			JsonLdGenerator.schemaToJson(stringSchema),
		) as Record<string, unknown>;
		expect(stringObj.acceptsReservations).toBe("Reservations required");
	});
});
