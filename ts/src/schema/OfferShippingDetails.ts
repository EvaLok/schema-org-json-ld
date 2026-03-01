import { TypedSchema } from "../TypedSchema.js";
import type { DefinedRegion } from "./DefinedRegion.js";
import type { MonetaryAmount } from "./MonetaryAmount.js";
import type { ShippingDeliveryTime } from "./ShippingDeliveryTime.js";

export class OfferShippingDetails extends TypedSchema {
	static readonly schemaType = "OfferShippingDetails";

	constructor(
		public readonly shippingDestination: DefinedRegion,
		public readonly shippingRate: MonetaryAmount | null = null,
		public readonly deliveryTime: ShippingDeliveryTime | null = null,
		public readonly doesNotShip: boolean | null = null,
	) {
		super();
	}
}
