import { TypedSchema } from "../TypedSchema.js";
import type { MemberProgramTier } from "./MemberProgramTier.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export interface UnitPriceSpecificationOptions {
	price: number;
	priceCurrency: string;
	priceType?: string | null;
	membershipPointsEarned?: number | null;
	validForMemberTier?: MemberProgramTier | null;
	referenceQuantity?: QuantitativeValue | null;
}

export class UnitPriceSpecification extends TypedSchema {
	static readonly schemaType = "UnitPriceSpecification";

	public readonly price: number;
	public readonly priceCurrency: string;
	public readonly priceType: string | null;
	public readonly membershipPointsEarned: number | null;
	public readonly validForMemberTier: MemberProgramTier | null;
	public readonly referenceQuantity: QuantitativeValue | null;

	constructor(options: UnitPriceSpecificationOptions) {
		super();
		this.price = options.price;
		this.priceCurrency = options.priceCurrency;
		this.priceType = options.priceType ?? null;
		this.membershipPointsEarned = options.membershipPointsEarned ?? null;
		this.validForMemberTier = options.validForMemberTier ?? null;
		this.referenceQuantity = options.referenceQuantity ?? null;
	}
}
