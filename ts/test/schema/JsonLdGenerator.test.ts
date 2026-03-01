import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { TypedSchema } from "../../src/TypedSchema";

class ExampleSchema extends TypedSchema {
	static readonly schemaType = "ExampleType";

	constructor(
		public readonly name: string,
		public readonly optional: string | null = null,
		public readonly tags: readonly string[] = [],
		public readonly nested: TypedSchema | null = null,
		public readonly numeric: number | undefined = undefined,
	) {
		super();
	}
}

class MappedSchema extends TypedSchema {
	static readonly schemaType = "MappedType";
	static readonly propertyMap: Record<string, string> = {
		mappedValue: "mapped-value",
	};

	constructor(public readonly mappedValue: string) {
		super();
	}
}

describe("JsonLdGenerator", () => {
	it("serializes a single schema", () => {
		const json = JsonLdGenerator.schemaToJson(new ExampleSchema("ACME"));
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ExampleType");
		expect(obj.name).toBe("ACME");
	});

	it("serializes multiple schemas using @graph", () => {
		const json = JsonLdGenerator.schemasToJson(
			new ExampleSchema("First"),
			new ExampleSchema("Second"),
		);
		const obj = JSON.parse(json) as {
			"@context": string;
			"@graph": Array<Record<string, unknown>>;
		};

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@graph"]).toHaveLength(2);
		expect(obj["@graph"][0]["@type"]).toBe("ExampleType");
		expect(obj["@graph"][1].name).toBe("Second");
	});

	it("omits null and undefined properties", () => {
		const json = JsonLdGenerator.schemaToJson(
			new ExampleSchema("ACME", null, [], null, undefined),
		);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("optional");
		expect(obj).not.toHaveProperty("numeric");
	});

	it("omits empty arrays", () => {
		const json = JsonLdGenerator.schemaToJson(new ExampleSchema("ACME"));
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("tags");
	});

	it("serializes nested TypedSchema values without nested @context", () => {
		const json = JsonLdGenerator.schemaToJson(
			new ExampleSchema("Parent", null, [], new ExampleSchema("Child")),
		);
		const obj = JSON.parse(json) as {
			nested: Record<string, unknown>;
		};

		expect(obj.nested["@type"]).toBe("ExampleType");
		expect(obj.nested.name).toBe("Child");
		expect(obj.nested).not.toHaveProperty("@context");
	});

	it("applies propertyMap remapping", () => {
		const json = JsonLdGenerator.schemaToJson(new MappedSchema("value"));
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["mapped-value"]).toBe("value");
		expect(obj).not.toHaveProperty("mappedValue");
	});

	it("returns valid JSON output", () => {
		const json = JsonLdGenerator.schemaToJson(new ExampleSchema("ACME"));

		expect(() => JSON.parse(json)).not.toThrow();
	});
});
