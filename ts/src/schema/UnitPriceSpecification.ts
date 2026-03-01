import { TypedSchema } from "../TypedSchema.js";
import type { MemberProgramTier } from "./MemberProgramTier.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export class UnitPriceSpecification extends TypedSchema {
	static readonly schemaType = "UnitPriceSpecification";

	constructor(
		public readonly price: number,
		public readonly priceCurrency: string,
		public readonly priceType: string | null = null,
		public readonly membershipPointsEarned: number | null = null,
		public readonly validForMemberTier: MemberProgramTier | null = null,
		public readonly referenceQuantity: QuantitativeValue | null = null,
	) {
		super();
	}
}
