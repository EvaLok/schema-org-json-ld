import { TypedSchema } from "../TypedSchema.js";

export interface BrandOptions {
	name: string;
	description?: string | null;
}

export class Brand extends TypedSchema {
	static readonly schemaType = "Brand";

	public readonly name: string;
	public readonly description: string | null;

	constructor(options: BrandOptions) {
		super();
		this.name = options.name;
		this.description = options.description ?? null;
	}
}
