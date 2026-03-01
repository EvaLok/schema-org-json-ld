<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\Enum\TierBenefitEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MemberProgram;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MemberProgramTier;
use PHPUnit\Framework\TestCase;

final class MemberProgramTest extends TestCase {
	public function testProgramWithSingleTier(): void {
		$schema = new MemberProgram(
			name: 'Example Rewards',
			description: 'Loyalty program for regular shoppers.',
			hasTiers: [
				new MemberProgramTier(
					name: 'Silver',
					hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPoints,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('MemberProgram', $obj->{'@type'});
		$this->assertEquals('Example Rewards', $obj->name);
		$this->assertEquals('Loyalty program for regular shoppers.', $obj->description);
		$this->assertEquals('MemberProgramTier', $obj->hasTiers[0]->{'@type'});
		$this->assertEquals('Silver', $obj->hasTiers[0]->name);
		$this->assertEquals('https://schema.org/TierBenefitLoyaltyPoints', $obj->hasTiers[0]->hasTierBenefit);
	}

	public function testProgramWithMultipleTiers(): void {
		$schema = new MemberProgram(
			name: 'Example Rewards',
			description: 'Loyalty program for regular shoppers.',
			hasTiers: [
				new MemberProgramTier(
					name: 'Silver',
					hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPoints,
				),
				new MemberProgramTier(
					name: 'Gold',
					hasTierBenefit: [
						TierBenefitEnumeration::TierBenefitLoyaltyPoints,
						TierBenefitEnumeration::TierBenefitLoyaltyPrice,
					],
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(2, $obj->hasTiers);
		$this->assertEquals('Silver', $obj->hasTiers[0]->name);
		$this->assertEquals('Gold', $obj->hasTiers[1]->name);
		$this->assertEquals('https://schema.org/TierBenefitLoyaltyPoints', $obj->hasTiers[1]->hasTierBenefit[0]);
		$this->assertEquals('https://schema.org/TierBenefitLoyaltyPrice', $obj->hasTiers[1]->hasTierBenefit[1]);
	}

	public function testOptionalUrlOmittedWhenNull(): void {
		$schema = new MemberProgram(
			name: 'Example Rewards',
			description: 'Loyalty program for regular shoppers.',
			hasTiers: [
				new MemberProgramTier(
					name: 'Silver',
					hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPoints,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'url'));
	}
}
