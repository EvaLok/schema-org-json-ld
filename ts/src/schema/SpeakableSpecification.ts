import { TypedSchema } from "../TypedSchema.js";

export class SpeakableSpecification extends TypedSchema {
	static readonly schemaType = "SpeakableSpecification";

	constructor(
		public readonly cssSelector: string | readonly string[] | null = null,
		public readonly xpath: string | readonly string[] | null = null,
	) {
		super();
	}
}
