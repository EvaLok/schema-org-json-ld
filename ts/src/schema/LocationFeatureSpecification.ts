import { TypedSchema } from "../TypedSchema.js";

export class LocationFeatureSpecification extends TypedSchema {
	static readonly schemaType = "LocationFeatureSpecification";

	constructor(
		public readonly name: string,
		public readonly value: boolean | string,
	) {
		super();
	}
}
