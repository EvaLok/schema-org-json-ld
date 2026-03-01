import { TypedSchema } from "../TypedSchema.js";

export class Brand extends TypedSchema {
	static readonly schemaType = "Brand";

	constructor(
		public readonly name: string,
		public readonly description: string | null = null,
	) {
		super();
	}
}
