<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use PHPUnit\Framework\TestCase;

final class AggregateRatingTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new AggregateRating(ratingValue: 4.5);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('AggregateRating', $obj->{'@type'});
		$this->assertEquals(4.5, $obj->ratingValue);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new AggregateRating(ratingValue: 4.5);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('bestRating', $obj);
		$this->assertObjectNotHasProperty('worstRating', $obj);
		$this->assertObjectNotHasProperty('ratingCount', $obj);
		$this->assertObjectNotHasProperty('reviewCount', $obj);
	}

	public function testFullOutput(): void {
		$schema = new AggregateRating(
			ratingValue: 4.5,
			bestRating: 5,
			worstRating: 1,
			ratingCount: 200,
			reviewCount: 50,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('AggregateRating', $obj->{'@type'});
		$this->assertEquals(4.5, $obj->ratingValue);
		$this->assertEquals(5, $obj->bestRating);
		$this->assertEquals(1, $obj->worstRating);
		$this->assertEquals(200, $obj->ratingCount);
		$this->assertEquals(50, $obj->reviewCount);
	}
}
