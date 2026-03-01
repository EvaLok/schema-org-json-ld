import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { TierBenefitEnumeration } from "../../src/enum/TierBenefitEnumeration";
import { MemberProgram } from "../../src/schema/MemberProgram";
import { MemberProgramTier } from "../../src/schema/MemberProgramTier";

describe("MemberProgram", () => {
it("produces minimal JSON-LD output with required fields only", () => {
const schema = new MemberProgram(
"Rewards+",
"Earn points on every purchase",
[
new MemberProgramTier(
"Gold",
TierBenefitEnumeration.TierBenefitLoyaltyPoints,
),
],
);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;
const hasTiers = obj.hasTiers as Record<string, unknown>[];

expect(obj["@context"]).toBe("https://schema.org/");
expect(obj["@type"]).toBe("MemberProgram");
expect(obj.name).toBe("Rewards+");
expect(obj.description).toBe("Earn points on every purchase");
expect(hasTiers).toHaveLength(1);
expect(hasTiers[0]?.["@type"]).toBe("MemberProgramTier");
});

it("omits optional fields when null", () => {
const schema = new MemberProgram("Rewards+", "Earn points", [], null);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;

expect(obj).not.toHaveProperty("url");
});

it("includes all fields when set", () => {
const schema = new MemberProgram(
"Rewards+",
"Earn points on every purchase",
[
new MemberProgramTier(
"Gold",
TierBenefitEnumeration.TierBenefitLoyaltyPrice,
),
],
"https://example.com/rewards",
);
const json = JsonLdGenerator.schemaToJson(schema);
const obj = JSON.parse(json) as Record<string, unknown>;

expect(obj.url).toBe("https://example.com/rewards");
});
});
