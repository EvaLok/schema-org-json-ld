<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\GeoCoordinates;
use PHPUnit\Framework\TestCase;

final class GeoCoordinatesTest extends TestCase {
	public function testMinimalOutput(): void {
		$geo = new GeoCoordinates(
			latitude: 52.37022,
			longitude: 4.89517,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $geo);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('GeoCoordinates', $obj->{'@type'});
		$this->assertEquals(52.37022, $obj->latitude);
		$this->assertEquals(4.89517, $obj->longitude);
	}

	public function testZeroCoordinatesAreSerialized(): void {
		$geo = new GeoCoordinates(
			latitude: 0.0,
			longitude: 0.0,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $geo);
		$obj = json_decode($json);

		$this->assertEquals(0.0, $obj->latitude);
		$this->assertEquals(0.0, $obj->longitude);
	}

	public function testNegativeCoordinatesAreSerialized(): void {
		$geo = new GeoCoordinates(
			latitude: -33.86882,
			longitude: -151.20929,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $geo);
		$obj = json_decode($json);

		$this->assertEquals(-33.86882, $obj->latitude);
		$this->assertEquals(-151.20929, $obj->longitude);
	}

	public function testHighPrecisionCoordinatesAreSerialized(): void {
		$geo = new GeoCoordinates(
			latitude: 12.345678901234,
			longitude: 98.765432109876,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $geo);
		$obj = json_decode($json);

		$this->assertEquals(12.345678901234, $obj->latitude);
		$this->assertEquals(98.765432109876, $obj->longitude);
	}
}
