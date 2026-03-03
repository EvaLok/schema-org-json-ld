import { TypedSchema } from "../TypedSchema.js";

export interface DataCatalogOptions {
	name: string;
}

export class DataCatalog extends TypedSchema {
	static readonly schemaType = "DataCatalog";

	public readonly name: string;

	constructor(options: DataCatalogOptions) {
		super();
		this.name = options.name;
	}
}
