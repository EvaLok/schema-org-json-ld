import { TypedSchema } from "../TypedSchema.js";

export class ContactPoint extends TypedSchema {
	static readonly schemaType = "ContactPoint";

	constructor(
		public readonly telephone: string | null = null,
		public readonly email: string | null = null,
		public readonly contactType: string | null = null,
		public readonly areaServed: string | null = null,
		public readonly availableLanguage: string | null = null,
	) {
		super();
	}
}
