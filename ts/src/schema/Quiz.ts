import { TypedSchema } from "../TypedSchema.js";
import type { AlignmentObject } from "./AlignmentObject.js";
import type { Question } from "./Question.js";

export class Quiz extends TypedSchema {
	static readonly schemaType = "Quiz";

	constructor(
		public readonly hasPart: readonly Question[],
		public readonly about: string | null = null,
		public readonly educationalAlignment: AlignmentObject | null = null,
		public readonly name: string | null = null,
		public readonly description: string | null = null,
	) {
		super();
	}
}
