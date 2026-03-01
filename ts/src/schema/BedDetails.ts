import { TypedSchema } from "../TypedSchema.js";

export class BedDetails extends TypedSchema {
	static readonly schemaType = "BedDetails";

	constructor(
		public readonly numberOfBeds: number,
		public readonly typeOfBed: string | null = null,
	) {
		super();
	}
}
