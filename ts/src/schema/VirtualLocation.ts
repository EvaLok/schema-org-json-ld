import { TypedSchema } from "../TypedSchema.js";

export interface VirtualLocationOptions {
	url: string;
	name?: string | null;
}

export class VirtualLocation extends TypedSchema {
	static readonly schemaType = "VirtualLocation";

	public readonly url: string;
	public readonly name: string | null;

	constructor(options: VirtualLocationOptions) {
		super();
		this.url = options.url;
		this.name = options.name ?? null;
	}
}
