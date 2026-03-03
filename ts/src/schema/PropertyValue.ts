import { TypedSchema } from "../TypedSchema.js";

export interface PropertyValueOptions {
	name: string;
	value: string;
}

export class PropertyValue extends TypedSchema {
	static readonly schemaType = "PropertyValue";

	public readonly name: string;
	public readonly value: string;

	constructor(options: PropertyValueOptions) {
		super();
		this.name = options.name;
		this.value = options.value;
	}
}
