import { TypedSchema } from "../TypedSchema.js";

export class Rating extends TypedSchema {
	static readonly schemaType = "Rating";

	constructor(
		public readonly ratingValue: number,
		public readonly bestRating: number | null = null,
		public readonly worstRating: number | null = null,
	) {
		super();
	}
}
