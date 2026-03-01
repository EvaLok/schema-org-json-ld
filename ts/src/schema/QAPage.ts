import { TypedSchema } from "../TypedSchema.js";
import type { Question } from "./Question.js";

export class QAPage extends TypedSchema {
	static readonly schemaType = "QAPage";

	constructor(public readonly mainEntity: Question) {
		super();
	}
}
