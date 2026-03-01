import { TypedSchema } from "../TypedSchema.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { Brand } from "./Brand.js";
import type { Product } from "./Product.js";
import type { Review } from "./Review.js";

export interface ProductGroupOptions {
	name: string;
	productGroupID?: string | null;
	variesBy?: string | readonly string[] | null;
	hasVariant?: Product | readonly Product[] | null;
	url?: string | null;
	description?: string | null;
	brand?: Brand | null;
	aggregateRating?: AggregateRating | null;
	review?: Review | null;
	subjectOf?: string | null;
}

export class ProductGroup extends TypedSchema {
	static readonly schemaType = "ProductGroup";

	public readonly name: string;
	public readonly productGroupID: string | null;
	public readonly variesBy: string | readonly string[] | null;
	public readonly hasVariant: Product | readonly Product[] | null;
	public readonly url: string | null;
	public readonly description: string | null;
	public readonly brand: Brand | null;
	public readonly aggregateRating: AggregateRating | null;
	public readonly review: Review | null;
	public readonly subjectOf: string | null;

	constructor(options: ProductGroupOptions) {
		super();
		this.name = options.name;
		this.productGroupID = options.productGroupID ?? null;
		this.variesBy = options.variesBy ?? null;
		this.hasVariant = options.hasVariant ?? null;
		this.url = options.url ?? null;
		this.description = options.description ?? null;
		this.brand = options.brand ?? null;
		this.aggregateRating = options.aggregateRating ?? null;
		this.review = options.review ?? null;
		this.subjectOf = options.subjectOf ?? null;
	}
}
