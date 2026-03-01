import { TypedSchema } from "../TypedSchema.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { GeoCoordinates } from "./GeoCoordinates.js";
import type { OpeningHoursSpecification } from "./OpeningHoursSpecification.js";
import type { PostalAddress } from "./PostalAddress.js";
import type { Review } from "./Review.js";

export interface LocalBusinessOptions {
	name: string;
	address: PostalAddress;
	url?: string | null;
	telephone?: string | null;
	description?: string | null;
	image?: readonly string[] | null;
	priceRange?: string | null;
	geo?: GeoCoordinates | null;
	openingHoursSpecification?: readonly OpeningHoursSpecification[] | null;
	aggregateRating?: AggregateRating | null;
	review?: Review | readonly Review[] | null;
	menu?: string | null;
	servesCuisine?: string | null;
	logo?: string | null;
	email?: string | null;
	sameAs?: readonly string[] | null;
	department?: LocalBusiness | readonly LocalBusiness[] | null;
}

export class LocalBusiness extends TypedSchema {
	static readonly schemaType: string = "LocalBusiness";

	public readonly name: string;
	public readonly address: PostalAddress;
	public readonly url: string | null;
	public readonly telephone: string | null;
	public readonly description: string | null;
	public readonly image: readonly string[] | null;
	public readonly priceRange: string | null;
	public readonly geo: GeoCoordinates | null;
	public readonly openingHoursSpecification:
		| readonly OpeningHoursSpecification[]
		| null;
	public readonly aggregateRating: AggregateRating | null;
	public readonly review: Review | readonly Review[] | null;
	public readonly menu: string | null;
	public readonly servesCuisine: string | null;
	public readonly logo: string | null;
	public readonly email: string | null;
	public readonly sameAs: readonly string[] | null;
	public readonly department: LocalBusiness | readonly LocalBusiness[] | null;

	constructor(options: LocalBusinessOptions) {
		super();
		this.name = options.name;
		this.address = options.address;
		this.url = options.url ?? null;
		this.telephone = options.telephone ?? null;
		this.description = options.description ?? null;
		this.image = options.image ?? null;
		this.priceRange = options.priceRange ?? null;
		this.geo = options.geo ?? null;
		this.openingHoursSpecification = options.openingHoursSpecification ?? null;
		this.aggregateRating = options.aggregateRating ?? null;
		this.review = options.review ?? null;
		this.menu = options.menu ?? null;
		this.servesCuisine = options.servesCuisine ?? null;
		this.logo = options.logo ?? null;
		this.email = options.email ?? null;
		this.sameAs = options.sameAs ?? null;
		this.department = options.department ?? null;
	}
}
