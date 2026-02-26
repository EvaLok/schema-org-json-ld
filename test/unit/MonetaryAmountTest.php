<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\MonetaryAmount;
use PHPUnit\Framework\TestCase;

final class MonetaryAmountTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new MonetaryAmount(currency: 'USD');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('MonetaryAmount', $obj->{'@type'});
		$this->assertEquals('USD', $obj->currency);
		$this->assertFalse(property_exists($obj, 'value'));
		$this->assertFalse(property_exists($obj, 'minValue'));
		$this->assertFalse(property_exists($obj, 'maxValue'));
		$this->assertFalse(property_exists($obj, 'unitText'));
	}

	public function testFullOutput(): void {
		$schema = new MonetaryAmount(
			currency: 'USD',
			value: 10.99,
			minValue: 5.00,
			maxValue: 15.00,
			unitText: 'HOUR',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('USD', $obj->currency);
		$this->assertEquals(10.99, $obj->value);
		$this->assertEquals(5.00, $obj->minValue);
		$this->assertEquals(15.00, $obj->maxValue);
		$this->assertEquals('HOUR', $obj->unitText);
	}
}
