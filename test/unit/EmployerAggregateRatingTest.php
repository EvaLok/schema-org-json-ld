<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\EmployerAggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Organization;
use PHPUnit\Framework\TestCase;

final class EmployerAggregateRatingTest extends TestCase {
	private function makeOrg(): Organization {
		return new Organization(name: 'Acme Corp');
	}

	public function testMinimalOutput(): void {
		$schema = new EmployerAggregateRating(
			itemReviewed: $this->makeOrg(),
			ratingValue: 4.5,
			ratingCount: 100,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('EmployerAggregateRating', $obj->{'@type'});
		$this->assertEquals(4.5, $obj->ratingValue);
		$this->assertEquals(100, $obj->ratingCount);
		$this->assertEquals('Organization', $obj->itemReviewed->{'@type'});
		$this->assertEquals('Acme Corp', $obj->itemReviewed->name);
	}

	public function testWithCustomScale(): void {
		$schema = new EmployerAggregateRating(
			itemReviewed: $this->makeOrg(),
			ratingValue: 78,
			ratingCount: 250,
			bestRating: 100,
			worstRating: 1,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('EmployerAggregateRating', $obj->{'@type'});
		$this->assertEquals(78, $obj->ratingValue);
		$this->assertEquals(100, $obj->bestRating);
		$this->assertEquals(1, $obj->worstRating);
	}

	public function testWithReviewCount(): void {
		$schema = new EmployerAggregateRating(
			itemReviewed: $this->makeOrg(),
			ratingValue: 3.8,
			reviewCount: 42,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('EmployerAggregateRating', $obj->{'@type'});
		$this->assertEquals(42, $obj->reviewCount);
		$this->assertObjectNotHasProperty('ratingCount', $obj);
	}

	public function testNullOmission(): void {
		$schema = new EmployerAggregateRating(
			itemReviewed: $this->makeOrg(),
			ratingValue: 4.0,
			ratingCount: 10,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertObjectNotHasProperty('reviewCount', $obj);
		$this->assertObjectNotHasProperty('bestRating', $obj);
		$this->assertObjectNotHasProperty('worstRating', $obj);
	}

	public function testFullOutput(): void {
		$schema = new EmployerAggregateRating(
			itemReviewed: $this->makeOrg(),
			ratingValue: 4.2,
			ratingCount: 300,
			reviewCount: 80,
			bestRating: 5,
			worstRating: 1,
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('EmployerAggregateRating', $obj->{'@type'});
		$this->assertEquals(4.2, $obj->ratingValue);
		$this->assertEquals(300, $obj->ratingCount);
		$this->assertEquals(80, $obj->reviewCount);
		$this->assertEquals(5, $obj->bestRating);
		$this->assertEquals(1, $obj->worstRating);
		$this->assertEquals('Organization', $obj->itemReviewed->{'@type'});
		$this->assertEquals('Acme Corp', $obj->itemReviewed->name);
	}
}
