import { TypedSchema } from "../TypedSchema.js";
import type { TierBenefitEnumeration } from "../enum/TierBenefitEnumeration.js";
import type { QuantitativeValue } from "./QuantitativeValue.js";

export interface MemberProgramTierOptions {
	name: string;
	hasTierBenefit: TierBenefitEnumeration | readonly TierBenefitEnumeration[];
	hasTierRequirement?: string | null;
	membershipPointsEarned?: QuantitativeValue | null;
	url?: string | null;
}

export class MemberProgramTier extends TypedSchema {
	static readonly schemaType = "MemberProgramTier";

	public readonly name: string;
	public readonly hasTierBenefit:
		| TierBenefitEnumeration
		| readonly TierBenefitEnumeration[];
	public readonly hasTierRequirement: string | null;
	public readonly membershipPointsEarned: QuantitativeValue | null;
	public readonly url: string | null;

	constructor(options: MemberProgramTierOptions) {
		super();
		this.name = options.name;
		this.hasTierBenefit = options.hasTierBenefit;
		this.hasTierRequirement = options.hasTierRequirement ?? null;
		this.membershipPointsEarned = options.membershipPointsEarned ?? null;
		this.url = options.url ?? null;
	}
}
