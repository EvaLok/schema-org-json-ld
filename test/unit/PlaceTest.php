<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\GeoCoordinates;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Place;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use PHPUnit\Framework\TestCase;

final class PlaceTest extends TestCase {
	public function testMinimalOutput(): void {
		$place = new Place(
			name: 'Main Theater',
			address: new PostalAddress(streetAddress: '123 Main Street'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $place);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Place', $obj->{'@type'});
		$this->assertEquals('Main Theater', $obj->name);
		$this->assertEquals('PostalAddress', $obj->address->{'@type'});
		$this->assertEquals('123 Main Street', $obj->address->streetAddress);
	}

	public function testWithGeoCoordinates(): void {
		$place = new Place(
			name: 'Main Theater',
			geo: new GeoCoordinates(
				latitude: 52.37,
				longitude: 4.89,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $place);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('Place', $obj->{'@type'});
		$this->assertEquals('Main Theater', $obj->name);
		$this->assertEquals('GeoCoordinates', $obj->geo->{'@type'});
		$this->assertEquals(52.37, $obj->geo->latitude);
		$this->assertEquals(4.89, $obj->geo->longitude);
		$this->assertFalse(property_exists($obj, 'address'));
	}
}
