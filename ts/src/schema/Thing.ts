import { TypedSchema } from "../TypedSchema.js";

export interface ThingOptions {
	name: string;
}

export class Thing extends TypedSchema {
	static readonly schemaType = "Thing";

	public readonly name: string;

	constructor(options: ThingOptions) {
		super();
		this.name = options.name;
	}
}
