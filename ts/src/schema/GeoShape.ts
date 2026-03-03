import { TypedSchema } from "../TypedSchema.js";

export interface GeoShapeOptions {
	box?: string | null;
}

export class GeoShape extends TypedSchema {
	static readonly schemaType = "GeoShape";

	public readonly box: string | null;

	constructor(options: GeoShapeOptions) {
		super();
		this.box = options.box ?? null;
	}
}
