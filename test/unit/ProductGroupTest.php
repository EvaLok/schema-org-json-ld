<?php

declare(strict_types=1);

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ProductGroup;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use PHPUnit\Framework\TestCase;

final class ProductGroupTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new ProductGroup(name: 'Winter Jacket');
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$this->assertIsString($json);

		$obj = json_decode($json);
		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('ProductGroup', $obj->{'@type'});
		$this->assertEquals('Winter Jacket', $obj->name);
		$this->assertObjectNotHasProperty('subjectOf', $obj);
	}

	public function testFullOutput(): void {
		$schema = new ProductGroup(
			name: 'Winter Jacket',
			productGroupID: 'JACKET-2026',
			variesBy: [
				'https://schema.org/color',
				'https://schema.org/size',
			],
			url: 'https://example.com/jackets',
			description: 'Waterproof winter jackets available in multiple sizes.',
			brand: new Brand(name: 'ACME'),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('JACKET-2026', $obj->productGroupID);
		$this->assertEquals(['https://schema.org/color', 'https://schema.org/size'], $obj->variesBy);
		$this->assertEquals('https://example.com/jackets', $obj->url);
		$this->assertEquals('Waterproof winter jackets available in multiple sizes.', $obj->description);
		$this->assertEquals('Brand', $obj->brand->{'@type'});
		$this->assertEquals('ACME', $obj->brand->name);
	}

	public function testOutputWithHasVariantProducts(): void {
		$schema = new ProductGroup(
			name: 'Winter Jacket',
			hasVariant: [
				new Product(
					name: 'Winter Jacket Blue M',
					image: ['https://example.com/blue-m.jpg'],
					description: 'Blue winter jacket, size M.',
					sku: 'JACKET-BLUE-M',
					offers: [],
				),
				new Product(
					name: 'Winter Jacket Black L',
					image: ['https://example.com/black-l.jpg'],
					description: 'Black winter jacket, size L.',
					sku: 'JACKET-BLACK-L',
					offers: [],
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertCount(2, $obj->hasVariant);
		$this->assertEquals('Product', $obj->hasVariant[0]->{'@type'});
		$this->assertEquals('Winter Jacket Blue M', $obj->hasVariant[0]->name);
		$this->assertEquals('Winter Jacket Black L', $obj->hasVariant[1]->name);
	}

	public function testOutputWithAggregateRating(): void {
		$schema = new ProductGroup(
			name: 'Winter Jacket',
			aggregateRating: new AggregateRating(
				ratingValue: 4.4,
				reviewCount: 89,
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertEquals(4.4, $obj->aggregateRating->ratingValue);
		$this->assertEquals(89, $obj->aggregateRating->reviewCount);
	}

	public function testOutputWithReview(): void {
		$schema = new ProductGroup(
			name: 'Winter Jacket',
			review: new Review(
				author: 'Jane Doe',
				reviewRating: new Rating(ratingValue: 5),
				reviewBody: 'Excellent jacket.',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Review', $obj->review->{'@type'});
		$this->assertEquals('Jane Doe', $obj->review->author);
		$this->assertEquals('Rating', $obj->review->reviewRating->{'@type'});
		$this->assertEquals(5, $obj->review->reviewRating->ratingValue);
		$this->assertEquals('Excellent jacket.', $obj->review->reviewBody);
	}

	public function testOutputWithSubjectOf(): void {
		$schema = new ProductGroup(
			name: 'Winter Jacket',
			subjectOf: 'https://example.com/models/winter-jacket.glb',
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://example.com/models/winter-jacket.glb', $obj->subjectOf);
	}
}
