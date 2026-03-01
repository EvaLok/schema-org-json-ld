import { TypedSchema } from "../TypedSchema.js";

export class AdministrativeArea extends TypedSchema {
	static readonly schemaType = "AdministrativeArea";

	constructor(public readonly name: string) {
		super();
	}
}
