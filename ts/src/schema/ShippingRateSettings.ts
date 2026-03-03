import { TypedSchema } from "../TypedSchema.js";

export interface ShippingRateSettingsOptions {
	orderPercentage?: number | null;
	weightPercentage?: number | null;
}

export class ShippingRateSettings extends TypedSchema {
	static readonly schemaType = "ShippingRateSettings";

	public readonly orderPercentage: number | null;
	public readonly weightPercentage: number | null;

	constructor(options: ShippingRateSettingsOptions = {}) {
		super();
		this.orderPercentage = options.orderPercentage ?? null;
		this.weightPercentage = options.weightPercentage ?? null;
	}
}
