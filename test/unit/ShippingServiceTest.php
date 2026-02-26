<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\Enum\FulfillmentTypeEnumeration;
use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MemberProgramTier;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ServicePeriod;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingConditions;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingService;
use EvaLok\SchemaOrgJsonLd\v1\Schema\TierBenefitEnumeration;
use PHPUnit\Framework\TestCase;

final class ShippingServiceTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new ShippingService(
			shippingConditions: new ShippingConditions(),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ShippingService', $obj->{'@type'});
		$this->assertEquals('ShippingConditions', $obj->shippingConditions->{'@type'});
		$this->assertFalse(property_exists($obj, 'name'));
		$this->assertFalse(property_exists($obj, 'description'));
	}

	public function testFullOutput(): void {
		$schema = new ShippingService(
			shippingConditions: [
				new ShippingConditions(doesNotShip: false),
				new ShippingConditions(doesNotShip: true),
			],
			name: 'Standard Shipping',
			description: 'Ships in 1-3 business days',
			fulfillmentType: FulfillmentTypeEnumeration::FulfillmentTypeDelivery,
			handlingTime: new ServicePeriod(cutoffTime: '15:00:00'),
			validForMemberTier: new MemberProgramTier(
				name: 'Gold',
				hasTierBenefit: TierBenefitEnumeration::TierBenefitLoyaltyPoints,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Standard Shipping', $obj->name);
		$this->assertEquals('Ships in 1-3 business days', $obj->description);
		$this->assertEquals('https://schema.org/FulfillmentTypeDelivery', $obj->fulfillmentType);
		$this->assertEquals('15:00:00', $obj->handlingTime->cutoffTime);
		$this->assertEquals('Gold', $obj->validForMemberTier->name);
		$this->assertFalse($obj->shippingConditions[0]->doesNotShip);
		$this->assertTrue($obj->shippingConditions[1]->doesNotShip);
	}
}
