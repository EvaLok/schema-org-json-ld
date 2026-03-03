import { TypedSchema } from "../TypedSchema.js";

export interface ContactPointOptions {
	telephone?: string | null;
	email?: string | null;
	contactType?: string | null;
	areaServed?: string | null;
	availableLanguage?: string | null;
}

export class ContactPoint extends TypedSchema {
	static readonly schemaType = "ContactPoint";

	public readonly telephone: string | null;
	public readonly email: string | null;
	public readonly contactType: string | null;
	public readonly areaServed: string | null;
	public readonly availableLanguage: string | null;

	constructor(options: ContactPointOptions) {
		super();
		this.telephone = options.telephone ?? null;
		this.email = options.email ?? null;
		this.contactType = options.contactType ?? null;
		this.areaServed = options.areaServed ?? null;
		this.availableLanguage = options.availableLanguage ?? null;
	}
}
