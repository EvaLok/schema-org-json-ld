import { TypedSchema } from "../TypedSchema.js";

export class Thing extends TypedSchema {
	static readonly schemaType = "Thing";

	constructor(public readonly name: string) {
		super();
	}
}
