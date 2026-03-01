import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";

export class Answer extends TypedSchema {
	static readonly schemaType = "Answer";

	constructor(
		public readonly text: string,
		public readonly author: Person | Organization | null = null,
		public readonly url: string | null = null,
		public readonly upvoteCount: number | null = null,
		public readonly datePublished: string | null = null,
		public readonly dateModified: string | null = null,
	) {
		super();
	}
}
