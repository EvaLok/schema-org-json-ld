import { TypedSchema } from "../TypedSchema.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export class ShippingDeliveryTime extends TypedSchema {
	static readonly schemaType = "ShippingDeliveryTime";

	constructor(
		public readonly handlingTime: QuantitativeValue,
		public readonly transitTime: QuantitativeValue,
	) {
		super();
	}
}
