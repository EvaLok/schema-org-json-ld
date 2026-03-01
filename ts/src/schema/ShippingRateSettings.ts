import { TypedSchema } from "../TypedSchema.js";

export class ShippingRateSettings extends TypedSchema {
	static readonly schemaType = "ShippingRateSettings";

	constructor(
		public readonly orderPercentage: number | null = null,
		public readonly weightPercentage: number | null = null,
	) {
		super();
	}
}
