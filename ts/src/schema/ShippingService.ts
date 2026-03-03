import { TypedSchema } from "../TypedSchema.js";
import type { FulfillmentTypeEnumeration } from "../enum/FulfillmentTypeEnumeration.js";
import type { MemberProgramTier } from "./MemberProgramTier.js";
import type { ServicePeriod } from "./ServicePeriod.js";
import type { ShippingConditions } from "./ShippingConditions.js";

export interface ShippingServiceOptions {
	shippingConditions: ShippingConditions | readonly ShippingConditions[];
	name?: string | null;
	description?: string | null;
	fulfillmentType?: FulfillmentTypeEnumeration | null;
	handlingTime?: ServicePeriod | null;
	validForMemberTier?: MemberProgramTier | null;
}

export class ShippingService extends TypedSchema {
	static readonly schemaType = "ShippingService";

	public readonly shippingConditions:
		| ShippingConditions
		| readonly ShippingConditions[];
	public readonly name: string | null;
	public readonly description: string | null;
	public readonly fulfillmentType: FulfillmentTypeEnumeration | null;
	public readonly handlingTime: ServicePeriod | null;
	public readonly validForMemberTier: MemberProgramTier | null;

	constructor(options: ShippingServiceOptions) {
		super();
		this.shippingConditions = options.shippingConditions;
		this.name = options.name ?? null;
		this.description = options.description ?? null;
		this.fulfillmentType = options.fulfillmentType ?? null;
		this.handlingTime = options.handlingTime ?? null;
		this.validForMemberTier = options.validForMemberTier ?? null;
	}
}
