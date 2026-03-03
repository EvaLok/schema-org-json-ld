import { TypedSchema } from "../TypedSchema.js";

export interface DefinedRegionOptions {
	addressCountry: string;
	addressRegion?: string | readonly string[] | null;
	postalCode?: string | null;
}

export class DefinedRegion extends TypedSchema {
	static readonly schemaType = "DefinedRegion";

	public readonly addressCountry: string;
	public readonly addressRegion: string | readonly string[] | null;
	public readonly postalCode: string | null;

	constructor(options: DefinedRegionOptions) {
		super();
		this.addressCountry = options.addressCountry;
		this.addressRegion = options.addressRegion ?? null;
		this.postalCode = options.postalCode ?? null;
	}
}
