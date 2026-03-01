import { TypedSchema } from "../TypedSchema.js";

export class Clip extends TypedSchema {
	static readonly schemaType = "Clip";

	constructor(
		public readonly name: string,
		public readonly startOffset: number,
		public readonly url: string,
		public readonly endOffset: number | null = null,
	) {
		super();
	}
}
