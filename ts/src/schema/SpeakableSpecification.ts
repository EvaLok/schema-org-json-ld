import { TypedSchema } from "../TypedSchema.js";

export interface SpeakableSpecificationOptions {
	cssSelector?: string | readonly string[] | null;
	xpath?: string | readonly string[] | null;
}

export class SpeakableSpecification extends TypedSchema {
	static readonly schemaType = "SpeakableSpecification";

	public readonly cssSelector: string | readonly string[] | null;
	public readonly xpath: string | readonly string[] | null;

	constructor(options: SpeakableSpecificationOptions = {}) {
		super();
		this.cssSelector = options.cssSelector ?? null;
		this.xpath = options.xpath ?? null;
	}
}
