import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { MerchantReturnEnumeration } from "../../src/enum/MerchantReturnEnumeration";
import { MerchantReturnPolicySeasonalOverride } from "../../src/schema/MerchantReturnPolicySeasonalOverride";

describe("MerchantReturnPolicySeasonalOverride", () => {
	it("produces minimal JSON-LD output with required fields only", () => {
		const schema = new MerchantReturnPolicySeasonalOverride({
			startDate: "2026-11-20",
			endDate: "2026-12-31",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
		});
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
		const schema = new MerchantReturnPolicySeasonalOverride({
			startDate: "2026-11-20",
			endDate: "2026-12-31",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			merchantReturnDays: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("merchantReturnDays");
	});

	it("includes all fields when set", () => {
		const schema = new MerchantReturnPolicySeasonalOverride({
			startDate: "2026-11-20",
			endDate: "2026-12-31",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			merchantReturnDays: 45,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.merchantReturnDays).toBe(45);
	});

	it("omits merchantReturnDays when null in minimal output", () => {
		const schema = new MerchantReturnPolicySeasonalOverride({
			startDate: "2026-11-20",
			endDate: "2026-12-31",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			merchantReturnDays: null,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj).not.toHaveProperty("merchantReturnDays");
	});

	it("preserves merchantReturnDays when set to zero", () => {
		const schema = new MerchantReturnPolicySeasonalOverride({
			startDate: "2026-12-24",
			endDate: "2026-12-26",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			merchantReturnDays: 0,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.merchantReturnDays).toBe(0);
	});

	it("serializes the enum value as the schema.org URL", () => {
		const schema = new MerchantReturnPolicySeasonalOverride({
			startDate: "2026-11-01",
			endDate: "2026-11-30",
			returnPolicyCategory:
				MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
			merchantReturnDays: 30,
		});
		const json = JsonLdGenerator.schemaToJson(schema);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj.returnPolicyCategory).toBe(
			MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow,
		);
	});
});
