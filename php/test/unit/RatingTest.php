<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use PHPUnit\Framework\TestCase;

final class RatingTest extends TestCase {
	public function testMinimalOutput(): void {
		$rating = new Rating(ratingValue: 4.5);
		$json = JsonLdGenerator::SchemaToJson(schema: $rating);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Rating', $obj->{'@type'});
		$this->assertEquals(4.5, $obj->ratingValue);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$rating = new Rating(ratingValue: 4);
		$json = JsonLdGenerator::SchemaToJson(schema: $rating);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'bestRating'));
		$this->assertFalse(property_exists($obj, 'worstRating'));
	}

	public function testFullOutput(): void {
		$rating = new Rating(
			ratingValue: 4,
			bestRating: 5,
			worstRating: 1,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $rating);
		$obj = json_decode($json);

		$this->assertEquals(4, $obj->ratingValue);
		$this->assertEquals(5, $obj->bestRating);
		$this->assertEquals(1, $obj->worstRating);
	}

	public function testZeroValuesAreSerialized(): void {
		$rating = new Rating(
			ratingValue: 0,
			bestRating: 0,
			worstRating: 0,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $rating);
		$obj = json_decode($json);

		$this->assertSame(0, $obj->ratingValue);
		$this->assertSame(0, $obj->bestRating);
		$this->assertSame(0, $obj->worstRating);
	}

	public function testDecimalValuesArePreserved(): void {
		$rating = new Rating(
			ratingValue: 4.75,
			bestRating: 5.0,
			worstRating: 0.5,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $rating);
		$obj = json_decode($json);

		$this->assertEquals(4.75, $obj->ratingValue);
		$this->assertEquals(5.0, $obj->bestRating);
		$this->assertEquals(0.5, $obj->worstRating);
	}
}
