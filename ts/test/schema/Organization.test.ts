import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { MerchantReturnEnumeration } from "../../src/enum/MerchantReturnEnumeration";
import { TierBenefitEnumeration } from "../../src/enum/TierBenefitEnumeration";
import { ContactPoint } from "../../src/schema/ContactPoint";
import { MemberProgram } from "../../src/schema/MemberProgram";
import { MemberProgramTier } from "../../src/schema/MemberProgramTier";
import { MerchantReturnPolicy } from "../../src/schema/MerchantReturnPolicy";
import { Organization } from "../../src/schema/Organization";
import { PostalAddress } from "../../src/schema/PostalAddress";
import { ShippingConditions } from "../../src/schema/ShippingConditions";
import { ShippingService } from "../../src/schema/ShippingService";

describe("Organization", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new Organization({ name: "Example Inc." });
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Organization");
		expect(obj.name).toBe("Example Inc.");
	});

	it("omits optional fields when null", () => {
		const schema = new Organization({
			name: "Example Inc.",
			url: null,
			logo: null,
			description: null,
			email: null,
			telephone: null,
			address: null,
			contactPoint: null,
			sameAs: null,
			foundingDate: null,
			alternateName: null,
			legalName: null,
			numberOfEmployees: null,
			taxID: null,
			vatID: null,
			naics: null,
			duns: null,
			leiCode: null,
			iso6523Code: null,
			globalLocationNumber: null,
			hasMerchantReturnPolicy: null,
			hasMemberProgram: null,
			hasShippingService: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("url");
		expect(obj).not.toHaveProperty("address");
		expect(obj).not.toHaveProperty("hasMerchantReturnPolicy");
		expect(obj).not.toHaveProperty("hasMemberProgram");
		expect(obj).not.toHaveProperty("hasShippingService");
	});

	it("includes nested fields when set", () => {
		const schema = new Organization({
			name: "Example Inc.",
			url: "https://example.com",
			address: new PostalAddress({
				streetAddress: "1600 Amphitheatre Parkway",
				addressLocality: "Mountain View",
			}),
			contactPoint: new ContactPoint("555-0100", "support@example.com"),
			sameAs: ["https://x.com/example"],
			hasMerchantReturnPolicy: new MerchantReturnPolicy({
				applicableCountry: "US",
				returnPolicyCategory:
					MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			}),
			hasMemberProgram: [
				new MemberProgram("Rewards+", "Loyalty program", [
					new MemberProgramTier(
						"Gold",
						TierBenefitEnumeration.TierBenefitLoyaltyPoints,
					),
				]),
			],
			hasShippingService: new ShippingService(new ShippingConditions({})),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const address = obj.address as Record<string, unknown>;
		const contactPoint = obj.contactPoint as Record<string, unknown>;
		const hasMerchantReturnPolicy = obj.hasMerchantReturnPolicy as Record<
			string,
			unknown
		>;
		const hasMemberProgram = obj.hasMemberProgram as Record<string, unknown>[];
		const hasShippingService = obj.hasShippingService as Record<
			string,
			unknown
		>;

		expect(obj.url).toBe("https://example.com");
		expect(address["@type"]).toBe("PostalAddress");
		expect(contactPoint["@type"]).toBe("ContactPoint");
		expect(hasMerchantReturnPolicy["@type"]).toBe("MerchantReturnPolicy");
		expect(hasMemberProgram[0]?.["@type"]).toBe("MemberProgram");
		expect(hasShippingService["@type"]).toBe("ShippingService");
	});

	it("supports union properties as arrays", () => {
		const schema = new Organization({
			name: "Example Inc.",
			hasMerchantReturnPolicy: [
				new MerchantReturnPolicy({
					applicableCountry: "US",
					returnPolicyCategory:
						MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
				}),
			],
			hasMemberProgram: [
				new MemberProgram("Rewards+", "Loyalty program", [
					new MemberProgramTier(
						"Gold",
						TierBenefitEnumeration.TierBenefitLoyaltyPoints,
					),
				]),
			],
			hasShippingService: [new ShippingService(new ShippingConditions({}))],
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(Array.isArray(obj.hasMerchantReturnPolicy)).toBe(true);
		expect(Array.isArray(obj.hasMemberProgram)).toBe(true);
		expect(Array.isArray(obj.hasShippingService)).toBe(true);
	});
});
