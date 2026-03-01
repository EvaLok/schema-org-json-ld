import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { MerchantReturnEnumeration } from "../../src/enum/MerchantReturnEnumeration";
import { OfferItemCondition } from "../../src/enum/OfferItemCondition";
import { RefundTypeEnumeration } from "../../src/enum/RefundTypeEnumeration";
import { ReturnFeesEnumeration } from "../../src/enum/ReturnFeesEnumeration";
import { ReturnLabelSourceEnumeration } from "../../src/enum/ReturnLabelSourceEnumeration";
import { ReturnMethodEnumeration } from "../../src/enum/ReturnMethodEnumeration";
import { MerchantReturnPolicy } from "../../src/schema/MerchantReturnPolicy";
import { MerchantReturnPolicySeasonalOverride } from "../../src/schema/MerchantReturnPolicySeasonalOverride";
import { MonetaryAmount } from "../../src/schema/MonetaryAmount";

describe("MerchantReturnPolicy", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new MerchantReturnPolicy({
			applicableCountry: "US",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("MerchantReturnPolicy");
		expect(obj.applicableCountry).toBe("US");
		expect(obj.returnPolicyCategory).toBe(
			"https://schema.org/MerchantReturnFiniteReturnWindow",
		);
	});

	it("omits optional fields when null", () => {
		const schema = new MerchantReturnPolicy({
			applicableCountry: "US",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			merchantReturnDays: null,
			merchantReturnLink: null,
			returnMethod: null,
			returnFees: null,
			returnShippingFeesAmount: null,
			refundType: null,
			itemCondition: null,
			returnLabelSource: null,
			returnPolicyCountry: null,
			restockingFee: null,
			customerRemorseReturnFees: null,
			customerRemorseReturnLabelSource: null,
			customerRemorseReturnShippingFeesAmount: null,
			itemDefectReturnFees: null,
			itemDefectReturnLabelSource: null,
			itemDefectReturnShippingFeesAmount: null,
			returnPolicySeasonalOverride: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("merchantReturnDays");
		expect(obj).not.toHaveProperty("returnMethod");
		expect(obj).not.toHaveProperty("restockingFee");
		expect(obj).not.toHaveProperty("returnPolicySeasonalOverride");
	});

	it("supports applicableCountry as an array", () => {
		const schema = new MerchantReturnPolicy({
			applicableCountry: ["US", "CA"],
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnUnlimitedWindow,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.applicableCountry).toEqual(["US", "CA"]);
	});

	it("supports restockingFee as a number", () => {
		const schema = new MerchantReturnPolicy({
			applicableCountry: "US",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			restockingFee: 12.5,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.restockingFee).toBe(12.5);
	});

	it("supports returnPolicySeasonalOverride as single and array", () => {
		const overrideA = new MerchantReturnPolicySeasonalOverride(
			"2026-11-20",
			"2026-12-31",
			MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			45,
		);
		const singleSchema = new MerchantReturnPolicy({
			applicableCountry: "US",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			returnPolicySeasonalOverride: overrideA,
		});
		const singleObj = JSON.parse(
			JsonLdGenerator.schemaToJson(singleSchema),
		) as Record<string, unknown>;
		const singleOverride = singleObj.returnPolicySeasonalOverride as Record<
			string,
			unknown
		>;

		expect(singleOverride["@type"]).toBe(
			"MerchantReturnPolicySeasonalOverride",
		);

		const overrideB = new MerchantReturnPolicySeasonalOverride(
			"2027-01-01",
			"2027-01-15",
			MerchantReturnEnumeration.MerchantReturnUnlimitedWindow,
		);
		const arraySchema = new MerchantReturnPolicy({
			applicableCountry: "US",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			returnPolicySeasonalOverride: [overrideA, overrideB],
		});
		const arrayObj = JSON.parse(
			JsonLdGenerator.schemaToJson(arraySchema),
		) as Record<string, unknown>;
		const overrides = arrayObj.returnPolicySeasonalOverride as Record<
			string,
			unknown
		>[];

		expect(overrides).toHaveLength(2);
		expect(overrides[0]?.["@type"]).toBe(
			"MerchantReturnPolicySeasonalOverride",
		);
	});

	it("includes representative optional fields when set", () => {
		const schema = new MerchantReturnPolicy({
			applicableCountry: "US",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			merchantReturnDays: 30,
			merchantReturnLink: "https://example.com/returns",
			returnMethod: ReturnMethodEnumeration.ReturnByMail,
			returnFees: ReturnFeesEnumeration.ReturnShippingFees,
			returnShippingFeesAmount: new MonetaryAmount("USD", 6.99),
			refundType: RefundTypeEnumeration.FullRefund,
			itemCondition: OfferItemCondition.NewCondition,
			returnLabelSource:
				ReturnLabelSourceEnumeration.ReturnLabelDownloadAndPrint,
			returnPolicyCountry: "US",
			restockingFee: new MonetaryAmount("USD", 10),
			customerRemorseReturnFees:
				ReturnFeesEnumeration.ReturnFeesCustomerResponsibility,
			customerRemorseReturnLabelSource:
				ReturnLabelSourceEnumeration.ReturnLabelCustomerResponsibility,
			customerRemorseReturnShippingFeesAmount: new MonetaryAmount("USD", 12),
			itemDefectReturnFees: ReturnFeesEnumeration.FreeReturn,
			itemDefectReturnLabelSource:
				ReturnLabelSourceEnumeration.ReturnLabelInBox,
			itemDefectReturnShippingFeesAmount: new MonetaryAmount("USD", 0),
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const restockingFee = obj.restockingFee as Record<string, unknown>;

		expect(obj.merchantReturnDays).toBe(30);
		expect(obj.returnMethod).toBe("https://schema.org/ReturnByMail");
		expect(restockingFee["@type"]).toBe("MonetaryAmount");
		expect(obj.itemDefectReturnFees).toBe("https://schema.org/FreeReturn");
	});
});
