import { TypedSchema } from "../TypedSchema.js";
import type { ItemAvailability } from "../enum/ItemAvailability.js";
import type { OfferItemCondition } from "../enum/OfferItemCondition.js";
import type { MerchantReturnPolicy } from "./MerchantReturnPolicy.js";
import type { OfferShippingDetails } from "./OfferShippingDetails.js";
import type { UnitPriceSpecification } from "./UnitPriceSpecification.js";

export interface OfferOptions {
	url: string;
	priceCurrency: string;
	price: number;
	availability: ItemAvailability;
	itemCondition?: OfferItemCondition | null;
	shippingDetails?: readonly OfferShippingDetails[] | null;
	validFrom?: string | null;
	priceValidUntil?: string | null;
	priceSpecification?:
		| UnitPriceSpecification
		| readonly UnitPriceSpecification[]
		| null;
	hasMerchantReturnPolicy?: MerchantReturnPolicy | null;
}

export class Offer extends TypedSchema {
	static readonly schemaType = "Offer";

	public readonly url: string;
	public readonly priceCurrency: string;
	public readonly price: number;
	public readonly availability: ItemAvailability;
	public readonly itemCondition: OfferItemCondition | null;
	public readonly shippingDetails: readonly OfferShippingDetails[] | null;
	public readonly validFrom: string | null;
	public readonly priceValidUntil: string | null;
	public readonly priceSpecification:
		| UnitPriceSpecification
		| readonly UnitPriceSpecification[]
		| null;
	public readonly hasMerchantReturnPolicy: MerchantReturnPolicy | null;

	constructor(options: OfferOptions) {
		super();
		this.url = options.url;
		this.priceCurrency = options.priceCurrency;
		this.price = options.price;
		this.availability = options.availability;
		this.itemCondition = options.itemCondition ?? null;
		this.shippingDetails = options.shippingDetails ?? null;
		this.validFrom = options.validFrom ?? null;
		this.priceValidUntil = options.priceValidUntil ?? null;
		this.priceSpecification = options.priceSpecification ?? null;
		this.hasMerchantReturnPolicy = options.hasMerchantReturnPolicy ?? null;
	}
}
