<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use PHPUnit\Framework\TestCase;

final class BrandTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Brand(name: 'ACME');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Brand', $obj->{'@type'});
		$this->assertEquals('ACME', $obj->name);
	}

	public function testFullOutput(): void {
		$schema = new Brand(
			name: 'ACME',
			description: 'ACME brand description',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('ACME brand description', $obj->description);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Brand(name: 'ACME');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'description'));
	}
}
