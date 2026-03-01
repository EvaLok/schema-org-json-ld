import { TypedSchema } from "../TypedSchema.js";

export class PeopleAudience extends TypedSchema {
	static readonly schemaType = "PeopleAudience";

	constructor(
		public readonly suggestedGender: string | null = null,
		public readonly suggestedMinAge: number | null = null,
		public readonly suggestedMaxAge: number | null = null,
	) {
		super();
	}
}
