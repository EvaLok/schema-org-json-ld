import { TypedSchema } from "../TypedSchema.js";

export interface WebPageElementOptions {
	isAccessibleForFree: boolean;
	cssSelector: string;
}

export class WebPageElement extends TypedSchema {
	static readonly schemaType = "WebPageElement";

	public readonly isAccessibleForFree: boolean;
	public readonly cssSelector: string;

	constructor(options: WebPageElementOptions) {
		super();
		this.isAccessibleForFree = options.isAccessibleForFree;
		this.cssSelector = options.cssSelector;
	}
}
