import { TypedSchema } from "../TypedSchema.js";

export interface AdministrativeAreaOptions {
	name: string;
}

export class AdministrativeArea extends TypedSchema {
	static readonly schemaType = "AdministrativeArea";

	public readonly name: string;

	constructor(options: AdministrativeAreaOptions) {
		super();
		this.name = options.name;
	}
}
