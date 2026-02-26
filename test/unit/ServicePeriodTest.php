<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DayOfWeek;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ServicePeriod;
use PHPUnit\Framework\TestCase;

final class ServicePeriodTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new ServicePeriod();
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ServicePeriod', $obj->{'@type'});
		$this->assertFalse(property_exists($obj, 'duration'));
		$this->assertFalse(property_exists($obj, 'businessDays'));
		$this->assertFalse(property_exists($obj, 'cutoffTime'));
	}

	public function testFullOutput(): void {
		$schema = new ServicePeriod(
			duration: new QuantitativeValue(
				minValue: 1.0,
				maxValue: 3.0,
				unitCode: 'DAY',
			),
			businessDays: [DayOfWeek::Monday, DayOfWeek::Friday],
			cutoffTime: '17:00:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('QuantitativeValue', $obj->duration->{'@type'});
		$this->assertEquals('DAY', $obj->duration->unitCode);
		$this->assertEquals('https://schema.org/Monday', $obj->businessDays[0]);
		$this->assertEquals('https://schema.org/Friday', $obj->businessDays[1]);
		$this->assertEquals('17:00:00', $obj->cutoffTime);
	}
}
