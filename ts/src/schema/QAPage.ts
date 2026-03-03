import { TypedSchema } from "../TypedSchema.js";
import type { Question } from "./Question.js";

export interface QAPageOptions {
	mainEntity: Question;
}

export class QAPage extends TypedSchema {
	static readonly schemaType = "QAPage";

	public readonly mainEntity: Question;

	constructor(options: QAPageOptions) {
		super();
		this.mainEntity = options.mainEntity;
	}
}
