import { TypedSchema } from "../TypedSchema.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { Person } from "./Person.js";
import type { Review } from "./Review.js";

export interface MovieOptions {
	name: string;
	image: string;
	aggregateRating?: AggregateRating | null;
	dateCreated?: string | null;
	datePublished?: string | null;
	director?: Person | null;
	review?: Review | null;
	description?: string | null;
	actor?: readonly Person[] | null;
}

export class Movie extends TypedSchema {
	static readonly schemaType = "Movie";

	public readonly name: string;
	public readonly image: string;
	public readonly aggregateRating: AggregateRating | null;
	public readonly dateCreated: string | null;
	public readonly datePublished: string | null;
	public readonly director: Person | null;
	public readonly review: Review | null;
	public readonly description: string | null;
	public readonly actor: readonly Person[] | null;

	constructor(options: MovieOptions) {
		super();
		this.name = options.name;
		this.image = options.image;
		this.aggregateRating = options.aggregateRating ?? null;
		this.dateCreated = options.dateCreated ?? null;
		this.datePublished = options.datePublished ?? null;
		this.director = options.director ?? null;
		this.review = options.review ?? null;
		this.description = options.description ?? null;
		this.actor = options.actor ?? null;
	}
}
