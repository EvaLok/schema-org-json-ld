import { TypedSchema } from "../TypedSchema.js";
import type { Question } from "./Question.js";

export class FAQPage extends TypedSchema {
	static readonly schemaType = "FAQPage";

	constructor(public readonly mainEntity: readonly Question[]) {
		super();
	}
}
