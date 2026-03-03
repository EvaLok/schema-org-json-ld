import { TypedSchema } from "../TypedSchema.js";

export interface PeopleAudienceOptions {
	suggestedGender?: string | null;
	suggestedMinAge?: number | null;
	suggestedMaxAge?: number | null;
}

export class PeopleAudience extends TypedSchema {
	static readonly schemaType = "PeopleAudience";

	public readonly suggestedGender: string | null;
	public readonly suggestedMinAge: number | null;
	public readonly suggestedMaxAge: number | null;

	constructor(options: PeopleAudienceOptions = {}) {
		super();
		this.suggestedGender = options.suggestedGender ?? null;
		this.suggestedMinAge = options.suggestedMinAge ?? null;
		this.suggestedMaxAge = options.suggestedMaxAge ?? null;
	}
}
