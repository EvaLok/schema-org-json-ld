<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DefinedRegion;
use PHPUnit\Framework\TestCase;

final class DefinedRegionTest extends TestCase {
	public function testCountryOnlyOutput(): void {
		$schema = new DefinedRegion(addressCountry: 'US');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('DefinedRegion', $obj->{'@type'});
		$this->assertEquals('US', $obj->addressCountry);
		$this->assertFalse(property_exists($obj, 'addressRegion'));
		$this->assertFalse(property_exists($obj, 'postalCode'));
	}

	public function testAddressRegionStringOutput(): void {
		$schema = new DefinedRegion(
			addressCountry: 'US',
			addressRegion: 'NY',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('NY', $obj->addressRegion);
	}

	public function testOutputWithAddressRegionArray(): void {
		$schema = new DefinedRegion(
			addressCountry: 'US',
			addressRegion: ['CA', 'NV'],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('DefinedRegion', $obj->{'@type'});
		$this->assertEquals('US', $obj->addressCountry);
		$this->assertEquals('CA', $obj->addressRegion[0]);
		$this->assertEquals('NV', $obj->addressRegion[1]);
	}

	public function testOutputWithPostalCode(): void {
		$schema = new DefinedRegion(
			addressCountry: 'US',
			postalCode: '94105',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('94105', $obj->postalCode);
	}
}
