import { describe, expect, it } from "vitest";

import { JsonLdGenerator } from "../../src/JsonLdGenerator";
import { TypedSchema } from "../../src/TypedSchema";
import { DayOfWeek } from "../../src/enum/DayOfWeek";
import { EventAttendanceModeEnumeration } from "../../src/enum/EventAttendanceModeEnumeration";
import { EventStatusType } from "../../src/enum/EventStatusType";
import { FulfillmentTypeEnumeration } from "../../src/enum/FulfillmentTypeEnumeration";
import { ItemAvailability } from "../../src/enum/ItemAvailability";
import { MerchantReturnEnumeration } from "../../src/enum/MerchantReturnEnumeration";
import { OfferItemCondition } from "../../src/enum/OfferItemCondition";
import { RefundTypeEnumeration } from "../../src/enum/RefundTypeEnumeration";
import { ReturnFeesEnumeration } from "../../src/enum/ReturnFeesEnumeration";
import { ReturnLabelSourceEnumeration } from "../../src/enum/ReturnLabelSourceEnumeration";
import { ReturnMethodEnumeration } from "../../src/enum/ReturnMethodEnumeration";
import { TierBenefitEnumeration } from "../../src/enum/TierBenefitEnumeration";

class EnumHolder extends TypedSchema {
	static readonly schemaType = "Thing";

	constructor(public readonly value: string) {
		super();
	}
}

describe("Enums", () => {
	it("exposes all enum values as schema.org URLs", () => {
		expect(DayOfWeek.Monday).toBe("https://schema.org/Monday");
		expect(DayOfWeek.Tuesday).toBe("https://schema.org/Tuesday");
		expect(DayOfWeek.Wednesday).toBe("https://schema.org/Wednesday");
		expect(DayOfWeek.Thursday).toBe("https://schema.org/Thursday");
		expect(DayOfWeek.Friday).toBe("https://schema.org/Friday");
		expect(DayOfWeek.Saturday).toBe("https://schema.org/Saturday");
		expect(DayOfWeek.Sunday).toBe("https://schema.org/Sunday");

		expect(EventStatusType.EventScheduled).toBe(
			"https://schema.org/EventScheduled",
		);
		expect(EventStatusType.EventCancelled).toBe(
			"https://schema.org/EventCancelled",
		);
		expect(EventStatusType.EventPostponed).toBe(
			"https://schema.org/EventPostponed",
		);
		expect(EventStatusType.EventRescheduled).toBe(
			"https://schema.org/EventRescheduled",
		);

		expect(EventAttendanceModeEnumeration.OfflineEventAttendanceMode).toBe(
			"https://schema.org/OfflineEventAttendanceMode",
		);
		expect(EventAttendanceModeEnumeration.OnlineEventAttendanceMode).toBe(
			"https://schema.org/OnlineEventAttendanceMode",
		);
		expect(EventAttendanceModeEnumeration.MixedEventAttendanceMode).toBe(
			"https://schema.org/MixedEventAttendanceMode",
		);

		expect(ItemAvailability.InStock).toBe("https://schema.org/InStock");
		expect(ItemAvailability.OutOfStock).toBe("https://schema.org/OutOfStock");
		expect(ItemAvailability.Discontinued).toBe(
			"https://schema.org/Discontinued",
		);

		expect(OfferItemCondition.NewCondition).toBe(
			"https://schema.org/NewCondition",
		);
		expect(OfferItemCondition.RefurbishedCondition).toBe(
			"https://schema.org/RefurbishedCondition",
		);
		expect(OfferItemCondition.UsedCondition).toBe(
			"https://schema.org/UsedCondition",
		);
		expect(OfferItemCondition.DamagedCondition).toBe(
			"https://schema.org/DamagedCondition",
		);

		expect(MerchantReturnEnumeration.MerchantReturnFiniteReturnWindow).toBe(
			"https://schema.org/MerchantReturnFiniteReturnWindow",
		);
		expect(MerchantReturnEnumeration.MerchantReturnNotPermitted).toBe(
			"https://schema.org/MerchantReturnNotPermitted",
		);
		expect(MerchantReturnEnumeration.MerchantReturnUnlimitedWindow).toBe(
			"https://schema.org/MerchantReturnUnlimitedWindow",
		);

		expect(ReturnFeesEnumeration.FreeReturn).toBe(
			"https://schema.org/FreeReturn",
		);
		expect(ReturnFeesEnumeration.ReturnFeesCustomerResponsibility).toBe(
			"https://schema.org/ReturnFeesCustomerResponsibility",
		);
		expect(ReturnFeesEnumeration.ReturnShippingFees).toBe(
			"https://schema.org/ReturnShippingFees",
		);

		expect(ReturnMethodEnumeration.ReturnAtKiosk).toBe(
			"https://schema.org/ReturnAtKiosk",
		);
		expect(ReturnMethodEnumeration.ReturnByMail).toBe(
			"https://schema.org/ReturnByMail",
		);
		expect(ReturnMethodEnumeration.ReturnInStore).toBe(
			"https://schema.org/ReturnInStore",
		);

		expect(ReturnLabelSourceEnumeration.ReturnLabelCustomerResponsibility).toBe(
			"https://schema.org/ReturnLabelCustomerResponsibility",
		);
		expect(ReturnLabelSourceEnumeration.ReturnLabelDownloadAndPrint).toBe(
			"https://schema.org/ReturnLabelDownloadAndPrint",
		);
		expect(ReturnLabelSourceEnumeration.ReturnLabelInBox).toBe(
			"https://schema.org/ReturnLabelInBox",
		);

		expect(RefundTypeEnumeration.ExchangeRefund).toBe(
			"https://schema.org/ExchangeRefund",
		);
		expect(RefundTypeEnumeration.FullRefund).toBe(
			"https://schema.org/FullRefund",
		);
		expect(RefundTypeEnumeration.StoreCreditRefund).toBe(
			"https://schema.org/StoreCreditRefund",
		);

		expect(TierBenefitEnumeration.TierBenefitLoyaltyPoints).toBe(
			"https://schema.org/TierBenefitLoyaltyPoints",
		);
		expect(TierBenefitEnumeration.TierBenefitLoyaltyPrice).toBe(
			"https://schema.org/TierBenefitLoyaltyPrice",
		);

		expect(FulfillmentTypeEnumeration.FulfillmentTypeDelivery).toBe(
			"https://schema.org/FulfillmentTypeDelivery",
		);
		expect(FulfillmentTypeEnumeration.FulfillmentTypeCollectionPoint).toBe(
			"https://schema.org/FulfillmentTypeCollectionPoint",
		);
	});

	it("serializes enum values through JsonLdGenerator", () => {
		const json = JsonLdGenerator.schemaToJson(
			new EnumHolder(ItemAvailability.InStock),
		);
		const obj = JSON.parse(json) as Record<string, unknown>;

		expect(obj["@context"]).toBe("https://schema.org/");
		expect(obj["@type"]).toBe("Thing");
		expect(obj.value).toBe("https://schema.org/InStock");
	});
});
