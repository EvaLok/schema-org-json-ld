<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PropertyValue;
use PHPUnit\Framework\TestCase;

final class PropertyValueTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new PropertyValue(
			name: 'MagsRUs',
			value: '1234567',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('PropertyValue', $obj->{'@type'});
		$this->assertEquals('MagsRUs', $obj->name);
		$this->assertEquals('1234567', $obj->value);
	}

	public function testFullOutput(): void {
		$schema = new PropertyValue(
			name: 'ExampleCorp',
			value: 'job-42',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('PropertyValue', $obj->{'@type'});
		$this->assertEquals('ExampleCorp', $obj->name);
		$this->assertEquals('job-42', $obj->value);
	}

	public function testOutputContainsOnlyExpectedProperties(): void {
		$schema = new PropertyValue(
			name: 'MagsRUs',
			value: '1234567',
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(4, get_object_vars($obj));
	}
}
