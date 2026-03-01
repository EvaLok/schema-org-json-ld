import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { Rating } from "./Rating.js";

export class Review extends TypedSchema {
	static readonly schemaType = "Review";

	constructor(
		public readonly author: string | Person | Organization,
		public readonly reviewRating: Rating,
		public readonly reviewBody: string | null = null,
		public readonly datePublished: string | null = null,
		public readonly name: string | null = null,
		public readonly itemReviewed: TypedSchema | null = null,
	) {
		super();
	}
}
