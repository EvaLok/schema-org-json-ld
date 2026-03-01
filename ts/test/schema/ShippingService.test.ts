import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { FulfillmentTypeEnumeration } from "../../src/enum/FulfillmentTypeEnumeration";
import { TierBenefitEnumeration } from "../../src/enum/TierBenefitEnumeration";
import { MemberProgramTier } from "../../src/schema/MemberProgramTier";
import { QuantitativeValue } from "../../src/schema/QuantitativeValue";
import { ServicePeriod } from "../../src/schema/ServicePeriod";
import { ShippingConditions } from "../../src/schema/ShippingConditions";
import { ShippingService } from "../../src/schema/ShippingService";

describe("ShippingService", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new ShippingService(new ShippingConditions({}));
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingConditions = obj.shippingConditions as Record<
			string,
			unknown
		>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("ShippingService");
		expect(shippingConditions["@type"]).toBe("ShippingConditions");
	});

	it("omits optional fields when null", () => {
		const schema = new ShippingService(
			new ShippingConditions({}),
			null,
			null,
			null,
			null,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("name");
		expect(obj).not.toHaveProperty("description");
		expect(obj).not.toHaveProperty("fulfillmentType");
		expect(obj).not.toHaveProperty("handlingTime");
		expect(obj).not.toHaveProperty("validForMemberTier");
	});

	it("includes optional fields when set and supports shippingConditions array", () => {
		const schema = new ShippingService(
			[
				new ShippingConditions({}),
				new ShippingConditions({ doesNotShip: false }),
			],
			"Priority Shipping",
			"Fast delivery service",
			FulfillmentTypeEnumeration.FulfillmentTypeDelivery,
			new ServicePeriod(new QuantitativeValue(2, "DAY")),
			new MemberProgramTier(
				"Gold",
				TierBenefitEnumeration.TierBenefitLoyaltyPoints,
			),
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;
		const shippingConditions = obj.shippingConditions as Record<
			string,
			unknown
		>[];
		const handlingTime = obj.handlingTime as Record<string, unknown>;
		const validForMemberTier = obj.validForMemberTier as Record<
			string,
			unknown
		>;

		expect(obj.name).toBe("Priority Shipping");
		expect(obj.description).toBe("Fast delivery service");
		expect(obj.fulfillmentType).toBe(
			"https://schema.org/FulfillmentTypeDelivery",
		);
		expect(shippingConditions).toHaveLength(2);
		expect(shippingConditions[0]?.["@type"]).toBe("ShippingConditions");
		expect(handlingTime["@type"]).toBe("ServicePeriod");
		expect(validForMemberTier["@type"]).toBe("MemberProgramTier");
	});
});
