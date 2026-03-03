import { TypedSchema } from "../TypedSchema.js";
import type { GeoCoordinates } from "./GeoCoordinates.js";
import type { GeoShape } from "./GeoShape.js";
import type { PostalAddress } from "./PostalAddress.js";

export interface PlaceOptions {
	name: string;
	address?: PostalAddress | null;
	geo?: GeoCoordinates | GeoShape | null;
}

export class Place extends TypedSchema {
	static readonly schemaType = "Place";

	public readonly name: string;
	public readonly address: PostalAddress | null;
	public readonly geo: GeoCoordinates | GeoShape | null;

	constructor(options: PlaceOptions) {
		super();
		this.name = options.name;
		this.address = options.address ?? null;
		this.geo = options.geo ?? null;
	}
}
