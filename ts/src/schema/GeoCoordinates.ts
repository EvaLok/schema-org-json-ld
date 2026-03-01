import { TypedSchema } from "../TypedSchema.js";

export class GeoCoordinates extends TypedSchema {
	static readonly schemaType = "GeoCoordinates";

	constructor(
		public readonly latitude: number,
		public readonly longitude: number,
	) {
		super();
	}
}
