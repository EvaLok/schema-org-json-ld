<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MemberProgramTier;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\TierBenefitEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\Schema\UnitPriceSpecification;
use PHPUnit\Framework\TestCase;

final class UnitPriceSpecificationTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new UnitPriceSpecification(
			price: 4.99,
			priceCurrency: 'USD',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('UnitPriceSpecification', $obj->{'@type'});
		$this->assertEquals(4.99, $obj->price);
		$this->assertEquals('USD', $obj->priceCurrency);
	}

	public function testFullOutput(): void {
		$schema = new UnitPriceSpecification(
			price: 2.39,
			priceCurrency: 'EUR',
			priceType: 'https://schema.org/StrikethroughPrice',
			membershipPointsEarned: 120.5,
			validForMemberTier: new MemberProgramTier(
				name: 'Gold',
				hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPrice,
			),
			referenceQuantity: new QuantitativeValue(
				value: 1,
				unitCode: 'KGM',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/StrikethroughPrice', $obj->priceType);
		$this->assertEquals(120.5, $obj->membershipPointsEarned);
		$this->assertEquals('MemberProgramTier', $obj->validForMemberTier->{'@type'});
		$this->assertEquals('Gold', $obj->validForMemberTier->name);
		$this->assertEquals('QuantitativeValue', $obj->referenceQuantity->{'@type'});
		$this->assertEquals(1, $obj->referenceQuantity->value);
		$this->assertEquals('KGM', $obj->referenceQuantity->unitCode);
	}
}
