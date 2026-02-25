<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\LocationFeatureSpecification;
use PHPUnit\Framework\TestCase;

final class LocationFeatureSpecificationTest extends TestCase {
	public function testBooleanValue(): void {
		$schema = new LocationFeatureSpecification(
			name: 'wifi',
			value: true,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('LocationFeatureSpecification', $obj->{'@type'});
		$this->assertEquals('wifi', $obj->name);
		$this->assertTrue($obj->value);
	}

	public function testStringValue(): void {
		$schema = new LocationFeatureSpecification(
			name: 'rating',
			value: '5 stars',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('rating', $obj->name);
		$this->assertEquals('5 stars', $obj->value);
	}
}
