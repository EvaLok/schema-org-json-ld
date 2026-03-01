import { TypedSchema } from "../TypedSchema.js";

export class QuantitativeValue extends TypedSchema {
	static readonly schemaType = "QuantitativeValue";

	constructor(
		public readonly value: number | null = null,
		public readonly unitCode: string | null = null,
		public readonly minValue: number | null = null,
		public readonly maxValue: number | null = null,
	) {
		super();
	}
}
