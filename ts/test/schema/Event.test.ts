import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { EventAttendanceModeEnumeration } from "../../src/enum/EventAttendanceModeEnumeration";
import { EventStatusType } from "../../src/enum/EventStatusType";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { Event } from "../../src/schema/Event";
import { Offer } from "../../src/schema/Offer";
import { Person } from "../../src/schema/Person";
import { Place } from "../../src/schema/Place";
import { VirtualLocation } from "../../src/schema/VirtualLocation";

describe("Event", () => {
	const offer = new Offer({
		url: "https://example.com/tickets",
		priceCurrency: "USD",
		price: 50,
		availability: ItemAvailability.InStock,
	});

	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Event({
			name: "Concert",
			startDate: "2026-04-01T20:00:00Z",
			location: new Place("City Hall"),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const location = obj.location as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Event");
		expect(obj.name).toBe("Concert");
		expect(location["@type"]).toBe("Place");
	});

	it("omits optional fields when null", () => {
		const schema = new Event({
			name: "Concert",
			startDate: "2026-04-01T20:00:00Z",
			location: new Place("City Hall"),
			description: null,
			endDate: null,
			eventAttendanceMode: null,
			eventStatus: null,
			image: null,
			offers: null,
			organizer: null,
			performer: null,
			previousStartDate: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("description");
		expect(obj).not.toHaveProperty("offers");
		expect(obj).not.toHaveProperty("performer");
	});

	it("supports location as VirtualLocation and array", () => {
		const virtualSchema = new Event({
			name: "Webinar",
			startDate: "2026-04-01T20:00:00Z",
			location: new VirtualLocation("https://example.com/live"),
		});
		const virtualObj = JSON.parse(
			JsonLdGenerator.schemaToJson(virtualSchema),
		) as Record<string, unknown>;
		const virtualLocation = virtualObj.location as Record<string, unknown>;
		expect(virtualLocation["@type"]).toBe("VirtualLocation");

		const arraySchema = new Event({
			name: "Hybrid Event",
			startDate: "2026-04-01T20:00:00Z",
			location: [
				new Place("City Hall"),
				new VirtualLocation("https://example.com/live"),
			],
		});
		const arrayObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arraySchema),
		) as Record<string, unknown>;
		const locations = arrayObj.location as Record<string, unknown>[];
		expect(locations).toHaveLength(2);
		expect(locations[0]?.["@type"]).toBe("Place");
		expect(locations[1]?.["@type"]).toBe("VirtualLocation");
	});

	it("supports offers single and array and serializes enum values", () => {
		const singleOfferSchema = new Event({
			name: "Concert",
			startDate: "2026-04-01T20:00:00Z",
			location: new Place("City Hall"),
			offers: offer,
			eventAttendanceMode:
				EventAttendanceModeEnumeration.OnlineEventAttendanceMode,
			eventStatus: EventStatusType.EventScheduled,
			performer: new Person({ name: "Alice" }),
		});
		const singleOfferObj = JSON.parse(
			JsonLdGenerator.schemaToJson(singleOfferSchema),
		) as Record<string, unknown>;
		const singleOffer = singleOfferObj.offers as Record<string, unknown>;
		expect(singleOffer["@type"]).toBe("Offer");
		expect(singleOfferObj.eventAttendanceMode).toBe(
			"https://schema.org/OnlineEventAttendanceMode",
		);
		expect(singleOfferObj.eventStatus).toBe(
			"https://schema.org/EventScheduled",
		);

		const arrayOfferSchema = new Event({
			name: "Concert",
			startDate: "2026-04-01T20:00:00Z",
			location: new Place("City Hall"),
			offers: [offer],
			performer: [new Person({ name: "Alice" })],
		});
		const arrayOfferObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arrayOfferSchema),
		) as Record<string, unknown>;
		const offers = arrayOfferObj.offers as Record<string, unknown>[];
		const performer = arrayOfferObj.performer as Record<string, unknown>[];
		expect(offers[0]?.["@type"]).toBe("Offer");
		expect(performer[0]?.["@type"]).toBe("Person");
	});
});
