<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MerchantReturnEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MerchantReturnPolicySeasonalOverride;
use PHPUnit\Framework\TestCase;

final class MerchantReturnPolicySeasonalOverrideTest extends TestCase {
	public function testBasicOutput(): void {
		$schema = new MerchantReturnPolicySeasonalOverride(
			startDate: '2026-11-15',
			endDate: '2027-01-15',
			returnPolicyCategory: MerchantReturnEnumeration::MerchantReturnFiniteReturnWindow,
			merchantReturnDays: 60,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('MerchantReturnPolicySeasonalOverride', $obj->{'@type'});
		$this->assertEquals('2026-11-15', $obj->startDate);
		$this->assertEquals('2027-01-15', $obj->endDate);
		$this->assertEquals('https://schema.org/MerchantReturnFiniteReturnWindow', $obj->returnPolicyCategory);
		$this->assertEquals(60, $obj->merchantReturnDays);
	}
}
