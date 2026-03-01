import { TypedSchema } from "../TypedSchema.js";

export class WebPageElement extends TypedSchema {
	static readonly schemaType = "WebPageElement";

	constructor(
		public readonly isAccessibleForFree: boolean,
		public readonly cssSelector: string,
	) {
		super();
	}
}
