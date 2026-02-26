<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ServicePeriod;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingConditions;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingRateSettings;
use PHPUnit\Framework\TestCase;

final class ShippingConditionsTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new ShippingConditions();
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ShippingConditions', $obj->{'@type'});
		$this->assertFalse(property_exists($obj, 'doesNotShip'));
		$this->assertFalse(property_exists($obj, 'shippingRate'));
	}

	public function testOutputWithMonetaryShippingRate(): void {
		$schema = new ShippingConditions(
			shippingRate: new MonetaryAmount(
				currency: 'USD',
				value: 0.0,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('MonetaryAmount', $obj->shippingRate->{'@type'});
		$this->assertEquals('USD', $obj->shippingRate->currency);
		$this->assertEquals(0.0, $obj->shippingRate->value);
	}

	public function testOutputWithShippingRateSettings(): void {
		$schema = new ShippingConditions(
			doesNotShip: false,
			numItems: new QuantitativeValue(value: 2.0),
			orderValue: new MonetaryAmount(currency: 'USD', value: 50.0),
			shippingDestination: new DefinedRegion(addressCountry: 'US', addressRegion: 'CA'),
			shippingOrigin: new DefinedRegion(addressCountry: 'US', postalCode: '94105'),
			shippingRate: new ShippingRateSettings(orderPercentage: 0.1),
			transitTime: new ServicePeriod(cutoffTime: '16:00:00'),
			weight: new QuantitativeValue(unitCode: 'KGM', maxValue: 10.0),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse($obj->doesNotShip);
		$this->assertEquals('ShippingRateSettings', $obj->shippingRate->{'@type'});
		$this->assertEquals(0.1, $obj->shippingRate->orderPercentage);
		$this->assertEquals('CA', $obj->shippingDestination->addressRegion);
		$this->assertEquals('94105', $obj->shippingOrigin->postalCode);
		$this->assertEquals('16:00:00', $obj->transitTime->cutoffTime);
	}
}
