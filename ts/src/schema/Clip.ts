import { TypedSchema } from "../TypedSchema.js";

export interface ClipOptions {
	name: string;
	startOffset: number;
	url: string;
	endOffset?: number | null;
}

export class Clip extends TypedSchema {
	static readonly schemaType = "Clip";

	public readonly name: string;
	public readonly startOffset: number;
	public readonly url: string;
	public readonly endOffset: number | null;

	constructor(options: ClipOptions) {
		super();
		this.name = options.name;
		this.startOffset = options.startOffset;
		this.url = options.url;
		this.endOffset = options.endOffset ?? null;
	}
}
