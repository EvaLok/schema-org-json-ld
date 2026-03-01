import { TypedSchema } from "../TypedSchema.js";

export class PropertyValue extends TypedSchema {
	static readonly schemaType = "PropertyValue";

	constructor(
		public readonly name: string,
		public readonly value: string,
	) {
		super();
	}
}
