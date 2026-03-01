import { TypedSchema } from "../TypedSchema.js";

export class DataCatalog extends TypedSchema {
	static readonly schemaType = "DataCatalog";

	constructor(public readonly name: string) {
		super();
	}
}
