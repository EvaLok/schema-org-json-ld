import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { DayOfWeek } from "../../src/enum/DayOfWeek";
import { FoodEstablishment } from "../../src/schema/FoodEstablishment";
import { OpeningHoursSpecification } from "../../src/schema/OpeningHoursSpecification";
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

	it("serializes menu and servesCuisine together", () => {
		const schema = new FoodEstablishment({
			name: "Cafe",
			address,
			menu: "https://example.com/menu",
			servesCuisine: "Italian",
		});
		const obj = JSON.parse(JsonLdGenerator.schemaToJson(schema)) as Record<
			string,
			unknown
		>;

		expect(obj.menu).toBe("https://example.com/menu");
		expect(obj.servesCuisine).toBe("Italian");
	});

	it("serializes opening hours across multiple days", () => {
		const schema = new FoodEstablishment({
			name: "Cafe",
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
});
