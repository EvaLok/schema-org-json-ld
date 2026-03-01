import { TypedSchema } from "../TypedSchema.js";

export class SizeSpecification extends TypedSchema {
	static readonly schemaType = "SizeSpecification";

	constructor(
		public readonly name: string,
		public readonly sizeGroup: string | null = null,
		public readonly sizeSystem: string | null = null,
	) {
		super();
	}
}
