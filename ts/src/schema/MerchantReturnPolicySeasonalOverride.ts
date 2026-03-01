import { TypedSchema } from "../TypedSchema.js";
import type { MerchantReturnEnumeration } from "../enum/MerchantReturnEnumeration.js";

export class MerchantReturnPolicySeasonalOverride extends TypedSchema {
	static readonly schemaType = "MerchantReturnPolicySeasonalOverride";

	constructor(
		public readonly startDate: string,
		public readonly endDate: string,
		public readonly returnPolicyCategory: MerchantReturnEnumeration,
		public readonly merchantReturnDays: number | null = null,
	) {
		super();
	}
}
