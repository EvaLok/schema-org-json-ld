import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Accommodation } from "../../src/schema/Accommodation";
import { BedDetails } from "../../src/schema/BedDetails";
import { LocationFeatureSpecification } from "../../src/schema/LocationFeatureSpecification";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";

describe("Accommodation", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Accommodation({
			occupancy: new QuantitativeValue(2, "C62"),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const occupancy = obj.occupancy as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Accommodation");
		expect(occupancy["@type"]).toBe("QuantitativeValue");
		expect(occupancy.value).toBe(2);
		expect(occupancy.unitCode).toBe("C62");
	});

	it("omits optional fields when null", () => {
		const schema = new Accommodation({
			occupancy: new QuantitativeValue(2, "C62"),
			additionalType: null,
			numberOfBedrooms: null,
			numberOfBathroomsTotal: null,
			numberOfRooms: null,
			floorSize: null,
			bed: null,
			amenityFeature: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("additionalType");
		expect(obj).not.toHaveProperty("numberOfBedrooms");
		expect(obj).not.toHaveProperty("numberOfBathroomsTotal");
		expect(obj).not.toHaveProperty("numberOfRooms");
		expect(obj).not.toHaveProperty("floorSize");
		expect(obj).not.toHaveProperty("bed");
		expect(obj).not.toHaveProperty("amenityFeature");
	});

	it("supports partial options object", () => {
		const schema = new Accommodation({
			occupancy: new QuantitativeValue(2, "C62"),
			numberOfRooms: 3,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.numberOfRooms).toBe(3);
		expect(obj).not.toHaveProperty("floorSize");
		expect(obj).not.toHaveProperty("bed");
	});

	it("includes all fields when set", () => {
		const schema = new Accommodation({
			occupancy: new QuantitativeValue(2, "C62"),
			additionalType: "https://example.com/types/loft",
			numberOfBedrooms: 1,
			numberOfBathroomsTotal: 1,
			numberOfRooms: 2,
			floorSize: new QuantitativeValue(45, "MTK"),
			bed: [new BedDetails(1, "Queen"), new BedDetails(1, "Sofa bed")],
			amenityFeature: [
				new LocationFeatureSpecification("WiFi", true),
				new LocationFeatureSpecification("Parking", false),
			],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const floorSize = obj.floorSize as Record<string, unknown>;
		const bed = obj.bed as Record<string, unknown>[];
		const amenityFeature = obj.amenityFeature as Record<string, unknown>[];

		expect(obj.additionalType).toBe("https://example.com/types/loft");
		expect(obj.numberOfBedrooms).toBe(1);
		expect(obj.numberOfBathroomsTotal).toBe(1);
		expect(obj.numberOfRooms).toBe(2);
		expect(floorSize["@type"]).toBe("QuantitativeValue");
		expect(floorSize.value).toBe(45);
		expect(bed).toHaveLength(2);
		expect(bed[0]?.["@type"]).toBe("BedDetails");
		expect(amenityFeature).toHaveLength(2);
		expect(amenityFeature[0]?.["@type"]).toBe("LocationFeatureSpecification");
	});
});
