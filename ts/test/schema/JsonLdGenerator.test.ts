import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { TypedSchema } from "../../src/TypedSchema";
import { MerchantReturnEnumeration } from "../../src/enum/MerchantReturnEnumeration";
import { Article } from "../../src/schema/Article";
import { Brand } from "../../src/schema/Brand";
import { MerchantReturnPolicy } from "../../src/schema/MerchantReturnPolicy";
import { MerchantReturnPolicySeasonalOverride } from "../../src/schema/MerchantReturnPolicySeasonalOverride";
import { Organization } from "../../src/schema/Organization";

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

class MultiTypeSchema extends TypedSchema {
	static readonly schemaType = ["Type1", "Type2"];

	constructor(public readonly name: string) {
		super();
	}
}

class MixedArraySchema extends TypedSchema {
	static readonly schemaType = "MixedArrayType";

	constructor(public readonly values: readonly (string | number | boolean)[]) {
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

	it("serializes special characters and UTF-8 values", () => {
		const json = JsonLdGenerator.schemaToJson(
			new Brand({
				name: 'Café "Élite" & Friends <Best> 😊',
				description: "AT&T &amp; snowman ☃",
			}),
		);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.name).toBe('Café "Élite" & Friends <Best> 😊');
		expect(obj.description).toBe("AT&T &amp; snowman ☃");
	});

	it("serializes deeply nested schema graphs with @graph", () => {
		const json = JsonLdGenerator.schemasToJson(
			new Article({ headline: "Deep graph article" }),
			new Organization({
				name: "ACME Corp",
				hasMerchantReturnPolicy: [
					new MerchantReturnPolicy({
						applicableCountry: ["US", "CA"],
						returnPolicyCategory:
							MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
						merchantReturnDays: 30,
						returnPolicySeasonalOverride: [
							new MerchantReturnPolicySeasonalOverride({
								startDate: "2026-11-01",
								endDate: "2026-12-31",
								returnPolicyCategory:
									MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
								merchantReturnDays: 60,
							}),
						],
					}),
				],
			}),
		);
		const obj = JSON.parse(json) as {
			"@graph": Array<Record<string, unknown>>;
		};

		expect(obj["@graph"][0]["@type"]).toBe("Article");
		expect(obj["@graph"][1]["@type"]).toBe("Organization");
		expect(
			(
				obj["@graph"][1].hasMerchantReturnPolicy as Array<
					Record<string, unknown>
				>
			)[0]["@type"],
		).toBe("MerchantReturnPolicy");
		expect(
			(
				(
					obj["@graph"][1].hasMerchantReturnPolicy as Array<
						Record<string, unknown>
					>
				)[0].returnPolicySeasonalOverride as Array<Record<string, unknown>>
			)[0]["@type"],
		).toBe("MerchantReturnPolicySeasonalOverride");
	});

	it("supports array @type values", () => {
		const json = JsonLdGenerator.schemaToJson(
			new MultiTypeSchema("Test Schema"),
		);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@type"]).toEqual(["Type1", "Type2"]);
		expect(obj.name).toBe("Test Schema");
	});

	it("handles schemasToJson with single and three schemas", () => {
		const single = JSON.parse(
			JsonLdGenerator.schemasToJson(new Article({ headline: "Single schema" })),
		) as {
			"@graph": Array<Record<string, unknown>>;
		};
		const three = JSON.parse(
			JsonLdGenerator.schemasToJson(
				new Article({ headline: "One" }),
				new Brand({ name: "Two" }),
				new Organization({ name: "Three" }),
			),
		) as {
			"@graph": Array<Record<string, unknown>>;
		};

		expect(single["@graph"]).toHaveLength(1);
		expect(three["@graph"]).toHaveLength(3);
		expect(three["@graph"][0]).not.toHaveProperty("@context");
		expect(three["@graph"][1]).not.toHaveProperty("@context");
		expect(three["@graph"][2]).not.toHaveProperty("@context");
	});

	it("exposes schemaToObject with and without initial @context", () => {
		const withContext = JsonLdGenerator.schemaToObject(
			new Brand({ name: "Object API" }),
		);
		const withoutContext = JsonLdGenerator.schemaToObject(
			new Brand({ name: "Object API" }),
			false,
		);

		expect(withContext["@context"]).toBe("https://schema.org/");
		expect(withContext["@type"]).toBe("Brand");
		expect(withContext.name).toBe("Object API");
		expect(withoutContext).not.toHaveProperty("@context");
		expect(withoutContext["@type"]).toBe("Brand");
	});

	it("serializes mixed scalar arrays without type loss", () => {
		const json = JsonLdGenerator.schemaToJson(
			new MixedArraySchema(["CA", 90210, true]),
		);
		const obj = JSON.parse(json) as {
			values: Array<string | number | boolean>;
		};

		expect(obj.values).toEqual(["CA", 90210, true]);
	});

	it("keeps single-item and multi-item arrays as arrays", () => {
		const single = JSON.parse(
			JsonLdGenerator.schemaToJson(new ExampleSchema("Single", null, ["one"])),
		) as {
			tags: string[];
		};
		const multiple = JSON.parse(
			JsonLdGenerator.schemaToJson(
				new ExampleSchema("Multiple", null, ["one", "two"]),
			),
		) as {
			tags: string[];
		};

		expect(single.tags).toEqual(["one"]);
		expect(multiple.tags).toEqual(["one", "two"]);
	});

	it("does not require propertyMap for normal schemas", () => {
		const json = JsonLdGenerator.schemaToJson(
			new Brand({ name: "Test Brand", description: "Test Description" }),
		);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@type"]).toBe("Brand");
		expect(obj.name).toBe("Test Brand");
		expect(obj.description).toBe("Test Description");
	});
});
