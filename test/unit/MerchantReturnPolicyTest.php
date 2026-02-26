<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\Enum\MerchantReturnEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Enum\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Enum\RefundTypeEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Enum\ReturnFeesEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Enum\ReturnLabelSourceEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Enum\ReturnMethodEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MerchantReturnPolicy;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MerchantReturnPolicySeasonalOverride;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use PHPUnit\Framework\TestCase;

final class MerchantReturnPolicyTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new MerchantReturnPolicy(
			applicableCountry: 'US',
			returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnNotPermitted,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('MerchantReturnPolicy', $obj->{'@type'});
		$this->assertEquals('US', $obj->applicableCountry);
		$this->assertEquals('https://schema.org/MerchantReturnNotPermitted', $obj->returnPolicyCategory);
		$this->assertFalse(property_exists($obj, 'merchantReturnDays'));
		$this->assertFalse(property_exists($obj, 'merchantReturnLink'));
	}

	public function testFullOutput(): void {
		$schema = new MerchantReturnPolicy(
			applicableCountry: ['US', 'CA'],
			returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow,
			merchantReturnDays: 30,
			merchantReturnLink: 'https://example.com/returns',
			returnMethod: ReturnMethodEnumeration::ReturnByMail,
			returnFees: ReturnFeesEnumeration::ReturnShippingFees,
			returnShippingFeesAmount: new MonetaryAmount(currency: 'USD', value: 8.99),
			refundType: RefundTypeEnumeration::FullRefund,
			itemCondition: OfferItemCondition::NewCondition,
			returnLabelSource: ReturnLabelSourceEnumeration::ReturnLabelDownloadAndPrint,
			returnPolicyCountry: 'US',
			restockingFee: 5.5,
			customerRemorseReturnFees: ReturnFeesEnumeration::ReturnFeesCustomerResponsibility,
			customerRemorseReturnLabelSource: ReturnLabelSourceEnumeration::ReturnLabelCustomerResponsibility,
			customerRemorseReturnShippingFeesAmount: new MonetaryAmount(currency: 'USD', value: 10.0),
			itemDefectReturnFees: ReturnFeesEnumeration::FreeReturn,
			itemDefectReturnLabelSource: ReturnLabelSourceEnumeration::ReturnLabelInBox,
			itemDefectReturnShippingFeesAmount: new MonetaryAmount(currency: 'USD', value: 0.0),
			returnPolicySeasonalOverride: new MerchantReturnPolicySeasonalOverride(
				startDate: '2026-11-15',
				endDate: '2027-01-15',
				returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow,
				merchantReturnDays: 60,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals(['US', 'CA'], $obj->applicableCountry);
		$this->assertEquals('https://schema.org/ReturnByMail', $obj->returnMethod);
		$this->assertEquals('https://schema.org/ReturnShippingFees', $obj->returnFees);
		$this->assertEquals('MonetaryAmount', $obj->returnShippingFeesAmount->{'@type'});
		$this->assertEquals(8.99, $obj->returnShippingFeesAmount->value);
		$this->assertEquals('https://schema.org/FullRefund', $obj->refundType);
		$this->assertEquals('https://schema.org/NewCondition', $obj->itemCondition);
		$this->assertEquals('https://schema.org/ReturnLabelDownloadAndPrint', $obj->returnLabelSource);
		$this->assertEquals(5.5, $obj->restockingFee);
		$this->assertEquals('MerchantReturnPolicySeasonalOverride', $obj->returnPolicySeasonalOverride->{'@type'});
		$this->assertEquals(60, $obj->returnPolicySeasonalOverride->merchantReturnDays);
	}

	public function testReturnPolicySeasonalOverrideAsArray(): void {
		$schema = new MerchantReturnPolicy(
			applicableCountry: 'US',
			returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow,
			returnPolicySeasonalOverride: [
				new MerchantReturnPolicySeasonalOverride(
					startDate: '2026-11-15',
					endDate: '2027-01-15',
					returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow,
					merchantReturnDays: 60,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(1, $obj->returnPolicySeasonalOverride);
		$this->assertEquals('MerchantReturnPolicySeasonalOverride', $obj->returnPolicySeasonalOverride[0]->{'@type'});
		$this->assertEquals('https://schema.org/MerchantReturnFiniteReturnWindow', $obj->returnPolicySeasonalOverride[0]->returnPolicyCategory);
	}
}
