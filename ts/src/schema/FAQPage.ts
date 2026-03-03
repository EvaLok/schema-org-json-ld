import { TypedSchema } from "../TypedSchema.js";
import type { Question } from "./Question.js";

export interface FAQPageOptions {
	mainEntity: readonly Question[];
}

export class FAQPage extends TypedSchema {
	static readonly schemaType = "FAQPage";

	public readonly mainEntity: readonly Question[];

	constructor(options: FAQPageOptions) {
		super();
		this.mainEntity = options.mainEntity;
	}
}
