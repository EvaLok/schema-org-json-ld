<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OccupationalExperienceRequirements;
use PHPUnit\Framework\TestCase;

final class OccupationalExperienceRequirementsTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new OccupationalExperienceRequirements(
			monthsOfExperience: 24,
		);

		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('OccupationalExperienceRequirements', $obj->{'@type'});
		$this->assertEquals(24, $obj->monthsOfExperience);
	}

	public function testZeroMonthsOfExperienceIsSerialized(): void {
		$schema = new OccupationalExperienceRequirements(
			monthsOfExperience: 0,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(0, $obj->monthsOfExperience);
	}

	public function testOnlyContextTypeAndMonthsOfExperienceAppear(): void {
		$schema = new OccupationalExperienceRequirements(
			monthsOfExperience: 18,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(
			['@context', '@type', 'monthsOfExperience'],
			array_keys(get_object_vars($obj)),
		);
	}

	public function testExactMonthsOfExperienceValueRoundTrips(): void {
		$schema = new OccupationalExperienceRequirements(
			monthsOfExperience: 7,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertSame(7, $obj->monthsOfExperience);
	}
}
