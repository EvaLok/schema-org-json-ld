import { TypedSchema } from "../TypedSchema.js";
import type { Answer } from "./Answer.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";

export interface QuestionOptions {
	name: string;
	acceptedAnswer?: Answer | null;
	suggestedAnswer?: readonly Answer[] | null;
	answerCount?: number | null;
	text?: string | null;
	upvoteCount?: number | null;
	author?: Person | Organization | null;
	datePublished?: string | null;
	dateModified?: string | null;
	eduQuestionType?: string | null;
}

export class Question extends TypedSchema {
	static readonly schemaType = "Question";

	public readonly name: string;
	public readonly acceptedAnswer: Answer | null;
	public readonly suggestedAnswer: readonly Answer[] | null;
	public readonly answerCount: number | null;
	public readonly text: string | null;
	public readonly upvoteCount: number | null;
	public readonly author: Person | Organization | null;
	public readonly datePublished: string | null;
	public readonly dateModified: string | null;
	public readonly eduQuestionType: string | null;

	constructor(options: QuestionOptions) {
		super();
		this.name = options.name;
		this.acceptedAnswer = options.acceptedAnswer ?? null;
		this.suggestedAnswer = options.suggestedAnswer ?? null;
		this.answerCount = options.answerCount ?? null;
		this.text = options.text ?? null;
		this.upvoteCount = options.upvoteCount ?? null;
		this.author = options.author ?? null;
		this.datePublished = options.datePublished ?? null;
		this.dateModified = options.dateModified ?? null;
		this.eduQuestionType = options.eduQuestionType ?? null;
	}
}
