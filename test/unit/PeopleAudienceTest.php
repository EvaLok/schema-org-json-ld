<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\PeopleAudience;
use PHPUnit\Framework\TestCase;

final class PeopleAudienceTest extends TestCase {
	public function testOutputWithSuggestedGenderOnly(): void {
		$schema = new PeopleAudience(suggestedGender: 'Unisex');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('PeopleAudience', $obj->{'@type'});
		$this->assertEquals('Unisex', $obj->suggestedGender);
		$this->assertFalse(property_exists($obj, 'suggestedMinAge'));
		$this->assertFalse(property_exists($obj, 'suggestedMaxAge'));
	}

	public function testOutputWithAgeRangeOnly(): void {
		$schema = new PeopleAudience(
			suggestedMinAge: 3,
			suggestedMaxAge: 12,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals(3, $obj->suggestedMinAge);
		$this->assertEquals(12, $obj->suggestedMaxAge);
		$this->assertFalse(property_exists($obj, 'suggestedGender'));
	}

	public function testFullOutput(): void {
		$schema = new PeopleAudience(
			suggestedGender: 'Female',
			suggestedMinAge: 16.5,
			suggestedMaxAge: 65,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Female', $obj->suggestedGender);
		$this->assertEquals(16.5, $obj->suggestedMinAge);
		$this->assertEquals(65, $obj->suggestedMaxAge);
	}
}
