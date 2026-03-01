import { TypedSchema } from "../TypedSchema.js";

export class GeoShape extends TypedSchema {
	static readonly schemaType = "GeoShape";

	constructor(public readonly box: string | null = null) {
		super();
	}
}
