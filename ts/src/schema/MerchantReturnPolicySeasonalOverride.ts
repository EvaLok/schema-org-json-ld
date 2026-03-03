import { TypedSchema } from "../TypedSchema.js";
import type { MerchantReturnEnumeration } from "../enum/MerchantReturnEnumeration.js";

export interface MerchantReturnPolicySeasonalOverrideOptions {
	startDate: string;
	endDate: string;
	returnPolicyCategory: MerchantReturnEnumeration;
	merchantReturnDays?: number | null;
}

export class MerchantReturnPolicySeasonalOverride extends TypedSchema {
	static readonly schemaType = "MerchantReturnPolicySeasonalOverride";

	public readonly startDate: string;
	public readonly endDate: string;
	public readonly returnPolicyCategory: MerchantReturnEnumeration;
	public readonly merchantReturnDays: number | null;

	constructor(options: MerchantReturnPolicySeasonalOverrideOptions) {
		super();
		this.startDate = options.startDate;
		this.endDate = options.endDate;
		this.returnPolicyCategory = options.returnPolicyCategory;
		this.merchantReturnDays = options.merchantReturnDays ?? null;
	}
}
