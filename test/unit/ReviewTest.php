<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Person;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use PHPUnit\Framework\TestCase;

final class ReviewTest extends TestCase {
	public function testMinimalOutput(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Review', $obj->{'@type'});
		$this->assertEquals('Jane Doe', $obj->author);
		$this->assertEquals('Rating', $obj->reviewRating->{'@type'});
		$this->assertEquals(5, $obj->reviewRating->ratingValue);
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'reviewBody'));
		$this->assertFalse(property_exists($obj, 'datePublished'));
		$this->assertFalse(property_exists($obj, 'name'));
	}

	public function testFullOutput(): void {
		$review = new Review(
			author: 'Jane Doe',
			reviewRating: new Rating(
				ratingValue: 4,
				bestRating: 5,
				worstRating: 1,
			),
			reviewBody: 'A great product!',
			datePublished: '2026-01-15',
			name: 'Excellent',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Jane Doe', $obj->author);
		$this->assertEquals(4, $obj->reviewRating->ratingValue);
		$this->assertEquals(5, $obj->reviewRating->bestRating);
		$this->assertEquals(1, $obj->reviewRating->worstRating);
		$this->assertEquals('A great product!', $obj->reviewBody);
		$this->assertEquals('2026-01-15', $obj->datePublished);
		$this->assertEquals('Excellent', $obj->name);
	}

	public function testAuthorAsPerson(): void {
		$review = new Review(
			author: new Person(name: 'Jane Doe'),
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Person', $obj->author->{'@type'});
		$this->assertEquals('Jane Doe', $obj->author->name);
	}

	public function testAuthorAsOrganization(): void {
		$review = new Review(
			author: new Organization(name: 'Example Inc'),
			reviewRating: new Rating(ratingValue: 5),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $review);
		$obj = json_decode($json);

		$this->assertEquals('Organization', $obj->author->{'@type'});
		$this->assertEquals('Example Inc', $obj->author->name);
	}
}
