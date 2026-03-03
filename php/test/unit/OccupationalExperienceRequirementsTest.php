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
}
