import { TypedSchema } from "../TypedSchema.js";
import type { DefinedRegion } from "./DefinedRegion.js";
import type { MonetaryAmount } from "./MonetaryAmount.js";
import type { ShippingDeliveryTime } from "./ShippingDeliveryTime.js";

export interface OfferShippingDetailsOptions {
	shippingDestination: DefinedRegion;
	shippingRate?: MonetaryAmount | null;
	deliveryTime?: ShippingDeliveryTime | null;
	doesNotShip?: boolean | null;
}

export class OfferShippingDetails extends TypedSchema {
	static readonly schemaType = "OfferShippingDetails";

	public readonly shippingDestination: DefinedRegion;
	public readonly shippingRate: MonetaryAmount | null;
	public readonly deliveryTime: ShippingDeliveryTime | null;
	public readonly doesNotShip: boolean | null;

	constructor(options: OfferShippingDetailsOptions) {
		super();
		this.shippingDestination = options.shippingDestination;
		this.shippingRate = options.shippingRate ?? null;
		this.deliveryTime = options.deliveryTime ?? null;
		this.doesNotShip = options.doesNotShip ?? null;
	}
}
