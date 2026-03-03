import { TypedSchema } from "../TypedSchema.js";

export interface OccupationalExperienceRequirementsOptions {
	monthsOfExperience: number;
}

export class OccupationalExperienceRequirements extends TypedSchema {
	static readonly schemaType = "OccupationalExperienceRequirements";

	public readonly monthsOfExperience: number;

	constructor(options: OccupationalExperienceRequirementsOptions) {
		super();
		this.monthsOfExperience = options.monthsOfExperience;
	}
}
