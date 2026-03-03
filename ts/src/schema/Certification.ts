import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Rating } from "./Rating.js";

export interface CertificationOptions {
	name: string;
	issuedBy: Organization;
	certificationIdentification?: string | null;
	certificationRating?: Rating | null;
}

export class Certification extends TypedSchema {
	static readonly schemaType = "Certification";

	public readonly name: string;
	public readonly issuedBy: Organization;
	public readonly certificationIdentification: string | null;
	public readonly certificationRating: Rating | null;

	constructor(options: CertificationOptions) {
		super();
		this.name = options.name;
		this.issuedBy = options.issuedBy;
		this.certificationIdentification =
			options.certificationIdentification ?? null;
		this.certificationRating = options.certificationRating ?? null;
	}
}
