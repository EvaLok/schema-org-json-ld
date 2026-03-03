import { TypedSchema } from "../TypedSchema.js";

export interface BedDetailsOptions {
	numberOfBeds: number;
	typeOfBed?: string | null;
}

export class BedDetails extends TypedSchema {
	static readonly schemaType = "BedDetails";

	public readonly numberOfBeds: number;
	public readonly typeOfBed: string | null;

	constructor(options: BedDetailsOptions) {
		super();
		this.numberOfBeds = options.numberOfBeds;
		this.typeOfBed = options.typeOfBed ?? null;
	}
}
