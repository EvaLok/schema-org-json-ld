import { TypedSchema } from "../TypedSchema.js";

export interface EducationalOccupationalCredentialOptions {
	credentialCategory: string;
}

export class EducationalOccupationalCredential extends TypedSchema {
	static readonly schemaType = "EducationalOccupationalCredential";

	public readonly credentialCategory: string;

	constructor(options: EducationalOccupationalCredentialOptions) {
		super();
		this.credentialCategory = options.credentialCategory;
	}
}
