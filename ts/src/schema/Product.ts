import { TypedSchema } from "../TypedSchema.js";
import type { AggregateOffer } from "./AggregateOffer.js";
import type { AggregateRating } from "./AggregateRating.js";
import type { Brand } from "./Brand.js";
import type { Certification } from "./Certification.js";
import type { Offer } from "./Offer.js";
import type { PeopleAudience } from "./PeopleAudience.js";
import type { ProductGroup } from "./ProductGroup.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";
import type { Review } from "./Review.js";
import type { SizeSpecification } from "./SizeSpecification.js";

export interface ProductOptions {
	name: string;
	image: readonly string[];
	description: string;
	sku: string;
	offers: readonly Offer[] | AggregateOffer;
	brand?: Brand | null;
	mpn?: string | null;
	weight?: QuantitativeValue | null;
	aggregateRating?: AggregateRating | null;
	review?: Review | readonly Review[] | null;
	color?: string | null;
	material?: string | null;
	pattern?: string | null;
	size?: string | SizeSpecification | null;
	inProductGroupWithID?: string | null;
	gtin?: string | null;
	gtin8?: string | null;
	gtin12?: string | null;
	gtin13?: string | null;
	gtin14?: string | null;
	isbn?: string | null;
	isVariantOf?: ProductGroup | null;
	audience?: PeopleAudience | null;
	hasCertification?: Certification | readonly Certification[] | null;
	subjectOf?: string | null;
}

export class Product extends TypedSchema {
	static readonly schemaType = "Product";

	public readonly name: string;
	public readonly image: readonly string[];
	public readonly description: string;
	public readonly sku: string;
	public readonly offers: readonly Offer[] | AggregateOffer;
	public readonly brand: Brand | null;
	public readonly mpn: string | null;
	public readonly weight: QuantitativeValue | null;
	public readonly aggregateRating: AggregateRating | null;
	public readonly review: Review | readonly Review[] | null;
	public readonly color: string | null;
	public readonly material: string | null;
	public readonly pattern: string | null;
	public readonly size: string | SizeSpecification | null;
	public readonly inProductGroupWithID: string | null;
	public readonly gtin: string | null;
	public readonly gtin8: string | null;
	public readonly gtin12: string | null;
	public readonly gtin13: string | null;
	public readonly gtin14: string | null;
	public readonly isbn: string | null;
	public readonly isVariantOf: ProductGroup | null;
	public readonly audience: PeopleAudience | null;
	public readonly hasCertification:
		| Certification
		| readonly Certification[]
		| null;
	public readonly subjectOf: string | null;

	constructor(options: ProductOptions) {
		super();
		this.name = options.name;
		this.image = options.image;
		this.description = options.description;
		this.sku = options.sku;
		this.offers = options.offers;
		this.brand = options.brand ?? null;
		this.mpn = options.mpn ?? null;
		this.weight = options.weight ?? null;
		this.aggregateRating = options.aggregateRating ?? null;
		this.review = options.review ?? null;
		this.color = options.color ?? null;
		this.material = options.material ?? null;
		this.pattern = options.pattern ?? null;
		this.size = options.size ?? null;
		this.inProductGroupWithID = options.inProductGroupWithID ?? null;
		this.gtin = options.gtin ?? null;
		this.gtin8 = options.gtin8 ?? null;
		this.gtin12 = options.gtin12 ?? null;
		this.gtin13 = options.gtin13 ?? null;
		this.gtin14 = options.gtin14 ?? null;
		this.isbn = options.isbn ?? null;
		this.isVariantOf = options.isVariantOf ?? null;
		this.audience = options.audience ?? null;
		this.hasCertification = options.hasCertification ?? null;
		this.subjectOf = options.subjectOf ?? null;
	}
}
