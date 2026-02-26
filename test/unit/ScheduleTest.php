<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Schedule;
use PHPUnit\Framework\TestCase;

final class ScheduleTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Schedule(repeatFrequency: 'P1W');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Schedule', $obj->{'@type'});
		$this->assertEquals('P1W', $obj->repeatFrequency);
	}

	public function testFullOutput(): void {
		$schema = new Schedule(
			repeatFrequency: 'P1W',
			repeatCount: 10,
			startDate: '2026-09-01',
			endDate: '2026-11-10',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals(10, $obj->repeatCount);
		$this->assertEquals('2026-09-01', $obj->startDate);
		$this->assertEquals('2026-11-10', $obj->endDate);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Schedule(repeatFrequency: 'P1W');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'repeatCount'));
		$this->assertFalse(property_exists($obj, 'startDate'));
		$this->assertFalse(property_exists($obj, 'endDate'));
	}
}
