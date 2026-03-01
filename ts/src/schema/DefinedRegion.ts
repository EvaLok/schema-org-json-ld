import { TypedSchema } from "../TypedSchema.js";

export class DefinedRegion extends TypedSchema {
	static readonly schemaType = "DefinedRegion";

	constructor(
		public readonly addressCountry: string,
		public readonly addressRegion: string | readonly string[] | null = null,
		public readonly postalCode: string | null = null,
	) {
		super();
	}
}
