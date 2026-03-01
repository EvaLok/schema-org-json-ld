import { TypedSchema } from "../TypedSchema.js";
import type { Accommodation } from "./Accommodation.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { Brand } from "./Brand.js";
import type { PostalAddress } from "./PostalAddress.js";
import type { Review } from "./Review.js";

export interface VacationRentalOptions {
	name: string;
	identifier: string;
	image: readonly string[];
	latitude: number;
	longitude: number;
	containsPlace: Accommodation;
	additionalType?: string | null;
	address?: PostalAddress | null;
	aggregateRating?: AggregateRating | null;
	brand?: Brand | null;
	checkinTime?: string | null;
	checkoutTime?: string | null;
	datePublished?: string | null;
	description?: string | null;
	knowsLanguage?: readonly string[] | null;
	review?: readonly Review[] | null;
}

export class VacationRental extends TypedSchema {
	static readonly schemaType = "VacationRental";

	public readonly name: string;
	public readonly identifier: string;
	public readonly image: readonly string[];
	public readonly latitude: number;
	public readonly longitude: number;
	public readonly containsPlace: Accommodation;
	public readonly additionalType: string | null;
	public readonly address: PostalAddress | null;
	public readonly aggregateRating: AggregateRating | null;
	public readonly brand: Brand | null;
	public readonly checkinTime: string | null;
	public readonly checkoutTime: string | null;
	public readonly datePublished: string | null;
	public readonly description: string | null;
	public readonly knowsLanguage: readonly string[] | null;
	public readonly review: readonly Review[] | null;

	constructor(options: VacationRentalOptions) {
		super();
		this.name = options.name;
		this.identifier = options.identifier;
		this.image = options.image;
		this.latitude = options.latitude;
		this.longitude = options.longitude;
		this.containsPlace = options.containsPlace;
		this.additionalType = options.additionalType ?? null;
		this.address = options.address ?? null;
		this.aggregateRating = options.aggregateRating ?? null;
		this.brand = options.brand ?? null;
		this.checkinTime = options.checkinTime ?? null;
		this.checkoutTime = options.checkoutTime ?? null;
		this.datePublished = options.datePublished ?? null;
		this.description = options.description ?? null;
		this.knowsLanguage = options.knowsLanguage ?? null;
		this.review = options.review ?? null;
	}
}
