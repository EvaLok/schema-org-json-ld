<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferShippingDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingDeliveryTime;
use PHPUnit\Framework\TestCase;

final class OfferShippingDetailsTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new OfferShippingDetails(
			shippingDestination: new DefinedRegion(
				addressCountry: 'US',
				addressRegion: ['CA', 'NV'],
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('OfferShippingDetails', $obj->{'@type'});
		$this->assertEquals('DefinedRegion', $obj->shippingDestination->{'@type'});
		$this->assertEquals('US', $obj->shippingDestination->addressCountry);
		$this->assertEquals('CA', $obj->shippingDestination->addressRegion[0]);
		$this->assertEquals('NV', $obj->shippingDestination->addressRegion[1]);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new OfferShippingDetails(
			shippingDestination: new DefinedRegion(
				addressCountry: 'US',
				addressRegion: ['CA', 'NV'],
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'shippingRate'));
		$this->assertFalse(property_exists($obj, 'deliveryTime'));
		$this->assertFalse(property_exists($obj, 'doesNotShip'));
	}

	public function testFullOutput(): void {
		$schema = new OfferShippingDetails(
			shippingDestination: new DefinedRegion(
				addressCountry: 'US',
				addressRegion: ['CA', 'NV'],
			),
			shippingRate: new MonetaryAmount(
				currency: 'USD',
				value: 10.99,
				minValue: 5.00,
				maxValue: 15.00,
			),
			deliveryTime: new ShippingDeliveryTime(
				handlingTime: new QuantitativeValue(
					minValue: 1.0,
					maxValue: 2.0,
					unitCode: 'DAY',
				),
				transitTime: new QuantitativeValue(
					minValue: 3.0,
					maxValue: 5.0,
					unitCode: 'DAY',
				),
			),
			doesNotShip: false,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('MonetaryAmount', $obj->shippingRate->{'@type'});
		$this->assertEquals(10.99, $obj->shippingRate->value);
		$this->assertEquals('ShippingDeliveryTime', $obj->deliveryTime->{'@type'});
		$this->assertEquals('QuantitativeValue', $obj->deliveryTime->handlingTime->{'@type'});
		$this->assertEquals(1.0, $obj->deliveryTime->handlingTime->minValue);
		$this->assertEquals(5.0, $obj->deliveryTime->transitTime->maxValue);
		$this->assertFalse($obj->doesNotShip);
	}
}
