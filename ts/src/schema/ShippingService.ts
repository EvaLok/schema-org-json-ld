import { TypedSchema } from "../TypedSchema.js";
import type { FulfillmentTypeEnumeration } from "../enum/FulfillmentTypeEnumeration.js";
import type { MemberProgramTier } from "./MemberProgramTier.js";
import type { ServicePeriod } from "./ServicePeriod.js";
import type { ShippingConditions } from "./ShippingConditions.js";

export class ShippingService extends TypedSchema {
	static readonly schemaType = "ShippingService";

	constructor(
		public readonly shippingConditions:
			| ShippingConditions
			| readonly ShippingConditions[],
		public readonly name: string | null = null,
		public readonly description: string | null = null,
		public readonly fulfillmentType: FulfillmentTypeEnumeration | null = null,
		public readonly handlingTime: ServicePeriod | null = null,
		public readonly validForMemberTier: MemberProgramTier | null = null,
	) {
		super();
	}
}
