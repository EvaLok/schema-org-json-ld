import { TypedSchema } from "../TypedSchema.js";

export class VirtualLocation extends TypedSchema {
	static readonly schemaType = "VirtualLocation";

	constructor(
		public readonly url: string,
		public readonly name: string | null = null,
	) {
		super();
	}
}
