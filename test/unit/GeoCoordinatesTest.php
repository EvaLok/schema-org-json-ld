<?php

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
}
