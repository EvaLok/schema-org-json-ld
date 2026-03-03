import { TypedSchema } from "../TypedSchema.js";

export interface DataDownloadOptions {
	contentUrl: string;
	encodingFormat?: string | null;
}

export class DataDownload extends TypedSchema {
	static readonly schemaType = "DataDownload";

	public readonly contentUrl: string;
	public readonly encodingFormat: string | null;

	constructor(options: DataDownloadOptions) {
		super();
		this.contentUrl = options.contentUrl;
		this.encodingFormat = options.encodingFormat ?? null;
	}
}
