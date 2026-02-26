<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\CourseInstance;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Schedule;
use PHPUnit\Framework\TestCase;

final class CourseInstanceTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new CourseInstance(courseMode: 'online');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('CourseInstance', $obj->{'@type'});
		$this->assertEquals('online', $obj->courseMode);
	}

	public function testFullOutput(): void {
		$schema = new CourseInstance(
			courseMode: 'online',
			instructor: new Person(name: 'Dr. Ada Lovelace'),
			courseSchedule: new Schedule(
				repeatFrequency: 'P1W',
				repeatCount: 10,
				startDate: '2026-09-01',
				endDate: '2026-11-10',
			),
			courseWorkload: 'PT22H',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Person', $obj->instructor->{'@type'});
		$this->assertEquals('Dr. Ada Lovelace', $obj->instructor->name);
		$this->assertEquals('Schedule', $obj->courseSchedule->{'@type'});
		$this->assertEquals('P1W', $obj->courseSchedule->repeatFrequency);
		$this->assertEquals(10, $obj->courseSchedule->repeatCount);
		$this->assertEquals('2026-09-01', $obj->courseSchedule->startDate);
		$this->assertEquals('2026-11-10', $obj->courseSchedule->endDate);
		$this->assertEquals('PT22H', $obj->courseWorkload);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new CourseInstance(courseMode: 'online');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'instructor'));
		$this->assertFalse(property_exists($obj, 'courseSchedule'));
		$this->assertFalse(property_exists($obj, 'courseWorkload'));
	}
}
