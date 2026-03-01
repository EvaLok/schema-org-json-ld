import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Rating } from "./Rating.js";

export class Certification extends TypedSchema {
	static readonly schemaType = "Certification";

	constructor(
		public readonly name: string,
		public readonly issuedBy: Organization,
		public readonly certificationIdentification: string | null = null,
		public readonly certificationRating: Rating | null = null,
	) {
		super();
	}
}
