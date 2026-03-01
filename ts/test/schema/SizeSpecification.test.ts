import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { SizeSpecification } from "../../src/schema/SizeSpecification";

describe("SizeSpecification", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new SizeSpecification("Small");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("SizeSpecification");
		expect(obj.name).toBe("Small");
	});

	it("omits optional fields when null", () => {
		const schema = new SizeSpecification("Small", null, null);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("sizeGroup");
		expect(obj).not.toHaveProperty("sizeSystem");
	});

	it("includes all fields when set", () => {
		const schema = new SizeSpecification("Medium", "Adult", "US");
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.sizeGroup).toBe("Adult");
		expect(obj.sizeSystem).toBe("US");
	});
});
