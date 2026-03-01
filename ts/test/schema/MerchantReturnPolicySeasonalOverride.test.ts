import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { MerchantReturnEnumeration } from "../../src/enum/MerchantReturnEnumeration";
import { MerchantReturnPolicySeasonalOverride } from "../../src/schema/MerchantReturnPolicySeasonalOverride";

describe("MerchantReturnPolicySeasonalOverride", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new MerchantReturnPolicySeasonalOverride(
			"2026-11-20",
			"2026-12-31",
			MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("MerchantReturnPolicySeasonalOverride");
		expect(obj.startDate).toBe("2026-11-20");
		expect(obj.endDate).toBe("2026-12-31");
		expect(obj.returnPolicyCategory).toBe(
			"https://schema.org/MerchantReturnFiniteReturnWindow",
		);
	});

	it("omits optional fields when null", () => {
		const schema = new MerchantReturnPolicySeasonalOverride(
			"2026-11-20",
			"2026-12-31",
			MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			null,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("merchantReturnDays");
	});

	it("includes all fields when set", () => {
		const schema = new MerchantReturnPolicySeasonalOverride(
			"2026-11-20",
			"2026-12-31",
			MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			45,
		);
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.merchantReturnDays).toBe(45);
	});
});
