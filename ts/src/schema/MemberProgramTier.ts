import { TypedSchema } from "../TypedSchema.js";
import type { TierBenefitEnumeration } from "../enum/TierBenefitEnumeration.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export class MemberProgramTier extends TypedSchema {
	static readonly schemaType = "MemberProgramTier";

	constructor(
		public readonly name: string,
		public readonly hasTierBenefit:
			| TierBenefitEnumeration
			| readonly TierBenefitEnumeration[],
		public readonly hasTierRequirement: string | null = null,
		public readonly membershipPointsEarned: QuantitativeValue | null = null,
		public readonly url: string | null = null,
	) {
		super();
	}
}
