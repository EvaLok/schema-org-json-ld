import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { Organization } from "../../src/schema/Organization";
import { Person } from "../../src/schema/Person";
import { ProfilePage } from "../../src/schema/ProfilePage";

describe("ProfilePage", () => {
	it("produces minimal JSON-LD output with Person as mainEntity", () => {
		const schema = new ProfilePage(new Person({ name: "Jane Doe" }));
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const mainEntity = obj.mainEntity as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ProfilePage");
		expect(mainEntity["@type"]).toBe("Person");
	});

	it("omits optional fields when null", () => {
		const schema = new ProfilePage(
			new Person({ name: "Jane Doe" }),
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("dateCreated");
		expect(obj).not.toHaveProperty("dateModified");
	});

	it("includes all fields and supports Organization as mainEntity", () => {
		const schema = new ProfilePage(
			new Organization({ name: "Example Org" }),
			"2026-02-01",
			"2026-03-01",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const mainEntity = obj.mainEntity as Record<string, unknown>;

		expect(mainEntity["@type"]).toBe("Organization");
		expect(obj.dateCreated).toBe("2026-02-01");
		expect(obj.dateModified).toBe("2026-03-01");
	});
});
