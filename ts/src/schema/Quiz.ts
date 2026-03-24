import { TypedSchema } from "../TypedSchema.js";
import type { AlignmentObject } from "./AlignmentObject.js";
import type { Question } from "./Question.js";
import type { Thing } from "./Thing.js";

export interface QuizOptions {
	hasPart: readonly Question[];
	about?: string | Thing | null;
	educationalAlignment?: AlignmentObject | readonly AlignmentObject[] | null;
	name?: string | null;
	description?: string | null;
}

export class Quiz extends TypedSchema {
	static readonly schemaType = "Quiz";

	public readonly hasPart: readonly Question[];
	public readonly about: string | Thing | null;
	public readonly educationalAlignment:
		| AlignmentObject
		| readonly AlignmentObject[]
		| null;
	public readonly name: string | null;
	public readonly description: string | null;

	constructor(options: QuizOptions) {
		super();
		this.hasPart = options.hasPart;
		this.about = options.about ?? null;
		this.educationalAlignment = options.educationalAlignment ?? null;
		this.name = options.name ?? null;
		this.description = options.description ?? null;
	}
}
