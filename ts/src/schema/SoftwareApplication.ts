import { TypedSchema } from "../TypedSchema.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { Offer } from "./Offer.js";
import type { Review } from "./Review.js";

export interface SoftwareApplicationOptions {
	name: string;
	offers: Offer | readonly Offer[];
	aggregateRating: AggregateRating | null;
	applicationCategory?: string | null;
	operatingSystem?: string | null;
	datePublished?: string | null;
	review?: Review | null;
	description?: string | null;
	screenshot?: string | null;
}

export class SoftwareApplication extends TypedSchema {
	static readonly schemaType: string = "SoftwareApplication";

	public readonly name: string;
	public readonly offers: Offer | readonly Offer[];
	public readonly aggregateRating: AggregateRating | null;
	public readonly applicationCategory: string | null;
	public readonly operatingSystem: string | null;
	public readonly datePublished: string | null;
	public readonly review: Review | null;
	public readonly description: string | null;
	public readonly screenshot: string | null;

	constructor(options: SoftwareApplicationOptions) {
		super();
		this.name = options.name;
		this.offers = options.offers;
		this.aggregateRating = options.aggregateRating;
		this.applicationCategory = options.applicationCategory ?? null;
		this.operatingSystem = options.operatingSystem ?? null;
		this.datePublished = options.datePublished ?? null;
		this.review = options.review ?? null;
		this.description = options.description ?? null;
		this.screenshot = options.screenshot ?? null;
	}
}
