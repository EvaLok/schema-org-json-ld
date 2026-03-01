import { TypedSchema } from "../TypedSchema.js";
import type { MerchantReturnEnumeration } from "../enum/MerchantReturnEnumeration.js";
import type { OfferItemCondition } from "../enum/OfferItemCondition.js";
import type { RefundTypeEnumeration } from "../enum/RefundTypeEnumeration.js";
import type { ReturnFeesEnumeration } from "../enum/ReturnFeesEnumeration.js";
import type { ReturnLabelSourceEnumeration } from "../enum/ReturnLabelSourceEnumeration.js";
import type { ReturnMethodEnumeration } from "../enum/ReturnMethodEnumeration.js";
import type { MerchantReturnPolicySeasonalOverride } from "./MerchantReturnPolicySeasonalOverride.js";
import type { MonetaryAmount } from "./MonetaryAmount.js";

export interface MerchantReturnPolicyOptions {
applicableCountry: string | readonly string[];
returnPolicyCategory: MerchantReturnEnumeration;
merchantReturnDays?: number | null;
merchantReturnLink?: string | null;
returnMethod?: ReturnMethodEnumeration | null;
returnFees?: ReturnFeesEnumeration | null;
returnShippingFeesAmount?: MonetaryAmount | null;
refundType?: RefundTypeEnumeration | null;
itemCondition?: OfferItemCondition | null;
returnLabelSource?: ReturnLabelSourceEnumeration | null;
returnPolicyCountry?: string | null;
restockingFee?: MonetaryAmount | number | null;
customerRemorseReturnFees?: ReturnFeesEnumeration | null;
customerRemorseReturnLabelSource?: ReturnLabelSourceEnumeration | null;
customerRemorseReturnShippingFeesAmount?: MonetaryAmount | null;
itemDefectReturnFees?: ReturnFeesEnumeration | null;
itemDefectReturnLabelSource?: ReturnLabelSourceEnumeration | null;
itemDefectReturnShippingFeesAmount?: MonetaryAmount | null;
returnPolicySeasonalOverride?:
| MerchantReturnPolicySeasonalOverride
| readonly MerchantReturnPolicySeasonalOverride[]
| null;
}

export class MerchantReturnPolicy extends TypedSchema {
static readonly schemaType = "MerchantReturnPolicy";

public readonly applicableCountry: string | readonly string[];
public readonly returnPolicyCategory: MerchantReturnEnumeration;
public readonly merchantReturnDays: number | null;
public readonly merchantReturnLink: string | null;
public readonly returnMethod: ReturnMethodEnumeration | null;
public readonly returnFees: ReturnFeesEnumeration | null;
public readonly returnShippingFeesAmount: MonetaryAmount | null;
public readonly refundType: RefundTypeEnumeration | null;
public readonly itemCondition: OfferItemCondition | null;
public readonly returnLabelSource: ReturnLabelSourceEnumeration | null;
public readonly returnPolicyCountry: string | null;
public readonly restockingFee: MonetaryAmount | number | null;
public readonly customerRemorseReturnFees: ReturnFeesEnumeration | null;
public readonly customerRemorseReturnLabelSource: ReturnLabelSourceEnumeration | null;
public readonly customerRemorseReturnShippingFeesAmount: MonetaryAmount | null;
public readonly itemDefectReturnFees: ReturnFeesEnumeration | null;
public readonly itemDefectReturnLabelSource: ReturnLabelSourceEnumeration | null;
public readonly itemDefectReturnShippingFeesAmount: MonetaryAmount | null;
public readonly returnPolicySeasonalOverride:
| MerchantReturnPolicySeasonalOverride
| readonly MerchantReturnPolicySeasonalOverride[]
| null;

constructor(options: MerchantReturnPolicyOptions) {
super();
this.applicableCountry = options.applicableCountry;
this.returnPolicyCategory = options.returnPolicyCategory;
this.merchantReturnDays = options.merchantReturnDays ?? null;
this.merchantReturnLink = options.merchantReturnLink ?? null;
this.returnMethod = options.returnMethod ?? null;
this.returnFees = options.returnFees ?? null;
this.returnShippingFeesAmount = options.returnShippingFeesAmount ?? null;
this.refundType = options.refundType ?? null;
this.itemCondition = options.itemCondition ?? null;
this.returnLabelSource = options.returnLabelSource ?? null;
this.returnPolicyCountry = options.returnPolicyCountry ?? null;
this.restockingFee = options.restockingFee ?? null;
this.customerRemorseReturnFees = options.customerRemorseReturnFees ?? null;
this.customerRemorseReturnLabelSource =
options.customerRemorseReturnLabelSource ?? null;
this.customerRemorseReturnShippingFeesAmount =
options.customerRemorseReturnShippingFeesAmount ?? null;
this.itemDefectReturnFees = options.itemDefectReturnFees ?? null;
this.itemDefectReturnLabelSource = options.itemDefectReturnLabelSource ?? null;
this.itemDefectReturnShippingFeesAmount =
options.itemDefectReturnShippingFeesAmount ?? null;
this.returnPolicySeasonalOverride = options.returnPolicySeasonalOverride ?? null;
}
}
