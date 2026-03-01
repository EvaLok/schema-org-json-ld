import { TypedSchema } from "../TypedSchema.js";
import type { DefinedRegion } from "./DefinedRegion.js";
import type { MonetaryAmount } from "./MonetaryAmount.js";
import type { OpeningHoursSpecification } from "./OpeningHoursSpecification.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";
import type { ServicePeriod } from "./ServicePeriod.js";
import type { ShippingRateSettings } from "./ShippingRateSettings.js";

export interface ShippingConditionsOptions {
	doesNotShip?: boolean | null;
	numItems?: QuantitativeValue | null;
	orderValue?: MonetaryAmount | null;
	shippingDestination?: DefinedRegion | null;
	shippingOrigin?: DefinedRegion | null;
	seasonalOverride?: OpeningHoursSpecification | null;
	shippingRate?: ShippingRateSettings | MonetaryAmount | null;
	transitTime?: ServicePeriod | null;
	weight?: QuantitativeValue | null;
}

export class ShippingConditions extends TypedSchema {
	static readonly schemaType = "ShippingConditions";

	public readonly doesNotShip: boolean | null;
	public readonly numItems: QuantitativeValue | null;
	public readonly orderValue: MonetaryAmount | null;
	public readonly shippingDestination: DefinedRegion | null;
	public readonly shippingOrigin: DefinedRegion | null;
	public readonly seasonalOverride: OpeningHoursSpecification | null;
	public readonly shippingRate: ShippingRateSettings | MonetaryAmount | null;
	public readonly transitTime: ServicePeriod | null;
	public readonly weight: QuantitativeValue | null;

	constructor(options: ShippingConditionsOptions = {}) {
		super();
		this.doesNotShip = options.doesNotShip ?? null;
		this.numItems = options.numItems ?? null;
		this.orderValue = options.orderValue ?? null;
		this.shippingDestination = options.shippingDestination ?? null;
		this.shippingOrigin = options.shippingOrigin ?? null;
		this.seasonalOverride = options.seasonalOverride ?? null;
		this.shippingRate = options.shippingRate ?? null;
		this.transitTime = options.transitTime ?? null;
		this.weight = options.weight ?? null;
	}
}
