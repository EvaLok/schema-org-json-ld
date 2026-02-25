<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use PHPUnit\Framework\TestCase;

final class QuantitativeValueTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new QuantitativeValue();
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('QuantitativeValue', $obj->{'@type'});
		$this->assertFalse(property_exists($obj, 'value'));
		$this->assertFalse(property_exists($obj, 'unitCode'));
		$this->assertFalse(property_exists($obj, 'minValue'));
		$this->assertFalse(property_exists($obj, 'maxValue'));
	}

	public function testValueWithUnitCode(): void {
		$schema = new QuantitativeValue(
			value: 1.5,
			unitCode: 'KGM',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals(1.5, $obj->value);
		$this->assertEquals('KGM', $obj->unitCode);
	}

	public function testMinMaxRange(): void {
		$schema = new QuantitativeValue(
			minValue: 1.0,
			maxValue: 5.0,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals(1.0, $obj->minValue);
		$this->assertEquals(5.0, $obj->maxValue);
	}
}
