<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MemberProgramTier;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\TierBenefitEnumeration;
use PHPUnit\Framework\TestCase;

final class MemberProgramTierTest extends TestCase {
	public function testSingleBenefitOutput(): void {
		$schema = new MemberProgramTier(
			name: 'Silver',
			hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPoints,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('MemberProgramTier', $obj->{'@type'});
		$this->assertEquals('Silver', $obj->name);
		$this->assertEquals('https://schema.org/TierBenefitLoyaltyPoints', $obj->hasTierBenefit);
	}

	public function testMultipleBenefitsOutput(): void {
		$schema = new MemberProgramTier(
			name: 'Gold',
			hasTierBenefit: [
				TierBenefitEnumeration::TierBenefitLoyaltyPoints,
				TierBenefitEnumeration::TierBenefitLoyaltyPrice,
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/TierBenefitLoyaltyPoints', $obj->hasTierBenefit[0]);
		$this->assertEquals('https://schema.org/TierBenefitLoyaltyPrice', $obj->hasTierBenefit[1]);
	}

	public function testMembershipPointsEarnedOutput(): void {
		$schema = new MemberProgramTier(
			name: 'Platinum',
			hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPoints,
			membershipPointsEarned: new QuantitativeValue(
				value: 2.0,
				unitCode: 'P1',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('QuantitativeValue', $obj->membershipPointsEarned->{'@type'});
		$this->assertEquals(2.0, $obj->membershipPointsEarned->value);
		$this->assertEquals('P1', $obj->membershipPointsEarned->unitCode);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new MemberProgramTier(
			name: 'Silver',
			hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPoints,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'hasTierRequirement'));
		$this->assertFalse(property_exists($obj, 'membershipPointsEarned'));
		$this->assertFalse(property_exists($obj, 'url'));
	}
}
