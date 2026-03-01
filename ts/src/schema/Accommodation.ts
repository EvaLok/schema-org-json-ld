import { TypedSchema } from "../TypedSchema.js";
import type { BedDetails } from "./BedDetails.js";
import type { LocationFeatureSpecification } from "./LocationFeatureSpecification.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export interface AccommodationOptions {
	occupancy: QuantitativeValue;
	additionalType?: string | null;
	numberOfBedrooms?: number | null;
	numberOfBathroomsTotal?: number | null;
	numberOfRooms?: number | null;
	floorSize?: QuantitativeValue | null;
	bed?: readonly BedDetails[] | null;
	amenityFeature?: readonly LocationFeatureSpecification[] | null;
}

export class Accommodation extends TypedSchema {
	static readonly schemaType = "Accommodation";

	public readonly occupancy: QuantitativeValue;
	public readonly additionalType: string | null;
	public readonly numberOfBedrooms: number | null;
	public readonly numberOfBathroomsTotal: number | null;
	public readonly numberOfRooms: number | null;
	public readonly floorSize: QuantitativeValue | null;
	public readonly bed: readonly BedDetails[] | null;
	public readonly amenityFeature:
		| readonly LocationFeatureSpecification[]
		| null;

	constructor(options: AccommodationOptions) {
		super();
		this.occupancy = options.occupancy;
		this.additionalType = options.additionalType ?? null;
		this.numberOfBedrooms = options.numberOfBedrooms ?? null;
		this.numberOfBathroomsTotal = options.numberOfBathroomsTotal ?? null;
		this.numberOfRooms = options.numberOfRooms ?? null;
		this.floorSize = options.floorSize ?? null;
		this.bed = options.bed ?? null;
		this.amenityFeature = options.amenityFeature ?? null;
	}
}
