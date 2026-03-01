import { TypedSchema } from "../TypedSchema.js";

export class DataDownload extends TypedSchema {
	static readonly schemaType = "DataDownload";

	constructor(
		public readonly contentUrl: string,
		public readonly encodingFormat: string | null = null,
	) {
		super();
	}
}
