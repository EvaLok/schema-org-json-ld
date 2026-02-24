<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\DayOfWeek;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OpeningHoursSpecification;
use PHPUnit\Framework\TestCase;

final class OpeningHoursSpecificationTest extends TestCase {
	public function testMinimalOutput(): void {
		$openingHoursSpecification = new OpeningHoursSpecification(
			dayOfWeek: DayOfWeek::Monday,
			opens: '09:00',
			closes: '18:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $openingHoursSpecification);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('OpeningHoursSpecification', $obj->{'@type'});
		$this->assertEquals('https://schema.org/Monday', $obj->dayOfWeek);
		$this->assertEquals('09:00', $obj->opens);
		$this->assertEquals('18:00', $obj->closes);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$openingHoursSpecification = new OpeningHoursSpecification(
			dayOfWeek: DayOfWeek::Tuesday,
			opens: '10:00',
			closes: '19:00',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $openingHoursSpecification);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'validFrom'));
		$this->assertFalse(property_exists($obj, 'validThrough'));
	}

	public function testFullOutput(): void {
		$openingHoursSpecification = new OpeningHoursSpecification(
			dayOfWeek: DayOfWeek::Wednesday,
			opens: '08:00',
			closes: '17:00',
			validFrom: '2026-01-01',
			validThrough: '2026-12-31',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $openingHoursSpecification);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/Wednesday', $obj->dayOfWeek);
		$this->assertEquals('08:00', $obj->opens);
		$this->assertEquals('17:00', $obj->closes);
		$this->assertEquals('2026-01-01', $obj->validFrom);
		$this->assertEquals('2026-12-31', $obj->validThrough);
	}
}
