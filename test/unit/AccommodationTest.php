<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Accommodation;
use EvaLok\SchemaOrgJsonLd\v1\Schema\BedDetails;
use EvaLok\SchemaOrgJsonLd\v1\Schema\LocationFeatureSpecification;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use PHPUnit\Framework\TestCase;

final class AccommodationTest extends TestCase {
	public function testRequiredOutput(): void {
		$accommodation = new Accommodation(
			occupancy: new QuantitativeValue(value: 6),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $accommodation);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Accommodation', $obj->{'@type'});
		$this->assertEquals('QuantitativeValue', $obj->occupancy->{'@type'});
		$this->assertEquals(6, $obj->occupancy->value);
	}

	public function testBedAndAmenityOutput(): void {
		$accommodation = new Accommodation(
			occupancy: new QuantitativeValue(value: 4),
			bed: [
				new BedDetails(numberOfBeds: 1, typeOfBed: 'King'),
				new BedDetails(numberOfBeds: 2, typeOfBed: 'Twin'),
			],
			amenityFeature: [
				new LocationFeatureSpecification(name: 'wifi', value: true),
				new LocationFeatureSpecification(name: 'internetType', value: 'fiber'),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $accommodation);
		$obj = json_decode($json);

		$this->assertEquals('BedDetails', $obj->bed[0]->{'@type'});
		$this->assertEquals('King', $obj->bed[0]->typeOfBed);
		$this->assertEquals('LocationFeatureSpecification', $obj->amenityFeature[0]->{'@type'});
		$this->assertTrue($obj->amenityFeature[0]->value);
		$this->assertEquals('fiber', $obj->amenityFeature[1]->value);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$accommodation = new Accommodation(
			occupancy: new QuantitativeValue(value: 2),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $accommodation);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'additionalType'));
		$this->assertFalse(property_exists($obj, 'numberOfBedrooms'));
		$this->assertFalse(property_exists($obj, 'numberOfBathroomsTotal'));
		$this->assertFalse(property_exists($obj, 'numberOfRooms'));
		$this->assertFalse(property_exists($obj, 'floorSize'));
		$this->assertFalse(property_exists($obj, 'bed'));
		$this->assertFalse(property_exists($obj, 'amenityFeature'));
	}
}
