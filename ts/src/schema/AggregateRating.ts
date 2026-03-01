import { TypedSchema } from "../TypedSchema.js";

export class AggregateRating extends TypedSchema {
	static readonly schemaType = "AggregateRating";

	constructor(
		public readonly ratingValue: number,
		public readonly bestRating: number | null = null,
		public readonly worstRating: number | null = null,
		public readonly ratingCount: number | null = null,
		public readonly reviewCount: number | null = null,
	) {
		super();
	}
}
