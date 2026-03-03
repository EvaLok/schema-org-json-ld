import { TypedSchema } from "../TypedSchema.js";

export interface RatingOptions {
	ratingValue: number;
	bestRating?: number | null;
	worstRating?: number | null;
}

export class Rating extends TypedSchema {
	static readonly schemaType = "Rating";

	public readonly ratingValue: number;
	public readonly bestRating: number | null;
	public readonly worstRating: number | null;

	constructor(options: RatingOptions) {
		super();
		this.ratingValue = options.ratingValue;
		this.bestRating = options.bestRating ?? null;
		this.worstRating = options.worstRating ?? null;
	}
}
