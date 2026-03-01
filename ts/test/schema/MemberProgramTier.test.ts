import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { TierBenefitEnumeration } from "../../src/enum/TierBenefitEnumeration";
import { MemberProgramTier } from "../../src/schema/MemberProgramTier";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";

describe("MemberProgramTier", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new MemberProgramTier(
			"Gold",
			TierBenefitEnumeration.TierBenefitLoyaltyPoints,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("MemberProgramTier");
		expect(obj.name).toBe("Gold");
		expect(obj.hasTierBenefit).toBe(
			"https://schema.org/TierBenefitLoyaltyPoints",
		);
	});

	it("omits optional fields when null", () => {
		const schema = new MemberProgramTier(
			"Gold",
			TierBenefitEnumeration.TierBenefitLoyaltyPoints,
			null,
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("hasTierRequirement");
		expect(obj).not.toHaveProperty("membershipPointsEarned");
		expect(obj).not.toHaveProperty("url");
	});

	it("supports hasTierBenefit as an array of enum values", () => {
		const schema = new MemberProgramTier("Platinum", [
			TierBenefitEnumeration.TierBenefitLoyaltyPoints,
			TierBenefitEnumeration.TierBenefitLoyaltyPrice,
		]);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.hasTierBenefit).toEqual([
			"https://schema.org/TierBenefitLoyaltyPoints",
			"https://schema.org/TierBenefitLoyaltyPrice",
		]);
	});

	it("includes all fields when set", () => {
		const schema = new MemberProgramTier(
			"Gold",
			TierBenefitEnumeration.TierBenefitLoyaltyPrice,
			"Spend at least $1000 annually",
			new QuantitativeValue(1200, "C62"),
			"https://example.com/membership/gold",
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const membershipPointsEarned = obj.membershipPointsEarned as Record<
			string,
			unknown
		>;

		expect(obj.hasTierRequirement).toBe("Spend at least $1000 annually");
		expect(membershipPointsEarned["@type"]).toBe("QuantitativeValue");
		expect(membershipPointsEarned.value).toBe(1200);
		expect(obj.url).toBe("https://example.com/membership/gold");
	});
});
