import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";

export interface EmployerAggregateRatingOptions {
	itemReviewed: Organization;
	ratingValue: number;
	ratingCount?: number | null;
	reviewCount?: number | null;
	bestRating?: number | null;
	worstRating?: number | null;
}

export class EmployerAggregateRating extends TypedSchema {
	static readonly schemaType = "EmployerAggregateRating";

	public readonly itemReviewed: Organization;
	public readonly ratingValue: number;
	public readonly ratingCount: number | null;
	public readonly reviewCount: number | null;
	public readonly bestRating: number | null;
	public readonly worstRating: number | null;

	constructor(options: EmployerAggregateRatingOptions) {
		super();
		this.itemReviewed = options.itemReviewed;
		this.ratingValue = options.ratingValue;
		this.ratingCount = options.ratingCount ?? null;
		this.reviewCount = options.reviewCount ?? null;
		this.bestRating = options.bestRating ?? null;
		this.worstRating = options.worstRating ?? null;
	}
}
