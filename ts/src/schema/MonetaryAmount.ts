import { TypedSchema } from "../TypedSchema.js";

export class MonetaryAmount extends TypedSchema {
	static readonly schemaType = "MonetaryAmount";

	constructor(
		public readonly currency: string,
		public readonly value: number | null = null,
		public readonly minValue: number | null = null,
		public readonly maxValue: number | null = null,
		public readonly unitText: string | null = null,
	) {
		super();
	}
}
