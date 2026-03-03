import { TypedSchema } from "../TypedSchema.js";

export interface SizeSpecificationOptions {
	name: string;
	sizeGroup?: string | null;
	sizeSystem?: string | null;
}

export class SizeSpecification extends TypedSchema {
	static readonly schemaType = "SizeSpecification";

	public readonly name: string;
	public readonly sizeGroup: string | null;
	public readonly sizeSystem: string | null;

	constructor(options: SizeSpecificationOptions) {
		super();
		this.name = options.name;
		this.sizeGroup = options.sizeGroup ?? null;
		this.sizeSystem = options.sizeSystem ?? null;
	}
}
