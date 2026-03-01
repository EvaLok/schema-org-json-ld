import { TypedSchema } from "../TypedSchema.js";

export class AggregateOffer extends TypedSchema {
	static readonly schemaType = "AggregateOffer";

	constructor(
		public readonly lowPrice: number,
		public readonly priceCurrency: string,
		public readonly highPrice: number | null = null,
		public readonly offerCount: number | null = null,
	) {
		super();
	}
}
