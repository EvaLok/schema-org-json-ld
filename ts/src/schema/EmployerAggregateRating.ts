import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";

export class EmployerAggregateRating extends TypedSchema {
	static readonly schemaType = "EmployerAggregateRating";

	constructor(
		public readonly itemReviewed: Organization,
		public readonly ratingValue: number,
		public readonly ratingCount: number | null = null,
		public readonly reviewCount: number | null = null,
		public readonly bestRating: number | null = null,
		public readonly worstRating: number | null = null,
	) {
		super();
	}
}
