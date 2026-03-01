import { TypedSchema } from "./TypedSchema.js";

type SchemaClass = typeof TypedSchema & {
	readonly propertyMap?: Record<string, string>;
};

export class JsonLdGenerator {
	static schemaToJson(schema: TypedSchema): string {
		return JSON.stringify(JsonLdGenerator.schemaToObject(schema), null, 2);
	}

	static schemasToJson(...schemas: TypedSchema[]): string {
		return JSON.stringify(
			{
				"@context": "https://schema.org/",
				"@graph": schemas.map((schema) =>
					JsonLdGenerator.schemaToObject(schema, false),
				),
			},
			null,
			2,
		);
	}

	static schemaToObject(
		schema: TypedSchema,
		initialContext = true,
	): Record<string, unknown> {
		const object: Record<string, unknown> = {};
		const schemaClass = schema.constructor as SchemaClass;
		const properties = { ...(schema as Record<string, unknown>) };

		if (initialContext) {
			object["@context"] = "https://schema.org/";
		}

		object["@type"] = schemaClass.schemaType;

		if (schemaClass.propertyMap !== undefined) {
			for (const [sourceName, targetName] of Object.entries(
				schemaClass.propertyMap,
			)) {
				if (sourceName in properties) {
					properties[targetName] = properties[sourceName];
					delete properties[sourceName];
				}
			}
		}

		for (const [key, value] of Object.entries(properties)) {
			const serializedValue = JsonLdGenerator.serializeValue(value);
			if (serializedValue !== undefined) {
				object[key] = serializedValue;
			}
		}

		return object;
	}

	private static serializeValue(value: unknown): unknown {
		if (value === null || value === undefined) {
			return undefined;
		}

		if (
			typeof value === "string" ||
			typeof value === "number" ||
			typeof value === "boolean"
		) {
			return value;
		}

		if (value instanceof TypedSchema) {
			return JsonLdGenerator.schemaToObject(value, false);
		}

		if (Array.isArray(value)) {
			if (value.length === 0) {
				return undefined;
			}

			const mappedValues = value
				.map((entry) => JsonLdGenerator.serializeValue(entry))
				.filter(
					(entry): entry is NonNullable<typeof entry> => entry !== undefined,
				);

			return mappedValues.length > 0 ? mappedValues : undefined;
		}

		return undefined;
	}
}
