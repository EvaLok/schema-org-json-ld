<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PostalAddress;
use PHPUnit\Framework\TestCase;

final class PostalAddressTest extends TestCase {

	public function testMinimalOutput(): void {
		$postalAddress = new PostalAddress();
		$json = JsonLdGenerator::SchemaToJson(schema: $postalAddress);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('PostalAddress', $obj->{'@type'});
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$postalAddress = new PostalAddress();
		$json = JsonLdGenerator::SchemaToJson(schema: $postalAddress);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'streetAddress'));
		$this->assertFalse(property_exists($obj, 'addressLocality'));
		$this->assertFalse(property_exists($obj, 'addressRegion'));
		$this->assertFalse(property_exists($obj, 'postalCode'));
		$this->assertFalse(property_exists($obj, 'addressCountry'));
		$this->assertFalse(property_exists($obj, 'postOfficeBoxNumber'));
	}

	public function testPartialAddressOutput(): void {
		$postalAddress = new PostalAddress(
			streetAddress: '1600 Amphitheatre Parkway',
			addressLocality: 'Mountain View',
			postalCode: '94043',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $postalAddress);
		$obj = json_decode($json);

		$this->assertEquals('1600 Amphitheatre Parkway', $obj->streetAddress);
		$this->assertEquals('Mountain View', $obj->addressLocality);
		$this->assertEquals('94043', $obj->postalCode);
		$this->assertFalse(property_exists($obj, 'addressRegion'));
		$this->assertFalse(property_exists($obj, 'addressCountry'));
		$this->assertFalse(property_exists($obj, 'postOfficeBoxNumber'));
	}
}
