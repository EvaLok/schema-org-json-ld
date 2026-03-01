import { TypedSchema } from "../TypedSchema.js";

export interface PostalAddressOptions {
	streetAddress?: string | null;
	addressLocality?: string | null;
	addressRegion?: string | null;
	postalCode?: string | null;
	addressCountry?: string | null;
	postOfficeBoxNumber?: string | null;
}

export class PostalAddress extends TypedSchema {
	static readonly schemaType = "PostalAddress";

	public readonly streetAddress: string | null;
	public readonly addressLocality: string | null;
	public readonly addressRegion: string | null;
	public readonly postalCode: string | null;
	public readonly addressCountry: string | null;
	public readonly postOfficeBoxNumber: string | null;

	constructor(options: PostalAddressOptions) {
		super();
		this.streetAddress = options.streetAddress ?? null;
		this.addressLocality = options.addressLocality ?? null;
		this.addressRegion = options.addressRegion ?? null;
		this.postalCode = options.postalCode ?? null;
		this.addressCountry = options.addressCountry ?? null;
		this.postOfficeBoxNumber = options.postOfficeBoxNumber ?? null;
	}
}
