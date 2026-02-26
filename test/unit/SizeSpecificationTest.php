<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\SizeSpecification;
use PHPUnit\Framework\TestCase;

final class SizeSpecificationTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new SizeSpecification(name: 'Large');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('SizeSpecification', $obj->{'@type'});
		$this->assertEquals('Large', $obj->name);
	}

	public function testFullOutput(): void {
		$schema = new SizeSpecification(
			name: '42',
			sizeGroup: 'regular',
			sizeSystem: 'EU',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('42', $obj->name);
		$this->assertEquals('regular', $obj->sizeGroup);
		$this->assertEquals('EU', $obj->sizeSystem);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new SizeSpecification(name: 'Large');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'sizeGroup'));
		$this->assertFalse(property_exists($obj, 'sizeSystem'));
	}
}
