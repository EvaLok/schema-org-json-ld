import { TypedSchema } from "../TypedSchema.js";
import type { ItemList } from "./ItemList.js";
import type { Organization } from "./Organization.js";
import type { Person } from "./Person.js";
import type { Rating } from "./Rating.js";

export interface ReviewOptions {
	author: string | Person | Organization;
	reviewRating: Rating;
	reviewBody?: string | null;
	datePublished?: string | null;
	name?: string | null;
	itemReviewed?: TypedSchema | null;
	positiveNotes?: ItemList | null;
	negativeNotes?: ItemList | null;
	contentReferenceTime?: string | null;
}

export class Review extends TypedSchema {
	static readonly schemaType = "Review";

	public readonly author: string | Person | Organization;
	public readonly reviewRating: Rating;
	public readonly reviewBody: string | null;
	public readonly datePublished: string | null;
	public readonly name: string | null;
	public readonly itemReviewed: TypedSchema | null;
	public readonly positiveNotes: ItemList | null;
	public readonly negativeNotes: ItemList | null;
	public readonly contentReferenceTime: string | null;

	constructor(options: ReviewOptions) {
		super();
		this.author = options.author;
		this.reviewRating = options.reviewRating;
		this.reviewBody = options.reviewBody ?? null;
		this.datePublished = options.datePublished ?? null;
		this.name = options.name ?? null;
		this.itemReviewed = options.itemReviewed ?? null;
		this.positiveNotes = options.positiveNotes ?? null;
		this.negativeNotes = options.negativeNotes ?? null;
		this.contentReferenceTime = options.contentReferenceTime ?? null;
	}
}
