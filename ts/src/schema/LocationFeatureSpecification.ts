import { TypedSchema } from "../TypedSchema.js";

export interface LocationFeatureSpecificationOptions {
	name: string;
	value: boolean | string;
}

export class LocationFeatureSpecification extends TypedSchema {
	static readonly schemaType = "LocationFeatureSpecification";

	public readonly name: string;
	public readonly value: boolean | string;

	constructor(options: LocationFeatureSpecificationOptions) {
		super();
		this.name = options.name;
		this.value = options.value;
	}
}
