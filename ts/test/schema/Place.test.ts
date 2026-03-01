import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { GeoCoordinates } from "../../src/schema/GeoCoordinates";
import { GeoShape } from "../../src/schema/GeoShape";
import { Place } from "../../src/schema/Place";
import { PostalAddress } from "../../src/schema/PostalAddress";

describe("Place", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Place("Googleplex");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Place");
		expect(obj.name).toBe("Googleplex");
	});

	it("omits optional fields when null", () => {
		const schema = new Place("Googleplex", null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("address");
		expect(obj).not.toHaveProperty("geo");
	});

	it("supports geo as GeoCoordinates", () => {
		const schema = new Place(
			"Googleplex",
			null,
			new GeoCoordinates(37.422, -122.084),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const geo = obj.geo as Record<string, unknown>;

		expect(geo["@type"]).toBe("GeoCoordinates");
		expect(geo.latitude).toBe(37.422);
		expect(geo.longitude).toBe(-122.084);
	});

	it("supports geo as GeoShape and includes address when set", () => {
		const schema = new Place(
			"Googleplex",
			new PostalAddress({ streetAddress: "1600 Amphitheatre Parkway" }),
			new GeoShape("37.42242 -122.08585 37.42242 -122.08585"),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const address = obj.address as Record<string, unknown>;
		const geo = obj.geo as Record<string, unknown>;

		expect(address["@type"]).toBe("PostalAddress");
		expect(address.streetAddress).toBe("1600 Amphitheatre Parkway");
		expect(geo["@type"]).toBe("GeoShape");
		expect(geo.box).toBe("37.42242 -122.08585 37.42242 -122.08585");
	});
});
