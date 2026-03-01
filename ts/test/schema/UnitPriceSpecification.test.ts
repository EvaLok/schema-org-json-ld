import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { TierBenefitEnumeration } from "../../src/enum/TierBenefitEnumeration";
import { MemberProgramTier } from "../../src/schema/MemberProgramTier";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { UnitPriceSpecification } from "../../src/schema/UnitPriceSpecification";

describe("UnitPriceSpecification", () => {
it("produces minimal JSON-LD output with required fields only", () => {
const schema = new UnitPriceSpecification(19.99, "USD");
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;

expect(obj["@context"]).toBe("https://schema.org/");
expect(obj["@type"]).toBe("UnitPriceSpecification");
expect(obj.price).toBe(19.99);
expect(obj.priceCurrency).toBe("USD");
});

it("omits optional fields when null", () => {
const schema = new UnitPriceSpecification(
19.99,
"USD",
null,
null,
null,
null,
);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;

expect(obj).not.toHaveProperty("priceType");
expect(obj).not.toHaveProperty("membershipPointsEarned");
expect(obj).not.toHaveProperty("validForMemberTier");
expect(obj).not.toHaveProperty("referenceQuantity");
});

it("includes all fields when set", () => {
const schema = new UnitPriceSpecification(
14.5,
"USD",
"https://schema.org/SalePrice",
150,
new MemberProgramTier(
"Gold",
TierBenefitEnumeration.TierBenefitLoyaltyPoints,
),
new QuantitativeValue(1, "C62"),
);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;
const validForMemberTier = obj.validForMemberTier as Record<string, unknown>;
const referenceQuantity = obj.referenceQuantity as Record<string, unknown>;

expect(obj.priceType).toBe("https://schema.org/SalePrice");
expect(obj.membershipPointsEarned).toBe(150);
expect(validForMemberTier["@type"]).toBe("MemberProgramTier");
expect(referenceQuantity["@type"]).toBe("QuantitativeValue");
});
});
