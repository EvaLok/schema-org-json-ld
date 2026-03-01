<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ShippingDeliveryTime;
use PHPUnit\Framework\TestCase;

final class ShippingDeliveryTimeTest extends TestCase {
	public function testOutput(): void {
		$schema = new ShippingDeliveryTime(
			handlingTime: new QuantitativeValue(
				minValue: 1.0,
				maxValue: 2.0,
				unitCode: 'DAY',
			),
			transitTime: new QuantitativeValue(
				minValue: 3.0,
				maxValue: 5.0,
				unitCode: 'DAY',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ShippingDeliveryTime', $obj->{'@type'});
		$this->assertEquals('QuantitativeValue', $obj->handlingTime->{'@type'});
		$this->assertEquals(1.0, $obj->handlingTime->minValue);
		$this->assertEquals('QuantitativeValue', $obj->transitTime->{'@type'});
		$this->assertEquals(5.0, $obj->transitTime->maxValue);
	}
}
