import { TypedSchema } from "../TypedSchema.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";

export interface AnswerOptions {
	text: string;
	author?: Person | Organization | null;
	url?: string | null;
	upvoteCount?: number | null;
	datePublished?: string | null;
	dateModified?: string | null;
}

export class Answer extends TypedSchema {
	static readonly schemaType = "Answer";

	public readonly text: string;
	public readonly author: Person | Organization | null;
	public readonly url: string | null;
	public readonly upvoteCount: number | null;
	public readonly datePublished: string | null;
	public readonly dateModified: string | null;

	constructor(options: AnswerOptions) {
		super();
		this.text = options.text;
		this.author = options.author ?? null;
		this.url = options.url ?? null;
		this.upvoteCount = options.upvoteCount ?? null;
		this.datePublished = options.datePublished ?? null;
		this.dateModified = options.dateModified ?? null;
	}
}
