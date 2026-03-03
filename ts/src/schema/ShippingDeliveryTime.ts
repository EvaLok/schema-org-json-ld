import { TypedSchema } from "../TypedSchema.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export interface ShippingDeliveryTimeOptions {
	handlingTime: QuantitativeValue;
	transitTime: QuantitativeValue;
}

export class ShippingDeliveryTime extends TypedSchema {
	static readonly schemaType = "ShippingDeliveryTime";

	public readonly handlingTime: QuantitativeValue;
	public readonly transitTime: QuantitativeValue;

	constructor(options: ShippingDeliveryTimeOptions) {
		super();
		this.handlingTime = options.handlingTime;
		this.transitTime = options.transitTime;
	}
}
