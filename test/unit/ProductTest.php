<?php

namespace EvaLok\SchemaOrgJsonLd\Test\Unit;

use EvaLok\SchemaOrgJsonLd\v1\JsonLdGenerator;
use EvaLok\SchemaOrgJsonLd\v1\Schema\AggregateRating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Brand;
use EvaLok\SchemaOrgJsonLd\v1\Schema\ItemAvailability;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Offer;
use EvaLok\SchemaOrgJsonLd\v1\Schema\OfferItemCondition;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Product;
use EvaLok\SchemaOrgJsonLd\v1\Schema\QuantitativeValue;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Rating;
use EvaLok\SchemaOrgJsonLd\v1\Schema\Review;
use PHPUnit\Framework\TestCase;

final class ProductTest extends TestCase {
	public function testMinimalOutput(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: [
				'https://example.com/photos/1x1/photo.jpg',
				'https://example.com/photos/4x3/photo.jpg',
			],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [
				new Offer(
					url: 'https://example.com/anvil',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('https://schema.org/', $obj->{'@context'});
		$this->assertEquals('Product', $obj->{'@type'});
		$this->assertEquals('Executive Anvil', $obj->name);
		$this->assertEquals(
			[
				'https://example.com/photos/1x1/photo.jpg',
				'https://example.com/photos/4x3/photo.jpg',
			],
			$obj->image,
		);
		$this->assertEquals('Sleeker than ACME\'s Classic Anvil.', $obj->description);
		$this->assertEquals('0446310786', $obj->sku);
		$this->assertCount(1, $obj->offers);
		$this->assertEquals('Offer', $obj->offers[0]->{'@type'});
	}

	public function testOptionalFieldsOmittedWhenNull(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [
				new Offer(
					url: 'https://example.com/anvil',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertFalse(property_exists($obj, 'brand'));
		$this->assertFalse(property_exists($obj, 'mpn'));
		$this->assertFalse(property_exists($obj, 'weight'));
		$this->assertFalse(property_exists($obj, 'aggregateRating'));
		$this->assertFalse(property_exists($obj, 'review'));
	}

	public function testFullOutput(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [
				new Offer(
					url: 'https://example.com/anvil',
					priceCurrency: 'USD',
					price: 119.99,
					itemCondition: OfferItemCondition::NewCondition,
					availability: ItemAvailability::InStock,
				),
			],
			brand: new Brand(name: 'ACME'),
			mpn: 'ACME0444246625',
			weight: new QuantitativeValue(
				value: 55.67,
				unitCode: 'LBR',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Brand', $obj->brand->{'@type'});
		$this->assertEquals('ACME', $obj->brand->name);
		$this->assertEquals('ACME0444246625', $obj->mpn);
		$this->assertEquals('QuantitativeValue', $obj->weight->{'@type'});
		$this->assertEquals(55.67, $obj->weight->value);
		$this->assertEquals('LBR', $obj->weight->unitCode);
	}

	public function testOutputWithAggregateRating(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
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
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
			review: new Review(
				author: 'Jane Doe',
				reviewRating: new Rating(ratingValue: 5),
				reviewBody: 'Excellent anvil.',
			),
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('Review', $obj->review->{'@type'});
		$this->assertEquals('Jane Doe', $obj->review->author);
		$this->assertEquals('Rating', $obj->review->reviewRating->{'@type'});
		$this->assertEquals(5, $obj->review->reviewRating->ratingValue);
		$this->assertEquals('Excellent anvil.', $obj->review->reviewBody);
	}

	public function testOutputWithAggregateRatingAndReviewArray(): void {
		$schema = new Product(
			name: 'Executive Anvil',
			image: ['https://example.com/photos/1x1/photo.jpg'],
			description: 'Sleeker than ACME\'s Classic Anvil.',
			sku: '0446310786',
			offers: [],
			aggregateRating: new AggregateRating(ratingValue: 4.4),
			review: [
				new Review(
					author: 'Jane Doe',
					reviewRating: new Rating(ratingValue: 5),
				),
				new Review(
					author: 'John Doe',
					reviewRating: new Rating(ratingValue: 4),
				),
			],
		);
		$json = JsonLdGenerator::SchemaToJson(schema: $schema);
		$obj = json_decode($json);

		$this->assertEquals('AggregateRating', $obj->aggregateRating->{'@type'});
		$this->assertCount(2, $obj->review);
		$this->assertEquals('Review', $obj->review[0]->{'@type'});
		$this->assertEquals('Jane Doe', $obj->review[0]->author);
		$this->assertEquals('John Doe', $obj->review[1]->author);
	}
}
