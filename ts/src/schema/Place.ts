import { TypedSchema } from "../TypedSchema.js";
import type { GeoCoordinates } from "./GeoCoordinates.js";
import type { GeoShape } from "./GeoShape.js";
import type { PostalAddress } from "./PostalAddress.js";

export class Place extends TypedSchema {
	static readonly schemaType = "Place";

	constructor(
		public readonly name: string,
		public readonly address: PostalAddress | null = null,
		public readonly geo: GeoCoordinates | GeoShape | null = null,
	) {
		super();
	}
}
