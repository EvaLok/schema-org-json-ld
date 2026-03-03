import { TypedSchema } from "../TypedSchema.js";

export interface AggregateRatingOptions {
	ratingValue: number;
	bestRating?: number | null;
	worstRating?: number | null;
	ratingCount?: number | null;
	reviewCount?: number | null;
	itemReviewed?: TypedSchema | null;
}

export class AggregateRating extends TypedSchema {
	static readonly schemaType = "AggregateRating";

	public readonly ratingValue: number;
	public readonly bestRating: number | null;
	public readonly worstRating: number | null;
	public readonly ratingCount: number | null;
	public readonly reviewCount: number | null;
	public readonly itemReviewed: TypedSchema | null;

	constructor(options: AggregateRatingOptions) {
		super();
		this.ratingValue = options.ratingValue;
		this.bestRating = options.bestRating ?? null;
		this.worstRating = options.worstRating ?? null;
		this.ratingCount = options.ratingCount ?? null;
		this.reviewCount = options.reviewCount ?? null;
		this.itemReviewed = options.itemReviewed ?? null;
	}
}
