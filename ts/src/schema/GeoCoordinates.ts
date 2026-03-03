import { TypedSchema } from "../TypedSchema.js";

export interface GeoCoordinatesOptions {
	latitude: number;
	longitude: number;
}

export class GeoCoordinates extends TypedSchema {
	static readonly schemaType = "GeoCoordinates";

	public readonly latitude: number;
	public readonly longitude: number;

	constructor(options: GeoCoordinatesOptions) {
		super();
		this.latitude = options.latitude;
		this.longitude = options.longitude;
	}
}
